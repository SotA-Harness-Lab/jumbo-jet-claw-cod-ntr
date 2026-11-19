//! Hooks declared in Claude Code's settings files, run by Codex in place.
//!
//! Claude Code keeps hooks under the `hooks` key of `~/.claude/settings.json` and of a
//! project's `.claude/settings.json` and `.claude/settings.local.json`, in the same shape
//! Codex reads from `hooks.json`. Handlers of a type Codex does not run are left out with
//! a warning, so that one of them does not take the whole file with it. Of the events Codex
//! does not have, `Notification`, `PostToolUseFailure` and `StopFailure` are kept apart and run
//! at the closest Codex moment (see `events::claude_only`); the others are ignored.

use std::fs;
use std::io;
use std::path::Path;

use codex_config::HookEventsToml;
use codex_config::MatcherGroup;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;

use crate::events::claude_only::ClaudeOnlyEvent;

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
    /// What hooks see as `CLAUDE_CODE_ENTRYPOINT`, which tells them whether someone attends
    /// the session (`cli`) or a program runs it (`sdk-cli`); see [`claude_entrypoint`].
    pub entrypoint: Option<String>,
    /// The Claude Code plugins handed to Codex's plugin loader. Their hooks see the
    /// variables above, are left out when a settings file sets `disableAllHooks`, and run
    /// without Codex's trust prompt when the user's own settings enable the plugin.
    pub plugins: Vec<ClaudePluginRoot>,
}

/// A Claude Code plugin whose hooks Codex's plugin loader found, as its hooks are treated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaudePluginRoot {
    /// The plugin's install path, the root its hooks are loaded from.
    pub root: AbsolutePathBuf,
    /// Whether the user's own `~/.claude/settings.json` enables the plugin. Claude Code runs an
    /// enabled plugin's hooks without asking, so these run like the user's own hooks; a plugin
    /// a project's settings alone enable waits for Codex's trust like the project's hooks.
    pub trusted: bool,
}

/// Claude Code's `CLAUDE_CODE_ENTRYPOINT` for a Codex session started from `source`: the
/// TUI is `cli`, `codex exec` is `sdk-cli` as for `claude -p`. Sessions of other sources get
/// none, since Claude Code has no counterpart for them.
pub fn claude_entrypoint(source: &codex_protocol::protocol::SessionSource) -> Option<String> {
    use codex_protocol::protocol::SessionSource;
    match source {
        SessionSource::Cli => Some("cli"),
        SessionSource::Exec => Some("sdk-cli"),
        SessionSource::VSCode => Some("claude-vscode"),
        SessionSource::Mcp
        | SessionSource::Custom(_)
        | SessionSource::Internal(_)
        | SessionSource::SubAgent(_)
        | SessionSource::Unknown => None,
    }
    .map(str::to_string)
}

impl ClaudeHookSettings {
    /// The settings of the user's home directory and of `project_dir`, or nothing when
    /// `CCN_CLAUDE_COMPAT=0` turns Claude Code compatibility off.
    pub fn discover(
        project_dir: Option<AbsolutePathBuf>,
        entrypoint: Option<String>,
    ) -> Option<Self> {
        if std::env::var_os("CCN_CLAUDE_COMPAT").is_some_and(|value| value == "0") {
            return None;
        }
        let user_dir = std::env::var_os("HOME")
            .map(|home| Path::new(&home).join(".claude"))
            .and_then(|dir| AbsolutePathBuf::from_absolute_path_checked(dir).ok());
        Some(Self {
            user_dir,
            project_dir,
            entrypoint,
            plugins: Vec::new(),
        })
    }

    /// The same settings, with the Claude Code plugins handed to Codex's plugin loader.
    pub fn with_plugins(mut self, plugins: Vec<ClaudePluginRoot>) -> Self {
        self.plugins = plugins;
        self
    }
}

/// The hooks of one settings file.
pub(crate) struct ClaudeSettingsHooks {
    pub(crate) path: AbsolutePathBuf,
    /// The user's own file, as opposed to a project's.
    pub(crate) is_user: bool,
    pub(crate) events: HookEventsToml,
    /// The groups of the events Codex's own configuration does not have.
    pub(crate) claude_only: Vec<(ClaudeOnlyEvent, Vec<MatcherGroup>)>,
}

/// Loads the hooks of every settings file that exists, user first. When one of them sets
/// `disableAllHooks`, none are returned, as in Claude Code.
pub(crate) fn load(
    settings: &ClaudeHookSettings,
    warnings: &mut Vec<String>,
) -> Vec<ClaudeSettingsHooks> {
    let mut loaded = Vec::new();
    for (path, is_user) in settings_files(settings) {
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
        let claude_only = claude_only_groups(&hooks, &path, warnings);
        match serde_json::from_value::<HookEventsToml>(hooks) {
            Ok(events) if !events.is_empty() || !claude_only.is_empty() => {
                loaded.push(ClaudeSettingsHooks {
                    path,
                    is_user,
                    events,
                    claude_only,
                })
            }
            Ok(_) => {}
            Err(err) => warnings.push(format!(
                "failed to parse hooks in Claude Code settings {}: {err}",
                path.display()
            )),
        }
    }
    loaded
}

/// Whether one of the settings files sets `disableAllHooks`, which in Claude Code turns off
/// the hooks of plugins too.
pub(crate) fn disables_all_hooks(settings: &ClaudeHookSettings) -> bool {
    settings_files(settings).iter().any(|(path, _)| {
        read_settings(path, &mut Vec::new()).is_some_and(|settings| {
            settings
                .get("disableAllHooks")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        })
    })
}

/// The settings files, user's first, each with whether it is the user's own.
fn settings_files(settings: &ClaudeHookSettings) -> Vec<(AbsolutePathBuf, bool)> {
    let mut files = Vec::new();
    if let Some(user_dir) = &settings.user_dir {
        files.push((user_dir.join("settings.json"), true));
    }
    if let Some(project_dir) = &settings.project_dir {
        let dot_claude = project_dir.join(".claude");
        // In the home directory, `.claude` is the user's own configuration.
        if settings.user_dir.as_ref() != Some(&dot_claude) {
            files.extend(
                SETTINGS_FILES
                    .iter()
                    .map(|name| (dot_claude.join(name), false)),
            );
        }
    }
    files
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

/// The groups of `Notification`, `PostToolUseFailure` and `StopFailure`, which
/// `HookEventsToml` has no place for.
fn claude_only_groups(
    hooks: &Value,
    path: &AbsolutePathBuf,
    warnings: &mut Vec<String>,
) -> Vec<(ClaudeOnlyEvent, Vec<MatcherGroup>)> {
    let mut groups = Vec::new();
    for event in ClaudeOnlyEvent::ALL {
        let Some(value) = hooks.get(event.label()) else {
            continue;
        };
        match serde_json::from_value::<Vec<MatcherGroup>>(value.clone()) {
            Ok(event_groups) if !event_groups.is_empty() => groups.push((event, event_groups)),
            Ok(_) => {}
            Err(err) => warnings.push(format!(
                "failed to parse {} hooks in Claude Code settings {}: {err}",
                event.label(),
                path.display()
            )),
        }
    }
    groups
}

/// Drops handlers of a type Codex does not know, such as `http`, reporting them.
fn supported_handlers(
    mut hooks: Value,
    path: &AbsolutePathBuf,
    warnings: &mut Vec<String>,
) -> Value {
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
