//! Claude Code's expansion of an explicitly invoked skill.
//!
//! A skill invoked as `/<name> <arguments>` (the first text input starts that way) is given to
//! the model as Claude Code gives it: without frontmatter, headed `Base directory for this
//! skill: <dir>`, with `$ARGUMENTS`, `$ARGUMENTS[N]`, `$N`, named arguments,
//! `${CLAUDE_SKILL_DIR}` and `${CLAUDE_SESSION_ID}` replaced. A skill mentioned as `$<name>`
//! keeps its text, apart from `${CLAUDE_SKILL_DIR}` and `${CLAUDE_SESSION_ID}`. Nothing is
//! changed when `CCN_CLAUDE_COMPAT=0`.

use std::path::Path;

use codex_protocol::user_input::UserInput;

/// Expands `contents`, the `SKILL.md` of the skill `name` at `skill_md_path` on the host,
/// for the turn whose input is `user_input`. `session_id` is the thread's id.
pub(crate) fn expand_explicit_skill(
    contents: String,
    name: &str,
    skill_md_path: &str,
    session_id: &str,
    user_input: &[UserInput],
) -> String {
    if !codex_claude_compat::enabled() {
        return contents;
    }
    let skill_dir = Path::new(skill_md_path)
        .parent()
        .map(|dir| dir.to_string_lossy().into_owned())
        .unwrap_or_default();
    let arguments = user_input.iter().find_map(|input| match input {
        UserInput::Text { text, .. } => codex_claude_compat::slash_invocation_arguments(text, name),
        _ => None,
    });
    match arguments {
        Some(arguments) => {
            codex_claude_compat::render_invoked_skill(&contents, &skill_dir, session_id, arguments)
        }
        None => codex_claude_compat::substitute_skill_variables(&contents, &skill_dir, session_id),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn text(text: &str) -> UserInput {
        UserInput::Text {
            text: text.to_string(),
            text_elements: Vec::new(),
        }
    }

    #[test]
    fn slash_invocation_renders_like_claude_code() {
        if !codex_claude_compat::enabled() {
            return;
        }
        let expanded = expand_explicit_skill(
            "---\nname: demo\n---\nReview $ARGUMENTS in ${CLAUDE_SESSION_ID}.\n".to_string(),
            "demo",
            "/skills/demo/SKILL.md",
            "thread-1",
            &[text("/demo src/lib.rs")],
        );
        assert_eq!(
            expanded,
            "Base directory for this skill: /skills/demo\n\nReview src/lib.rs in thread-1.\n"
        );
    }

    #[test]
    fn mention_keeps_the_text_but_replaces_skill_variables() {
        if !codex_claude_compat::enabled() {
            return;
        }
        let contents = "---\nname: demo\n---\nRun ${CLAUDE_SKILL_DIR}/x.sh on $ARGUMENTS.\n";
        assert_eq!(
            expand_explicit_skill(
                contents.to_string(),
                "demo",
                "/skills/demo/SKILL.md",
                "thread-1",
                &[text("use $demo here")],
            ),
            "---\nname: demo\n---\nRun /skills/demo/x.sh on $ARGUMENTS.\n"
        );
    }
}
