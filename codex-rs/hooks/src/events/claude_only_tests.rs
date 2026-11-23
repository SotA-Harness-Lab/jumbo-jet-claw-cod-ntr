// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use codex_protocol::ThreadId;
use codex_protocol::protocol::CodexErrorInfo;
use codex_utils_absolute_path::AbsolutePathBuf;
use futures::future::BoxFuture;
use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;
use tempfile::TempDir;

use super::ClaudeOnlyEvent;
use super::NotificationRequest;
use super::PostToolUseFailureRequest;
use super::StopFailureRequest;
use super::post_tool_use_failure_context;
use super::stop_failure_error;
use crate::ClaudeHookSettings;
use crate::EnvironmentHookOutput;
use crate::HookEnvironmentExecutor;
use crate::HookJudgeRequest;
use crate::engine::ClaudeHooksEngine;
use crate::engine::CommandShell;
use crate::engine::ConfiguredHandlerKind;
use crate::engine::HandlerRunResult;
use crate::engine::command_runner::CommandHookRuntime;
use crate::mcp::HookMcpCall;
use crate::mcp::HookMcpExecutor;
use crate::mcp::HookMcpOutput;

struct NoMcp;

impl HookMcpExecutor for NoMcp {
    fn execute(&self, _call: HookMcpCall) -> BoxFuture<'_, anyhow::Result<HookMcpOutput>> {
        Box::pin(futures::future::ready(Err::<HookMcpOutput, _>(
            anyhow::anyhow!("no MCP servers in these tests"),
        )))
    }
}

#[derive(Default)]
struct HookCalls {
    judges: Mutex<Vec<HookJudgeRequest>>,
    mcp: Mutex<Vec<HookMcpCall>>,
}

impl HookEnvironmentExecutor for HookCalls {
    fn execute(
        &self,
        _environment_id: &str,
        _command: &str,
        _env: &HashMap<String, String>,
        _input_json: &str,
        _timeout: Duration,
    ) -> BoxFuture<'_, anyhow::Result<EnvironmentHookOutput>> {
        Box::pin(async { anyhow::bail!("unexpected command hook") })
    }

    fn judge(&self, request: HookJudgeRequest) -> BoxFuture<'_, anyhow::Result<String>> {
        self.judges.lock().expect("judge calls").push(request);
        Box::pin(async { Ok(r#"{"ok":false,"reason":"inspect the failure"}"#.to_string()) })
    }
}

impl HookMcpExecutor for HookCalls {
    fn execute(&self, call: HookMcpCall) -> BoxFuture<'_, anyhow::Result<HookMcpOutput>> {
        self.mcp.lock().expect("MCP calls").push(call);
        Box::pin(async {
            Ok(HookMcpOutput {
                text: json!({
                    "hookSpecificOutput": {
                        "hookEventName": "PostToolUseFailure",
                        "additionalContext": "check the generated report"
                    }
                })
                .to_string(),
                structured_content: None,
                source_tool_namespace: None,
            })
        })
    }
}

fn absolute(path: &Path) -> AbsolutePathBuf {
    AbsolutePathBuf::from_absolute_path_checked(path).expect("absolute path")
}

fn engine(tmp: &TempDir, bypass_hook_trust: bool) -> ClaudeHooksEngine {
    let (result_sender, _result_receiver) = async_channel::unbounded();
    let runtime = CommandHookRuntime::new(
        CommandShell {
            program: String::new(),
            args: Vec::new(),
        },
        Arc::new(std::env::vars_os().collect()),
        ThreadId::new(),
        result_sender,
    );
    ClaudeHooksEngine::new_with_claude_settings(
        /*enabled*/ true,
        bypass_hook_trust,
        /*config_layer_stack*/ None,
        Vec::new(),
        Vec::new(),
        runtime,
        Arc::new(NoMcp),
        Some(&ClaudeHookSettings {
            user_dir: Some(absolute(&tmp.path().join("home/.claude"))),
            home_dir: Some(absolute(&tmp.path().join("home"))),
            project_dir: Some(absolute(&tmp.path().join("project"))),
            entrypoint: Some("cli".to_string()),
            plugins: Vec::new(),
        }),
    )
}

/// Writes the user's settings: each Claude-only event's hook saves its stdin to
/// `<event>.json` in the temp dir.
fn write_user_settings(tmp: &TempDir) {
    let dir = tmp.path().display().to_string();
    let save = |name: &str| format!("cat > '{dir}/{name}.json'");
    let context = json!({
        "hookSpecificOutput": {
            "hookEventName": "PostToolUseFailure",
            "additionalContext": "look at the link first",
        }
    })
    .to_string();
    let settings = json!({
        "hooks": {
            "PostToolUse": [{"matcher": "Bash", "hooks": [
                {"type": "command", "command": "true"}
            ]}],
            "PostToolUseFailure": [{"matcher": "Bash", "hooks": [
                {"type": "command", "command": format!("{}; printf '%s' '{context}'", save("failure"))}
            ]}],
            "Notification": [{"matcher": "permission_prompt|elicitation_dialog", "hooks": [
                {"type": "command", "command": save("notification")}
            ]}],
            "StopFailure": [{"hooks": [
                {"type": "command", "command": save("stop_failure")}
            ]}],
        }
    });
    fs::create_dir_all(tmp.path().join("home/.claude")).expect("create");
    fs::create_dir_all(tmp.path().join("project")).expect("create");
    fs::write(
        tmp.path().join("home/.claude/settings.json"),
        settings.to_string(),
    )
    .expect("write settings");
}

async fn read_when_written(path: &Path) -> Value {
    for _ in 0..200 {
        if let Ok(contents) = fs::read_to_string(path)
            && let Ok(value) = serde_json::from_str(&contents)
        {
            return value;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("{} was not written", path.display());
}

#[tokio::test]
async fn post_tool_use_failure_hooks_get_claude_input_and_give_context() {
    let tmp = TempDir::new().expect("temp dir");
    write_user_settings(&tmp);
    let engine = engine(&tmp, /*bypass_hook_trust*/ false);
    assert_eq!(engine.claude_only_handlers.len(), 3);
    let session_id = ThreadId::new();
    let request = |tool_name: &str| PostToolUseFailureRequest {
        session_id,
        cwd: absolute(tmp.path()),
        transcript_path: None,
        permission_mode: "default".to_string(),
        tool_name: tool_name.to_string(),
        matcher_aliases: Vec::new(),
        tool_use_id: "call-1".to_string(),
        tool_input: json!({"command": "cargo build"}),
        error: "Exit code 101\nNot a directory (os error 20)".to_string(),
        is_interrupt: false,
    };

    let outcome = engine.run_post_tool_use_failure(request("Bash")).await;

    assert_eq!(
        outcome
            .additional_contexts
            .iter()
            .map(|context| context.text.as_str())
            .collect::<Vec<_>>(),
        vec!["look at the link first"]
    );
    let input: Value = serde_json::from_str(
        &fs::read_to_string(tmp.path().join("failure.json")).expect("hook stdin"),
    )
    .expect("json");
    assert_eq!(
        input,
        json!({
            "session_id": session_id.to_string(),
            "transcript_path": null,
            "cwd": tmp.path().display().to_string(),
            "permission_mode": "default",
            "hook_event_name": "PostToolUseFailure",
            "tool_name": "Bash",
            "tool_input": {"command": "cargo build"},
            "tool_use_id": "call-1",
            "error": "Exit code 101\nNot a directory (os error 20)",
            "is_interrupt": false,
        })
    );

    let other = engine
        .run_post_tool_use_failure(request("apply_patch"))
        .await;
    assert!(other.additional_contexts.is_empty());
}

#[tokio::test]
async fn failure_hooks_dispatch_judges_and_mcp_and_apply_http_policy() {
    let tmp = TempDir::new().expect("temp dir");
    let root = tmp.path().join("home/.claude");
    fs::create_dir_all(&root).expect("create settings directory");
    let http = json!({"type":"http", "url":"http://127.0.0.1:1/hook"});
    let mcp = json!({"type":"mcp_tool", "server":"report", "tool":"inspect", "input":{}});
    fs::write(
        root.join("settings.json"),
        json!({
            "allowedHttpHookUrls": [],
            "hooks": {
                "Notification": [{"hooks": [http, mcp]}],
                "StopFailure": [{"hooks": [http, mcp]}],
                "PostToolUseFailure": [{"matcher":"Bash", "hooks": [
                    {"type":"prompt", "prompt":"Review $ARGUMENTS", "model":"haiku"},
                    {"type":"agent", "prompt":"Review $ARGUMENTS"},
                    http,
                    mcp
                ]}]
            }
        })
        .to_string(),
    )
    .expect("write settings");
    let mut engine = engine(&tmp, /*bypass_hook_trust*/ false);
    assert!(engine.warnings().is_empty(), "{:?}", engine.warnings());
    for event in [ClaudeOnlyEvent::Notification, ClaudeOnlyEvent::StopFailure] {
        let handlers = engine
            .claude_only_handlers
            .iter()
            .filter(|entry| entry.event == event)
            .map(|entry| &entry.handler.kind)
            .collect::<Vec<_>>();
        assert!(matches!(
            handlers.as_slice(),
            [
                ConfiguredHandlerKind::Http(_),
                ConfiguredHandlerKind::McpTool { .. }
            ]
        ));
    }
    assert!(engine.claude_only_handlers.iter().any(|entry| {
        entry.event == ClaudeOnlyEvent::PostToolUseFailure
            && matches!(&entry.handler.kind, ConfiguredHandlerKind::Http(hook) if hook.policy.allowed_urls == Some(Vec::new()))
    }));

    let calls = Arc::new(HookCalls::default());
    engine.command_runtime.environment_executor = Some(calls.clone());
    engine.mcp_executor = calls.clone();
    let outcome = engine
        .run_post_tool_use_failure(PostToolUseFailureRequest {
            session_id: ThreadId::new(),
            cwd: absolute(tmp.path()),
            transcript_path: None,
            permission_mode: "default".to_string(),
            tool_name: "Bash".to_string(),
            matcher_aliases: Vec::new(),
            tool_use_id: "call-1".to_string(),
            tool_input: json!({"command":"cargo test"}),
            error: "Exit code 1".to_string(),
            is_interrupt: false,
        })
        .await;
    assert_eq!(
        outcome
            .additional_contexts
            .iter()
            .map(|context| context.text.as_str())
            .collect::<Vec<_>>(),
        [
            "Prompt hook condition was not met: inspect the failure",
            "Agent hook condition was not met: inspect the failure",
            "check the generated report"
        ]
    );
    let judges = calls.judges.lock().expect("judge calls");
    assert_eq!(judges.len(), 2);
    assert!(judges.iter().all(|call| {
        call.prompt
            .contains(r#""hook_event_name":"PostToolUseFailure""#)
    }));
    assert!(
        judges
            .iter()
            .any(|call| !call.agentic && call.model.as_deref() == Some("haiku"))
    );
    assert!(judges.iter().any(|call| call.agentic));
    assert_eq!(calls.mcp.lock().expect("MCP calls")[0].server, "report");
}

#[tokio::test]
async fn notification_and_stop_failure_hooks_run_in_the_background() {
    let tmp = TempDir::new().expect("temp dir");
    write_user_settings(&tmp);
    let engine = engine(&tmp, /*bypass_hook_trust*/ false);
    let session_id = ThreadId::new();

    assert!(
        engine
            .select_claude_only(
                ClaudeOnlyEvent::Notification,
                &["idle_prompt"],
                /*tool_call*/ None,
            )
            .is_empty()
    );
    engine.spawn_notification(NotificationRequest {
        session_id,
        cwd: absolute(tmp.path()),
        transcript_path: Some(tmp.path().join("rollout.jsonl")),
        permission_mode: "default".to_string(),
        notification_type: "permission_prompt".to_string(),
        message: "Codex needs your permission to use Bash".to_string(),
        title: None,
    });
    engine.spawn_stop_failure(StopFailureRequest {
        session_id,
        cwd: absolute(tmp.path()),
        transcript_path: None,
        permission_mode: "default".to_string(),
        error: "rate_limit".to_string(),
        error_details: Some("Rate limit reached".to_string()),
        last_assistant_message: None,
    });

    assert_eq!(
        read_when_written(&tmp.path().join("notification.json")).await,
        json!({
            "session_id": session_id.to_string(),
            "transcript_path": tmp.path().join("rollout.jsonl").display().to_string(),
            "cwd": tmp.path().display().to_string(),
            "permission_mode": "default",
            "hook_event_name": "Notification",
            "message": "Codex needs your permission to use Bash",
            "notification_type": "permission_prompt",
        })
    );
    assert_eq!(
        read_when_written(&tmp.path().join("stop_failure.json")).await,
        json!({
            "session_id": session_id.to_string(),
            "transcript_path": null,
            "cwd": tmp.path().display().to_string(),
            "permission_mode": "default",
            "hook_event_name": "StopFailure",
            "error": "rate_limit",
            "error_details": "Rate limit reached",
        })
    );
}

#[test]
fn failed_shell_commands_leave_out_claude_settings_post_tool_use_hooks() {
    let tmp = TempDir::new().expect("temp dir");
    write_user_settings(&tmp);
    let engine = engine(&tmp, /*bypass_hook_trust*/ false);

    assert_eq!(
        engine
            .post_tool_use_handlers(/*skip_claude_settings*/ false)
            .len(),
        1
    );
    assert!(
        engine
            .post_tool_use_handlers(/*skip_claude_settings*/ true)
            .is_empty()
    );
}

#[test]
fn claude_only_handlers_get_claude_code_environment() {
    let tmp = TempDir::new().expect("temp dir");
    write_user_settings(&tmp);
    let engine = engine(&tmp, /*bypass_hook_trust*/ false);

    let ConfiguredHandlerKind::Command { env, .. } = &engine.claude_only_handlers[0].handler.kind
    else {
        panic!("expected a command handler");
    };
    assert_eq!(
        env,
        &HashMap::from([
            (
                "CLAUDE_PROJECT_DIR".to_string(),
                tmp.path().join("project").display().to_string()
            ),
            ("CLAUDE_CODE_ENTRYPOINT".to_string(), "cli".to_string()),
        ])
    );
}

#[test]
fn project_claude_only_hooks_need_the_trust_bypass() {
    let tmp = TempDir::new().expect("temp dir");
    fs::create_dir_all(tmp.path().join("project/.claude")).expect("create");
    fs::write(
        tmp.path().join("project/.claude/settings.json"),
        r#"{"hooks": {"Notification": [{"hooks": [{"type": "command", "command": "true"}]}]}}"#,
    )
    .expect("write settings");

    let untrusted = engine(&tmp, /*bypass_hook_trust*/ false);
    assert!(untrusted.claude_only_handlers.is_empty());
    assert!(
        untrusted
            .warnings()
            .iter()
            .any(|warning| warning.contains("skipping Claude Code Notification hooks")),
        "{:?}",
        untrusted.warnings()
    );

    let bypassed = engine(&tmp, /*bypass_hook_trust*/ true);
    assert_eq!(bypassed.claude_only_handlers.len(), 1);
    assert_eq!(
        bypassed.claude_only_handlers[0].event,
        ClaudeOnlyEvent::Notification
    );
}

#[test]
fn turn_errors_map_to_claude_code_error_kinds() {
    assert_eq!(
        [
            CodexErrorInfo::RateLimitExceeded,
            CodexErrorInfo::UsageLimitExceeded,
            CodexErrorInfo::Unauthorized,
            CodexErrorInfo::ContextWindowExceeded,
            CodexErrorInfo::ServerOverloaded,
            CodexErrorInfo::ResponseStreamDisconnected {
                http_status_code: None
            },
            CodexErrorInfo::SandboxError,
        ]
        .iter()
        .map(stop_failure_error)
        .collect::<Vec<_>>(),
        vec![
            "rate_limit",
            "billing_error",
            "authentication_failed",
            "invalid_request",
            "server_error",
            "server_error",
            "unknown",
        ]
    );
}

#[test]
fn post_tool_use_failure_context_comes_from_json_or_exit_code_two() {
    let result = |exit_code: Option<i32>, stdout: &str, stderr: &str| HandlerRunResult {
        structured_content: None,
        context_metadata: codex_protocol::models::ContentItemMetadata::harness(),
        started_at: 0,
        completed_at: 0,
        duration_ms: 0,
        exit_code,
        stdout: stdout.to_string(),
        stderr: stderr.to_string(),
        error: None,
    };
    let json = r#"{"hookSpecificOutput":{"hookEventName":"PostToolUseFailure","additionalContext":"retry"}}"#;

    assert_eq!(
        post_tool_use_failure_context(&result(Some(0), json, "")),
        Some("retry".to_string())
    );
    assert_eq!(
        post_tool_use_failure_context(&result(Some(2), "", "check the path\n")),
        Some("check the path".to_string())
    );
    assert_eq!(
        post_tool_use_failure_context(&result(Some(0), "plain text", "")),
        None
    );
    assert_eq!(
        post_tool_use_failure_context(&result(Some(1), json, "")),
        None
    );
}

#[test]
fn claude_entrypoint_follows_the_session_source() {
    use codex_protocol::protocol::SessionSource;

    assert_eq!(
        crate::claude_entrypoint(&SessionSource::Cli).as_deref(),
        Some("cli")
    );
    assert_eq!(
        crate::claude_entrypoint(&SessionSource::Exec).as_deref(),
        Some("sdk-cli")
    );
    assert_eq!(crate::claude_entrypoint(&SessionSource::Mcp), None);
}
