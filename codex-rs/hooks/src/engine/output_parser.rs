// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: follow more of Claude Code's behaviour

#[derive(Debug, Clone)]
pub(crate) struct UniversalOutput {
    pub continue_processing: bool,
    pub stop_reason: Option<String>,
    pub suppress_output: bool,
    pub system_message: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct SessionStartOutput {
    pub universal: UniversalOutput,
    pub additional_context: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PreToolUseOutput {
    pub universal: UniversalOutput,
    pub block_reason: Option<String>,
    pub additional_context: Option<String>,
    pub updated_input: Option<serde_json::Value>,
    /// What the hook decided about asking the user, short of blocking.
    pub permission: Option<crate::PreToolUsePermission>,
    pub invalid_reason: Option<String>,
}

/// Claude Code's reason for a block that gives none.
const DEFAULT_BLOCK_REASON: &str = "Blocked by hook";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PermissionRequestDecision {
    Allow,
    Deny { message: String },
}

#[derive(Debug, Clone)]
pub(crate) struct PermissionRequestOutput {
    pub universal: UniversalOutput,
    pub decision: Option<PermissionRequestDecision>,
    pub invalid_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PostToolUseOutput {
    pub universal: UniversalOutput,
    pub should_block: bool,
    pub reason: Option<String>,
    pub invalid_block_reason: Option<String>,
    pub additional_context: Option<String>,
    pub invalid_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct UserPromptSubmitOutput {
    pub universal: UniversalOutput,
    pub should_block: bool,
    pub reason: Option<String>,
    pub invalid_block_reason: Option<String>,
    pub additional_context: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct StopOutput {
    pub universal: UniversalOutput,
    pub should_block: bool,
    pub reason: Option<String>,
    pub invalid_block_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct StatelessHookOutput {
    pub universal: UniversalOutput,
    pub invalid_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct InterruptOutput {
    pub system_message: Option<String>,
}

use crate::schema::BlockDecisionWire;
use crate::schema::HookUniversalOutputWire;
use crate::schema::InterruptCommandOutputWire;
use crate::schema::PermissionRequestBehaviorWire;
use crate::schema::PermissionRequestCommandOutputWire;
use crate::schema::PermissionRequestDecisionWire;
use crate::schema::PostCompactCommandOutputWire;
use crate::schema::PostToolUseCommandOutputWire;
use crate::schema::PreCompactCommandOutputWire;
use crate::schema::PreToolUseCommandOutputWire;
use crate::schema::PreToolUseDecisionWire;
use crate::schema::PreToolUsePermissionDecisionWire;
use crate::schema::SessionStartCommandOutputWire;
use crate::schema::StopCommandOutputWire;
use crate::schema::SubagentStartCommandOutputWire;
use crate::schema::SubagentStopCommandOutputWire;
use crate::schema::UserPromptSubmitCommandOutputWire;

pub(crate) fn parse_session_start(stdout: &str) -> Option<SessionStartOutput> {
    let wire: SessionStartCommandOutputWire = parse_json(stdout)?;
    Some(session_start_output(
        wire.universal,
        wire.hook_specific_output
            .and_then(|output| output.additional_context),
    ))
}

pub(crate) fn parse_subagent_start(stdout: &str) -> Option<SessionStartOutput> {
    let wire: SubagentStartCommandOutputWire = parse_json(stdout)?;
    Some(session_start_output(
        wire.universal,
        wire.hook_specific_output
            .and_then(|output| output.additional_context),
    ))
}

fn session_start_output(
    universal: HookUniversalOutputWire,
    additional_context: Option<String>,
) -> SessionStartOutput {
    SessionStartOutput {
        universal: UniversalOutput::from(universal),
        additional_context,
    }
}

pub(crate) fn parse_pre_tool_use(stdout: &str) -> Option<PreToolUseOutput> {
    let PreToolUseCommandOutputWire {
        universal: universal_wire,
        decision,
        reason,
        hook_specific_output,
    } = parse_json(stdout)?;
    let universal = UniversalOutput::from(universal_wire);
    let hook_specific_output = hook_specific_output.as_ref();
    let additional_context =
        hook_specific_output.and_then(|output| output.additional_context.clone());
    let invalid_reason = unsupported_pre_tool_use_universal(&universal);

    // As in Claude Code: the top-level `decision` first, then the more specific
    // `permissionDecision`, which wins; a block without a reason still blocks.
    let mut block_reason = None;
    let mut permission = None;
    match decision {
        Some(PreToolUseDecisionWire::Block) => {
            block_reason = Some(reason_or_default(reason.as_deref()));
        }
        Some(PreToolUseDecisionWire::Approve) => {
            permission = Some(crate::PreToolUsePermission::Allow)
        }
        None => {}
    }
    if let Some(output) = hook_specific_output
        && let Some(permission_decision) = &output.permission_decision
    {
        let specific_reason = output
            .permission_decision_reason
            .as_deref()
            .and_then(trimmed_reason);
        match permission_decision {
            PreToolUsePermissionDecisionWire::Deny => {
                permission = None;
                block_reason =
                    Some(specific_reason.unwrap_or_else(|| reason_or_default(reason.as_deref())));
            }
            PreToolUsePermissionDecisionWire::Ask => {
                block_reason = None;
                permission = Some(crate::PreToolUsePermission::Ask {
                    reason: specific_reason.or_else(|| reason.as_deref().and_then(trimmed_reason)),
                });
            }
            PreToolUsePermissionDecisionWire::Allow => {
                block_reason = None;
                permission = Some(crate::PreToolUsePermission::Allow);
            }
        }
    }
    let (block_reason, permission, updated_input) = if invalid_reason.is_none() {
        let updated_input = if block_reason.is_none() {
            hook_specific_output.and_then(|output| output.updated_input.clone())
        } else {
            None
        };
        (block_reason, permission, updated_input)
    } else {
        // `continue: false` and `stopReason` are not honoured here, but a block given with
        // them still refuses the call, as in Claude Code.
        (block_reason, None, None)
    };

    Some(PreToolUseOutput {
        universal,
        block_reason,
        additional_context,
        updated_input,
        permission,
        invalid_reason,
    })
}

pub(crate) fn parse_permission_request(stdout: &str) -> Option<PermissionRequestOutput> {
    let wire: PermissionRequestCommandOutputWire = parse_json(stdout)?;
    let universal = UniversalOutput::from(wire.universal);
    let hook_specific_output = wire.hook_specific_output.as_ref();
    let decision = hook_specific_output.and_then(|output| output.decision.as_ref());
    let invalid_reason = unsupported_permission_request_universal(&universal).or_else(|| {
        hook_specific_output.and_then(|output| {
            unsupported_permission_request_hook_specific_output(output.decision.as_ref())
        })
    });
    let decision = if invalid_reason.is_none() {
        decision.map(permission_request_decision)
    } else {
        // What Codex does not honour beside it does not undo a denial.
        decision
            .filter(|decision| matches!(decision.behavior, PermissionRequestBehaviorWire::Deny))
            .map(permission_request_decision)
    };

    Some(PermissionRequestOutput {
        universal,
        decision,
        invalid_reason,
    })
}

pub(crate) fn parse_post_tool_use(stdout: &str) -> Option<PostToolUseOutput> {
    let wire: PostToolUseCommandOutputWire = parse_json(stdout)?;
    let universal = UniversalOutput::from(wire.universal);
    let invalid_reason = unsupported_post_tool_use_universal(&universal).or_else(|| {
        wire.hook_specific_output
            .as_ref()
            .and_then(unsupported_post_tool_use_hook_specific_output)
    });
    let should_block = matches!(wire.decision, Some(BlockDecisionWire::Block));
    let reason = should_block.then(|| reason_or_default(wire.reason.as_deref()));
    let invalid_block_reason = None;
    let additional_context = wire
        .hook_specific_output
        .and_then(|output| output.additional_context);

    Some(PostToolUseOutput {
        universal,
        should_block: should_block && invalid_reason.is_none(),
        reason: reason.or(wire.reason),
        invalid_block_reason,
        additional_context,
        invalid_reason,
    })
}

pub(crate) fn parse_pre_compact(stdout: &str) -> Option<StatelessHookOutput> {
    let wire: PreCompactCommandOutputWire = parse_json(stdout)?;
    let universal = UniversalOutput::from(wire.universal);
    Some(StatelessHookOutput {
        universal,
        invalid_reason: None,
    })
}

pub(crate) fn parse_post_compact(stdout: &str) -> Option<StatelessHookOutput> {
    let wire: PostCompactCommandOutputWire = parse_json(stdout)?;
    let universal = UniversalOutput::from(wire.universal);
    Some(StatelessHookOutput {
        universal,
        invalid_reason: None,
    })
}

pub(crate) fn parse_interrupt(stdout: &str) -> Option<InterruptOutput> {
    let wire: InterruptCommandOutputWire = parse_json(stdout)?;
    Some(InterruptOutput {
        system_message: wire.system_message,
    })
}

pub(crate) fn parse_user_prompt_submit(stdout: &str) -> Option<UserPromptSubmitOutput> {
    let wire: UserPromptSubmitCommandOutputWire = parse_json(stdout)?;
    let should_block = matches!(wire.decision, Some(BlockDecisionWire::Block));
    let invalid_block_reason = None;
    let additional_context = wire
        .hook_specific_output
        .and_then(|output| output.additional_context);
    Some(UserPromptSubmitOutput {
        universal: UniversalOutput::from(wire.universal),
        should_block,
        reason: if should_block {
            Some(reason_or_default(wire.reason.as_deref()))
        } else {
            wire.reason
        },
        invalid_block_reason,
        additional_context,
    })
}

pub(crate) fn parse_stop(stdout: &str) -> Option<StopOutput> {
    let wire: StopCommandOutputWire = parse_json(stdout)?;
    Some(stop_output(wire.universal, wire.decision, wire.reason))
}

pub(crate) fn parse_subagent_stop(stdout: &str) -> Option<StopOutput> {
    let wire: SubagentStopCommandOutputWire = parse_json(stdout)?;
    Some(stop_output(wire.universal, wire.decision, wire.reason))
}

fn stop_output(
    universal: HookUniversalOutputWire,
    decision: Option<BlockDecisionWire>,
    reason: Option<String>,
) -> StopOutput {
    let should_block = matches!(decision, Some(BlockDecisionWire::Block));
    StopOutput {
        universal: UniversalOutput::from(universal),
        should_block,
        reason: if should_block {
            Some(reason_or_default(reason.as_deref()))
        } else {
            reason
        },
        invalid_block_reason: None,
    }
}

impl From<HookUniversalOutputWire> for UniversalOutput {
    fn from(value: HookUniversalOutputWire) -> Self {
        Self {
            continue_processing: value.r#continue,
            stop_reason: value.stop_reason,
            suppress_output: value.suppress_output,
            system_message: value.system_message,
        }
    }
}

fn parse_json<T>(stdout: &str) -> Option<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(trimmed).ok()?;
    if !value.is_object() {
        return None;
    }
    serde_json::from_value(value).ok()
}

pub(crate) fn looks_like_json(stdout: &str) -> bool {
    let trimmed = stdout.trim_start();
    trimmed.starts_with('{') || trimmed.starts_with('[')
}

fn unsupported_pre_tool_use_universal(universal: &UniversalOutput) -> Option<String> {
    // `suppressOutput` only hides the hook's output from a transcript view.
    if !universal.continue_processing {
        Some("PreToolUse hook returned unsupported continue:false".to_string())
    } else if universal.stop_reason.is_some() {
        Some("PreToolUse hook returned unsupported stopReason".to_string())
    } else {
        None
    }
}

fn unsupported_permission_request_universal(universal: &UniversalOutput) -> Option<String> {
    if !universal.continue_processing {
        Some("PermissionRequest hook returned unsupported continue:false".to_string())
    } else if universal.stop_reason.is_some() {
        Some("PermissionRequest hook returned unsupported stopReason".to_string())
    } else {
        None
    }
}

fn unsupported_post_tool_use_universal(_universal: &UniversalOutput) -> Option<String> {
    None
}

fn unsupported_permission_request_hook_specific_output(
    decision: Option<&PermissionRequestDecisionWire>,
) -> Option<String> {
    let decision = decision?;
    if decision.updated_input.is_some() {
        Some("PermissionRequest hook returned unsupported updatedInput".to_string())
    } else if decision.updated_permissions.is_some() {
        Some("PermissionRequest hook returned unsupported updatedPermissions".to_string())
    } else if decision.interrupt {
        Some("PermissionRequest hook returned unsupported interrupt:true".to_string())
    } else {
        None
    }
}

fn permission_request_decision(
    decision: &PermissionRequestDecisionWire,
) -> PermissionRequestDecision {
    match decision.behavior {
        PermissionRequestBehaviorWire::Allow => PermissionRequestDecision::Allow,
        PermissionRequestBehaviorWire::Deny => PermissionRequestDecision::Deny {
            message: decision
                .message
                .as_deref()
                .and_then(trimmed_reason)
                .unwrap_or_else(|| "PermissionRequest hook denied approval".to_string()),
        },
    }
}

fn unsupported_post_tool_use_hook_specific_output(
    output: &crate::schema::PostToolUseHookSpecificOutputWire,
) -> Option<String> {
    if output.updated_mcp_tool_output.is_some() {
        Some("PostToolUse hook returned unsupported updatedMCPToolOutput".to_string())
    } else {
        None
    }
}

fn reason_or_default(reason: Option<&str>) -> String {
    reason
        .and_then(trimmed_reason)
        .unwrap_or_else(|| DEFAULT_BLOCK_REASON.to_string())
}

fn trimmed_reason(reason: &str) -> Option<String> {
    let trimmed = reason.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use super::parse_permission_request;
    use super::parse_pre_tool_use;
    use super::parse_user_prompt_submit;

    #[test]
    fn structured_output_rejects_invalid_shapes_and_types() {
        for stdout in [
            "[]",
            r#"{"systemMessage":123}"#,
            r#"{"hookSpecificOutput":{"hookEventName":"UserPromptSubmit","additionalContext":123}}"#,
            r#"{"hookSpecificOutput":{"additionalContext":"missing event name"}}"#,
            "{",
        ] {
            assert!(
                parse_user_prompt_submit(stdout).is_none(),
                "invalid structured output should fail: {stdout}",
            );
        }
    }

    #[test]
    fn permission_request_rejects_reserved_updated_input_field() {
        let parsed = parse_permission_request(
            &json!({
                "continue": true,
                "hookSpecificOutput": {
                    "hookEventName": "PermissionRequest",
                    "decision": {
                        "behavior": "allow",
                        "updatedInput": {}
                    }
                }
            })
            .to_string(),
        )
        .expect("permission request hook output should parse");

        assert_eq!(
            parsed.invalid_reason,
            Some("PermissionRequest hook returned unsupported updatedInput".to_string())
        );
    }

    #[test]
    fn permission_request_rejects_reserved_updated_permissions_field() {
        let parsed = parse_permission_request(
            &json!({
                "continue": true,
                "hookSpecificOutput": {
                    "hookEventName": "PermissionRequest",
                    "decision": {
                        "behavior": "allow",
                        "updatedPermissions": {}
                    }
                }
            })
            .to_string(),
        )
        .expect("permission request hook output should parse");

        assert_eq!(
            parsed.invalid_reason,
            Some("PermissionRequest hook returned unsupported updatedPermissions".to_string())
        );
    }

    #[test]
    fn permission_request_rejects_reserved_interrupt_field() {
        let parsed = parse_permission_request(
            &json!({
                "continue": true,
                "hookSpecificOutput": {
                    "hookEventName": "PermissionRequest",
                    "decision": {
                        "behavior": "allow",
                        "interrupt": true
                    }
                }
            })
            .to_string(),
        )
        .expect("permission request hook output should parse");

        assert_eq!(
            parsed.invalid_reason,
            Some("PermissionRequest hook returned unsupported interrupt:true".to_string())
        );
    }

    #[test]
    fn a_pre_tool_use_block_stands_beside_continue_false() {
        for output in [
            json!({
                "continue": false,
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "no secrets"
                }
            }),
            json!({"continue": false, "decision": "block", "reason": "no secrets"}),
        ] {
            let parsed = parse_pre_tool_use(&output.to_string()).expect("output should parse");
            assert_eq!(
                (parsed.block_reason, parsed.invalid_reason),
                (
                    Some("no secrets".to_string()),
                    Some("PreToolUse hook returned unsupported continue:false".to_string())
                ),
                "{output}"
            );
        }

        let allowed = parse_pre_tool_use(
            &json!({
                "stopReason": "done",
                "hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "allow"}
            })
            .to_string(),
        )
        .expect("output should parse");
        assert_eq!(
            (allowed.block_reason, allowed.permission.is_none()),
            (None, true)
        );
    }

    #[test]
    fn a_permission_request_denial_stands_beside_interrupt() {
        let parsed = parse_permission_request(
            &json!({
                "hookSpecificOutput": {
                    "hookEventName": "PermissionRequest",
                    "decision": {"behavior": "deny", "message": "no", "interrupt": true}
                }
            })
            .to_string(),
        )
        .expect("permission request hook output should parse");

        assert_eq!(
            parsed.decision,
            Some(super::PermissionRequestDecision::Deny {
                message: "no".to_string()
            })
        );
    }
}
