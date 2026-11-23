// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Glob` tool: description, parameters, the ripgrep arguments it runs and
//! the text of its results.

use serde_json::Value;
use serde_json::json;

use super::AGENT;
use super::GLOB;
use super::input::Input;
use super::ripgrep::relative_to;

/// The most files a search returns (Claude Code's default).
pub const MAX_RESULTS: usize = 100;

pub const PATTERN_PARAM_DESCRIPTION: &str = "The glob pattern to match files against";
pub const PATH_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/glob/path_param_description.md");

pub const TRUNCATED_NOTE: &str =
    "(Results are truncated. Consider using a more specific path or pattern.)";

/// The tool description, after Claude Code's. `available` lists the Claude Code tools
/// offered, so that the line pointing to the `Agent` tool is there only with it.
pub fn description(available: &[&str]) -> String {
    let mut text = String::from(codex_prompt_texts::text!(
        "claude-compat/tools/glob/description/fast_file_pattern_matching_tool.md"
    ));
    if available.contains(&AGENT) {
        text.push_str(&format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/glob/description/when_you_are_doing_an.md"
            ),
            AGENT = AGENT
        ));
    }
    text
}

/// A `Glob` call's input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobInput {
    pub pattern: String,
    pub path: Option<String>,
}

impl GlobInput {
    /// Reads the model's arguments; `Err` is Claude Code's input validation message.
    pub fn parse(arguments: Value) -> Result<Self, String> {
        let mut input = Input::new(GLOB, arguments, &["pattern", "path"]);
        let pattern = input.required_string("pattern");
        let path = input.string("path");
        input.finish()?;
        Ok(Self {
            pattern,
            path: path.filter(|path| !path.is_empty()),
        })
    }
}

/// Where to search and with which pattern. ripgrep's `--glob` takes relative patterns, so
/// an absolute pattern is split, as Claude Code splits it, into the directory before its
/// first wildcard and the rest; `path` is then not used. `None` keeps `path` (or the
/// working directory) and the pattern as given.
pub fn split_absolute_pattern(pattern: &str) -> Option<(String, String)> {
    if !is_absolute(pattern) {
        return None;
    }
    // A literal path splits into its directory and its name.
    let wildcard = pattern.find(['*', '?', '[', '{']).unwrap_or(pattern.len());
    let split = pattern[..wildcard].rfind(['/', '\\'])?;
    let directory = if split == 0 {
        pattern[..1].to_string()
    } else {
        drive_root(&pattern[..split])
    };
    Some((directory, pattern[split + 1..].to_string()))
}

fn is_absolute(path: &str) -> bool {
    let bytes = path.as_bytes();
    path.starts_with(['/', '\\'])
        || (bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && (bytes[2] == b'/' || bytes[2] == b'\\'))
}

/// `C:` names the current directory of drive C; its root is `C:\`.
fn drive_root(directory: &str) -> String {
    let bytes = directory.as_bytes();
    if bytes.len() == 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
        format!("{directory}\\")
    } else {
        directory.to_string()
    }
}

/// The ripgrep arguments listing the files that match `pattern`, oldest first, before the
/// directory searched: Claude Code's defaults, which include hidden files and files that
/// `.gitignore` names.
pub fn rg_args(pattern: &str) -> Vec<String> {
    [
        "--files",
        "--glob",
        pattern,
        "--sort=modified",
        "--no-ignore",
        "--hidden",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

/// Claude Code's `Directory does not exist` message.
pub fn directory_not_found_message(path: &str, cwd: &str) -> String {
    format!("Directory does not exist: {path}. Note: your current working directory is {cwd}.")
}

/// Claude Code's message for a `path` that is a file.
pub fn not_a_directory_message(path: &str) -> String {
    format!("Path is not a directory: {path}")
}

/// The result of a search: the text for the model and Claude Code's `tool_response`.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobResult {
    pub text: String,
    pub tool_response: Value,
}

/// Formats the paths ripgrep listed as Claude Code does: the first [`MAX_RESULTS`], paths
/// inside `cwd` shortened, a note when more were found, `No files found` when none were.
pub fn format_result(paths: Vec<String>, cwd: &str, duration_ms: u64) -> GlobResult {
    let truncated = paths.len() > MAX_RESULTS;
    let filenames: Vec<String> = paths
        .iter()
        .take(MAX_RESULTS)
        .map(|path| relative_to(path, cwd))
        .collect();
    let text = if filenames.is_empty() {
        "No files found".to_string()
    } else {
        let mut lines = filenames.clone();
        if truncated {
            lines.push(TRUNCATED_NOTE.to_string());
        }
        lines.join("\n")
    };
    GlobResult {
        text,
        tool_response: json!({
            "durationMs": duration_ms,
            "numFiles": filenames.len(),
            "filenames": filenames,
            "truncated": truncated,
        }),
    }
}

#[cfg(test)]
#[path = "glob_tests.rs"]
mod tests;
