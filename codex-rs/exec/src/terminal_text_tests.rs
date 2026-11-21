// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use super::*;
use pretty_assertions::assert_eq;

#[test]
fn plain_text_is_borrowed() {
    let text = "line one\n\tline two\n";

    assert!(matches!(keep_only_styling(text), Cow::Borrowed(_)));
}

#[test]
fn styling_is_kept() {
    let text = "\u{1b}[1;31mred\u{1b}[0m and \u{1b}[38:5:208morange\u{1b}[m";

    assert_eq!(keep_only_styling(text), text);
}

#[test]
fn osc_sequences_are_removed() {
    let text = "a\u{1b}]52;c;aGVsbG8=\u{7}b\u{1b}]0;title\u{1b}\\c\u{1b}]8;;https://x\u{9c}d";

    assert_eq!(keep_only_styling(text), "abcd");
}

#[test]
fn cursor_and_erase_sequences_are_removed() {
    let text = "keep\u{1b}[2J\u{1b}[H\u{1b}[1A\u{1b}[?1049h\u{1b}[6n\u{1b}7\u{1b}8\u{1b}(0 this";

    assert_eq!(keep_only_styling(text), "keep this");
}

#[test]
fn other_controls_are_removed() {
    let text = "a\u{7}b\u{8}c\rd\u{0}e\u{7f}f\u{9b}2Jg\u{9d}0;t\u{7}h\r\n";

    assert_eq!(keep_only_styling(text), "abcdefgh\r\n");
}

#[test]
fn cut_short_sequences_are_removed() {
    assert_eq!(keep_only_styling("a\u{1b}[31"), "a");
    assert_eq!(keep_only_styling("a\u{1b}]0;never ends"), "a");
    assert_eq!(keep_only_styling("a\u{1b}"), "a");
}

#[test]
fn text_for_a_pipe_is_unchanged() {
    let text = "a\u{1b}]0;title\u{7}b";

    assert_eq!(for_terminal(text, /*is_terminal*/ false), text);
    assert_eq!(for_terminal(text, /*is_terminal*/ true), "ab");
}
