// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Grep` tool: description, parameters, the ripgrep arguments it runs and
//! the text of its results.

use serde_json::Value;
use serde_json::json;

use super::AGENT;
use super::BASH;
use super::GREP;
use super::input::Input;
use super::ripgrep::VCS_DIRECTORIES;
use super::ripgrep::apply_head_limit;
use super::ripgrep::relative_to;

pub const PATTERN_PARAM_DESCRIPTION: &str =
    "The regular expression pattern to search for in file contents";
pub const PATH_PARAM_DESCRIPTION: &str =
    "File or directory to search in (rg PATH). Defaults to current working directory.";
pub const GLOB_PARAM_DESCRIPTION: &str =
    r#"Glob pattern to filter files (e.g. "*.js", "*.{ts,tsx}") - maps to rg --glob"#;
pub const OUTPUT_MODE_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/output_mode_param_description.md");
pub const BEFORE_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/before_param_description.md");
pub const AFTER_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/after_param_description.md");
pub const C_PARAM_DESCRIPTION: &str = "Alias for context.";
pub const CONTEXT_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/context_param_description.md");
pub const LINE_NUMBERS_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/line_numbers_param_description.md");
pub const CASE_INSENSITIVE_PARAM_DESCRIPTION: &str = "Case insensitive search (rg -i)";
pub const TYPE_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/type_param_description.md");
pub const HEAD_LIMIT_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/head_limit_param_description.md");
pub const OFFSET_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/offset_param_description.md");
pub const MULTILINE_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/grep/multiline_param_description.md");

/// The values of `output_mode`.
pub const OUTPUT_MODES: &[&str] = &["content", "files_with_matches", "count"];

/// The tool description, after Claude Code's. `available` lists the Claude Code tools
/// offered, so that the line pointing to the `Agent` tool is there only with it.
pub fn description(available: &[&str]) -> String {
    let mut text = format!(
        codex_prompt_texts::text!(
            "claude-compat/tools/grep/description/a_powerful_search_tool_built.md"
        ),
        GREP = GREP,
        BASH = BASH
    );
    if available.contains(&AGENT) {
        text.push_str(&format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/grep/description/use_agent_tool_for_open.md"
            ),
            AGENT = AGENT
        ));
    }
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/grep/description/pattern_syntax_uses_ripgrep_not.md"
    ));
    text
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    Content,
    FilesWithMatches,
    Count,
}

impl OutputMode {
    pub fn name(self) -> &'static str {
        match self {
            OutputMode::Content => "content",
            OutputMode::FilesWithMatches => "files_with_matches",
            OutputMode::Count => "count",
        }
    }
}

/// A `Grep` call's input, with Claude Code's defaults filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrepInput {
    pub pattern: String,
    pub path: Option<String>,
    pub glob: Option<String>,
    pub file_type: Option<String>,
    pub output_mode: OutputMode,
    pub before: Option<usize>,
    pub after: Option<usize>,
    pub context_c: Option<usize>,
    pub context: Option<usize>,
    pub line_numbers: bool,
    pub case_insensitive: bool,
    pub head_limit: Option<usize>,
    pub offset: usize,
    pub multiline: bool,
}

const PARAMETERS: &[&str] = &[
    "pattern",
    "path",
    "glob",
    "output_mode",
    "-B",
    "-A",
    "-C",
    "context",
    "-n",
    "-i",
    "type",
    "head_limit",
    "offset",
    "multiline",
];

impl GrepInput {
    /// Reads the model's arguments; `Err` is Claude Code's input validation message.
    pub fn parse(arguments: Value) -> Result<Self, String> {
        let mut input = Input::new(GREP, arguments, PARAMETERS);
        let pattern = input.required_string("pattern");
        let path = input.string("path");
        let glob = input.string("glob");
        let output_mode = match input.string("output_mode").as_deref() {
            None | Some("files_with_matches") => Some(OutputMode::FilesWithMatches),
            Some("content") => Some(OutputMode::Content),
            Some("count") => Some(OutputMode::Count),
            Some(_) => None,
        };
        let before = input.count("-B");
        let after = input.count("-A");
        let context_c = input.count("-C");
        let context = input.count("context");
        let line_numbers = input.boolean("-n").unwrap_or(true);
        let case_insensitive = input.boolean("-i").unwrap_or(false);
        let file_type = input.string("type");
        let head_limit = input.count("head_limit");
        let offset = input.count("offset").unwrap_or(0);
        let multiline = input.boolean("multiline").unwrap_or(false);
        let mode_issue = output_mode.is_none().then(|| {
            format!(
                "The parameter `output_mode` is expected as one of {}",
                OUTPUT_MODES
                    .iter()
                    .map(|mode| format!("\"{mode}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        });
        match (input.finish(), mode_issue) {
            (Err(message), Some(issue)) => return Err(format!("{message}\n{issue}")),
            (Err(message), None) => return Err(message),
            (Ok(()), Some(issue)) => {
                return Err(format!(
                    "InputValidationError: {GREP} failed due to the following issue:\n{issue}"
                ));
            }
            (Ok(()), None) => {}
        }
        Ok(Self {
            pattern,
            path: path.filter(|path| !path.is_empty()),
            glob: glob.filter(|glob| !glob.is_empty()),
            file_type: file_type.filter(|file_type| !file_type.is_empty()),
            output_mode: output_mode.unwrap_or(OutputMode::FilesWithMatches),
            before,
            after,
            context_c,
            context,
            line_numbers,
            case_insensitive,
            head_limit,
            offset,
            multiline,
        })
    }
}

/// The ripgrep arguments of a search, before the path searched, as Claude Code builds
/// them. Files with matches are listed newest first by ripgrep itself (`--sortr=modified`),
/// where Claude Code stats each file afterwards. Lines and counts are sorted by path, so
/// that `offset` pages through the same order on every call; counts always name their
/// file, also when the path searched is one file.
pub fn rg_args(input: &GrepInput) -> Vec<String> {
    let mut args: Vec<String> = vec!["--hidden".to_string()];
    for directory in VCS_DIRECTORIES {
        args.push("--glob".to_string());
        args.push(format!("!{directory}"));
    }
    args.extend(["--max-columns".to_string(), "500".to_string()]);
    if input.multiline {
        args.extend(["-U".to_string(), "--multiline-dotall".to_string()]);
    }
    if input.case_insensitive {
        args.push("-i".to_string());
    }
    match input.output_mode {
        OutputMode::FilesWithMatches => {
            args.extend(["-l".to_string(), "--sortr=modified".to_string()]);
        }
        OutputMode::Count => args.extend([
            "-c".to_string(),
            "--with-filename".to_string(),
            "--sort=path".to_string(),
        ]),
        OutputMode::Content => {
            args.push("--sort=path".to_string());
            if input.line_numbers {
                args.push("-n".to_string());
            }
            if let Some(context) = input.context.or(input.context_c) {
                args.extend(["-C".to_string(), context.to_string()]);
            } else {
                if let Some(before) = input.before {
                    args.extend(["-B".to_string(), before.to_string()]);
                }
                if let Some(after) = input.after {
                    args.extend(["-A".to_string(), after.to_string()]);
                }
            }
        }
    }
    if input.pattern.starts_with('-') {
        args.extend(["-e".to_string(), input.pattern.clone()]);
    } else {
        args.push(input.pattern.clone());
    }
    if let Some(file_type) = &input.file_type {
        args.extend(["--type".to_string(), file_type.clone()]);
    }
    if let Some(glob) = &input.glob {
        for pattern in split_glob(glob) {
            args.extend(["--glob".to_string(), pattern]);
        }
    }
    args
}

/// Claude Code's reading of `glob`: patterns separated by spaces, or by commas outside a
/// pattern with braces.
pub fn split_glob(glob: &str) -> Vec<String> {
    let mut patterns = Vec::new();
    for raw in glob.split_whitespace() {
        if raw.contains('{') && raw.contains('}') {
            patterns.push(raw.to_string());
        } else {
            patterns.extend(
                raw.split(',')
                    .filter(|pattern| !pattern.is_empty())
                    .map(str::to_string),
            );
        }
    }
    patterns
}

/// Claude Code's `Path does not exist` message.
pub fn path_not_found_message(path: &str, cwd: &str) -> String {
    format!("Path does not exist: {path}. Note: your current working directory is {cwd}.")
}

/// The result of a search: the text for the model and Claude Code's `tool_response`.
#[derive(Debug, Clone, PartialEq)]
pub struct GrepResult {
    pub text: String,
    pub tool_response: Value,
}

/// Formats the lines ripgrep printed as Claude Code does: `head_limit` and `offset`
/// applied, paths inside `cwd` shortened, and the summary of each output mode. The totals
/// of `count` are those of the whole search, not of the page; an `offset` past the last
/// result says so instead of reporting no matches.
pub fn format_result(input: &GrepInput, lines: Vec<String>, cwd: &str) -> GrepResult {
    let total = lines.len();
    let (matches, files) = count_totals(&lines);
    let (lines, applied_limit) = apply_head_limit(lines, input.head_limit, input.offset);
    let past_end = lines.is_empty() && total > 0;
    let lines: Vec<String> = lines.iter().map(|line| relative_to(line, cwd)).collect();
    let applied_offset = (input.offset > 0).then_some(input.offset);
    let limit_info = limit_info(applied_limit, applied_offset);

    let mut tool_response = json!({ "mode": input.output_mode.name() });
    let text = match input.output_mode {
        OutputMode::Content => {
            let content = lines.join("\n");
            tool_response["numFiles"] = json!(0);
            tool_response["filenames"] = json!([]);
            tool_response["content"] = json!(content);
            tool_response["numLines"] = json!(lines.len());
            let shown = if past_end {
                past_end_message(input.offset, total, "line")
            } else if content.is_empty() {
                "No matches found".to_string()
            } else {
                content
            };
            if limit_info.is_empty() {
                shown
            } else {
                format!("{shown}\n\n[Showing results with pagination = {limit_info}]")
            }
        }
        OutputMode::Count => {
            let content = lines.join("\n");
            tool_response["numFiles"] = json!(files);
            tool_response["filenames"] = json!([]);
            tool_response["content"] = json!(content);
            tool_response["numMatches"] = json!(matches);
            let shown = if past_end {
                past_end_message(input.offset, total, "entry")
            } else if content.is_empty() {
                "No matches found".to_string()
            } else {
                content
            };
            let pagination = if limit_info.is_empty() {
                String::new()
            } else {
                format!(" with pagination = {limit_info}")
            };
            format!(
                "{shown}\n\nFound {matches} total {} across {files} {}.{pagination}",
                if matches == 1 {
                    "occurrence"
                } else {
                    "occurrences"
                },
                if files == 1 { "file" } else { "files" },
            )
        }
        OutputMode::FilesWithMatches => {
            tool_response["numFiles"] = json!(lines.len());
            tool_response["filenames"] = json!(lines);
            if past_end {
                past_end_message(input.offset, total, "file")
            } else if lines.is_empty() {
                "No files found".to_string()
            } else {
                let limit = if limit_info.is_empty() {
                    String::new()
                } else {
                    format!(" {limit_info}")
                };
                format!(
                    "Found {} {}{limit}\n{}",
                    lines.len(),
                    if lines.len() == 1 { "file" } else { "files" },
                    lines.join("\n")
                )
            }
        }
    };
    if let Some(limit) = applied_limit {
        tool_response["appliedLimit"] = json!(limit);
    }
    if let Some(offset) = applied_offset {
        tool_response["appliedOffset"] = json!(offset);
    }
    GrepResult {
        text,
        tool_response,
    }
}

/// The matches and files of `count` lines (`path:N`).
fn count_totals(lines: &[String]) -> (u64, u64) {
    let mut matches = 0u64;
    let mut files = 0u64;
    for line in lines {
        if let Some((_, count)) = line.rsplit_once(':')
            && let Ok(count) = count.parse::<u64>()
        {
            matches += count;
            files += 1;
        }
    }
    (matches, files)
}

/// The text for an `offset` at or past the last of `total` results.
fn past_end_message(offset: usize, total: usize, noun: &str) -> String {
    let nouns = match (noun, total) {
        (_, 1) => noun.to_string(),
        ("entry", _) => "entries".to_string(),
        _ => format!("{noun}s"),
    };
    format!("No results at offset {offset}: the search found {total} {nouns} in all")
}

fn limit_info(applied_limit: Option<usize>, applied_offset: Option<usize>) -> String {
    let mut parts = Vec::new();
    if let Some(limit) = applied_limit {
        parts.push(format!("limit: {limit}"));
    }
    if let Some(offset) = applied_offset {
        parts.push(format!("offset: {offset}"));
    }
    parts.join(", ")
}

#[cfg(test)]
#[path = "grep_tests.rs"]
mod tests;
