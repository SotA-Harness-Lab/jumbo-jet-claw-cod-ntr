// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: read Claude Code's configuration in place

use std::collections::HashSet;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;

use codex_extension_api::Instructions;
use codex_extension_api::LoadInstructionsFuture;
use codex_extension_api::LoadedUserInstructions;
use codex_extension_api::UserInstructionsProvider;
use codex_utils_absolute_path::AbsolutePathBuf;

mod claude;

pub use claude::default_claude_home;

const DEFAULT_AGENTS_MD_FILENAME: &str = "AGENTS.md";
const LOCAL_AGENTS_MD_FILENAME: &str = "AGENTS.override.md";

/// Loads user instructions from a Codex home directory.
#[derive(Clone, Debug)]
pub struct CodexHomeUserInstructionsProvider {
    codex_home: AbsolutePathBuf,
    // Claude Code's configuration directory, when its instructions are shared with Codex.
    claude_home: Option<AbsolutePathBuf>,
    // Cached instructions and warning history are shared across provider clones.
    state: Arc<Mutex<InstructionsState>>,
}

#[derive(Debug, Default)]
struct InstructionsState {
    // Failed refreshes retain the last good value; confirmed absence still clears it.
    last_successful_instructions: Option<Instructions>,
    // Report ongoing failures once, then again if they return after recovery.
    active_warnings: HashSet<String>,
}

impl CodexHomeUserInstructionsProvider {
    /// Creates a provider rooted at the supplied absolute Codex home directory.
    pub fn new(codex_home: AbsolutePathBuf) -> Self {
        Self {
            codex_home,
            claude_home: None,
            state: Arc::new(Mutex::new(InstructionsState::default())),
        }
    }

    /// Also loads the user's global Claude Code instructions (`CLAUDE.md` and `rules/`)
    /// from `claude_home`, appended after Codex's own global AGENTS.md.
    pub fn with_claude_home(mut self, claude_home: Option<AbsolutePathBuf>) -> Self {
        self.claude_home = claude_home;
        self
    }

    #[tracing::instrument(name = "instructions.load", skip_all, fields(provider = "global"))]
    async fn load_from_codex_home(&self) -> LoadedUserInstructions {
        let mut warnings = Vec::new();
        let mut instructions = self.load_agents_md(&mut warnings).await;
        if let Some(claude_home) = &self.claude_home
            && let Some(claude) = claude::load(claude_home, &mut warnings).await
        {
            instructions = Some(match instructions {
                Some(own) => Instructions {
                    text: format!("{}\n\n{}", own.text, claude.text),
                    source: own.source,
                },
                None => Instructions {
                    text: claude.text,
                    source: Some(claude.source),
                },
            });
        }
        LoadedUserInstructions {
            instructions,
            warnings,
        }
    }

    async fn load_agents_md(&self, warnings: &mut Vec<String>) -> Option<Instructions> {
        for candidate in [LOCAL_AGENTS_MD_FILENAME, DEFAULT_AGENTS_MD_FILENAME] {
            let path = self.codex_home.join(candidate);
            match tokio::fs::metadata(path.as_path()).await {
                Ok(metadata) if !metadata.is_file() => continue,
                Ok(_) => {}
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            }
            let data = match tokio::fs::read(path.as_path()).await {
                Ok(data) => data,
                Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
                Err(err) => {
                    warnings.push(format!(
                        "Failed to read global AGENTS.md instructions from `{}`: {err}",
                        path.display()
                    ));
                    continue;
                }
            };
            let contents = String::from_utf8_lossy(&data);
            let trimmed = contents.trim();
            if !trimmed.is_empty() {
                return Some(Instructions {
                    text: trimmed.to_string(),
                    source: Some(path),
                });
            }
        }
        None
    }
}

impl UserInstructionsProvider for CodexHomeUserInstructionsProvider {
    fn load_user_instructions(&self) -> LoadInstructionsFuture<'_> {
        Box::pin(async move {
            let mut loaded = self.load_from_codex_home().await;
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if loaded.instructions.is_some() || loaded.warnings.is_empty() {
                state
                    .last_successful_instructions
                    .clone_from(&loaded.instructions);
            } else {
                loaded.instructions = state.last_successful_instructions.clone();
            }
            let previous_warnings = std::mem::take(&mut state.active_warnings);
            loaded.warnings.retain(|warning| {
                state.active_warnings.insert(warning.clone())
                    && !previous_warnings.contains(warning)
            });
            loaded
        })
    }
}

#[cfg(test)]
mod tests;
