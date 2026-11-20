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
    fs::create_dir_all(root.join("agents/review")).expect("create");
    fs::create_dir_all(root.join("extra")).expect("create");
    fs::write(
        root.join(".claude-plugin/plugin.json"),
        r#"{
            "name": "demo",
            "agents": ["./extra"],
            "userConfig": {
                "api_url": {"type": "string", "title": "URL", "description": "d"},
                "api-token": {"type": "string", "title": "Token", "description": "d", "sensitive": true},
                "retries": {"type": "number", "title": "Retries", "description": "d"},
                "unset": {"type": "string", "title": "Unset", "description": "d"}
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
        r#"{"pluginConfigs": {"demo@market": {"options": {"api_url": "https://api.example.com", "retries": 3}}}}"#,
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
            ("CLAUDE_PLUGIN_OPTION_API_TOKEN".to_string(), "s3cret".to_string()),
            (
                "CLAUDE_PLUGIN_OPTION_API_URL".to_string(),
                "https://api.example.com".to_string()
            ),
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
        plugin.substitute("${CLAUDE_PLUGIN_ROOT}/run --url ${user_config.api_url} -t ${user_config.api-token}"),
        Ok(format!(
            "{}/run --url https://api.example.com -t s3cret",
            root.display()
        ))
    );
    assert_eq!(
        plugin.substitute("run ${user_config.unset}"),
        Err("unset".to_string())
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
fn a_directory_without_a_manifest_is_no_plugin() {
    let tmp = TempDir::new().expect("temp dir");
    assert_eq!(PluginVariables::load_with_home(tmp.path(), None), None);
    assert_eq!(plugin_root_of(&tmp.path().join("agents/a.md")), None);
}

#[test]
fn plugin_agents_are_named_after_the_plugin_and_their_directories() {
    let tmp = TempDir::new().expect("temp dir");
    let (_, root) = installed_plugin(&tmp);
    let nested = root.join("agents/review/security.md");
    fs::write(&nested, "Look for injection in ${CLAUDE_PLUGIN_ROOT}/rules.\n").expect("write");
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

    // A file of the plugin outside its agent directories is an ordinary subagent file.
    let outside = root.join("docs/note.md");
    assert_eq!(parse_subagent_at("Just a note.\n", &outside), None);
}
