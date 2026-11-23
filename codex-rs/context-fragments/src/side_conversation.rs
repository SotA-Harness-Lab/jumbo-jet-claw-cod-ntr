//! Shared instructions and the inherited-history boundary for side conversations.

use crate::ContextualUserFragment;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::ResponseInputItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::models::input_text;

pub const SIDE_BOUNDARY_TITLE: &str = "Side conversation boundary.";

const SIDE_BOUNDARY_PROMPT: &str =
    codex_prompt_texts::text!("context-fragments/side_conversation/side_boundary_prompt.md");

const SIDE_DEVELOPER_INSTRUCTIONS: &str =
    codex_prompt_texts::text!("context-fragments/side_conversation/side_developer_instructions.md");

/// Developer policy checkpointed before the first side-chat turn.
pub enum SideConversationPolicy {
    Attached,
    Temporary,
}

impl SideConversationPolicy {
    /// Extends a thread's configured developer instructions with the side-chat policy.
    pub fn instructions(&self, existing: Option<&str>) -> String {
        let mut instructions = match existing {
            Some(existing) if !existing.trim().is_empty() => {
                format!("{existing}\n\n{SIDE_DEVELOPER_INSTRUCTIONS}")
            }
            _ => SIDE_DEVELOPER_INSTRUCTIONS.to_string(),
        };
        if matches!(self, Self::Temporary) {
            instructions.push_str(codex_prompt_texts::text!(
                "context-fragments/side_conversation/temporary_worktree_notice.md"
            ));
        }
        instructions
    }

    /// An inherited turn can suppress the fork's configured instructions. Append the
    /// developer policy before the history boundary so the first child request sees it.
    pub fn fork_items(self) -> Vec<ResponseItem> {
        vec![
            ContextualUserFragment::into(self),
            side_conversation_boundary(),
        ]
    }
}

impl ContextualUserFragment for SideConversationPolicy {
    fn role(&self) -> &'static str {
        "developer"
    }

    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("side_conversation.policy".to_string())
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<side_conversation>", "</side_conversation>")
    }

    fn body(&self) -> ContentItem {
        input_text(self.instructions(/*existing*/ None))
    }
}

/// The existing user-role delimiter between the inherited transcript and side-chat turns.
pub fn side_conversation_boundary() -> ResponseItem {
    ResponseInputItem::Message {
        role: "user".to_string(),
        content: vec![input_text(format!(
            "{SIDE_BOUNDARY_TITLE}\n\n{SIDE_BOUNDARY_PROMPT}"
        ))],
        phase: None,
    }
    .into()
}
