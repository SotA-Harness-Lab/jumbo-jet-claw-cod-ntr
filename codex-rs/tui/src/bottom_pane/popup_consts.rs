// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Shared popup-related constants for bottom pane widgets.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::key_hint;
use crate::key_hint::ShortcutHint;
use crate::keymap::ListAction;
use crate::keymap::ListKeymap;
use crossterm::event::KeyCode;

/// Maximum number of rows any popup should attempt to display.
/// Keep this consistent across all popups for a uniform feel.
pub(crate) const MAX_POPUP_ROWS: usize = 8;

/// Standard footer hint text used by popups.
pub(crate) fn standard_popup_hint_line() -> Line<'static> {
    Line::from(vec![
        tr!("Press ").into(),
        key_hint::plain(KeyCode::Enter).into(),
        tr!(" to confirm or ").into(),
        key_hint::plain(KeyCode::Esc).into(),
        tr!(" to go back").into(),
    ])
}

/// Compact footer for shared pickers, using only the configured list actions.
pub(crate) fn picker_hint_line_for_keymap(list_keymap: &ListKeymap) -> Line<'static> {
    let mut spans = Vec::new();
    for (action, label) in [
        (ListAction::Accept, tr!(" select")),
        (ListAction::Cancel, tr!(" back")),
    ] {
        if let Some(hint) = list_keymap.primary_hint(action) {
            if !spans.is_empty() {
                spans.push(" · ".dim());
            }
            spans.extend(hint.spans());
            spans.push(label.dim());
        }
    }
    spans.into()
}

pub(crate) fn accept_cancel_hint_line(
    accept: Option<ShortcutHint>,
    accept_label: &'static str,
    cancel: Option<ShortcutHint>,
    cancel_label: &'static str,
) -> Line<'static> {
    let mut spans = Vec::new();
    if let Some(accept) = accept {
        let accept_label = crate::i18n::translate_in("accept-cancel-hint", accept_label);
        spans.push(tr!("Press ").dim());
        spans.extend(accept.spans());
        spans.push(format!(" {accept_label}").dim());
    }
    if let Some(cancel) = cancel {
        let cancel_label = crate::i18n::translate_in("accept-cancel-hint", cancel_label);
        spans.push(
            if spans.is_empty() {
                tr!("Press ")
            } else {
                tr!(" or ")
            }
            .dim(),
        );
        spans.extend(cancel.spans());
        spans.push(format!(" {cancel_label}").dim());
    }
    spans.into()
}
