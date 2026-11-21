// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;

use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::ClaudeHandlerKind;
use super::ClaudeHookSettings;
use super::ClaudeSettingsStamp;
use super::hook_conditions;
use super::load;
use super::plugin_handler_extras;
use super::settings_http_hook_policy;

fn absolute(path: std::path::PathBuf) -> AbsolutePathBuf {
    AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute path")
}

fn settings(tmp: &TempDir) -> ClaudeHookSettings {
    ClaudeHookSettings {
        user_dir: Some(absolute(tmp.path().join("home/.claude"))),
        project_dir: Some(absolute(tmp.path().join("project"))),
        entrypoint: Some("cli".to_string()),
        plugins: Vec::new(),
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
                    {"type": "webhook", "url": "https://example.invalid"}
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
    assert!(warnings[0].contains("of type webhook"), "{warnings:?}");
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

#[test]
fn conditions_are_kept_by_the_position_handlers_load_with() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{
            "hooks": {
                "PreToolUse": [
                    {"matcher": "Bash", "hooks": [
                        {"type": "webhook", "url": "https://example.invalid", "if": "Bash(curl *)"},
                        {"type": "command", "command": "plain.sh"},
                        {"type": "command", "command": "git.sh", "if": "Bash(git *)"}
                    ]},
                    {"hooks": [{"type": "command", "command": "edit.sh", "if": "Edit(*.rs)"}]}
                ],
                "PostToolUseFailure": [
                    {"hooks": [{"type": "command", "command": "failed.sh", "if": "Bash"}]}
                ]
            }
        }"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    let loaded = load(&settings(&tmp), &mut warnings);

    // The handler of an unknown type is dropped before positions are counted.
    let mut conditions = loaded[0]
        .extras
        .conditions
        .iter()
        .map(|((event, group, handler), condition)| {
            (event.as_str(), *group, *handler, condition.as_str())
        })
        .collect::<Vec<_>>();
    conditions.sort();
    assert_eq!(
        conditions,
        vec![
            ("PostToolUseFailure", 0, 0, "Bash"),
            ("PreToolUse", 0, 1, "Bash(git *)"),
            ("PreToolUse", 1, 0, "Edit(*.rs)"),
        ]
    );
}

#[test]
fn a_hooks_value_that_is_not_an_object_has_no_conditions() {
    assert_eq!(hook_conditions(&serde_json::json!([])).len(), 0);
}

#[test]
fn prompt_agent_and_http_handlers_keep_what_they_declare() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{
            "allowedHttpHookUrls": ["https://hooks.example.com/*"],
            "hooks": {
                "Stop": [{"hooks": [
                    {"type": "prompt", "prompt": "Did the tests run? $ARGUMENTS", "timeout": 12, "model": "claude-haiku-4-5"},
                    {"type": "agent", "prompt": "Verify the build."},
                    {"type": "command", "command": "plain.sh"}
                ]}],
                "PostToolUse": [{"matcher": "Bash", "hooks": [
                    {"type": "http", "url": "https://hooks.example.com/post",
                     "headers": {"Authorization": "Bearer $TOKEN"},
                     "allowedEnvVars": ["TOKEN"], "statusMessage": "Reporting"}
                ]}]
            }
        }"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    let loaded = load(&settings(&tmp), &mut warnings);

    assert_eq!(warnings, Vec::<String>::new());
    let handlers = &loaded[0].extras.handlers;
    assert_eq!(handlers.len(), 3);
    let prompt = &handlers[&("Stop".to_string(), 0, 0)];
    assert_eq!(
        (&prompt.kind, prompt.timeout_sec),
        (
            &ClaudeHandlerKind::Prompt {
                prompt: "Did the tests run? $ARGUMENTS".to_string(),
                model: Some("claude-haiku-4-5".to_string()),
            },
            Some(12)
        )
    );
    assert_eq!(
        handlers[&("Stop".to_string(), 0, 1)].kind,
        ClaudeHandlerKind::Agent {
            prompt: "Verify the build.".to_string(),
            model: None,
        }
    );
    let http = &handlers[&("PostToolUse".to_string(), 0, 0)];
    assert_eq!(http.status_message.as_deref(), Some("Reporting"));
    let ClaudeHandlerKind::Http(hook) = &http.kind else {
        panic!("expected an http handler, got {:?}", http.kind);
    };
    assert_eq!(hook.url, "https://hooks.example.com/post");
    assert_eq!(hook.allowed_env_vars, vec!["TOKEN".to_string()]);
    assert_eq!(
        hook.policy.allowed_urls,
        Some(vec!["https://hooks.example.com/*".to_string()])
    );
    assert_eq!(hook.policy.allowed_env_vars, None);
    // The `http` handler is parsed in the place of a `prompt` one.
    assert_eq!(loaded[0].events.post_tool_use[0].hooks.len(), 1);
}

#[test]
fn a_plugins_http_handlers_keep_what_they_declare_under_the_settings_policy() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{"allowedHttpHookUrls": ["https://hooks.example.com/*"]}"#,
    )
    .expect("write");
    let hooks_file = tmp.path().join("plugin/hooks/hooks.json");
    fs::create_dir_all(hooks_file.parent().expect("parent")).expect("create");
    fs::write(
        &hooks_file,
        r#"{"hooks": {"PostToolUse": [{"matcher": "Bash", "hooks": [
            {"type": "command", "command": "plain.sh"},
            {"type": "http", "url": "https://hooks.example.com/post", "timeout": 5}
        ]}]}}"#,
    )
    .expect("write");

    let extras = plugin_handler_extras(
        &absolute(hooks_file),
        &settings_http_hook_policy(&settings(&tmp)),
    );

    let http = &extras.handlers[&("PostToolUse".to_string(), 0, 1)];
    assert_eq!(http.timeout_sec, Some(5));
    let ClaudeHandlerKind::Http(hook) = &http.kind else {
        panic!("expected an http handler, got {:?}", http.kind);
    };
    assert_eq!(hook.url, "https://hooks.example.com/post");
    assert_eq!(
        hook.policy.allowed_urls,
        Some(vec!["https://hooks.example.com/*".to_string()])
    );
}

#[test]
fn a_malformed_entry_is_left_out_and_the_rest_of_the_file_kept() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{"hooks": {
            "PreToolUse": [
                {"matcher": "Bash", "hooks": [
                    {"type": "command"},
                    {"type": "command", "command": "guard.sh", "timeout": 1.5, "if": "Bash(rm *)"}
                ]},
                {"matcher": 5, "hooks": [{"type": "command", "command": "other.sh"}]}
            ],
            "Stop": {"hooks": []},
            "SessionStart": [{"hooks": [{"type": "command", "command": "start.sh", "timeout": "30"}]}],
            "UserPromptSubmit": [{"hooks": [{"type": "command", "command": "prompt.sh"}]}]
        }}"#,
    )
    .expect("write");

    let mut warnings = Vec::new();
    let loaded = load(&settings(&tmp), &mut warnings);

    assert_eq!(warnings.len(), 4, "{warnings:?}");
    assert_eq!(loaded.len(), 1);
    let events = &loaded[0].events;
    assert_eq!(
        (
            events.pre_tool_use.len(),
            events.session_start.len(),
            events.user_prompt_submit.len()
        ),
        (1, 1, 1)
    );
    assert_eq!(events.session_start[0].hooks.len(), 0);
    assert_eq!(
        events.pre_tool_use[0].hooks,
        vec![codex_config::HookHandlerConfig::Command {
            command: "guard.sh".to_string(),
            command_windows: None,
            timeout_sec: Some(2),
            r#async: false,
            status_message: None,
            additional_context_limit: None,
        }]
    );
    // The condition stays with its handler, now the first of its group.
    assert_eq!(
        loaded[0]
            .extras
            .conditions
            .get(&("PreToolUse".to_string(), 0, 0))
            .map(String::as_str),
        Some("Bash(rm *)")
    );
}

#[test]
fn a_stamp_tells_when_a_settings_file_changed() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::create_dir_all(tmp.path().join("project")).expect("create");
    fs::write(tmp.path().join("home/.claude/settings.json"), "{}").expect("write");
    let settings = settings(&tmp);

    let stamp = ClaudeSettingsStamp::of(&settings);
    assert!(!stamp.changed());

    // A project file created during the session, its `.claude` folder with it.
    fs::create_dir_all(tmp.path().join("project/.claude")).expect("create");
    fs::write(
        tmp.path().join("project/.claude/settings.local.json"),
        r#"{"hooks": {}}"#,
    )
    .expect("write");
    assert!(stamp.changed());

    let stamp = ClaudeSettingsStamp::of(&settings);
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        r#"{"disableAllHooks": true}"#,
    )
    .expect("write");
    assert!(stamp.changed());
}
