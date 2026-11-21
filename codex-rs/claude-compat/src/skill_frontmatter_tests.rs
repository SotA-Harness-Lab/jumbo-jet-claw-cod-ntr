// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::SkillFrontmatter;
use super::skill_frontmatter;

fn user_invocable(frontmatter: &str) -> bool {
    skill_frontmatter(&format!("---\n{frontmatter}\n---\nBody\n")).user_invocable
}

#[test]
fn only_a_literal_true_keeps_a_skill_user_invocable() {
    assert_eq!(user_invocable("name: demo"), true);
    assert_eq!(user_invocable("user-invocable: true"), true);
    assert_eq!(user_invocable("user-invocable: \"true\""), true);
    assert_eq!(user_invocable("user-invocable: false"), false);
    assert_eq!(user_invocable("user-invocable: yes"), false);
    assert_eq!(user_invocable("user-invocable:"), false);
    assert_eq!(
        skill_frontmatter("No frontmatter\n"),
        SkillFrontmatter::default()
    );
}
