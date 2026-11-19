//! A persistent mode label and responsive hints, using the configured action for details.

use super::*;
use crate::bottom_pane::TranscriptFooter;
use crate::footer_hint::first_fitting_line;
use ratatui::style::Stylize;
use ratatui::text::Line;

impl App {
    pub(crate) fn prompt_navigation_footer(&self, width: u16) -> Option<TranscriptFooter> {
        if !self.backtrack.overlay_preview_active
            || (self.overlay.is_none()
                && (self.transcript_view.has_active_interaction()
                    || self.transcript_view.history == TranscriptHistoryState::Failed))
        {
            return None;
        }
        let hint_details = crate::i18n::tr!("prompt-navigation-hint-details");
        let hint_scroll = crate::i18n::tr!("prompt-navigation-hint-scroll");
        let hint_prompts = crate::i18n::tr!("prompt-navigation-hint-prompts");
        let hint_rewind = crate::i18n::tr!("prompt-navigation-hint-rewind");
        let hint_back = crate::i18n::tr!("prompt-navigation-hint-back");
        let label_long = crate::i18n::tr!("prompt-navigation-browsing-transcript");
        let label = crate::i18n::tr!("prompt-navigation-browsing");
        let details = self
            .keymap
            .primary_hint(crate::keymap::KeymapContext::Global, "open_transcript")
            .map(|key| (key.display_label(), hint_details));
        let mut full_hints = vec![
            ("↑↓/jk".to_string(), hint_scroll),
            ("←→/hl".to_string(), hint_prompts),
        ];
        full_hints.extend(details.clone());
        full_hints.extend([
            ("↵".to_string(), hint_rewind),
            ("esc".to_string(), hint_back),
        ]);
        let mut compact_hints = vec![("↑↓/jk ←→/hl".to_string(), "")];
        compact_hints.extend(details);
        compact_hints.extend([
            ("↵".to_string(), hint_rewind),
            ("esc".to_string(), hint_back),
        ]);
        let line = first_fitting_line(
            [
                (label_long, full_hints.clone()),
                (label, full_hints),
                (label, compact_hints),
                (
                    label,
                    vec![
                        ("↑↓/jk ←→/hl".to_string(), ""),
                        ("↵".to_string(), hint_rewind),
                        ("esc".to_string(), ""),
                    ],
                ),
                (
                    label,
                    vec![
                        ("↵".to_string(), hint_rewind),
                        ("esc".to_string(), hint_back),
                    ],
                ),
                (label, vec![("esc".to_string(), hint_back)]),
                (label, vec![("esc".to_string(), "")]),
                (label, Vec::new()),
            ]
            .map(|(label, hints)| {
                let mut line = Line::from(label.fg(crate::style::accent_color()));
                for (keys, action) in hints {
                    line.spans.push(" · ".dim());
                    line.spans.extend(crate::key_hint::key_label_spans(&keys));
                    if !action.is_empty() {
                        line.spans.push(format!(" {action}").dim());
                    }
                }
                line
            }),
            width,
        );
        Some(TranscriptFooter {
            text: line.into(),
            cursor_column: None,
            is_interactive: true,
        })
    }
}
