// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Read`: Claude Code's file reading tool.
//!
//! Delegated: reading through the executor filesystem under the turn's sandbox, and images
//! to Codex's `view_image` runtime (which checks the model takes images, sanitizes and
//! resizes them, and emits the image event). Added: Claude Code's parameters, its line
//! selection and `N<TAB>line` numbering, its size limits and messages, remembering the read
//! for `Edit` and `Write`, notebooks shown cell by cell, and PDFs shown page by page.
//!
//! A PDF is rendered as Claude Code renders one for a model that cannot take PDF documents:
//! each page an image made with poppler's `pdftoppm`, here with the page's text from
//! `pdftotext` beside it, since Codex's models take no PDF document. Without images (a
//! text-only model) the text alone is returned.

use std::sync::Arc;

use codex_claude_compat::tools::READ;
use codex_claude_compat::tools::format_file_size;
use codex_claude_compat::tools::notebook;
use codex_claude_compat::tools::notebook::NotebookPart;
use codex_claude_compat::tools::read;
use codex_exec_server::FileMetadata;
use codex_protocol::models::DEFAULT_IMAGE_DETAIL;
use codex_protocol::models::FunctionCallOutputContentItem;
use codex_protocol::models::ImageReference;
use codex_protocol::openai_models::InputModality;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde_json::Value;
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

mod pdf;

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
            (
                "pages".to_string(),
                JsonSchema::string(Some(read::pages_param_description())),
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
        let arguments: Value = parse_claude_arguments(READ, &invocation)?;
        let args = read::ReadInput::parse(arguments).map_err(|message| rejected(&message))?;
        // An empty `pages` is taken as left out, as Claude Code takes it.
        let pages = args
            .pages
            .as_deref()
            .filter(|pages| !pages.trim().is_empty())
            .map(read::validate_pages)
            .transpose()
            .map_err(|message| rejected(&message))?;
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
        if read::is_pdf(&args.file_path) {
            return pdf::read(&invocation, &target, &args.file_path, pages).await;
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
        if !metadata.is_file {
            return Err(rejected(&read::not_a_regular_file_message(&args.file_path)));
        }
        // A notebook's limit applies to its cells as shown, not to the file.
        if read::is_notebook(&args.file_path) {
            let bytes = target.read_bytes().await?;
            let text = String::from_utf8_lossy(&bytes);
            return read_notebook(&invocation, &target, &args.file_path, &text, &metadata);
        }
        if args.limit.is_none() && metadata.size > read::MAX_SIZE_BYTES {
            return Err(failed(read::too_large_message(metadata.size)));
        }
        // A file too large to read whole is read in chunks, keeping only the lines asked for.
        let slice = if metadata.size > read::MAX_SIZE_BYTES {
            target.read_lines(args.offset, args.limit).await?
        } else {
            let bytes = target.read_bytes().await?;
            read::slice_text(&String::from_utf8_lossy(&bytes), args.offset, args.limit)
        };
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

/// Whether the session's model takes images.
fn model_takes_images(invocation: &ToolInvocation) -> bool {
    invocation
        .step_context
        .settings
        .model_info
        .input_modalities
        .contains(&InputModality::Image)
}

fn image_item(media_type: &str, base64: &str) -> FunctionCallOutputContentItem {
    FunctionCallOutputContentItem::InputImage {
        image: ImageReference::Inline {
            image_url: format!("data:{media_type};base64,{base64}"),
        },
        detail: Some(DEFAULT_IMAGE_DETAIL),
    }
}

/// A notebook, cell by cell, as Claude Code reads one.
fn read_notebook(
    invocation: &ToolInvocation,
    target: &FileTarget<'_>,
    file_path: &str,
    text: &str,
    metadata: &FileMetadata,
) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let cells = notebook::read_cells(text).map_err(failed)?;
    let cells_json = serde_json::to_string(&cells).unwrap_or_default();
    if cells_json.len() as u64 > read::MAX_SIZE_BYTES {
        return Err(failed(notebook::too_large_message(
            &format_file_size(cells_json.len() as u64),
            &format_file_size(read::MAX_SIZE_BYTES),
            file_path,
        )));
    }
    if read::exceeds_token_limit(&cells_json) {
        return Err(failed(read::token_limit_message(read::estimated_tokens(
            &cells_json,
        ))));
    }
    ReadFiles::of(&invocation.session).insert(
        target.key(),
        FileRecord {
            modified_at_ms: metadata.modified_at_ms,
            content: None,
        },
    );

    let images = model_takes_images(invocation);
    let parts = notebook::render(&cells);
    let mut log_text = String::new();
    let mut content = Vec::new();
    for part in parts {
        match part {
            NotebookPart::Text(text) => {
                log_text.push_str(&text);
                content.push(FunctionCallOutputContentItem::InputText { text });
            }
            NotebookPart::Image(image) if images => {
                log_text.push_str("\n[image]");
                content.push(image_item(&image.media_type, &image.image_data));
            }
            NotebookPart::Image(_) => {}
        }
    }
    let tool_response = json!({
        "type": "notebook",
        "file": { "filePath": file_path, "cells": cells },
    });
    Ok(boxed_tool_output(ClaudeToolOutput::with_content(
        log_text,
        content,
        tool_response,
    )))
}
