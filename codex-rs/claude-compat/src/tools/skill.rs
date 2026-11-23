// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Skill`: the tool Claude Code's model invokes a skill with, by name and with arguments,
//! getting the skill's text expanded as for `/<name> <arguments>`.

/// The tool's description, Claude Code's, with the place of the skill list named as it is
/// here.
pub const DESCRIPTION: &str = codex_prompt_texts::text!("claude-compat/tools/skill/description.md");

pub const SKILL_PARAM_DESCRIPTION: &str =
    r#"The skill name. E.g., "commit", "review-pr", or "pdf""#;
pub const ARGS_PARAM_DESCRIPTION: &str = "Optional arguments for the skill";

/// How many of a subagent's `skills` are preloaded, as in Claude Code.
pub const MAX_PRELOADED_SKILLS: usize = 32;

/// The skill a subagent's `skills` entry `name` stands for among the names in `available`,
/// found as Claude Code finds it: the name itself, then the agent's plugin's skill of that
/// name (`<plugin>:<name>`, for an agent `agent_type` of `<plugin>:...`), then any skill
/// whose name ends in `:<name>`.
pub fn resolve_preloaded_skill<'a>(
    available: &[&'a str],
    agent_type: &str,
    name: &str,
) -> Option<&'a str> {
    let exact = |wanted: &str| available.iter().copied().find(|skill| *skill == wanted);
    exact(name)
        .or_else(|| {
            let (plugin, _) = agent_type.split_once(':')?;
            exact(&format!("{plugin}:{name}"))
        })
        .or_else(|| {
            let suffix = format!(":{name}");
            available
                .iter()
                .copied()
                .find(|skill| skill.ends_with(&suffix))
        })
}

/// The name the model gave, without the `/` it may have typed in front of it.
pub fn normalize_name(name: &str) -> &str {
    let name = name.trim();
    name.strip_prefix('/').unwrap_or(name)
}

pub fn unknown_skill_message(name: &str) -> String {
    format!("Unknown skill: {name}")
}

pub fn model_invocation_disabled_message(name: &str) -> String {
    format!("Skill {name} cannot be used with Skill tool due to disable-model-invocation")
}

/// The tool's result for a `context: fork` skill, which ran in a subagent.
pub fn forked_result_text(name: &str, result: &str) -> String {
    format!("Skill \"{name}\" completed (forked execution).\n\nResult:\n{result}")
}

/// The tool's result: Claude Code's line, then the expanded skill, which Claude Code adds to
/// the conversation as a message of its own.
pub fn result_text(name: &str, expanded: &str) -> String {
    format!("Launching skill: {name}\n\n{expanded}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a_preloaded_skill_is_found_by_name_then_by_plugin() {
        let available = ["review", "tools:lint", "docs:lint", "docs:format"];
        assert_eq!(
            resolve_preloaded_skill(&available, "reviewer", "review"),
            Some("review")
        );
        assert_eq!(
            resolve_preloaded_skill(&available, "docs:writer", "lint"),
            Some("docs:lint")
        );
        assert_eq!(
            resolve_preloaded_skill(&available, "writer", "lint"),
            Some("tools:lint")
        );
        assert_eq!(
            resolve_preloaded_skill(&available, "writer", "format"),
            Some("docs:format")
        );
        assert_eq!(
            resolve_preloaded_skill(&available, "writer", "deploy"),
            None
        );
    }

    #[test]
    fn a_leading_slash_is_dropped() {
        assert_eq!(normalize_name(" /commit "), "commit");
        assert_eq!(normalize_name("demo:pdf"), "demo:pdf");
    }

    #[test]
    fn a_forked_skill_reports_its_subagent_s_answer() {
        assert_eq!(
            forked_result_text("review", "All good."),
            "Skill \"review\" completed (forked execution).\n\nResult:\nAll good."
        );
    }
}
