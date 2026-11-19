// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::format_file_size;
use super::tool_use_error;

#[test]
fn file_sizes_read_as_in_claude_code() {
    assert_eq!(format_file_size(512), "512 bytes");
    assert_eq!(format_file_size(256 * 1024), "256KB");
    assert_eq!(format_file_size(1536), "1.5KB");
    assert_eq!(format_file_size(3 * 1024 * 1024), "3MB");
    assert_eq!(format_file_size(5 * 1024 * 1024 * 1024 / 2), "2.5GB");
}

#[test]
fn rejected_input_is_wrapped() {
    assert_eq!(
        tool_use_error("No changes"),
        "<tool_use_error>No changes</tool_use_error>"
    );
}
