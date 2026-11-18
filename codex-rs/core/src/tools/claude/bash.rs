//! `Bash`: Claude Code's shell tool.
//!
//! Delegated to Codex's `exec_command` runtime: the shell, execution policy and command
//! rules, approval, sandbox (`dangerouslyDisableSandbox` asks for an unsandboxed run as
//! `sandbox_permissions: "require_escalated"` does), the timeout (a one-shot run),
//! background processes (an interactive run, read and stopped with `write_stdin`), output
//! truncation, events and the TUI. Added: Claude Code's parameters, its timeout default and
//! cap, and its result text (`Exit code N` for a failed command).

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
use super::delegated;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::handlers::WriteStdinHandler;
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
fn exec_command_arguments(args: &BashArgs, background: bool) -> Value {
    let mut arguments = json!({
        "cmd": args.command,
        "max_output_tokens": crate::unified_exec::resolve_max_tokens(/*max_tokens*/ None),
    });
    if background {
        arguments["yield_time_ms"] = json!(BACKGROUND_YIELD_MS);
    } else {
        arguments["timeout_ms"] = json!(bash::timeout_ms(args.timeout));
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

impl BashHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: BashArgs = parse_claude_arguments(BASH, &invocation)?;
        let (handler, background) = match &self.background {
            Some(background) if args.run_in_background == Some(true) => (background, true),
            _ => (&self.foreground, false),
        };
        let payload = ToolPayload::Function {
            arguments: exec_command_arguments(&args, background).to_string(),
        };
        let output = handler
            .handle(delegated(&invocation, payload.clone()))
            .await?;

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
            return Ok(boxed_tool_output(ClaudeToolOutput::new(
                bash::background_message(&id),
                tool_response,
            )));
        }

        // A one-shot run stopped at its timeout has neither an exit code nor a process; an
        // `apply_patch` command Codex intercepts has neither either, but no output chunk.
        let timed_out = !background && exit_code.is_none() && result.get("chunk_id").is_some();
        let run = bash::CommandRun {
            output: text,
            exit_code,
            timed_out,
            timeout_ms: bash::timeout_ms(args.timeout),
        };
        let (model_text, is_error) = bash::format_result(&run);
        let tool_response = json!({
            "stdout": bash::strip_empty_lines(text),
            "stderr": "",
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
    exec_command_arguments(&args, background)
}
