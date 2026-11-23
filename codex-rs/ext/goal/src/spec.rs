//! Responses API tool definitions for persisted thread goals.

use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use serde_json::json;
use std::collections::BTreeMap;

pub const GET_GOAL_TOOL_NAME: &str = "get_goal";
pub const CREATE_GOAL_TOOL_NAME: &str = "create_goal";
pub const UPDATE_GOAL_TOOL_NAME: &str = "update_goal";

pub fn create_get_goal_tool() -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: GET_GOAL_TOOL_NAME.to_string(),
        description: codex_prompt_texts::text!("ext/goal/spec/get_goal_description.md").to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(BTreeMap::new(), Some(Vec::new()), Some(false.into())),
        output_schema: None,
    })
}

pub fn create_create_goal_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "objective".to_string(),
            JsonSchema::string(Some(
                codex_prompt_texts::text!(
                    "ext/goal/spec/create_goal_objective_param_description.md"
                )
                .to_string(),
            )),
        ),
        (
            "token_budget".to_string(),
            JsonSchema::integer(Some(
                "Positive token budget for the new goal. Omit unless explicitly requested."
                    .to_string(),
            )),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: CREATE_GOAL_TOOL_NAME.to_string(),
        description: format!(
            codex_prompt_texts::text!("ext/goal/spec/create_goal_description.md"),
            UPDATE_GOAL_TOOL_NAME = UPDATE_GOAL_TOOL_NAME
        ),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            /*required*/ Some(vec!["objective".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

pub fn create_update_goal_tool() -> ToolSpec {
    let properties = BTreeMap::from([(
        "status".to_string(),
        JsonSchema::string_enum(
            vec![json!("complete"), json!("blocked"), json!("paused")],
            Some(
                codex_prompt_texts::text!("ext/goal/spec/update_goal_status_param_description.md")
                    .to_string(),
            ),
        ),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: UPDATE_GOAL_TOOL_NAME.to_string(),
        description: codex_prompt_texts::text!("ext/goal/spec/update_goal_description.md")
            .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            /*required*/ Some(vec!["status".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}
