// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use codex_login::CodexAuth;
use codex_model_provider_info::built_in_model_providers;
use codex_protocol::protocol::AgentStatus;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::Op;
use codex_protocol::protocol::TurnCompleteEvent;

use super::close_when_finished;
use crate::StartThreadOptions;
use crate::ThreadManager;
use crate::session::tests::make_session_and_context;

#[tokio::test]
async fn a_finished_background_agent_gives_back_its_thread() {
    let (_session, turn) = make_session_and_context().await;
    let manager = ThreadManager::with_models_provider_for_tests(
        CodexAuth::from_api_key("dummy"),
        built_in_model_providers(/*openai_base_url*/ None)["openai"].clone(),
    );
    let agent = manager
        .start_thread(StartThreadOptions::new(turn.config.as_ref().clone()))
        .await
        .expect("start thread");
    let thread_id = agent.thread_id;
    close_when_finished(manager.agent_control(), thread_id);
    assert_ne!(
        manager.agent_control().get_status(thread_id).await,
        AgentStatus::NotFound
    );

    let agent_turn = agent.thread.session.new_default_turn().await;
    agent
        .thread
        .session
        .send_event(
            agent_turn.as_ref(),
            EventMsg::TurnComplete(TurnCompleteEvent {
                root_turn_id: None,
                turn_id: agent_turn.sub_id.clone(),
                started_at: None,
                last_agent_message: Some("done".to_string()),
                error: None,
                completed_at: None,
                duration_ms: None,
                time_to_first_token_ms: None,
            }),
        )
        .await;

    tokio::time::timeout(Duration::from_secs(5), async {
        while manager.agent_control().get_status(thread_id).await != AgentStatus::NotFound {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("the finished agent should be closed");
    assert!(
        manager
            .captured_ops()
            .iter()
            .any(|(id, op)| *id == thread_id && matches!(op, Op::Shutdown))
    );
}
