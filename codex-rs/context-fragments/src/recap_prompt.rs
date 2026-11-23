//! Bounded catch-up instructions for a temporary recap request.
//! History selection belongs to the caller; this fragment caps the complete prompt.

use crate::ContextualUserFragment;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;
use codex_utils_string::approx_bytes_for_tokens;

const PROMPT_PREFIX: &str =
    codex_prompt_texts::text!("context-fragments/recap_prompt/prompt_prefix.md");

/// A recap prompt built from recent user-visible conversation, never tool output.
pub struct RecapPrompt<'a> {
    history: &'a str,
}

impl<'a> RecapPrompt<'a> {
    /// Complete prompt budget using the shared four-bytes-per-token estimate.
    pub const MAX_ESTIMATED_TOKENS: usize = 8_192;
    /// Total UTF-8 bytes, including instructions and conversation labels.
    /// This is a byte ceiling, not an exact model-token count.
    pub const MAX_BYTES: usize = approx_bytes_for_tokens(Self::MAX_ESTIMATED_TOKENS);
    /// Space available after the fixed instructions; callers must count their labels.
    pub const HISTORY_MAX_BYTES: usize = Self::MAX_BYTES - PROMPT_PREFIX.len();

    pub fn new(history: &'a str) -> Self {
        let end = history.floor_char_boundary(Self::HISTORY_MAX_BYTES.min(history.len()));
        Self {
            history: &history[..end],
        }
    }
}

impl ContextualUserFragment for RecapPrompt<'_> {
    fn role(&self) -> &'static str {
        "user"
    }

    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("recap.prompt".to_string())
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> ContentItem {
        input_text(format!("{PROMPT_PREFIX}{}", self.history))
    }
}

#[cfg(test)]
#[path = "recap_prompt_tests.rs"]
mod tests;
