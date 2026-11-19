// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Resolves and validates the shared shortcut prefix without assigning action bindings.
//! The default yields to explicit shortcuts; active consumers determine whether a menu opens.

use super::MAIN_RESERVED_BINDINGS;
use super::chords::RuntimeChordKeymap;
use super::chords::normalize_chord_binding;
use super::configured_context_alias_is_used;
use super::resolve_bindings;
use crate::key_hint;
use codex_config::types::TuiKeymap;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use itertools::Itertools;

pub(super) fn resolve(config: &TuiKeymap, chords: &mut RuntimeChordKeymap) -> Result<(), String> {
    let default = key_hint::ctrl(KeyCode::Char('x'));
    let shadowed = config.global.leader.is_none()
        && (configured_context_alias_is_used(config, "ctrl-x")
            || chords
                .bindings
                .iter()
                .any(|binding| binding.chord.prefix == default));
    if config.global.leader.as_ref().is_some_and(|bindings| {
        bindings
            .specs()
            .iter()
            .any(|spec| spec.as_str().contains(' '))
    }) {
        return Err(crate::i18n::tr!("leader-single-prefix-key").into());
    }
    let defaults = [default];
    chords.leader = resolve_bindings(
        config.global.leader.as_ref(),
        if shadowed { &[] } else { &defaults },
        "tui.keymap.global.leader",
    )?
    .into_iter()
    .map(normalize_chord_binding)
    .unique()
    .collect();

    for prefix in &chords.leader {
        let (key, modifiers) = prefix.parts();
        let label = prefix.display_label();
        if matches!(key, KeyCode::Char(_))
            && (key_hint::is_plain_text_key_event(KeyEvent::new(key, modifiers))
                || key_hint::is_altgr(modifiers))
        {
            return Err(crate::i18n::tr!("leader-needs-modifier", label = label));
        }
        #[cfg(unix)]
        if *prefix == key_hint::ctrl(KeyCode::Char('z')) {
            return Err(crate::i18n::tr!("leader-ctrl-z-reserved", label = label));
        }
        if let Some((reserved_action, _)) = MAIN_RESERVED_BINDINGS
            .iter()
            .find(|(_, reserved)| prefix.parts() == reserved.parts())
        {
            return Err(crate::i18n::tr!(
                "leader-key-reserved",
                label = label,
                reserved_action = *reserved_action
            ));
        }
    }
    Ok(())
}
