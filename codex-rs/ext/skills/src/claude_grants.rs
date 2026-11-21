// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: a skill's `allowed-tools`.
//!
//! Claude Code runs the tools a skill's frontmatter lists in `allowed-tools` without asking
//! during the turn that invokes the skill (by `/<name>`, or by the model through the
//! `Skill` tool); the grant ends with the turn. The rules are kept in the turn's store, and
//! the Claude tools' permission check adds them as allow rules.

use std::path::Path;
use std::sync::Mutex;

use codex_extension_api::ExtensionData;

const SKILL_DIR_PLACEHOLDER: &str = "${CLAUDE_SKILL_DIR}";

/// The rules granted in one turn.
#[derive(Default)]
struct SkillToolGrants(Mutex<Vec<String>>);

/// The permission rules the `allowed-tools` of the skill whose `SKILL.md` at
/// `skill_md_path` holds `contents` grants, with `${CLAUDE_SKILL_DIR}` replaced by the
/// skill's directory, as Claude Code replaces it in `Bash` rules. None with
/// `CCN_CLAUDE_COMPAT=0`.
pub fn skill_tool_grants(contents: &str, skill_md_path: &str) -> Vec<String> {
    if !codex_claude_compat::enabled() {
        return Vec::new();
    }
    let skill_dir = Path::new(skill_md_path)
        .parent()
        .map(|dir| dir.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    codex_claude_compat::skill_frontmatter(contents)
        .allowed_tools
        .into_iter()
        .map(|rule| rule.replace(SKILL_DIR_PLACEHOLDER, &skill_dir))
        .collect()
}

/// Grants `rules` for the rest of the turn whose store is `turn_store`.
pub fn grant_skill_tools(turn_store: &ExtensionData, rules: Vec<String>) {
    if rules.is_empty() {
        return;
    }
    let grants = turn_store.get_or_init(SkillToolGrants::default);
    let mut granted = grants
        .0
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    for rule in rules {
        if !granted.contains(&rule) {
            granted.push(rule);
        }
    }
}

/// The rules granted so far in the turn whose store is `turn_store`.
pub fn granted_skill_tools(turn_store: &ExtensionData) -> Vec<String> {
    turn_store
        .get::<SkillToolGrants>()
        .map(|grants| {
            grants
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone()
        })
        .unwrap_or_default()
}

#[cfg(test)]
#[path = "claude_grants_tests.rs"]
mod tests;
