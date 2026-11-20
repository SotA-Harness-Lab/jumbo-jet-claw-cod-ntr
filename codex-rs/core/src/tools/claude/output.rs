// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The result of a Claude tool: Claude Code's text for the model and Claude Code's
//! `tool_response` for hooks.

use codex_protocol::models::FunctionCallOutputBody;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::FunctionCallOutputPayload;
use codex_protocol::models::ResponseInputItem;
use serde_json::Value;

use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;

pub(crate) struct ClaudeToolOutput {
    text: String,
    is_error: bool,
    tool_response: Value,
    external_context: bool,
    /// Text and images in place of `text`, for a result with images (a PDF's pages, a
    /// notebook's plots); `text` is then what logs and code mode see.
    content: Option<Vec<FunctionCallOutputContentItem>>,
}

impl ClaudeToolOutput {
    pub(crate) fn new(text: String, tool_response: Value) -> Self {
        Self {
            text,
            is_error: false,
            tool_response,
            external_context: false,
            content: None,
        }
    }

    /// A result whose content items (text and images) are what the model receives.
    pub(crate) fn with_content(
        text: String,
        content: Vec<FunctionCallOutputContentItem>,
        tool_response: Value,
    ) -> Self {
        Self {
            content: Some(content),
            ..Self::new(text, tool_response)
        }
    }

    /// A result Claude Code marks as an error, such as a command that exited with a
    /// non-zero code: the model sees `text`, `PostToolUseFailure` hooks run with it.
    pub(crate) fn error(text: String, tool_response: Value) -> Self {
        Self {
            text,
            is_error: true,
            tool_response,
            external_context: false,
            content: None,
        }
    }

    /// Marks the result as content from outside the session (a web page, search results),
    /// as Codex's own web tools do.
    pub(crate) fn with_external_context(mut self) -> Self {
        self.external_context = true;
        self
    }
}

impl ToolOutput for ClaudeToolOutput {
    fn log_output(&self) -> String {
        self.text.clone()
    }

    fn success_for_logging(&self) -> bool {
        true
    }

    fn to_response_item(&self, call_id: &str, _payload: &ToolPayload) -> ResponseInputItem {
        ResponseInputItem::FunctionCallOutput {
            call_id: call_id.to_string(),
            output: FunctionCallOutputPayload {
                body: match &self.content {
                    Some(content) => FunctionCallOutputBody::ContentItems(content.clone()),
                    None => FunctionCallOutputBody::Text(self.text.clone()),
                },
                success: Some(!self.is_error),
            },
        }
    }

    fn post_tool_use_response(&self, _call_id: &str, _payload: &ToolPayload) -> Option<Value> {
        Some(self.tool_response.clone())
    }

    fn contains_external_context(&self) -> bool {
        self.external_context
    }

    fn post_tool_use_failure(&self) -> Option<String> {
        self.is_error.then(|| self.text.clone())
    }

    fn code_mode_result(&self, _payload: &ToolPayload) -> Value {
        Value::String(self.text.clone())
    }
}
