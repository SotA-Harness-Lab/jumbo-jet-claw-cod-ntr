//! Exit output distinguishes a disconnected client from a stopped embedded server.
//! Persisted sessions offer a direct UUID command and an optional named picker hint.

use super::*;
use crate::RemoteAppServerEndpoint;
use crate::exec_command::escape_command;
use crate::status::remote_connection::sanitized_websocket_url;

/// A persisted thread that can be resumed after the TUI exits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResumableThread {
    pub thread_id: ThreadId,
    pub thread_name: Option<String>,
}

/// Reconnection and stop guidance for a task owned by a persistent app server.
#[derive(Debug, Clone)]
pub struct DisconnectInfo {
    /// CLI arguments identifying the server, without credential values.
    pub command: Vec<String>,
    pub stop_hint: String,
}

impl App {
    pub(super) async fn finish_pending_archives(
        &mut self,
        deadline: tokio::time::Instant,
    ) -> Vec<String> {
        let mut warnings = Vec::new();
        for (thread_id, outcome) in self.agents_overview.pending_archives.drain() {
            match tokio::time::timeout_at(deadline, outcome).await {
                Ok(Ok(Ok(()))) => {}
                Ok(Ok(Err(error))) => {
                    warnings.push(crate::i18n::tr!(
                        "exit-summary-failed-archive-thread",
                        thread_id = thread_id.to_string(),
                        error = error.to_string()
                    ));
                }
                Ok(Err(_)) | Err(_) => {
                    warnings.push(crate::i18n::tr!(
                        "exit-summary-archive-not-confirmed",
                        thread_id = thread_id.to_string()
                    ));
                }
            }
        }
        warnings.sort();
        warnings
    }

    pub(super) fn exit_info(&self, exit_reason: ExitReason) -> AppExitInfo {
        let thread_id = self.chat_widget.thread_id().or(self.primary_thread_id);
        let disconnect_info = thread_id.and_then(|_| {
            let command = match &self.app_server_target {
                AppServerTarget::Embedded | AppServerTarget::Cloud => return None,
                AppServerTarget::LocalDaemon { .. } => vec!["codex".to_string()],
                AppServerTarget::Remote { endpoint } => {
                    let address = match endpoint {
                        RemoteAppServerEndpoint::WebSocket { websocket_url, .. } => {
                            sanitized_websocket_url(websocket_url)
                                .and_then(|url| {
                                    let host = url.host()?;
                                    let port = url.port_or_known_default()?;
                                    // The CLI requires an explicit port, including 80/443.
                                    Some(format!("{}://{host}:{port}{}", url.scheme(), url.path()))
                                })
                                .unwrap_or_else(|| "<server-address>".to_string())
                        }
                        RemoteAppServerEndpoint::UnixSocket { socket_path } => {
                            format!("unix://{}", socket_path.display())
                        }
                    };
                    vec!["codex".to_string(), "--remote".to_string(), address]
                }
            };
            let stop_hint = self
                .keymap
                .agents
                .primary_hint("stop", &self.keymap.agents.stop)
                .map_or_else(
                    || crate::i18n::tr!("exit-summary-use-stop-shortcut").to_string(),
                    |key| crate::i18n::tr!("exit-summary-press-key", key = key.display_label()),
                );
            Some(DisconnectInfo { command, stop_hint })
        });
        AppExitInfo {
            token_usage: self.token_usage(),
            thread_id,
            resume_hint: resumable_thread(
                thread_id,
                self.chat_widget.thread_name(),
                self.chat_widget.rollout_path().as_deref(),
            ),
            disconnect_info,
            archive_warnings: Vec::new(),
            update_action: self.pending_update_action,
            exit_reason,
        }
    }
}

impl AppExitInfo {
    /// Format the terminal summary for both the CLI and standalone TUI binaries.
    pub fn format_exit_messages(self, color_enabled: bool) -> Vec<String> {
        let color_command = |command: String| {
            if color_enabled {
                format!("\u{1b}[36m{command}\u{1b}[39m")
            } else {
                command
            }
        };
        let mut lines = self.archive_warnings;
        if let Some(disconnect) = self.disconnect_info
            && let Some(thread_id) = self.thread_id
        {
            let turn_interrupted = matches!(self.exit_reason, ExitReason::TurnInterrupted);
            let message = match self.exit_reason {
                ExitReason::UserRequested => {
                    crate::i18n::tr!("exit-summary-disconnected-running-continues")
                }
                ExitReason::Fatal(_) => {
                    crate::i18n::tr!("exit-summary-disconnected-may-be-running")
                }
                ExitReason::TurnInterrupted => {
                    crate::i18n::tr!("exit-summary-disconnected-turn-stopped")
                }
            };
            lines.push(message.to_string());
            let mut resume_command = disconnect.command.clone();
            resume_command.extend(["resume".to_string(), thread_id.to_string()]);
            lines.push(crate::i18n::tr!("exit-summary-to-reconnect").to_string());
            lines.push(format!(
                "  {}",
                color_command(escape_command(&resume_command)),
            ));
            if !turn_interrupted {
                let mut agents_command = disconnect.command;
                agents_command.push("agents".to_string());
                lines.push(crate::i18n::tr!(
                    "exit-summary-stop-current-turn",
                    command = color_command(escape_command(&agents_command)),
                    hint = disconnect.stop_hint,
                ));
            }
            if !self.token_usage.is_zero() {
                let usage = self.token_usage.to_string();
                lines.push(if turn_interrupted {
                    usage
                } else {
                    usage.replacen("Token usage:", "Token usage so far:", /*count*/ 1)
                });
            }
            return lines;
        }

        if !self.token_usage.is_zero() {
            lines.push(self.token_usage.to_string());
        }
        if let Some(thread) = self.resume_hint {
            lines.push(crate::i18n::tr!("exit-summary-to-continue").to_string());
            lines.push(format!(
                "  {}",
                color_command(format!("codex resume {}", thread.thread_id)),
            ));
            if let Some(thread_name) = thread.thread_name.filter(|name| !name.is_empty()) {
                lines.push(crate::i18n::tr!(
                    "exit-summary-or-run-select",
                    command = color_command("codex resume".to_string()),
                    name = color_command(thread_name),
                ));
            }
        } else if let Some(thread_id) = self.thread_id {
            lines.push(crate::i18n::tr!(
                "exit-summary-session-id",
                thread_id = thread_id.to_string()
            ));
        }
        lines
    }
}
