// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: read Claude Code skill frontmatter

use pretty_assertions::assert_eq;

use super::ParsedSkillFrontmatter;
use super::parse_skill_frontmatter_metadata;

#[test]
fn parses_repairs_and_sanitizes_frontmatter() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname:  deploy  service\ndescription: Build for AWS: ECS\nmetadata:\n  short-description:  Deploy   safely\n---\n",
        || "fallback".to_string(),
    )
    .expect("valid frontmatter");

    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "deploy service".to_string(),
            description: "Build for AWS: ECS".to_string(),
            short_description: Some("Deploy safely".to_string()),
            disable_model_invocation: false,
        }
    );
}

#[test]
fn uses_default_name_and_requires_description() {
    let parsed = parse_skill_frontmatter_metadata("---\ndescription: Demo skill\n---\n", || {
        "demo".to_string()
    })
    .expect("valid frontmatter");
    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "demo".to_string(),
            description: "Demo skill".to_string(),
            short_description: None,
            disable_model_invocation: false,
        }
    );

    let error =
        parse_skill_frontmatter_metadata("---\nname: demo\n---\n", || "fallback".to_string())
            .expect_err("description should be required");
    assert_eq!(error.to_string(), "missing field `description`");
}

#[test]
fn repairs_short_descriptions_containing_colons_and_apostrophes() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname: short\ndescription: Short skill\nmetadata:\n  short-description: What's included: builds and tests\n---\n",
        || "fallback".to_string(),
    )
    .expect("frontmatter with an unquoted short description should be repaired");

    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "short".to_string(),
            description: "Short skill".to_string(),
            short_description: Some("What's included: builds and tests".to_string()),
            disable_model_invocation: false,
        }
    );
}

#[test]
fn repairs_unrecognized_frontmatter_fields_that_need_quotes() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname: unknown\ndescription: Unknown fields\nargument-hint: <duration: e.g. 7d, 2w>\ntags: [next,@supabase/ssr]\n---\n",
        || "fallback".to_string(),
    )
    .expect("frontmatter with unrecognized fields should be repaired");

    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "unknown".to_string(),
            description: "Unknown fields".to_string(),
            short_description: None,
            disable_model_invocation: false,
        }
    );
}

#[test]
fn preserves_block_scalar_bodies_while_repairing_other_fields() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname: block\ndescription: |-\n  Build for AWS: ECS\nargument-hint: <duration: e.g. 7d>\n---\n",
        || "fallback".to_string(),
    )
    .expect("frontmatter repair should preserve block scalar bodies");

    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "block".to_string(),
            description: "Build for AWS: ECS".to_string(),
            short_description: None,
            disable_model_invocation: false,
        }
    );
}

#[test]
fn preserves_overlong_descriptions_and_short_descriptions() {
    let description = "💡".repeat(/*n*/ 1_025);
    let short_description = "x".repeat(/*n*/ 1_025);
    let parsed = parse_skill_frontmatter_metadata(
        &format!(
            "---\nname: long\ndescription: {description}\nmetadata:\n  short-description: {short_description}\n---\n"
        ),
        || "fallback".to_string(),
    )
    .expect("descriptions should not be truncated during frontmatter parsing");

    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "long".to_string(),
            description,
            short_description: Some(short_description),
            disable_model_invocation: false,
        }
    );
}

#[test]
fn reads_claude_code_frontmatter() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname: review\ndescription: Review a change\nwhen_to_use: Before a merge\ndisable-model-invocation: true\nallowed-tools: Bash(git diff:*)\n---\nBody\n",
        || "fallback".to_string(),
    )
    .expect("valid frontmatter");
    assert_eq!(
        parsed,
        ParsedSkillFrontmatter {
            name: "review".to_string(),
            description: "Review a change - Before a merge".to_string(),
            short_description: None,
            disable_model_invocation: true,
        }
    );

    for (value, expected) in [("\"true\"", true), ("false", false), ("yes", false)] {
        let parsed = parse_skill_frontmatter_metadata(
            &format!("---\ndescription: Demo\ndisable-model-invocation: {value}\n---\n"),
            || "demo".to_string(),
        )
        .expect("valid frontmatter");
        assert_eq!(parsed.disable_model_invocation, expected, "{value}");
    }
}

#[test]
fn takes_a_missing_description_from_the_body() {
    let parsed = parse_skill_frontmatter_metadata(
        "---\nname: demo\n---\n\n# Deploy   the service\n\nMore text.\n",
        || "fallback".to_string(),
    )
    .expect("the body should supply the description");
    assert_eq!(parsed.description, "Deploy the service");

    let long_line = "word ".repeat(40);
    let parsed =
        parse_skill_frontmatter_metadata(&format!("---\nname: demo\n---\n{long_line}\n"), || {
            "fallback".to_string()
        })
        .expect("the body should supply the description");
    assert_eq!(parsed.description.chars().count(), 100);
    assert!(parsed.description.ends_with("..."));
}

#[test]
fn frontmatter_flag_hides_a_skill_unless_its_policy_says_otherwise() {
    use crate::model::SkillPolicy;

    assert_eq!(
        ParsedSkillFrontmatter::policy_with_frontmatter(false, None),
        None
    );
    assert_eq!(
        ParsedSkillFrontmatter::policy_with_frontmatter(true, None),
        Some(SkillPolicy {
            allow_implicit_invocation: Some(false),
            products: Vec::new(),
        })
    );
    let explicit = SkillPolicy {
        allow_implicit_invocation: Some(true),
        products: Vec::new(),
    };
    assert_eq!(
        ParsedSkillFrontmatter::policy_with_frontmatter(true, Some(explicit.clone())),
        Some(explicit)
    );
}
