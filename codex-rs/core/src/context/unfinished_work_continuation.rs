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
            "\nYour turn was about to end, but the work does not look finished: {}\n\n\
             The user expects the whole request to be done when your turn ends, so do not stop \
             to say what you will do next: do it now, with tool calls, and keep going until the \
             user's request is fully done. Do not repeat what you already wrote. End the turn \
             only when the work is complete, or when you cannot go on without the user (a \
             decision that is theirs, information or access only they have): then say plainly \
             what you need from them. If you find the work is in fact complete, give your final \
             answer now.\n",
            self.reason.trim()
        ))
    }
}
