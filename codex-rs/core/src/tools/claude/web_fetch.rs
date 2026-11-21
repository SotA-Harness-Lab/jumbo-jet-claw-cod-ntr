// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `WebFetch`: Claude Code's URL fetching tool. Codex has no such tool, so this one is new;
//! it is built from Codex's parts wherever they exist.
//!
//! Delegated: the request runs as a `curl` command through Codex's `exec_command` runtime,
//! so the sandbox's network rule, the managed network proxy and its host approvals, command
//! rules, approval prompts, events and the TUI are Codex's, exactly as for a command the
//! model runs. Where the sandbox has no network and no managed proxy stands in for it, the
//! command asks to run outside the sandbox, which Codex approves or refuses under its
//! approval policy. The answer to the prompt comes from the session's model client, as a
//! one-shot request without tools. Added: Claude Code's parameters, URL rules (HTTPS
//! upgrade, same-site redirects followed and others reported, size and time limits), HTML
//! to Markdown, the 15-minute cache, the secondary model's prompt and the result text.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use codex_claude_compat::tools::BASH;
use codex_claude_compat::tools::WEB_FETCH;
use codex_claude_compat::tools::web_fetch;
use codex_claude_compat::tools::web_fetch::CommandShell;
use codex_claude_compat::tools::web_fetch::FetchCache;
use codex_claude_compat::tools::web_fetch::FetchedContent;
use codex_protocol::error::CodexErr;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ReasoningEffort;
use codex_rollout_trace::InferenceTraceContext;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;
use url::Url;

use super::OFFERED_TOOLS;
use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::delegated;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use crate::Prompt;
use crate::client_common::ResponseEvent;
use crate::function_tool::FunctionCallError;
use crate::responses_metadata::CodexResponsesRequestKind;
use crate::session::session::Session;
use crate::shell::ShellType;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::ExecCommandHandler;
use crate::tools::handlers::ExecCommandHandlerOptions;
use crate::tools::handlers::selected_executor;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `WebFetch` when the model may run commands (`Bash` is offered), since the request
/// is one.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if !swap.has(BASH) {
        return;
    }
    swap.add(WebFetchHandler {
        exec: ExecCommandHandler::one_shot(ExecCommandHandlerOptions::default()),
    });
}

struct WebFetchHandler {
    /// Runs the request command to completion.
    exec: ExecCommandHandler,
}

#[derive(Deserialize)]
struct WebFetchArgs {
    url: String,
    prompt: String,
}

/// Claude Code's cache of fetched pages, one per session.
#[derive(Default)]
struct WebFetchCache(Mutex<FetchCache>);

impl WebFetchCache {
    fn of(session: &Session) -> Arc<Self> {
        session
            .services
            .session_extension_data
            .get_or_init::<WebFetchCache>(WebFetchCache::default)
    }

    fn get(&self, url: &str) -> Option<FetchedContent> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(url, Instant::now())
    }

    fn insert(&self, url: String, content: FetchedContent) {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(url, content, Instant::now());
    }
}

enum Fetched {
    Content(FetchedContent),
    /// A redirect to another host, which Claude Code reports instead of following.
    Redirect {
        original: String,
        redirect: String,
        status: u16,
    },
}

impl ToolExecutor<ToolInvocation> for WebFetchHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(WEB_FETCH)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "url".to_string(),
                JsonSchema::string(Some(web_fetch::URL_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "prompt".to_string(),
                JsonSchema::string(Some(web_fetch::PROMPT_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: WEB_FETCH.to_string(),
            description: web_fetch::description(OFFERED_TOOLS),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["url".to_string(), "prompt".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for WebFetchHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl WebFetchHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let start = Instant::now();
        let args: WebFetchArgs = parse_claude_arguments(WEB_FETCH, &invocation)?;
        if Url::parse(&args.url).is_err() {
            return Err(rejected(&web_fetch::invalid_url_message(&args.url)));
        }
        let Some(url) = web_fetch::validate_url(&args.url) else {
            return Err(failed("Invalid URL"));
        };
        let _permission =
            check_permission(&invocation, WEB_FETCH, PermissionInput::Url(&args.url)).await?;

        let cache = WebFetchCache::of(&invocation.session);
        let page = match cache.get(&args.url) {
            Some(page) => page,
            None => match self
                .fetch(&invocation, web_fetch::upgrade_to_https(url.clone()))
                .await?
            {
                Fetched::Content(page) => {
                    cache.insert(args.url.clone(), page.clone());
                    page
                }
                Fetched::Redirect {
                    original,
                    redirect,
                    status,
                } => {
                    let message =
                        web_fetch::redirect_message(&original, &redirect, status, &args.prompt);
                    let tool_response = json!({
                        "bytes": message.len(),
                        "code": status,
                        "codeText": web_fetch::redirect_status_text(status),
                        "result": message,
                        "durationMs": elapsed_ms(start),
                        "url": args.url,
                    });
                    return Ok(boxed_tool_output(ClaudeToolOutput::new(
                        message,
                        tool_response,
                    )));
                }
            },
        };

        let preapproved = web_fetch::is_preapproved(&url);
        let result = if web_fetch::returns_content_unprocessed(
            preapproved,
            &page.content_type,
            &page.content,
        ) {
            page.content.clone()
        } else {
            let prompt =
                web_fetch::secondary_model_prompt(&page.content, &args.prompt, preapproved);
            answer_prompt(&invocation, prompt).await?
        };

        let tool_response = json!({
            "bytes": page.bytes,
            "code": page.code,
            "codeText": page.code_text,
            "result": result,
            "durationMs": elapsed_ms(start),
            "url": args.url,
        });
        Ok(boxed_tool_output(
            ClaudeToolOutput::new(result, tool_response).with_external_context(),
        ))
    }

    /// Fetches `target` with `curl` through `exec_command`, following same-site redirects.
    async fn fetch(
        &self,
        invocation: &ToolInvocation,
        mut target: Url,
    ) -> Result<Fetched, FunctionCallError> {
        let environment = selected_executor(
            &invocation.environments,
            &invocation.step_context.environments,
        )?;
        let shell = match environment.shell.as_ref().map(|shell| shell.shell_type) {
            Some(ShellType::PowerShell) => CommandShell::PowerShell,
            Some(ShellType::Cmd) => CommandShell::Cmd,
            _ => CommandShell::Posix,
        };
        // With a managed proxy, the sandbox lets the command reach the proxy, which applies
        // the network policy and asks for unknown hosts; escalating would bypass it. Without
        // one, a sandbox without network can only be left with the user's approval.
        let escalate = !invocation.turn.network_sandbox_policy().is_enabled()
            && invocation.turn.network.is_none();

        for _ in 0..=web_fetch::MAX_REDIRECTS {
            let mut arguments = json!({
                "cmd": web_fetch::fetch_command(&target, shell),
                "timeout_ms": (web_fetch::FETCH_TIMEOUT_SECS + 10) * 1000,
                "max_output_tokens": crate::unified_exec::UNIFIED_EXEC_OUTPUT_MAX_TOKENS,
            });
            if escalate {
                arguments["sandbox_permissions"] = json!("require_escalated");
                arguments["justification"] = json!(format!("{WEB_FETCH}: fetch {target}"));
            }
            let payload = ToolPayload::Function {
                arguments: arguments.to_string(),
            };
            let output = self
                .exec
                .handle(delegated(invocation, payload.clone()))
                .await?;
            let result = output.code_mode_result(&payload);
            let text = result
                .get("output")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let exit_code = result
                .get("exit_code")
                .and_then(Value::as_i64)
                .and_then(|code| i32::try_from(code).ok());
            let response = match exit_code {
                Some(0) => web_fetch::parse_fetch_output(text),
                _ => None,
            };
            let Some(response) = response else {
                return Err(failed(web_fetch::command_error_message(exit_code, text)));
            };

            if web_fetch::is_redirect(response.status) {
                let Ok(redirect) = Url::parse(response.redirect_url) else {
                    return Err(failed("Redirect missing Location header"));
                };
                if web_fetch::is_permitted_redirect(&target, &redirect) {
                    target = redirect;
                    continue;
                }
                return Ok(Fetched::Redirect {
                    original: target.to_string(),
                    redirect: redirect.to_string(),
                    status: response.status,
                });
            }
            if !(200..300).contains(&response.status) {
                return Err(failed(web_fetch::status_error_message(response.status)));
            }
            let content =
                convert_page(response.body.to_string(), response.content_type.to_string()).await?;
            return Ok(Fetched::Content(FetchedContent {
                bytes: response.bytes,
                code: response.status,
                code_text: web_fetch::status_text(response.status).to_string(),
                content,
                content_type: response.content_type.to_string(),
            }));
        }
        Err(failed(format!(
            "Too many redirects (exceeded {})",
            web_fetch::MAX_REDIRECTS
        )))
    }
}

/// Converts a page to Markdown on a blocking thread, so that a large page does not hold up
/// the async workers.
async fn convert_page(body: String, content_type: String) -> Result<String, FunctionCallError> {
    tokio::task::spawn_blocking(move || web_fetch::to_markdown(&body, &content_type))
        .await
        .map_err(|err| failed(format!("Converting the page failed: {err}")))
}

fn elapsed_ms(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX)
}

/// Runs Claude Code's secondary model step: one request without tools to the session's
/// model, at low reasoning effort where the model has it, with `prompt` as the only input.
async fn answer_prompt(
    invocation: &ToolInvocation,
    prompt: String,
) -> Result<String, FunctionCallError> {
    let turn = &invocation.turn;
    let model_info = turn.model_info();
    let request = Arc::new(Prompt {
        input: vec![ResponseItem::Message {
            encrypted_content: None,
            status: None,
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText { text: prompt }],
            phase: None,
            internal_chat_message_metadata_passthrough: None,
        }],
        ..Default::default()
    });
    let effort = model_info
        .supported_reasoning_levels
        .iter()
        .any(|preset| preset.effort == ReasoningEffort::Low)
        .then_some(ReasoningEffort::Low);
    let metadata = invocation
        .session
        .responses_metadata(&invocation.step_context, CodexResponsesRequestKind::Turn)
        .await;
    let mut client = invocation.session.services.model_client.new_session();
    let answer = async {
        let mut stream = client
            .stream(
                &request,
                model_info,
                &turn.session_telemetry,
                effort,
                turn.reasoning_summary(),
                turn.config.service_tier.clone(),
                &metadata,
                &InferenceTraceContext::disabled(),
            )
            .await?;
        let mut text = String::new();
        while let Some(event) = stream.next().await {
            match event? {
                ResponseEvent::OutputTextDelta(delta) => text.push_str(&delta),
                ResponseEvent::OutputItemDone(ResponseItem::Message { role, content, .. })
                    if text.is_empty() && role == "assistant" =>
                {
                    if let Some(item_text) = crate::compact::content_items_to_text(&content) {
                        text = item_text;
                    }
                }
                ResponseEvent::Completed { .. } => break,
                _ => {}
            }
        }
        Ok::<_, CodexErr>(text)
    };
    let text = tokio::select! {
        result = answer => result.map_err(|err| failed(err.to_string()))?,
        () = invocation.cancellation_token.cancelled() => {
            return Err(failed("The request was cancelled"));
        }
    };
    if text.trim().is_empty() {
        return Ok(web_fetch::NO_RESPONSE.to_string());
    }
    Ok(text)
}
