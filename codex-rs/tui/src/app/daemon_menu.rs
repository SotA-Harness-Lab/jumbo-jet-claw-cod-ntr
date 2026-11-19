// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Local daemon maintenance confirms restarts and updates before acting.
//! Restarts use normal TUI reconnection; updates run after restoring the terminal.

use super::*;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::update_action::DaemonUpdateSource;
use crate::wrapping::word_wrap_lines;
use ratatui::buffer::Buffer;
use ratatui::widgets::Paragraph;

struct DaemonMenuHeader(Vec<Line<'static>>);

impl Renderable for DaemonMenuHeader {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Renderable::render(
            &Paragraph::new(word_wrap_lines(&self.0, usize::from(area.width))),
            area,
            buf,
        );
    }

    fn desired_height(&self, width: u16) -> u16 {
        word_wrap_lines(&self.0, usize::from(width)).len() as u16
    }
}

impl App {
    pub(super) async fn verify_managed_daemon(&mut self) {
        // A discovered socket alone does not prove it belongs to a managed daemon.
        // Probe only when the user opens a view that offers restart, never during startup.
        if !self.reconnect.offline
            && let Some(connection) = &mut self.chat_widget.remote_connection
            && connection.is_local_daemon
            && !connection.is_managed_daemon
        {
            connection.is_managed_daemon =
                codex_app_server_daemon::run(codex_app_server_daemon::LifecycleCommand::Version)
                    .await
                    .is_ok_and(|output| output.backend.is_some());
        }
    }

    pub(super) fn open_daemon_menu(&mut self) {
        let status = self
            .chat_widget
            .remote_connection
            .as_ref()
            .filter(|_| matches!(self.app_server_target, AppServerTarget::LocalDaemon { .. }))
            .map(|connection| {
                crate::i18n::tr!(
                    "daemon-menu-running",
                    version = connection.version.to_string()
                )
            })
            .unwrap_or_else(|| crate::i18n::tr!("daemon-menu-not-connected").to_string());
        let mut header = vec![
            Line::from(crate::i18n::tr!("daemon-menu-title").bold()),
            Line::from(status.dim()),
        ];
        let unavailable = if matches!(
            self.app_server_target,
            AppServerTarget::Remote { .. } | AppServerTarget::Cloud
        ) {
            Some(crate::i18n::tr!("daemon-menu-remote-unavailable"))
        } else if self.daemon_cli_executable.is_none() {
            Some(crate::i18n::tr!("daemon-menu-run-cli"))
        } else {
            None
        };
        if let Some(guidance) = unavailable {
            header.push(Line::from(guidance.dim()));
        }
        let has_package = self.daemon_cli_executable.as_ref().is_some_and(|path| {
            codex_install_context::InstallContext::from_exe(
                cfg!(target_os = "macos"),
                Some(path.as_path()),
                /*method_override*/ None,
            )
            .package_layout
            .is_some()
        });
        let mut items: Vec<SelectionItem> = [
            (
                DaemonUpdateSource::PublicStable,
                crate::i18n::tr!("daemon-menu-install-stable"),
            ),
            (
                DaemonUpdateSource::ThisCli,
                crate::i18n::tr!("daemon-menu-use-this-cli"),
            ),
        ]
        .into_iter()
        .map(|(source, name)| SelectionItem {
            name: name.to_string(),
            is_disabled: unavailable.is_some(),
            disabled_reason: (unavailable.is_none()
                && source == DaemonUpdateSource::ThisCli
                && !has_package)
                .then(|| crate::i18n::tr!("daemon-menu-no-local-package").to_string()),
            actions: vec![Box::new(move |tx| {
                tx.send(AppEvent::ConfirmDaemonUpdate(source));
            })],
            dismiss_on_select: true,
            ..Default::default()
        })
        .collect();
        items.push(SelectionItem {
            name: crate::i18n::tr!("daemon-menu-restart").to_string(),
            is_disabled: !self
                .chat_widget
                .remote_connection
                .as_ref()
                .is_some_and(|connection| connection.is_managed_daemon)
                || self.reconnect.offline
                || self.daemon_restart.is_some(),
            disabled_reason: self
                .chat_widget
                .remote_connection
                .as_ref()
                .filter(|connection| connection.is_local_daemon && !connection.is_managed_daemon)
                .map(|_| crate::i18n::tr!("daemon-menu-restart-unverified").to_string()),
            actions: vec![Box::new(|tx| tx.send(AppEvent::ConfirmDaemonRestart))],
            dismiss_on_select: true,
            ..Default::default()
        });
        self.chat_widget.show_selection_view(SelectionViewParams {
            header: Box::new(DaemonMenuHeader(header)),
            items,
            ..SelectionViewParams::picker()
        });
    }

    pub(super) fn confirm_daemon_restart(&mut self) {
        if !self
            .chat_widget
            .remote_connection
            .as_ref()
            .is_some_and(|connection| connection.is_managed_daemon)
            || self.daemon_restart.is_some()
        {
            return;
        }
        self.chat_widget.show_selection_view(SelectionViewParams {
            header: Box::new(DaemonMenuHeader(vec![
                Line::from(crate::i18n::tr!("daemon-menu-restart-title").bold()),
                Line::from(crate::i18n::tr!("daemon-menu-restart-interrupt")),
                Line::from(crate::i18n::tr!("daemon-menu-restart-reconnect")),
            ])),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("daemon-menu-cancel").to_string(),
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("daemon-menu-restart-button").to_string(),
                    actions: vec![Box::new(|tx| tx.send(AppEvent::RunDaemonRestart))],
                    require_explicit_confirmation: true,
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::confirmation()
        });
    }

    pub(super) fn confirm_daemon_update(&mut self, source: DaemonUpdateSource) {
        let Some(executable) = &self.daemon_cli_executable else {
            return;
        };
        if matches!(
            self.app_server_target,
            AppServerTarget::Remote { .. } | AppServerTarget::Cloud
        ) {
            return;
        }
        let mut explanation = match source {
            DaemonUpdateSource::PublicStable => {
                crate::i18n::tr!("daemon-menu-stable-explanation").to_string()
            }
            DaemonUpdateSource::ThisCli => {
                let version = codex_install_context::InstallContext::current()
                    .package_manifest()
                    .map_or_else(
                        || CODEX_CLI_VERSION.to_string(),
                        |manifest| manifest.version.to_string(),
                    );
                crate::i18n::tr!(
                    "daemon-menu-this-cli-explanation",
                    version = version,
                    path = executable.display().to_string()
                )
            }
        };
        explanation.push('\n');
        explanation.push_str(crate::i18n::tr!("daemon-menu-update-interrupt"));
        explanation.push('\n');
        explanation.push_str(crate::i18n::tr!("daemon-menu-update-exit"));
        let mut header = vec![Line::from(
            crate::i18n::tr!("daemon-menu-update-title").bold(),
        )];
        header.extend(explanation.lines().map(|line| Line::from(line.to_owned())));
        self.chat_widget.show_selection_view(SelectionViewParams {
            header: Box::new(DaemonMenuHeader(header)),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("daemon-menu-cancel").to_string(),
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("daemon-menu-update-and-exit").to_string(),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::RunDaemonUpdate(source))
                    })],
                    require_explicit_confirmation: true,
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::confirmation()
        });
    }
}

#[cfg(test)]
#[path = "daemon_menu_tests.rs"]
mod tests;
