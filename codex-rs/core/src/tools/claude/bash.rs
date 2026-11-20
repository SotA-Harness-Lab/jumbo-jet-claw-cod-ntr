// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Bash`: Claude Code's shell tool.
//!
//! Delegated to Codex's `exec_command` runtime: the shell, execution policy and command
//! rules, approval, sandbox (`dangerouslyDisableSandbox` asks for an unsandboxed run as
//! `sandbox_permissions: "require_escalated"` does), the timeout (a one-shot run),
//! background processes (an interactive run, read and stopped with `write_stdin`), output
//! truncation, events and the TUI. Added: Claude Code's parameters, its timeout default and
//! cap, its result text (`Exit code N` for a failed command), and a working directory that
//! carries over from one command to the next (see `shell_cwd`).

use std::collections::BTreeMap;

use codex_claude_compat::tools::BASH;
use codex_claude_compat::tools::bash;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

use super::OFFERED_TOOLS;
use super::PermissionInput;
use super::check_permission;
use super::delegated;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::shell_cwd::Tracking;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::handlers::WriteStdinHandler;
use crate::tools::handlers::selected_executor;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use crate::tools::registry::ToolRegistry;

/// How long a command started with `run_in_background` is watched before the call returns.
const BACKGROUND_YIELD_MS: u64 = 1_000;

/// Registers `Bash` in place of `exec_command`, with `write_stdin` beside it when Codex
/// keeps processes running (unified exec), so that background commands can be read.
pub(crate) fn register(
    registry: &mut ToolRegistry,
    options: ExecCommandHandlerOptions,
    unified_exec: bool,
) {
    if !registry
        .tool_policy
        .allows(&ToolName::plain("exec_command"))
    {
        return;
    }
    registry.add(BashHandler {
        foreground: ExecCommandHandler::one_shot(options),
        background: unified_exec.then(|| ExecCommandHandler::new(options)),
    });
    if unified_exec {
        registry.add(WriteStdinHandler);
    }
}

struct BashHandler {
    /// Runs a command to completion or its timeout.
    foreground: ExecCommandHandler,
    /// Starts a command that keeps running; `None` when Codex's unified exec is off.
    background: Option<ExecCommandHandler>,
}

#[derive(Deserialize)]
struct BashArgs {
    command: String,
    #[serde(default)]
    timeout: Option<f64>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    run_in_background: Option<bool>,
    #[serde(default, rename = "dangerouslyDisableSandbox")]
    dangerously_disable_sandbox: Option<bool>,
}

impl ToolExecutor<ToolInvocation> for BashHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(BASH)
    }

    fn spec(&self) -> ToolSpec {
        let mut properties = BTreeMap::from([
            (
                "command".to_string(),
                JsonSchema::string(Some(bash::COMMAND_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "timeout".to_string(),
                JsonSchema::number(Some(bash::timeout_param_description())),
            ),
            (
                "description".to_string(),
                JsonSchema::string(Some(bash::DESCRIPTION_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "dangerouslyDisableSandbox".to_string(),
                JsonSchema::boolean(Some(bash::DISABLE_SANDBOX_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        if self.background.is_some() {
            properties.insert(
                "run_in_background".to_string(),
                JsonSchema::boolean(Some(bash::RUN_IN_BACKGROUND_PARAM_DESCRIPTION.to_string())),
            );
        }
        ToolSpec::Function(ResponsesApiTool {
            name: BASH.to_string(),
            description: bash::description(OFFERED_TOOLS, self.background.is_some()),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["command".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for BashHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

/// The `exec_command` arguments for a Claude Code `Bash` call. Output is truncated by
/// Codex, to the token budget its shell tool uses by default.
fn exec_command_arguments(args: &BashArgs, run: BashRun, workdir: Option<&str>) -> Value {
    let mut arguments = json!({
        "cmd": args.command,
        "max_output_tokens": crate::unified_exec::resolve_max_tokens(/*max_tokens*/ None),
    });
    if let Some(workdir) = workdir {
        arguments["workdir"] = json!(workdir);
    }
    match run {
        BashRun::Background => arguments["yield_time_ms"] = json!(BACKGROUND_YIELD_MS),
        BashRun::ForegroundThenBackground => {
            arguments["foreground_wait_ms"] = json!(bash::timeout_ms(args.timeout));
        }
        BashRun::Foreground => arguments["timeout_ms"] = json!(bash::timeout_ms(args.timeout)),
    }
    if args.dangerously_disable_sandbox == Some(true) {
        arguments["sandbox_permissions"] = json!("require_escalated");
        if let Some(description) = args
            .description
            .as_deref()
            .filter(|description| !description.trim().is_empty())
        {
            arguments["justification"] = json!(description);
        }
    }
    arguments
}

/// How a `Bash` call runs its command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BashRun {
    /// Until it exits or its timeout stops it.
    Foreground,
    /// Until it exits; at its timeout it is left running and the call returns.
    ForegroundThenBackground,
    /// `run_in_background`: the call returns at once.
    Background,
}

impl BashHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: BashArgs = parse_claude_arguments(BASH, &invocation)?;
        let _permission =
            check_permission(&invocation, BASH, PermissionInput::Command(&args.command)).await?;
        let background_on_timeout = invocation
            .turn
            .config
            .mod_config
            .harness
            .background_on_timeout
            .unwrap_or_else(crate::config::refinements_default)
            && bash::backgrounds_on_timeout(&args.command);
        let (handler, run) = match &self.background {
            Some(background) if args.run_in_background == Some(true) => {
                (background, BashRun::Background)
            }
            Some(background) if background_on_timeout => {
                (background, BashRun::ForegroundThenBackground)
            }
            _ => (&self.foreground, BashRun::Foreground),
        };
        let environment = selected_executor(
            &invocation.environments,
            &invocation.step_context.environments,
        )?;
        let tracking = Tracking::for_call(&invocation, environment);
        let workdir = tracking
            .as_ref()
            .and_then(|tracking| tracking.start.as_ref())
            .and_then(|start| start.as_path().to_str());
        let payload = ToolPayload::Function {
            arguments: exec_command_arguments(&args, run, workdir).to_string(),
        };
        // A background command's directory does not carry over, as in Claude Code. One left in
        // the background at its timeout reports nothing either: its result has no exit code.
        let report = tracking
            .as_ref()
            .filter(|_| run != BashRun::Background)
            .and_then(|tracking| tracking.request_report(&invocation.session, &invocation.call_id));
        let output = handler
            .handle(delegated(&invocation, payload.clone()))
            .await;
        // The exec runtime took the request unless the command never ran.
        let _ = super::take_cwd_report(&invocation.session, &invocation.call_id);
        let output = output?;

        // The structured result of `exec_command`: its truncated output, exit code, and the
        // session of a process still running.
        let result = output.code_mode_result(&payload);
        let text = result
            .get("output")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let exit_code = result
            .get("exit_code")
            .and_then(Value::as_i64)
            .and_then(|code| i32::try_from(code).ok());
        let session_id = result.get("session_id").and_then(Value::as_i64);

        if let Some(session_id) = session_id {
            let id = session_id.to_string();
            let tool_response = json!({
                "stdout": "",
                "stderr": "",
                "interrupted": false,
                "isImage": false,
                "backgroundTaskId": id,
            });
            let message = match run {
                BashRun::ForegroundThenBackground => bash::moved_to_background_message(
                    &id,
                    bash::timeout_ms(args.timeout),
                    text,
                ),
                BashRun::Foreground | BashRun::Background => bash::background_message(&id),
            };
            return Ok(boxed_tool_output(ClaudeToolOutput::new(
                message,
                tool_response,
            )));
        }

        // A one-shot run stopped at its timeout reports the exit code `timeout` itself uses.
        let timed_out = run == BashRun::Foreground && exit_code == Some(124);
        let run = bash::CommandRun {
            output: text,
            exit_code,
            timed_out,
            timeout_ms: bash::timeout_ms(args.timeout),
        };
        let (mut model_text, is_error) = bash::format_result(&run);
        let reset = match (&tracking, &report) {
            (Some(tracking), Some((_directory, file))) if exit_code == Some(0) && !timed_out => {
                std::fs::read_to_string(file).ok().and_then(|contents| {
                    bash::recorded_cwd(&contents)
                        .and_then(|cwd| tracking.finish(&invocation.session, cwd))
                })
            }
            _ => None,
        };
        let stdout = bash::strip_empty_lines(text);
        if let Some(reset) = &reset {
            if stdout.is_empty() && !is_error {
                model_text = reset.clone();
            } else {
                model_text.push('\n');
                model_text.push_str(reset);
            }
        }
        let tool_response = json!({
            "stdout": stdout,
            "stderr": reset.unwrap_or_default(),
            "interrupted": timed_out,
            "isImage": false,
        });
        Ok(boxed_tool_output(if is_error {
            ClaudeToolOutput::error(model_text, tool_response)
        } else {
            ClaudeToolOutput::new(model_text, tool_response)
        }))
    }
}

#[cfg(test)]
pub(super) fn exec_command_arguments_for_test(arguments: &str, background: bool) -> Value {
    let args: BashArgs = serde_json::from_str(arguments)
        .unwrap_or_else(|err| panic!("invalid Bash arguments: {err}"));
    exec_command_arguments(
        &args,
        if background {
            BashRun::Background
        } else {
            BashRun::Foreground
        },
        /*workdir*/ None,
    )
}
