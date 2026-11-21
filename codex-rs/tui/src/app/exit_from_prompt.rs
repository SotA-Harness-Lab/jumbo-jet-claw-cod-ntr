// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Quitting from the prompt (`/exit`, `/quit`, `exit` and the other exit words), as Claude Code
//! does: quit at once when nothing is running, and otherwise ask first, naming the work that
//! quitting would stop.
//!
//! With the embedded app server, quitting stops everything this TUI started: the turn in
//! progress, the other sessions and subagents, side conversations and background terminals.
//! With a local daemon the work can outlive the TUI, so a running turn gets the daemon's own
//! menu (cancel it, leave it running in the background, or stop it and quit).

use super::App;
use crate::AppServerTarget;
use crate::app_event::AppEvent;
use crate::app_event::ExitMode;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use codex_app_server_protocol::ThreadStatus;

/// Work that quitting from the prompt would stop.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct RunningWork {
    pub(super) turn: bool,
    pub(super) sessions: usize,
    pub(super) side_conversations: usize,
    pub(super) background_terminals: usize,
}

impl RunningWork {
    pub(super) fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// The work as a list for the dialog's subtitle, such as "the turn in progress, 2 background
    /// terminals".
    pub(super) fn describe(&self) -> String {
        let mut parts = Vec::new();
        if self.turn {
            parts.push(crate::i18n::tr!("exit-running-work-turn").to_string());
        }
        if self.sessions > 0 {
            parts.push(crate::i18n::tr!(
                "exit-running-work-sessions",
                count = self.sessions as i64
            ));
        }
        if self.side_conversations > 0 {
            parts.push(crate::i18n::tr!(
                "exit-running-work-side",
                count = self.side_conversations as i64
            ));
        }
        if self.background_terminals > 0 {
            parts.push(crate::i18n::tr!(
                "exit-running-work-terminals",
                count = self.background_terminals as i64
            ));
        }
        parts.join(crate::i18n::tr!("exit-running-work-separator"))
    }
}

impl App {
    pub(super) async fn exit_from_prompt(&mut self) {
        match self.app_server_target {
            AppServerTarget::Embedded => {
                let work = self.running_work().await;
                if work.is_empty() {
                    self.app_event_tx.send(AppEvent::Exit(ExitMode::ShutdownFirst));
                } else {
                    self.show_running_work_exit_dialog(work);
                }
            }
            AppServerTarget::LocalDaemon { .. } => {
                if !self.show_running_task_exit_picker().await {
                    self.app_event_tx.send(AppEvent::Exit(ExitMode::ShutdownFirst));
                }
            }
            AppServerTarget::Cloud | AppServerTarget::Remote { .. } => {
                self.app_event_tx.send(AppEvent::Exit(ExitMode::ShutdownFirst));
            }
        }
    }

    async fn running_work(&self) -> RunningWork {
        let displayed = self.current_displayed_thread_id();
        let sessions = self
            .agents_overview
            .threads
            .iter()
            .filter(|(id, thread)| {
                Some(**id) != displayed
                    && !self.side_threads.contains_key(*id)
                    && matches!(
                        thread.as_ref().map(|thread| &thread.status),
                        Some(ThreadStatus::Active { .. })
                    )
            })
            .count();
        let mut side_conversations = 0;
        for thread_id in self.side_threads.keys().copied() {
            if self.active_turn_id_for_thread(thread_id).await.is_some() {
                side_conversations += 1;
            }
        }
        RunningWork {
            turn: self.chat_widget.is_user_turn_pending_or_running(),
            sessions,
            side_conversations,
            background_terminals: self.chat_widget.background_terminal_count(),
        }
    }

    fn show_running_work_exit_dialog(&mut self, work: RunningWork) {
        self.chat_widget.show_selection_view(SelectionViewParams {
            title: Some(crate::i18n::tr!("exit-running-work-title").to_string()),
            subtitle: Some(crate::i18n::tr!(
                "exit-running-work-subtitle",
                work = work.describe()
            )),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("exit-running-work-stay").to_string(),
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("exit-running-work-exit").to_string(),
                    // Confirmed: skip the embedded server's own question about other sessions.
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::ConfirmedEmbeddedExit(ExitMode::ShutdownFirst));
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::confirmation()
        });
    }
}

#[cfg(test)]
#[path = "exit_from_prompt_tests.rs"]
mod tests;
