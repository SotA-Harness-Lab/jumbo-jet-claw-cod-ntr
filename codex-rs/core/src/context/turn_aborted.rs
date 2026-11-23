use super::ContextualUserFragment;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TurnAborted {
    pub(crate) guidance: String,
}

impl TurnAborted {
    pub(crate) const INTERRUPTED_GUIDANCE: &'static str =
        codex_prompt_texts::text!("core/context/turn_aborted/interrupted_guidance.md");
    pub(crate) const INTERRUPTED_NEUTRAL_GUIDANCE: &'static str =
        codex_prompt_texts::text!("core/context/turn_aborted/interrupted_neutral_guidance.md");

    pub(crate) fn new(guidance: impl Into<String>) -> Self {
        Self {
            guidance: guidance.into(),
        }
    }
}

impl ContextualUserFragment for TurnAborted {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("generic.turn_aborted".to_string())
    }

    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<turn_aborted>", "</turn_aborted>")
    }

    fn body(&self) -> ContentItem {
        input_text(format!("\n{}\n", self.guidance))
    }
}
