// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code hook events that Codex's own hook configuration does not have:
//! `Notification`, `PostToolUseFailure` and `StopFailure`.
//!
//! They are read only from Claude Code's settings files and run only command handlers. Their
//! stdin is shaped as Claude Code shapes it, and, as in Claude Code, they run outside the
//! transcript: no hook run is reported to the UI. Only `PostToolUseFailure` can talk back,
//! through `additionalContext` (or exit code 2 and stderr), which reaches the model.

use std::path::Path;
use std::path::PathBuf;

use codex_protocol::ThreadId;
use codex_protocol::protocol::CodexErrorInfo;
use codex_protocol::protocol::HookEventName;
use codex_protocol::protocol::HookExecutionMode;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;
use serde_json::json;

use super::common;
use crate::engine::ClaudeHooksEngine;
use crate::engine::ConfiguredHandler;
use crate::engine::HandlerRunResult;
use crate::engine::dispatcher;
use crate::output_spill::AdditionalContext;

/// A Claude Code hook event that Codex's own configuration does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClaudeOnlyEvent {
    /// Codex waits for the user: an approval prompt or an MCP elicitation.
    Notification,
    /// A tool call failed, including a shell command that exited with a non-zero code.
    PostToolUseFailure,
    /// A turn ended on an error from the model provider instead of an answer.
    StopFailure,
}

impl ClaudeOnlyEvent {
    pub(crate) const ALL: [Self; 3] = [
        Self::Notification,
        Self::PostToolUseFailure,
        Self::StopFailure,
    ];

    /// The event's name in Claude Code's settings and in `hook_event_name`.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Notification => "Notification",
            Self::PostToolUseFailure => "PostToolUseFailure",
            Self::StopFailure => "StopFailure",
        }
    }

    /// The Codex event whose name a handler of this event carries internally. It never
    /// reaches a hook's stdin or the UI.
    pub(crate) fn stand_in(self) -> HookEventName {
        match self {
            Self::Notification => HookEventName::PermissionRequest,
            Self::PostToolUseFailure => HookEventName::PostToolUse,
            Self::StopFailure => HookEventName::Stop,
        }
    }
}

/// A command handler of a Claude-only event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClaudeOnlyHandler {
    pub event: ClaudeOnlyEvent,
    pub handler: ConfiguredHandler,
}

/// Codex is waiting for the user.
#[derive(Debug, Clone)]
pub struct NotificationRequest {
    pub session_id: ThreadId,
    pub cwd: AbsolutePathBuf,
    pub transcript_path: Option<PathBuf>,
    pub permission_mode: String,
    /// Claude Code's notification type, matched by the handlers' matchers:
    /// `permission_prompt`, `elicitation_dialog` or `elicitation_url_dialog`.
    pub notification_type: String,
    pub message: String,
    pub title: Option<String>,
}

/// A tool call failed.
#[derive(Debug, Clone)]
pub struct PostToolUseFailureRequest {
    pub session_id: ThreadId,
    pub cwd: AbsolutePathBuf,
    pub transcript_path: Option<PathBuf>,
    pub permission_mode: String,
    pub tool_name: String,
    pub matcher_aliases: Vec<String>,
    pub tool_use_id: String,
    pub tool_input: Value,
    /// What went wrong, as the model sees it; for a shell command, `Exit code N` and its output.
    pub error: String,
    pub is_interrupt: bool,
}

#[derive(Debug, Default)]
pub struct PostToolUseFailureOutcome {
    pub additional_contexts: Vec<crate::HookContext>,
}

/// A turn ended on an error instead of an answer.
#[derive(Debug, Clone)]
pub struct StopFailureRequest {
    pub session_id: ThreadId,
    pub cwd: AbsolutePathBuf,
    pub transcript_path: Option<PathBuf>,
    pub permission_mode: String,
    /// Claude Code's error kind, matched by the handlers' matchers; see [`stop_failure_error`].
    pub error: String,
    pub error_details: Option<String>,
    pub last_assistant_message: Option<String>,
}

/// Claude Code's name for the kind of error that ended a turn (`rate_limit`, `server_error`,
/// ...), which `StopFailure` hooks receive as `error` and match against.
pub fn stop_failure_error(info: &CodexErrorInfo) -> &'static str {
    match info {
        CodexErrorInfo::Unauthorized => "authentication_failed",
        CodexErrorInfo::UsageLimitExceeded | CodexErrorInfo::SessionBudgetExceeded => {
            "billing_error"
        }
        CodexErrorInfo::RateLimitExceeded => "rate_limit",
        CodexErrorInfo::BadRequest
        | CodexErrorInfo::InvalidPrompt
        | CodexErrorInfo::ContextWindowExceeded
        | CodexErrorInfo::ModelNotFound => "invalid_request",
        CodexErrorInfo::ServerOverloaded
        | CodexErrorInfo::InternalServerError
        | CodexErrorInfo::FlexUnavailable
        | CodexErrorInfo::PolicyCapacityUnavailable
        | CodexErrorInfo::HttpConnectionFailed { .. }
        | CodexErrorInfo::ResponseStreamConnectionFailed { .. }
        | CodexErrorInfo::ResponseStreamDisconnected { .. }
        | CodexErrorInfo::ResponseTooManyFailedAttempts { .. } => "server_error",
        _ => "unknown",
    }
}

impl ClaudeHooksEngine {
    pub(crate) fn has_claude_only_handlers(&self, event: ClaudeOnlyEvent) -> bool {
        self.claude_only_handlers
            .iter()
            .any(|handler| handler.event == event)
    }

    fn select_claude_only(
        &self,
        event: ClaudeOnlyEvent,
        matcher_inputs: &[&str],
    ) -> Vec<ConfiguredHandler> {
        self.claude_only_handlers
            .iter()
            .filter(|handler| handler.event == event)
            .filter(|handler| {
                matcher_inputs.iter().any(|input| {
                    common::matches_matcher(handler.handler.matcher.as_ref(), Some(*input))
                })
            })
            .map(|handler| handler.handler.clone())
            .collect()
    }

    /// Starts the `Notification` hooks in the background, so that the prompt the user is
    /// waiting for is not held up by them.
    pub(crate) fn spawn_notification(&self, request: NotificationRequest) {
        let handlers = self.select_claude_only(
            ClaudeOnlyEvent::Notification,
            &[request.notification_type.as_str()],
        );
        if handlers.is_empty() {
            return;
        }
        let input = notification_input(&request);
        self.spawn_all(handlers, input, request.cwd);
    }

    /// Starts the `StopFailure` hooks in the background; Claude Code does not wait for them
    /// either, and their output is ignored.
    pub(crate) fn spawn_stop_failure(&self, request: StopFailureRequest) {
        let handlers =
            self.select_claude_only(ClaudeOnlyEvent::StopFailure, &[request.error.as_str()]);
        if handlers.is_empty() {
            return;
        }
        let input = stop_failure_input(&request);
        self.spawn_all(handlers, input, request.cwd);
    }

    /// Runs the `PostToolUseFailure` hooks and collects what they have to tell the model.
    pub(crate) async fn run_post_tool_use_failure(
        &self,
        request: PostToolUseFailureRequest,
    ) -> PostToolUseFailureOutcome {
        let matcher_inputs = common::matcher_inputs(&request.tool_name, &request.matcher_aliases);
        let handlers =
            self.select_claude_only(ClaudeOnlyEvent::PostToolUseFailure, &matcher_inputs);
        if handlers.is_empty() {
            return PostToolUseFailureOutcome::default();
        }
        let input = post_tool_use_failure_input(&request);
        let (background, foreground): (Vec<_>, Vec<_>) = handlers
            .into_iter()
            .partition(|handler| handler.execution_mode() == HookExecutionMode::Async);
        if !background.is_empty() {
            self.spawn_all(background, input.clone(), request.cwd.clone());
        }
        let results = futures::future::join_all(foreground.iter().map(|handler| {
            dispatcher::execute_handler(
                self,
                handler,
                &input,
                request.cwd.as_path(),
                /*metadata*/ None,
            )
        }))
        .await;
        let additional_contexts = foreground
            .iter()
            .zip(results)
            .filter_map(|(handler, result)| {
                post_tool_use_failure_context(&result).map(|text| AdditionalContext {
                    context: crate::HookContext {
                        text,
                        metadata: result.context_metadata.clone(),
                    },
                    limit: handler.additional_context_limit,
                })
            })
            .collect();
        PostToolUseFailureOutcome {
            additional_contexts: self
                .command_runtime
                .output_spiller()
                .maybe_spill_additional_contexts(additional_contexts)
                .await,
        }
    }

    fn spawn_all(&self, handlers: Vec<ConfiguredHandler>, input: Value, cwd: AbsolutePathBuf) {
        let engine = self.clone();
        self.command_runtime.schedule_async_task(async move {
            futures::future::join_all(
                handlers
                    .iter()
                    .map(|handler| run_ignoring_output(&engine, handler, &input, cwd.as_path())),
            )
            .await;
        });
    }
}

async fn run_ignoring_output(
    engine: &ClaudeHooksEngine,
    handler: &ConfiguredHandler,
    input: &Value,
    cwd: &Path,
) {
    let result = dispatcher::execute_handler(engine, handler, input, cwd, /*metadata*/ None).await;
    if let Some(error) = result.error {
        tracing::warn!(source_path = %handler.source_path, %error, "Claude Code hook failed");
    }
}

/// The context a `PostToolUseFailure` handler gives the model: `additionalContext` of its
/// JSON output on exit code 0, its stderr on exit code 2, as in Claude Code.
fn post_tool_use_failure_context(result: &HandlerRunResult) -> Option<String> {
    if result.error.is_some() {
        return None;
    }
    match result.exit_code {
        Some(0) => serde_json::from_str::<Value>(result.stdout.trim())
            .ok()?
            .pointer("/hookSpecificOutput/additionalContext")?
            .as_str()
            .and_then(common::trimmed_non_empty),
        Some(2) => common::trimmed_non_empty(&result.stderr),
        _ => None,
    }
}

fn path_value(path: Option<&PathBuf>) -> Value {
    path.map_or(Value::Null, |path| {
        Value::String(path.display().to_string())
    })
}

fn notification_input(request: &NotificationRequest) -> Value {
    let mut input = json!({
        "session_id": request.session_id.to_string(),
        "transcript_path": path_value(request.transcript_path.as_ref()),
        "cwd": request.cwd.display().to_string(),
        "permission_mode": request.permission_mode,
        "hook_event_name": ClaudeOnlyEvent::Notification.label(),
        "message": request.message,
        "notification_type": request.notification_type,
    });
    if let Some(title) = &request.title {
        input["title"] = Value::String(title.clone());
    }
    input
}

fn post_tool_use_failure_input(request: &PostToolUseFailureRequest) -> Value {
    json!({
        "session_id": request.session_id.to_string(),
        "transcript_path": path_value(request.transcript_path.as_ref()),
        "cwd": request.cwd.display().to_string(),
        "permission_mode": request.permission_mode,
        "hook_event_name": ClaudeOnlyEvent::PostToolUseFailure.label(),
        "tool_name": request.tool_name,
        "tool_input": request.tool_input,
        "tool_use_id": request.tool_use_id,
        "error": request.error,
        "is_interrupt": request.is_interrupt,
    })
}

fn stop_failure_input(request: &StopFailureRequest) -> Value {
    let mut input = json!({
        "session_id": request.session_id.to_string(),
        "transcript_path": path_value(request.transcript_path.as_ref()),
        "cwd": request.cwd.display().to_string(),
        "permission_mode": request.permission_mode,
        "hook_event_name": ClaudeOnlyEvent::StopFailure.label(),
        "error": request.error,
    });
    if let Some(details) = &request.error_details {
        input["error_details"] = Value::String(details.clone());
    }
    if let Some(message) = &request.last_assistant_message {
        input["last_assistant_message"] = Value::String(message.clone());
    }
    input
}

#[cfg(test)]
#[path = "claude_only_tests.rs"]
mod tests;
