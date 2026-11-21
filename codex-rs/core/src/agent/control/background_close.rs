// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Orders the close of a finished background agent after its completion notice.
//!
//! A background agent that Claude Code's `Agent` tool starts on multi-agent v1 is closed once
//! it finishes, so that it gives back its thread slot. Closing turns its status into
//! `Shutdown` and removes its thread, so the completion watcher must have read its final
//! status and told the parent first, and `wait_agent` must still find the result afterwards.

use super::LocalAgentControl;
use codex_protocol::ThreadId;
use codex_protocol::protocol::AgentStatus;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::PoisonError;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::time::Instant;

#[derive(Default)]
pub(crate) struct BackgroundCloses {
    /// Agents whose completion watcher has finished, until a close claims them.
    noticed: Mutex<HashSet<ThreadId>>,
    noticed_changed: Notify,
    /// The final status of each agent closed after finishing in the background.
    results: Mutex<HashMap<ThreadId, AgentStatus>>,
}

impl LocalAgentControl {
    /// Records that the completion watcher of `thread_id` has done its work.
    pub(crate) fn completion_watcher_finished(&self, thread_id: ThreadId) {
        let closes = &self.runtime.background_closes;
        closes
            .noticed
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(thread_id);
        closes.noticed_changed.notify_waiters();
    }

    /// Waits until the completion watcher of `thread_id` has finished, at most `limit`;
    /// whether it did.
    pub(crate) async fn wait_for_completion_watcher(
        &self,
        thread_id: ThreadId,
        limit: Duration,
    ) -> bool {
        let closes = &self.runtime.background_closes;
        let deadline = Instant::now() + limit;
        loop {
            // Registered before the check, so that a notice in between still wakes it
            // (`notify_waiters` wakes only futures already enabled).
            let changed = closes.noticed_changed.notified();
            tokio::pin!(changed);
            changed.as_mut().enable();
            let noticed = closes
                .noticed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .remove(&thread_id);
            if noticed {
                return true;
            }
            if tokio::time::timeout_at(deadline, changed).await.is_err() {
                return false;
            }
        }
    }

    /// Keeps the final status of `thread_id`, about to be closed, for `wait_agent`.
    pub(crate) fn record_closed_result(&self, thread_id: ThreadId, status: AgentStatus) {
        self.runtime
            .background_closes
            .results
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(thread_id, status);
    }

    /// The final status of `thread_id` if it was closed after finishing in the background.
    pub(crate) fn closed_result(&self, thread_id: ThreadId) -> Option<AgentStatus> {
        self.runtime
            .background_closes
            .results
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&thread_id)
            .cloned()
    }
}

#[cfg(test)]
#[path = "background_close_tests.rs"]
mod tests;
