// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use serde_json::json;

use super::auto_memory_base_dir;
use super::auto_memory_dir_override;
use super::auto_memory_enabled;
use super::auto_memory_instructions;

type Env = Box<dyn Fn(&str) -> Option<String>>;

fn env_of(vars: &[(&str, &str)]) -> Env {
    let vars = vars
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect::<HashMap<_, _>>();
    Box::new(move |name| vars.get(name).cloned())
}

#[test]
fn instructions_name_the_directory_with_a_trailing_separator() {
    let text = auto_memory_instructions(Path::new("/h/.claude/projects/-p/memory"), &env_of(&[]));
    assert!(text.starts_with(
        "# auto memory\n\nYou have a persistent, file-based memory system at `/h/.claude/projects/-p/memory/`. This directory already exists — write to it directly with the apply_patch tool"
    ));
    assert!(text.contains("**Step 2** — add a pointer to that file in `MEMORY.md`."));
    assert!(text.contains("type: {{user, feedback, project, reference}}"));
    assert!(text.ends_with(
        "memory should be reserved for information that will be useful in future conversations."
    ));
    assert!(!text.contains("{{MEMORY_DIR}}"));

    let same = auto_memory_instructions(Path::new("/h/.claude/projects/-p/memory/"), &env_of(&[]));
    assert_eq!(same, text);
}

#[test]
fn extra_guidelines_follow_the_instructions() {
    let plain = auto_memory_instructions(Path::new("/m"), &env_of(&[]));
    let extra = auto_memory_instructions(
        Path::new("/m"),
        &env_of(&[(
            "CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES",
            "Never save secrets.",
        )]),
    );
    assert_eq!(extra, format!("{plain}\n\nNever save secrets."));
    let blank = auto_memory_instructions(
        Path::new("/m"),
        &env_of(&[("CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES", "  \n")]),
    );
    assert_eq!(blank, plain);
}

#[test]
fn enabled_by_default_and_by_the_last_setting() {
    let off = json!({ "autoMemoryEnabled": false });
    let on = json!({ "autoMemoryEnabled": true });
    let other = json!({ "model": "opus" });
    assert!(auto_memory_enabled(&[], &env_of(&[])));
    assert!(auto_memory_enabled(&[&other], &env_of(&[])));
    assert!(!auto_memory_enabled(&[&off], &env_of(&[])));
    assert!(!auto_memory_enabled(&[&on, &off, &other], &env_of(&[])));
    assert!(auto_memory_enabled(&[&off, &on], &env_of(&[])));
}

#[test]
fn environment_switches_come_first() {
    let off = json!({ "autoMemoryEnabled": false });
    let on = json!({ "autoMemoryEnabled": true });
    let disable = |value: &str| env_of(&[("CLAUDE_CODE_DISABLE_AUTO_MEMORY", value)]);
    assert!(!auto_memory_enabled(&[&on], &disable("1")));
    assert!(!auto_memory_enabled(&[&on], &disable(" TRUE ")));
    assert!(auto_memory_enabled(&[&off], &disable("0")));
    assert!(auto_memory_enabled(&[&off], &disable("off")));
    assert!(!auto_memory_enabled(&[&off], &disable("maybe")));
    assert!(!auto_memory_enabled(
        &[&on],
        &env_of(&[("CLAUDE_CODE_SIMPLE", "1")])
    ));
    assert!(auto_memory_enabled(
        &[],
        &env_of(&[
            ("CLAUDE_CODE_SIMPLE", "1"),
            ("CLAUDE_CODE_DISABLE_AUTO_MEMORY", "false")
        ])
    ));
    assert!(!auto_memory_enabled(
        &[],
        &env_of(&[("CLAUDE_CODE_REMOTE", "true")])
    ));
    assert!(auto_memory_enabled(
        &[],
        &env_of(&[
            ("CLAUDE_CODE_REMOTE", "true"),
            ("CLAUDE_CODE_REMOTE_MEMORY_DIR", "/mnt/memory")
        ])
    ));
}

#[test]
fn base_dir_is_claude_home_unless_remote_storage_is_set() {
    let claude_home = Path::new("/h/.claude");
    assert_eq!(
        auto_memory_base_dir(claude_home, &env_of(&[])),
        PathBuf::from("/h/.claude")
    );
    assert_eq!(
        auto_memory_base_dir(
            claude_home,
            &env_of(&[("CLAUDE_CODE_REMOTE_MEMORY_DIR", "/mnt/memory")])
        ),
        PathBuf::from("/mnt/memory")
    );
}

#[test]
fn directory_setting_expands_home_and_normalizes() {
    let home = Some(Path::new("/home/u"));
    let dir = |raw: &str| {
        let settings = json!({ "autoMemoryDirectory": raw });
        auto_memory_dir_override(&[&settings], home, &env_of(&[]))
    };
    assert_eq!(
        dir("~/notes/memory"),
        Some(PathBuf::from("/home/u/notes/memory"))
    );
    assert_eq!(dir("/srv/a/../memory/"), Some(PathBuf::from("/srv/memory")));
    assert_eq!(dir("~/"), None);
    assert_eq!(dir("~/."), None);
    assert_eq!(dir("~/a/../.."), None);
    assert_eq!(dir("relative/memory"), None);
    assert_eq!(dir("/"), None);
    assert_eq!(dir("/a"), None);
    assert_eq!(dir(""), None);
    assert_eq!(auto_memory_dir_override(&[], home, &env_of(&[])), None);
    let settings = json!({ "autoMemoryDirectory": "~/memory" });
    assert_eq!(
        auto_memory_dir_override(&[&settings], None, &env_of(&[])),
        None
    );
}

#[test]
fn highest_setting_wins_even_when_invalid() {
    let home = Some(Path::new("/home/u"));
    let user = json!({ "autoMemoryDirectory": "/srv/user-memory" });
    let local = json!({ "autoMemoryDirectory": "not/absolute" });
    let unset = json!({});
    assert_eq!(
        auto_memory_dir_override(&[&user, &local], home, &env_of(&[])),
        None
    );
    assert_eq!(
        auto_memory_dir_override(&[&user, &unset], home, &env_of(&[])),
        Some(PathBuf::from("/srv/user-memory"))
    );
}

#[test]
fn environment_override_comes_before_settings() {
    let home = Some(Path::new("/home/u"));
    let settings = json!({ "autoMemoryDirectory": "/srv/settings-memory" });
    assert_eq!(
        auto_memory_dir_override(
            &[&settings],
            home,
            &env_of(&[("CLAUDE_COWORK_MEMORY_PATH_OVERRIDE", "/space/memory")])
        ),
        Some(PathBuf::from("/space/memory"))
    );
    // The variable is not expanded; an invalid one leaves the settings in charge.
    assert_eq!(
        auto_memory_dir_override(
            &[&settings],
            home,
            &env_of(&[("CLAUDE_COWORK_MEMORY_PATH_OVERRIDE", "~/memory")])
        ),
        Some(PathBuf::from("/srv/settings-memory"))
    );
}
