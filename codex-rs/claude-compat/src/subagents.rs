// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code subagents: `agents/**/*.md`, a frontmatter with `name` and `description`
//! and a body that is the subagent's system prompt.
//!
//! A plugin's agents are the same files under the plugin, with Claude Code's differences:
//! the agent is known as `<plugin>:<subdirectories>:<name>`, the name defaults to the file's
//! and the description to a stock one, and the body may refer to the plugin's variables.

use std::path::Path;

use serde_yaml::Value;

use crate::markdown::frontmatter_string;
use crate::markdown::parse_document;
use crate::markdown::parse_frontmatter;
use crate::plugin_vars::PluginVariables;
use crate::plugin_vars::plugin_root_of;
use crate::tools::agent::ToolList;

/// The parts of a subagent definition Codex can use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Subagent {
    pub name: String,
    pub description: String,
    /// The body, the subagent's instructions.
    pub prompt: String,
    /// `effort`, as written.
    pub effort: Option<String>,
    /// `tools` and `disallowedTools`, applied when Codex offers Claude Code's tools.
    pub tools: ToolList,
}

/// Parses a subagent file. `None` when `name`, `description` or the body is missing, as
/// Claude Code then ignores the file too.
pub fn parse_subagent(contents: &str) -> Option<Subagent> {
    parse_with(contents, /*plugin_agent*/ None)
}

/// Parses the subagent file at `path`: by the rules for a plugin's agents when the file is
/// in one of the agent directories of a Claude Code plugin, as [`parse_subagent`] otherwise.
pub fn parse_subagent_at(contents: &str, path: &Path) -> Option<Subagent> {
    let plugin_agent = plugin_root_of(path)
        .and_then(PluginVariables::load)
        .and_then(|plugin| {
            let namespace = plugin.agent_dirs().iter().find_map(|dir| {
                let relative = path.strip_prefix(dir).ok()?;
                Some(
                    relative
                        .parent()?
                        .components()
                        .map(|part| part.as_os_str().to_string_lossy().into_owned())
                        .collect::<Vec<_>>(),
                )
            })?;
            let file_name = path.file_stem()?.to_string_lossy().into_owned();
            Some(PluginAgent {
                plugin,
                namespace,
                file_name,
            })
        });
    parse_with(contents, plugin_agent.as_ref())
}

/// Where a plugin's agent file stands.
struct PluginAgent {
    plugin: PluginVariables,
    /// The directories between the plugin's agent directory and the file.
    namespace: Vec<String>,
    /// The file's name without its extension.
    file_name: String,
}

fn parse_with(contents: &str, plugin_agent: Option<&PluginAgent>) -> Option<Subagent> {
    let document = parse_document(contents);
    let frontmatter: Value = match document.frontmatter {
        // As in Claude Code, a frontmatter that cannot be read leaves every field unset.
        Some(frontmatter) => parse_frontmatter(frontmatter).unwrap_or(Value::Null),
        // Only a plugin's agent may go without a frontmatter.
        None => {
            plugin_agent?;
            Value::Null
        }
    };
    let field = |key: &str| {
        frontmatter
            .get(key)
            .and_then(frontmatter_string)
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    };
    let prompt = document.body.trim();
    if prompt.is_empty() {
        return None;
    }
    let (name, description, prompt) = match plugin_agent {
        Some(agent) => {
            let base = field("name").unwrap_or_else(|| agent.file_name.clone());
            let name = std::iter::once(agent.plugin.name.clone())
                .chain(agent.namespace.iter().cloned())
                .chain(std::iter::once(base))
                .collect::<Vec<_>>()
                .join(":");
            let description = field("description")
                .or_else(|| field("when-to-use"))
                .unwrap_or_else(|| format!("Agent from {} plugin", agent.plugin.name));
            (
                name,
                description,
                agent.plugin.substitute_in_content(prompt),
            )
        }
        None => (field("name")?, field("description")?, prompt.to_string()),
    };
    Some(Subagent {
        name,
        description,
        prompt,
        effort: field("effort"),
        tools: ToolList::from_frontmatter(
            frontmatter.get("tools"),
            frontmatter.get("disallowedTools"),
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::Subagent;
    use super::parse_subagent;
    use crate::tools::agent::ToolList;
    use pretty_assertions::assert_eq;

    #[test]
    fn subagent_needs_name_description_and_body() {
        assert_eq!(
            parse_subagent(
                "---\nname: w\ndescription: does x\ntools: Read\neffort: high\n---\n\nDo x.\n"
            ),
            Some(Subagent {
                name: "w".to_string(),
                description: "does x".to_string(),
                prompt: "Do x.".to_string(),
                effort: Some("high".to_string()),
                tools: ToolList {
                    allowed: Some(vec!["Read".to_string()]),
                    disallowed: Vec::new(),
                },
            })
        );
        assert_eq!(parse_subagent("---\nname: w\n---\nDo x."), None);
        assert_eq!(parse_subagent("---\nname: w\ndescription: d\n---\n"), None);
    }

    #[test]
    fn reads_frontmatter_that_is_not_valid_yaml_as_claude_code_does() {
        let agent = parse_subagent(
            "---\nname: reviewer\ndescription: Use this agent when: the user asks for a review\n---\nReview.\n",
        )
        .expect("the description is quoted and the frontmatter read again");
        assert_eq!(
            (agent.name.as_str(), agent.description.as_str()),
            (
                "reviewer",
                "Use this agent when: the user asks for a review"
            )
        );

        let agent = parse_subagent("---\nname: 3000\ndescription: true\n---\nDo x.\n")
            .expect("a bare number or boolean is taken as text");
        assert_eq!(
            (agent.name.as_str(), agent.description.as_str()),
            ("3000", "true")
        );
        assert_eq!(
            parse_subagent("---\nname: w\ndescription: d\n- stray\n---\nDo x.\n"),
            None
        );
    }
}
