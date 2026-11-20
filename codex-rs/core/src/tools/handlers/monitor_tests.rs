// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use regex_lite::Regex;

use super::LineBuffer;
use super::MAX_LINES_PER_EVENT;
use super::Pending;

#[test]
fn bytes_become_lines_whatever_the_chunks() {
    let mut buffer = LineBuffer::default();
    let mut lines = Vec::new();
    for chunk in ["first li", "ne\nsecond\r\nthi", "rd"] {
        buffer.push(chunk.as_bytes(), |line| lines.push(line.to_string()));
    }
    assert_eq!(lines, vec!["first line", "second\r"]);
    buffer.finish(|line| lines.push(line.to_string()));
    assert_eq!(lines, vec!["first line", "second\r", "third"]);
}

#[test]
fn only_matching_lines_wait_and_no_more_than_an_event_holds() {
    let pattern = Regex::new("ERROR|WARN").expect("regex");
    let mut pending = Pending::default();
    for line in ["INFO started", "ERROR disk full\r", "", "WARN slow"] {
        pending.push(line, Some(&pattern));
    }
    assert_eq!(pending.lines, vec!["ERROR disk full", "WARN slow"]);

    let mut pending = Pending::default();
    for index in 0..MAX_LINES_PER_EVENT + 3 {
        pending.push(&format!("line {index}"), None);
    }
    assert_eq!((pending.lines.len(), pending.omitted), (MAX_LINES_PER_EVENT, 3));
}
