//! `TodoWrite`: Claude Code's task list tool.
//!
//! Delegated: the list itself, sent to Codex's `update_plan` runtime as its plan, so that
//! the TUI's plan display, the plan events and the Plan-mode refusal stay Codex's. Added:
//! Claude Code's parameters (`content`, `status`, `activeForm`), its checks, description and
//! result text.

use std::collections::BTreeMap;
use std::sync::Arc;

use codex_claude_compat::tools::TODO_WRITE;
use codex_claude_compat::tools::todo_write;
use codex_claude_compat::tools::todo_write::Todo;
use codex_claude_compat::tools::todo_write::TodoStatus;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;

use super::PermissionInput;
use super::check_permission;
use super::ToolSwap;
use super::delegated;
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

/// Offers `TodoWrite` in place of `update_plan`, when the turn has the plan tool.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    // Claude Code always has TodoWrite, while Codex offers `update_plan` only when
    // `[tools.update_plan]` enables it: build the plan tool's runtime when it is missing, unless
    // the tool policy forbids it.
    let update_plan = swap.take("update_plan").or_else(|| {
        swap.allows("update_plan").then(|| {
            Arc::new(crate::tools::handlers::PlanHandler) as Arc<dyn CoreToolRuntime>
        })
    });
    if let Some(update_plan) = update_plan {
        swap.add(TodoWriteHandler { update_plan });
    }
}

struct TodoWriteHandler {
    update_plan: Arc<dyn CoreToolRuntime>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TodoWriteArgs {
    todos: Vec<TodoArg>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TodoArg {
    content: String,
    status: String,
    #[serde(rename = "activeForm")]
    active_form: String,
}

impl ToolExecutor<ToolInvocation> for TodoWriteHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TODO_WRITE)
    }

    fn spec(&self) -> ToolSpec {
        let item = JsonSchema::object(
            BTreeMap::from([
                ("content".to_string(), JsonSchema::string(None)),
                (
                    "status".to_string(),
                    JsonSchema::string_enum(
                        TodoStatus::ALL.iter().map(|status| json!(status)).collect(),
                        None,
                    ),
                ),
                ("activeForm".to_string(), JsonSchema::string(None)),
            ]),
            Some(vec![
                "content".to_string(),
                "status".to_string(),
                "activeForm".to_string(),
            ]),
            Some(false.into()),
        );
        let properties = BTreeMap::from([(
            "todos".to_string(),
            JsonSchema::array(item, Some(todo_write::TODOS_PARAM_DESCRIPTION.to_string())),
        )]);
        ToolSpec::Function(ResponsesApiTool {
            name: TODO_WRITE.to_string(),
            description: todo_write::description(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["todos".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for TodoWriteHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

/// Claude Code's todos from the arguments, checked as Claude Code checks them.
fn parse_todos(invocation: &ToolInvocation) -> Result<Vec<Todo>, FunctionCallError> {
    let args: TodoWriteArgs = parse_claude_arguments(TODO_WRITE, invocation)?;
    let invalid = |issue: String| {
        rejected(&format!(
            "InputValidationError: {TODO_WRITE} failed due to the following issue: {issue}"
        ))
    };
    let todos = args
        .todos
        .into_iter()
        .map(|todo| {
            let status = TodoStatus::parse(&todo.status).ok_or_else(|| {
                invalid(format!(
                    "Invalid status '{}', expected one of {}",
                    todo.status,
                    TodoStatus::ALL.join(", ")
                ))
            })?;
            Ok(Todo {
                content: todo.content,
                status,
                active_form: todo.active_form,
            })
        })
        .collect::<Result<Vec<_>, FunctionCallError>>()?;
    todo_write::validate(&todos).map_err(invalid)?;
    Ok(todos)
}

impl TodoWriteHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let todos = parse_todos(&invocation)?;
        let _permission = check_permission(&invocation, TODO_WRITE, PermissionInput::Tool).await?;
        let arguments = todo_write::update_plan_arguments(&todos).to_string();
        self.update_plan
            .handle(delegated(&invocation, ToolPayload::Function { arguments }))
            .await?;
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            todo_write::RESULT_MESSAGE.to_string(),
            json!({ "newTodos": todo_write::todos_json(&todos) }),
        )))
    }
}
