// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: the user's global Claude Code instructions, read in place.
//!
//! The files stay where Claude Code keeps them and are read again on every refresh, so
//! an edit there reaches Codex at the next request without a copy to keep in sync. They
//! are loaded and rendered as Claude Code does: `CLAUDE.md` with its `@` imports, then
//! the rules under `rules/` that are not limited to some files.

use codex_claude_compat::INSTRUCTIONS_HEADER;
use codex_claude_compat::InstructionExcludes;
use codex_claude_compat::LocalFiles;
use codex_claude_compat::render_instruction_files;
use codex_claude_compat::user_instructions;
use codex_utils_absolute_path::AbsolutePathBuf;

const PREAMBLE: &str = "\
The instructions below come from Claude Code's configuration and are shared with Codex, \
binding here as well. They are maintained for Claude Code: do not edit those files unless \
the user asks. Where they name Claude Code features, `/<name>` and the Skill tool mean the \
skill of that name; the hooks and `settings.json` entries they describe may not all run in \
Codex, so follow the rules those would have enforced yourself.";

/// Returns the user's Claude Code configuration directory from the launch environment,
/// with `~/.claude` as the default. Returns nothing when compatibility is disabled.
pub fn default_claude_home() -> Option<AbsolutePathBuf> {
    if !codex_claude_compat::enabled() {
        return None;
    }
    let path = codex_claude_compat::user_config_dir()?;
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
    let source = InstructionExcludes::load(
        &LocalFiles,
        Some(claude_home.as_path()),
        home_dir.as_deref(),
        /*project_dir*/ None,
    );
    let files = match user_instructions(&source, claude_home.as_path(), home_dir.as_deref()).await {
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
