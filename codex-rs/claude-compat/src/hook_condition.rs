// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The `if` field of a Claude Code hook handler.
//!
//! A handler with `if` runs only for tool calls its condition covers. The condition is one
//! permission rule, `Tool` or `Tool(content)`, read as Claude Code's `prepareIfConditionMatcher`
//! reads it: the tool has to be the rule's, and the content is matched against the one input
//! the tool is known by (the command, the file path, the search pattern).

use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::permissions::bash::ShellRule;
use crate::permissions::bash::deny_forms;
use crate::permissions::bash::matches_wildcard;
use crate::permissions::bash::parse_shell_rule;
use crate::permissions::bash::split_command;
use crate::permissions::file_pattern_matches;
use crate::permissions::normalize_tool_name;
use crate::permissions::parse_rule;

/// Whether a hook whose `if` is `condition` runs for a call of the tool known by
/// `tool_names` (its own name and the Claude Code names it is also matched by) with
/// `tool_input`, in the working directory `cwd`. A file path is matched as a `Read(...)`
/// or `Edit(...)` permission rule matches it (`.env`, `src/**`, `~/.ssh/**`, `//etc/**`).
///
/// Where the input lacks what the content is matched against, the hook runs: it then sees the
/// call and decides for itself, which is safer than a guard that silently never fires.
pub fn hook_condition_matches(
    condition: &str,
    tool_names: &[&str],
    tool_input: &Value,
    cwd: &Path,
) -> bool {
    let (tool, content) = parse_rule(condition.trim());
    if !tool_names
        .iter()
        .any(|name| normalize_tool_name(name) == tool)
    {
        return false;
    }
    let Some(content) = content else {
        return true;
    };
    let field = match tool.as_str() {
        "Bash" => "command",
        "Read" | "Edit" | "Write" => "file_path",
        "Glob" | "Grep" => "pattern",
        // Claude Code matches content for these tools only.
        _ => return false,
    };
    let Some(value) = tool_input.get(field).and_then(Value::as_str) else {
        return true;
    };
    if tool == "Bash" {
        command_matches(&content, value)
    } else if field == "file_path" {
        let home_dir = std::env::var_os("HOME").map(PathBuf::from);
        file_pattern_matches(&content, Path::new(value), cwd, home_dir.as_deref())
    } else {
        matches_wildcard(&content, value)
    }
}

/// A command meets the content when one of its simple commands does, so that
/// `ls && git push` and `echo "$(git push)"` still run a hook for `Bash(git *)`, while
/// `echo $(date)` does not: the commands nested in substitutions, subshells and control flow
/// are matched like the others. A command that cannot be taken apart (an unclosed quote) runs
/// the hook.
fn command_matches(content: &str, command: &str) -> bool {
    let Some(split) = split_command(command) else {
        return true;
    };
    let rule = parse_shell_rule(content);
    // Each simple command in the forms deny rules meet: `sudo rm x` and `"rm" x` still run a
    // hook for `Bash(rm *)`.
    split
        .subcommands
        .iter()
        .flat_map(|subcommand| deny_forms(subcommand))
        .any(|form| match rule {
            ShellRule::Prefix(prefix) => {
                form == prefix
                    || form
                        .strip_prefix(prefix)
                        .is_some_and(|rest| rest.starts_with(' '))
            }
            ShellRule::Exact(pattern) | ShellRule::Wildcard(pattern) => {
                matches_wildcard(pattern, &form)
            }
        })
}

#[cfg(test)]
#[path = "hook_condition_tests.rs"]
mod tests;
