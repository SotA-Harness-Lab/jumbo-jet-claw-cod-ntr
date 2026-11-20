// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Runs the model behind Claude Code's `prompt` and `agent` hooks.
//!
//! The hook is judged in a child session of the turn it belongs to, with the session's
//! model, read-only permissions and no approvals to ask for. A `prompt` hook is told to
//! answer from its prompt alone; an `agent` hook may look around with its tools first.
//! Either ends with one JSON object, `{"ok": bool, "reason": string}`.

use std::sync::Arc;

use anyhow::Context;
use anyhow::bail;
use codex_features::Feature;
use codex_hooks::HookJudgeRequest;
use codex_protocol::config_types::WebSearchMode;
use codex_protocol::models::BaseInstructionsProvenance;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use codex_user_input::ResolvedUserInput;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::codex_delegate::run_codex_thread_one_shot;
use crate::config::Constrained;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;

/// The name the child session of a hook goes by. Hooks do not run inside it for a `prompt`
/// or `agent` handler again, which would otherwise never end.
const HOOK_JUDGE: &str = "hook_judge";

const PROMPT_INSTRUCTIONS: &str = "You are evaluating a hook. The user message describes a \
condition and gives the hook's input. Decide from that message alone whether the condition \
is met; do not call any tool.

Your answer must be a JSON object matching one of the following:
1. If the condition is met: {\"ok\": true, \"reason\": null}
2. If the condition is not met: {\"ok\": false, \"reason\": \"Reason for why it is not met\"}";

const AGENT_INSTRUCTIONS: &str = "You are verifying a stop condition for a coding agent. The \
user message describes the condition and gives the hook's input, which names the transcript \
of the session and the working directory. Use the tools you have to inspect the codebase and \
check whether the condition holds. Only read: change nothing. Use as few steps as possible.

When you are done, your final answer must be a JSON object matching one of the following:
1. If the condition is met: {\"ok\": true, \"reason\": null}
2. If the condition is not met: {\"ok\": false, \"reason\": \"Reason for why it is not met\"}";

/// Cancels the child session when the hook that waits for it gives up.
struct CancelOnDrop(CancellationToken);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

pub(crate) async fn judge(
    session: &Arc<Session>,
    turn: &Arc<TurnContext>,
    request: HookJudgeRequest,
) -> anyhow::Result<String> {
    if matches!(
        &turn.session_source,
        SessionSource::SubAgent(SubAgentSource::Other(name)) if name == HOOK_JUDGE
    ) {
        bail!("prompt and agent hooks do not run inside a hook's own session");
    }

    let mut config = turn.config.as_ref().clone();
    let _ = config.web_search_mode.set(WebSearchMode::Disabled);
    let _ = config.features.disable(Feature::Collab);
    let _ = config.features.disable(Feature::MultiAgentV2);
    config.base_instructions = Some(
        if request.agentic {
            AGENT_INSTRUCTIONS
        } else {
            PROMPT_INSTRUCTIONS
        }
        .to_string(),
    );
    config.base_instructions_provenance = Some(BaseInstructionsProvenance::Custom);
    config.permissions.approval_policy = Constrained::allow_only(AskForApproval::Never);
    if let Some(read_only) = config
        .permissions
        .permission_profile()
        .intersect_with_read_only()
    {
        let _ = config.permissions.set_permission_profile(read_only);
    }
    config.model = Some(turn.model_info().slug.clone());

    let cancellation = CancellationToken::new();
    let _cancel_on_drop = CancelOnDrop(cancellation.clone());
    let (_child, io) = run_codex_thread_one_shot(
        config,
        Arc::clone(&session.services.auth_manager),
        Arc::clone(&session.services.models_manager),
        vec![ResolvedUserInput::text(request.prompt)],
        Arc::clone(session),
        Arc::clone(turn),
        cancellation,
        SubAgentSource::Other(HOOK_JUDGE.to_string()),
        Some(json!({
            "type": "object",
            "properties": {
                "ok": { "type": "boolean" },
                "reason": { "type": ["string", "null"] },
            },
            // Strict output wants every property listed; `reason` is null when met.
            "required": ["ok", "reason"],
            "additionalProperties": false,
        })),
        /*initial_history*/ None,
    )
    .await
    .context("failed to start the hook's session")?;

    while let Ok(event) = io.rx_event.recv().await {
        match event.msg {
            EventMsg::TurnComplete(turn_complete) => {
                return turn_complete
                    .last_agent_message
                    .context("the model gave no answer");
            }
            EventMsg::TurnAborted(_) => bail!("the hook's session was interrupted"),
            _ => {}
        }
    }
    bail!("the hook's session ended without an answer")
}
