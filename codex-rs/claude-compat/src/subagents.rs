//! Claude Code subagents: `agents/**/*.md`, a frontmatter with `name` and `description`
//! and a body that is the subagent's system prompt.

use serde_yaml::Value;

use crate::markdown::parse_document;

/// The parts of a subagent definition Codex can use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subagent {
    pub name: String,
    pub description: String,
    /// The body, the subagent's instructions.
    pub prompt: String,
    /// `effort`, as written.
    pub effort: Option<String>,
}

/// Parses a subagent file. `None` when `name`, `description` or the body is missing, as
/// Claude Code then ignores the file too.
pub fn parse_subagent(contents: &str) -> Option<Subagent> {
    let document = parse_document(contents);
    let frontmatter: Value = serde_yaml::from_str(document.frontmatter?).ok()?;
    let field = |key: &str| {
        frontmatter
            .get(key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    };
    let prompt = document.body.trim();
    if prompt.is_empty() {
        return None;
    }
    Some(Subagent {
        name: field("name")?,
        description: field("description")?,
        prompt: prompt.to_string(),
        effort: field("effort"),
    })
}

#[cfg(test)]
mod tests {
    use super::Subagent;
    use super::parse_subagent;
    use pretty_assertions::assert_eq;

    #[test]
    fn subagent_needs_name_description_and_body() {
        assert_eq!(
            parse_subagent("---\nname: w\ndescription: does x\ntools: Read\neffort: high\n---\n\nDo x.\n"),
            Some(Subagent {
                name: "w".to_string(),
                description: "does x".to_string(),
                prompt: "Do x.".to_string(),
                effort: Some("high".to_string()),
            })
        );
        assert_eq!(parse_subagent("---\nname: w\n---\nDo x."), None);
        assert_eq!(parse_subagent("---\nname: w\ndescription: d\n---\n"), None);
    }
}
