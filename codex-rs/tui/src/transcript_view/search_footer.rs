// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Render Find query input and navigation hints without owning search transitions.

use super::*;
use crate::bottom_pane::TextAreaState;
use ratatui::text::Span;
use ratatui::widgets::StatefulWidgetRef;
use unicode_width::UnicodeWidthStr;

impl TranscriptView {
    /// Project the editor and its caret from the same one-row viewport into the existing footer.
    pub(crate) fn search_footer(&self, width: u16) -> Option<(Line<'static>, u16)> {
        if !self.is_search_editing() || width == 0 {
            return None;
        }
        let area = Rect::new(/*x*/ 0, /*y*/ 0, width, /*height*/ 1);
        let mut buffer = Buffer::empty(area);
        let prefix_width = width.saturating_sub(/*rhs*/ 1).min(/*other*/ 6);
        Line::from(tr!("Find: ")).dim().render(
            Rect::new(/*x*/ 0, /*y*/ 0, prefix_width, /*height*/ 1),
            &mut buffer,
        );
        let query_area = Rect::new(
            prefix_width,
            /*y*/ 0,
            width - prefix_width,
            /*height*/ 1,
        );
        let mut state = TextAreaState::default();
        StatefulWidgetRef::render_ref(&&self.search.editor, query_area, &mut buffer, &mut state);
        let (cursor_column, _) = self
            .search
            .editor
            .cursor_pos_with_state(query_area, state)?;
        let mut spans = Vec::new();
        let mut column = 0;
        while column < width {
            let cell = &buffer[(column, 0)];
            spans.push(Span::styled(cell.symbol().to_string(), cell.style()));
            column += cell.symbol().width().max(/*other*/ 1) as u16;
        }
        Some((Line::from(spans), cursor_column))
    }
}

impl Search {
    pub(in crate::transcript_view) fn status_line(
        &self,
        width: u16,
        history: TranscriptHistoryState,
    ) -> Line<'static> {
        let previous = crate::key_hint::ctrl(crossterm::event::KeyCode::Char('p')).display_label();
        let next = crate::key_hint::ctrl(crossterm::event::KeyCode::Char('n')).display_label();
        let retry_hint = tr!("{previous} retry", previous = previous.as_str());
        let unavailable_hint = tr!(
            "History unavailable · {retry_hint}",
            retry_hint = retry_hint.as_str()
        );
        let next_hint = tr!(
            "enter accept · {previous} older · {next} newer",
            previous = previous.as_str(),
            next = next.as_str()
        );
        let exhausted_hint = tr!(
            "No more matches · {next_hint}",
            next_hint = next_hint.as_str()
        );
        let (status, compact) = match self.progress {
            Progress::Idle | Progress::Found if self.is_reading() => {
                (tr!("ctrl+p older · ctrl+n newer"), tr!("ctrl+p older"))
            }
            Progress::Idle if !self.editor.is_empty() => (next_hint.as_str(), tr!("enter accept")),
            Progress::Idle => (tr!("Type to find"), tr!("Type to find")),
            Progress::Restart | Progress::Scanning(_) => (tr!("Searching…"), tr!("Searching…")),
            Progress::AwaitingHistory if history == TranscriptHistoryState::Failed => {
                (unavailable_hint.as_str(), retry_hint.as_str())
            }
            Progress::AwaitingHistory => (tr!("Searching earlier history…"), tr!("Loading…")),
            Progress::Found => (next_hint.as_str(), tr!("enter accept")),
            Progress::Exhausted if self.is_reading() && self.current.is_some() => (
                tr!("No more matches · ctrl+p older · ctrl+n newer"),
                tr!("ctrl+p older"),
            ),
            Progress::Exhausted if self.current.is_some() => {
                (exhausted_hint.as_str(), tr!("enter accept"))
            }
            Progress::Exhausted => (tr!("No matches"), tr!("No matches")),
        };
        let limit = if self.query_truncated {
            tr!(" · query limited to 4 KiB")
        } else {
            ""
        };
        if self.is_reading() {
            return crate::footer_hint::first_fitting_line(
                [
                    tr!("Find · {status} · esc latest", status),
                    tr!("Find · {compact} · esc latest", compact),
                    tr!("esc latest").to_owned(),
                ]
                .map(|hint| crate::transcript_view::footer::navigation_line(&hint)),
                width,
            );
        }
        crate::footer_hint::first_fitting_line(
            [
                tr!(
                    "{status} · full transcript · esc cancel{limit}",
                    status,
                    limit
                ),
                tr!("{status} · esc cancel{limit}", status, limit),
                tr!("{compact} · esc cancel", compact),
                tr!("{compact} · esc", compact),
                tr!("esc cancel").to_owned(),
            ]
            .map(|hint| crate::transcript_view::footer::navigation_line(&hint)),
            width,
        )
    }
}
