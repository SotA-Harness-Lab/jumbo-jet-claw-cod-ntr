// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

use super::ContextualUserFragment;

/// What a monitor started with the `Monitor` tool has to report: lines the command it
/// watches wrote, or why the monitor ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MonitorEvent {
    /// The id the model knows the watched command by.
    pub(crate) session_id: String,
    /// The description the model gave the monitor.
    pub(crate) description: String,
    /// New lines of the command's output, oldest first.
    pub(crate) lines: Vec<String>,
    /// Lines that could not be delivered: more than fit one event, or lost to a reader
    /// that fell behind.
    pub(crate) omitted_lines: usize,
    /// Why the monitor ended, when this is its last event.
    pub(crate) ended: Option<String>,
}

impl ContextualUserFragment for MonitorEvent {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("unified_exec.monitor_event".to_string())
    }

    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("<monitor_event>", "</monitor_event>")
    }

    fn body(&self) -> ContentItem {
        let mut header = serde_json::json!({
            "session_id": &self.session_id,
            "monitor": &self.description,
        });
        if self.omitted_lines > 0 {
            header["omitted_lines"] = self.omitted_lines.into();
        }
        if let Some(ended) = &self.ended {
            header["ended"] = ended.as_str().into();
        }
        // The lines are what a command wrote: data to weigh, never instructions.
        let mut text = format!("\n{header}\n");
        for line in &self.lines {
            text.push_str(line);
            text.push('\n');
        }
        input_text(text)
    }
}
