//! The plugins the user installed and enabled in Claude Code, and which of them Codex uses.
//!
//! Claude Code records installed plugins in `~/.claude/plugins/installed_plugins.json` and turns
//! them on and off with `enabledPlugins` in its settings files. A plugin is loaded only when the
//! merged `enabledPlugins` maps its `<plugin>@<marketplace>` id to `true`; an id that is absent or
//! `false` is not. The plugin is read from the `installPath` of the first installation recorded
//! for it, as Claude Code does at startup.
//!
//! Codex's own plugin loader reads Claude Code's plugin format; this module only decides which
//! plugins to hand it. Every one is handed over, whatever it contains, except a plugin that relies
//! on a mechanism of Claude Code's that Codex does not have (see [`incompatibility`]), which
//! could misbehave in Codex; the user can still turn such a plugin on in `ccn-config.toml`.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::mod_config::MOD_CONFIG_FILE;
use crate::settings::read_settings;
use crate::settings::settings_paths;

/// Claude Code's record of installed plugins, relative to `~/.claude`.
pub const INSTALLED_PLUGINS_FILE: &str = "plugins/installed_plugins.json";

/// The marketplace of the plugins built into Claude Code, which have no install path.
const BUILTIN_MARKETPLACE: &str = "builtin";

/// Where a Claude Code plugin keeps its manifest.
const CLAUDE_MANIFEST: &str = ".claude-plugin/plugin.json";
/// Manifests Codex reads before Claude Code's, in the plugin's own directory.
const CODEX_MANIFESTS: &[&str] = &[".codex-plugin/plugin.json"];

/// Claude Code settings that configure only Claude Code's own interface, which Codex does not
/// have. A plugin file that sets one in Claude Code's settings makes the plugin incompatible.
const UI_SETTINGS: &[&str] = &[
    "statusLine",
    "statusLineEnabled",
    "outputStyle",
    "spinnerVerbs",
    "spinnerTipsEnabled",
    "spinnerTipsOverride",
    "companyAnnouncements",
    "prefersReducedMotion",
    "syntaxHighlightingDisabled",
    "terminalTitleFromRename",
];
/// Claude Code's settings files, by the names a plugin file would use for them.
const SETTINGS_FILES: &[&str] = &["settings.json", "settings.local.json"];
/// The plugin's directories whose files are read for [`UI_SETTINGS`]: what the model is told
/// (commands, agents, skills) and what runs (hooks, scripts), besides the paths its manifest
/// declares for the first four.
const SCANNED_DIRS: &[&str] = &["commands", "agents", "skills", "hooks", "scripts"];
const SCANNED_FIELDS: &[&str] = &["commands", "agents", "skills", "hooks"];
/// How deep below a scanned directory, and how large, files are read.
const MAX_SCAN_DEPTH: usize = 6;
const MAX_SCAN_BYTES: u64 = 1024 * 1024;

/// A plugin installed and enabled in Claude Code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClaudePlugin {
    /// `<plugin>@<marketplace>`, the id Claude Code and Codex both use.
    pub id: String,
    /// The installed plugin's directory.
    pub install_path: PathBuf,
    /// Whether the user's own `~/.claude/settings.json` enables it, as opposed to a project's
    /// settings alone.
    pub enabled_by_user: bool,
}

/// The install path of each installed plugin in `installed_plugins.json` (version 2): the
/// first installation recorded for the id, whatever its scope, as Claude Code reads it.
pub fn installed_plugin_paths(installed: &Value) -> BTreeMap<String, PathBuf> {
    let Some(plugins) = installed.get("plugins").and_then(Value::as_object) else {
        return BTreeMap::new();
    };
    plugins
        .iter()
        .filter_map(|(id, installations)| {
            let path = installations
                .as_array()?
                .first()?
                .get("installPath")?
                .as_str()?;
            (!path.is_empty()).then(|| (id.clone(), PathBuf::from(path)))
        })
        .collect()
}

/// The ids `enabledPlugins` turns on, merged over the settings files lowest precedence
/// first (a later file's value for an id wins), each with whether the first file, the user's,
/// says `true` for it. Ids not of the form `<plugin>@<marketplace>`, and Claude Code's
/// built-in plugins, are left out.
pub fn enabled_plugin_ids(settings: &[Value], first_is_user: bool) -> BTreeMap<String, bool> {
    let mut merged = BTreeMap::<String, bool>::new();
    for file in settings {
        let Some(enabled) = file.get("enabledPlugins").and_then(Value::as_object) else {
            continue;
        };
        for (id, value) in enabled {
            // Claude Code takes only `true` as enabled; any other value turns the id off.
            merged.insert(id.clone(), value.as_bool() == Some(true));
        }
    }
    let user_enabled = |id: &str| {
        first_is_user
            && settings
                .first()
                .and_then(|file| file.get("enabledPlugins"))
                .and_then(|enabled| enabled.get(id))
                .and_then(Value::as_bool)
                == Some(true)
    };
    merged
        .into_iter()
        .filter(|(id, enabled)| *enabled && is_plugin_id(id))
        .map(|(id, _)| {
            let by_user = user_enabled(&id);
            (id, by_user)
        })
        .collect()
}

fn is_plugin_id(id: &str) -> bool {
    match id.split_once('@') {
        Some((plugin, marketplace)) => {
            !plugin.is_empty()
                && !marketplace.is_empty()
                && !marketplace.contains('@')
                && marketplace != BUILTIN_MARKETPLACE
        }
        None => false,
    }
}

/// The plugins both installed and enabled, sorted by id, and a warning for each enabled id
/// that is not installed.
pub fn installed_enabled_plugins(
    installed: &Value,
    settings: &[Value],
    first_is_user: bool,
) -> (Vec<ClaudePlugin>, Vec<String>) {
    let paths = installed_plugin_paths(installed);
    let mut plugins = Vec::new();
    let mut warnings = Vec::new();
    for (id, enabled_by_user) in enabled_plugin_ids(settings, first_is_user) {
        match paths.get(&id) {
            Some(install_path) => plugins.push(ClaudePlugin {
                id,
                install_path: install_path.clone(),
                enabled_by_user,
            }),
            None => warnings.push(format!(
                "Claude Code plugin {id} skipped: it is enabled but not installed"
            )),
        }
    }
    (plugins, warnings)
}

/// Reads Claude Code's records under `claude_home` (`~/.claude`) and the settings of
/// `project_dir`, and returns the installed and enabled plugins with the warnings met on the
/// way. Missing files mean no plugins.
pub fn load_claude_plugins(
    claude_home: &Path,
    project_dir: Option<&Path>,
) -> (Vec<ClaudePlugin>, Vec<String>) {
    let mut warnings = Vec::new();
    let installed_path = claude_home.join(INSTALLED_PLUGINS_FILE);
    let installed = match read_settings(&installed_path) {
        Ok(Some(installed)) => installed,
        Ok(None) => return (Vec::new(), warnings),
        Err(err) => {
            warnings.push(format!(
                "Failed to read Claude Code plugins in {}: {err}",
                installed_path.display()
            ));
            return (Vec::new(), warnings);
        }
    };
    let mut settings = Vec::new();
    // The user's file is always first, read or not, so that its place is known.
    for (index, path) in settings_paths(Some(claude_home), project_dir)
        .iter()
        .enumerate()
    {
        match read_settings(path) {
            Ok(Some(file)) => settings.push(file),
            Ok(None) if index == 0 => settings.push(Value::Null),
            Ok(None) => {}
            Err(err) => {
                warnings.push(format!(
                    "Failed to read Claude Code settings {}: {err}",
                    path.display()
                ));
                if index == 0 {
                    settings.push(Value::Null);
                }
            }
        }
    }
    let (plugins, missing) =
        installed_enabled_plugins(&installed, &settings, /*first_is_user*/ true);
    warnings.extend(missing);
    (plugins, warnings)
}

/// Why the plugin in `root` is incompatible with Codex, or `None` when it is not. A plugin is
/// incompatible when it relies on a mechanism of Claude Code's that Codex does not have, so that
/// in Codex it could misbehave or change things wrongly:
///
/// - its manifest or its conventional files declare a component Codex has no counterpart for:
///   output styles (`outputStyles`, `output-styles/`), LSP servers (`lspServers`, `.lsp.json`)
///   or channels (`channels`);
/// - or one of its files configures Claude Code's own interface: a file under `commands/`,
///   `agents/`, `skills/`, `hooks/` or `scripts/` (or a path the manifest declares for the first
///   four), README files and hidden or `node_modules` directories left out, that names one of
///   Claude Code's settings files (`settings.json`, `settings.local.json`) and, as a whole word,
///   one of the settings that configure only Claude Code's interface ([`UI_SETTINGS`]:
///   `statusLine`, `outputStyle` and the like).
pub fn incompatibility(root: &Path) -> Option<String> {
    let manifest = std::fs::read_to_string(root.join(CLAUDE_MANIFEST))
        .ok()
        .and_then(|contents| serde_json::from_str::<Value>(&contents).ok());
    let field = |name: &str| manifest.as_ref().and_then(|manifest| manifest.get(name));

    let output_styles = match field("outputStyles") {
        Some(declared) => declared_present(declared, root),
        None => root.join("output-styles").exists(),
    };
    for (present, component) in [
        (output_styles, "output styles"),
        (
            root.join(".lsp.json").is_file()
                || field("lspServers").is_some_and(|declared| declared_present(declared, root)),
            "LSP servers",
        ),
        (
            field("channels").is_some_and(|declared| declared_present(declared, root)),
            "channels",
        ),
    ] {
        if present {
            return Some(format!(
                "it declares {component}, which Codex does not have"
            ));
        }
    }

    let declared = SCANNED_FIELDS
        .iter()
        .copied()
        .filter_map(&field)
        .flat_map(relative_paths);
    for path in SCANNED_DIRS
        .iter()
        .map(ToString::to_string)
        .chain(declared)
    {
        if path.split(['/', '\\']).any(|part| part == "..") {
            continue;
        }
        let path = path.trim_start_matches("./");
        if let Some((file, setting)) = ui_setting_in(&root.join(path), MAX_SCAN_DEPTH) {
            let file = file.strip_prefix(root).unwrap_or(file.as_path());
            return Some(format!(
                "its {} configures Claude Code's `{setting}` setting, which Codex does not have",
                file.display()
            ));
        }
    }
    None
}

/// Whether a manifest field declares something: a path that exists, or a non-empty inline
/// value.
fn declared_present(declared: &Value, root: &Path) -> bool {
    match declared {
        Value::String(path) => root.join(path).exists(),
        Value::Array(entries) => entries.iter().any(|entry| declared_present(entry, root)),
        Value::Object(entries) => !entries.is_empty(),
        _ => false,
    }
}

fn relative_paths(declared: &Value) -> Vec<String> {
    match declared {
        Value::String(path) => vec![path.clone()],
        Value::Array(paths) => paths
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

/// The first file at or below `path` (in name order, `depth` levels down) that configures one of
/// [`UI_SETTINGS`], with that setting.
fn ui_setting_in(path: &Path, depth: usize) -> Option<(PathBuf, &'static str)> {
    let name = path.file_name()?.to_string_lossy().to_lowercase();
    if name.starts_with('.') || name == "node_modules" || name.starts_with("readme") {
        return None;
    }
    if path.is_file() {
        let small = std::fs::metadata(path).is_ok_and(|meta| meta.len() <= MAX_SCAN_BYTES);
        let contents = small
            .then(|| std::fs::read_to_string(path).ok())
            .flatten()?;
        return ui_setting(&contents).map(|setting| (path.to_path_buf(), setting));
    }
    if depth == 0 {
        return None;
    }
    let mut entries = std::fs::read_dir(path)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    entries.sort();
    entries
        .iter()
        .find_map(|entry| ui_setting_in(entry, depth - 1))
}

/// The setting of [`UI_SETTINGS`] that `contents` configures in Claude Code's settings: one it
/// names as a whole word, in a text that also names one of Claude Code's settings files.
fn ui_setting(contents: &str) -> Option<&'static str> {
    if !SETTINGS_FILES.iter().any(|file| contents.contains(file)) {
        return None;
    }
    UI_SETTINGS
        .iter()
        .copied()
        .find(|setting| contains_word(contents, setting))
}

/// Whether `word` occurs in `text` with no letter, digit or `_` right before or after it.
fn contains_word(text: &str, word: &str) -> bool {
    let is_word_byte = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
    text.match_indices(word).any(|(start, _)| {
        let end = start + word.len();
        let before = start
            .checked_sub(1)
            .and_then(|index| text.as_bytes().get(index));
        let after = text.as_bytes().get(end);
        !before.is_some_and(|byte| is_word_byte(*byte))
            && !after.is_some_and(|byte| is_word_byte(*byte))
    })
}

/// The plugins Codex uses, and a warning for each one it leaves out: one whose install path is
/// gone or that has no manifest Codex can read (Codex cannot load those), and one that is
/// [incompatible](incompatibility) with Codex, unless `enable` (`[claude.plugins] enable` of
/// `ccn-config.toml`) lists its id. Every other plugin is used, whatever it contains.
pub fn applicable_plugins(
    plugins: Vec<ClaudePlugin>,
    enable: &[String],
) -> (Vec<ClaudePlugin>, Vec<String>) {
    let mut applicable = Vec::new();
    let mut warnings = Vec::new();
    for plugin in plugins {
        let root = &plugin.install_path;
        if !root.is_dir() {
            warnings.push(format!(
                "Claude Code plugin {} skipped: its install path {} does not exist",
                plugin.id,
                root.display()
            ));
            continue;
        }
        if !CODEX_MANIFESTS
            .iter()
            .chain([&CLAUDE_MANIFEST])
            .any(|manifest| manifest_parses(&root.join(manifest)))
        {
            warnings.push(format!(
                "Claude Code plugin {} skipped: Codex needs a readable {CLAUDE_MANIFEST}",
                plugin.id
            ));
            continue;
        }
        if !enable.contains(&plugin.id)
            && let Some(reason) = incompatibility(root)
        {
            warnings.push(format!(
                "Claude Code plugin {id} disabled: {reason}. To use it anyway, add \"{id}\" to \
                 `enable` under `[claude.plugins]` in $CODEX_HOME/{MOD_CONFIG_FILE}",
                id = plugin.id,
            ));
            continue;
        }
        applicable.push(plugin);
    }
    (applicable, warnings)
}

fn manifest_parses(path: &Path) -> bool {
    std::fs::read_to_string(path).ok().is_some_and(|contents| {
        serde_json::from_str::<Value>(&contents).is_ok_and(|v| v.is_object())
    })
}

#[cfg(test)]
#[path = "plugins_tests.rs"]
mod tests;
