// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::PluginVariables;
use super::plugin_root_of;
use crate::parse_subagent_at;

/// A Claude Code home with one installed plugin, `demo@market`, and the plugin's root.
fn installed_plugin(tmp: &TempDir) -> (PathBuf, PathBuf) {
    let claude_home = tmp.path().join("home/.claude");
    let root = claude_home.join("plugins/cache/market/demo/1.0.0");
    fs::create_dir_all(root.join(".claude-plugin")).expect("create");
    fs::create_dir_all(root.join("agents")).expect("create");
    fs::create_dir_all(root.join("extra/review")).expect("create");
    fs::write(
        root.join(".claude-plugin/plugin.json"),
        r#"{
            "name": "demo",
            "agents": ["./extra"],
            "userConfig": {
                "api_url": {"type": "string", "title": "URL", "description": "d"},
                "api-token": {"type": "string", "title": "Token", "description": "d", "sensitive": true},
                "retries": {"type": "number", "title": "Retries", "description": "d"},
                "unset": {"type": "string", "title": "Unset", "description": "d"},
                "needed": {"type": "string", "title": "Needed", "description": "d", "required": true},
                "region": {"type": "string", "title": "Region", "description": "d", "default": "eu"},
                "port": {"type": "number", "title": "Port", "description": "d", "default": 80}
            }
        }"#,
    )
    .expect("write");
    fs::write(
        claude_home.join("plugins/installed_plugins.json"),
        serde_json::json!({
            "version": 2,
            "plugins": {"demo@market": [{"scope": "user", "installPath": root}]}
        })
        .to_string(),
    )
    .expect("write");
    fs::write(
        claude_home.join("settings.json"),
        r#"{"pluginConfigs": {"demo@market": {"options": {"api_url": "https://api.example.com", "retries": 3, "port": 8080}}}}"#,
    )
    .expect("write");
    fs::write(
        claude_home.join(".credentials.json"),
        r#"{"pluginSecrets": {"demo@market": {"api-token": "s3cret"}}}"#,
    )
    .expect("write");
    (claude_home, root)
}

fn load(claude_home: &Path, root: &Path) -> PluginVariables {
    PluginVariables::load_with_home(root, Some(claude_home)).expect("plugin")
}

#[test]
fn options_come_from_settings_and_the_secure_store() {
    let tmp = TempDir::new().expect("temp dir");
    let (claude_home, root) = installed_plugin(&tmp);
    let plugin = load(&claude_home, &root);

    assert_eq!(plugin.name, "demo");
    assert_eq!(
        plugin.option_env(),
        vec![
            (
                "CLAUDE_PLUGIN_OPTION_API_TOKEN".to_string(),
                "s3cret".to_string()
            ),
            (
                "CLAUDE_PLUGIN_OPTION_API_URL".to_string(),
                "https://api.example.com".to_string()
            ),
            ("CLAUDE_PLUGIN_OPTION_PORT".to_string(), "8080".to_string()),
            ("CLAUDE_PLUGIN_OPTION_REGION".to_string(), "eu".to_string()),
            ("CLAUDE_PLUGIN_OPTION_RETRIES".to_string(), "3".to_string()),
        ]
    );
}

#[test]
fn commands_get_every_value_or_name_the_missing_option() {
    let tmp = TempDir::new().expect("temp dir");
    let (claude_home, root) = installed_plugin(&tmp);
    let plugin = load(&claude_home, &root);

    assert_eq!(
        plugin.substitute(
            "${CLAUDE_PLUGIN_ROOT}/run --url ${user_config.api_url} -t ${user_config.api-token}"
        ),
        Ok(format!(
            "{}/run --url https://api.example.com -t s3cret",
            root.display()
        ))
    );
    // A default stands in for a value never stored; a stored value wins over it.
    assert_eq!(
        plugin.substitute("--region=${user_config.region} --port=${user_config.port}"),
        Ok("--region=eu --port=8080".to_string())
    );
    // An optional option left blank reads as empty; a required or undeclared one is missing.
    assert_eq!(
        plugin.substitute("run ${user_config.unset}"),
        Ok("run ".to_string())
    );
    assert_eq!(
        plugin.substitute("run ${user_config.needed}"),
        Err("needed".to_string())
    );
    assert_eq!(
        plugin.substitute("run ${user_config.undeclared}"),
        Err("undeclared".to_string())
    );
}

#[test]
fn prose_hides_sensitive_values_and_keeps_unset_ones_as_written() {
    let tmp = TempDir::new().expect("temp dir");
    let (claude_home, root) = installed_plugin(&tmp);
    let plugin = load(&claude_home, &root);

    assert_eq!(
        plugin.substitute_in_content(
            "Call ${user_config.api_url} with ${user_config.api-token}, ${user_config.unset}."
        ),
        "Call https://api.example.com with [sensitive option 'api-token' not available in skill content], ${user_config.unset}."
    );
}

#[test]
fn the_data_directory_is_codex_s_and_is_created_when_named() {
    let tmp = TempDir::new().expect("temp dir");
    let (claude_home, root) = installed_plugin(&tmp);
    let codex_home = tmp.path().join("codex");
    let data = codex_home.join("plugins/data/demo-market");

    // Without Codex's home the data directory is unknown and stays as written.
    assert_eq!(
        load(&claude_home, &root).substitute_in_content("${CLAUDE_PLUGIN_DATA}/cache"),
        "${CLAUDE_PLUGIN_DATA}/cache"
    );
    let plugin = load(&claude_home, &root).with_codex_home(&codex_home);
    assert!(!data.exists());
    assert_eq!(
        plugin.substitute_in_content(
            "Read ${CLAUDE_PLUGIN_ROOT}/docs, keep notes in ${CLAUDE_PLUGIN_DATA}, at ${user_config.region}."
        ),
        format!(
            "Read {}/docs, keep notes in {}, at eu.",
            root.display(),
            data.display()
        )
    );
    assert!(data.is_dir());
    assert_eq!(
        plugin.server_env(),
        vec![
            ("CLAUDE_PLUGIN_ROOT".to_string(), root.display().to_string()),
            ("CLAUDE_PLUGIN_DATA".to_string(), data.display().to_string()),
        ]
    );
}

#[test]
fn a_plugin_s_skill_gets_its_variables_and_other_files_stay_as_written() {
    let tmp = TempDir::new().expect("temp dir");
    let (_, root) = installed_plugin(&tmp);
    let text = "Run ${CLAUDE_PLUGIN_ROOT}/bin/check.".to_string();

    assert_eq!(
        super::substitute_in_plugin_content(text.clone(), &root.join("skills/x/SKILL.md")),
        format!("Run {}/bin/check.", root.display())
    );
    assert_eq!(
        super::substitute_in_plugin_content(
            text.clone(),
            &tmp.path().join("home/.claude/skills/x/SKILL.md")
        ),
        text
    );
}

#[test]
fn a_directory_without_a_manifest_is_no_plugin() {
    let tmp = TempDir::new().expect("temp dir");
    assert_eq!(PluginVariables::load_with_home(tmp.path(), None), None);
    assert_eq!(plugin_root_of(&tmp.path().join("agents/a.md")), None);
}

#[test]
fn plugin_agents_are_named_after_the_plugin_and_their_directories() {
    let tmp = TempDir::new().expect("temp dir");
    let (_, root) = installed_plugin(&tmp);
    let nested = root.join("extra/review/security.md");
    fs::write(
        &nested,
        "Look for injection in ${CLAUDE_PLUGIN_ROOT}/rules.\n",
    )
    .expect("write");
    let named = root.join("extra/helper.md");
    fs::write(
        &named,
        "---\nname: assistant\ndescription: Helps.\ntools: Read\n---\nHelp.\n",
    )
    .expect("write");

    let security = parse_subagent_at(&fs::read_to_string(&nested).expect("read"), &nested)
        .expect("plugin agent without frontmatter");
    assert_eq!(security.name, "demo:review:security");
    assert_eq!(security.description, "Agent from demo plugin");
    assert_eq!(
        security.prompt,
        format!("Look for injection in {}/rules.", root.display())
    );

    let helper = parse_subagent_at(&fs::read_to_string(&named).expect("read"), &named)
        .expect("plugin agent");
    assert_eq!(
        (helper.name.as_str(), helper.description.as_str()),
        ("demo:assistant", "Helps.")
    );

    // A file of the plugin outside its agent directories is an ordinary subagent file, and
    // the manifest's `agents` list stands in place of `agents/`, as in Claude Code.
    let outside = root.join("docs/note.md");
    assert_eq!(parse_subagent_at("Just a note.\n", &outside), None);
    let replaced = root.join("agents/old.md");
    assert_eq!(
        parse_subagent_at("An agent no longer listed.\n", &replaced),
        None
    );
}
