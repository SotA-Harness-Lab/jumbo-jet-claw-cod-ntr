//! Hooks declared in Claude Code's settings files, run by Codex in place.
//!
//! Claude Code keeps hooks under the `hooks` key of `~/.claude/settings.json` and of a
//! project's `.claude/settings.json` and `.claude/settings.local.json`, in the same shape
//! Codex reads from `hooks.json`. Handlers of a type Codex does not run are left out with
//! a warning, so that one of them does not take the whole file with it; events Codex does
//! not have are ignored.

use std::fs;
use std::io;
use std::path::Path;

use codex_config::HookEventsToml;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;

const SETTINGS_FILES: &[&str] = &["settings.json", "settings.local.json"];
/// Handler types Codex runs or knowingly skips; any other is dropped before parsing.
const KNOWN_HANDLER_TYPES: &[&str] = &["command", "prompt", "agent", "mcp_tool"];

/// Where Claude Code's settings files are looked for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClaudeHookSettings {
    /// The user's Claude Code configuration directory, `~/.claude`.
    pub user_dir: Option<AbsolutePathBuf>,
    /// The project directory: its `.claude/settings.json` and `.claude/settings.local.json`
    /// are read, and hooks see it as `CLAUDE_PROJECT_DIR`.
    pub project_dir: Option<AbsolutePathBuf>,
}

impl ClaudeHookSettings {
    /// The settings of the user's home directory and of `project_dir`, or nothing when
    /// `CODEX_CLAUDE_COMPAT=0` turns Claude Code compatibility off.
    pub fn discover(project_dir: Option<AbsolutePathBuf>) -> Option<Self> {
        if std::env::var_os("CODEX_CLAUDE_COMPAT").is_some_and(|value| value == "0") {
            return None;
        }
        let user_dir = std::env::var_os("HOME")
            .map(|home| Path::new(&home).join(".claude"))
            .and_then(|dir| AbsolutePathBuf::from_absolute_path_checked(dir).ok());
        Some(Self {
            user_dir,
            project_dir,
        })
    }
}

/// The hooks of one settings file.
pub(crate) struct ClaudeSettingsHooks {
    pub(crate) path: AbsolutePathBuf,
    /// The user's own file, as opposed to a project's.
    pub(crate) is_user: bool,
    pub(crate) events: HookEventsToml,
}

/// Loads the hooks of every settings file that exists, user first. When one of them sets
/// `disableAllHooks`, none are returned, as in Claude Code.
pub(crate) fn load(
    settings: &ClaudeHookSettings,
    warnings: &mut Vec<String>,
) -> Vec<ClaudeSettingsHooks> {
    let mut files = Vec::new();
    if let Some(user_dir) = &settings.user_dir {
        files.push((user_dir.join("settings.json"), true));
    }
    if let Some(project_dir) = &settings.project_dir {
        let dot_claude = project_dir.join(".claude");
        // In the home directory, `.claude` is the user's own configuration.
        if settings.user_dir.as_ref() != Some(&dot_claude) {
            files.extend(SETTINGS_FILES.iter().map(|name| (dot_claude.join(name), false)));
        }
    }

    let mut loaded = Vec::new();
    for (path, is_user) in files {
        let Some(settings) = read_settings(&path, warnings) else {
            continue;
        };
        if settings
            .get("disableAllHooks")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return Vec::new();
        }
        let Some(hooks) = settings.get("hooks") else {
            continue;
        };
        let hooks = supported_handlers(hooks.clone(), &path, warnings);
        match serde_json::from_value::<HookEventsToml>(hooks) {
            Ok(events) if !events.is_empty() => loaded.push(ClaudeSettingsHooks {
                path,
                is_user,
                events,
            }),
            Ok(_) => {}
            Err(err) => warnings.push(format!(
                "failed to parse hooks in Claude Code settings {}: {err}",
                path.display()
            )),
        }
    }
    loaded
}

fn read_settings(path: &AbsolutePathBuf, warnings: &mut Vec<String>) -> Option<Value> {
    let contents = match fs::read_to_string(path.as_path()) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return None,
        Err(err) => {
            warnings.push(format!(
                "failed to read Claude Code settings {}: {err}",
                path.display()
            ));
            return None;
        }
    };
    match serde_json::from_str(&contents) {
        Ok(value) => Some(value),
        Err(err) => {
            warnings.push(format!(
                "failed to parse Claude Code settings {}: {err}",
                path.display()
            ));
            None
        }
    }
}

/// Drops handlers of a type Codex does not know, such as `http`, reporting them.
fn supported_handlers(mut hooks: Value, path: &AbsolutePathBuf, warnings: &mut Vec<String>) -> Value {
    let mut dropped = Vec::new();
    if let Some(events) = hooks.as_object_mut() {
        for groups in events.values_mut().filter_map(Value::as_array_mut) {
            for handlers in groups
                .iter_mut()
                .filter_map(|group| group.get_mut("hooks"))
                .filter_map(Value::as_array_mut)
            {
                handlers.retain(|handler| {
                    let kind = handler.get("type").and_then(Value::as_str).unwrap_or("");
                    let known = KNOWN_HANDLER_TYPES.contains(&kind);
                    if !known {
                        dropped.push(kind.to_string());
                    }
                    known
                });
            }
        }
    }
    if !dropped.is_empty() {
        dropped.sort();
        dropped.dedup();
        warnings.push(format!(
            "skipping Claude Code hooks of type {} in {}: Codex does not run them",
            dropped.join(", "),
            path.display()
        ));
    }
    hooks
}

#[cfg(test)]
#[path = "claude_settings_tests.rs"]
mod tests;
