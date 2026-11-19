// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Edit` tool: description, the exact string replacement and its messages.

use super::BASH;
use super::READ;

pub const FILE_PATH_PARAM_DESCRIPTION: &str = "The absolute path to the file to modify";
pub const OLD_STRING_PARAM_DESCRIPTION: &str = "The text to replace";
pub const NEW_STRING_PARAM_DESCRIPTION: &str =
    "The text to replace it with (must be different from old_string)";
pub const REPLACE_ALL_PARAM_DESCRIPTION: &str =
    "Replace all occurrences of old_string (default false)";

/// Refusal of an edit or write of an existing file the session has not read.
pub const NOT_READ_MESSAGE: &str =
    "File has not been read yet. Read it first before writing to it.";
/// Refusal of an edit or write of a file changed since the session read it.
pub const MODIFIED_SINCE_READ_MESSAGE: &str = "File has been modified since read, either by the user or by a linter. Read it again before attempting to write it.";

/// The tool description, after Claude Code's.
pub fn description() -> String {
    format!(
        "Performs exact string replacements in files.

Usage:
- You must use your `{READ}` tool at least once in the conversation before editing. This tool will error if you attempt an edit without reading the file.
- When editing text from Read tool output, ensure you preserve the exact indentation (tabs/spaces) as it appears AFTER the line number prefix. The line number prefix format is: line number + tab. Everything after that is the actual file content to match. Never include any part of the line number prefix in the old_string or new_string.
- ALWAYS prefer editing existing files in the codebase. NEVER write new files unless explicitly required.
- Only use emojis if the user explicitly requests it. Avoid adding emojis to files unless asked.
- The edit will FAIL if `old_string` is not unique in the file. Either provide a larger string with more surrounding context to make it unique or use `replace_all` to change every instance of `old_string`.
- Use `replace_all` for replacing and renaming strings across the file. This parameter is useful if you want to rename a variable for instance.
- The file_path must be a file path, not a directory path. If the path resolves to an existing directory, the tool will reject it. Use a path that points to an existing file."
    )
}

/// Why an edit is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditError {
    /// `old_string` and `new_string` are the same.
    NoChange,
    /// The file does not exist and `old_string` is not empty.
    FileMissing,
    /// `old_string` is empty but the file has content.
    FileExists,
    /// `old_string` is not in the file.
    NotFound { old_string: String },
    /// `old_string` is in the file more than once and `replace_all` is false.
    NotUnique { matches: usize, old_string: String },
    /// The replacement leaves the file as it was.
    Unchanged,
}

impl EditError {
    /// Claude Code's message; [`EditError::FileMissing`] needs the working directory and is
    /// written by the caller.
    pub fn message(&self) -> String {
        match self {
            Self::NoChange => {
                "No changes to make: old_string and new_string are exactly the same.".to_string()
            }
            Self::FileMissing => "File does not exist.".to_string(),
            Self::FileExists => "Cannot create new file - file already exists.".to_string(),
            Self::NotFound { old_string } => {
                format!("String to replace not found in file.\nString: {old_string}")
            }
            Self::NotUnique {
                matches,
                old_string,
            } => format!(
                "Found {matches} matches of the string to replace, but replace_all is false. To replace all occurrences, set replace_all to true. To replace only one occurrence, please provide more context to uniquely identify the instance.\nString: {old_string}"
            ),
            Self::Unchanged => {
                "Original and edited file match exactly. Failed to apply edit.".to_string()
            }
        }
    }
}

/// Applies an edit as Claude Code does. `content` is the file with `\r\n` turned into `\n`,
/// or `None` when the file does not exist. An empty `old_string` creates a file, or fills
/// an empty one. Removing text (an empty `new_string`) also removes the line break after
/// it when `old_string` does not end with one.
pub fn apply_edit(
    content: Option<&str>,
    old_string: &str,
    new_string: &str,
    replace_all: bool,
) -> Result<String, EditError> {
    if old_string == new_string {
        return Err(EditError::NoChange);
    }
    let Some(content) = content else {
        return if old_string.is_empty() {
            Ok(new_string.to_string())
        } else {
            Err(EditError::FileMissing)
        };
    };
    if old_string.is_empty() {
        return if content.trim().is_empty() {
            Ok(new_string.to_string())
        } else {
            Err(EditError::FileExists)
        };
    }
    let matches = content.matches(old_string).count();
    if matches == 0 {
        return Err(EditError::NotFound {
            old_string: old_string.to_string(),
        });
    }
    if matches > 1 && !replace_all {
        return Err(EditError::NotUnique {
            matches,
            old_string: old_string.to_string(),
        });
    }
    let with_newline = format!("{old_string}\n");
    let search = if new_string.is_empty()
        && !old_string.ends_with('\n')
        && content.contains(&with_newline)
    {
        with_newline.as_str()
    } else {
        old_string
    };
    let updated = if replace_all {
        content.replace(search, new_string)
    } else {
        content.replacen(search, new_string, 1)
    };
    if updated == content {
        return Err(EditError::Unchanged);
    }
    Ok(updated)
}

/// The text the model receives after a successful edit.
pub fn success_message(file_path: &str, replace_all: bool) -> String {
    if replace_all {
        format!(
            "The file {file_path} has been updated. All occurrences were successfully replaced."
        )
    } else {
        format!("The file {file_path} has been updated successfully.")
    }
}

pub fn directory_message(file_path: &str) -> String {
    format!(
        "Cannot edit '{file_path}': the specified path is an existing directory. Use {BASH} ls to list files in this directory, or specify a filename path to edit a specific file."
    )
}

#[cfg(test)]
#[path = "edit_tests.rs"]
mod tests;
