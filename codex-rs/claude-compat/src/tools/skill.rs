// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Skill`: the tool Claude Code's model invokes a skill with, by name and with arguments,
//! getting the skill's text expanded as for `/<name> <arguments>`.

/// The tool's description, Claude Code's, with the place of the skill list named as it is
/// here.
pub const DESCRIPTION: &str = r#"Execute a skill within the main conversation

When users ask you to perform tasks, check if any of the available skills match. Skills provide specialized capabilities and domain knowledge.

When users reference a "slash command" or "/<something>" (e.g., "/commit", "/review-pr"), they are referring to a skill. Use this tool to invoke it.

How to invoke:
- Use this tool with the skill name and optional arguments
- Examples:
  - `skill: "pdf"` - invoke the pdf skill
  - `skill: "commit", args: "-m 'Fix bug'"` - invoke with arguments
  - `skill: "review-pr", args: "123"` - invoke with arguments
  - `skill: "ms-office-suite:pdf"` - invoke using fully qualified name

Important:
- Available skills are listed in the skills section of your instructions
- When a skill matches the user's request, this is a BLOCKING REQUIREMENT: invoke the relevant Skill tool BEFORE generating any other response about the task
- NEVER mention a skill without actually calling this tool
- Do not invoke a skill that is already running
- Do not use this tool for built-in CLI commands (like /help, /clear, etc.)
- If the skill's instructions were already given to you in the current turn, the skill has ALREADY been loaded - follow the instructions directly instead of calling this tool again
"#;

pub const SKILL_PARAM_DESCRIPTION: &str =
    r#"The skill name. E.g., "commit", "review-pr", or "pdf""#;
pub const ARGS_PARAM_DESCRIPTION: &str = "Optional arguments for the skill";

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
    fn a_leading_slash_is_dropped() {
        assert_eq!(normalize_name(" /commit "), "commit");
        assert_eq!(normalize_name("demo:pdf"), "demo:pdf");
    }
}
