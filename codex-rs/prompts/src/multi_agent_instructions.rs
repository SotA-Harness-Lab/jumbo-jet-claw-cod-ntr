//! Assembles multi-agent role instructions from selected text and runtime capabilities.
//! The segment owns rendering and attribution; consumers select and capture its inputs.

use crate::without_update_plan_instructions;
use codex_context_fragments::ContextualUserFragment;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

const DEFAULT_MULTI_AGENT_V2_MODEL_OVERRIDE_USAGE_HINT_TEXT: &str = codex_prompt_texts::text!(
    "templates/multi_agent_instructions/default_multi_agent_v2_model_override_usage_hint_text.md"
);
const DEFAULT_MULTI_AGENT_V2_WAIT_AGENT_USAGE_HINT_TEXT: &str =
    "When calling `wait_agent`, prefer longer waits (minutes) to avoid busy polling.";
const DEFAULT_MULTI_AGENT_V2_SHARED_USAGE_HINT_TEXT: &str = codex_prompt_texts::text!(
    "templates/multi_agent_instructions/default_multi_agent_v2_shared_usage_hint_text.md"
);

/// Multi-agent role text and the captured capabilities used to render its context segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiAgentRoleInstructions {
    /// Complete configured instructions, emitted verbatim without catalog markers.
    Configured(String),
    /// Selected catalog or bundled text, composed with captured runtime guidance.
    Composed {
        base: String,
        marked: bool,
        omit_update_plan_instructions: bool,
        max_concurrency: usize,
        wait_agent_enabled: bool,
        expose_model_overrides: bool,
    },
}

impl MultiAgentRoleInstructions {
    /// Content classification shared by role producers and inherited-history filtering.
    pub const KIND: &str = "multi_agent.role_instructions";
}

impl ContextualUserFragment for MultiAgentRoleInstructions {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind(Self::KIND.to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        match self {
            Self::Composed { marked: true, .. } => Self::type_markers(),
            Self::Configured(_) | Self::Composed { marked: false, .. } => ("", ""),
        }
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<multi_agent_role>", "</multi_agent_role>")
    }

    fn body(&self) -> ContentItem {
        input_text(match self {
            Self::Configured(text) => text.clone(),
            Self::Composed {
                base,
                omit_update_plan_instructions,
                max_concurrency,
                wait_agent_enabled,
                expose_model_overrides,
                ..
            } => {
                let base = if *omit_update_plan_instructions {
                    without_update_plan_instructions(base)
                } else {
                    base.clone()
                };
                let wait_agent_guidance = if *wait_agent_enabled {
                    format!(
                        codex_prompt_texts::text!(
                            "templates/multi_agent_instructions/concurrency_slots/default_multi_agent_v2_wait.md"
                        ),
                        DEFAULT_MULTI_AGENT_V2_WAIT_AGENT_USAGE_HINT_TEXT =
                            DEFAULT_MULTI_AGENT_V2_WAIT_AGENT_USAGE_HINT_TEXT
                    )
                } else {
                    String::new()
                };
                let shared = DEFAULT_MULTI_AGENT_V2_SHARED_USAGE_HINT_TEXT;
                let mut text = format!(
                    codex_prompt_texts::text!(
                        "templates/multi_agent_instructions/concurrency_slots/base_shared_wait_agent_guidance.md"
                    ),
                    base = base,
                    shared = shared,
                    wait_agent_guidance = wait_agent_guidance,
                    max_concurrency = max_concurrency
                );
                if *expose_model_overrides {
                    text.push_str("\n\n");
                    text.push_str(DEFAULT_MULTI_AGENT_V2_MODEL_OVERRIDE_USAGE_HINT_TEXT);
                }
                text
            }
        })
    }
}

#[cfg(test)]
#[path = "multi_agent_instructions_tests.rs"]
mod tests;
