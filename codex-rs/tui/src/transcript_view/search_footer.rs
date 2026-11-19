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
        Line::from(crate::i18n::tr!("search-footer-find-prefix"))
            .dim()
            .render(
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
        let retry_hint = crate::i18n::tr!("search-footer-retry", previous = previous.as_str());
        let unavailable_hint = crate::i18n::tr!(
            "search-footer-history-unavailable",
            retry_hint = retry_hint.as_str()
        );
        let next_hint = crate::i18n::tr!(
            "search-footer-next-hint",
            previous = previous.as_str(),
            next = next.as_str()
        );
        let exhausted_hint = crate::i18n::tr!(
            "search-footer-exhausted-hint",
            next_hint = next_hint.as_str()
        );
        let (status, compact) = match self.progress {
            Progress::Idle | Progress::Found if self.is_reading() => (
                crate::i18n::tr!("search-footer-reading-nav"),
                crate::i18n::tr!("search-footer-ctrl-p-older"),
            ),
            Progress::Idle if !self.editor.is_empty() => (
                next_hint.as_str(),
                crate::i18n::tr!("search-footer-enter-accept"),
            ),
            Progress::Idle => (
                crate::i18n::tr!("search-footer-type-to-find"),
                crate::i18n::tr!("search-footer-type-to-find"),
            ),
            Progress::Restart | Progress::Scanning(_) => (
                crate::i18n::tr!("search-footer-searching"),
                crate::i18n::tr!("search-footer-searching"),
            ),
            Progress::AwaitingHistory if history == TranscriptHistoryState::Failed => {
                (unavailable_hint.as_str(), retry_hint.as_str())
            }
            Progress::AwaitingHistory => (
                crate::i18n::tr!("search-footer-searching-earlier"),
                crate::i18n::tr!("search-footer-loading"),
            ),
            Progress::Found => (
                next_hint.as_str(),
                crate::i18n::tr!("search-footer-enter-accept"),
            ),
            Progress::Exhausted if self.is_reading() && self.current.is_some() => (
                crate::i18n::tr!("search-footer-no-more-reading"),
                crate::i18n::tr!("search-footer-ctrl-p-older"),
            ),
            Progress::Exhausted if self.current.is_some() => (
                exhausted_hint.as_str(),
                crate::i18n::tr!("search-footer-enter-accept"),
            ),
            Progress::Exhausted => (
                crate::i18n::tr!("search-footer-no-matches"),
                crate::i18n::tr!("search-footer-no-matches"),
            ),
        };
        let limit = if self.query_truncated {
            crate::i18n::tr!("search-footer-query-limited")
        } else {
            ""
        };
        if self.is_reading() {
            return crate::footer_hint::first_fitting_line(
                [
                    crate::i18n::tr!("search-footer-reading-status", status = status),
                    crate::i18n::tr!("search-footer-reading-compact", compact = compact),
                    crate::i18n::tr!("search-footer-esc-latest").to_owned(),
                ]
                .map(|hint| crate::transcript_view::footer::navigation_line(&hint)),
                width,
            );
        }
        crate::footer_hint::first_fitting_line(
            [
                crate::i18n::tr!(
                    "search-footer-full-transcript",
                    status = status,
                    limit = limit
                ),
                crate::i18n::tr!(
                    "search-footer-status-cancel",
                    status = status,
                    limit = limit
                ),
                crate::i18n::tr!("search-footer-compact-cancel", compact = compact),
                crate::i18n::tr!("search-footer-compact-esc", compact = compact),
                crate::i18n::tr!("search-footer-esc-cancel").to_owned(),
            ]
            .map(|hint| crate::transcript_view::footer::navigation_line(&hint)),
            width,
        )
    }
}
