// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What the model is told when its turn was about to end with work left, and it is sent back
//! to finish it.

use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

use super::ContextualUserFragment;

pub(crate) struct UnfinishedWorkContinuation {
    /// Why the harness found the work unfinished, in a sentence or two addressed to the model.
    reason: String,
}

impl UnfinishedWorkContinuation {
    pub(crate) fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
        }
    }
}

impl ContextualUserFragment for UnfinishedWorkContinuation {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("generic.unfinished_work_continuation".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            "<unfinished_work_continuation>",
            "</unfinished_work_continuation>",
        )
    }

    fn body(&self) -> ContentItem {
        input_text(format!(
            codex_prompt_texts::text!("core/context/unfinished_work_continuation/body.md"),
            self.reason.trim()
        ))
    }
}
