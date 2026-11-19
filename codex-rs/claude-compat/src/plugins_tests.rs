use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use serde_json::json;

use super::ClaudePlugin;
use super::applicable_plugins;
use super::enabled_plugin_ids;
use super::incompatibility;
use super::installed_enabled_plugins;
use super::installed_plugin_paths;
use super::load_claude_plugins;

fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("parent")).expect("create dir");
    fs::write(path, contents).expect("write file");
}

#[test]
fn install_path_is_the_first_installation() {
    let installed = json!({
        "version": 2,
        "plugins": {
            "hud@hud": [
                {"scope": "project", "projectPath": "/p", "installPath": "/cache/hud/1.0.0"},
                {"scope": "user", "installPath": "/cache/hud/0.9.0"}
            ],
            "empty@m": [],
            "no-path@m": [{"scope": "user"}]
        }
    });

    assert_eq!(
        installed_plugin_paths(&installed),
        BTreeMap::from([("hud@hud".to_string(), PathBuf::from("/cache/hud/1.0.0"))])
    );
}

#[test]
fn only_true_enables_and_later_files_win() {
    let user = json!({"enabledPlugins": {
        "a@m": true,
        "b@m": true,
        "c@m": false,
        "d@m": "yes",
        "no-marketplace": true,
        "x@builtin": true
    }});
    let project = json!({"enabledPlugins": {"b@m": false, "c@m": true, "e@m": true}});
    let local = json!({"enabledPlugins": {"e@m": true}});

    assert_eq!(
        enabled_plugin_ids(&[user, project, local], /*first_is_user*/ true),
        BTreeMap::from([
            ("a@m".to_string(), true),
            ("c@m".to_string(), false),
            ("e@m".to_string(), false),
        ])
    );
}

#[test]
fn enabled_but_not_installed_is_reported() {
    let installed = json!({"version": 2, "plugins": {
        "a@m": [{"scope": "user", "installPath": "/cache/a"}],
        "unused@m": [{"scope": "user", "installPath": "/cache/unused"}]
    }});
    let settings = [json!({"enabledPlugins": {"a@m": true, "gone@m": true}})];

    let (plugins, warnings) =
        installed_enabled_plugins(&installed, &settings, /*first_is_user*/ true);

    assert_eq!(
        plugins,
        vec![ClaudePlugin {
            id: "a@m".to_string(),
            install_path: PathBuf::from("/cache/a"),
            enabled_by_user: true,
        }]
    );
    assert_eq!(
        warnings,
        vec!["Claude Code plugin gone@m skipped: it is enabled but not installed".to_string()]
    );
}

#[test]
fn loads_records_and_settings_from_disk() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let claude_home = tmp.path().join("home/.claude");
    let project = tmp.path().join("project");
    write(
        &claude_home.join("plugins/installed_plugins.json"),
        r#"{"version": 2, "plugins": {
            "a@m": [{"scope": "user", "installPath": "/cache/a"}],
            "b@m": [{"scope": "project", "projectPath": "/project", "installPath": "/cache/b"}]
        }}"#,
    );
    write(
        &claude_home.join("settings.json"),
        r#"{"enabledPlugins": {"a@m": true}}"#,
    );
    write(
        &project.join(".claude/settings.json"),
        r#"{"enabledPlugins": {"b@m": true}}"#,
    );

    let (plugins, warnings) = load_claude_plugins(&claude_home, Some(&project));

    assert_eq!(
        plugins,
        vec![
            ClaudePlugin {
                id: "a@m".to_string(),
                install_path: PathBuf::from("/cache/a"),
                enabled_by_user: true,
            },
            ClaudePlugin {
                id: "b@m".to_string(),
                install_path: PathBuf::from("/cache/b"),
                enabled_by_user: false,
            },
        ]
    );
    assert_eq!(warnings, Vec::<String>::new());
}

#[test]
fn project_settings_alone_are_not_the_users() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let claude_home = tmp.path().join(".claude");
    let project = tmp.path().join("project");
    write(
        &claude_home.join("plugins/installed_plugins.json"),
        r#"{"version": 2, "plugins": {"b@m": [{"scope": "user", "installPath": "/cache/b"}]}}"#,
    );
    write(
        &project.join(".claude/settings.local.json"),
        r#"{"enabledPlugins": {"b@m": true}}"#,
    );

    let (plugins, _) = load_claude_plugins(&claude_home, Some(&project));

    assert_eq!(
        plugins,
        vec![ClaudePlugin {
            id: "b@m".to_string(),
            install_path: PathBuf::from("/cache/b"),
            enabled_by_user: false,
        }]
    );
}

#[test]
fn no_record_means_no_plugins() {
    let tmp = tempfile::tempdir().expect("tempdir");

    assert_eq!(
        load_claude_plugins(tmp.path(), /*project_dir*/ None),
        (Vec::new(), Vec::new())
    );
}

/// claude-hud 0.8.0's shape: a manifest that declares only commands, whose `commands/setup.md`
/// merges a `statusLine` into Claude Code's `settings.json`, and a `README.md` and compiled
/// `dist/` that are not read.
fn write_claude_hud(root: &Path) {
    write(
        &root.join(".claude-plugin/plugin.json"),
        r#"{
            "name": "claude-hud",
            "version": "0.8.0",
            "commands": ["./commands/setup.md", "./commands/configure.md"]
        }"#,
    );
    write(
        &root.join("commands/setup.md"),
        r#"## Step 3: Apply Configuration

Read the settings file and merge in the statusLine config, preserving all existing settings:
- **Platform `darwin` or `linux`**: `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/settings.json`

```json
{
  "statusLine": {
    "type": "command",
    "command": "{GENERATED_COMMAND}"
  }
}
```
"#,
    );
    write(
        &root.join("commands/configure.md"),
        "Edit ~/.claude/plugins/claude-hud/config.json.",
    );
    write(&root.join("dist/index.js"), "// reads the statusLine input");
    write(
        &root.join("README.md"),
        "Adds a statusLine to settings.json.",
    );
}

#[test]
fn claude_hud_configures_the_status_line() {
    let tmp = tempfile::tempdir().expect("tempdir");
    write_claude_hud(tmp.path());

    assert_eq!(
        incompatibility(tmp.path()),
        Some(format!(
            "its {} configures Claude Code's `statusLine` setting, which Codex does not have",
            Path::new("commands/setup.md").display()
        ))
    );
}

#[test]
fn simple_plugins_are_compatible() {
    let tmp = tempfile::tempdir().expect("tempdir");
    // Only commands.
    let commands = tmp.path().join("commands-only");
    write(
        &commands.join(".claude-plugin/plugin.json"),
        r#"{"name": "c"}"#,
    );
    write(&commands.join("commands/run.md"), "Run `make test`.");
    // Only a script, which edits Claude Code's settings but no interface setting of them.
    let script = tmp.path().join("script-only");
    write(
        &script.join(".claude-plugin/plugin.json"),
        r#"{"name": "s"}"#,
    );
    write(
        &script.join("scripts/allow.sh"),
        "jq '.permissions.allow += [\"Bash(ls)\"]' ~/.claude/settings.json",
    );
    // Nothing at all.
    let empty = tmp.path().join("empty");
    write(
        &empty.join(".claude-plugin/plugin.json"),
        r#"{"name": "e"}"#,
    );
    // A skill that names an interface setting, but no settings file.
    let skill = tmp.path().join("skill");
    write(
        &skill.join(".claude-plugin/plugin.json"),
        r#"{"name": "k"}"#,
    );
    write(
        &skill.join("skills/style/SKILL.md"),
        "Explain how outputStyles work in a plugin manifest; mention the statusLineX field.",
    );

    for root in [commands, script, empty, skill] {
        assert_eq!(incompatibility(&root), None, "{}", root.display());
    }
}

#[test]
fn interface_settings_must_be_whole_words() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();
    write(&root.join(".claude-plugin/plugin.json"), r#"{"name": "p"}"#);
    write(
        &root.join("skills/x/SKILL.md"),
        "Read settings.json; the field mystatusLine and outputStyles are not settings.",
    );
    assert_eq!(incompatibility(root), None);

    write(
        &root.join("hooks/set.py"),
        "s = json.load(open('settings.local.json'))\ns['outputStyle'] = 'terse'",
    );
    assert_eq!(
        incompatibility(root),
        Some(format!(
            "its {} configures Claude Code's `outputStyle` setting, which Codex does not have",
            Path::new("hooks/set.py").display()
        ))
    );
}

#[test]
fn readmes_hidden_and_vendored_files_are_not_read() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();
    let text = "Set statusLine in settings.json.";
    write(&root.join("skills/README.md"), text);
    write(&root.join("scripts/.cache/x.sh"), text);
    write(&root.join("scripts/node_modules/dep/index.js"), text);
    write(&root.join("src/index.ts"), text);

    assert_eq!(incompatibility(root), None);
}

#[test]
fn declared_paths_are_read_too() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path();
    write(
        &root.join(".claude-plugin/plugin.json"),
        r#"{"name": "p", "commands": ["./cmds/setup.md"], "skills": "../outside"}"#,
    );
    write(
        &root.join("cmds/setup.md"),
        "Add spinnerVerbs to ~/.claude/settings.json.",
    );

    assert_eq!(
        incompatibility(root),
        Some(format!(
            "its {} configures Claude Code's `spinnerVerbs` setting, which Codex does not have",
            Path::new("cmds/setup.md").display()
        ))
    );
}

#[test]
fn interface_components_are_incompatible() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let styles = tmp.path().join("styles");
    write(&styles.join("output-styles/terse.md"), "");
    let declared_styles = tmp.path().join("declared-styles");
    write(
        &declared_styles.join(".claude-plugin/plugin.json"),
        r#"{"name": "p", "outputStyles": "./styles"}"#,
    );
    write(&declared_styles.join("styles/terse.md"), "");
    let lsp = tmp.path().join("lsp");
    write(
        &lsp.join(".claude-plugin/plugin.json"),
        r#"{"name": "p", "lspServers": {"rust": {"command": "rust-analyzer"}}}"#,
    );
    let lsp_file = tmp.path().join("lsp-file");
    write(&lsp_file.join(".lsp.json"), "{}");
    let channels = tmp.path().join("channels");
    write(
        &channels.join(".claude-plugin/plugin.json"),
        r#"{"name": "p", "channels": [{"server": "telegram"}]}"#,
    );

    let reason = |component: &str| {
        Some(format!(
            "it declares {component}, which Codex does not have"
        ))
    };
    assert_eq!(incompatibility(&styles), reason("output styles"));
    assert_eq!(incompatibility(&declared_styles), reason("output styles"));
    assert_eq!(incompatibility(&lsp), reason("LSP servers"));
    assert_eq!(incompatibility(&lsp_file), reason("LSP servers"));
    assert_eq!(incompatibility(&channels), reason("channels"));
}

#[test]
fn applicable_plugins_disable_incompatible_ones_unless_enabled() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let commands = tmp.path().join("commands");
    write(
        &commands.join(".claude-plugin/plugin.json"),
        r#"{"name": "commands"}"#,
    );
    write(&commands.join("commands/run.md"), "");
    let hud = tmp.path().join("hud");
    write_claude_hud(&hud);
    let lsp = tmp.path().join("lsp");
    write(
        &lsp.join(".claude-plugin/plugin.json"),
        r#"{"name": "lsp", "lspServers": {"rust": {"command": "rust-analyzer"}}}"#,
    );
    let no_manifest = tmp.path().join("no-manifest");
    write(&no_manifest.join("hooks/hooks.json"), r#"{"hooks": {}}"#);
    let plugin = |id: &str, path: &Path| ClaudePlugin {
        id: id.to_string(),
        install_path: path.to_path_buf(),
        enabled_by_user: true,
    };
    let plugins = vec![
        plugin("commands@m", &commands),
        plugin("claude-hud@claude-hud", &hud),
        plugin("lsp@m", &lsp),
        plugin("no-manifest@m", &no_manifest),
        plugin("gone@m", &tmp.path().join("gone")),
    ];

    let (applicable, warnings) = applicable_plugins(plugins.clone(), &[]);

    assert_eq!(applicable, vec![plugin("commands@m", &commands)]);
    let override_hint = |id: &str| {
        format!(
            "To use it anyway, add \"{id}\" to `enable` under `[claude.plugins]` in \
             $CODEX_HOME/ccn-config.toml"
        )
    };
    assert_eq!(
        warnings,
        vec![
            format!(
                "Claude Code plugin claude-hud@claude-hud disabled: its {} configures Claude \
                 Code's `statusLine` setting, which Codex does not have. {}",
                Path::new("commands/setup.md").display(),
                override_hint("claude-hud@claude-hud")
            ),
            format!(
                "Claude Code plugin lsp@m disabled: it declares LSP servers, which Codex does not \
                 have. {}",
                override_hint("lsp@m")
            ),
            "Claude Code plugin no-manifest@m skipped: Codex needs a readable .claude-plugin/plugin.json"
                .to_string(),
            format!(
                "Claude Code plugin gone@m skipped: its install path {} does not exist",
                tmp.path().join("gone").display()
            ),
        ]
    );

    let (applicable, warnings) =
        applicable_plugins(plugins, &["claude-hud@claude-hud".to_string()]);

    assert_eq!(
        applicable,
        vec![
            plugin("commands@m", &commands),
            plugin("claude-hud@claude-hud", &hud),
        ]
    );
    assert_eq!(warnings.len(), 3);
}
