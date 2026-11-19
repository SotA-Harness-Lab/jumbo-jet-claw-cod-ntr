// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Edit`: Claude Code's exact string replacement tool.
//!
//! Delegated: the write, as a patch applied by Codex's `apply_patch` runtime (approval,
//! sandbox, patch events and the turn's diff). Added: Claude Code's parameters, its exact
//! match and uniqueness rules (`codex_claude_compat::tools::edit`), the rule that the file
//! is read first and not changed since, and its messages.

use std::collections::BTreeMap;
use std::sync::Arc;

use codex_claude_compat::tools::EDIT;
use codex_claude_compat::tools::edit;
use codex_claude_compat::tools::edit::EditError;
use codex_claude_compat::tools::patch;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;

use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::file_state::ReadFiles;
use super::file_state::write_refusal;
use super::files::FileTarget;
use super::files::apply_patch;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use super::write::patch_runtime;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `Edit` in place of `apply_patch`.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if let Some(apply_patch) = patch_runtime(swap) {
        swap.add(EditHandler { apply_patch });
    }
}

struct EditHandler {
    apply_patch: Arc<dyn CoreToolRuntime>,
}

#[derive(Deserialize)]
struct EditArgs {
    file_path: String,
    old_string: String,
    new_string: String,
    #[serde(default)]
    replace_all: Option<bool>,
}

impl ToolExecutor<ToolInvocation> for EditHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(EDIT)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "file_path".to_string(),
                JsonSchema::string(Some(edit::FILE_PATH_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "old_string".to_string(),
                JsonSchema::string(Some(edit::OLD_STRING_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "new_string".to_string(),
                JsonSchema::string(Some(edit::NEW_STRING_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "replace_all".to_string(),
                JsonSchema::boolean(Some(edit::REPLACE_ALL_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: EDIT.to_string(),
            description: edit::description(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec![
                    "file_path".to_string(),
                    "old_string".to_string(),
                    "new_string".to_string(),
                ]),
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

impl CoreToolRuntime for EditHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl EditHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: EditArgs = parse_claude_arguments(EDIT, &invocation)?;
        let replace_all = args.replace_all.unwrap_or(false);
        if args.old_string == args.new_string {
            return Err(rejected(&EditError::NoChange.message()));
        }
        let target = FileTarget::resolve(&invocation, &args.file_path)?;
        let path = target.native_path();
        if !patch::is_patchable_path(&path) {
            return Err(rejected(&format!(
                "Cannot edit '{}': the path has a line break or leading or trailing spaces.",
                args.file_path
            )));
        }
        let _permission =
            check_permission(&invocation, EDIT, PermissionInput::EditPath(&path)).await?;

        let current = match target.metadata().await? {
            None => None,
            Some(metadata) if metadata.is_directory => {
                return Err(rejected(&edit::directory_message(&args.file_path)));
            }
            Some(metadata) => {
                let raw = target.read_text().await?;
                let normalized = raw.replace("\r\n", "\n");
                let record = ReadFiles::of(&invocation.session).get(&target.key());
                if let Some(message) =
                    write_refusal(record.as_ref(), metadata.modified_at_ms, &normalized)
                {
                    return Err(rejected(message));
                }
                Some((raw, normalized))
            }
        };

        let updated = edit::apply_edit(
            current.as_ref().map(|(_, normalized)| normalized.as_str()),
            &args.old_string,
            &args.new_string,
            replace_all,
        )
        .map_err(|error| match error {
            EditError::FileMissing => rejected(
                &codex_claude_compat::tools::file_not_found_message(&target.cwd_display()),
            ),
            error => rejected(&error.message()),
        })?;
        let patch_text = match &current {
            Some((raw, _)) => patch::update_file_patch(&path, raw, &updated),
            None => Some(patch::add_file_patch(&path, &updated)),
        };
        if let Some(patch_text) = patch_text {
            apply_patch(&self.apply_patch, &invocation, patch_text).await?;
        }
        target.record_written(&invocation).await;

        let tool_response = json!({
            "filePath": args.file_path,
            "oldString": args.old_string,
            "newString": args.new_string,
            "originalFile": current.map(|(_, normalized)| normalized).unwrap_or_default(),
            "structuredPatch": [],
            "userModified": false,
            "replaceAll": replace_all,
        });
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            edit::success_message(&args.file_path, replace_all),
            tool_response,
        )))
    }
}
