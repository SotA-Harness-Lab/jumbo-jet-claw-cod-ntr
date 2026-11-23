// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

use super::ContextualUserFragment;

/// Tells the model that a command it left running in the background has ended, which it
/// would otherwise only learn by asking for the command's output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BackgroundCommandNotification {
    /// The id the model knows the command by: the `session_id` of `write_stdin`.
    pub(crate) session_id: String,
    pub(crate) command: String,
    /// `None` when the command could not be run to an exit code.
    pub(crate) exit_code: Option<i32>,
    /// What went wrong, when the command failed without an exit code.
    pub(crate) failure: Option<String>,
}

impl ContextualUserFragment for BackgroundCommandNotification {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("unified_exec.background_command_notification".to_string())
    }

    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            "<background_command_notification>",
            "</background_command_notification>",
        )
    }

    fn body(&self) -> ContentItem {
        input_text(format!(
            codex_prompt_texts::text!("core/context/background_command_notification/body.md"),
            serde_json::json!({
                "session_id": &self.session_id,
                "command": &self.command,
                "exit_code": &self.exit_code,
                "failure": &self.failure,
            })
        ))
    }
}
