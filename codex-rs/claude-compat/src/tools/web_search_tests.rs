// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::description;
use super::render;
use super::search_commands;
use super::validate;

#[test]
fn query_must_have_two_characters() {
    assert!(validate("a", None, None).is_err());
    assert_eq!(validate("ab", None, None), Ok(()));
}

#[test]
fn allowed_and_blocked_domains_exclude_each_other() {
    let allowed = vec!["a.com".to_string()];
    let blocked = vec!["b.com".to_string()];
    assert_eq!(
        validate("rust", Some(&allowed), Some(&blocked)),
        Err(
            "Error: Cannot specify both allowed_domains and blocked_domains in the same request"
                .to_string()
        )
    );
    assert_eq!(validate("rust", Some(&allowed), Some(&[])), Ok(()));
}

#[test]
fn allowed_domains_become_the_search_filter() {
    let allowed = vec!["docs.rs".to_string()];
    assert_eq!(
        search_commands("serde derive", Some(&allowed), None),
        json!({ "search_query": [{ "q": "serde derive", "domains": ["docs.rs"] }] })
    );
    assert_eq!(
        search_commands("serde derive", None, None),
        json!({ "search_query": [{ "q": "serde derive" }] })
    );
}

#[test]
fn blocked_domains_are_excluded_in_the_query() {
    let blocked = vec!["pinterest.com".to_string(), " ".to_string()];
    assert_eq!(
        search_commands("cake", None, Some(&blocked)),
        json!({ "search_query": [{ "q": "cake -site:pinterest.com" }] })
    );
}

#[test]
fn results_are_wrapped_like_claude_code() {
    assert_eq!(
        render("rust", "1. [Rust](https://rust-lang.org)\n"),
        "Web search results for query: \"rust\"\n\n1. [Rust](https://rust-lang.org)\n\n\nREMINDER: You MUST include the sources above in your response to the user using markdown hyperlinks."
    );
    assert!(render("rust", "  ").contains("No search results found."));
}

#[test]
fn description_names_the_current_month() {
    assert!(description("November 2026").contains("The current month is November 2026."));
}
