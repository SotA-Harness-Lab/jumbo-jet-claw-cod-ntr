// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The fields of a skill's frontmatter that Claude Code acts on beyond its name, its
//! description and who may invoke it on its own initiative.

use crate::markdown::parse_document;
use crate::markdown::parse_frontmatter;

/// What a skill's frontmatter says about how it is offered and run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrontmatter {
    /// `user-invocable`: whether the user can run the skill as `/<name>` and finds it in
    /// the `/` menu. Only a literal `true` keeps it there once the field is present, as
    /// Claude Code reads it.
    pub user_invocable: bool,
}

impl Default for SkillFrontmatter {
    fn default() -> Self {
        Self {
            user_invocable: true,
        }
    }
}

/// Reads the fields of [`SkillFrontmatter`] from the text of a `SKILL.md` or command file.
/// A file without frontmatter, or whose frontmatter cannot be read, has the defaults.
pub fn skill_frontmatter(contents: &str) -> SkillFrontmatter {
    let Some(value) = parse_document(contents)
        .frontmatter
        .and_then(parse_frontmatter)
    else {
        return SkillFrontmatter::default();
    };
    SkillFrontmatter {
        user_invocable: value.get("user-invocable").is_none_or(is_true),
    }
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
