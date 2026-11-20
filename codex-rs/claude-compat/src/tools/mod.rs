// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's built-in tools as the model sees them: their descriptions, the meaning of
//! their parameters and the text of their results.
//!
//! Codex runs these tools on its own machinery (see `core/src/tools/claude/`); this module
//! holds only the parts that do not depend on it, so they can be tested on their own. Each
//! tool has its own file; a new tool adds a file and a line below, nothing else.

pub mod agent;
pub mod ask_user_question;
pub mod bash;
pub mod cron;
pub mod edit;
pub mod glob;
pub mod grep;
pub mod input;
pub mod patch;
pub mod read;
pub mod ripgrep;
pub mod skill;
pub mod todo_write;
pub mod web_fetch;
pub mod web_search;
pub mod write;

/// Claude Code's name of its shell tool.
pub const BASH: &str = "Bash";
/// Claude Code's name of its file reading tool.
pub const READ: &str = "Read";
/// Claude Code's name of its file writing tool.
pub const WRITE: &str = "Write";
/// Claude Code's name of its string replacement tool.
pub const EDIT: &str = "Edit";
/// Claude Code's name of its file search tool.
pub const GLOB: &str = "Glob";
/// Claude Code's name of its content search tool.
pub const GREP: &str = "Grep";
/// Claude Code's name of its task list tool.
pub const TODO_WRITE: &str = "TodoWrite";
/// Claude Code's name of its subagent tool.
pub const AGENT: &str = "Agent";
/// Claude Code's name of its web search tool.
pub const WEB_SEARCH: &str = "WebSearch";
/// Claude Code's name of its URL fetching tool.
pub const WEB_FETCH: &str = "WebFetch";
/// Claude Code's name of its tool asking the user multiple-choice questions.
pub const ASK_USER_QUESTION: &str = "AskUserQuestion";
/// Claude Code's tool for invoking a skill.
pub const SKILL: &str = "Skill";

/// Wraps a message the way Claude Code reports an input it rejects before running the tool.
pub fn tool_use_error(message: &str) -> String {
    format!("<tool_use_error>{message}</tool_use_error>")
}

/// Claude Code's message for a path that does not exist.
pub fn file_not_found_message(cwd: &str) -> String {
    format!("File does not exist. Note: your current working directory is {cwd}.")
}

/// Claude Code's rendering of a byte count (`formatFileSize`).
pub fn format_file_size(size: u64) -> String {
    let kb = size as f64 / 1024.0;
    if kb < 1.0 {
        return format!("{size} bytes");
    }
    let one_decimal = |value: f64| {
        let text = format!("{value:.1}");
        text.strip_suffix(".0").map(str::to_string).unwrap_or(text)
    };
    if kb < 1024.0 {
        return format!("{}KB", one_decimal(kb));
    }
    let mb = kb / 1024.0;
    if mb < 1024.0 {
        return format!("{}MB", one_decimal(mb));
    }
    format!("{}GB", one_decimal(mb / 1024.0))
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
