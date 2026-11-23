// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `idle_prompt` notification after a turn.

use super::*;
use crate::session::idle_prompt::IDLE_PROMPT_MESSAGE;
use crate::session::idle_prompt::sends_idle_prompt;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use std::path::Path;
use std::path::PathBuf;

/// Gives the session a `Notification` hook of the user's Claude Code settings that matches
/// `idle_prompt` and saves its stdin to the returned file.
fn install_idle_prompt_hook(session: &Session, dir: &Path) -> PathBuf {
    let claude_dir = dir.join("home/.claude");
    std::fs::create_dir_all(&claude_dir).expect("create the Claude directory");
    let saved = dir.join("notification.json");
    let settings = serde_json::json!({
        "hooks": {
            "Notification": [{"matcher": "idle_prompt", "hooks": [
                {"type": "command", "command": format!("cat > '{}'", saved.display())}
            ]}],
        }
    });
    std::fs::write(claude_dir.join("settings.json"), settings.to_string())
        .expect("write the settings");
    let hooks = session.hooks().reconfigured(codex_hooks::HooksConfig {
        feature_enabled: true,
        claude_settings: Some(codex_hooks::ClaudeHookSettings {
            user_dir: Some(
                AbsolutePathBuf::from_absolute_path_checked(&claude_dir).expect("absolute path"),
            ),
            home_dir: None,
            project_dir: None,
            entrypoint: Some("cli".to_string()),
            plugins: Vec::new(),
        }),
        ..codex_hooks::HooksConfig::default()
    });
    assert!(hooks.has_notification_hooks());
    session.services.hooks.store(Arc::new(hooks));
    saved
}

async fn read_when_written(path: &Path) -> serde_json::Value {
    for _ in 0..200 {
        if let Ok(contents) = std::fs::read_to_string(path)
            && let Ok(value) = serde_json::from_str(&contents)
        {
            return value;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("{} was not written", path.display());
}

#[test]
fn idle_prompt_is_sent_only_from_attended_sessions() {
    assert!(sends_idle_prompt(&SessionSource::Cli));
    assert!(sends_idle_prompt(&SessionSource::VSCode));
    assert!(!sends_idle_prompt(&SessionSource::Exec));
    assert!(!sends_idle_prompt(&SessionSource::SubAgent(
        SubAgentSource::Review
    )));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn idle_prompt_runs_the_notification_hooks_after_the_wait() {
    let (session, turn, _rx) = make_session_and_context_with_rx().await;
    let dir = tempfile::tempdir().expect("temp dir");
    let saved = install_idle_prompt_hook(&session, dir.path());

    session
        .start_idle_prompt_wait_after(Arc::clone(&turn), Duration::from_millis(50))
        .await;

    let input = read_when_written(&saved).await;
    assert_eq!(
        (
            input["hook_event_name"].clone(),
            input["notification_type"].clone(),
            input["message"].clone(),
        ),
        (
            serde_json::json!("Notification"),
            serde_json::json!("idle_prompt"),
            serde_json::json!(IDLE_PROMPT_MESSAGE),
        )
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_new_turn_cancels_the_idle_prompt() {
    let (session, turn, _rx) = make_session_and_context_with_rx().await;
    let dir = tempfile::tempdir().expect("temp dir");
    let saved = install_idle_prompt_hook(&session, dir.path());

    session
        .start_idle_prompt_wait_after(Arc::clone(&turn), Duration::from_millis(500))
        .await;
    // The user starts another turn within the wait and interrupts it: the session is idle
    // again when the wait ends, but the user did not leave it waiting.
    session
        .spawn_task(
            Arc::clone(&turn),
            Vec::new(),
            NeverEndingTask {
                kind: TaskKind::Regular,
                listen_to_cancellation_token: true,
            },
        )
        .await;
    session.abort_all_tasks(TurnAbortReason::Interrupted).await;
    tokio::time::sleep(Duration::from_millis(1500)).await;

    assert_eq!(saved.exists(), false);
}
