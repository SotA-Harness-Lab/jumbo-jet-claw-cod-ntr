// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `WebSearch` tool: description, parameters, the query handed to Codex's
//! search and the result text.
//!
//! The search itself is Codex's standalone search tool (`web.run`); this module only maps
//! Claude Code's parameters onto its `search_query` command and wraps its answer.

use serde_json::Value;
use serde_json::json;

pub const QUERY_PARAM_DESCRIPTION: &str = "The search query to use";
pub const ALLOWED_DOMAINS_PARAM_DESCRIPTION: &str =
    "Only include search results from these domains";
pub const BLOCKED_DOMAINS_PARAM_DESCRIPTION: &str =
    "Never include search results from these domains";

/// Shortest query Claude Code accepts.
pub const MIN_QUERY_LENGTH: usize = 2;

/// The tool description, after Claude Code's. `month_year` is the current month, such as
/// `November 2026`.
pub fn description(month_year: &str) -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/web_search/description.md"),
        month_year = month_year
    )
}

/// Claude Code's checks before searching; the message is its error.
pub fn validate(
    query: &str,
    allowed_domains: Option<&[String]>,
    blocked_domains: Option<&[String]>,
) -> Result<(), String> {
    if query.chars().count() < MIN_QUERY_LENGTH {
        return Err(format!(
            "InputValidationError: WebSearch failed due to the following issue:\nThe parameter `query` must be at least {MIN_QUERY_LENGTH} characters long"
        ));
    }
    if allowed_domains.is_some_and(|domains| !domains.is_empty())
        && blocked_domains.is_some_and(|domains| !domains.is_empty())
    {
        return Err(
            "Error: Cannot specify both allowed_domains and blocked_domains in the same request"
                .to_string(),
        );
    }
    Ok(())
}

/// The arguments of Codex's `web.run` for one Claude Code search: `allowed_domains` is the
/// search's own domain filter; blocked domains, which it has no filter for, are excluded
/// with `-site:` operators in the query.
pub fn search_commands(
    query: &str,
    allowed_domains: Option<&[String]>,
    blocked_domains: Option<&[String]>,
) -> Value {
    let mut q = query.to_string();
    for domain in blocked_domains.unwrap_or_default() {
        let domain = domain.trim();
        if !domain.is_empty() {
            q.push_str(" -site:");
            q.push_str(domain);
        }
    }
    let mut search = json!({ "q": q });
    if let Some(domains) = allowed_domains.filter(|domains| !domains.is_empty()) {
        search["domains"] = json!(domains);
    }
    json!({ "search_query": [search] })
}

/// Claude Code's result text around the search's answer.
pub fn render(query: &str, results: &str) -> String {
    let results = results.trim();
    let results = if results.is_empty() {
        "No search results found."
    } else {
        results
    };
    format!(
        codex_prompt_texts::text!("claude-compat/tools/web_search/search_results.md"),
        query = query,
        results = results
    )
}

#[cfg(test)]
#[path = "web_search_tests.rs"]
mod tests;
