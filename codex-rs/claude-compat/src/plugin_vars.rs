// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What a Claude Code plugin's files may refer to: its install path, as
//! `${CLAUDE_PLUGIN_ROOT}`, and the options its manifest declares under `userConfig`, as
//! `${user_config.KEY}` in text and `CLAUDE_PLUGIN_OPTION_<KEY>` in a hook's environment.
//!
//! The values are the ones Claude Code stored when the user filled them in: the plain ones
//! under `pluginConfigs.<plugin id>.options` of `~/.claude/settings.json`, the ones marked
//! `sensitive` under `pluginSecrets.<plugin id>` of `~/.claude/.credentials.json`. On macOS
//! Claude Code keeps the latter in the keychain, where they are not read, so a sensitive
//! option has no value there. Codex has no way to ask for a value: an option the user never
//! set in Claude Code stays unset.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use regex_lite::Regex;
use serde_json::Value;

use crate::CLAUDE_DIR;
use crate::plugins::INSTALLED_PLUGINS_FILE;
use crate::plugins::installed_plugin_paths;
use crate::settings::read_settings;

const MANIFEST: &str = ".claude-plugin/plugin.json";
const PLUGIN_ROOT: &str = "${CLAUDE_PLUGIN_ROOT}";

static USER_CONFIG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{user_config\.([^}]+)\}")
        .unwrap_or_else(|err| panic!("valid user_config pattern: {err}"))
});

/// The variables of one installed Claude Code plugin.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PluginVariables {
    /// The plugin's name, from its manifest or else its directory.
    pub name: String,
    /// The plugin's install path.
    pub root: PathBuf,
    /// The manifest's `agents` entries, relative to the root as written.
    pub agent_paths: Vec<String>,
    /// The options that have a value, as text.
    options: BTreeMap<String, String>,
    /// The options the manifest marks `sensitive`.
    sensitive: BTreeSet<String>,
}

impl PluginVariables {
    /// The variables of the plugin installed at `root`, with the option values found under
    /// the user's home directory. `None` when `root` holds no Claude Code plugin manifest.
    pub fn load(root: &Path) -> Option<Self> {
        let claude_home = std::env::var_os("HOME").map(|home| Path::new(&home).join(CLAUDE_DIR));
        Self::load_with_home(root, claude_home.as_deref())
    }

    /// As [`Self::load`], with Claude Code's configuration directory given.
    pub fn load_with_home(root: &Path, claude_home: Option<&Path>) -> Option<Self> {
        let manifest = read_settings(&root.join(MANIFEST)).ok()??;
        let name = manifest
            .get("name")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_string)
            .or_else(|| Some(root.file_name()?.to_string_lossy().into_owned()))?;
        let agent_paths = match manifest.get("agents") {
            Some(Value::String(path)) => vec![path.clone()],
            Some(Value::Array(paths)) => paths
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
            _ => Vec::new(),
        };
        let declared = manifest.get("userConfig").and_then(Value::as_object);
        let sensitive = declared
            .into_iter()
            .flatten()
            .filter(|(_, option)| {
                option
                    .get("sensitive")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            })
            .map(|(key, _)| key.clone())
            .collect();
        let mut options = BTreeMap::new();
        if declared.is_some()
            && let Some(claude_home) = claude_home
            && let Some(id) = plugin_id(root, claude_home)
        {
            let stored = |file: &str, pointer: &[&str]| {
                let mut value = read_settings(&claude_home.join(file)).ok()??;
                for key in pointer {
                    let inner = value.get_mut(*key)?.take();
                    value = inner;
                }
                match value {
                    Value::Object(values) => Some(values),
                    _ => None,
                }
            };
            // The secure store comes last and wins where both name an option.
            for values in [
                stored("settings.json", &["pluginConfigs", &id, "options"]),
                stored(".credentials.json", &["pluginSecrets", &id]),
            ]
            .into_iter()
            .flatten()
            {
                for (key, value) in values {
                    if let Some(text) = option_text(&value) {
                        options.insert(key, text);
                    }
                }
            }
        }
        Some(Self {
            name,
            root: root.to_path_buf(),
            agent_paths,
            options,
            sensitive,
        })
    }

    /// `text` with `${CLAUDE_PLUGIN_ROOT}` and every `${user_config.KEY}` filled in, for a
    /// command or a server's configuration. `Err` names the first option that has no value.
    pub fn substitute(&self, text: &str) -> Result<String, String> {
        let text = text.replace(PLUGIN_ROOT, &self.root.to_string_lossy());
        let mut missing = None;
        let substituted = USER_CONFIG.replace_all(&text, |captures: &regex_lite::Captures<'_>| {
            let key = &captures[1];
            match self.options.get(key) {
                Some(value) => value.clone(),
                None => {
                    missing.get_or_insert_with(|| key.to_string());
                    String::new()
                }
            }
        });
        match missing {
            Some(key) => Err(key),
            None => Ok(substituted.into_owned()),
        }
    }

    /// `text` with the same variables filled in, for prose a model reads (an agent's
    /// instructions): a sensitive option is named, not shown, and an option without a value
    /// stays as written.
    pub fn substitute_in_content(&self, text: &str) -> String {
        let text = text.replace(PLUGIN_ROOT, &self.root.to_string_lossy());
        USER_CONFIG
            .replace_all(&text, |captures: &regex_lite::Captures<'_>| {
                let key = &captures[1];
                if self.sensitive.contains(key) {
                    format!("[sensitive option '{key}' not available in skill content]")
                } else {
                    self.options
                        .get(key)
                        .cloned()
                        .unwrap_or_else(|| captures[0].to_string())
                }
            })
            .into_owned()
    }

    /// The options as a hook's environment sees them: `CLAUDE_PLUGIN_OPTION_<KEY>`, the key
    /// in upper case with anything but letters, digits and `_` turned into `_`.
    pub fn option_env(&self) -> Vec<(String, String)> {
        self.options
            .iter()
            .map(|(key, value)| {
                let key = key
                    .chars()
                    .map(|ch| {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            ch.to_ascii_uppercase()
                        } else {
                            '_'
                        }
                    })
                    .collect::<String>();
                (format!("CLAUDE_PLUGIN_OPTION_{key}"), value.clone())
            })
            .collect()
    }

    /// The directories the plugin keeps agents in: `agents/` and the directories among the
    /// manifest's `agents` entries. An entry that names a single file is not among them.
    pub fn agent_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = vec![self.root.join("agents")];
        for path in &self.agent_paths {
            let dir = self.root.join(path.trim_start_matches("./"));
            if !dirs.contains(&dir) {
                dirs.push(dir);
            }
        }
        dirs.retain(|dir| dir.starts_with(&self.root) && dir.is_dir());
        dirs
    }
}

/// The plugin a file belongs to: the nearest directory above it with a Claude Code plugin
/// manifest.
pub fn plugin_root_of(file: &Path) -> Option<&Path> {
    file.ancestors()
        .skip(1)
        .find(|dir| dir.join(MANIFEST).is_file())
}

/// `<plugin>@<marketplace>` of the plugin installed at `root`, from Claude Code's record of
/// installed plugins.
fn plugin_id(root: &Path, claude_home: &Path) -> Option<String> {
    let installed = read_settings(&claude_home.join(INSTALLED_PLUGINS_FILE)).ok()??;
    installed_plugin_paths(&installed)
        .into_iter()
        .find(|(_, path)| path == root)
        .map(|(id, _)| id)
}

/// A stored option as the text it is substituted as: JavaScript's `String(value)`.
fn option_text(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        Value::Array(items) => Some(
            items
                .iter()
                .filter_map(option_text)
                .collect::<Vec<_>>()
                .join(","),
        ),
        Value::Null | Value::Object(_) => None,
    }
}

#[cfg(test)]
#[path = "plugin_vars_tests.rs"]
mod tests;
