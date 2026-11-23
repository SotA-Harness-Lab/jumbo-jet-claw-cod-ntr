// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Bash` tool: description, parameters and result text.

use serde_json::Value;

use super::AGENT;
use super::BASH;
use super::EDIT;
use super::GLOB;
use super::GREP;
use super::READ;
use super::TODO_WRITE;
use super::WRITE;
use super::input::Input;

/// Timeout of a command that does not give one, in milliseconds (Claude Code's default).
pub const DEFAULT_TIMEOUT_MS: u64 = 120_000;
/// Longest timeout a command may ask for, in milliseconds (Claude Code's default).
pub const MAX_TIMEOUT_MS: u64 = 600_000;

pub const COMMAND_PARAM_DESCRIPTION: &str = "The command to execute";

/// A `Bash` call's input.
#[derive(Debug, Clone, PartialEq)]
pub struct BashInput {
    pub command: String,
    /// Milliseconds.
    pub timeout: Option<f64>,
    pub description: Option<String>,
    pub run_in_background: Option<bool>,
    pub dangerously_disable_sandbox: Option<bool>,
}

impl BashInput {
    /// Reads the model's arguments: numbers and booleans may come as strings (`"60000"`,
    /// `"true"`), and `command_description` stands for `description`; `Err` is Claude
    /// Code's input validation message.
    pub fn parse(arguments: Value) -> Result<Self, String> {
        let mut input =
            Input::ignoring_unknown(BASH, arguments, &[("command_description", "description")]);
        let command = input.required_string("command");
        let timeout = input.number("timeout");
        let description = input.string("description");
        let run_in_background = input.boolean("run_in_background");
        let dangerously_disable_sandbox = input.boolean("dangerouslyDisableSandbox");
        input.finish()?;
        Ok(Self {
            command,
            timeout,
            description,
            run_in_background,
            dangerously_disable_sandbox,
        })
    }
}

pub const DESCRIPTION_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/bash/description_param_description.md");

pub const RUN_IN_BACKGROUND_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/bash/run_in_background_param_description.md");

pub const DISABLE_SANDBOX_PARAM_DESCRIPTION: &str =
    "Set this to true to dangerously override sandbox mode and run commands without sandboxing.";

/// Description of the `timeout` parameter.
pub fn timeout_param_description() -> String {
    format!("Optional timeout in milliseconds (max {MAX_TIMEOUT_MS})")
}

/// The timeout a call runs with: Claude Code takes a missing or zero timeout as the default
/// and caps it at the maximum.
pub fn timeout_ms(requested: Option<f64>) -> u64 {
    match requested {
        Some(timeout) if timeout.is_finite() && timeout >= 1.0 => {
            (timeout as u64).min(MAX_TIMEOUT_MS)
        }
        _ => DEFAULT_TIMEOUT_MS,
    }
}

/// The tool description, after Claude Code's. `available` lists the Claude Code tools the
/// model is offered, so that the description only points to tools that exist;
/// `background` says whether `run_in_background` is offered.
///
/// Where Codex differs from Claude Code the text says what Codex does: the sandbox is
/// Codex's, and a background command is read with `write_stdin`. The commit and pull request instructions carry no attribution
/// lines, as with Claude Code's `attribution` setting left empty.
pub fn description(available: &[&str], background: bool) -> String {
    let has = |tool: &str| available.contains(&tool);
    let mut preferences = Vec::new();
    if has(GLOB) {
        preferences.push(format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/file_search_use_glob_not.md"
            ),
            GLOB = GLOB
        ));
    }
    if has(GREP) {
        preferences.push(format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/content_search_use_grep_not.md"
            ),
            GREP = GREP
        ));
    }
    if has(READ) {
        preferences.push(format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/read_files_use_read_not.md"
            ),
            READ = READ
        ));
    }
    if has(EDIT) {
        preferences.push(format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/edit_files_use_edit_not.md"
            ),
            EDIT = EDIT
        ));
    }
    if has(WRITE) {
        preferences.push(format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/write_files_use_write_not.md"
            ),
            WRITE = WRITE
        ));
    }
    preferences.push(
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/communication_output_text_directly_not.md"
        )
        .to_string(),
    );
    let avoid = if has(GLOB) && has(GREP) {
        codex_prompt_texts::text!("claude-compat/tools/bash/description/find_grep_cat_head_tail.md")
    } else {
        codex_prompt_texts::text!("claude-compat/tools/bash/description/cat_head_tail_sed_awk.md")
    };

    let mut text = String::new();
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/executes_a_given_bash_command.md"
    ));
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/the_working_directory_carries_over.md"
    ));
    text.push_str(&format!(
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/important_avoid_using_this_tool.md"
        ),
        avoid = avoid
    ));
    for preference in &preferences {
        text.push_str(&format!(" - {preference}\n"));
    }
    text.push_str(&format!(
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/while_the_bash_tool_can.md"
        ),
        BASH = BASH
    ));
    text.push_str("# Instructions\n");
    let mut instructions = vec![
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/if_your_command_will_create.md"
        )
        .to_string(),
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/always_quote_file_paths_that.md"
        )
        .to_string(),
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/try_to_maintain_your_current.md"
        )
        .to_string(),
        format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/you_may_specify_an_optional.md"
            ),
            MAX_TIMEOUT_MS / 60_000,
            DEFAULT_TIMEOUT_MS / 60_000,
            MAX_TIMEOUT_MS = MAX_TIMEOUT_MS,
            DEFAULT_TIMEOUT_MS = DEFAULT_TIMEOUT_MS
        ),
    ];
    if background {
        instructions.push(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/you_can_use_the_run.md"
            )
            .to_string(),
        );
    }
    for instruction in instructions {
        text.push_str(&format!(" - {instruction}\n"));
    }
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/when_issuing_multiple_commands.md"
    ));
    for item in [
        format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/if_the_commands_are_independent.md"
            ),
            BASH = BASH
        ),
        format!(
            codex_prompt_texts::text!(
                "claude-compat/tools/bash/description/if_the_commands_depend_on.md"
            ),
            BASH = BASH
        ),
        codex_prompt_texts::text!("claude-compat/tools/bash/description/use_only_when_you_need.md")
            .to_string(),
        codex_prompt_texts::text!("claude-compat/tools/bash/description/do_not_use_newlines_to.md")
            .to_string(),
    ] {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(" - For git commands:\n");
    for item in [
        codex_prompt_texts::text!("claude-compat/tools/bash/description/prefer_to_create_a_new.md"),
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/before_running_destructive_operations_e.md"
        ),
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/never_skip_hooks_no_verify.md"
        ),
    ] {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/avoid_unnecessary_sleep_commands.md"
    ));
    let mut sleep_items = vec![codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/do_not_sleep_between_commands.md"
    )];
    if background {
        sleep_items.push(codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/for_long_running_commands_use.md"
        ));
    }
    sleep_items.extend([
        codex_prompt_texts::text!(
            "claude-compat/tools/bash/description/do_not_retry_failing_commands.md"
        ),
        codex_prompt_texts::text!("claude-compat/tools/bash/description/if_you_must_sleep_keep.md"),
    ]);
    for item in sleep_items {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(codex_prompt_texts::text!(
        "claude-compat/tools/bash/description/command_sandbox_by_default_your.md"
    ));
    text.push('\n');
    text.push_str(&commit_and_pr_instructions(has(TODO_WRITE), has(AGENT)));
    text
}

fn commit_and_pr_instructions(todo_write: bool, agent: bool) -> String {
    let forbidden_tools = match (todo_write, agent) {
        (true, true) => Some(format!("the {TODO_WRITE} or {AGENT} tools")),
        (true, false) => Some(format!("the {TODO_WRITE} tool")),
        (false, true) => Some(format!("the {AGENT} tool")),
        (false, false) => None,
    };
    let never_use = forbidden_tools
        .as_ref()
        .map(|tools| format!("- NEVER use {tools}\n"))
        .unwrap_or_default();
    let do_not_use = forbidden_tools
        .as_ref()
        .map(|tools| format!("- DO NOT use {tools}\n"))
        .unwrap_or_default();
    format!(
        codex_prompt_texts::text!("claude-compat/tools/bash/commit_and_pr_instructions.md"),
        BASH = BASH,
        never_use = never_use,
        do_not_use = do_not_use
    )
}

/// What a finished command produced, as Codex's shell tool reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandRun<'a> {
    /// Output of the command, stdout and stderr together, as Codex truncated it.
    pub output: &'a str,
    /// Exit code; `None` when the command did not exit by itself.
    pub exit_code: Option<i32>,
    /// Whether the command was stopped at its timeout.
    pub timed_out: bool,
    /// The timeout the command ran with.
    pub timeout_ms: u64,
}

/// The text the model receives, and whether Claude Code reports it as an error: a command
/// that exits with a non-zero code is one (`Exit code N` and the output), as is one stopped
/// at its timeout.
pub fn format_result(run: &CommandRun<'_>) -> (String, bool) {
    let output = strip_empty_lines(run.output);
    if run.timed_out {
        let mut text = format!(
            "Command timed out after {}",
            format_duration(run.timeout_ms)
        );
        if !output.is_empty() {
            text.push('\n');
            text.push_str(&output);
        }
        return (text, true);
    }
    match run.exit_code {
        Some(code) if code != 0 => {
            let mut text = format!("Exit code {code}");
            if !output.is_empty() {
                text.push('\n');
                text.push_str(&output);
            }
            (text, true)
        }
        _ if output.is_empty() => (format!("({BASH} completed with no output)"), false),
        _ => (output, false),
    }
}

/// The result of a command started with `run_in_background`.
pub fn background_message(id: &str) -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/bash/background_message.md"),
        id = id
    )
}

/// The result of a command that ran into its timeout and was left running in the background,
/// with the output it had written until then.
pub fn moved_to_background_message(id: &str, timeout_ms: u64, output: &str) -> String {
    let output = strip_empty_lines(output);
    let mut text = format!(
        codex_prompt_texts::text!("claude-compat/tools/bash/moved_to_background_message.md"),
        format_duration(timeout_ms),
        id = id
    );
    if output.is_empty() {
        text.push_str(" It has written no output so far.");
    } else {
        text.push_str("\nOutput so far:\n");
        text.push_str(&output);
    }
    text
}

/// Whether a command may be left in the background at its timeout. Not `sleep`: waiting is
/// all it does, so there is nothing to go on with while it runs.
pub fn backgrounds_on_timeout(command: &str) -> bool {
    command.split_whitespace().next() != Some("sleep")
}

/// Whether `command` changes its directory (`cd`, `pushd`, `popd`, `chdir`), which does not
/// carry over when it runs in the background.
pub fn changes_directory(command: &str) -> bool {
    command
        .split(|c: char| c.is_whitespace() || ";&|(){}`".contains(c))
        .any(|word| matches!(word, "cd" | "pushd" | "popd" | "chdir"))
}

/// Claude Code's note after the result of a background command that changes its directory:
/// the next command still starts in `cwd`.
pub fn background_cwd_note(cwd: &str) -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/bash/background_cwd_note.md"),
        cwd = cwd
    )
}

/// Lines added after a command, in the shell that runs it, to carry its working directory
/// over to the next one: after a command that succeeded, its physical working directory
/// (`pwd -P`, as Claude Code records it) is written to `file`. The command's exit status
/// stays the call's.
pub fn cwd_trailer(file: &str) -> String {
    format!(
        "\n__ccn_status=$?\nif [ \"$__ccn_status\" -eq 0 ]; then pwd -P >| {} 2>/dev/null; fi\nexit \"$__ccn_status\"",
        shell_quote(file)
    )
}

/// `text` as one single-quoted word of a POSIX shell.
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// Put before a command run by zsh: `EQUALS` off, so that a word starting with `=`
/// (`[ a == b ]`, `echo ====`) is not expanded to the path of a command and does not stop
/// the command with `= not found`, as in Claude Code. On the same line as the command, so
/// that the line numbers of its errors stay the same.
pub const ZSH_PRELUDE: &str = "setopt NO_EQUALS; ";

/// The working directory recorded by [`cwd_trailer`]: the file's text without its line end.
pub fn recorded_cwd(contents: &str) -> Option<&str> {
    let cwd = contents.trim_end_matches(['\n', '\r']);
    (!cwd.is_empty()).then_some(cwd)
}

/// What Claude Code adds to a command's result when the command left the project: the next
/// command starts in the project again.
pub fn shell_reset_message(project_dir: &str) -> String {
    format!("Shell cwd was reset to {project_dir}")
}

/// Drops the blank lines before the first and after the last line with text, and the
/// trailing whitespace, as Claude Code does with a command's output.
pub fn strip_empty_lines(output: &str) -> String {
    let lines: Vec<&str> = output.split('\n').collect();
    let Some(start) = lines.iter().position(|line| !line.trim().is_empty()) else {
        return String::new();
    };
    let end = lines
        .iter()
        .rposition(|line| !line.trim().is_empty())
        .unwrap_or(start);
    lines[start..=end].join("\n").trim_end().to_string()
}

/// Claude Code's short rendering of a duration (`2m 0s`, `30s`).
pub fn format_duration(ms: u64) -> String {
    let seconds = ms / 1000;
    if seconds < 60 {
        format!("{seconds}s")
    } else {
        format!("{}m {}s", seconds / 60, seconds % 60)
    }
}

#[cfg(test)]
#[path = "bash_tests.rs"]
mod tests;
