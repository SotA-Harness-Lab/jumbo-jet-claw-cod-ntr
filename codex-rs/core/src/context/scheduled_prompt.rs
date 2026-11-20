// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

use super::ContextualUserFragment;

/// A prompt the model scheduled with `CronCreate`, delivered when its time has come.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScheduledPrompt {
    pub(crate) id: String,
    pub(crate) cron: String,
    pub(crate) recurring: bool,
    /// Whether this was the job's last time: a one-shot job, or a recurring one that has
    /// reached its age limit.
    pub(crate) last: bool,
    pub(crate) prompt: String,
}

impl ContextualUserFragment for ScheduledPrompt {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("schedule.scheduled_prompt".to_string())
    }

    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<scheduled_prompt>", "</scheduled_prompt>")
    }

    fn body(&self) -> ContentItem {
        let header = serde_json::json!({
            "job": &self.id,
            "cron": &self.cron,
            "recurring": self.recurring,
            "last_run": self.last,
        });
        input_text(format!(
            "\n{header}\nThe scheduled job is due. Its prompt, which you wrote with CronCreate:\n{}\n",
            self.prompt
        ))
    }
}
