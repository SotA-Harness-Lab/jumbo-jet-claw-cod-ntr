// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Agent` tool (called `Task` before): its description listing the agent
//! types, its result texts, and the tool lists (`tools`, `disallowedTools`) of subagent
//! definitions.

use serde_yaml::Value as YamlValue;

use super::AGENT;
use super::BASH;
use super::GLOB;
use super::READ;

/// The earlier name of `Agent`, still accepted in tool lists and permission rules.
pub const LEGACY_AGENT: &str = "Task";

/// Claude Code's agent type used when `subagent_type` is left out.
pub const GENERAL_PURPOSE: &str = "general-purpose";
/// Claude Code's description of its general-purpose agent.
pub const GENERAL_PURPOSE_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/agent/general_purpose_description.md");

pub const DESCRIPTION_PARAM_DESCRIPTION: &str = "A short (3-5 word) description of the task";
pub const PROMPT_PARAM_DESCRIPTION: &str = "The task for the agent to perform";
pub const SUBAGENT_TYPE_PARAM_DESCRIPTION: &str =
    "The type of specialized agent to use for this task";
pub const MODEL_PARAM_DESCRIPTION: &str =
    codex_prompt_texts::text!("claude-compat/tools/agent/model_param_description.md");
pub const RUN_IN_BACKGROUND_PARAM_DESCRIPTION: &str =
    "Set to true to run this agent in the background. You will be notified when it completes.";

/// Claude Code's text for a subagent that finished without a message.
pub const NO_OUTPUT_MESSAGE: &str = "(Subagent completed but returned no output.)";

/// An agent type the model may choose.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentType {
    pub name: String,
    pub description: String,
    pub tools: ToolList,
}

/// The tools a subagent definition gives its agent: `tools` (all when absent) without
/// `disallowedTools`, by Claude Code tool name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolList {
    /// `None` for every tool.
    pub allowed: Option<Vec<String>>,
    pub disallowed: Vec<String>,
}

impl ToolList {
    /// Reads `tools` and `disallowedTools` as Claude Code does: a comma- or space-separated
    /// string or a list; a missing `tools` or one holding `*` allows every tool, an empty
    /// one none; `Bash(git *)` stands for `Bash`, `Task` for `Agent`.
    pub fn from_frontmatter(tools: Option<&YamlValue>, disallowed: Option<&YamlValue>) -> Self {
        let allowed = match tools {
            None => None,
            Some(value) => {
                let names = tool_names(value);
                (!names.iter().any(|name| name == "*")).then_some(names)
            }
        };
        let disallowed = disallowed
            .map(tool_names)
            .unwrap_or_default()
            .into_iter()
            .filter(|name| name != "*")
            .collect();
        Self {
            allowed,
            disallowed,
        }
    }

    pub fn is_unrestricted(&self) -> bool {
        self.allowed.is_none() && self.disallowed.is_empty()
    }

    /// Whether the agent keeps the tool the model knows as `name`. A Codex tool that only
    /// serves a Claude tool (`write_stdin` for `Bash`) names that tool as `owner`, and is kept
    /// with it unless it is denied itself.
    pub fn permits(&self, name: &str, owner: Option<&str>) -> bool {
        if self
            .disallowed
            .iter()
            .any(|denied| entry_covers(denied, name, /*any_mcp*/ true))
        {
            return false;
        }
        // A helper tool goes with the tool it serves (`write_stdin` with `Bash`).
        if let Some(owner) = owner {
            return self.permits(owner, None);
        }
        self.allowed.as_ref().is_none_or(|allowed| {
            allowed
                .iter()
                .any(|allowed| entry_covers(allowed, name, /*any_mcp*/ false))
        })
    }

    /// The entries of `tools` when they give the agent none of the `available` tools (the
    /// names the model sees, each with the Claude tool it serves, as for [`Self::permits`]):
    /// Claude Code then refuses to start the agent. `None` when the agent gets a tool, when
    /// `tools` is absent, and when it is empty once `disallowedTools` is taken off it.
    pub fn unresolved_entries(&self, available: &[(String, Option<&str>)]) -> Option<Vec<String>> {
        let entries = self
            .allowed
            .as_ref()?
            .iter()
            .filter(|entry| !self.disallowed.contains(*entry))
            .cloned()
            .collect::<Vec<_>>();
        if entries.is_empty()
            || available
                .iter()
                .any(|(name, owner)| self.permits(name, *owner))
        {
            return None;
        }
        Some(entries)
    }

    /// Claude Code's summary of the list in the `Agent` description.
    pub fn summary(&self) -> String {
        match (&self.allowed, self.disallowed.is_empty()) {
            (Some(allowed), _) => {
                let effective = allowed
                    .iter()
                    .filter(|name| !self.disallowed.contains(*name))
                    .cloned()
                    .collect::<Vec<_>>();
                if effective.is_empty() {
                    "None".to_string()
                } else {
                    effective.join(", ")
                }
            }
            (None, false) => format!("All tools except {}", self.disallowed.join(", ")),
            (None, true) => "All tools".to_string(),
        }
    }
}

/// Whether an entry of a tool list covers the tool `name`: the tool itself, or, for an MCP
/// tool, the whole server as `mcp__<server>` or `mcp__<server>__*` (every tool whose name
/// starts with `mcp__<server>__`, as in Claude Code), and with `any_mcp` every MCP tool as
/// `mcp__*`, which Claude Code takes in `disallowedTools`.
fn entry_covers(entry: &str, name: &str, any_mcp: bool) -> bool {
    if entry == name {
        return true;
    }
    let (Some(entry), Some(tool)) = (entry.strip_prefix("mcp__"), name.strip_prefix("mcp__"))
    else {
        return false;
    };
    if entry == "*" {
        return any_mcp;
    }
    let server = match entry.strip_suffix("__*") {
        Some(server) => server,
        None if !entry.contains("__") => entry,
        None => return false,
    };
    !server.is_empty()
        && tool
            .strip_prefix(server)
            .is_some_and(|rest| rest.starts_with("__"))
}

/// The Claude tool a Codex tool kept beside the Claude tools works for, if any: it goes
/// with that tool in a subagent's tool list.
pub fn owner_of(codex_tool: &str) -> Option<&'static str> {
    match codex_tool {
        "write_stdin" => Some(BASH),
        "wait_agent" | "send_input" | "resume_agent" | "close_agent" | "send_message"
        | "followup_task" | "interrupt_agent" | "list_agents" => Some(AGENT),
        _ => None,
    }
}

/// The tool names in a `tools` value (Claude Code's `parseToolListFromCLI`): entries split
/// at commas and spaces outside parentheses, each reduced to its tool name.
fn tool_names(value: &YamlValue) -> Vec<String> {
    tool_list_entries(value)
        .iter()
        .map(|entry| rule_tool_name(entry))
        .collect()
}

/// The entries of a list of tools or rules (`tools`, a skill's `allowed-tools`) as Claude
/// Code's `parseToolListFromCLI` splits them: a string or a list of strings, split at
/// commas and spaces outside parentheses, so that `Bash(git add *)` stays one entry.
pub(crate) fn tool_list_entries(value: &YamlValue) -> Vec<String> {
    let entries: Vec<&str> = match value {
        YamlValue::String(text) => vec![text.as_str()],
        YamlValue::Sequence(items) => items.iter().filter_map(YamlValue::as_str).collect(),
        _ => Vec::new(),
    };
    let mut parts = Vec::new();
    for entry in entries {
        let mut current = String::new();
        let mut in_parens = false;
        for ch in entry.chars() {
            match ch {
                '(' => {
                    in_parens = true;
                    current.push(ch);
                }
                ')' => {
                    in_parens = false;
                    current.push(ch);
                }
                ',' | ' ' if !in_parens => {
                    if !current.trim().is_empty() {
                        parts.push(current.trim().to_string());
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.trim().is_empty() {
            parts.push(current.trim().to_string());
        }
    }
    parts
}

/// The tool a rule like `Bash(git *)` names, with the legacy `Task` read as `Agent`.
fn rule_tool_name(rule: &str) -> String {
    let name = rule.split_once('(').map_or(rule, |(name, _)| name).trim();
    if name == LEGACY_AGENT {
        AGENT.to_string()
    } else {
        name.to_string()
    }
}

/// The Codex agent role for a `subagent_type`: none (Codex's default agent) when it is left
/// out or is Claude Code's `general-purpose` and no role has that name.
pub fn role_for(subagent_type: Option<&str>, has_role: impl Fn(&str) -> bool) -> Option<&str> {
    let subagent_type = subagent_type
        .map(str::trim)
        .filter(|name| !name.is_empty())?;
    if subagent_type == GENERAL_PURPOSE && !has_role(GENERAL_PURPOSE) {
        return None;
    }
    Some(subagent_type)
}

/// Claude Code's refusal of an unknown agent type.
pub fn unknown_agent_type_message(subagent_type: &str, agent_types: &[AgentType]) -> String {
    let available = agent_types
        .iter()
        .map(|agent_type| agent_type.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!("Agent type '{subagent_type}' not found. Available agents: {available}")
}

/// Claude Code's refusal of an agent type whose `tools` gives it no tool, the entries
/// grouped as it groups them: an MCP entry or a Claude Code tool this session lacks
/// matched no tools in this session, any other entry is unrecognized.
pub fn zero_tools_message(agent_type: &str, entries: &[String]) -> String {
    let (absent, unrecognized): (Vec<&str>, Vec<&str>) = entries
        .iter()
        .map(String::as_str)
        .partition(|entry| entry.starts_with("mcp__") || CLAUDE_CODE_TOOLS.contains(entry));
    let mut groups = Vec::new();
    if !unrecognized.is_empty() {
        groups.push(format!("unrecognized [{}]", unrecognized.join(", ")));
    }
    if !absent.is_empty() {
        groups.push(format!(
            "matched no tools in this session [{}]",
            absent.join(", ")
        ));
    }
    format!(
        codex_prompt_texts::text!("claude-compat/tools/agent/zero_tools_message.md"),
        groups.join("; "),
        agent_type = agent_type
    )
}

/// Names of Claude Code's own tools (its tools reference), for telling an entry this
/// session lacks from a typo.
const CLAUDE_CODE_TOOLS: &[&str] = &[
    "Agent",
    "Artifact",
    "AskUserQuestion",
    "Bash",
    "CronCreate",
    "CronDelete",
    "CronList",
    "Edit",
    "EndConversation",
    "EnterPlanMode",
    "EnterWorktree",
    "ExitPlanMode",
    "ExitWorktree",
    "Glob",
    "Grep",
    "LSP",
    "ListAgents",
    "ListMcpResourcesTool",
    "ListPlugins",
    "ListSkills",
    "Monitor",
    "NotebookEdit",
    "PowerShell",
    "PushNotification",
    "Read",
    "ReadMcpResourceTool",
    "RemoteTrigger",
    "ReportFindings",
    "ScheduleWakeup",
    "SearchPlugins",
    "SearchSkills",
    "SendFeedback",
    "SendMessage",
    "SendUserFile",
    "ShareOnboardingGuide",
    "Skill",
    "SubagentHandback",
    "SuggestPluginInstall",
    "SuggestSkills",
    "TaskCreate",
    "TaskGet",
    "TaskList",
    "TaskOutput",
    "TaskStop",
    "TaskUpdate",
    "TodoWrite",
    "ToolSearch",
    "WaitForMcpServers",
    "WebFetch",
    "WebSearch",
    "Workflow",
    "Write",
];

/// A name for the agent's task in Codex's agent tree: the description in lower-case
/// snake_case, made unique by the end of the call id.
pub fn task_name(description: &str, call_id: &str) -> String {
    let mut name = String::new();
    for ch in description.chars().flat_map(char::to_lowercase) {
        if ch.is_ascii_lowercase() || ch.is_ascii_digit() {
            name.push(ch);
        } else if !name.is_empty() && !name.ends_with('_') {
            name.push('_');
        }
        if name.len() >= 40 {
            break;
        }
    }
    let name = name.trim_end_matches('_');
    let name = if name.is_empty() || name == "root" {
        "agent"
    } else {
        name
    };
    let suffix = call_id
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect::<Vec<_>>();
    let suffix = suffix[suffix.len().saturating_sub(6)..]
        .iter()
        .collect::<String>();
    if suffix.is_empty() {
        name.to_string()
    } else {
        format!("{name}_{suffix}")
    }
}

/// The text the model receives from a subagent that finished: its final message.
pub fn completed_text(final_message: Option<&str>) -> String {
    match final_message.map(str::trim) {
        Some(message) if !message.is_empty() => message.to_string(),
        _ => NO_OUTPUT_MESSAGE.to_string(),
    }
}

/// The text the model receives for an agent started in the background.
pub fn async_launched_text(agent_id: &str) -> String {
    format!(
        codex_prompt_texts::text!("claude-compat/tools/agent/async_launched.md"),
        agent_id = agent_id
    )
}

/// The tool description, after Claude Code's, listing the agent types.
pub fn description(agent_types: &[AgentType]) -> String {
    let agent_list = agent_types
        .iter()
        .map(|agent_type| {
            format!(
                "- {}: {} (Tools: {})",
                agent_type.name,
                agent_type.description,
                agent_type.tools.summary()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        codex_prompt_texts::text!("claude-compat/tools/agent/description.md"),
        AGENT = AGENT,
        agent_list = agent_list,
        GENERAL_PURPOSE = GENERAL_PURPOSE,
        READ = READ,
        GLOB = GLOB
    )
}

#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;
