// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `idle_prompt` notification: once a turn has ended and the session has
//! waited for the user's next prompt for about a minute, the `Notification` hooks run.

use std::sync::Arc;
use std::time::Duration;

use codex_protocol::protocol::AgentStatus;
use codex_protocol::protocol::SessionSource;
use tokio_util::task::AbortOnDropHandle;

use crate::session::session::Session;
use crate::session::turn_context::TurnContext;

/// How long after a turn Claude Code waits before it sends `idle_prompt`.
const IDLE_PROMPT_DELAY: Duration = Duration::from_secs(60);

/// The notification's message, Claude Code's "Claude is waiting for your input".
pub(crate) const IDLE_PROMPT_MESSAGE: &str = "Codex is waiting for your input";

/// Whether a session started from `source` sends `idle_prompt`. Claude Code sends it from
/// the sessions someone attends, which in Codex are the TUI and the IDE.
pub(super) fn sends_idle_prompt(source: &SessionSource) -> bool {
    matches!(source, SessionSource::Cli | SessionSource::VSCode)
}

impl Session {
    /// Starts the wait for `idle_prompt` after a turn of `turn_context` ended without being
    /// interrupted. The next turn cancels it.
    pub(crate) async fn start_idle_prompt_wait(self: &Arc<Self>, turn_context: Arc<TurnContext>) {
        if sends_idle_prompt(&turn_context.session_source) {
            self.start_idle_prompt_wait_after(turn_context, IDLE_PROMPT_DELAY)
                .await;
        }
    }

    /// Runs the `idle_prompt` hooks once `delay` has passed, unless a turn has started in
    /// the meantime, the session is shutting down, or a subagent of the session is still at
    /// work (Claude Code waits for its background agents too).
    pub(super) async fn start_idle_prompt_wait_after(
        self: &Arc<Self>,
        turn_context: Arc<TurnContext>,
        delay: Duration,
    ) {
        if !self.hooks().has_notification_hooks() {
            return;
        }
        let session = Arc::downgrade(self);
        let wait = AbortOnDropHandle::new(tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            let Some(session) = session.upgrade() else {
                return;
            };
            let turn_running = session.active_turn.lock().await.is_some();
            if turn_running {
                return;
            }
            let shutting_down = session.state.lock().await.shutting_down;
            if shutting_down || session.has_running_subagent(&turn_context).await {
                return;
            }
            crate::hook_runtime::run_notification_hooks(
                &session,
                &turn_context,
                "idle_prompt",
                IDLE_PROMPT_MESSAGE.to_string(),
            )
            .await;
        }));
        // Replacing an earlier wait cancels it.
        self.state.lock().await.idle_prompt_wait = Some(wait);
    }

    /// Cancels the wait for `idle_prompt`: a turn starts.
    pub(crate) async fn cancel_idle_prompt_wait(&self) {
        let wait = self.state.lock().await.idle_prompt_wait.take();
        drop(wait);
    }

    async fn has_running_subagent(&self, turn_context: &TurnContext) -> bool {
        let Ok(agents) = self
            .services
            .agent_control
            .list(
                self.thread_id,
                turn_context.parent_thread_id,
                &turn_context.session_source,
                /*path_prefix*/ None,
            )
            .await
        else {
            return false;
        };
        agents.iter().any(|agent| {
            agent.thread_id != self.thread_id
                && matches!(
                    agent.status,
                    AgentStatus::PendingInit | AgentStatus::Running
                )
        })
    }
}
