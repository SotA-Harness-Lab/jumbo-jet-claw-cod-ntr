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
//! [ui]
//! language = "zh-CN"                 # the TUI's language; the system's when left out
//!
//! [harness]
//! output_limit_continuations = 3     # how often a response cut off at the output limit is continued
//! background_on_timeout = true       # a Bash command past its timeout keeps running in the background
//! spill_long_output = true           # command output too long for the model is kept in a file
//! background_tools = true            # the Monitor, CronCreate, CronList and CronDelete tools
//! file_checkpoints = true            # keep what edits replace, to put files back on a rewind
//! ```
//!
//! A missing file means the defaults. A file that does not parse is reported as a startup
//! warning and the defaults are used: it never stops Codex from starting. A key or section this
//! program does not know is left alone, with a line in the log: another build of the program,
//! or a later version, may share the file and keep settings of its own there.

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
    /// `[ui]`: the terminal interface.
    pub ui: UiModConfig,
    /// `[harness]`: how the agent itself works.
    pub harness: HarnessModConfig,
}

/// `[harness]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct HarnessModConfig {
    /// `output_limit_continuations`: how many responses in a row, cut off at the output token
    /// limit, are continued before the turn fails as it used to. `0` never continues; 3 when
    /// left out.
    pub output_limit_continuations: Option<u32>,
    /// `background_on_timeout`: whether a `Bash` command that runs into its timeout is left
    /// running in the background, where its output can still be read, in place of being
    /// stopped. On when left out.
    pub background_on_timeout: Option<bool>,
    /// `spill_long_output`: whether the output of a command that is too long to show the
    /// model whole is kept in a file whose place the model is told. On when left out.
    pub spill_long_output: Option<bool>,
    /// `background_tools`: whether the model is offered `Monitor`, to be told what a
    /// long-running command writes, and `CronCreate`, `CronList` and `CronDelete`, to
    /// schedule prompts for itself. On when left out.
    pub background_tools: Option<bool>,
    /// `file_checkpoints`: whether what the agent's edits replace is kept, so that rewinding
    /// the conversation can put the files back. On when left out.
    pub file_checkpoints: Option<bool>,
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
