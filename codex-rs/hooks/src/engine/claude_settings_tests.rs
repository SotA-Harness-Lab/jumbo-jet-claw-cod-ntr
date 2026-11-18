use std::fs;

use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::ClaudeHookSettings;
use super::load;

fn absolute(path: std::path::PathBuf) -> AbsolutePathBuf {
    AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute path")
}

fn settings(tmp: &TempDir) -> ClaudeHookSettings {
    ClaudeHookSettings {
        user_dir: Some(absolute(tmp.path().join("home/.claude"))),
        project_dir: Some(absolute(tmp.path().join("project"))),
        entrypoint: Some("cli".to_string()),
    }
}

#[test]
fn loads_user_and_project_hooks_and_drops_unknown_handler_types() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::create_dir_all(tmp.path().join("project/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{
            "model": "opus",
            "hooks": {
                "PreToolUse": [{"matcher": "Bash", "hooks": [
                    {"type": "command", "command": "check.sh", "timeout": 5},
                    {"type": "http", "url": "https://example.invalid"}
                ]}],
                "Notification": [{"hooks": [{"type": "command", "command": "notify.sh"}]}]
            }
        }"#,
    )
    .expect("write");
    fs::write(
        tmp.path().join("project/.claude/settings.local.json"),
        r#"{"hooks": {"Stop": [{"hooks": [{"type": "command", "command": "stop.sh"}]}]}}"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    let loaded = load(&settings(&tmp), &mut warnings);

    assert_eq!(
        loaded
            .iter()
            .map(|hooks| (
                hooks.path.as_path().to_path_buf(),
                hooks.is_user,
                hooks.events.pre_tool_use.len(),
                hooks.events.stop.len()
            ))
            .collect::<Vec<_>>(),
        vec![
            (tmp.path().join("home/.claude/settings.json"), true, 1, 0),
            (
                tmp.path().join("project/.claude/settings.local.json"),
                false,
                0,
                1
            ),
        ]
    );
    assert_eq!(loaded[0].events.pre_tool_use[0].hooks.len(), 1);
    // Notification has no place in Codex's own configuration and is kept apart.
    assert_eq!(
        loaded[0]
            .claude_only
            .iter()
            .map(|(event, groups)| (event.label(), groups.len()))
            .collect::<Vec<_>>(),
        vec![("Notification", 1)]
    );
    assert_eq!(warnings.len(), 1, "{warnings:?}");
    assert!(warnings[0].contains("of type http"), "{warnings:?}");
}

#[test]
fn disable_all_hooks_turns_every_file_off() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::create_dir_all(tmp.path().join("project/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{"hooks": {"Stop": [{"hooks": [{"type": "command", "command": "stop.sh"}]}]}}"#,
    )
    .expect("write");
    fs::write(
        tmp.path().join("project/.claude/settings.json"),
        r#"{"disableAllHooks": true}"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    assert!(load(&settings(&tmp), &mut warnings).is_empty());
    assert_eq!(warnings, Vec::<String>::new());
}

#[test]
fn a_file_with_only_claude_only_events_is_loaded() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{"hooks": {
            "StopFailure": [{"hooks": [{"type": "command", "command": "a.sh"}]}],
            "PostToolUseFailure": [{"matcher": "Bash", "hooks": [{"type": "command", "command": "b.sh"}]}]
        }}"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    let loaded = load(&settings(&tmp), &mut warnings);

    assert_eq!(warnings, Vec::<String>::new());
    assert_eq!(loaded.len(), 1);
    assert!(loaded[0].events.is_empty());
    assert_eq!(
        loaded[0]
            .claude_only
            .iter()
            .map(|(event, groups)| (event.label(), groups[0].matcher.clone()))
            .collect::<Vec<_>>(),
        vec![
            ("PostToolUseFailure", Some("Bash".to_string())),
            ("StopFailure", None),
        ]
    );
}
