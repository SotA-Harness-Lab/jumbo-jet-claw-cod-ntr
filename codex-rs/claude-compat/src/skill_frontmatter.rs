// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The fields of a skill's frontmatter that Claude Code acts on beyond its name, its
//! description and who may invoke it on its own initiative.

use crate::markdown::frontmatter_string;
use crate::markdown::parse_document;
use crate::markdown::parse_frontmatter;
use crate::rules::rule_globs;
use crate::tools::agent::tool_list_entries;

/// What a skill's frontmatter says about how it is offered and run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontmatter {
    /// `user-invocable`: whether the user can run the skill as `/<name>` and finds it in
    /// the `/` menu. Only a literal `true` keeps it there once the field is present, as
    /// Claude Code reads it.
    pub user_invocable: bool,
    /// `argument-hint`: what to type after `/<name>`, shown while the name is typed.
    pub argument_hint: Option<String>,
    /// `paths`: the globs of the files that bring the skill up, read as a rule's `paths:`
    /// are. `None` for a skill offered from the start.
    pub paths: Option<Vec<String>>,
    /// `allowed-tools`: permission rules (`Read`, `Bash(git add *)`) the turn that invokes
    /// the skill runs without asking, as written.
    pub allowed_tools: Vec<String>,
    /// `context: fork`: the skill runs in a subagent, whose answer is its result.
    pub context_fork: bool,
    /// `agent`: the subagent type a `context: fork` skill runs in.
    pub agent: Option<String>,
    /// `model`: the Claude model the skill runs with; `inherit` is none.
    pub model: Option<String>,
    /// `effort`: the effort level the skill runs with (`low`, `medium`, `high`, `xhigh`,
    /// `max`); another value is none.
    pub effort: Option<String>,
}

impl Default for SkillFrontmatter {
    fn default() -> Self {
        Self {
            user_invocable: true,
            argument_hint: None,
            paths: None,
            allowed_tools: Vec::new(),
            context_fork: false,
            agent: None,
            model: None,
            effort: None,
        }
    }
}

/// Reads the fields of [`SkillFrontmatter`] from the text of a `SKILL.md` or command file.
/// A file without frontmatter, or whose frontmatter cannot be read, has the defaults.
pub fn skill_frontmatter(contents: &str) -> SkillFrontmatter {
    let Some(frontmatter) = parse_document(contents).frontmatter else {
        return SkillFrontmatter::default();
    };
    let Some(value) = parse_frontmatter(frontmatter) else {
        return SkillFrontmatter::default();
    };
    SkillFrontmatter {
        user_invocable: value.get("user-invocable").is_none_or(is_true),
        argument_hint: value
            .get("argument-hint")
            .and_then(|hint| text_value(hint, frontmatter, "argument-hint")),
        paths: rule_globs(frontmatter),
        allowed_tools: value
            .get("allowed-tools")
            .map(tool_list_entries)
            .unwrap_or_default(),
        context_fork: value.get("context").and_then(serde_yaml::Value::as_str) == Some("fork"),
        agent: string_field(&value, "agent"),
        model: string_field(&value, "model").filter(|model| model != "inherit"),
        effort: string_field(&value, "effort").filter(|effort| {
            matches!(effort.as_str(), "low" | "medium" | "high" | "xhigh" | "max")
        }),
    }
}

/// A field's value as text, trimmed; none when it is missing, empty or not a scalar.
fn string_field(value: &serde_yaml::Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(frontmatter_string)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

/// A field's value as text. A scalar is taken as Claude Code turns it into a string; a
/// list or map, which YAML makes of a hint such as `[topic: foo | bar]`, is taken as
/// written in the frontmatter. An empty value is none.
fn text_value(value: &serde_yaml::Value, frontmatter: &str, key: &str) -> Option<String> {
    let text = match value {
        serde_yaml::Value::Null => return None,
        serde_yaml::Value::Sequence(_) | serde_yaml::Value::Mapping(_) => frontmatter
            .lines()
            .find_map(|line| line.strip_prefix(key)?.strip_prefix(':'))?
            .trim()
            .to_string(),
        other => frontmatter_string(other)?,
    };
    (!text.trim().is_empty()).then_some(text)
}

/// Claude Code's reading of a boolean field: `true`, as a boolean or as a string.
fn is_true(value: &serde_yaml::Value) -> bool {
    match value {
        serde_yaml::Value::Bool(flag) => *flag,
        serde_yaml::Value::String(text) => text == "true",
        _ => false,
    }
}

#[cfg(test)]
#[path = "skill_frontmatter_tests.rs"]
mod tests;
