// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

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
const INSTRUCTIONS_TEMPLATE: &str =
    codex_prompt_texts::text!("claude-compat/memory_instructions/instructions_template.md");

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
