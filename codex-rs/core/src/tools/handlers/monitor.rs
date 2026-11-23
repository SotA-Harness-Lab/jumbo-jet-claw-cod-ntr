// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Monitor`: the model starts a long-running command and is told of what it writes, a
//! few lines at a time, in place of asking for its output again and again.
//!
//! The command is started as any background command is, with the same approvals and
//! sandbox, and stays one: its output can still be read with `write_stdin`, and its exit is
//! reported like any background command's. A task beside it reads the lines as they come
//! and delivers them to the session, starting a turn when it is idle.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Weak;
use std::time::Duration;

use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use regex_lite::Regex;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use tokio::sync::broadcast::error::RecvError;
use tokio::time::Instant;

use crate::context::ContextualUserFragment;
use crate::context::MonitorEvent;
use crate::function_tool::FunctionCallError;
use crate::session::session::Session;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use crate::unified_exec::OutputSubscription;

const MONITOR: &str = "Monitor";

const DEFAULT_TIMEOUT_MS: u64 = 300_000;
const MIN_TIMEOUT_MS: u64 = 1_000;
const MAX_TIMEOUT_MS: u64 = 1_800_000;

/// How long the command is watched by the starting call, for what it writes at once.
const START_YIELD_MS: u64 = 1_000;
/// Lines that arrive within this long of each other make one event.
const BATCH: Duration = Duration::from_millis(200);
/// At least this long lies between two events, so that a chatty command does not start a
/// turn for every line.
const MIN_EVENT_INTERVAL: Duration = Duration::from_secs(2);
/// A monitor that has sent this many events is stopped: it is not watching for anything
/// in particular.
const MAX_EVENTS: usize = 100;
/// What one event carries at most; the rest is counted.
const MAX_LINES_PER_EVENT: usize = 40;
const MAX_LINE_BYTES: usize = 2_000;

const DESCRIPTION: &str = codex_prompt_texts::text!("core/tools/handlers/monitor/description.md");

pub struct MonitorHandler {
    /// Starts the command, as `exec_command` would.
    exec: ExecCommandHandler,
    /// Whether the Claude tool set is on, whose permission rules for `Bash` then decide
    /// about the command first.
    claude_tools: bool,
}

impl MonitorHandler {
    pub(crate) fn new(exec: ExecCommandHandler, claude_tools: bool) -> Self {
        Self { exec, claude_tools }
    }
}

#[derive(Debug, Deserialize)]
struct MonitorArgs {
    command: String,
    description: String,
    #[serde(default)]
    timeout_ms: Option<u64>,
    #[serde(default)]
    pattern: Option<String>,
}

impl ToolExecutor<ToolInvocation> for MonitorHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(MONITOR)
    }

    fn spec(&self) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: MONITOR.to_string(),
            description: DESCRIPTION.to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                BTreeMap::from([
                    (
                        "command".to_string(),
                        JsonSchema::string(Some(
                            codex_prompt_texts::text!("core/tools/handlers/monitor/command_description.md")
                                .to_string(),
                        )),
                    ),
                    (
                        "description".to_string(),
                        JsonSchema::string(Some(
                            "What is being watched, in a few words; it names the monitor in its events."
                                .to_string(),
                        )),
                    ),
                    (
                        "timeout_ms".to_string(),
                        JsonSchema::number(Some(format!(
                            codex_prompt_texts::text!("core/tools/handlers/monitor/timeout_description.md")
                        , DEFAULT_TIMEOUT_MS = DEFAULT_TIMEOUT_MS, MIN_TIMEOUT_MS = MIN_TIMEOUT_MS, MAX_TIMEOUT_MS = MAX_TIMEOUT_MS))),
                    ),
                    (
                        "pattern".to_string(),
                        JsonSchema::string(Some(
                            "A regular expression; only lines it matches are delivered. Without it every line is."
                                .to_string(),
                        )),
                    ),
                ]),
                Some(vec!["command".to_string(), "description".to_string()]),
                /*additional_properties*/ Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for MonitorHandler {}

impl MonitorHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn crate::tools::context::ToolOutput>, FunctionCallError> {
        let ToolPayload::Function { arguments } = &invocation.payload else {
            return Err(FunctionCallError::RespondToModel(format!(
                "{MONITOR} handler received unsupported payload"
            )));
        };
        let args: MonitorArgs = parse_arguments(arguments)?;
        if args.command.trim().is_empty() || args.description.trim().is_empty() {
            return Err(FunctionCallError::RespondToModel(
                "command and description must not be empty".to_string(),
            ));
        }
        let pattern = args
            .pattern
            .as_deref()
            .filter(|pattern| !pattern.is_empty())
            .map(Regex::new)
            .transpose()
            .map_err(|err| {
                FunctionCallError::RespondToModel(format!("pattern is not a valid regex: {err}"))
            })?;
        let timeout = Duration::from_millis(
            args.timeout_ms
                .unwrap_or(DEFAULT_TIMEOUT_MS)
                .clamp(MIN_TIMEOUT_MS, MAX_TIMEOUT_MS),
        );
        let _permission = if self.claude_tools {
            Some(
                crate::tools::claude::check_permission(
                    &invocation,
                    codex_claude_compat::tools::BASH,
                    crate::tools::claude::PermissionInput::Command(&args.command),
                )
                .await?,
            )
        } else {
            None
        };

        let payload = ToolPayload::Function {
            arguments: json!({
                "cmd": args.command,
                "yield_time_ms": START_YIELD_MS,
            })
            .to_string(),
        };
        let output = self
            .exec
            .handle(crate::tools::claude::delegated(
                &invocation,
                payload.clone(),
            ))
            .await?;
        let result = output.code_mode_result(&payload);
        let first_output = result
            .get("output")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .trim_end()
            .to_string();
        let Some(session_id) = result.get("session_id").and_then(Value::as_i64) else {
            let exit_code = result.get("exit_code").and_then(Value::as_i64);
            return Ok(text(format!(
                "The command ended before it could be monitored{}. A monitor is for a command that keeps running.{}",
                exit_code.map_or_else(String::new, |code| format!(" (exit code {code})")),
                output_section(&first_output),
            )));
        };
        let process_id = i32::try_from(session_id).unwrap_or(i32::MAX);
        let Some(subscription) = invocation
            .session
            .services
            .unified_exec_manager
            .subscribe_output(process_id)
            .await
        else {
            return Ok(text(format!(
                codex_prompt_texts::text!("core/tools/handlers/monitor/output_not_followed.md"),
                output_section(&first_output),
                session_id = session_id,
            )));
        };
        tokio::spawn(watch(Monitor {
            session: Arc::downgrade(&invocation.session),
            session_id: session_id.to_string(),
            process_id,
            description: args.description.clone(),
            pattern,
            timeout,
            subscription,
        }));
        Ok(text(format!(
            codex_prompt_texts::text!("core/tools/handlers/monitor/started.md"),
            args.description,
            if args
                .pattern
                .as_deref()
                .is_some_and(|pattern| !pattern.is_empty())
            {
                " that match the pattern"
            } else {
                ""
            },
            timeout.as_secs(),
            output_section(&first_output),
            session_id = session_id,
            MAX_EVENTS = MAX_EVENTS,
        )))
    }
}

fn text(text: String) -> Box<dyn crate::tools::context::ToolOutput> {
    boxed_tool_output(FunctionToolOutput::from_text(text, Some(true)))
}

fn output_section(output: &str) -> String {
    if output.is_empty() {
        String::new()
    } else {
        format!("\nOutput so far:\n{output}")
    }
}

struct Monitor {
    session: Weak<Session>,
    session_id: String,
    process_id: i32,
    description: String,
    pattern: Option<Regex>,
    timeout: Duration,
    subscription: OutputSubscription,
}

/// Lines waiting to be sent as one event.
#[derive(Default)]
struct Pending {
    lines: Vec<String>,
    omitted: usize,
}

impl Pending {
    fn push(&mut self, line: &str, pattern: Option<&Regex>) {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() || pattern.is_some_and(|pattern| !pattern.is_match(line)) {
            return;
        }
        if self.lines.len() >= MAX_LINES_PER_EVENT {
            self.omitted += 1;
            return;
        }
        let mut end = line.len().min(MAX_LINE_BYTES);
        while !line.is_char_boundary(end) {
            end -= 1;
        }
        self.lines.push(line[..end].to_string());
    }

    fn is_empty(&self) -> bool {
        self.lines.is_empty() && self.omitted == 0
    }
}

/// Splits the bytes of a stream into its complete lines, keeping the unfinished last one.
#[derive(Default)]
struct LineBuffer(Vec<u8>);

impl LineBuffer {
    fn push(&mut self, bytes: &[u8], mut line: impl FnMut(&str)) {
        self.0.extend_from_slice(bytes);
        while let Some(end) = self.0.iter().position(|byte| *byte == b'\n') {
            let complete = self.0.drain(..=end).collect::<Vec<_>>();
            line(&String::from_utf8_lossy(&complete[..complete.len() - 1]));
        }
        // A line without an end is not held for ever.
        if self.0.len() > 4 * MAX_LINE_BYTES {
            let complete = std::mem::take(&mut self.0);
            line(&String::from_utf8_lossy(&complete));
        }
    }

    fn finish(&mut self, mut line: impl FnMut(&str)) {
        if !self.0.is_empty() {
            let rest = std::mem::take(&mut self.0);
            line(&String::from_utf8_lossy(&rest));
        }
    }
}

async fn watch(mut monitor: Monitor) {
    let deadline = Instant::now() + monitor.timeout;
    let mut buffer = LineBuffer::default();
    let mut pending = Pending::default();
    let mut events = 0usize;
    // When the lines waiting are sent: a short while after the first of them, and not too
    // soon after the event before.
    let mut send_at: Option<Instant> = None;
    let mut last_event: Option<Instant> = None;
    let ended = loop {
        let flush = async {
            match send_at {
                Some(send_at) => tokio::time::sleep_until(send_at).await,
                None => std::future::pending().await,
            }
        };
        tokio::select! {
            chunk = monitor.subscription.output.recv() => match chunk {
                Ok(bytes) => {
                    buffer.push(&bytes, |line| pending.push(line, monitor.pattern.as_ref()));
                }
                Err(RecvError::Lagged(_)) => pending.omitted += 1,
                Err(RecvError::Closed) => break None,
            },
            () = monitor.subscription.exited.cancelled() => {
                // What the command wrote last may still be on its way.
                tokio::time::sleep(Duration::from_millis(100)).await;
                while let Ok(bytes) = monitor.subscription.output.try_recv() {
                    buffer.push(&bytes, |line| pending.push(line, monitor.pattern.as_ref()));
                }
                break None;
            }
            () = flush => {
                send_at = None;
                if !deliver(&monitor, std::mem::take(&mut pending), /*ended*/ None).await {
                    return;
                }
                last_event = Some(Instant::now());
                events += 1;
                if events >= MAX_EVENTS {
                    break Some(format!(
                        "stopped after {MAX_EVENTS} events: make the command write less, or give a pattern"
                    ));
                }
            }
            () = tokio::time::sleep_until(deadline) => {
                break Some(format!("timed out after {}s", monitor.timeout.as_secs()));
            }
        }
        if send_at.is_none() && !pending.is_empty() {
            let earliest = last_event.map_or_else(Instant::now, |last| last + MIN_EVENT_INTERVAL);
            send_at = Some((Instant::now() + BATCH).max(earliest));
        }
    };
    buffer.finish(|line| pending.push(line, monitor.pattern.as_ref()));
    match ended {
        // The monitor ends its command; the last lines go with the reason.
        Some(reason) => {
            if let Some(session) = monitor.session.upgrade() {
                session
                    .services
                    .unified_exec_manager
                    .terminate_process(monitor.process_id)
                    .await;
            }
            deliver(&monitor, pending, Some(reason)).await;
        }
        // The command ended by itself, which is reported as any background command's exit
        // is; only lines not yet sent are left to deliver.
        None if !pending.is_empty() => {
            deliver(&monitor, pending, Some("the command ended".to_string())).await;
        }
        None => {}
    }
}

/// Sends one event to the session; `false` when the session is gone.
async fn deliver(monitor: &Monitor, pending: Pending, ended: Option<String>) -> bool {
    let Some(session) = monitor.session.upgrade() else {
        return false;
    };
    if session.turn_starts_closed() {
        return false;
    }
    session
        .deliver_background_notice(ContextualUserFragment::into(MonitorEvent {
            session_id: monitor.session_id.clone(),
            description: monitor.description.clone(),
            lines: pending.lines,
            omitted_lines: pending.omitted,
            ended,
        }))
        .await;
    true
}

#[cfg(test)]
#[path = "monitor_tests.rs"]
mod tests;
