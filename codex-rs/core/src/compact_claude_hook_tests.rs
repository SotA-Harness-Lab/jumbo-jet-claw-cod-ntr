// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! A compaction hands its summary to the `PostCompact` hooks of Claude Code's settings.

use super::*;
use crate::session::tests::make_session_and_context_with_auth_and_config_and_rx;
use codex_login::CodexAuth;
use codex_model_provider_info::ModelProviderInfo;
use codex_utils_absolute_path::AbsolutePathBuf;
use core_test_support::responses;
use core_test_support::skip_if_no_network;
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_compaction_gives_its_summary_to_a_post_compact_hook_of_claude_settings()
-> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let server = responses::start_mock_server().await;
    let provider = ModelProviderInfo::create_openai_provider(Some(format!("{}/v1", server.uri())));
    let (session, turn, _events) = make_session_and_context_with_auth_and_config_and_rx(
        CodexAuth::create_dummy_chatgpt_auth_for_testing(),
        Vec::new(),
        move |config| {
            config.model = Some("gpt-5.2".to_string());
            config.model_provider = provider;
            config.model_provider.supports_websockets = false;
        },
    )
    .await;

    let dir = tempfile::tempdir()?;
    let claude_dir = dir.path().join("home/.claude");
    std::fs::create_dir_all(&claude_dir)?;
    let saved = dir.path().join("post_compact.json");
    let settings = serde_json::json!({
        "hooks": {
            "PostCompact": [{"hooks": [
                {"type": "command", "command": format!("cat > '{}'", saved.display())}
            ]}],
        }
    });
    std::fs::write(claude_dir.join("settings.json"), settings.to_string())?;
    let hooks = session.hooks().reconfigured(codex_hooks::HooksConfig {
        feature_enabled: true,
        claude_settings: Some(codex_hooks::ClaudeHookSettings {
            user_dir: Some(AbsolutePathBuf::from_absolute_path_checked(&claude_dir)?),
            home_dir: None,
            project_dir: None,
            entrypoint: Some("cli".to_string()),
            plugins: Vec::new(),
        }),
        ..codex_hooks::HooksConfig::default()
    });
    session.services.hooks.store(Arc::new(hooks));

    responses::mount_sse_sequence(
        &server,
        vec![responses::sse(vec![
            responses::ev_assistant_message("summary", "The prior calls finished."),
            responses::ev_completed("compact-response"),
        ])],
    )
    .await;
    let step = session
        .capture_step_context(turn, &tokio_util::sync::CancellationToken::new())
        .await?;
    let world_state = Arc::new(
        session
            .build_world_state_for_step(&step, /*new_window*/ true)
            .await?,
    );
    run_compact_task(
        Arc::clone(&session),
        step,
        world_state,
        vec![ResolvedUserInput::text(
            "Summarize the conversation.".to_string(),
        )],
    )
    .await?;

    let input: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&saved)?)?;
    assert_eq!(input["hook_event_name"], "PostCompact");
    assert_eq!(input["trigger"], "manual");
    assert_eq!(input["compact_summary"], "The prior calls finished.");
    Ok(())
}
