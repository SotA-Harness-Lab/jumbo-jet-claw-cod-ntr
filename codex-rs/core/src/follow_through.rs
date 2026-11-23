// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Keeps a turn going when it is about to end with its work left undone.
//!
//! A turn ends as soon as a model request brings no tool call. Models that narrate their work
//! often end one that way in the middle of the job: a message saying what they will do next,
//! with nothing after it, or a progress report while steps of their own plan are still open.
//! The user then finds half the work done and a promise of the rest. Before such a turn ends,
//! this module looks for three signs, cheapest first, and when one is there it tells the model
//! why and lets it go on, at most `[harness] continue_unfinished` times in a row:
//!
//! - the last message was commentary (the model's own label for text that tools follow), and
//!   no tool call followed it;
//! - the plan the model keeps with `update_plan` or `TodoWrite`, updated in this turn, still
//!   has steps that are not completed (asked once for each version of the plan);
//! - a model reading the user's request and the last answer finds work left that the agent
//!   could do itself (`[harness] unfinished_judge`), in a turn that used tools.
//!
//! A tool call in between starts the count in a row again; four times the limit in all ends
//! the turn whatever is left.
//!
//! The model's instructions also tell it to carry on to the end ([`GUIDANCE`]). None of it
//! runs for the harness's own sessions (memory consolidation, reviews, compaction, hooks),
//! nor in Plan mode, where a turn is meant to end with a plan.

use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use codex_protocol::config_types::ModeKind;
use codex_protocol::items::TurnItem;
use codex_protocol::plan_tool::StepStatus;
use codex_protocol::plan_tool::UpdatePlanArgs;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use codex_protocol::protocol::WarningEvent;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

use crate::context::ContextualUserFragment;
use crate::context::UnfinishedWorkContinuation;
use crate::session::session::Session;
use crate::session::step_context::StepContext;
use crate::session::turn_context::TurnContext;

/// How many times in a row a turn is sent back to work, where `[harness] continue_unfinished`
/// does not say.
const DEFAULT_CONTINUATIONS: u32 = 3;

/// How many times one turn may be sent back to work in all, as a multiple of the limit in a
/// row: a model that does a little each time and stops again could otherwise go on without
/// end.
const TOTAL_CONTINUATIONS_PER_LIMIT: u32 = 4;

/// How long the judge may take before the turn ends as it would have.
const JUDGE_TIMEOUT: Duration = Duration::from_secs(60);

/// How much of the user's request and of the last answer the judge reads, from their ends.
const JUDGE_TEXT_LIMIT: usize = 6_000;

/// Added to the model's developer instructions while this is on.
pub(crate) const GUIDANCE: &str = codex_prompt_texts::text!("core/follow_through/guidance.md");

const JUDGE_INSTRUCTIONS: &str =
    codex_prompt_texts::text!("core/follow_through/judge_instructions.md");

/// The answer the judge must give.
fn judge_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "verdict": { "type": "string", "enum": ["done", "needs_user", "unfinished"] },
            "remaining": { "type": ["string", "null"] },
        },
        "required": ["verdict", "remaining"],
        "additionalProperties": false,
    })
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct JudgeVerdict {
    verdict: String,
    remaining: Option<String>,
}

/// How many times in a row this turn may be sent back to work.
pub(crate) fn continuation_limit(turn: &TurnContext) -> u32 {
    turn.config
        .mod_config
        .harness
        .continue_unfinished
        .unwrap_or(if crate::config::refinements_default() {
            DEFAULT_CONTINUATIONS
        } else {
            0
        })
}

/// Whether a session from `source` does the user's work, so that stopping halfway matters:
/// the user's own sessions and the subagents they start, not the harness's.
pub(crate) fn applies_to(source: &SessionSource) -> bool {
    match source {
        SessionSource::Cli
        | SessionSource::VSCode
        | SessionSource::Exec
        | SessionSource::Mcp
        | SessionSource::Custom(_)
        | SessionSource::SubAgent(SubAgentSource::ThreadSpawn { .. }) => true,
        SessionSource::Internal(_) | SessionSource::SubAgent(_) | SessionSource::Unknown => false,
    }
}

/// Whether the model of a session from `source` is told [`GUIDANCE`].
pub(crate) fn gives_guidance(turn: &TurnContext, source: &SessionSource) -> bool {
    applies_to(source) && continuation_limit(turn) > 0
}

/// The plan the model last gave this thread, kept in the thread's extension data.
#[derive(Default)]
pub(crate) struct LatestPlan(Mutex<PlanState>);

#[derive(Default)]
struct PlanState {
    /// The turn in which the plan was last updated.
    turn_id: String,
    /// Goes up with each update, so that one version of the plan is asked about once.
    version: u64,
    /// The steps not completed, in the plan's order.
    unfinished: Vec<String>,
    /// The version the model was last sent back to work for.
    continued_version: Option<u64>,
}

/// Keeps the plan an `update_plan` or `TodoWrite` call just set.
pub(crate) fn record_plan(sess: &Session, turn: &TurnContext, args: &UpdatePlanArgs) {
    let plan = sess
        .services
        .thread_extension_data
        .get_or_init(LatestPlan::default);
    let mut state = plan
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    state.turn_id.clone_from(&turn.sub_id);
    state.version += 1;
    state.unfinished = unfinished_steps(args);
}

fn unfinished_steps(args: &UpdatePlanArgs) -> Vec<String> {
    args.plan
        .iter()
        .filter(|item| !matches!(item.status, StepStatus::Completed))
        .map(|item| item.step.trim().to_string())
        .filter(|step| !step.is_empty())
        .collect()
}

/// The unfinished steps of a plan updated in this turn, with its version.
fn plan_left_in_turn(sess: &Session, turn: &TurnContext) -> Option<(u64, Vec<String>)> {
    let plan = sess.services.thread_extension_data.get::<LatestPlan>()?;
    let state = plan
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    (state.turn_id == turn.sub_id && !state.unfinished.is_empty())
        .then(|| (state.version, state.unfinished.clone()))
}

fn mark_plan_continued(sess: &Session, version: u64) {
    if let Some(plan) = sess.services.thread_extension_data.get::<LatestPlan>() {
        plan.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .continued_version = Some(version);
    }
}

fn plan_continued_at(sess: &Session, version: u64) -> bool {
    sess.services
        .thread_extension_data
        .get::<LatestPlan>()
        .is_some_and(|plan| {
            plan.0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .continued_version
                == Some(version)
        })
}

/// What the turn loop knows when a model request ended without a tool call.
pub(crate) struct TurnEnd<'a> {
    /// The request's last message was commentary or a partial answer, with no final answer
    /// after it.
    pub(crate) ended_on_commentary: bool,
    /// The text of that last message.
    pub(crate) last_message: Option<&'a str>,
    /// Whether any request of this turn called a tool.
    pub(crate) turn_used_tools: bool,
    /// How many times in a row this turn was already sent back to work.
    pub(crate) continued: u32,
    /// How many times this turn was sent back to work in all.
    pub(crate) continued_total: u32,
}

/// Why the work was found unfinished.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Unfinished {
    Commentary,
    Plan(Vec<String>),
    Judged(String),
}

impl Unfinished {
    /// What the model is told.
    fn reason(&self) -> String {
        match self {
            Unfinished::Commentary => "your last message said what you would do, and no tool \
                call followed it, so none of it was done."
                .to_string(),
            Unfinished::Plan(steps) => {
                let steps = steps
                    .iter()
                    .map(|step| format!("- {step}"))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    codex_prompt_texts::text!("core/follow_through/plan_steps_reason.md"),
                    steps = steps
                )
            }
            Unfinished::Judged(remaining) => format!(
                codex_prompt_texts::text!("core/follow_through/judged_reason.md"),
                remaining.trim()
            ),
        }
    }

    /// What the user is told, in a few words.
    fn summary(&self) -> &'static str {
        match self {
            Unfinished::Commentary => "it ended on what it would do next",
            Unfinished::Plan(_) => "its plan has steps left",
            Unfinished::Judged(_) => "its last answer leaves work it can do",
        }
    }
}

/// Whether the turn about to end should go on, and why.
pub(crate) async fn check(
    sess: &Arc<Session>,
    turn: &Arc<TurnContext>,
    end: TurnEnd<'_>,
) -> Option<Unfinished> {
    let limit = continuation_limit(turn);
    if end.continued >= limit
        || end.continued_total >= limit.saturating_mul(TOTAL_CONTINUATIONS_PER_LIMIT)
        || !applies_to(&turn.session_source)
        || turn.mode() == ModeKind::Plan
    {
        return None;
    }
    if end.ended_on_commentary {
        return Some(Unfinished::Commentary);
    }
    let plan = plan_left_in_turn(sess, turn);
    if let Some((version, steps)) = &plan
        && !plan_continued_at(sess, *version)
    {
        mark_plan_continued(sess, *version);
        return Some(Unfinished::Plan(steps.clone()));
    }
    let last_message = end
        .last_message
        .map(str::trim)
        .filter(|text| !text.is_empty())?;
    if !end.turn_used_tools
        || !turn
            .config
            .mod_config
            .harness
            .unfinished_judge
            .unwrap_or(true)
    {
        return None;
    }
    let plan_steps = plan.map(|(_, steps)| steps).unwrap_or_default();
    judge(sess, turn, last_message, &plan_steps)
        .await
        .map(Unfinished::Judged)
}

/// Asks the judge; `Some` with what is left when it finds the work unfinished. A judge that
/// fails or is slow lets the turn end, as it would have without it.
async fn judge(
    sess: &Arc<Session>,
    turn: &Arc<TurnContext>,
    last_message: &str,
    plan_steps: &[String],
) -> Option<String> {
    let request = latest_user_request(sess).await;
    let prompt = judge_prompt(request.as_deref(), last_message, plan_steps);
    let answer = tokio::time::timeout(
        JUDGE_TIMEOUT,
        crate::hook_judge::ask_small_model(sess, turn, JUDGE_INSTRUCTIONS, judge_schema(), prompt),
    )
    .await;
    let answer = match answer {
        Ok(Ok(answer)) => answer,
        Ok(Err(err)) => {
            tracing::warn!("could not judge whether the turn's work was finished: {err:#}");
            return None;
        }
        Err(_) => {
            tracing::warn!("judging whether the turn's work was finished took too long");
            return None;
        }
    };
    parse_verdict(&answer)
}

fn parse_verdict(answer: &str) -> Option<String> {
    let answer = answer.trim();
    let answer = answer
        .strip_prefix("```json")
        .or_else(|| answer.strip_prefix("```"))
        .and_then(|rest| rest.strip_suffix("```"))
        .unwrap_or(answer);
    let verdict = match serde_json::from_str::<JudgeVerdict>(answer.trim()) {
        Ok(verdict) => verdict,
        Err(err) => {
            tracing::warn!("the judge of unfinished work gave no verdict ({err}): {answer}");
            return None;
        }
    };
    (verdict.verdict == "unfinished").then(|| {
        verdict
            .remaining
            .map(|remaining| remaining.trim().to_string())
            .filter(|remaining| !remaining.is_empty())
            .unwrap_or_else(|| "Finish what the user asked for.".to_string())
    })
}

/// The text of the user's latest message in the thread's history.
async fn latest_user_request(sess: &Session) -> Option<String> {
    let history = sess.clone_history().await;
    history
        .raw_items()
        .rev()
        .find_map(|item| match crate::event_mapping::parse_turn_item(item)? {
            TurnItem::UserMessage(message) => {
                Some(message.message()).filter(|text| !text.trim().is_empty())
            }
            _ => None,
        })
}

fn judge_prompt(request: Option<&str>, last_message: &str, plan_steps: &[String]) -> String {
    let mut prompt = format!(
        "<user_request>\n{}\n</user_request>\n\n<agent_last_message>\n{}\n</agent_last_message>\n",
        tail(request.unwrap_or("(not available)"), JUDGE_TEXT_LIMIT),
        tail(last_message, JUDGE_TEXT_LIMIT),
    );
    if !plan_steps.is_empty() {
        prompt.push_str("\n<unfinished_plan_steps>\n");
        for step in plan_steps {
            prompt.push_str("- ");
            prompt.push_str(step);
            prompt.push('\n');
        }
        prompt.push_str("</unfinished_plan_steps>\n");
    }
    prompt
}

/// The last `limit` characters of `text`, marked where it was cut.
fn tail(text: &str, limit: usize) -> String {
    let text = text.trim();
    let count = text.chars().count();
    if count <= limit {
        return text.to_string();
    }
    let start = text
        .char_indices()
        .nth(count - limit)
        .map_or(0, |(index, _)| index);
    format!("[...]{}", &text[start..])
}

/// Tells the model why its turn goes on, and the user that it does.
pub(crate) async fn continue_turn(
    sess: &Arc<Session>,
    step_context: &Arc<StepContext>,
    unfinished: &Unfinished,
    continuation: u32,
) {
    let turn_context = &step_context.turn;
    let limit = continuation_limit(turn_context);
    let item = ContextualUserFragment::into(UnfinishedWorkContinuation::new(unfinished.reason()));
    sess.record_conversation_items(turn_context, &step_context.settings.model_info, &[item])
        .await;
    sess.send_event(
        turn_context,
        EventMsg::Warning(WarningEvent {
            message: format!(
                "The model stopped before its work was done ({}); asking it to continue \
                 ({continuation}/{limit}).",
                unfinished.summary()
            ),
        }),
    )
    .await;
}

#[cfg(test)]
#[path = "follow_through_tests.rs"]
mod tests;
