// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Destination and filename prompts for on-demand transcript exports.

use super::*;
use crate::app_event::TranscriptExportDestination;
use crate::bottom_pane::popup_consts::picker_hint_line_for_keymap;

impl ChatWidget {
    pub(super) fn show_transcript_export_popup(&mut self) {
        self.show_selection_view(SelectionViewParams {
            header: Box::new(
                Paragraph::new(vec![
                    Line::from(tr!("Export conversation").bold()),
                    Line::from(tr!("Save the complete conversation as Markdown").dim()),
                ])
                .wrap(Wrap { trim: false }),
            ),
            footer_hint: Some(picker_hint_line_for_keymap(&self.bottom_pane.list_keymap())),
            items: vec![
                SelectionItem {
                    name: "Copy to clipboard".to_string(),
                    description: Some("Copy the complete Markdown transcript".to_string()),
                    is_disabled: cfg!(target_os = "android"),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::ExportTranscript {
                            destination: TranscriptExportDestination::Clipboard,
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: "Save to file".to_string(),
                    description: Some("Choose a Markdown filename".to_string()),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::OpenTranscriptExportFilePrompt);
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::fixed_choices()
        });
        self.defer_input_until_settings_applied();
        self.request_redraw();
    }

    pub(crate) fn show_transcript_export_file_prompt(&mut self) {
        let tx = self.app_event_tx.clone();
        let filename = self.thread_id().map_or_else(
            || "codex-session.md".to_string(),
            |thread_id| format!("codex-session-{thread_id}.md"),
        );
        let view = CustomPromptView::new(
            tr!("Save conversation").to_string(),
            tr!("Type a filename and press Enter").to_string(),
            filename,
            /*context_label*/ None,
            Box::new(move |filename| {
                tx.send(AppEvent::ExportTranscript {
                    destination: TranscriptExportDestination::File(PathBuf::from(filename)),
                });
            }),
        );
        self.bottom_pane.show_text_prompt(view);
        self.request_redraw();
    }
}
