// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_extension_api::ExtensionData;
use pretty_assertions::assert_eq;

use super::grant_skill_tools;
use super::granted_skill_tools;
use super::skill_tool_grants;

#[test]
fn rules_are_granted_once_for_the_turn_whose_store_holds_them() {
    let rules = vec!["Read".to_string(), "Bash(npm test)".to_string()];
    let turn_store = ExtensionData::new("turn");
    assert_eq!(granted_skill_tools(&turn_store), Vec::<String>::new());
    grant_skill_tools(&turn_store, rules.clone());
    grant_skill_tools(&turn_store, rules.clone());
    assert_eq!(granted_skill_tools(&turn_store), rules);
    // Another turn has a store of its own.
    assert_eq!(
        granted_skill_tools(&ExtensionData::new("next turn")),
        Vec::<String>::new()
    );
}

#[test]
fn the_skill_directory_is_filled_into_the_rules() {
    if !codex_claude_compat::enabled() {
        return;
    }
    let contents =
        "---\nname: render\nallowed-tools: Read Bash(${CLAUDE_SKILL_DIR}/run.sh *)\n---\nRun it.\n";
    assert_eq!(
        skill_tool_grants(contents, "/skills/render/SKILL.md"),
        vec![
            "Read".to_string(),
            "Bash(/skills/render/run.sh *)".to_string()
        ]
    );
}
