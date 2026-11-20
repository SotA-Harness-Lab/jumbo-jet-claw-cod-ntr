// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Grep`: Claude Code's content search tool.
//!
//! Delegated: the search, a ripgrep command run through Codex's `exec_command` runtime
//! (sandbox, approval, execution policy, command events), and reading the path through the
//! executor filesystem. Added: Claude Code's parameters, the ripgrep arguments it builds,
//! `head_limit` and `offset`, shortened paths and its result text.

use std::collections::BTreeMap;

use codex_claude_compat::tools::GREP;
use codex_claude_compat::tools::grep;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde_json::Value;

use super::OFFERED_TOOLS;
use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::files::FileTarget;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use super::ripgrep::Ripgrep;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `Grep` where commands can run, that is where `Bash` is offered. It replaces no
/// Codex tool.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if swap.offers(codex_claude_compat::tools::BASH) {
        swap.add(GrepHandler {
            ripgrep: Ripgrep::new(),
        });
    }
}

struct GrepHandler {
    ripgrep: Ripgrep,
}

impl ToolExecutor<ToolInvocation> for GrepHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(GREP)
    }

    fn spec(&self) -> ToolSpec {
        let string = |description: &str| JsonSchema::string(Some(description.to_string()));
        let number = |description: &str| JsonSchema::number(Some(description.to_string()));
        let boolean = |description: &str| JsonSchema::boolean(Some(description.to_string()));
        let properties = BTreeMap::from([
            (
                "pattern".to_string(),
                string(grep::PATTERN_PARAM_DESCRIPTION),
            ),
            ("path".to_string(), string(grep::PATH_PARAM_DESCRIPTION)),
            ("glob".to_string(), string(grep::GLOB_PARAM_DESCRIPTION)),
            (
                "output_mode".to_string(),
                JsonSchema::string_enum(
                    grep::OUTPUT_MODES
                        .iter()
                        .map(|mode| Value::String((*mode).to_string()))
                        .collect(),
                    Some(grep::OUTPUT_MODE_PARAM_DESCRIPTION.to_string()),
                ),
            ),
            ("-B".to_string(), number(grep::BEFORE_PARAM_DESCRIPTION)),
            ("-A".to_string(), number(grep::AFTER_PARAM_DESCRIPTION)),
            ("-C".to_string(), number(grep::C_PARAM_DESCRIPTION)),
            (
                "context".to_string(),
                number(grep::CONTEXT_PARAM_DESCRIPTION),
            ),
            (
                "-n".to_string(),
                boolean(grep::LINE_NUMBERS_PARAM_DESCRIPTION),
            ),
            (
                "-i".to_string(),
                boolean(grep::CASE_INSENSITIVE_PARAM_DESCRIPTION),
            ),
            ("type".to_string(), string(grep::TYPE_PARAM_DESCRIPTION)),
            (
                "head_limit".to_string(),
                number(grep::HEAD_LIMIT_PARAM_DESCRIPTION),
            ),
            ("offset".to_string(), number(grep::OFFSET_PARAM_DESCRIPTION)),
            (
                "multiline".to_string(),
                boolean(grep::MULTILINE_PARAM_DESCRIPTION),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: GREP.to_string(),
            description: grep::description(OFFERED_TOOLS),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["pattern".to_string()]),
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

impl CoreToolRuntime for GrepHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl GrepHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let arguments: Value = parse_claude_arguments(GREP, &invocation)?;
        let input = grep::GrepInput::parse(arguments).map_err(|message| rejected(&message))?;
        let target = FileTarget::resolve(&invocation, input.path.as_deref().unwrap_or_default())?;
        if let Some(path) = &input.path
            && target.metadata().await?.is_none()
        {
            return Err(rejected(&grep::path_not_found_message(
                path,
                &target.cwd_display(),
            )));
        }

        let _permission = check_permission(
            &invocation,
            GREP,
            PermissionInput::ReadPath(&target.native_path()),
        )
        .await?;
        let lines = self
            .ripgrep
            .run(
                &invocation,
                target.environment,
                &grep::rg_args(&input),
                &target.native_path(),
            )
            .await?;
        let result = grep::format_result(&input, lines, &target.cwd_display());
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            result.text,
            result.tool_response,
        )))
    }
}
