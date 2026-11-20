// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: run Claude Code's prompt, agent and http hooks

//! Owner-admitted hooks bound to the exact environment that supplied them.

use std::collections::HashMap;
use std::time::Duration;

use codex_protocol::hooks::HookEventsToml;
use codex_utils_path_uri::PathUri;
use futures::future::BoxFuture;

/// An admitted requirements source. The owner must not populate this from user configuration.
pub struct EnvironmentHookSource {
    pub environment_id: String,
    /// Working directory from the same captured environment used for execution.
    pub cwd: PathUri,
    pub hooks: HookEventsToml,
}

/// Executes commands in ready environments without coupling the hooks engine to core or transports.
/// Implementations must honor the timeout, terminate timed-out processes, and bound captured output.
pub trait HookEnvironmentExecutor: Send + Sync {
    fn execute(
        &self,
        environment_id: &str,
        command: &str,
        env: &HashMap<String, String>,
        input_json: &str,
        timeout: Duration,
    ) -> BoxFuture<'_, anyhow::Result<EnvironmentHookOutput>>;

    /// Has a model judge a `prompt` or `agent` hook of a Claude Code hooks file and returns
    /// the JSON it answers with, `{"ok": bool, "reason": string}`. The engine has no model of
    /// its own, so an implementation that has none either keeps this default and the hook
    /// fails without blocking.
    fn judge(&self, request: HookJudgeRequest) -> BoxFuture<'_, anyhow::Result<String>> {
        let _ = request;
        Box::pin(async { anyhow::bail!("no model is available to run this hook here") })
    }
}

/// One `prompt` or `agent` hook to be judged by a model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookJudgeRequest {
    /// The handler's prompt, with the hook input already filled in.
    pub prompt: String,
    /// An `agent` hook, which may look around with tools before it answers; a `prompt` hook
    /// answers from the prompt alone.
    pub agentic: bool,
    /// How long the handler may take.
    pub timeout: Duration,
}

pub struct EnvironmentHookOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}
