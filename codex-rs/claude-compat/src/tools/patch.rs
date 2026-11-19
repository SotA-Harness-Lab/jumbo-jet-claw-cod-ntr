// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Turning a file's new content into an `apply_patch` patch, so that Claude Code's file
//! tools write through Codex's patch tool (its approval, sandbox, hooks and diff tracking).
//!
//! An update is one chunk spanning the whole file: the unchanged lines before and after the
//! change are context, so the chunk can only match the file at its start. The patch tool
//! reads the file again when it applies the chunk; a file changed in between does not
//! match and the write fails instead of landing in the wrong place.

const BEGIN: &str = "*** Begin Patch\n";
const END: &str = "*** End Patch\n";

/// Splits text into lines the way `apply_patch` does: a line ends at `\r\n`, `\n` or `\r`,
/// and the end of the text after a line ending starts no further line.
pub fn split_lines(text: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let bytes = text.as_bytes();
    let mut start = 0;
    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\r' if bytes.get(cursor + 1) == Some(&b'\n') => {
                lines.push(&text[start..cursor]);
                cursor += 2;
                start = cursor;
            }
            b'\r' | b'\n' => {
                lines.push(&text[start..cursor]);
                cursor += 1;
                start = cursor;
            }
            _ => cursor += 1,
        }
    }
    if start < bytes.len() {
        lines.push(&text[start..]);
    }
    lines
}

/// Whether a path can be written into a patch header.
pub fn is_patchable_path(path: &str) -> bool {
    !path.is_empty() && !path.contains(['\n', '\r']) && path.trim() == path
}

/// A patch that creates `path` with `content`. `apply_patch` ends every line it adds with
/// `\n`, so the file ends with a line break even when `content` does not.
pub fn add_file_patch(path: &str, content: &str) -> String {
    let mut patch = String::from(BEGIN);
    patch.push_str(&format!("*** Add File: {path}\n"));
    for line in split_lines(content) {
        patch.push('+');
        patch.push_str(line);
        patch.push('\n');
    }
    patch.push_str(END);
    patch
}

/// A patch that turns the file at `path`, now `old`, into `new`; `None` when the two have
/// the same lines, so there is nothing to apply. Lines `apply_patch` adds take the file's
/// first line ending, the lines left alone keep theirs, and the last line always ends with
/// a line break.
pub fn update_file_patch(path: &str, old: &str, new: &str) -> Option<String> {
    let old_lines = split_lines(old);
    let new_lines = split_lines(new);
    if old_lines == new_lines {
        return None;
    }
    let prefix = old_lines
        .iter()
        .zip(&new_lines)
        .take_while(|(old, new)| old == new)
        .count();
    let suffix = old_lines[prefix..]
        .iter()
        .rev()
        .zip(new_lines[prefix..].iter().rev())
        .take_while(|(old, new)| old == new)
        .count();

    let mut patch = String::from(BEGIN);
    patch.push_str(&format!("*** Update File: {path}\n@@\n"));
    let mut push = |marker: char, line: &str| {
        patch.push(marker);
        patch.push_str(line);
        patch.push('\n');
    };
    for line in &old_lines[..prefix] {
        push(' ', line);
    }
    for line in &old_lines[prefix..old_lines.len() - suffix] {
        push('-', line);
    }
    for line in &new_lines[prefix..new_lines.len() - suffix] {
        push('+', line);
    }
    for line in &old_lines[old_lines.len() - suffix..] {
        push(' ', line);
    }
    patch.push_str(END);
    Some(patch)
}

#[cfg(test)]
#[path = "patch_tests.rs"]
mod tests;
