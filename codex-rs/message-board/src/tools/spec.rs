//! Schemas for shared board tools. The host supplies their selection and namespace.

use super::selection::MessageBoardTool;
use codex_tools::ResponsesApiNamespace;
use codex_tools::ResponsesApiNamespaceTool;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use codex_tools::parse_tool_input_schema;
use serde_json::json;

pub(super) fn tool(
    tool: MessageBoardTool,
    namespace: Option<&str>,
    namespace_description: &str,
    description_override: Option<&str>,
) -> ToolSpec {
    let (description, fields, required): (&str, &[&str], &[&str]) = match tool {
        MessageBoardTool::CreateChannel => (
            codex_prompt_texts::text!("message-board/tools/spec/create_channel_description.md"),
            &["channel_name", "description", "subscribe"],
            &["channel_name"],
        ),
        MessageBoardTool::UpsertChannel => (
            codex_prompt_texts::text!("message-board/tools/spec/upsert_channel_description.md"),
            &["channel_name", "description"],
            &["channel_name", "description"],
        ),
        MessageBoardTool::GetChannels => (
            codex_prompt_texts::text!("message-board/tools/spec/get_channels_description.md"),
            &["query", "recent_first", "limit", "cursor"],
            &[],
        ),
        MessageBoardTool::ListThreads => (
            codex_prompt_texts::text!("message-board/tools/spec/list_threads_description.md"),
            &[
                "channel_name",
                "sort",
                "recent_first",
                "limit",
                "cursor",
                "max_chars_per_post",
            ],
            &["channel_name"],
        ),
        MessageBoardTool::SearchPosts => (
            codex_prompt_texts::text!("message-board/tools/spec/search_posts_description.md"),
            &[
                "channel_name",
                "query",
                "after_message_id",
                "author",
                "limit",
                "cursor",
                "max_chars_per_post",
            ],
            &[],
        ),
        MessageBoardTool::ReadThread => (
            codex_prompt_texts::text!("message-board/tools/spec/read_thread_description.md"),
            &["thread_id", "limit", "cursor", "max_chars_per_post"],
            &["thread_id"],
        ),
        MessageBoardTool::ReadPost => (
            codex_prompt_texts::text!("message-board/tools/spec/read_post_description.md"),
            &["message_id", "offset_chars", "limit_chars"],
            &["message_id"],
        ),
        MessageBoardTool::Subscribe => (
            codex_prompt_texts::text!("message-board/tools/spec/subscribe_description.md"),
            &["channel_name", "thread_id", "target_agent"],
            &[],
        ),
        MessageBoardTool::Unsubscribe => (
            codex_prompt_texts::text!("message-board/tools/spec/unsubscribe_description.md"),
            &["channel_name", "thread_id", "target_agent"],
            &[],
        ),
        MessageBoardTool::Post => (
            codex_prompt_texts::text!("message-board/tools/spec/post_description.md"),
            &[
                "text",
                "channel_name",
                "new_channel_name",
                "thread_id",
                "agents_to_notify",
            ],
            &["text"],
        ),
        MessageBoardTool::Append => (
            codex_prompt_texts::text!("message-board/tools/spec/append_description.md"),
            &["text", "channel_name", "new_channel_name", "thread_id"],
            &["text"],
        ),
    };
    let mut properties = serde_json::Map::new();
    for field in fields {
        let schema = match *field {
            "description" if tool == MessageBoardTool::UpsertChannel => {
                json!({"type":"string","description":"Channel purpose, at most 512 UTF-8 bytes."})
            }
            "description" => {
                json!({"type":"string","description":"Optional channel purpose, at most 512 UTF-8 bytes."})
            }
            "new_channel_name" if tool == MessageBoardTool::Append => {
                json!({"type":"string","description":"Create this channel without subscribing anyone."})
            }
            "new_channel_name" => {
                json!({"type":"string","description":"Create and subscribe to this channel."})
            }
            "subscribe" => {
                json!({"type":"boolean","description":"Subscribe to new top-level posts. Default true."})
            }
            "author" | "target_agent" => {
                json!({"type":"string","description":"An absolute agent path or a reference relative to you."})
            }
            "recent_first" => json!({"type":"boolean","description":"Newest first by default."}),
            "limit" => {
                json!({"type":"integer","minimum":1,"description":"Maximum results, default 20; output budgets may return fewer. Continue with next_cursor."})
            }
            "offset_chars" => json!({"type":"integer","minimum":0,"description":"Default 0."}),
            "limit_chars" | "max_chars_per_post" => {
                json!({"type":"integer","minimum":1,"description":codex_prompt_texts::text!("message-board/tools/spec/limit_chars_description.md")})
            }
            "sort" => json!({"type":"string","enum":["created","activity"]}),
            "agents_to_notify" => {
                json!({"type":"array","items":{"type":"string"},"description":"Absolute agent paths or references relative to you; maximum 256."})
            }
            "cursor" => {
                json!({"type":"string","description":codex_prompt_texts::text!("message-board/tools/spec/cursor_description.md")})
            }
            _ => json!({"type":"string"}),
        };
        properties.insert((*field).into(), schema);
    }
    let parameters = json!({"type":"object","properties":properties,"required":required,"additionalProperties":false});
    let tool = ResponsesApiTool {
        name: tool.name().into(),
        description: description_override.unwrap_or(description).into(),
        strict: false,
        defer_loading: None,
        parameters: parse_tool_input_schema(&parameters)
            .unwrap_or_else(|error| panic!("message-board schema must parse: {error}")),
        output_schema: None,
    };
    match namespace {
        Some(namespace) => ToolSpec::Namespace(ResponsesApiNamespace {
            name: namespace.into(),
            description: namespace_description.into(),
            tools: vec![ResponsesApiNamespaceTool::Function(tool)],
        }),
        None => ToolSpec::Function(tool),
    }
}
