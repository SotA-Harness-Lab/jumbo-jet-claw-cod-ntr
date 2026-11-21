// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Running ripgrep for `Glob` and `Grep`. The search is a command run through Codex's
//! `exec_command` runtime, as `Bash` runs commands: the turn's sandbox, approval policy,
//! execution policy and command events apply to it. Claude Code runs its own ripgrep; here
//! `rg` is the one installed in the session's environment.

use codex_claude_compat::tools::ripgrep;
use codex_claude_compat::tools::ripgrep::RgRun;
use codex_claude_compat::tools::ripgrep::Shell;
use serde_json::Value;
use serde_json::json;

use super::delegated;
use super::failed;
use super::scratch_file;
use crate::function_tool::FunctionCallError;
use crate::session::turn_context::TurnEnvironment;
use crate::shell::ShellType;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::registry::ToolExecutor;

pub(super) struct Ripgrep {
    /// Runs a command to completion or its timeout. Its options only shape the spec of
    /// `exec_command`, which the model never sees here.
    exec: ExecCommandHandler,
}

impl Ripgrep {
    pub(super) fn new() -> Self {
        Self {
            exec: ExecCommandHandler::one_shot(ExecCommandHandlerOptions::default()),
        }
    }

    /// Runs `rg` with `args` on `target` (a native path in `environment`), in `workdir`
    /// when one is given, and returns the lines it printed, or the error the model is told.
    /// lines it printed and whether the search stopped at its timeout (the lines are then
    /// partial), or the error the model is told.
    pub(super) async fn run(
        &self,
        invocation: &ToolInvocation,
        environment: &TurnEnvironment,
        args: &[String],
        target: &str,
        workdir: Option<&str>,
    ) -> Result<(Vec<String>, bool), FunctionCallError> {
        // The shell `exec_command` runs the command line with.
        let shell_type = environment
            .shell
            .as_ref()
            .map(|shell| shell.shell_type)
            .unwrap_or_else(|| invocation.session.user_shell().shell_type);
        let shell = match shell_type {
            ShellType::PowerShell => Shell::PowerShell,
            ShellType::Cmd => Shell::Cmd,
            ShellType::Zsh | ShellType::Bash | ShellType::Sh => Shell::Posix,
        };
        let result = self
            .run_command(
                invocation,
                ripgrep::command_line(args, target, shell),
                workdir,
            )
            .await?;
        let output = result
            .get("output")
            .and_then(Value::as_str)
            .unwrap_or_default();
        // Past 1 MiB the output loses its middle, so that counts and later pages would skip
        // results: the search runs again with its results written to a file.
        if shell == Shell::Posix
            && ripgrep::output_was_cut(output)
            && let Some(lines) = self
                .run_to_file(invocation, environment, args, target, workdir)
                .await?
        {
            return Ok(lines);
        }
        let (exit_code, timed_out) = exit_status(&result);
        ripgrep::result_lines(&RgRun {
            output,
            results: None,
            truncated: false,
            exit_code,
            timed_out,
            target,
        })
        .map(|lines| (lines, timed_out))
        .map_err(failed)
    }

    /// [`Self::run`] with the results written to a file of the tool's own and read back
    /// from it, up to [`ripgrep::MAX_OUTPUT_BYTES`]; `Ok(None)` when the file cannot be
    /// created, written or read.
    async fn run_to_file(
        &self,
        invocation: &ToolInvocation,
        environment: &TurnEnvironment,
        args: &[String],
        target: &str,
        workdir: Option<&str>,
    ) -> Result<Option<(Vec<String>, bool)>, FunctionCallError> {
        let key = ripgrep::command_line(args, target, Shell::Posix);
        let Some(file) =
            scratch_file::create(environment, /*posix*/ true, "ccn-rg", key.as_bytes()).await
        else {
            return Ok(None);
        };
        let result = self
            .run_command(
                invocation,
                ripgrep::command_line_to_file(args, target, &file.inferred_native_path_string()),
                workdir,
            )
            .await;
        let contents = scratch_file::take(environment, &file).await;
        let result = result?;
        let Ok(mut contents) = contents else {
            return Ok(None);
        };
        let truncated = contents.len() >= ripgrep::MAX_OUTPUT_BYTES;
        contents.truncate(ripgrep::MAX_OUTPUT_BYTES);
        let results = String::from_utf8_lossy(&contents);
        let (exit_code, timed_out) = exit_status(&result);
        ripgrep::result_lines(&RgRun {
            output: result
                .get("output")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            results: Some(&*results),
            truncated,
            exit_code,
            timed_out,
            target,
        })
        .map(|lines| Some((lines, timed_out)))
        .map_err(failed)
    }

    /// Runs `cmd` to completion or its timeout, in `workdir` when one is given, and returns
    /// what `exec_command` reports of it.
    async fn run_command(
        &self,
        invocation: &ToolInvocation,
        cmd: String,
        workdir: Option<&str>,
    ) -> Result<Value, FunctionCallError> {
        // No `max_output_tokens`: the whole output comes back, and the Claude tool limits
        // it as Claude Code does.
        let mut arguments = json!({
            "cmd": cmd,
            "timeout_ms": ripgrep::TIMEOUT_MS,
        });
        if let Some(workdir) = workdir {
            arguments["workdir"] = json!(workdir);
        }
        let payload = ToolPayload::Function {
            arguments: arguments.to_string(),
        };
        let output = self
            .exec
            .handle(delegated(invocation, payload.clone()))
            .await?;
        Ok(output.code_mode_result(&payload))
    }
}

/// The exit code of a finished command, and whether it was stopped at its timeout.
fn exit_status(result: &Value) -> (Option<i32>, bool) {
    let exit_code = result
        .get("exit_code")
        .and_then(Value::as_i64)
        .and_then(|code| i32::try_from(code).ok());
    // A one-shot run stopped at its timeout has no exit code.
    let timed_out = exit_code.is_none() && result.get("chunk_id").is_some();
    (exit_code, timed_out)
}
