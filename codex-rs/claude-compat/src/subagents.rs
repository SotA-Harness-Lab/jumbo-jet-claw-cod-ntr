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
    /// `model`, as written: a Claude model's alias or id, or `inherit`.
    pub model: Option<String>,
    /// `maxTurns`: how many model requests the agent may make; a value that is not a
    /// positive whole number is left out.
    pub max_turns: Option<u32>,
    /// `skills`: the skills to preload into the agent, as written.
    pub skills: Vec<String>,
    /// `permissionMode`, as written; `None` for a plugin's agent, which Claude Code does not
    /// let set it.
    pub permission_mode: Option<String>,
    /// The MCP servers `mcpServers` defines inline, by name, in the shape of a `.mcp.json`
    /// entry; the names it lists of servers already configured are left out, the agent
    /// having those anyway. Empty for a plugin's agent, which Claude Code does not let
    /// define servers.
    pub mcp_servers: Vec<(String, serde_json::Value)>,
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
            let namespace = plugin
                .agent_dirs()
                .iter()
                .find_map(|dir| {
                    let relative = path.strip_prefix(dir).ok()?;
                    Some(
                        relative
                            .parent()?
                            .components()
                            .map(|part| part.as_os_str().to_string_lossy().into_owned())
                            .collect::<Vec<_>>(),
                    )
                })
                // A file the manifest names on its own has no directories in between.
                .or_else(|| {
                    plugin
                        .agent_files()
                        .iter()
                        .any(|file| file == path)
                        .then(Vec::new)
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
        model: field("model"),
        max_turns: field("maxTurns")
            .and_then(|turns| turns.parse::<u32>().ok())
            .filter(|turns| *turns > 0),
        skills: frontmatter
            .get("skills")
            .map(skill_names)
            .unwrap_or_default(),
        permission_mode: field("permissionMode").filter(|_| plugin_agent.is_none()),
        mcp_servers: match (plugin_agent, frontmatter.get("mcpServers")) {
            (None, Some(servers)) => inline_mcp_servers(servers),
            _ => Vec::new(),
        },
        tools: ToolList::from_frontmatter(
            frontmatter.get("tools"),
            frontmatter.get("disallowedTools"),
        ),
    })
}

/// The servers an `mcpServers` value defines inline: each entry of its list that maps a
/// name to a server's settings (a name alone refers to a server already configured), or each
/// entry of a map.
fn inline_mcp_servers(value: &Value) -> Vec<(String, serde_json::Value)> {
    let maps: Vec<&serde_yaml::Mapping> = match value {
        Value::Sequence(items) => items.iter().filter_map(Value::as_mapping).collect(),
        Value::Mapping(map) => vec![map],
        _ => Vec::new(),
    };
    maps.into_iter()
        .flat_map(|map| map.iter())
        .filter_map(|(name, server)| {
            let name = name.as_str()?.trim();
            let server = serde_json::to_value(server).ok()?;
            (!name.is_empty() && server.is_object()).then(|| (name.to_string(), server))
        })
        .collect()
}

/// The names in a `skills` value: a list, or one string of names separated by commas or
/// spaces.
fn skill_names(value: &Value) -> Vec<String> {
    let entries: Vec<&str> = match value {
        Value::String(text) => vec![text.as_str()],
        Value::Sequence(items) => items.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    };
    entries
        .into_iter()
        .flat_map(|entry| entry.split(|ch: char| ch == ',' || ch.is_whitespace()))
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .collect()
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
                "---\nname: w\ndescription: does x\ntools: Read\neffort: high\nmodel: haiku\n---\n\nDo x.\n"
            ),
            Some(Subagent {
                name: "w".to_string(),
                description: "does x".to_string(),
                prompt: "Do x.".to_string(),
                effort: Some("high".to_string()),
                model: Some("haiku".to_string()),
                max_turns: None,
                skills: Vec::new(),
                permission_mode: None,
                mcp_servers: Vec::new(),
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
    fn skills_are_a_list_or_names_separated_by_commas() {
        let skills = |value: &str| {
            parse_subagent(&format!(
                "---\nname: w\ndescription: d\nskills: {value}\n---\nDo x.\n"
            ))
            .expect("subagent")
            .skills
        };
        let both = vec!["api-conventions".to_string(), "errors".to_string()];
        assert_eq!(skills("[api-conventions, errors]"), both);
        assert_eq!(skills("api-conventions, errors"), both);
        assert_eq!(skills("\"api-conventions errors\""), both);
        assert_eq!(skills("{}"), Vec::<String>::new());
    }

    #[test]
    fn inline_mcp_servers_are_kept_and_references_left_out() {
        let agent = parse_subagent(
            "---\nname: w\ndescription: d\nmcpServers:\n  - playwright:\n      type: stdio\n      command: npx\n      args: [\"-y\", \"@playwright/mcp\"]\n  - github\n---\nDo x.\n",
        )
        .expect("subagent");
        assert_eq!(
            agent.mcp_servers,
            vec![(
                "playwright".to_string(),
                serde_json::json!({
                    "type": "stdio",
                    "command": "npx",
                    "args": ["-y", "@playwright/mcp"],
                })
            )]
        );
    }

    #[test]
    fn the_permission_mode_is_kept_as_written() {
        assert_eq!(
            parse_subagent("---\nname: w\ndescription: d\npermissionMode: dontAsk\n---\nDo x.\n")
                .expect("subagent")
                .permission_mode,
            Some("dontAsk".to_string())
        );
    }

    #[test]
    fn max_turns_is_a_positive_whole_number() {
        let max_turns = |value: &str| {
            parse_subagent(&format!(
                "---\nname: w\ndescription: d\nmaxTurns: {value}\n---\nDo x.\n"
            ))
            .expect("subagent")
            .max_turns
        };
        assert_eq!(max_turns("5"), Some(5));
        assert_eq!(max_turns("\"12\""), Some(12));
        assert_eq!(max_turns("0"), None);
        assert_eq!(max_turns("-3"), None);
        assert_eq!(max_turns("2.5"), None);
        assert_eq!(max_turns("many"), None);
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
