// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use super::RunningWork;
use crate::app::agents_overview::tests::overview_thread;
use crate::app::tests::make_test_app_with_channels;
use crate::app_event::AppEvent;
use crate::app_event::ExitMode;
use assert_matches::assert_matches;
use codex_app_server_protocol::ThreadStatus;
use codex_protocol::ThreadId;
use pretty_assertions::assert_eq;

#[test]
fn running_work_is_listed_in_order() {
    assert!(RunningWork::default().is_empty());
    let work = RunningWork {
        turn: true,
        sessions: 1,
        side_conversations: 0,
        background_terminals: 2,
    };
    assert!(!work.is_empty());
    assert_eq!(
        work.describe(),
        "the turn in progress, 1 other session or subagent, 2 background terminals"
    );
}

#[tokio::test]
async fn quits_at_once_when_nothing_runs() {
    let (mut app, mut rx, _op_rx) = make_test_app_with_channels().await;
    app.exit_from_prompt().await;
    assert_matches!(rx.try_recv(), Ok(AppEvent::Exit(ExitMode::ShutdownFirst)));
}

#[tokio::test]
async fn asks_first_while_another_session_runs() {
    let (mut app, mut rx, _op_rx) = make_test_app_with_channels().await;
    let id = ThreadId::from_u128(/*value*/ 7);
    app.agents_overview.threads.insert(
        id,
        Some(overview_thread(
            id,
            /*parent_thread_id*/ None,
            "worker",
            ThreadStatus::Active {
                active_flags: vec![],
            },
        )),
    );
    app.exit_from_prompt().await;
    assert!(rx.try_recv().is_err(), "nothing is sent before the answer");
    assert!(!app.chat_widget.no_modal_or_popup_active());

    let idle = ThreadId::from_u128(/*value*/ 8);
    app.agents_overview.threads.clear();
    app.agents_overview.threads.insert(
        idle,
        Some(overview_thread(
            idle,
            /*parent_thread_id*/ None,
            "done",
            ThreadStatus::Idle,
        )),
    );
    assert_eq!(app.running_work().await, RunningWork::default());
}
