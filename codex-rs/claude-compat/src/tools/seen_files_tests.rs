// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::PastCall;
use super::seen_files;
use crate::tools::edit::success_message;
use crate::tools::write::created_message;
use crate::tools::write::updated_message;

struct Call {
    tool: &'static str,
    arguments: String,
    output: String,
}

fn call(tool: &'static str, arguments: serde_json::Value, output: impl Into<String>) -> Call {
    Call {
        tool,
        arguments: arguments.to_string(),
        output: output.into(),
    }
}

/// The files `calls` leave known, with paths taken as they are given.
fn seen(calls: &[Call]) -> Vec<(String, String)> {
    seen_files(
        calls.iter().map(|call| PastCall {
            tool: call.tool,
            arguments: &call.arguments,
            output: &call.output,
        }),
        |file_path| Some(file_path.to_string()),
    )
}

#[test]
fn a_whole_read_gives_the_content_its_lines_show() {
    let calls = [
        call(
            "Read",
            json!({ "file_path": "/a.rs" }),
            "1\tfn main() {\n2\t}",
        ),
        call(
            "Read",
            json!({ "file_path": "/empty" }),
            "<system-reminder>Warning: the file exists but the contents are empty.</system-reminder>",
        ),
    ];
    assert_eq!(
        seen(&calls),
        vec![
            ("/a.rs".to_string(), "fn main() {\n}".to_string()),
            ("/empty".to_string(), String::new()),
        ]
    );
}

#[test]
fn reads_of_some_lines_and_results_that_are_not_lines_are_left_out() {
    let calls = [
        call(
            "Read",
            json!({ "file_path": "/a", "offset": 5, "limit": 2 }),
            "5\tx\n6\ty",
        ),
        call(
            "Read",
            json!({ "file_path": "/b" }),
            "File content (30000 tokens) exceeds",
        ),
        call("Read", json!({ "file_path": "/c" }), "1\tone\n3\tthree"),
        call("Read", json!({ "file_path": "/d.ipynb" }), "1\t{"),
        call("Bash", json!({ "command": "cat /e" }), "1\te"),
    ];
    assert_eq!(seen(&calls), Vec::new());
}

#[test]
fn writes_and_edits_are_followed_and_failures_ignored() {
    let calls = [
        call(
            "Write",
            json!({ "file_path": "/a", "content": "one\r\ntwo\r\n" }),
            created_message("/a"),
        ),
        call(
            "Edit",
            json!({ "file_path": "/a", "old_string": "two", "new_string": "2" }),
            success_message("/a", false),
        ),
        call(
            "Edit",
            json!({ "file_path": "/a", "old_string": "zzz", "new_string": "y" }),
            "<tool_use_error>String to replace not found in file.</tool_use_error>",
        ),
        call("Read", json!({ "file_path": "/b" }), "1\tlast line"),
        call(
            "Edit",
            json!({ "file_path": "/b", "old_string": "last line\n", "new_string": "end\n" }),
            success_message("/b", false),
        ),
        call(
            "Write",
            json!({ "path": "/c", "file_text": "c" }),
            updated_message("/c"),
        ),
    ];
    assert_eq!(
        seen(&calls),
        vec![
            ("/a".to_string(), "one\n2\n".to_string()),
            ("/b".to_string(), "end\n".to_string()),
            ("/c".to_string(), "c".to_string()),
        ]
    );
}

#[test]
fn an_edit_of_a_file_never_seen_whole_leaves_it_unknown() {
    let calls = [
        call("Read", json!({ "file_path": "/a" }), "1\tx"),
        call(
            "Write",
            json!({ "file_path": "/a", "content": "y" }),
            "PostToolUse hook blocked the tool result",
        ),
        call(
            "Edit",
            json!({ "file_path": "/b", "old_string": "x", "new_string": "y" }),
            success_message("/b", false),
        ),
        call(
            "Edit",
            json!({ "file_path": "/a", "old_string": "q", "new_string": "r" }),
            success_message("/a", false),
        ),
    ];
    assert_eq!(seen(&calls), Vec::new());
}
