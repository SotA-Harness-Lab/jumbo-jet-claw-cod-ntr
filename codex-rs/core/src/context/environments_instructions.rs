use codex_protocol::protocol::ENVIRONMENTS_INSTRUCTIONS_CLOSE_TAG;
use codex_protocol::protocol::ENVIRONMENTS_INSTRUCTIONS_OPEN_TAG;

use super::ContextualUserFragment;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

pub(crate) struct EnvironmentsInstructions;

impl ContextualUserFragment for EnvironmentsInstructions {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("environments.instructions".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            ENVIRONMENTS_INSTRUCTIONS_OPEN_TAG,
            ENVIRONMENTS_INSTRUCTIONS_CLOSE_TAG,
        )
    }

    fn body(&self) -> ContentItem {
        input_text(
            codex_prompt_texts::text!("core/context/environments_instructions/body.md").to_string(),
        )
    }
}
