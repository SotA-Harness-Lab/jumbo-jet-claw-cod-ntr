// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::Todo;
use super::TodoStatus;
use super::todos_json;
use super::update_plan_arguments;
use super::validate;

fn todo(content: &str, status: TodoStatus, active_form: &str) -> Todo {
    Todo {
        content: content.to_string(),
        status,
        active_form: active_form.to_string(),
    }
}

#[test]
fn todos_become_plan_steps_with_the_active_form_in_progress() {
    let todos = vec![
        todo("Run tests", TodoStatus::Completed, "Running tests"),
        todo("Build", TodoStatus::InProgress, "Building"),
        todo("Ship", TodoStatus::Pending, "Shipping"),
    ];
    assert_eq!(
        update_plan_arguments(&todos),
        json!({
            "plan": [
                { "step": "Run tests", "status": "completed" },
                { "step": "Building", "status": "in_progress" },
                { "step": "Ship", "status": "pending" },
            ]
        })
    );
    assert_eq!(
        todos_json(&todos[1..2]),
        json!([{ "content": "Build", "status": "in_progress", "activeForm": "Building" }])
    );
    assert_eq!(update_plan_arguments(&[]), json!({ "plan": [] }));
}

#[test]
fn both_forms_must_be_given() {
    assert_eq!(validate(&[todo("a", TodoStatus::Pending, "b")]), Ok(()));
    assert_eq!(
        validate(&[todo("", TodoStatus::Pending, "b")]),
        Err("Content cannot be empty".to_string())
    );
    assert_eq!(
        validate(&[todo("a", TodoStatus::Pending, "")]),
        Err("Active form cannot be empty".to_string())
    );
    assert_eq!(
        TodoStatus::parse("in_progress"),
        Some(TodoStatus::InProgress)
    );
    assert_eq!(TodoStatus::parse("done"), None);
}
