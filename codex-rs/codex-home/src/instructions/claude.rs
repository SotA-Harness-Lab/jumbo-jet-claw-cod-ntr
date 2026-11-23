// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: the user's global Claude Code instructions, read in place.
//!
//! The files stay where Claude Code keeps them and are read again on every refresh, so
//! an edit there reaches Codex at the next request without a copy to keep in sync. They
//! are loaded and rendered as Claude Code does: `CLAUDE.md` with its `@` imports, then
//! the rules under `rules/` that are not limited to some files.

use codex_claude_compat::INSTRUCTIONS_HEADER;
use codex_claude_compat::LocalFiles;
use codex_claude_compat::render_instruction_files;
use codex_claude_compat::user_instructions;
use codex_utils_absolute_path::AbsolutePathBuf;

const PREAMBLE: &str = codex_prompt_texts::text!("codex-home/instructions/claude/preamble.md");

/// Returns the user's Claude Code configuration directory, `~/.claude`, or nothing when
/// `CCN_CLAUDE_COMPAT=0` turns the compatibility mode off.
pub fn default_claude_home() -> Option<AbsolutePathBuf> {
    if !codex_claude_compat::enabled() {
        return None;
    }
    let path = dirs::home_dir()?.join(codex_claude_compat::CLAUDE_DIR);
    AbsolutePathBuf::from_absolute_path_checked(path).ok()
}

/// Claude Code instructions rendered for Codex.
pub(super) struct ClaudeInstructions {
    pub text: String,
    /// The first file that contributed, reported as the source when Codex has no
    /// global AGENTS.md of its own.
    pub source: AbsolutePathBuf,
}

/// Loads the user's global Claude Code instructions. Missing files are not an error;
/// unreadable ones are reported in `warnings`.
pub(super) async fn load(
    claude_home: &AbsolutePathBuf,
    warnings: &mut Vec<String>,
) -> Option<ClaudeInstructions> {
    let home_dir = dirs::home_dir();
    let files =
        match user_instructions(&LocalFiles, claude_home.as_path(), home_dir.as_deref()).await {
            Ok(files) => files,
            Err(err) => {
                warnings.push(format!(
                    "Failed to read Claude Code instructions from `{}`: {err}",
                    claude_home.display()
                ));
                return None;
            }
        };
    let first = files.first()?;
    let source = AbsolutePathBuf::from_absolute_path_checked(&first.path).ok()?;
    Some(ClaudeInstructions {
        text: format!(
            "{PREAMBLE}\n\n{INSTRUCTIONS_HEADER}\n\n{}",
            render_instruction_files(&files)
        ),
        source,
    })
}
