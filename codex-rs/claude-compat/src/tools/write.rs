//! Claude Code's `Write` tool: description and messages.

use super::BASH;
use super::EDIT;
use super::READ;

pub const FILE_PATH_PARAM_DESCRIPTION: &str =
    "The absolute path to the file to write (must be absolute, not relative)";
pub const CONTENT_PARAM_DESCRIPTION: &str = "The content to write to the file";

/// The tool description, after Claude Code's.
pub fn description() -> String {
    format!(
        "Writes a file to the local filesystem.

Usage:
- This tool will overwrite the existing file if there is one at the provided path.
- If this is an existing file, you MUST use the {READ} tool first to read the file's contents. This tool will fail if you did not read the file first.
- Prefer the {EDIT} tool for modifying existing files \u{2014} it only sends the diff. Only use this tool to create new files or for complete rewrites.
- NEVER create documentation files (*.md) or README files unless explicitly requested by the User.
- Only use emojis if the user explicitly requests it. Avoid writing emojis to files unless asked.
- The file_path must be a distinct file path, not a directory path. If the path resolves to an existing directory, the tool will reject it with a clear error message. Use a path that includes a filename with an appropriate extension (e.g., `my-docs/analysis/api/report.md`)."
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
