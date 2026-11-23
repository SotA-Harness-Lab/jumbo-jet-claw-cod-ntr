//! Bounded model-facing contracts for the local managed-worktree service.

use codex_app_server_protocol::DynamicToolFunctionSpec;
use codex_app_server_protocol::DynamicToolSpec;
use serde_json::json;

pub(crate) fn specs() -> Vec<DynamicToolSpec> {
    [
        ("create_worktree", codex_prompt_texts::text!("tui/managed_worktree_tool_specs/create_worktree_description.md"),
            json!({"ref":{"type":"string","maxLength":128},"allowAsync":{"type":"boolean","const":true}}), vec!["allowAsync"]),
        ("get_worktree_creation_status", codex_prompt_texts::text!("tui/managed_worktree_tool_specs/get_worktree_creation_status_description.md"),
            json!({"operationId":{"type":"string","maxLength":128}}), vec!["operationId"]),
        ("list_worktrees", codex_prompt_texts::text!("tui/managed_worktree_tool_specs/list_worktrees_description.md"),
            json!({"cursor":{"type":"string","maxLength":512}}), vec![]),
    ].into_iter().map(|(name, description, properties, required)| DynamicToolSpec::Function(DynamicToolFunctionSpec {
        name: name.to_owned(), description: description.to_owned(),
        input_schema: json!({"type":"object", "additionalProperties":false, "properties":properties,"required":required}), defer_loading: true,
    })).collect()
}
