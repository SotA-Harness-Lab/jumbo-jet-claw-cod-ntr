// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Write` tool: description and messages.

use serde_json::Value;

use super::BASH;
use super::EDIT;
use super::READ;
use super::WRITE;
use super::input::Input;

pub const FILE_PATH_PARAM_DESCRIPTION: &str =
    "The absolute path to the file to write (must be absolute, not relative)";
pub const CONTENT_PARAM_DESCRIPTION: &str = "The content to write to the file";

/// A `Write` call's input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteInput {
    pub file_path: String,
    pub content: String,
}

impl WriteInput {
    /// Reads the model's arguments, taking `path` for `file_path` and `file_text` or
    /// `file_content` for `content`, as Claude Code does; `Err` is Claude Code's input
    /// validation message.
    pub fn parse(arguments: Value) -> Result<Self, String> {
        let mut input = Input::ignoring_unknown(
            WRITE,
            arguments,
            &[
                ("path", "file_path"),
                ("file_text", "content"),
                ("file_content", "content"),
            ],
        );
        let file_path = input.required_string("file_path");
        let content = input.required_string("content");
        input.finish()?;
        Ok(Self { file_path, content })
    }
}

/// The tool description, after Claude Code's.
pub fn description() -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/write/description.md"),
        READ = READ,
        EDIT = EDIT
    )
}

/// The text the model receives after a file was created.
pub fn created_message(file_path: &str) -> String {
    format!("File created successfully at: {file_path}")
}

/// The text the model receives after an existing file was overwritten.
pub fn updated_message(file_path: &str) -> String {
    format!("The file {file_path} has been updated successfully.")
}

pub fn directory_message(file_path: &str) -> String {
    format!(
        "Cannot write to '{file_path}': the specified path is an existing directory. To write a file inside this directory, use a path that includes a filename with extension, e.g. '{file_path}/<filename>.md'. Use {BASH} ls to list existing files in this directory."
    )
}
