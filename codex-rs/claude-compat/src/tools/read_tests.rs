// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::TextSlice;
use super::binary_extension;
use super::description;
use super::exceeds_token_limit;
use super::is_image;
use super::render;
use super::slice_text;
use super::too_large_message;

#[test]
fn whole_file_is_numbered_from_one() {
    let slice = slice_text("\u{feff}fn main() {\r\n}\n", None, None);
    assert_eq!(
        slice,
        TextSlice {
            content: "fn main() {\n}\n".to_string(),
            start_line: 1,
            num_lines: 3,
            total_lines: 3,
        }
    );
    assert_eq!(render(&slice), "1\tfn main() {\n2\t}\n3\t");
}

#[test]
fn offset_and_limit_select_lines() {
    let text = "a\nb\nc\nd\ne";
    let slice = slice_text(text, Some(2), Some(2));
    assert_eq!(slice.content, "b\nc");
    assert_eq!(slice.total_lines, 5);
    assert_eq!(render(&slice), "2\tb\n3\tc");
    assert_eq!(slice_text(text, Some(0), Some(1)).content, "a");
}

#[test]
fn empty_file_and_short_file_get_warnings() {
    assert_eq!(
        render(&slice_text("", None, None)),
        "<system-reminder>Warning: the file exists but the contents are empty.</system-reminder>"
    );
    assert_eq!(
        render(&slice_text("a\nb", Some(10), None)),
        "<system-reminder>Warning: the file exists but is shorter than the provided offset (10). The file has 2 lines.</system-reminder>"
    );
}

#[test]
fn file_kinds_follow_the_extension() {
    assert!(is_image("/tmp/Shot.PNG"));
    assert!(!is_image("/tmp/png"));
    assert_eq!(binary_extension("/a/b.tar.gz"), Some(".gz".to_string()));
    assert_eq!(binary_extension("/a/doc.pdf"), Some(".pdf".to_string()));
    assert_eq!(binary_extension("/a/.bashrc"), None);
    assert_eq!(binary_extension("/a/main.rs"), None);
}

#[test]
fn limits_and_messages() {
    assert!(!exceeds_token_limit(&"x".repeat(100_000)));
    assert!(exceeds_token_limit(&"x".repeat(100_001)));
    assert!(
        too_large_message(300 * 1024)
            .starts_with("File content (300KB) exceeds maximum allowed size (256KB).")
    );
    assert!(description(true).contains("read images"));
    assert!(!description(false).contains("read images"));
}
