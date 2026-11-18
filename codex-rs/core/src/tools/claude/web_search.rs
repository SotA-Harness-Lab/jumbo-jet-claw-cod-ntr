//! `WebSearch`: Claude Code's web search tool.
//!
//! Delegated to Codex's standalone search tool (`web.run`, from the web search extension):
//! the search backend, its settings (`web_search` mode, location, filters), its events and
//! the TUI. Added: Claude Code's parameters (`query`, `allowed_domains` as the search's
//! domain filter, `blocked_domains` as `-site:` operators), its checks and its result text.
//!
//! Codex's other search, the hosted `web_search` tool, runs inside the model provider: the
//! model calls it in its own format and Codex never sees a function call to rename or
//! re-parameterise. When only that one is available it stays as it is (see
//! `docs/claude-compat.md`).

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use codex_claude_compat::tools::WEB_SEARCH;
use codex_claude_compat::tools::web_search;
use codex_protocol::models::ResponseInputItem;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;

use super::PermissionInput;
use super::check_permission;
use super::delegated;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use crate::tools::registry::ToolRegistry;

fn codex_search_tool() -> ToolName {
    ToolName::namespaced("web", "run")
}

/// Offers `WebSearch` in place of Codex's standalone search, when this step has it.
pub(super) fn install(registry: &mut ToolRegistry) {
    let Some(web_run) = registry.remove(&codex_search_tool()) else {
        return;
    };
    registry.add(WebSearchHandler { web_run });
}

struct WebSearchHandler {
    web_run: Arc<dyn CoreToolRuntime>,
}

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
    #[serde(default)]
    allowed_domains: Option<Vec<String>>,
    #[serde(default)]
    blocked_domains: Option<Vec<String>>,
}

impl ToolExecutor<ToolInvocation> for WebSearchHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(WEB_SEARCH)
    }

    fn spec(&self) -> ToolSpec {
        let domains = |description: &str| {
            JsonSchema::array(
                JsonSchema::string(/*description*/ None),
                Some(description.to_string()),
            )
        };
        let properties = BTreeMap::from([
            (
                "query".to_string(),
                JsonSchema::string(Some(web_search::QUERY_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "allowed_domains".to_string(),
                domains(web_search::ALLOWED_DOMAINS_PARAM_DESCRIPTION),
            ),
            (
                "blocked_domains".to_string(),
                domains(web_search::BLOCKED_DOMAINS_PARAM_DESCRIPTION),
            ),
        ]);
        let month_year = chrono::Local::now().format("%B %Y").to_string();
        ToolSpec::Function(ResponsesApiTool {
            name: WEB_SEARCH.to_string(),
            description: web_search::description(&month_year),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["query".to_string()]),
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

impl CoreToolRuntime for WebSearchHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl WebSearchHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let start = Instant::now();
        let args: WebSearchArgs = parse_claude_arguments(WEB_SEARCH, &invocation)?;
        let allowed = args.allowed_domains.as_deref();
        let blocked = args.blocked_domains.as_deref();
        web_search::validate(&args.query, allowed, blocked)
            .map_err(|message| rejected(&message))?;
        let _permission = check_permission(&invocation, WEB_SEARCH, PermissionInput::Tool).await?;

        let payload = ToolPayload::Function {
            arguments: web_search::search_commands(&args.query, allowed, blocked).to_string(),
        };
        // `web.run` checks its own name to take the thread's search reservation.
        let search = ToolInvocation {
            tool_name: codex_search_tool(),
            ..delegated(&invocation, payload.clone())
        };
        let output = self.web_run.handle(search).await?;
        let (text, success) = match output.to_response_item(&invocation.call_id, &payload) {
            ResponseInputItem::FunctionCallOutput { output, .. } => {
                (output.body.to_text().unwrap_or_default(), output.success)
            }
            _ => (String::new(), None),
        };
        if success == Some(false) {
            return Err(failed(text));
        }

        let tool_response = json!({
            "query": args.query,
            "results": [text],
            "durationSeconds": start.elapsed().as_secs_f64(),
        });
        Ok(boxed_tool_output(
            ClaudeToolOutput::new(web_search::render(&args.query, &text), tool_response)
                .with_external_context(),
        ))
    }
}
