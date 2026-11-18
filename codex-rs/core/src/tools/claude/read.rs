//! `Read`: Claude Code's file reading tool.
//!
//! Delegated: reading through the executor filesystem under the turn's sandbox, and images
//! to Codex's `view_image` runtime (which checks the model takes images, sanitizes and
//! resizes them, and emits the image event). Added: Claude Code's parameters, its line
//! selection and `N<TAB>line` numbering, its size limits and messages, and remembering the
//! read for `Edit` and `Write`.

use std::sync::Arc;

use codex_claude_compat::tools::READ;
use codex_claude_compat::tools::read;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;

use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::delegated;
use super::failed;
use super::file_state::FileRecord;
use super::file_state::ReadFiles;
use super::files::FileTarget;
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

/// Offers `Read` in place of `view_image`, whose runtime shows image files.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    let view_image = swap.take("view_image");
    swap.add(ReadHandler { view_image });
}

struct ReadHandler {
    view_image: Option<Arc<dyn CoreToolRuntime>>,
}

#[derive(Deserialize)]
struct ReadArgs {
    file_path: String,
    #[serde(default)]
    offset: Option<usize>,
    #[serde(default)]
    limit: Option<usize>,
}

impl ToolExecutor<ToolInvocation> for ReadHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(READ)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "file_path".to_string(),
                JsonSchema::string(Some(read::FILE_PATH_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "offset".to_string(),
                JsonSchema::integer(Some(read::OFFSET_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "limit".to_string(),
                JsonSchema::integer(Some(read::LIMIT_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: READ.to_string(),
            description: read::description(self.view_image.is_some()),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["file_path".to_string()]),
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

impl CoreToolRuntime for ReadHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl ReadHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: ReadArgs = parse_claude_arguments(READ, &invocation)?;
        let target = FileTarget::resolve(&invocation, &args.file_path)?;
        let _permission = check_permission(
            &invocation,
            READ,
            PermissionInput::ReadPath(&target.native_path()),
        )
        .await?;

        if read::is_image(&args.file_path) {
            let Some(view_image) = &self.view_image else {
                return Err(rejected(
                    "This session cannot show images: Codex's view_image tool is off.",
                ));
            };
            let arguments = json!({ "path": target.native_path() }).to_string();
            return view_image
                .handle(delegated(&invocation, ToolPayload::Function { arguments }))
                .await;
        }
        if let Some(extension) = read::binary_extension(&args.file_path) {
            return Err(rejected(&read::binary_message(&extension)));
        }

        let Some(metadata) = target.metadata().await? else {
            return Err(failed(codex_claude_compat::tools::file_not_found_message(
                &target.cwd_display(),
            )));
        };
        if metadata.is_directory {
            return Err(rejected(&read::directory_message(&args.file_path)));
        }
        if args.limit.is_none() && metadata.size > read::MAX_SIZE_BYTES {
            return Err(failed(read::too_large_message(metadata.size)));
        }
        let bytes = target.read_bytes().await?;
        let text = String::from_utf8_lossy(&bytes);
        let slice = read::slice_text(&text, args.offset, args.limit);
        if read::exceeds_token_limit(&slice.content) {
            return Err(failed(read::token_limit_message(read::estimated_tokens(
                &slice.content,
            ))));
        }

        ReadFiles::of(&invocation.session).insert(
            target.key(),
            FileRecord {
                modified_at_ms: metadata.modified_at_ms,
                content: None,
            },
        );

        let tool_response = json!({
            "type": "text",
            "file": {
                "filePath": args.file_path,
                "content": slice.content,
                "numLines": slice.num_lines,
                "startLine": slice.start_line,
                "totalLines": slice.total_lines,
            }
        });
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            read::render(&slice),
            tool_response,
        )))
    }
}
