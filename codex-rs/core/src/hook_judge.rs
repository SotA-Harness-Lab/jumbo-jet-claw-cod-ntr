// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Runs the model behind Claude Code's `prompt` and `agent` hooks.
//!
//! A `prompt` hook is one request without tools, as in Claude Code: short instructions, the
//! hook's prompt, the answer as JSON, at low reasoning effort where the model has it. An
//! `agent` hook is judged in a child session of the turn it belongs to, with read-only
//! permissions and no approvals to ask for, where it may look around with its tools first.
//! Either ends with one JSON object, `{"ok": bool, "reason": string}`.
//!
//! A handler names a Claude model, or none, for which Claude Code takes its small, fast one.
//! `[hooks.models]` of `ccn-config.toml` says which Codex model judges in the place of each;
//! where it has no entry, the handler's `model` is ignored and the session's own model
//! judges, as it does when the model of the entry cannot be used.

use std::sync::Arc;

use anyhow::Context;
use anyhow::bail;
use codex_features::Feature;
use codex_hooks::HookJudgeRequest;
use codex_protocol::config_types::WebSearchMode;
use codex_protocol::items::TurnItem;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::BaseInstructionsProvenance;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ReasoningEffort;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use codex_rollout_trace::InferenceTraceContext;
use codex_user_input::ResolvedUserInput;
use futures::StreamExt;
use serde_json::Value;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::Prompt;
use crate::client_common::ResponseEvent;
use crate::codex_delegate::run_codex_thread_one_shot;
use crate::config::Constrained;
use crate::responses_metadata::CodexResponsesRequestKind;
use crate::session::session::Session;
use crate::session::turn_context::TurnContext;

/// The name the child session of a hook goes by. Hooks do not run inside it for a `prompt`
/// or `agent` handler again, which would otherwise never end.
const HOOK_JUDGE: &str = "hook_judge";

/// How many tool calls an `agent` hook's verifier may make before it has to have answered.
/// Claude Code ends its verifier after 50 turns; a turn here shows as the tool calls it
/// starts, so this is the stricter count.
const MAX_AGENT_HOOK_TOOL_CALLS: usize = 50;

/// How both kinds of hook read the hook's text: Claude Code's hooks are written either as a
/// condition or as a rule, and a rule read as a condition lets through what it blocks.
macro_rules! hook_text_kinds {
    () => {
        "The hook's text is either a condition that must hold, such as \"All unit tests pass\", \
or a rule about what to block or allow, such as \"Block any Bash command that reads .env \
files\". A condition is met when it holds. A rule is met when the input is not something the \
rule blocks, and is something it allows where it names what to allow: when the input is what \
a rule blocks, the rule is not met. On a hook that runs when the agent is about to stop, not \
met means the agent has to keep working."
    };
}

const PROMPT_INSTRUCTIONS: &str = concat!(
    "You are evaluating a hook. The user message gives the hook's text and the hook's \
input. Decide from that message alone whether the hook's text is met.

",
    hook_text_kinds!(),
    "

Your answer must be a JSON object matching one of the following:
1. If it is met: {\"ok\": true, \"reason\": null}
2. If it is not met: {\"ok\": false, \"reason\": \"Reason for why it is not met\"}"
);

const AGENT_INSTRUCTIONS: &str = concat!(
    "You are verifying a hook for a coding agent. The user message gives the hook's text \
and the hook's input, which names the transcript of the session and the working directory. \
Use the tools you have to inspect the codebase and check whether the hook's text is met. Only \
read: change nothing. Use as few steps as possible.

",
    hook_text_kinds!(),
    "

When you are done, your final answer must be a JSON object matching one of the following:
1. If it is met: {\"ok\": true, \"reason\": null}
2. If it is not met: {\"ok\": false, \"reason\": \"Reason for why it is not met\"}"
);

/// Cancels the child session when the hook that waits for it gives up.
struct CancelOnDrop(CancellationToken);

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

/// The answer every hook of these two types ends with.
fn verdict_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "ok": { "type": "boolean" },
            "reason": { "type": ["string", "null"] },
        },
        // Strict output wants every property listed; `reason` is null when met.
        "required": ["ok", "reason"],
        "additionalProperties": false,
    })
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
    let model = turn
        .config
        .mod_config
        .hooks
        .model_for(request.model.as_deref());
    let answer = judge_with(session, turn, &request, model).await;
    // A model of `[hooks.models]` that cannot be used (a misspelt name, one the account
    // lacks) must not turn the guard off: as in Claude Code, the session's own model judges.
    match (answer, model) {
        (Err(err), Some(mapped))
            if falls_back_to_session_model(&err, mapped, &turn.model_info().slug) =>
        {
            tracing::warn!(
                "the {mapped} model of [hooks.models] failed to judge a hook ({err:#}); \
                 the session's model judges it instead"
            );
            judge_with(session, turn, &request, /*model*/ None).await
        }
        (answer, _) => answer,
    }
}

/// Whether `err`, from judging with the model `mapped` of `[hooks.models]`, is the model's
/// failure, which the session's model `session_model` may judge in its place.
fn falls_back_to_session_model(err: &anyhow::Error, mapped: &str, session_model: &str) -> bool {
    err.downcast_ref::<ModelFailed>().is_some() && mapped != session_model
}

/// The model could not be used: the hook's session would not start with it, its request failed,
/// or it gave no answer. Such a failure of a model of `[hooks.models]` is tried again with the
/// session's model.
#[derive(Debug)]
struct ModelFailed(&'static str);

impl std::fmt::Display for ModelFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

async fn judge_with(
    session: &Arc<Session>,
    turn: &Arc<TurnContext>,
    request: &HookJudgeRequest,
    model: Option<&str>,
) -> anyhow::Result<String> {
    if request.agentic {
        judge_with_tools(session, turn, request.prompt.clone(), model).await
    } else {
        judge_once(session, turn, request.prompt.clone(), model).await
    }
}

/// A `prompt` hook: one request, no tools, nothing recorded in any history.
async fn judge_once(
    session: &Arc<Session>,
    turn: &Arc<TurnContext>,
    prompt: String,
    model: Option<&str>,
) -> anyhow::Result<String> {
    let model_info = match model {
        Some(model) => Arc::new(
            session
                .services
                .models_manager
                .get_model_info(model, &turn.config.to_models_manager_config())
                .await,
        ),
        None => Arc::clone(turn.model_info()),
    };
    let request = Arc::new(Prompt {
        input: vec![ResponseItem::Message {
            encrypted_content: None,
            status: None,
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText { text: prompt }],
            phase: None,
            internal_chat_message_metadata_passthrough: None,
        }],
        base_instructions: BaseInstructions {
            text: PROMPT_INSTRUCTIONS.to_string(),
            provenance: Some(BaseInstructionsProvenance::Custom),
        },
        output_schema: Some(verdict_schema()),
        output_schema_strict: true,
        ..Default::default()
    });
    let effort = model_info
        .supported_reasoning_levels
        .iter()
        .any(|preset| preset.effort == ReasoningEffort::Low)
        .then_some(ReasoningEffort::Low);
    let metadata = session
        .responses_metadata_for_turn(turn, CodexResponsesRequestKind::Turn)
        .await;
    let mut client = session.services.model_client.new_session();
    let mut stream = client
        .stream(
            &request,
            &model_info,
            &turn.session_telemetry,
            effort,
            turn.reasoning_summary(),
            turn.config.service_tier.clone(),
            &metadata,
            &InferenceTraceContext::disabled(),
        )
        .await
        .context(ModelFailed("the request to the model failed"))?;
    let mut text = String::new();
    while let Some(event) = stream.next().await {
        match event.context(ModelFailed("the model's answer was cut short"))? {
            ResponseEvent::OutputTextDelta(delta) => text.push_str(&delta),
            ResponseEvent::OutputItemDone(ResponseItem::Message { role, content, .. })
                if text.is_empty() && role == "assistant" =>
            {
                if let Some(item_text) = crate::compact::content_items_to_text(&content) {
                    text = item_text;
                }
            }
            ResponseEvent::Completed { .. } => break,
            _ => {}
        }
    }
    if text.trim().is_empty() {
        return Err(anyhow::Error::msg(ModelFailed("the model gave no answer")));
    }
    Ok(text)
}

/// An `agent` hook: a child session that may use its tools, read-only, before it answers.
async fn judge_with_tools(
    session: &Arc<Session>,
    turn: &Arc<TurnContext>,
    prompt: String,
    model: Option<&str>,
) -> anyhow::Result<String> {
    let mut config = turn.config.as_ref().clone();
    let _ = config.web_search_mode.set(WebSearchMode::Disabled);
    let _ = config.features.disable(Feature::Collab);
    let _ = config.features.disable(Feature::MultiAgentV2);
    config.base_instructions = Some(AGENT_INSTRUCTIONS.to_string());
    config.base_instructions_provenance = Some(BaseInstructionsProvenance::Custom);
    config.permissions.approval_policy = Constrained::allow_only(AskForApproval::Never);
    if let Some(read_only) = config
        .permissions
        .permission_profile()
        .intersect_with_read_only()
    {
        let _ = config.permissions.set_permission_profile(read_only);
    }
    config.model = Some(model.map_or_else(|| turn.model_info().slug.clone(), str::to_string));

    let cancellation = CancellationToken::new();
    let _cancel_on_drop = CancelOnDrop(cancellation.clone());
    let (_child, io) = run_codex_thread_one_shot(
        config,
        Arc::clone(&session.services.auth_manager),
        Arc::clone(&session.services.models_manager),
        vec![ResolvedUserInput::text(prompt)],
        Arc::clone(session),
        Arc::clone(turn),
        cancellation,
        SubAgentSource::Other(HOOK_JUDGE.to_string()),
        Some(verdict_schema()),
        /*initial_history*/ None,
    )
    .await
    .context(ModelFailed("failed to start the hook's session"))?;

    let mut tool_calls = 0usize;
    while let Ok(event) = io.rx_event.recv().await {
        match event.msg {
            EventMsg::TurnComplete(turn_complete) => {
                return turn_complete
                    .last_agent_message
                    .context(ModelFailed("the model gave no answer"));
            }
            EventMsg::TurnAborted(_) => bail!("the hook's session was interrupted"),
            // Each step of the verifier shows as the tool calls it starts. Leaving here
            // drops the guard above, which ends the session.
            EventMsg::ItemStarted(started)
                if !matches!(
                    started.item,
                    TurnItem::UserMessage(_)
                        | TurnItem::AgentMessage(_)
                        | TurnItem::Reasoning(_)
                        | TurnItem::HookPrompt(_)
                        | TurnItem::Plan(_)
                ) =>
            {
                tool_calls += 1;
                if tool_calls > MAX_AGENT_HOOK_TOOL_CALLS {
                    bail!(
                        "the verifier did not answer within {MAX_AGENT_HOOK_TOOL_CALLS} tool calls"
                    );
                }
            }
            _ => {}
        }
    }
    bail!("the hook's session ended without an answer")
}

#[cfg(test)]
#[path = "hook_judge_tests.rs"]
mod tests;
