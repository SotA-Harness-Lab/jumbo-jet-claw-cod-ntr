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

#[test]
fn the_argument_hint_is_read_as_text() {
    let hint = |frontmatter: &str| {
        skill_frontmatter(&format!("---\n{frontmatter}\n---\nBody\n")).argument_hint
    };
    assert_eq!(hint("name: demo"), None);
    assert_eq!(hint("argument-hint: <file>"), Some("<file>".to_string()));
    assert_eq!(hint("argument-hint: 42"), Some("42".to_string()));
    assert_eq!(
        hint("argument-hint: [filename] [format]"),
        Some("[filename] [format]".to_string())
    );
    // YAML reads this as a list holding a map; it is shown as written.
    assert_eq!(
        hint("argument-hint: [topic: foo | bar]"),
        Some("[topic: foo | bar]".to_string())
    );
    assert_eq!(hint("argument-hint:"), None);
}
