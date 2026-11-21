// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::HookMatcher;

fn matches(pattern: &str, input: &str) -> bool {
    HookMatcher::new(pattern)
        .expect("valid matcher")
        .matches(Some(input))
}

#[test]
fn comma_and_spaced_lists_are_exact_names() {
    for pattern in ["Edit, Write", "Edit,Write", "Edit | Write", " Edit ,Write "] {
        assert_eq!(
            (matches(pattern, "Edit"), matches(pattern, "Write")),
            (true, true),
            "{pattern:?}"
        );
        assert!(!matches(pattern, "Bash"), "{pattern:?}");
        assert!(!matches(pattern, "NotebookEdit"), "{pattern:?}");
    }
}

#[test]
fn hyphenated_names_match_exactly() {
    assert!(matches("code-reviewer", "code-reviewer"));
    assert!(!matches("code-reviewer", "senior-code-reviewer"));
    assert!(!matches(
        "mcp__brave-search",
        "mcp__brave-search__web_search"
    ));
    assert!(matches(
        "mcp__brave-search__web_search",
        "mcp__brave-search__web_search"
    ));
}

#[test]
fn other_characters_keep_the_regex_path() {
    assert!(matches(
        "mcp__brave-search__.*",
        "mcp__brave-search__web_search"
    ));
    assert!(matches("^Notebook", "NotebookEdit"));
}

#[test]
fn the_narrow_set_keeps_hyphens_spaces_and_commas_on_the_regex_path() {
    let narrow = |pattern: &str, input: &str| {
        HookMatcher::new_narrow(pattern)
            .expect("valid matcher")
            .matches(Some(input))
    };
    assert!(narrow("rate_limit|server_error", "server_error"));
    assert!(narrow(
        "rate_limit,server_error",
        "x rate_limit,server_error y"
    ));
    assert!(!narrow("rate_limit,server_error", "rate_limit"));
}
