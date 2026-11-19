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
        r#"
- Allows Claude to search the web and use the results to inform responses
- Provides up-to-date information for current events and recent data
- Returns search result information formatted as search result blocks, including links as markdown hyperlinks
- Use this tool for accessing information beyond Claude's knowledge cutoff
- Searches are performed automatically within a single API call

CRITICAL REQUIREMENT - You MUST follow this:
  - After answering the user's question, you MUST include a "Sources:" section at the end of your response
  - In the Sources section, list all relevant URLs from the search results as markdown hyperlinks: [Title](URL)
  - This is MANDATORY - never skip including sources in your response
  - Example format:

    [Your answer here]

    Sources:
    - [Source Title 1](https://example.com/1)
    - [Source Title 2](https://example.com/2)

Usage notes:
  - Domain filtering is supported to include or block specific websites

IMPORTANT - Use the correct year in search queries:
  - The current month is {month_year}. You MUST use this year when searching for recent information, documentation, or current events.
  - Example: If the user asks for "latest React docs", search for "React documentation" with the current year, NOT last year
"#
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
        "Web search results for query: \"{query}\"\n\n{results}\n\n\nREMINDER: You MUST include the sources above in your response to the user using markdown hyperlinks."
    )
}

#[cfg(test)]
#[path = "web_search_tests.rs"]
mod tests;
