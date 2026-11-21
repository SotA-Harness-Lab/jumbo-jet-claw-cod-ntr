// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: honour the if condition of Claude Code's hooks, and match
// lists and hyphenated names exactly as Claude Code does

//! Compiled hook matchers retain their original spelling for configuration identity.
//! Regex parsing happens during discovery; dispatch only evaluates admitted matchers.

#[derive(Debug, Clone)]
pub(crate) enum HookMatcher {
    All(String),
    Exact(String),
    Regex(regex::Regex),
    /// A matcher, or none, narrowed by the handler's `if` of a Claude Code hooks file: a
    /// permission rule such as `Bash(git *)` that the tool call has to meet as well.
    Conditional {
        matcher: Option<Box<HookMatcher>>,
        condition: String,
    },
}

impl HookMatcher {
    /// A matcher made only of letters, digits, `_`, `-`, spaces, `,` and `|` is a list of exact
    /// names separated by `|` or `,` (`Edit|Write`, `Edit, Write`, `code-reviewer`), as in
    /// Claude Code; anything else is an unanchored regular expression.
    pub(crate) fn new(pattern: &str) -> Result<Self, regex::Error> {
        Self::with_exact_set(pattern, |ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | ' ' | ',' | '|')
        })
    }

    /// The narrower rule Claude Code keeps for `StopFailure`: only letters, digits, `_` and `|`
    /// make an exact list, so a hyphen, space or comma leaves the matcher a regular expression.
    pub(crate) fn new_narrow(pattern: &str) -> Result<Self, regex::Error> {
        Self::with_exact_set(pattern, |ch| {
            ch.is_ascii_alphanumeric() || matches!(ch, '_' | '|')
        })
    }

    fn with_exact_set(pattern: &str, exact: impl Fn(char) -> bool) -> Result<Self, regex::Error> {
        if pattern.is_empty() || pattern == "*" {
            Ok(Self::All(pattern.to_string()))
        } else if pattern.chars().all(exact) {
            Ok(Self::Exact(pattern.to_string()))
        } else {
            regex::Regex::new(pattern).map(Self::Regex)
        }
    }

    /// `matcher` narrowed by an `if` condition.
    pub(crate) fn conditional(matcher: Option<Self>, condition: String) -> Self {
        Self::Conditional {
            matcher: matcher.map(Box::new),
            condition,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::All(pattern) | Self::Exact(pattern) => pattern,
            Self::Regex(regex) => regex.as_str(),
            Self::Conditional { matcher, .. } => matcher.as_deref().map_or("", Self::as_str),
        }
    }

    /// The handler's `if` condition, when it has one.
    pub(crate) fn condition(&self) -> Option<&str> {
        match self {
            Self::All(_) | Self::Exact(_) | Self::Regex(_) => None,
            Self::Conditional { condition, .. } => Some(condition),
        }
    }

    /// Whether the name matches; the `if` condition is checked apart, against the tool input.
    pub(crate) fn matches(&self, input: Option<&str>) -> bool {
        match self {
            Self::All(_) => true,
            Self::Exact(pattern) => input.is_some_and(|input| {
                pattern
                    .split(['|', ','])
                    .any(|candidate| candidate.trim() == input)
            }),
            Self::Regex(regex) => input.is_some_and(|input| regex.is_match(input)),
            Self::Conditional { matcher, .. } => matcher
                .as_deref()
                .is_none_or(|matcher| matcher.matches(input)),
        }
    }
}

impl PartialEq for HookMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str() && self.condition() == other.condition()
    }
}

impl Eq for HookMatcher {}

#[cfg(test)]
#[path = "matcher_tests.rs"]
mod tests;
