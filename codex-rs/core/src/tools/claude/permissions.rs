// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's permission rules (`permissions.allow`, `ask`, `deny` of the user's, the
//! project's and the local settings files) for the Claude tools.
//!
//! The rules are matched in `codex_claude_compat::PermissionRules`, as Claude Code matches
//! them; what they decide is handed to Codex's own approval path, not to a second one:
//!
//! - Deny: the tool is not run; the model gets Claude Code's denial.
//! - Allow: the call is recorded, and when Codex's orchestrator decides the call's approval
//!   (`exec_command` for `Bash`, `apply_patch` for `Write` and `Edit`), a call Codex would
//!   ask about runs without asking, still in the sandbox. MCP calls skip their approval.
//! - Ask: for a tool Codex asks about, the orchestrator asks even where Codex would not,
//!   with the rule as the reason and no approval remembered for later calls. A tool Codex
//!   never asks about (`Read`, `Glob`, `Grep`, `WebFetch`, MCP tools, ...) asks here, with
//!   Codex's approval prompt. When the approval policy cannot ask (`never`), it is denied,
//!   as Claude Code denies what it cannot ask about in a non-interactive session.
//! - No rule: Codex decides as for its own tools.
//!
//! A Claude tool calls [`check_permission`] before it delegates, and keeps the returned
//! guard until the delegated call has finished.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

use codex_claude_compat::PermissionRules;
use codex_claude_compat::RuleVerdict;
use codex_claude_compat::ToolInput;
use codex_protocol::approvals::ExecApprovalKind;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::ReviewDecision;
use codex_utils_path_uri::PathUri;

use super::failed;
use crate::exec_policy::prompt_is_rejected_by_policy;
use crate::function_tool::FunctionCallError;
use crate::guardian::GuardianReviewContext;
use crate::sandboxing::SandboxPermissions;
use crate::session::session::Session;
use crate::tools::context::ToolInvocation;
use crate::tools::handlers::selected_executor;
use crate::tools::sandboxing::ExecApprovalRequirement;

/// Claude Code's answer to the model when the user declines a tool call.
const REJECT_MESSAGE: &str = "The user doesn't want to proceed with this tool use. The tool use was rejected (eg. if it was a file edit, the new_string was NOT written to the file). STOP what you are doing and wait for the user to tell you how to proceed.";

/// What a Claude tool call is checked on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum PermissionInput<'a> {
    /// Only the tool's name: `TodoWrite`, `WebSearch`, `AskUserQuestion`, an MCP tool.
    Tool,
    /// The command of `Bash`. Codex's `exec_command` asks about it.
    Command(&'a str),
    /// A file or directory read, as an absolute path: `Read`, `Glob`, `Grep`.
    ReadPath(&'a str),
    /// A file written, as an absolute path: `Write`, `Edit`. Codex's `apply_patch` asks
    /// about it.
    EditPath(&'a str),
    /// The URL of `WebFetch`.
    Url(&'a str),
    /// Content a rule names exactly, such as the subagent type of `Agent`.
    Content(&'a str),
}

impl PermissionInput<'_> {
    /// Whether the Codex tool the call is delegated to has its approval decided by Codex's
    /// orchestrator, which then asks for an `Ask` rule.
    fn codex_asks(self) -> bool {
        matches!(self, Self::Command(_) | Self::EditPath(_))
    }
}

/// What a rule decided for a call, for the Codex runtime it is delegated to.
#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleDecision {
    Allow,
    Ask { reason: String },
}

/// The rules of each project directory of the session, read once.
#[derive(Default)]
struct SessionRules(Mutex<HashMap<PathBuf, Arc<PermissionRules>>>);

/// Decisions of the calls in flight, by call id.
#[derive(Default)]
struct CallDecisions(Mutex<HashMap<String, RuleDecision>>);

impl CallDecisions {
    fn of(session: &Session) -> Arc<Self> {
        session
            .services
            .session_extension_data
            .get_or_init::<CallDecisions>(CallDecisions::default)
    }

    fn get(&self, call_id: &str) -> Option<RuleDecision> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(call_id)
            .cloned()
    }
}

/// Keeps a call's decision for Codex's runtimes until the call is over.
#[must_use]
pub(crate) struct PermissionGuard {
    session: Arc<Session>,
    call_id: Option<String>,
}

impl PermissionGuard {
    fn record(&mut self, call_id: &str, decision: RuleDecision) {
        CallDecisions::of(&self.session)
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(call_id.to_string(), decision);
        self.call_id = Some(call_id.to_string());
    }
}

impl Drop for PermissionGuard {
    fn drop(&mut self) {
        if let Some(call_id) = self.call_id.take() {
            CallDecisions::of(&self.session)
                .0
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .remove(&call_id);
        }
    }
}

/// The rules for the session's project directory.
fn rules_for(session: &Session, project_dir: &Path) -> Arc<PermissionRules> {
    let cache = session
        .services
        .session_extension_data
        .get_or_init::<SessionRules>(SessionRules::default);
    let mut cache = cache.0.lock().unwrap_or_else(PoisonError::into_inner);
    let rules = cache.entry(project_dir.to_path_buf()).or_insert_with(|| {
        // Unit tests never read the real home directory's Claude Code settings.
        if cfg!(test) {
            return Arc::new(PermissionRules::default());
        }
        let home = dirs::home_dir();
        let (rules, errors) = PermissionRules::load(home.as_deref(), Some(project_dir));
        for (path, err) in errors {
            tracing::warn!(
                "failed to read Claude Code settings {}: {err}",
                path.display()
            );
        }
        Arc::new(rules)
    });
    Arc::clone(rules)
}

/// The working directory of the call: its executor's, or the session's when the call has
/// no single executor (an MCP tool).
fn call_directory(invocation: &ToolInvocation) -> (PathUri, Option<String>) {
    match selected_executor(
        &invocation.environments,
        &invocation.step_context.environments,
    ) {
        Ok(environment) => (
            environment.cwd().clone(),
            Some(environment.selection.environment_id.clone()),
        ),
        Err(_) => (PathUri::from_abs_path(&invocation.turn.config.cwd), None),
    }
}

/// Checks a Claude tool call against Claude Code's permission rules before the tool runs.
/// A denied call, or one the user declines, is an error with Claude Code's text for the
/// model; otherwise the returned guard carries the decision to Codex's runtimes and must
/// be held until the delegated call has finished.
pub(crate) async fn check_permission(
    invocation: &ToolInvocation,
    tool: &str,
    input: PermissionInput<'_>,
) -> Result<PermissionGuard, FunctionCallError> {
    let mut guard = PermissionGuard {
        session: Arc::clone(&invocation.session),
        call_id: None,
    };
    if !super::enabled(&invocation.turn) {
        return Ok(guard);
    }
    let rules = rules_for(&invocation.session, invocation.turn.config.cwd.as_path());
    if rules.is_empty() {
        return Ok(guard);
    }
    let (cwd_uri, environment_id) = call_directory(invocation);
    let cwd = PathBuf::from(cwd_uri.inferred_native_path_string());
    let paths = match input {
        PermissionInput::ReadPath(path) | PermissionInput::EditPath(path) => {
            vec![cwd.join(path)]
        }
        _ => Vec::new(),
    };
    let tool_input = match input {
        PermissionInput::Tool => ToolInput::Tool,
        PermissionInput::Command(command) => ToolInput::Command { command, cwd: &cwd },
        PermissionInput::ReadPath(_) => ToolInput::Read {
            paths: &paths,
            cwd: &cwd,
        },
        PermissionInput::EditPath(_) => ToolInput::Edit {
            paths: &paths,
            cwd: &cwd,
        },
        PermissionInput::Url(url) => ToolInput::Url(url),
        PermissionInput::Content(content) => ToolInput::Content(content),
    };

    match rules.evaluate(tool, tool_input) {
        RuleVerdict::NoMatch => {}
        RuleVerdict::Deny { message, .. } => return Err(failed(message)),
        RuleVerdict::Allow { .. } => guard.record(&invocation.call_id, RuleDecision::Allow),
        RuleVerdict::Ask { rule } => {
            let reason = format!("Claude Code permission rule `{rule}` asks before this call.");
            if input.codex_asks() {
                guard.record(&invocation.call_id, RuleDecision::Ask { reason });
            } else {
                ask_user(invocation, tool, input, reason, cwd_uri, environment_id).await?;
                // The user said yes: Codex does not ask again for this call.
                guard.record(&invocation.call_id, RuleDecision::Allow);
            }
        }
    }
    Ok(guard)
}

/// Asks the user about a call of a tool Codex has no approval step for, with Codex's
/// command approval prompt showing the tool and its input.
async fn ask_user(
    invocation: &ToolInvocation,
    tool: &str,
    input: PermissionInput<'_>,
    reason: String,
    cwd: PathUri,
    environment_id: Option<String>,
) -> Result<(), FunctionCallError> {
    if prompt_is_rejected_by_policy(
        invocation.step_context.settings.approval_policy(),
        /*prompt_is_rule*/ true,
    )
    .is_some()
    {
        return Err(failed(format!(
            "Permission to use {tool} has been denied: a Claude Code permission rule asks \
             before this call, and this session cannot ask."
        )));
    }
    let mut shown = vec![tool.to_string()];
    match input {
        PermissionInput::Tool => {}
        PermissionInput::Command(text)
        | PermissionInput::ReadPath(text)
        | PermissionInput::EditPath(text)
        | PermissionInput::Url(text)
        | PermissionInput::Content(text) => shown.push(text.to_string()),
    }
    let review = GuardianReviewContext::from(&invocation.step_context);
    let decision = invocation
        .session
        .request_command_approval(
            &invocation.step_context.turn,
            ExecApprovalKind::Command,
            review.model_context(),
            invocation.call_id.clone(),
            Some(&invocation.call_id),
            /*approval_id*/ None,
            environment_id,
            shown,
            cwd,
            Some(reason),
            /*network_approval*/ None,
            /*proposed_execpolicy_amendment*/ None,
            /*additional_permissions*/ None,
            Some(vec![
                ReviewDecision::Approved,
                ReviewDecision::denied("rejected by user"),
                ReviewDecision::Abort,
            ]),
            /*plugin_attribution_override*/ None,
        )
        .await;
    match decision {
        ReviewDecision::Approved | ReviewDecision::ApprovedForSession => Ok(()),
        _ => Err(failed(REJECT_MESSAGE)),
    }
}

/// Whether a Claude Code allow rule (or the user, for an ask rule) already approved the
/// call `call_id`, so that Codex does not ask about it again.
pub(crate) fn rule_allows(session: &Session, call_id: &str) -> bool {
    CallDecisions::of(session).get(call_id) == Some(RuleDecision::Allow)
}

/// Codex's approval requirement for a call, after the Claude Code rule recorded for it:
/// an allow rule runs without asking what Codex would ask about (in the sandbox; a request
/// to leave the sandbox, a managed rule that wants a person, and a forbidden command keep
/// Codex's requirement), an ask rule asks even where Codex would not.
pub(crate) fn apply_rule_decision(
    session: &Session,
    call_id: &str,
    approval_policy: AskForApproval,
    sandbox_permissions: SandboxPermissions,
    requirement: ExecApprovalRequirement,
) -> ExecApprovalRequirement {
    let Some(decision) = CallDecisions::of(session).get(call_id) else {
        return requirement;
    };
    match (decision, requirement) {
        (_, requirement @ ExecApprovalRequirement::Forbidden { .. })
        | (_, requirement @ ExecApprovalRequirement::NeedsHumanApproval { .. }) => requirement,
        (RuleDecision::Allow, ExecApprovalRequirement::NeedsApproval { .. })
            if !sandbox_permissions.requires_escalated_permissions() =>
        {
            ExecApprovalRequirement::Skip {
                bypass_sandbox: false,
                proposed_execpolicy_amendment: None,
            }
        }
        (RuleDecision::Allow, requirement) => requirement,
        (RuleDecision::Ask { reason }, _) => {
            match prompt_is_rejected_by_policy(approval_policy, /*prompt_is_rule*/ true) {
                Some(rejection) => ExecApprovalRequirement::Forbidden {
                    reason: format!("{reason} {rejection}"),
                },
                None => ExecApprovalRequirement::NeedsApproval {
                    reason: Some(reason),
                    proposed_execpolicy_amendment: None,
                },
            }
        }
    }
}
