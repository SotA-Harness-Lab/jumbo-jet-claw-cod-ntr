// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Persist the next launch's renderer preference without touching terminal ownership.

use super::App;
use crate::config_update::format_config_error;
use crate::legacy_core::config::edit::ConfigEdit;
use crate::legacy_core::config::edit::ConfigEditsBuilder;

impl App {
    pub(super) async fn save_fullscreen_transcript(&mut self, enabled: bool) {
        let result =
            ConfigEditsBuilder::for_config_path(self.local_settings.user_config_path.as_path())
                .with_edits([ConfigEdit::SetPath {
                    segments: vec!["tui".into(), "fullscreen_transcript".into()],
                    value: toml_edit::value(enabled),
                }])
                .apply()
                .await;
        match result {
            Ok(()) => {
                self.local_settings.tui.fullscreen_transcript = enabled;
                self.chat_widget.local_settings.tui.fullscreen_transcript = enabled;
                let mode = if enabled {
                    tr!("Fullscreen")
                } else {
                    tr!("Scrollback")
                };
                self.chat_widget.add_info_message(
                    tr!("Saved TUI mode: {mode}. Restart Codex to apply; launch overrides still apply.", mode),
                    /*hint*/ None,
                );
            }
            Err(error) => self.chat_widget.add_error_message(tr!(
                "Failed to save TUI mode: {error}",
                error = format_config_error(&error)
            )),
        }
    }
}

#[cfg(test)]
#[path = "tui_mode_picker_tests.rs"]
mod tests;
