//! Hook-facing tool names and matcher compatibility aliases.
//!
//! Hook stdin exposes one canonical `tool_name`, but matcher selection may also
//! need to recognize names from adjacent tool ecosystems. Keeping those two
//! concepts together prevents handlers from accidentally serializing a
//! compatibility alias, such as `Write`, as the stable hook payload name.

/// Identifies a tool in hook payloads and hook matcher selection.
///
/// `name` is the canonical value serialized into hook stdin. Matcher aliases are
/// internal-only compatibility names that may select the same hook handlers but
/// must not change the payload seen by hook processes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HookToolName {
    name: String,
    matcher_aliases: Vec<String>,
}

impl HookToolName {
    /// Builds a hook tool name with no matcher aliases.
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            matcher_aliases: Vec::new(),
        }
    }

    /// Returns the hook identity for file edits performed through `apply_patch`.
    ///
    /// The serialized name remains `apply_patch` so logs and policies can key
    /// off the actual Codex tool. `Write` and `Edit` are accepted as matcher
    /// aliases for compatibility with hook configurations that describe edits
    /// using Claude Code-style names.
    pub(crate) fn apply_patch() -> Self {
        Self {
            name: "apply_patch".to_string(),
            matcher_aliases: vec!["Write".to_string(), "Edit".to_string()],
        }
    }

    /// Returns the hook identity for spawning sub-agents.
    ///
    /// The serialized name remains `spawn_agent`, while `Agent` (and `Task`, its
    /// former name) is accepted as a matcher alias for compatibility with hook
    /// configurations that describe sub-agent creation using Claude Code-style names.
    pub(crate) fn spawn_agent() -> Self {
        Self {
            name: "spawn_agent".to_string(),
            matcher_aliases: vec!["Agent".to_string(), "Task".to_string()],
        }
    }

    /// Returns the hook identity of a function tool, with the Claude Code name of its
    /// counterpart, if it has one, as a matcher alias: `request_user_input` is matched by
    /// `AskUserQuestion` (both take `questions[].question`) and `update_plan` by `TodoWrite`.
    pub(crate) fn function(name: impl Into<String>) -> Self {
        let name = name.into();
        let matcher_aliases = match name.as_str() {
            "request_user_input" => vec!["AskUserQuestion".to_string()],
            "update_plan" => vec!["TodoWrite".to_string()],
            _ => Vec::new(),
        };
        Self {
            name,
            matcher_aliases,
        }
    }

    /// Returns the hook identity historically used for shell-like tools.
    pub(crate) fn bash() -> Self {
        Self::new("Bash")
    }

    /// Returns the canonical hook name serialized into hook stdin.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Returns additional matcher inputs that should select the same handlers.
    pub(crate) fn matcher_aliases(&self) -> &[String] {
        &self.matcher_aliases
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::HookToolName;

    #[test]
    fn function_tools_with_a_claude_code_counterpart_get_its_name_as_an_alias() {
        let question = HookToolName::function("request_user_input");
        assert_eq!(question.name(), "request_user_input");
        assert_eq!(
            question.matcher_aliases().to_vec(),
            vec!["AskUserQuestion".to_string()]
        );
        assert_eq!(
            HookToolName::function("update_plan")
                .matcher_aliases()
                .to_vec(),
            vec!["TodoWrite".to_string()]
        );
        assert_eq!(
            HookToolName::function("view_image")
                .matcher_aliases()
                .to_vec(),
            Vec::<String>::new()
        );
        assert_eq!(
            HookToolName::spawn_agent().matcher_aliases().to_vec(),
            vec!["Agent".to_string(), "Task".to_string()]
        );
    }
}
