// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What a Claude Code plugin's files may refer to: its install path, as
//! `${CLAUDE_PLUGIN_ROOT}`, its data directory, as `${CLAUDE_PLUGIN_DATA}`, and the options its
//! manifest declares under `userConfig`, as `${user_config.KEY}` in text and
//! `CLAUDE_PLUGIN_OPTION_<KEY>` in a hook's environment.
//!
//! The data directory is the one Codex keeps for the plugin, `$CODEX_HOME/plugins/data/
//! <plugin>-<marketplace>`, which the plugin's hooks are given too; like Claude Code, it is
//! created when something refers to it.
//!
//! The values are the ones Claude Code stored when the user filled them in: the plain ones
//! under `pluginConfigs.<plugin id>.options` of `~/.claude/settings.json`, the ones marked
//! `sensitive` under `pluginSecrets.<plugin id>` of `~/.claude/.credentials.json`. On macOS
//! Claude Code keeps the latter in the keychain, where they are not read, so a sensitive
//! option has no value there. Codex has no way to ask for a value: an option the user never
//! set in Claude Code takes the manifest's `default`, or else stays unset.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use regex_lite::Regex;
use serde_json::Value;

use crate::plugins::INSTALLED_PLUGINS_FILE;
use crate::plugins::installed_plugin_paths;
use crate::settings::read_settings;
use crate::user_config_dir;

const MANIFEST: &str = ".claude-plugin/plugin.json";
const PLUGIN_ROOT: &str = "${CLAUDE_PLUGIN_ROOT}";
const PLUGIN_DATA: &str = "${CLAUDE_PLUGIN_DATA}";
/// Where Codex keeps its plugins' data directories, below `$CODEX_HOME`.
const PLUGINS_DATA_DIR: &str = "plugins/data";

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
    /// `<plugin>@<marketplace>`, from Claude Code's record of installed plugins.
    id: Option<String>,
    /// The plugin's data directory, once known.
    data: Option<PathBuf>,
    /// The options that have a value, as text.
    options: BTreeMap<String, String>,
    /// The options the manifest marks `sensitive`.
    sensitive: BTreeSet<String>,
    /// The options the manifest declares without marking them `required`, which read as
    /// empty while they have no value.
    optional: BTreeSet<String>,
}

impl PluginVariables {
    /// The variables of the plugin installed at `root`, with the option values found under
    /// the user's home directory. `None` when `root` holds no Claude Code plugin manifest.
    pub fn load(root: &Path) -> Option<Self> {
        let claude_home = user_config_dir();
        let plugin = Self::load_with_home(root, claude_home.as_deref())?;
        Some(match codex_home() {
            Some(codex_home) => plugin.with_codex_home(&codex_home),
            None => plugin,
        })
    }

    /// The same variables, with the data directory Codex keeps for the plugin under
    /// `codex_home`. Without a record of the plugin's id there is none.
    pub fn with_codex_home(mut self, codex_home: &Path) -> Self {
        self.data = self
            .id
            .as_deref()
            .and_then(|id| id.rsplit_once('@'))
            .filter(|(name, marketplace)| !name.is_empty() && !marketplace.is_empty())
            .map(|(name, marketplace)| {
                codex_home
                    .join(PLUGINS_DATA_DIR)
                    .join(format!("{name}-{marketplace}"))
            });
        self
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
        let optional = declared
            .into_iter()
            .flatten()
            .filter(|(_, option)| {
                !option
                    .get("required")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            })
            .map(|(key, _)| key.clone())
            .collect();
        // A declared `default` is the value while the user has stored none.
        let mut options = declared
            .into_iter()
            .flatten()
            .filter_map(|(key, option)| Some((key.clone(), option_text(option.get("default")?)?)))
            .collect::<BTreeMap<_, _>>();
        let id = claude_home.and_then(|claude_home| plugin_id(root, claude_home));
        if declared.is_some()
            && let Some(claude_home) = claude_home
            && let Some(id) = &id
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
                stored("settings.json", &["pluginConfigs", id, "options"]),
                stored(".credentials.json", &["pluginSecrets", id]),
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
            id,
            data: None,
            options,
            sensitive,
            optional,
        })
    }

    /// `text` with `${CLAUDE_PLUGIN_ROOT}` filled in, and `${CLAUDE_PLUGIN_DATA}` when the
    /// data directory is known, which is then created.
    fn substitute_paths(&self, text: &str) -> String {
        let text = text.replace(PLUGIN_ROOT, &self.root.to_string_lossy());
        match &self.data {
            Some(data) if text.contains(PLUGIN_DATA) => {
                ensure_dir(data);
                text.replace(PLUGIN_DATA, &data.to_string_lossy())
            }
            _ => text,
        }
    }

    /// The variables a plugin's MCP server sees in its environment, as in Claude Code:
    /// `CLAUDE_PLUGIN_ROOT`, and `CLAUDE_PLUGIN_DATA` when the data directory is known, which
    /// is then created.
    pub fn server_env(&self) -> Vec<(String, String)> {
        let mut env = vec![(
            "CLAUDE_PLUGIN_ROOT".to_string(),
            self.root.to_string_lossy().into_owned(),
        )];
        if let Some(data) = &self.data {
            ensure_dir(data);
            env.push((
                "CLAUDE_PLUGIN_DATA".to_string(),
                data.to_string_lossy().into_owned(),
            ));
        }
        env
    }

    /// `text` with `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` and every
    /// `${user_config.KEY}` filled in, for a command or a server's configuration; an option
    /// declared without `required` that has no value reads as empty. `Err` names the first
    /// other option that has no value: one the manifest does not declare, or a required one.
    pub fn substitute(&self, text: &str) -> Result<String, String> {
        let text = self.substitute_paths(text);
        let mut missing = None;
        let substituted = USER_CONFIG.replace_all(&text, |captures: &regex_lite::Captures<'_>| {
            let key = &captures[1];
            match self.options.get(key) {
                Some(value) => value.clone(),
                None if self.optional.contains(key) => String::new(),
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
    /// instructions, a skill or a command): a sensitive option is named, not shown, and an
    /// option without a value stays as written.
    pub fn substitute_in_content(&self, text: &str) -> String {
        let text = self.substitute_paths(text);
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

    /// The directories the plugin keeps agents in: the directories among the manifest's
    /// `agents` entries, which stand in place of `agents/` as in Claude Code, or else
    /// `agents/`. An entry that names a single file is not among them.
    pub fn agent_dirs(&self) -> Vec<PathBuf> {
        let mut dirs = Vec::new();
        if self.agent_paths.is_empty() {
            dirs.push(self.root.join("agents"));
        }
        for path in &self.agent_paths {
            let dir = self.root.join(path.trim_start_matches("./"));
            if !dirs.contains(&dir) {
                dirs.push(dir);
            }
        }
        dirs.retain(|dir| dir.starts_with(&self.root) && dir.is_dir());
        dirs
    }

    /// The single agent files among the manifest's `agents` entries: those that name a
    /// Markdown file inside the plugin. Such an agent is `<plugin>:<name>`, with no
    /// directories in between, as in Claude Code.
    pub fn agent_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        for path in &self.agent_paths {
            let relative = Path::new(path.trim_start_matches("./"));
            if relative
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
            {
                continue;
            }
            let file = self.root.join(relative);
            if file.extension().is_some_and(|extension| extension == "md")
                && file.is_file()
                && !files.contains(&file)
            {
                files.push(file);
            }
        }
        files
    }
}

/// A command that the `commands` map of a plugin's manifest declares from a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MappedCommand {
    /// The map's key, the name the command is known by after the plugin's prefix.
    pub name: String,
    /// The entry's `description`, which stands in place of the file's.
    pub description: Option<String>,
}

/// The entry of the `commands` map in the manifest of the plugin at `root` whose `source` is
/// `file`. `None` when the manifest gives `commands` in another form or has no entry for the
/// file.
pub fn mapped_plugin_command(root: &Path, file: &Path) -> Option<MappedCommand> {
    let manifest = read_settings(&root.join(MANIFEST)).ok()??;
    let commands = manifest.get("commands")?.as_object()?;
    let file = canonical(file);
    commands.iter().find_map(|(name, entry)| {
        let source = entry.get("source")?.as_str()?;
        (canonical(&root.join(source.trim_start_matches("./"))) == file).then(|| MappedCommand {
            name: name.clone(),
            description: entry
                .get("description")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|description| !description.is_empty())
                .map(str::to_string),
        })
    })
}

/// `path` with its links resolved, or as given when it cannot be.
fn canonical(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// The plugin a file belongs to: the nearest directory above it with a Claude Code plugin
/// manifest.
pub fn plugin_root_of(file: &Path) -> Option<&Path> {
    file.ancestors()
        .skip(1)
        .find(|dir| dir.join(MANIFEST).is_file())
}

/// `contents` of `file`, a skill's or a command's, with its plugin's variables filled in as
/// [`PluginVariables::substitute_in_content`] fills them, when the file belongs to a Claude
/// Code plugin; as it is otherwise.
pub fn substitute_in_plugin_content(contents: String, file: &Path) -> String {
    match plugin_root_of(file).and_then(PluginVariables::load) {
        Some(plugin) => plugin.substitute_in_content(&contents),
        None => contents,
    }
}

/// Codex's home directory: `$CODEX_HOME` when set, else `~/.codex`, as Codex finds it.
fn codex_home() -> Option<PathBuf> {
    match std::env::var_os("CODEX_HOME").filter(|home| !home.is_empty()) {
        Some(home) => {
            let home = PathBuf::from(home);
            Some(std::fs::canonicalize(&home).unwrap_or(home))
        }
        None => std::env::var_os("HOME").map(|home| Path::new(&home).join(".codex")),
    }
}

/// Creates `dir` if it is missing; a failure is left to whatever then uses it.
fn ensure_dir(dir: &Path) {
    if let Err(err) = std::fs::create_dir_all(dir) {
        tracing::warn!(
            "failed to create plugin data directory {}: {err}",
            dir.display()
        );
    }
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
