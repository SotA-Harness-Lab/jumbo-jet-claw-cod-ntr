// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `prompt` and `agent` hook handlers: a model reads the handler's prompt,
//! with the hook input in place of `$ARGUMENTS`, and answers whether the condition it
//! describes is met.
//!
//! The verdict is turned into what a command hook would have produced, so that every event
//! reads it the way it reads a command: met is exit code 0, not met is exit code 2 with the
//! model's reason on stderr (which blocks, as in Claude Code), and anything that goes wrong
//! is an error that does not block.

use std::time::Duration;
use std::time::Instant;

use codex_protocol::models::ContentItemMetadata;
use serde::Deserialize;

use super::ConfiguredHandler;
use super::HandlerRunResult;
use crate::HookEnvironmentExecutor;
use crate::HookJudgeRequest;

const ARGUMENTS: &str = "$ARGUMENTS";

#[derive(Deserialize)]
struct Verdict {
    ok: bool,
    #[serde(default)]
    reason: Option<String>,
}

pub(crate) async fn run_judge(
    executor: Option<&dyn HookEnvironmentExecutor>,
    handler: &ConfiguredHandler,
    prompt: &str,
    agentic: bool,
    model: Option<&str>,
    input_json: &str,
) -> HandlerRunResult {
    let started_at = chrono::Utc::now().timestamp();
    let started = Instant::now();
    let kind = if agentic { "Agent" } else { "Prompt" };
    let timeout = Duration::from_secs(handler.timeout_sec);
    let answer = match executor {
        Some(executor) => {
            let request = HookJudgeRequest {
                prompt: with_arguments(prompt, input_json),
                agentic,
                model: model.map(str::to_string),
                timeout,
            };
            match tokio::time::timeout(timeout, executor.judge(request)).await {
                Ok(answer) => answer.map_err(|err| err.to_string()),
                Err(_) => Err(format!("timed out after {}s", handler.timeout_sec)),
            }
        }
        None => Err("no model is available to run this hook here".to_string()),
    };
    let (exit_code, stdout, stderr) = match answer {
        Ok(answer) => verdict_output(kind, &answer),
        Err(error) => (
            1,
            String::new(),
            format!("Error executing {} hook: {error}", kind.to_lowercase()),
        ),
    };
    HandlerRunResult {
        structured_content: None,
        context_metadata: ContentItemMetadata::command_hook(),
        started_at,
        completed_at: chrono::Utc::now().timestamp(),
        duration_ms: started.elapsed().as_millis().try_into().unwrap_or(i64::MAX),
        exit_code: Some(exit_code),
        stdout,
        stderr,
        error: None,
    }
}

/// The prompt with the hook input in place of each `$ARGUMENTS`, or after the prompt when it
/// names none, as Claude Code's argument substitution does.
fn with_arguments(prompt: &str, input_json: &str) -> String {
    if prompt.contains(ARGUMENTS) {
        prompt.replace(ARGUMENTS, input_json)
    } else {
        format!("{prompt}\n\nARGUMENTS: {input_json}")
    }
}

/// Exit code, stdout and stderr for the model's answer.
fn verdict_output(kind: &str, answer: &str) -> (i32, String, String) {
    let trimmed = answer.trim();
    match serde_json::from_str::<Verdict>(trimmed) {
        Ok(Verdict { ok: true, .. }) => (0, String::new(), String::new()),
        Ok(Verdict { ok: false, reason }) => (
            2,
            String::new(),
            format!(
                "{kind} hook condition was not met: {}",
                reason.as_deref().unwrap_or("no reason given")
            ),
        ),
        Err(error) => (
            1,
            trimmed.to_string(),
            format!("JSON validation failed: {error}"),
        ),
    }
}

#[cfg(test)]
#[path = "judge_runner_tests.rs"]
mod tests;
