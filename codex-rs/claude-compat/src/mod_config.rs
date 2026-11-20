// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The fork's own settings, kept in `$CODEX_HOME/ccn-config.toml` so that Codex's `config.toml`
//! holds only upstream Codex's settings.
//!
//! ```toml
//! [claude]
//! tools = true                       # Claude Code's tool set in place of Codex's
//!
//! [claude.plugins]
//! enable = ["claude-hud@claude-hud"] # Claude Code plugins used even when found incompatible
//!
//! [hooks.models]                     # Claude models named by `prompt` and `agent` hooks,
//! haiku = "gpt-5.5-mini"             # and the Codex model that judges in their place
//!
//! [ui]
//! language = "zh-CN"                 # the TUI's language; the system's when left out
//! ```
//!
//! A missing file means the defaults. A file that does not parse is reported as a startup
//! warning and the defaults are used: it never stops Codex from starting. A key or section this
//! program does not know is left alone, with a line in the log: another build of the program,
//! or a later version, may share the file and keep settings of its own there.

use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use serde::Deserialize;

/// The file's name in `$CODEX_HOME`.
pub const MOD_CONFIG_FILE: &str = "ccn-config.toml";

/// The contents of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ModConfig {
    /// `[claude]`: how Claude Code's configuration is used.
    pub claude: ClaudeModConfig,
    /// `[hooks]`: how Claude Code's hooks are run.
    pub hooks: HooksModConfig,
    /// `[ui]`: the terminal interface.
    pub ui: UiModConfig,
}

/// `[hooks]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct HooksModConfig {
    /// `[hooks.models]`: which Codex model judges a `prompt` or `agent` hook that names a
    /// Claude model. See [`HooksModConfig::model_for`].
    pub models: BTreeMap<String, String>,
}

/// The entry of `[hooks.models]` for a handler that names no model, for which Claude Code
/// uses its small, fast one.
pub const DEFAULT_HOOK_MODEL: &str = "default";

impl HooksModConfig {
    /// The Codex model to judge a hook whose handler names the Claude model `model`: the
    /// entry for that very name, or else the longest entry whose name `model` contains, so
    /// that `haiku` covers `claude-haiku-4-5`. A handler without `model` takes the entry
    /// `default`. `None` when no entry fits: the handler's `model` is then ignored and the
    /// session's own model judges.
    pub fn model_for(&self, model: Option<&str>) -> Option<&str> {
        let wanted = model
            .map(str::trim)
            .filter(|model| !model.is_empty())
            .unwrap_or(DEFAULT_HOOK_MODEL)
            .to_ascii_lowercase();
        let entries = self
            .models
            .iter()
            .filter_map(|(name, codex_model)| {
                let name = name.trim().to_ascii_lowercase();
                let codex_model = codex_model.trim();
                (!name.is_empty() && !codex_model.is_empty()).then_some((name, codex_model))
            })
            .collect::<Vec<(String, &str)>>();
        entries
            .iter()
            .find(|(name, _)| *name == wanted)
            .or_else(|| {
                entries
                    .iter()
                    .filter(|(name, _)| name != DEFAULT_HOOK_MODEL && wanted.contains(name.as_str()))
                    .max_by_key(|(name, _)| name.len())
            })
            .map(|(_, codex_model)| *codex_model)
    }
}

/// `[ui]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct UiModConfig {
    /// `language`: the TUI's language as a BCP 47 tag (`en`, `zh-CN`). When left out, the
    /// system's language is used, and English when the TUI has no translation for it.
    pub language: Option<String>,
}

/// `[claude]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ClaudeModConfig {
    /// `tools`: offer the model Claude Code's tool set in place of Codex's. Off by default.
    pub tools: bool,
    /// `[claude.plugins]`: which of Claude Code's plugins are used.
    pub plugins: ClaudePluginsModConfig,
}

/// `[claude.plugins]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct ClaudePluginsModConfig {
    /// `enable`: `<plugin>@<marketplace>` ids used even when found incompatible with Codex.
    pub enable: Vec<String>,
}

impl ModConfig {
    /// Parses the contents of `ccn-config.toml`, leaving out what this program does not know.
    pub fn parse(contents: &str) -> Result<Self, String> {
        Self::parse_with_unknown_keys(contents).map(|(config, _)| config)
    }

    /// Parses the contents of `ccn-config.toml` and names the keys and sections in it that
    /// this program does not know, as dotted paths.
    pub fn parse_with_unknown_keys(contents: &str) -> Result<(Self, Vec<String>), String> {
        let error = |err: toml::de::Error| err.to_string().trim_end().to_string();
        let value = toml::from_str::<toml::Value>(contents).map_err(error)?;
        let mut unknown = Vec::new();
        let config = serde_ignored::deserialize(value, |path| unknown.push(path.to_string()))
            .map_err(error)?;
        // By name, whatever order the sections are read in.
        unknown.sort();
        Ok((config, unknown))
    }

    /// Reads `ccn-config.toml` in `codex_home`: the defaults when it is missing, and the defaults with
    /// a warning when it cannot be read or parsed.
    pub fn load(codex_home: &Path) -> (Self, Option<String>) {
        let path = codex_home.join(MOD_CONFIG_FILE);
        let contents = match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(err) if err.kind() == io::ErrorKind::NotFound => return (Self::default(), None),
            Err(err) => {
                return (
                    Self::default(),
                    Some(format!(
                        "Failed to read {}: {err}; using the defaults",
                        path.display()
                    )),
                );
            }
        };
        match Self::parse_with_unknown_keys(&contents) {
            Ok((config, unknown)) => {
                if !unknown.is_empty() {
                    tracing::warn!(
                        "ignoring settings this program does not know in {}: {}",
                        path.display(),
                        unknown.join(", ")
                    );
                }
                (config, None)
            }
            Err(err) => (
                Self::default(),
                Some(format!(
                    "Ignoring {}, which is not valid: {err}; using the defaults",
                    path.display()
                )),
            ),
        }
    }

    /// Whether the model is offered Claude Code's tool set: `[claude] tools = true`, and
    /// `CCN_CLAUDE_COMPAT` is not `0`.
    pub fn claude_tools(&self) -> bool {
        self.claude.tools && crate::enabled()
    }
}

#[cfg(test)]
#[path = "mod_config_tests.rs"]
mod tests;
