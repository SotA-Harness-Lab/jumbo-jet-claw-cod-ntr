//! Assembles reviewer policy instructions from captured configuration and model messages.
//! Typed host fragments supply optional historical-authorization instructions unchanged.

use codex_context_fragments::ContextualUserFragment;
use codex_prompts::GuardianPolicyInstructions;
use codex_prompts::ResolvedModelMessages;
use codex_protocol::TranscriptFormat;

use crate::guardian_output_contract_prompt;

/// Default instructions used by the host's typed conversation-history fragment.
pub const GUARDIAN_CONVERSATION_HISTORY_PROMPT: &str = codex_prompt_texts::text!(
    "ext/guardian-reviewer/instructions/guardian_conversation_history_prompt.md"
);

/// Reminder supplied through the host's typed fragment on subsequent reviews.
pub const GUARDIAN_FOLLOWUP_REVIEW_REMINDER: &str = concat!(
    "Use prior reviews as context, not binding precedent. ",
    "Follow the Workspace Policy. ",
    "If the user explicitly approves a previously rejected action after being informed of the ",
    "concrete risks, set outcome to \"allow\" unless the policy explicitly disallows user ",
    "overwrites in such cases."
);

/// Composes the same base policy and optional history fragment used for every review.
pub fn reviewer_instructions(
    transcript_format: TranscriptFormat,
    tenant_policy_config: &str,
    extra_policy: Option<&str>,
    policy_template: Option<&str>,
    model_messages: ResolvedModelMessages<'_>,
    conversation_history: Option<&str>,
) -> String {
    let auto_review = model_messages.auto_review();
    let extra_policy = extra_policy.unwrap_or_default();
    let policy_template = policy_template.unwrap_or(auto_review.policy_template);
    let mut instructions = GuardianPolicyInstructions::new(
        transcript_format,
        tenant_policy_config,
        extra_policy,
        policy_template,
        guardian_output_contract_prompt(),
    )
    .render()
    .into_text()
    .unwrap_or_default();
    if let Some(conversation_history) = conversation_history {
        instructions.push('\n');
        instructions.push_str(conversation_history);
    }
    instructions
}
