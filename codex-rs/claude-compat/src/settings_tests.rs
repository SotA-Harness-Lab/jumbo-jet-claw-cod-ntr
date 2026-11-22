// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use serde_json::json;

use super::BashPrefixRule;
use super::PermissionDecision;
use super::bash_prefix_rules;
use super::local_settings_paths;
use super::read_settings;
use super::settings_effort_level;
use super::settings_env;
use super::settings_paths;
use super::settings_reply_language;
use super::settings_stamp;

fn rule(prefix: &[&str], decision: PermissionDecision) -> BashPrefixRule {
    BashPrefixRule {
        prefix: prefix.iter().map(ToString::to_string).collect(),
        decision,
    }
}

#[test]
fn bash_rules_become_prefixes() {
    let settings = json!({
        "permissions": {
            "allow": ["Bash(git status)", "Bash(git diff*)", "Bash(gh pr *)", "Bash(npm run:*)", "Read(~/x)"],
            "ask": ["Bash(rm *)"],
            "deny": ["Bash(git push --force*)", "Bash(curl * | sh)", "Bash"]
        }
    });

    let (rules, unsupported) = bash_prefix_rules(&settings);

    assert_eq!(
        rules,
        vec![
            rule(&["git", "status"], PermissionDecision::Allow),
            rule(&["git", "diff"], PermissionDecision::Allow),
            rule(&["gh", "pr"], PermissionDecision::Allow),
            rule(&["npm", "run"], PermissionDecision::Allow),
            rule(&["rm"], PermissionDecision::Ask),
            rule(&["git", "push", "--force"], PermissionDecision::Deny),
        ]
    );
    assert_eq!(unsupported, vec!["Read(~/x)", "Bash(curl * | sh)", "Bash"]);
}

#[test]
fn exec_policy_does_not_import_approvals_that_need_the_full_command() {
    let settings = json!({
        "permissions": {
            "allow": ["Bash(find *)", "Bash(watch -n 5 date)", "Bash(git status)"],
            "deny": ["Bash(find *)"]
        }
    });
    let (rules, unsupported) = bash_prefix_rules(&settings);
    assert_eq!(
        rules,
        vec![
            rule(&["git", "status"], PermissionDecision::Allow),
            rule(&["find"], PermissionDecision::Deny),
        ]
    );
    assert_eq!(unsupported, vec!["Bash(find *)", "Bash(watch -n 5 date)"]);
}

#[test]
fn reply_language_accepts_language_names_and_ignores_empty_or_non_string_values() {
    assert_eq!(
        settings_reply_language(&json!({"language": " 简体中文 "})),
        Some("简体中文")
    );
    assert_eq!(settings_reply_language(&json!({"language": "  "})), None);
    assert_eq!(settings_reply_language(&json!({"language": true})), None);
}

#[test]
fn env_values_are_strings() {
    let settings = json!({"env": {"A": "1", "B": 2, "C": true}});
    assert_eq!(
        settings_env(&settings),
        vec![
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string()),
            ("C".to_string(), "true".to_string()),
        ]
    );
}

#[test]
fn settings_paths_skip_the_home_directory_as_a_project() {
    let home = Path::new("/home/u/.claude");
    assert_eq!(
        settings_paths(Some(home), home.parent(), Some(Path::new("/home/u"))),
        vec![PathBuf::from("/home/u/.claude/settings.json")]
    );
    assert_eq!(
        settings_paths(Some(home), home.parent(), Some(Path::new("/work/p"))),
        vec![
            PathBuf::from("/home/u/.claude/settings.json"),
            PathBuf::from("/work/p/.claude/settings.json"),
            PathBuf::from("/work/p/.claude/settings.local.json"),
        ]
    );
}

#[test]
fn settings_with_a_byte_order_mark_are_read() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("settings.json");
    std::fs::write(&path, "\u{feff}{\"env\": {\"A\": \"1\"}}").unwrap();

    assert_eq!(
        read_settings(&path).unwrap(),
        Some(json!({"env": {"A": "1"}}))
    );
}

#[test]
fn settings_that_are_not_a_regular_file_count_as_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("settings.json");
    std::fs::create_dir(&path).unwrap();

    assert_eq!(read_settings(&path).unwrap(), None);
}

#[cfg(unix)]
#[test]
fn settings_linked_to_a_device_count_as_missing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(".mcp.json");
    std::os::unix::fs::symlink("/dev/null", &path).unwrap();

    assert_eq!(read_settings(&path).unwrap(), None);
}

#[test]
fn effort_levels_become_codex_efforts() {
    let effort = |value: serde_json::Value| settings_effort_level(&json!({ "effortLevel": value }));
    assert_eq!(effort(json!("low")), Some("low"));
    assert_eq!(effort(json!("medium")), Some("medium"));
    assert_eq!(effort(json!("high")), Some("high"));
    assert_eq!(effort(json!("xhigh")), Some("xhigh"));
    assert_eq!(effort(json!("max")), Some("xhigh"));
    assert_eq!(effort(json!("auto")), None);
    assert_eq!(effort(json!("High")), None);
    assert_eq!(effort(json!(3)), None);
    assert_eq!(settings_effort_level(&json!({})), None);
}

#[test]
fn the_local_file_of_the_repository_root_is_read_from_a_subdirectory() {
    let base = tempfile::tempdir().unwrap();
    let root = base.path().join("repo");
    let sub = root.join("a").join("b");
    std::fs::create_dir_all(root.join(".git")).unwrap();
    std::fs::create_dir_all(&sub).unwrap();
    let home = base.path().join("home");

    assert_eq!(
        local_settings_paths(&sub, Some(&home)),
        vec![
            sub.join(".claude").join("settings.local.json"),
            root.join(".claude").join("settings.local.json"),
        ]
    );
    // At the root itself the two are one file.
    assert_eq!(
        local_settings_paths(&root, Some(&home)),
        vec![root.join(".claude").join("settings.local.json")]
    );
    // Settings files come in Claude Code's order, the root's local file last.
    let claude_home = home.join(".claude");
    assert_eq!(
        settings_paths(Some(&claude_home), Some(&home), Some(&sub)),
        vec![
            claude_home.join("settings.json"),
            sub.join(".claude").join("settings.json"),
            sub.join(".claude").join("settings.local.json"),
            root.join(".claude").join("settings.local.json"),
        ]
    );
}

#[test]
fn the_local_file_stays_in_the_working_directory_outside_a_repository_or_at_home() {
    let base = tempfile::tempdir().unwrap();
    let dir = base.path().join("plain").join("sub");
    std::fs::create_dir_all(&dir).unwrap();
    assert_eq!(
        local_settings_paths(&dir, None),
        vec![dir.join(".claude").join("settings.local.json")]
    );

    // A home directory that is a repository keeps the file where the session starts.
    let home = base.path().join("home");
    let project = home.join("project");
    std::fs::create_dir_all(home.join(".git")).unwrap();
    std::fs::create_dir_all(&project).unwrap();
    assert_eq!(
        local_settings_paths(&project, Some(&home)),
        vec![project.join(".claude").join("settings.local.json")]
    );
}

#[test]
fn a_linked_worktree_reads_the_local_file_of_its_main_checkout() {
    let base = tempfile::tempdir().unwrap();
    let main = base.path().join("main");
    let worktree = base.path().join("wt");
    let worktree_git_dir = main.join(".git").join("worktrees").join("wt");
    std::fs::create_dir_all(&worktree_git_dir).unwrap();
    std::fs::create_dir_all(worktree.join("sub")).unwrap();
    std::fs::write(worktree_git_dir.join("commondir"), "../..\n").unwrap();
    std::fs::write(
        worktree_git_dir.join("gitdir"),
        format!("{}\n", worktree.join(".git").display()),
    )
    .unwrap();
    std::fs::write(
        worktree.join(".git"),
        format!("gitdir: {}\n", worktree_git_dir.display()),
    )
    .unwrap();

    let sub = worktree.join("sub");
    assert_eq!(
        local_settings_paths(&sub, None),
        vec![
            sub.join(".claude").join("settings.local.json"),
            main.join(".claude").join("settings.local.json"),
        ]
    );
}

#[test]
fn a_stamp_changes_when_a_settings_file_is_written_created_or_removed() {
    let dir = tempfile::tempdir().unwrap();
    let written = dir.path().join("settings.json");
    let created = dir.path().join("settings.local.json");
    std::fs::write(&written, "{}").unwrap();
    let paths = vec![written.clone(), created.clone()];
    let before = settings_stamp(&paths);
    assert_eq!(settings_stamp(&paths), before);

    std::fs::write(&written, "{\"env\": {}}").unwrap();
    let after_write = settings_stamp(&paths);
    assert!(after_write != before);

    std::fs::write(&created, "{}").unwrap();
    let after_create = settings_stamp(&paths);
    assert!(after_create != after_write);

    std::fs::remove_file(&created).unwrap();
    assert!(settings_stamp(&paths) != after_create);
}
