// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `TodoWrite` tool: description, the checks on its list, its translation to
//! Codex's plan tool (`update_plan`) and its result text.

use serde_json::Value;
use serde_json::json;

use super::EDIT;

pub const TODOS_PARAM_DESCRIPTION: &str = "The updated todo list";

/// The text the model receives after the list was updated.
pub const RESULT_MESSAGE: &str =
    codex_prompt_texts::text!("claude-compat/tools/todo_write/result_message.md");

/// The status of a todo; Codex's plan steps have the same three.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl TodoStatus {
    pub const ALL: [&'static str; 3] = ["pending", "in_progress", "completed"];

    pub fn parse(status: &str) -> Option<Self> {
        match status {
            "pending" => Some(Self::Pending),
            "in_progress" => Some(Self::InProgress),
            "completed" => Some(Self::Completed),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
        }
    }
}

/// One item of the list, as the model writes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    pub content: String,
    pub status: TodoStatus,
    pub active_form: String,
}

/// Claude Code's checks on the list beyond its schema: neither form may be empty.
pub fn validate(todos: &[Todo]) -> Result<(), String> {
    for todo in todos {
        if todo.content.is_empty() {
            return Err("Content cannot be empty".to_string());
        }
        if todo.active_form.is_empty() {
            return Err("Active form cannot be empty".to_string());
        }
    }
    Ok(())
}

/// Claude Code clears the list once no incomplete item remains.
pub fn retained_todos(todos: &[Todo]) -> &[Todo] {
    if todos
        .iter()
        .all(|todo| todo.status == TodoStatus::Completed)
    {
        &[]
    } else {
        todos
    }
}

/// The `update_plan` arguments for a list: one step a todo, with its status. The step in
/// progress shows its present continuous form, as Claude Code's own list does.
pub fn update_plan_arguments(todos: &[Todo]) -> Value {
    let plan = todos
        .iter()
        .map(|todo| {
            let step = match todo.status {
                TodoStatus::InProgress => &todo.active_form,
                TodoStatus::Pending | TodoStatus::Completed => &todo.content,
            };
            json!({ "step": step, "status": todo.status.as_str() })
        })
        .collect::<Vec<_>>();
    json!({ "plan": plan })
}

/// The list as Claude Code reports it to hooks (`newTodos`).
pub fn todos_json(todos: &[Todo]) -> Value {
    Value::Array(
        todos
            .iter()
            .map(|todo| {
                json!({
                    "content": todo.content,
                    "status": todo.status.as_str(),
                    "activeForm": todo.active_form,
                })
            })
            .collect(),
    )
}

/// The tool description, after Claude Code's.
pub fn description() -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/todo_write/description.md"),
        EDIT = EDIT
    )
}

#[cfg(test)]
#[path = "todo_write_tests.rs"]
mod tests;
