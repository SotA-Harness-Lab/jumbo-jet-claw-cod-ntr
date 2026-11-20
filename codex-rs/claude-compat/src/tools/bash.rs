// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Bash` tool: description, parameters and result text.

use super::AGENT;
use super::BASH;
use super::EDIT;
use super::GLOB;
use super::GREP;
use super::READ;
use super::TODO_WRITE;
use super::WRITE;

/// Timeout of a command that does not give one, in milliseconds (Claude Code's default).
pub const DEFAULT_TIMEOUT_MS: u64 = 120_000;
/// Longest timeout a command may ask for, in milliseconds (Claude Code's default).
pub const MAX_TIMEOUT_MS: u64 = 600_000;

pub const COMMAND_PARAM_DESCRIPTION: &str = "The command to execute";

pub const DESCRIPTION_PARAM_DESCRIPTION: &str = r#"Clear, concise description of what this command does in active voice. Never use words like "complex" or "risk" in the description - just describe what it does.

For simple commands (git, npm, standard CLI tools), keep it brief (5-10 words):
- ls → "List files in current directory"
- git status → "Show working tree status"
- npm install → "Install package dependencies"

For commands that are harder to parse at a glance (piped commands, obscure flags, etc.), add enough context to clarify what it does:
- find . -name "*.tmp" -exec rm {} \; → "Find and delete all .tmp files recursively"
- git reset --hard origin/main → "Discard all local changes and match remote main"
- curl -s url | jq '.data[]' → "Fetch JSON from URL and extract data array elements""#;

pub const RUN_IN_BACKGROUND_PARAM_DESCRIPTION: &str = "Set to true to run this command in the background. Use write_stdin with the returned ID to read the output later.";

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
        preferences.push(format!("File search: Use {GLOB} (NOT find or ls)"));
    }
    if has(GREP) {
        preferences.push(format!("Content search: Use {GREP} (NOT grep or rg)"));
    }
    if has(READ) {
        preferences.push(format!("Read files: Use {READ} (NOT cat/head/tail)"));
    }
    if has(EDIT) {
        preferences.push(format!("Edit files: Use {EDIT} (NOT sed/awk)"));
    }
    if has(WRITE) {
        preferences.push(format!("Write files: Use {WRITE} (NOT echo >/cat <<EOF)"));
    }
    preferences.push("Communication: Output text directly (NOT echo/printf)".to_string());
    let avoid = if has(GLOB) && has(GREP) {
        "`find`, `grep`, `cat`, `head`, `tail`, `sed`, `awk`, or `echo`"
    } else {
        "`cat`, `head`, `tail`, `sed`, `awk`, or `echo`"
    };

    let mut text = String::new();
    text.push_str("Executes a given bash command and returns its output.\n\n");
    text.push_str(
        "The working directory persists between commands, but shell state does not. The shell environment is initialized from the user's profile (bash or zsh).\n\n",
    );
    text.push_str(&format!(
        "IMPORTANT: Avoid using this tool to run {avoid} commands, unless explicitly instructed or after you have verified that a dedicated tool cannot accomplish your task. Instead, use the appropriate dedicated tool as this will provide a much better experience for the user:\n\n"
    ));
    for preference in &preferences {
        text.push_str(&format!(" - {preference}\n"));
    }
    text.push_str(&format!(
        "While the {BASH} tool can do similar things, it’s better to use the built-in tools as they provide a better user experience and make it easier to review tool calls and give permission.\n\n"
    ));
    text.push_str("# Instructions\n");
    let mut instructions = vec![
        "If your command will create new directories or files, first use this tool to run `ls` to verify the parent directory exists and is the correct location.".to_string(),
        "Always quote file paths that contain spaces with double quotes in your command (e.g., cd \"path with spaces/file.txt\")".to_string(),
        "Try to maintain your current working directory throughout the session by using absolute paths and avoiding usage of `cd`. You may use `cd` if the User explicitly requests it.".to_string(),
        format!(
            "You may specify an optional timeout in milliseconds (up to {MAX_TIMEOUT_MS}ms / {} minutes). By default, your command will timeout after {DEFAULT_TIMEOUT_MS}ms ({} minutes).",
            MAX_TIMEOUT_MS / 60_000,
            DEFAULT_TIMEOUT_MS / 60_000
        ),
    ];
    if background {
        instructions.push(
            "You can use the `run_in_background` parameter to run the command in the background. Only use this if you don't need the result immediately. The result gives the command's ID: read its output later with write_stdin, passing that ID as session_id and empty chars. You do not need to use '&' at the end of the command when using this parameter.".to_string(),
        );
    }
    for instruction in instructions {
        text.push_str(&format!(" - {instruction}\n"));
    }
    text.push_str(" - When issuing multiple commands:\n");
    for item in [
        format!(
            "If the commands are independent and can run in parallel, make multiple {BASH} tool calls in a single message. Example: if you need to run \"git status\" and \"git diff\", send a single message with two {BASH} tool calls in parallel."
        ),
        format!(
            "If the commands depend on each other and must run sequentially, use a single {BASH} call with '&&' to chain them together."
        ),
        "Use ';' only when you need to run commands sequentially but don't care if earlier commands fail.".to_string(),
        "DO NOT use newlines to separate commands (newlines are ok in quoted strings).".to_string(),
    ] {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(" - For git commands:\n");
    for item in [
        "Prefer to create a new commit rather than amending an existing commit.",
        "Before running destructive operations (e.g., git reset --hard, git push --force, git checkout --), consider whether there is a safer alternative that achieves the same goal. Only use destructive operations when they are truly the best approach.",
        "Never skip hooks (--no-verify) or bypass signing (--no-gpg-sign, -c commit.gpgsign=false) unless the user has explicitly asked for it. If a hook fails, investigate and fix the underlying issue.",
    ] {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(" - Avoid unnecessary `sleep` commands:\n");
    let mut sleep_items =
        vec!["Do not sleep between commands that can run immediately — just run them."];
    if background {
        sleep_items.push("For long-running commands, use `run_in_background`.");
    }
    sleep_items.extend([
        "Do not retry failing commands in a sleep loop — diagnose the root cause.",
        "If you must sleep, keep the duration short (1-5 seconds) to avoid blocking the user.",
    ]);
    for item in sleep_items {
        text.push_str(&format!("   - {item}\n"));
    }
    text.push_str(
        "\n## Command sandbox\nBy default, your command runs in Codex's sandbox. If a command fails and you see evidence of sandbox restrictions (\"Operation not permitted\", access denied to paths outside the writable directories, network failures), retry it with `dangerouslyDisableSandbox: true`; the user is asked to approve it. Treat each such command individually.\n",
    );
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
        r#"# Committing changes with git

Only create commits when requested by the user. If unclear, ask first. When the user asks you to create a new git commit, follow these steps carefully:

You can call multiple tools in a single response. When multiple independent pieces of information are requested and all commands are likely to succeed, run multiple tool calls in parallel for optimal performance. The numbered steps below indicate which commands should be batched in parallel.

Git Safety Protocol:
- NEVER update the git config
- NEVER run destructive git commands (push --force, reset --hard, checkout ., restore ., clean -f, branch -D) unless the user explicitly requests these actions. Taking unauthorized destructive actions is unhelpful and can result in lost work, so it's best to ONLY run these commands when given direct instructions
- NEVER skip hooks (--no-verify, --no-gpg-sign, etc) unless the user explicitly requests it
- NEVER run force push to main/master, warn the user if they request it
- CRITICAL: Always create NEW commits rather than amending, unless the user explicitly requests a git amend. When a pre-commit hook fails, the commit did NOT happen — so --amend would modify the PREVIOUS commit, which may result in destroying work or losing previous changes. Instead, after hook failure, fix the issue, re-stage, and create a NEW commit
- When staging files, prefer adding specific files by name rather than using "git add -A" or "git add .", which can accidentally include sensitive files (.env, credentials) or large binaries
- NEVER commit changes unless the user explicitly asks you to. It is VERY IMPORTANT to only commit when explicitly asked, otherwise the user will feel that you are being too proactive

1. Run the following bash commands in parallel, each using the {BASH} tool:
  - Run a git status command to see all untracked files. IMPORTANT: Never use the -uall flag as it can cause memory issues on large repos.
  - Run a git diff command to see both staged and unstaged changes that will be committed.
  - Run a git log command to see recent commit messages, so that you can follow this repository's commit message style.
2. Analyze all staged changes (both previously staged and newly added) and draft a commit message:
  - Summarize the nature of the changes (eg. new feature, enhancement to an existing feature, bug fix, refactoring, test, docs, etc.). Ensure the message accurately reflects the changes and their purpose (i.e. "add" means a wholly new feature, "update" means an enhancement to an existing feature, "fix" means a bug fix, etc.).
  - Do not commit files that likely contain secrets (.env, credentials.json, etc). Warn the user if they specifically request to commit those files
  - Draft a concise (1-2 sentences) commit message that focuses on the "why" rather than the "what"
  - Ensure it accurately reflects the changes and their purpose
3. Run the following commands in parallel:
   - Add relevant untracked files to the staging area.
   - Create the commit with a message.
   - Run git status after the commit completes to verify success.
   Note: git status depends on the commit completing, so run it sequentially after the commit.
4. If the commit fails due to pre-commit hook: fix the issue and create a NEW commit

Important notes:
- NEVER run additional commands to read or explore code, besides git bash commands
{never_use}- DO NOT push to the remote repository unless the user explicitly asks you to do so
- IMPORTANT: Never use git commands with the -i flag (like git rebase -i or git add -i) since they require interactive input which is not supported.
- IMPORTANT: Do not use --no-edit with git rebase commands, as the --no-edit flag is not a valid option for git rebase.
- If there are no changes to commit (i.e., no untracked files and no modifications), do not create an empty commit
- In order to ensure good formatting, ALWAYS pass the commit message via a HEREDOC, a la this example:
<example>
git commit -m "$(cat <<'EOF'
   Commit message here.
   EOF
   )"
</example>

# Creating pull requests
Use the gh command via the Bash tool for ALL GitHub-related tasks including working with issues, pull requests, checks, and releases. If given a Github URL use the gh command to get the information needed.

IMPORTANT: When the user asks you to create a pull request, follow these steps carefully:

1. Run the following bash commands in parallel using the {BASH} tool, in order to understand the current state of the branch since it diverged from the main branch:
   - Run a git status command to see all untracked files (never use -uall flag)
   - Run a git diff command to see both staged and unstaged changes that will be committed
   - Check if the current branch tracks a remote branch and is up to date with the remote, so you know if you need to push to the remote
   - Run a git log command and `git diff [base-branch]...HEAD` to understand the full commit history for the current branch (from the time it diverged from the base branch)
2. Analyze all changes that will be included in the pull request, making sure to look at all relevant commits (NOT just the latest commit, but ALL commits that will be included in the pull request!!!), and draft a pull request title and summary:
   - Keep the PR title short (under 70 characters)
   - Use the description/body for details, not the title
3. Run the following commands in parallel:
   - Create new branch if needed
   - Push to remote with -u flag if needed
   - Create PR using gh pr create with the format below. Use a HEREDOC to pass the body to ensure correct formatting.
<example>
gh pr create --title "the pr title" --body "$(cat <<'EOF'
## Summary
<1-3 bullet points>

## Test plan
[Bulleted markdown checklist of TODOs for testing the pull request...]
EOF
)"
</example>

Important:
{do_not_use}- Return the PR URL when you're done, so the user can see it

# Other common operations
- View comments on a Github PR: gh api repos/foo/bar/pulls/123/comments"#
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
        "Command running in background with ID: {id}. Read its output with write_stdin, passing session_id {id} and empty chars."
    )
}

/// The result of a command that ran into its timeout and was left running in the background,
/// with the output it had written until then.
pub fn moved_to_background_message(id: &str, timeout_ms: u64, output: &str) -> String {
    let output = strip_empty_lines(output);
    let mut text = format!(
        "Command did not complete within its {} timeout and was moved to the background with ID: {id}. It is still running. Read its further output with write_stdin, passing session_id {id} and empty chars.",
        format_duration(timeout_ms)
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
