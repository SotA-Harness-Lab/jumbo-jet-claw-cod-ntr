// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_claude_compat::tools::edit::MODIFIED_SINCE_READ_MESSAGE;
use codex_claude_compat::tools::edit::NOT_READ_MESSAGE;
use codex_protocol::models::ResponseItem;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::ContentDigest;
use super::FileRecord;
use super::FileStamp;
use super::MAX_RECORDS;
use super::ReadFiles;
use super::restore_into;
use super::write_refusal;

fn stamp(modified_at_ms: i64, size: u64) -> FileStamp {
    FileStamp {
        modified_at_ms,
        size,
    }
}

#[test]
fn a_file_never_read_is_refused() {
    assert_eq!(
        write_refusal(None, stamp(10, 1), "x"),
        Some(NOT_READ_MESSAGE)
    );
}

#[test]
fn a_file_read_whole_has_changed_only_if_its_content_has() {
    let read = FileRecord::Whole(ContentDigest::of("a\r\nb\n"));
    // A newer modification time alone, as a formatter or `touch` leaves it.
    assert_eq!(write_refusal(Some(&read), stamp(99, 4), "a\nb\n"), None);
    assert_eq!(write_refusal(Some(&read), stamp(99, 3), "a\nb"), None);
    // New content with the modification time it had, as `cp -p` or a write within the
    // same millisecond leaves it.
    assert_eq!(
        write_refusal(Some(&read), stamp(10, 4), "a\nc\n"),
        Some(MODIFIED_SINCE_READ_MESSAGE)
    );
}

#[test]
fn a_file_read_in_part_has_changed_when_its_time_or_size_has() {
    let read = FileRecord::Part(stamp(10, 4));
    assert_eq!(write_refusal(Some(&read), stamp(10, 4), "a\nb\n"), None);
    for now in [stamp(11, 4), stamp(9, 4), stamp(10, 5)] {
        assert_eq!(
            write_refusal(Some(&read), now, "a\nb\n"),
            Some(MODIFIED_SINCE_READ_MESSAGE),
            "{now:?}"
        );
    }
}

#[test]
fn the_files_used_longest_ago_are_forgotten() {
    let files = ReadFiles::default();
    let record = FileRecord::Part(stamp(1, 1));
    for index in 0..MAX_RECORDS {
        files.insert(index.to_string(), record.clone());
    }
    assert_eq!(files.get("0"), Some(record.clone()));
    files.insert("new".to_string(), record.clone());
    assert_eq!(files.get("0"), Some(record.clone()));
    assert_eq!(files.get("1"), None);
    assert_eq!(files.get("new"), Some(record));
}

fn item(value: serde_json::Value) -> ResponseItem {
    serde_json::from_value(value).expect("a response item")
}

#[test]
fn a_resumed_session_knows_the_files_its_history_read_whole() {
    let dir = tempfile::tempdir().expect("tempdir");
    let cwd = PathUri::from_host_native_path(dir.path()).expect("cwd");
    let history = [
        item(json!({
            "type": "function_call",
            "name": "Read",
            "arguments": json!({ "file_path": "notes.txt" }).to_string(),
            "call_id": "read-1",
        })),
        item(json!({
            "type": "function_call_output",
            "call_id": "read-1",
            "output": "1\tfirst\n2\tsecond",
        })),
        item(json!({
            "type": "function_call",
            "name": "Read",
            "arguments": json!({ "file_path": "part.txt", "offset": 3 }).to_string(),
            "call_id": "read-2",
        })),
        item(json!({
            "type": "function_call_output",
            "call_id": "read-2",
            "output": "3\tthird",
        })),
    ];
    let files = ReadFiles::default();
    restore_into(&files, &cwd, &history);

    let key = cwd.join("notes.txt").expect("path").to_string();
    let record = files.get(&key).expect("a record of the whole read");
    assert_eq!(
        write_refusal(Some(&record), stamp(1, 1), "first\r\nsecond\r\n"),
        None
    );
    assert_eq!(
        write_refusal(Some(&record), stamp(1, 1), "first\nchanged\n"),
        Some(MODIFIED_SINCE_READ_MESSAGE)
    );
    let part = cwd.join("part.txt").expect("path").to_string();
    assert_eq!(files.get(&part), None);
}
