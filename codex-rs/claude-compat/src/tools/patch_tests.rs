// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::add_file_patch;
use super::is_patchable_path;
use super::split_lines;
use super::update_file_patch;

#[test]
fn lines_split_on_every_line_ending() {
    assert_eq!(split_lines("a\r\nb\rc\nd"), vec!["a", "b", "c", "d"]);
    assert_eq!(split_lines("a\n"), vec!["a"]);
    assert_eq!(split_lines("a\n\n"), vec!["a", ""]);
    assert_eq!(split_lines(""), Vec::<&str>::new());
}

#[test]
fn update_keeps_unchanged_lines_as_context() {
    assert_eq!(
        update_file_patch("/w/f.txt", "one\ntwo\nthree\n", "one\n2\nthree\n"),
        Some(
            "*** Begin Patch\n*** Update File: /w/f.txt\n@@\n one\n-two\n+2\n three\n*** End Patch\n"
                .to_string()
        )
    );
}

#[test]
fn file_lines_that_look_like_patch_markers_stay_content() {
    let patch =
        update_file_patch("/w/f", "*** End Patch\n@@\n", "*** End Patch\nx\n").expect("patch");
    assert_eq!(
        patch,
        "*** Begin Patch\n*** Update File: /w/f\n@@\n *** End Patch\n-@@\n+x\n*** End Patch\n"
    );
}

#[test]
fn identical_lines_need_no_patch() {
    assert_eq!(update_file_patch("/w/f", "a\r\nb\r\n", "a\nb\n"), None);
}

#[test]
fn empty_files_are_filled_and_emptied() {
    assert_eq!(
        update_file_patch("/w/f", "", "x\n"),
        Some("*** Begin Patch\n*** Update File: /w/f\n@@\n+x\n*** End Patch\n".to_string())
    );
    assert_eq!(
        update_file_patch("/w/f", "x\n", ""),
        Some("*** Begin Patch\n*** Update File: /w/f\n@@\n-x\n*** End Patch\n".to_string())
    );
}

#[test]
fn new_file_lines_are_added() {
    assert_eq!(
        add_file_patch("/w/new.txt", "a\nb"),
        "*** Begin Patch\n*** Add File: /w/new.txt\n+a\n+b\n*** End Patch\n"
    );
    assert_eq!(
        add_file_patch("/w/empty", ""),
        "*** Begin Patch\n*** Add File: /w/empty\n*** End Patch\n"
    );
}

#[test]
fn paths_with_line_breaks_or_edge_spaces_are_refused() {
    assert!(is_patchable_path("/w/a b.txt"));
    assert!(!is_patchable_path("/w/a\nb"));
    assert!(!is_patchable_path("/w/a "));
}
