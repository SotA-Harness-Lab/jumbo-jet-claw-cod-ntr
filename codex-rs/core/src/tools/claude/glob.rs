//! `Glob`: Claude Code's file search tool.
//!
//! Delegated: listing the files, a ripgrep command (`rg --files`, which matches the glob
//! and sorts by modification time) run through Codex's `exec_command` runtime (sandbox,
//! approval, execution policy, command events), and checking the directory through the
//! executor filesystem. Added: Claude Code's parameters, the split of an absolute pattern,
//! the 100-file limit, shortened paths and its result text.

use std::collections::BTreeMap;
use std::time::Instant;

use codex_claude_compat::tools::GLOB;
use codex_claude_compat::tools::glob;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde_json::Value;

use super::PermissionInput;
use super::check_permission;
use super::OFFERED_TOOLS;
use super::ToolSwap;
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

/// Offers `Glob` where commands can run, that is where `Bash` is offered. It replaces no
/// Codex tool.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if swap.offers(codex_claude_compat::tools::BASH) {
        swap.add(GlobHandler {
            ripgrep: Ripgrep::new(),
        });
    }
}

struct GlobHandler {
    ripgrep: Ripgrep,
}

impl ToolExecutor<ToolInvocation> for GlobHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(GLOB)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "pattern".to_string(),
                JsonSchema::string(Some(glob::PATTERN_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "path".to_string(),
                JsonSchema::string(Some(glob::PATH_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: GLOB.to_string(),
            description: glob::description(OFFERED_TOOLS),
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

impl CoreToolRuntime for GlobHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl GlobHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let start = Instant::now();
        let arguments: Value = parse_claude_arguments(GLOB, &invocation)?;
        let input = glob::GlobInput::parse(arguments).map_err(|message| rejected(&message))?;
        if let Some(path) = &input.path {
            let directory = FileTarget::resolve(&invocation, path)?;
            match directory.metadata().await? {
                None => {
                    return Err(rejected(&glob::directory_not_found_message(
                        path,
                        &directory.cwd_display(),
                    )));
                }
                Some(metadata) if !metadata.is_directory => {
                    return Err(rejected(&glob::not_a_directory_message(path)));
                }
                Some(_) => {}
            }
        }

        // An absolute pattern names its own directory, as in Claude Code.
        let (directory, pattern) = match glob::split_absolute_pattern(&input.pattern) {
            Some((directory, pattern)) => (Some(directory), pattern),
            None => (input.path.clone(), input.pattern.clone()),
        };
        let target = FileTarget::resolve(&invocation, directory.as_deref().unwrap_or_default())?;
        let _permission = check_permission(
            &invocation,
            GLOB,
            PermissionInput::ReadPath(&target.native_path()),
        )
        .await?;
        let paths = self
            .ripgrep
            .run(
                &invocation,
                target.environment,
                &glob::rg_args(&pattern),
                &target.native_path(),
            )
            .await?;
        let duration_ms = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
        let result = glob::format_result(paths, &target.cwd_display(), duration_ms);
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            result.text,
            result.tool_response,
        )))
    }
}
