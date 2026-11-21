// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What a session's history shows of the files the model read and wrote with `Read`,
//! `Write` and `Edit`, so that a resumed session knows those files as the session that made
//! the calls did, as Claude Code rebuilds its file state from the transcript on resume.

use std::collections::HashMap;

use serde_json::Value;

use super::EDIT;
use super::READ;
use super::WRITE;
use super::edit;
use super::edit::EditInput;
use super::read;
use super::read::ReadInput;
use super::write;
use super::write::WriteInput;

/// A call of a Claude tool in a session's history, with the text the model received.
#[derive(Debug, Clone, Copy)]
pub struct PastCall<'a> {
    pub tool: &'a str,
    pub arguments: &'a str,
    pub output: &'a str,
}

/// The whole content (`\r\n` as `\n`) of each file the model knew whole after `calls`, in
/// the order of the calls: by the key `key` gives the call's `file_path`, the file used last
/// at the end. A file read whole has the content its numbered lines show, a file written the
/// content written, and a file edited its content before with the edit made again. A read of
/// some lines leaves what was known as it was; a call that failed changes nothing; an edit of
/// a file whose content is not known, or that cannot be made again, makes it unknown.
pub fn seen_files<'a>(
    calls: impl IntoIterator<Item = PastCall<'a>>,
    key: impl Fn(&str) -> Option<String>,
) -> Vec<(String, String)> {
    // Each file's content (`None` when it is no longer known) and the call that used it last.
    let mut files: HashMap<String, (Option<String>, usize)> = HashMap::new();
    for (order, call) in calls.into_iter().enumerate() {
        let Ok(arguments) = serde_json::from_str::<Value>(call.arguments) else {
            continue;
        };
        let (file_path, change) = match call.tool {
            READ => match ReadInput::parse(arguments) {
                Ok(input) => (input.file_path.clone(), Change::Read(input)),
                Err(_) => continue,
            },
            WRITE => match WriteInput::parse(arguments) {
                Ok(input) => (input.file_path.clone(), Change::Write(input)),
                Err(_) => continue,
            },
            EDIT => match EditInput::parse(arguments) {
                Ok(input) => (input.file_path.clone(), Change::Edit(input)),
                Err(_) => continue,
            },
            _ => continue,
        };
        let Some(key) = key(&file_path) else {
            continue;
        };
        let known = files.get(&key).and_then(|(content, _)| content.as_deref());
        let content = match change {
            Change::Read(input) => {
                if !reads_whole_text(&input) {
                    continue;
                }
                match read_content(call.output) {
                    Some(content) => Some(content),
                    None => continue,
                }
            }
            Change::Write(input) => {
                if call.output != write::created_message(&input.file_path)
                    && call.output != write::updated_message(&input.file_path)
                {
                    continue;
                }
                Some(input.content.replace("\r\n", "\n"))
            }
            Change::Edit(input) => {
                let replace_all = input.replace_all.unwrap_or(false);
                if call.output != edit::success_message(&input.file_path, replace_all) {
                    continue;
                }
                edit_again(known, &input, replace_all)
            }
        };
        files.insert(key, (content, order));
    }
    let mut known: Vec<(usize, String, String)> = files
        .into_iter()
        .filter_map(|(key, (content, order))| content.map(|content| (order, key, content)))
        .collect();
    known.sort_by_key(|(order, _, _)| *order);
    known
        .into_iter()
        .map(|(_, key, content)| (key, content))
        .collect()
}

enum Change {
    Read(ReadInput),
    Write(WriteInput),
    Edit(EditInput),
}

/// Whether the read returned every line of a text file as numbered lines.
fn reads_whole_text(input: &ReadInput) -> bool {
    let path = input.file_path.as_str();
    input.offset.is_none_or(|offset| offset <= 1)
        && input.limit.is_none()
        && input.pages.is_none()
        && !read::is_image(path)
        && !read::is_pdf(path)
        && !read::is_notebook(path)
        && read::binary_extension(path).is_none()
}

/// The file's content from the text a read of all of it returned: its numbered lines, or
/// the empty-file reminder. `None` for any other text (an error, a hook's replacement).
fn read_content(output: &str) -> Option<String> {
    if output == read::render(&read::slice_text("", None, None)) {
        return Some(String::new());
    }
    let mut lines = Vec::new();
    for (index, line) in output.split('\n').enumerate() {
        let (number, text) = line.split_once('\t')?;
        if number != (index + 1).to_string() {
            return None;
        }
        lines.push(text);
    }
    Some(lines.join("\n"))
}

/// The edit made again on `known`. A read's lines do not show whether the file ends in a
/// line break, so an edit that does not apply without one is tried with one.
fn edit_again(known: Option<&str>, input: &EditInput, replace_all: bool) -> Option<String> {
    let apply = |content: Option<&str>| {
        edit::apply_edit(content, &input.old_string, &input.new_string, replace_all).ok()
    };
    match known {
        None => apply(None),
        Some(content) => apply(Some(content)).or_else(|| {
            (!content.ends_with('\n'))
                .then(|| apply(Some(&format!("{content}\n"))))
                .flatten()
        }),
    }
}

#[cfg(test)]
#[path = "seen_files_tests.rs"]
mod tests;
