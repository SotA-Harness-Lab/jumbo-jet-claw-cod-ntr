// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Write`: Claude Code's file writing tool.
//!
//! Delegated: the write itself, as a patch applied by Codex's `apply_patch` runtime
//! (approval, sandbox, patch events and the turn's diff). Added: Claude Code's parameters,
//! the rule that an existing file is read first and not changed since, and its messages.

use std::collections::BTreeMap;
use std::sync::Arc;

use codex_claude_compat::tools::WRITE;
use codex_claude_compat::tools::patch;
use codex_claude_compat::tools::write;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde_json::Value;
use serde_json::json;

use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::file_state::FileStamp;
use super::file_state::ReadFiles;
use super::file_state::write_refusal;
use super::files::FileTarget;
use super::files::apply_patch;
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

/// Offers `Write` in place of `apply_patch`.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if let Some(apply_patch) = patch_runtime(swap) {
        swap.add(WriteHandler { apply_patch });
    }
}

/// Codex's `apply_patch` runtime, taken from the registry; when the model's catalog does not
/// offer `apply_patch`, a runtime of its own, unless the tool policy forbids patches.
pub(super) fn patch_runtime(swap: &mut ToolSwap<'_>) -> Option<Arc<dyn CoreToolRuntime>> {
    swap.take("apply_patch").or_else(|| {
        swap.allows("apply_patch").then(|| {
            Arc::new(crate::tools::handlers::ApplyPatchHandler::default())
                as Arc<dyn CoreToolRuntime>
        })
    })
}

struct WriteHandler {
    apply_patch: Arc<dyn CoreToolRuntime>,
}

impl ToolExecutor<ToolInvocation> for WriteHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(WRITE)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "file_path".to_string(),
                JsonSchema::string(Some(write::FILE_PATH_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "content".to_string(),
                JsonSchema::string(Some(write::CONTENT_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: WRITE.to_string(),
            description: write::description(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["file_path".to_string(), "content".to_string()]),
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

impl CoreToolRuntime for WriteHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl WriteHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let arguments: Value = parse_claude_arguments(WRITE, &invocation)?;
        let args = write::WriteInput::parse(arguments).map_err(|message| rejected(&message))?;
        let target = FileTarget::resolve(&invocation, &args.file_path)?;
        let path = target.native_path();
        if !patch::is_patchable_path(&path) {
            return Err(rejected(&format!(
                "Cannot write to '{}': the path has a line break or leading or trailing spaces.",
                args.file_path
            )));
        }
        let _permission =
            check_permission(&invocation, WRITE, PermissionInput::EditPath(&path)).await?;

        let (patch_text, original) = match target.metadata().await? {
            None => (Some(patch::add_file_patch(&path, &args.content)), None),
            Some(metadata) if metadata.is_directory => {
                return Err(rejected(&write::directory_message(&args.file_path)));
            }
            Some(metadata) => {
                let current = target.read_text().await?;
                let record = ReadFiles::of(&invocation.session).get(&target.key());
                if let Some(message) = write_refusal(
                    record.as_ref(),
                    FileStamp::of(&metadata),
                    &current.replace("\r\n", "\n"),
                ) {
                    return Err(rejected(message));
                }
                (
                    patch::update_file_patch(&path, &current, &args.content),
                    Some(current),
                )
            }
        };
        if let Some(patch_text) = patch_text {
            apply_patch(&self.apply_patch, &invocation, patch_text).await?;
        }
        target.record_written(&invocation).await;

        let (text, kind) = match &original {
            None => (write::created_message(&args.file_path), "create"),
            Some(_) => (write::updated_message(&args.file_path), "update"),
        };
        let tool_response = json!({
            "type": kind,
            "filePath": args.file_path,
            "content": args.content,
            "structuredPatch": [],
            "originalFile": original,
        });
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            text,
            tool_response,
        )))
    }
}
