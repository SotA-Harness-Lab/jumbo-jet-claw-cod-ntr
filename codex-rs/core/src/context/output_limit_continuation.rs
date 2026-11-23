// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What the model is told when its response was cut off at the output token limit and it is
//! asked to go on.

use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

use super::ContextualUserFragment;

pub(crate) struct OutputLimitContinuation;

impl ContextualUserFragment for OutputLimitContinuation {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("generic.output_limit_continuation".to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            "<output_limit_continuation>",
            "</output_limit_continuation>",
        )
    }

    fn body(&self) -> ContentItem {
        input_text(codex_prompt_texts::text!(
            "core/context/output_limit_continuation/body.md"
        ))
    }
}
