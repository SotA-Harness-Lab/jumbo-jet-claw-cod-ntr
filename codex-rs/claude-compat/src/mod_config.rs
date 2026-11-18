//! The fork's own settings, kept in `$CODEX_HOME/ccn-config.toml` so that Codex's `config.toml`
//! holds only upstream Codex's settings.
//!
//! ```toml
//! [claude]
//! tools = true                       # Claude Code's tool set in place of Codex's
//!
//! [claude.plugins]
//! enable = ["claude-hud@claude-hud"] # Claude Code plugins used even when found incompatible
//! ```
//!
//! A missing file means the defaults. A file that does not parse, or names a key this fork does
//! not know, is reported as a startup warning and the defaults are used: it never stops Codex
//! from starting.

use std::io;
use std::path::Path;

use serde::Deserialize;

/// The file's name in `$CODEX_HOME`.
pub const MOD_CONFIG_FILE: &str = "ccn-config.toml";

/// The contents of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ModConfig {
    /// `[claude]`: how Claude Code's configuration is used.
    pub claude: ClaudeModConfig,
}

/// `[claude]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ClaudeModConfig {
    /// `tools`: offer the model Claude Code's tool set in place of Codex's. Off by default.
    pub tools: bool,
    /// `[claude.plugins]`: which of Claude Code's plugins are used.
    pub plugins: ClaudePluginsModConfig,
}

/// `[claude.plugins]` of `ccn-config.toml`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ClaudePluginsModConfig {
    /// `enable`: `<plugin>@<marketplace>` ids used even when found incompatible with Codex.
    pub enable: Vec<String>,
}

impl ModConfig {
    /// Parses the contents of `ccn-config.toml`.
    pub fn parse(contents: &str) -> Result<Self, String> {
        toml::from_str(contents).map_err(|err| err.to_string().trim_end().to_string())
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
        match Self::parse(&contents) {
            Ok(config) => (config, None),
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
    /// `CODEX_CLAUDE_COMPAT` is not `0`.
    pub fn claude_tools(&self) -> bool {
        self.claude.tools && crate::enabled()
    }
}

#[cfg(test)]
#[path = "mod_config_tests.rs"]
mod tests;
