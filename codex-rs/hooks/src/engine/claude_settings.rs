// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Hooks declared in Claude Code's settings files, run by Codex in place.
//!
//! Claude Code keeps hooks under the `hooks` key of `~/.claude/settings.json` and of a
//! project's `.claude/settings.json` and `.claude/settings.local.json`, in the same shape
//! Codex reads from `hooks.json`. Handlers of a type Codex does not run are left out with
//! a warning, so that one of them does not take the whole file with it. Of the events Codex
//! does not have, `Notification`, `PostToolUseFailure` and `StopFailure` are kept apart and run
//! at the closest Codex moment (see `events::claude_only`); the others are ignored.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use codex_config::HookEventsToml;
use codex_config::HookHandlerConfig;
use codex_config::MatcherGroup;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;

use super::http_runner::HttpHook;
use super::http_runner::HttpHookPolicy;
use crate::events::claude_only::ClaudeOnlyEvent;

const SETTINGS_FILES: &[&str] = &["settings.json", "settings.local.json"];
/// Handler types Codex runs; any other is dropped before parsing.
const KNOWN_HANDLER_TYPES: &[&str] = &["command", "prompt", "agent", "http", "mcp_tool"];

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

/// Where a handler stands in a hooks file: event name, group and position in the group.
pub(crate) type HandlerPosition = (String, usize, usize);

/// The `if` conditions of a hooks file's handlers.
pub(crate) type HookConditions = HashMap<HandlerPosition, String>;

/// What a handler of a type only Claude Code's hooks files have is to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ClaudeHandlerKind {
    /// `type: "http"`: the hook input is posted to a URL.
    Http(HttpHook),
    /// `type: "prompt"`: a model answers whether the prompt's condition is met. `model` is
    /// the handler's own, a Claude model's name.
    Prompt {
        prompt: String,
        model: Option<String>,
    },
    /// `type: "agent"`: as `prompt`, by a model that may look around with tools first.
    Agent {
        prompt: String,
        model: Option<String>,
    },
}

/// A `prompt`, `agent` or `http` handler as its hooks file declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClaudeHandlerSpec {
    pub(crate) kind: ClaudeHandlerKind,
    pub(crate) timeout_sec: Option<u64>,
    pub(crate) status_message: Option<String>,
    /// The handler as written, so that trust given to it ends when any of it changes.
    pub(crate) declaration: String,
}

/// What a hooks file in Claude Code's format says that `HookEventsToml` has no place for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ClaudeHandlerExtras {
    pub(crate) conditions: HookConditions,
    pub(crate) handlers: HashMap<HandlerPosition, ClaudeHandlerSpec>,
    /// A plugin's variables, filled in in its hooks' commands before they run.
    pub(crate) plugin: Option<codex_claude_compat::PluginVariables>,
}

/// The hooks of one settings file.
pub(crate) struct ClaudeSettingsHooks {
    pub(crate) path: AbsolutePathBuf,
    /// The user's own file, as opposed to a project's.
    pub(crate) is_user: bool,
    pub(crate) events: HookEventsToml,
    /// The groups of the events Codex's own configuration does not have.
    pub(crate) claude_only: Vec<(ClaudeOnlyEvent, Vec<MatcherGroup>)>,
    /// The handlers' `if` conditions and the handlers of Claude Code's own types.
    pub(crate) extras: ClaudeHandlerExtras,
}

/// Loads the hooks of every settings file that exists, user first. When one of them sets
/// `disableAllHooks`, none are returned, as in Claude Code.
pub(crate) fn load(
    settings: &ClaudeHookSettings,
    warnings: &mut Vec<String>,
) -> Vec<ClaudeSettingsHooks> {
    let mut loaded = Vec::new();
    let files = settings_files(settings)
        .into_iter()
        .filter_map(|(path, is_user)| {
            read_settings(&path, warnings).map(|settings| (path, is_user, settings))
        })
        .collect::<Vec<_>>();
    let http_policy = http_hook_policy(files.iter().map(|(_, _, settings)| settings));
    for (path, is_user, settings) in files {
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
        let mut hooks = drop_malformed(hooks, &path, warnings);
        let extras = handler_extras(&hooks, &http_policy);
        stand_in_for_http(&mut hooks);
        let claude_only = claude_only_groups(&hooks, &path, warnings);
        match serde_json::from_value::<HookEventsToml>(hooks) {
            Ok(events) if !events.is_empty() || !claude_only.is_empty() => {
                loaded.push(ClaudeSettingsHooks {
                    path,
                    is_user,
                    events,
                    claude_only,
                    extras,
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

/// `allowedHttpHookUrls` and `httpHookAllowedEnvVars` of the settings files together: a
/// list set in several files is their concatenation, and one set nowhere restricts nothing.
fn http_hook_policy<'a>(settings: impl Iterator<Item = &'a Value> + Clone) -> HttpHookPolicy {
    let list = |key: &str| {
        let lists = settings
            .clone()
            .filter_map(|settings| settings.get(key).and_then(Value::as_array))
            .collect::<Vec<_>>();
        (!lists.is_empty()).then(|| {
            lists
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
    };
    HttpHookPolicy {
        allowed_urls: list("allowedHttpHookUrls"),
        allowed_env_vars: list("httpHookAllowedEnvVars"),
    }
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

/// Each handler under `hooks` with where it stands.
fn handlers_by_position(hooks: &Value) -> Vec<(HandlerPosition, &Value)> {
    let mut handlers = Vec::new();
    let Some(events) = hooks.as_object() else {
        return handlers;
    };
    for (event, groups) in events {
        let groups = groups.as_array().map(Vec::as_slice).unwrap_or_default();
        for (group_index, group) in groups.iter().enumerate() {
            let group_handlers = group
                .get("hooks")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or_default();
            for (handler_index, handler) in group_handlers.iter().enumerate() {
                handlers.push(((event.clone(), group_index, handler_index), handler));
            }
        }
    }
    handlers
}

/// The `if` of each handler that has one, by where the handler stands under `hooks`.
pub(crate) fn hook_conditions(hooks: &Value) -> HookConditions {
    handlers_by_position(hooks)
        .into_iter()
        .filter_map(|(position, handler)| {
            let condition = handler.get("if").and_then(Value::as_str)?;
            (!condition.trim().is_empty()).then(|| (position, condition.to_string()))
        })
        .collect()
}

/// The conditions and the `prompt`, `agent` and `http` handlers under `hooks`.
pub(crate) fn handler_extras(hooks: &Value, http_policy: &HttpHookPolicy) -> ClaudeHandlerExtras {
    let handlers = handlers_by_position(hooks)
        .into_iter()
        .filter_map(|(position, handler)| {
            let text = |key: &str| handler.get(key).and_then(Value::as_str).map(str::to_string);
            let kind = match handler.get("type").and_then(Value::as_str)? {
                "prompt" => ClaudeHandlerKind::Prompt {
                    prompt: text("prompt")?,
                    model: text("model"),
                },
                "agent" => ClaudeHandlerKind::Agent {
                    prompt: text("prompt")?,
                    model: text("model"),
                },
                "http" => ClaudeHandlerKind::Http(HttpHook {
                    url: text("url")?,
                    headers: handler
                        .get("headers")
                        .and_then(Value::as_object)
                        .map(|headers| {
                            headers
                                .iter()
                                .filter_map(|(name, value)| {
                                    Some((name.clone(), value.as_str()?.to_string()))
                                })
                                .collect::<BTreeMap<_, _>>()
                        })
                        .unwrap_or_default(),
                    allowed_env_vars: handler
                        .get("allowedEnvVars")
                        .and_then(Value::as_array)
                        .map(|names| {
                            names
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_string)
                                .collect()
                        })
                        .unwrap_or_default(),
                    policy: http_policy.clone(),
                }),
                _ => return None,
            };
            Some((
                position,
                ClaudeHandlerSpec {
                    kind,
                    timeout_sec: handler
                        .get("timeout")
                        .and_then(Value::as_f64)
                        .filter(|timeout| *timeout > 0.0)
                        .map(|timeout| timeout.ceil() as u64),
                    status_message: text("statusMessage"),
                    declaration: handler.to_string(),
                },
            ))
        })
        .collect();
    ClaudeHandlerExtras {
        conditions: hook_conditions(hooks),
        handlers,
        plugin: None,
    }
}

/// Gives `http` handlers the type of `prompt` ones for parsing: `HookEventsToml` has no
/// `http`, and the handler keeps its place while [`handler_extras`] holds what it is.
fn stand_in_for_http(hooks: &mut Value) {
    let Some(events) = hooks.as_object_mut() else {
        return;
    };
    for groups in events.values_mut().filter_map(Value::as_array_mut) {
        for handler in groups
            .iter_mut()
            .filter_map(|group| group.get_mut("hooks"))
            .filter_map(Value::as_array_mut)
            .flatten()
        {
            if handler.get("type").and_then(Value::as_str) == Some("http") {
                handler["type"] = Value::String("prompt".to_string());
            }
        }
    }
}

/// What a Claude Code plugin's hooks file, whose events are under its `hooks` key, says
/// beyond `HookEventsToml`. A plugin's `http` handlers are not among them: Codex's plugin
/// loader reads the file first and does not know the type.
pub(crate) fn plugin_handler_extras(path: &AbsolutePathBuf) -> ClaudeHandlerExtras {
    read_settings(path, &mut Vec::new())
        .and_then(|file| {
            file.get("hooks")
                .map(|hooks| handler_extras(hooks, &HttpHookPolicy::default()))
        })
        .unwrap_or_default()
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

/// Leaves out, with a warning, each event, matcher group and handler under `hooks` that Codex
/// cannot read (a handler without `command`, a `matcher` that is not a string, an event that is
/// not a list), so that one bad entry does not take the file's other hooks with it, as in
/// Claude Code. A `timeout` that is not a whole number of seconds is rounded up first. Events
/// Codex does not run are left as they are.
fn drop_malformed(mut hooks: Value, path: &AbsolutePathBuf, warnings: &mut Vec<String>) -> Value {
    let Some(events) = hooks.as_object_mut() else {
        warnings.push(format!(
            "skipping the hooks of Claude Code settings {}: `hooks` is not an object",
            path.display()
        ));
        return Value::Object(serde_json::Map::new());
    };
    let codex_events = serde_json::to_value(HookEventsToml::default())
        .ok()
        .and_then(|events| {
            events
                .as_object()
                .map(|events| events.keys().cloned().collect::<Vec<_>>())
        })
        .unwrap_or_default();
    let mut skipped = Vec::new();
    events.retain(|event, groups| {
        let read_here = codex_events.contains(event)
            || ClaudeOnlyEvent::ALL
                .iter()
                .any(|claude_only| claude_only.label() == event.as_str());
        if !read_here {
            return true;
        }
        let Some(groups) = groups.as_array_mut() else {
            skipped.push(format!("{event}: not a list of matcher groups"));
            return false;
        };
        groups.retain_mut(|group| {
            let well_formed = group.is_object()
                && group
                    .get("matcher")
                    .is_none_or(|matcher| matcher.is_string() || matcher.is_null())
                && group.get("hooks").is_none_or(Value::is_array);
            if !well_formed {
                skipped.push(format!(
                    "{event}: a matcher group that is not an object with a string `matcher` \
                     and a list of `hooks`"
                ));
                return false;
            }
            if let Some(handlers) = group.get_mut("hooks").and_then(Value::as_array_mut) {
                handlers.retain_mut(|handler| {
                    round_up_timeout(handler);
                    let mut parsed = handler.clone();
                    // `http` handlers are parsed as `prompt` ones; see `stand_in_for_http`.
                    if parsed.get("type").and_then(Value::as_str) == Some("http") {
                        parsed["type"] = Value::String("prompt".to_string());
                    }
                    match serde_json::from_value::<HookHandlerConfig>(parsed) {
                        Ok(_) => true,
                        Err(err) => {
                            skipped.push(format!("{event}: a handler ({err})"));
                            false
                        }
                    }
                });
            }
            true
        });
        true
    });
    for entry in skipped {
        warnings.push(format!(
            "skipping a Claude Code hook in {}: {entry}",
            path.display()
        ));
    }
    hooks
}

/// Claude Code takes any positive number of seconds; Codex's configuration takes whole ones.
fn round_up_timeout(handler: &mut Value) {
    if let Some(timeout) = handler.get("timeout").and_then(Value::as_f64)
        && timeout > 0.0
        && timeout.fract() != 0.0
    {
        handler["timeout"] = Value::from(timeout.ceil() as u64);
    }
}

#[cfg(test)]
#[path = "claude_settings_tests.rs"]
mod tests;
