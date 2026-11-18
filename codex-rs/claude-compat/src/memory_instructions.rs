//! What Claude Code tells the model about writing its auto memory, and the switches and
//! settings that decide whether, and where, the memory is kept.

use std::ffi::OsStr;
use std::path::Component;
use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

/// `1`/`true` turns auto memory off, `0`/`false` on, whatever the settings say.
const DISABLE_ENV: &str = "CLAUDE_CODE_DISABLE_AUTO_MEMORY";
/// Claude Code's `--bare` mode, which has no auto memory.
const SIMPLE_ENV: &str = "CLAUDE_CODE_SIMPLE";
/// Claude Code on the web: no auto memory unless it has persistent storage.
const REMOTE_ENV: &str = "CLAUDE_CODE_REMOTE";
/// The directory that takes the place of `~/.claude` for memory.
const REMOTE_MEMORY_DIR_ENV: &str = "CLAUDE_CODE_REMOTE_MEMORY_DIR";
/// The full path of the memory directory, set by Claude Cowork.
const PATH_OVERRIDE_ENV: &str = "CLAUDE_COWORK_MEMORY_PATH_OVERRIDE";
/// Text appended to the instructions, set by Claude Cowork.
const EXTRA_GUIDELINES_ENV: &str = "CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES";

const ENABLED_SETTING: &str = "autoMemoryEnabled";
const DIRECTORY_SETTING: &str = "autoMemoryDirectory";

const MEMORY_DIR_PLACEHOLDER: &str = "{{MEMORY_DIR}}";

/// Claude Code's auto memory instructions (`buildMemoryLines` in its `memdir/memdir.ts`, as
/// Claude Code builds them by default: one directory, with the `MEMORY.md` index). Changed
/// for Codex: the model writes with `apply_patch` instead of the Write tool, `AGENTS.md`
/// is named beside `CLAUDE.md`, and the task list is Codex's `update_plan` tool.
const INSTRUCTIONS_TEMPLATE: &str = r#"# auto memory

You have a persistent, file-based memory system at `{{MEMORY_DIR}}`. This directory already exists — write to it directly with the apply_patch tool (do not run mkdir or check for its existence).

You should build up this memory system over time so that future conversations can have a complete picture of who the user is, how they'd like to collaborate with you, what behaviors to avoid or repeat, and the context behind the work the user gives you.

If the user explicitly asks you to remember something, save it immediately as whichever type fits best. If they ask you to forget something, find and remove the relevant entry.

## Types of memory

<types>
<type>
    <name>user</name>
    <description>The user's role, goals, preferences, responsibilities, and knowledge. Use these to tailor your behavior to the user.</description>
</type>
<type>
    <name>feedback</name>
    <description>Guidance from the user about how to approach work — what to avoid and what to keep doing. Record from failure AND success. Include *why* so you can judge edge cases later. Structure content as: rule/fact, then **Why:** and **How to apply:** lines.</description>
</type>
<type>
    <name>project</name>
    <description>Information about ongoing work, goals, initiatives, bugs, or incidents not derivable from code or git history. Convert relative dates to absolute dates when saving (e.g., "Thursday" → "2026-03-05").</description>
</type>
<type>
    <name>reference</name>
    <description>Pointers to external systems where information can be found (e.g., Linear projects, Slack channels, Grafana dashboards).</description>
</type>
</types>

## What NOT to save in memory

- Code patterns, conventions, architecture, file paths, or project structure — these can be derived by reading the current project state.
- Git history, recent changes, or who-changed-what — `git log` / `git blame` are authoritative.
- Debugging solutions or fix recipes — the fix is in the code; the commit message has the context.
- Anything already documented in CLAUDE.md or AGENTS.md files.
- Ephemeral task details: in-progress work, temporary state, current conversation context.

These exclusions apply even when the user explicitly asks you to save. If they ask you to save a PR list or activity summary, ask what was *surprising* or *non-obvious* about it — that is the part worth keeping.

## How to save memories

Saving a memory is a two-step process:

**Step 1** — write the memory to its own file (e.g., `user_role.md`, `feedback_testing.md`) using this frontmatter format:

```markdown
---
name: {{memory name}}
description: {{one-line description — used to decide relevance in future conversations, so be specific}}
type: {{user, feedback, project, reference}}
---

{{memory content — for feedback/project types, structure as: rule/fact, then **Why:** and **How to apply:** lines}}
```

**Step 2** — add a pointer to that file in `MEMORY.md`. `MEMORY.md` is an index, not a memory — each entry should be one line, under ~150 characters: `- [Title](file.md) — one-line hook`. It has no frontmatter. Never write memory content directly into `MEMORY.md`.

- `MEMORY.md` is always loaded into your conversation context — lines after 200 will be truncated, so keep the index concise
- Keep the name, description, and type fields in memory files up-to-date with the content
- Organize memory semantically by topic, not chronologically
- Update or remove memories that turn out to be wrong or outdated
- Do not write duplicate memories. First check if there is an existing memory you can update before writing a new one.

## When to access memories
- When memories seem relevant, or the user references prior-conversation work.
- You MUST access memory when the user explicitly asks you to check, recall, or remember.
- If the user says to *ignore* or *not use* memory: proceed as if MEMORY.md were empty. Do not apply remembered facts, cite, compare against, or mention memory content.
- Memory records can become stale over time. Use memory as context for what was true at a given point in time. Before answering the user or building assumptions based solely on information in memory records, verify that the memory is still correct and up-to-date by reading the current state of the files or resources. If a recalled memory conflicts with current information, trust what you observe now — and update or remove the stale memory rather than acting on it.

## Before recommending from memory

A memory that names a specific function, file, or flag is a claim that it existed *when the memory was written*. It may have been renamed, removed, or never merged. Before recommending it:

- If the memory names a file path: check the file exists.
- If the memory names a function or flag: grep for it.
- If the user is about to act on your recommendation (not just asking about history), verify first.

"The memory says X exists" is not the same as "X exists now."

A memory that summarizes repo state (activity logs, architecture snapshots) is frozen in time. If the user asks about *recent* or *current* state, prefer `git log` or reading the code over recalling the snapshot.

## Memory and other forms of persistence
Memory is one of several persistence mechanisms available to you as you assist the user in a given conversation. The distinction is often that memory can be recalled in future conversations and should not be used for persisting information that is only useful within the scope of the current conversation.
- When to use or update a plan instead of memory: If you are about to start a non-trivial implementation task and would like to reach alignment with the user on your approach you should use a Plan rather than saving this information to memory. Similarly, if you already have a plan within the conversation and you have changed your approach persist that change by updating the plan rather than saving a memory.
- When to use or update tasks instead of memory: When you need to break your work in current conversation into discrete steps or keep track of your progress use the update_plan tool instead of saving to memory. Its steps are great for persisting information about the work that needs to be done in the current conversation, but memory should be reserved for information that will be useful in future conversations."#;

/// Whether Claude Code keeps auto memory, decided as Claude Code decides it: the
/// `CLAUDE_CODE_DISABLE_AUTO_MEMORY` switch either way, then off in `--bare` mode and on the
/// web without persistent storage, then the last `autoMemoryEnabled` of `settings` (lowest
/// precedence first), on by default. `env` looks up an environment variable.
pub fn auto_memory_enabled(settings: &[&Value], env: &dyn Fn(&str) -> Option<String>) -> bool {
    let disable = env(DISABLE_ENV);
    if env_truthy(disable.as_deref()) {
        return false;
    }
    if env_defined_falsy(disable.as_deref()) {
        return true;
    }
    if env_truthy(env(SIMPLE_ENV).as_deref()) {
        return false;
    }
    if env_truthy(env(REMOTE_ENV).as_deref())
        && env(REMOTE_MEMORY_DIR_ENV).unwrap_or_default().is_empty()
    {
        return false;
    }
    settings
        .iter()
        .rev()
        .find_map(|settings| settings.get(ENABLED_SETTING).and_then(Value::as_bool))
        .unwrap_or(true)
}

/// The directory the default memory directories are kept under: `claude_home`, unless
/// `CLAUDE_CODE_REMOTE_MEMORY_DIR` names another.
pub fn auto_memory_base_dir(claude_home: &Path, env: &dyn Fn(&str) -> Option<String>) -> PathBuf {
    env(REMOTE_MEMORY_DIR_ENV)
        .filter(|dir| !dir.is_empty())
        .map_or_else(|| claude_home.to_path_buf(), PathBuf::from)
}

/// The memory directory set in place of the project's default one: the
/// `CLAUDE_COWORK_MEMORY_PATH_OVERRIDE` environment variable, else the last
/// `autoMemoryDirectory` of `trusted_settings` (the user's and the local settings, never
/// the project's shared ones, which a repository could use to get write access anywhere),
/// where `~/` stands for `home_dir`. A path that is not absolute, or names a filesystem
/// root or the home directory itself, is ignored, as Claude Code ignores it.
pub fn auto_memory_dir_override(
    trusted_settings: &[&Value],
    home_dir: Option<&Path>,
    env: &dyn Fn(&str) -> Option<String>,
) -> Option<PathBuf> {
    if let Some(dir) = env(PATH_OVERRIDE_ENV).and_then(|raw| validate_memory_path(&raw, None)) {
        return Some(dir);
    }
    // Like Claude Code, the highest source that sets it wins, even when its path is invalid.
    let raw = trusted_settings
        .iter()
        .rev()
        .find_map(|settings| settings.get(DIRECTORY_SETTING).and_then(Value::as_str))?;
    validate_memory_path(raw, Some(home_dir?))
}

/// Claude Code's instructions for keeping auto memory in `memory_dir`, followed by the text
/// of `CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES` when it is set.
pub fn auto_memory_instructions(memory_dir: &Path, env: &dyn Fn(&str) -> Option<String>) -> String {
    // Claude Code shows the directory with a trailing separator.
    let mut dir = memory_dir.display().to_string();
    if !dir.ends_with(MAIN_SEPARATOR) {
        dir.push(MAIN_SEPARATOR);
    }
    let mut text = INSTRUCTIONS_TEMPLATE.replacen(MEMORY_DIR_PLACEHOLDER, &dir, 1);
    if let Some(extra) = env(EXTRA_GUIDELINES_ENV).filter(|extra| !extra.trim().is_empty()) {
        text.push_str("\n\n");
        text.push_str(&extra);
    }
    text
}

/// `validateMemoryPath` of Claude Code: `~/` is expanded when `home_dir` is given, the path
/// is normalized and must be absolute and not a root.
fn validate_memory_path(raw: &str, home_dir: Option<&Path>) -> Option<PathBuf> {
    if raw.is_empty() {
        return None;
    }
    let candidate = match (
        home_dir,
        raw.strip_prefix("~/").or_else(|| raw.strip_prefix("~\\")),
    ) {
        (Some(home_dir), Some(rest)) => {
            // `~/`, `~/.` and `~/..` would name the home directory or its parent.
            if matches!(normalize(rest).as_str(), "." | "..") {
                return None;
            }
            home_dir.join(rest).to_string_lossy().into_owned()
        }
        _ => raw.to_string(),
    };
    let normalized = normalize(&candidate);
    let normalized = normalized.trim_end_matches(['/', '\\']);
    let is_drive_root = normalized.len() == 2
        && normalized.as_bytes()[0].is_ascii_alphabetic()
        && normalized.as_bytes()[1] == b':';
    if !Path::new(normalized).is_absolute()
        || normalized.len() < 3
        || is_drive_root
        || normalized.starts_with("\\\\")
        || normalized.starts_with("//")
        || normalized.contains('\0')
    {
        return None;
    }
    Some(PathBuf::from(normalized))
}

/// Resolves `.` and `..` without touching the filesystem, as Node's `path.normalize` does:
/// `..` above the root stays at the root, `..` at the start of a relative path is kept,
/// and an empty result is `.`.
fn normalize(path: &str) -> String {
    let path = Path::new(path);
    let mut prefix = PathBuf::new();
    let mut parts: Vec<&OsStr> = Vec::new();
    let mut absolute = false;
    for component in path.components() {
        match component {
            Component::Prefix(prefix_component) => prefix.push(prefix_component.as_os_str()),
            Component::RootDir => {
                prefix.push(component.as_os_str());
                absolute = true;
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if parts.last().is_some_and(|last| *last != OsStr::new("..")) {
                    parts.pop();
                } else if !absolute {
                    parts.push(component.as_os_str());
                }
            }
            Component::Normal(name) => parts.push(name),
        }
    }
    let mut normalized = prefix;
    for part in parts {
        normalized.push(part);
    }
    let normalized = normalized.to_string_lossy().into_owned();
    if normalized.is_empty() {
        ".".to_string()
    } else {
        normalized
    }
}

/// `isEnvTruthy` of Claude Code.
fn env_truthy(value: Option<&str>) -> bool {
    value.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// `isEnvDefinedFalsy` of Claude Code.
fn env_defined_falsy(value: Option<&str>) -> bool {
    value.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "0" | "false" | "no" | "off"
        )
    })
}

#[cfg(test)]
#[path = "memory_instructions_tests.rs"]
mod tests;
