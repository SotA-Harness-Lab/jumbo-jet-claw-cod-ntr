//! Resolves multi-agent role bases and mode text while preserving catalog provenance.
//! Consumers own role assembly, suppression, and effort-dependent mode selection.

use super::ResolvedMessage;
use codex_protocol::openai_models::MultiAgentMessages;

const DEFAULT_MULTI_AGENT_V2_ROOT_AGENT_USAGE_HINT_TEXT: &str = codex_prompt_texts::text!(
    "templates/model_messages/multi_agent/default_multi_agent_v2_root_agent_usage_hint_text.md"
);
const DEFAULT_MULTI_AGENT_V2_SUBAGENT_USAGE_HINT_TEXT: &str = codex_prompt_texts::text!(
    "templates/model_messages/multi_agent/default_multi_agent_v2_subagent_usage_hint_text.md"
);
const EXPLICIT_REQUEST_ONLY_MULTI_AGENT_MODE_TEXT: &str = codex_prompt_texts::text!(
    "templates/model_messages/multi_agent/explicit_request_only_multi_agent_mode_text.md"
);
const PROACTIVE_MULTI_AGENT_MODE_TEXT: &str = codex_prompt_texts::text!(
    "templates/model_messages/multi_agent/proactive_multi_agent_mode_text.md"
);

/// Model-only role bases and mode alternatives for runtime selection.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedMultiAgentMessages<'a> {
    pub root: ResolvedMessage<'a>,
    pub subagent: ResolvedMessage<'a>,
    pub explicit: ResolvedMessage<'a>,
    pub proactive: ResolvedMessage<'a>,
    /// A supplied hint overrides both effort-specific modes, including when empty.
    pub hint: Option<&'a str>,
}

impl<'a> ResolvedMultiAgentMessages<'a> {
    pub(crate) fn new(messages: Option<&'a MultiAgentMessages>) -> Self {
        let role = messages.and_then(|messages| messages.role.as_ref());
        let mode = messages.and_then(|messages| messages.mode.as_ref());
        Self {
            root: ResolvedMessage::new(
                role.and_then(|role| role.root.as_deref()),
                DEFAULT_MULTI_AGENT_V2_ROOT_AGENT_USAGE_HINT_TEXT,
            ),
            subagent: ResolvedMessage::new(
                role.and_then(|role| role.subagent.as_deref()),
                DEFAULT_MULTI_AGENT_V2_SUBAGENT_USAGE_HINT_TEXT,
            ),
            explicit: ResolvedMessage::new(
                mode.and_then(|mode| mode.explicit.as_deref()),
                EXPLICIT_REQUEST_ONLY_MULTI_AGENT_MODE_TEXT,
            ),
            proactive: ResolvedMessage::new(
                mode.and_then(|mode| mode.proactive.as_deref()),
                PROACTIVE_MULTI_AGENT_MODE_TEXT,
            ),
            hint: mode.and_then(|mode| mode.hint_text.as_deref()),
        }
    }
}
