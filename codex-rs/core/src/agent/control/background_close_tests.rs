// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use codex_protocol::ThreadId;
use codex_protocol::protocol::AgentStatus;
use pretty_assertions::assert_eq;

use crate::agent::control::LocalAgentControl;

#[tokio::test]
async fn a_watcher_that_finished_first_is_found_at_once() {
    let control = LocalAgentControl::default();
    let thread_id = ThreadId::new();
    control.completion_watcher_finished(thread_id);

    assert!(
        control
            .wait_for_completion_watcher(thread_id, Duration::from_millis(1))
            .await
    );
    // The notice is claimed once.
    assert!(
        !control
            .wait_for_completion_watcher(thread_id, Duration::from_millis(20))
            .await
    );
}

#[tokio::test]
async fn the_close_waits_for_the_watcher() {
    let control = LocalAgentControl::default();
    let thread_id = ThreadId::new();
    let waiter = {
        let control = control.clone();
        tokio::spawn(async move {
            control
                .wait_for_completion_watcher(thread_id, Duration::from_secs(5))
                .await
        })
    };
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(!waiter.is_finished());

    // Another agent's notice does not end the wait.
    control.completion_watcher_finished(ThreadId::new());
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(!waiter.is_finished());

    control.completion_watcher_finished(thread_id);
    assert!(waiter.await.expect("waiter"));
}

#[tokio::test]
async fn a_closed_agent_keeps_its_result() {
    let control = LocalAgentControl::default();
    let thread_id = ThreadId::new();
    assert_eq!(control.closed_result(thread_id), None);

    control.record_closed_result(thread_id, AgentStatus::Completed(Some("done".to_string())));
    assert_eq!(
        control.closed_result(thread_id),
        Some(AgentStatus::Completed(Some("done".to_string())))
    );
}
