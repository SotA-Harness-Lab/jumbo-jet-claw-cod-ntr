// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_claude_compat::tools::edit::MODIFIED_SINCE_READ_MESSAGE;
use codex_claude_compat::tools::edit::NOT_READ_MESSAGE;
use pretty_assertions::assert_eq;

use super::ContentDigest;
use super::FileRecord;
use super::FileStamp;
use super::MAX_RECORDS;
use super::ReadFiles;
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
