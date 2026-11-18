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

    /// Runs `rg` with `args` on `target` (a native path in `environment`) and returns the
    /// lines it printed, or the error the model is told.
    pub(super) async fn run(
        &self,
        invocation: &ToolInvocation,
        environment: &TurnEnvironment,
        args: &[String],
        target: &str,
    ) -> Result<Vec<String>, FunctionCallError> {
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
        // No `max_output_tokens`: the whole output comes back, and the Claude tool limits
        // it as Claude Code does.
        let payload = ToolPayload::Function {
            arguments: json!({
                "cmd": ripgrep::command_line(args, target, shell),
                "timeout_ms": ripgrep::TIMEOUT_MS,
            })
            .to_string(),
        };
        let output = self
            .exec
            .handle(delegated(invocation, payload.clone()))
            .await?;
        let result = output.code_mode_result(&payload);
        let exit_code = result
            .get("exit_code")
            .and_then(Value::as_i64)
            .and_then(|code| i32::try_from(code).ok());
        // A one-shot run stopped at its timeout has no exit code.
        let timed_out = exit_code.is_none() && result.get("chunk_id").is_some();
        ripgrep::result_lines(&RgRun {
            output: result
                .get("output")
                .and_then(Value::as_str)
                .unwrap_or_default(),
            exit_code,
            timed_out,
            target,
        })
        .map_err(failed)
    }
}
