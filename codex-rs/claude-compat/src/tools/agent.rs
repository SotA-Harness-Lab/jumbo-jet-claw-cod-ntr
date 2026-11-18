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
pub const GENERAL_PURPOSE_DESCRIPTION: &str = "General-purpose agent for researching complex questions, searching for code, and executing multi-step tasks. When you are searching for a keyword or file and are not confident that you will find the right match in the first few tries use this agent to perform the search for you.";

pub const DESCRIPTION_PARAM_DESCRIPTION: &str = "A short (3-5 word) description of the task";
pub const PROMPT_PARAM_DESCRIPTION: &str = "The task for the agent to perform";
pub const SUBAGENT_TYPE_PARAM_DESCRIPTION: &str =
    "The type of specialized agent to use for this task";
pub const MODEL_PARAM_DESCRIPTION: &str = "Optional model override for this agent, by the name of a model this session can use. Takes precedence over the agent definition's model. If omitted, uses the agent definition's model, or inherits from the parent.";
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
        if self.disallowed.iter().any(|denied| denied == name) {
            return false;
        }
        let listed = self
            .allowed
            .as_ref()
            .is_none_or(|allowed| allowed.iter().any(|allowed| allowed == name));
        listed || owner.is_some_and(|owner| self.permits(owner, None))
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
    let entries: Vec<&str> = match value {
        YamlValue::String(text) => vec![text.as_str()],
        YamlValue::Sequence(items) => items.iter().filter_map(YamlValue::as_str).collect(),
        _ => Vec::new(),
    };
    let mut names = Vec::new();
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
                        names.push(rule_tool_name(current.trim()));
                    }
                    current.clear();
                }
                _ => current.push(ch),
            }
        }
        if !current.trim().is_empty() {
            names.push(rule_tool_name(current.trim()));
        }
    }
    names
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
pub fn role_for<'a>(
    subagent_type: Option<&'a str>,
    has_role: impl Fn(&str) -> bool,
) -> Option<&'a str> {
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
        "Async agent launched successfully.\nagentId: {agent_id} (internal ID - do not mention to user.)\nThe agent is working in the background. Its result is added to the conversation when it completes; to wait for it, use wait_agent. Do not duplicate this agent's work — avoid working with the same files or topics it is using. Work on non-overlapping tasks, or briefly tell the user what you launched."
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
        r#"Launch a new agent to handle complex, multi-step tasks autonomously.

The {AGENT} tool launches specialized agents (subprocesses) that autonomously handle complex tasks. Each agent type has specific capabilities and tools available to it.

Available agent types and the tools they have access to:
{agent_list}

When using the {AGENT} tool, specify a subagent_type parameter to select which agent type to use. If omitted, the {GENERAL_PURPOSE} agent is used.

When NOT to use the {AGENT} tool:
- If you want to read a specific file path, use the {READ} tool or the {GLOB} tool instead of the {AGENT} tool, to find the match more quickly
- If you are searching for a specific class definition like "class Foo", use the {GLOB} tool instead, to find the match more quickly
- If you are searching for code within a specific file or set of 2-3 files, use the {READ} tool instead of the {AGENT} tool, to find the match more quickly
- Other tasks that are not related to the agent descriptions above


Usage notes:
- Always include a short description (3-5 words) summarizing what the agent will do
- Launch multiple agents concurrently whenever possible, to maximize performance; to do that, use a single message with multiple tool uses
- When the agent is done, it will return a single message back to you. The result returned by the agent is not visible to the user. To show the user the result, you should send a text message back to the user with a concise summary of the result.
- You can optionally run agents in the background using the run_in_background parameter. When an agent runs in the background, its result is added to the conversation when it completes — do NOT sleep, poll, or proactively check on its progress. Continue with other work or respond to the user instead.
- **Foreground vs background**: Use foreground (default) when you need the agent's results before you can proceed — e.g., research agents whose findings inform your next steps. Use background when you have genuinely independent work to do in parallel.
- Each {AGENT} invocation starts fresh — provide a complete task description.
- The agent's outputs should generally be trusted
- Clearly tell the agent whether you expect it to write code or just to do research (search, file reads, web fetches, etc.), since it is not aware of the user's intent
- If the agent description mentions that it should be used proactively, then you should try your best to use it without the user having to ask for it first. Use your judgement.
- If the user specifies that they want you to run agents "in parallel", you MUST send a single message with multiple {AGENT} tool use content blocks. For example, if you need to launch both a build-validator agent and a test-runner agent in parallel, send a single message with both tool calls.

## Writing the prompt

Brief the agent like a smart colleague who just walked into the room — it hasn't seen this conversation, doesn't know what you've tried, doesn't understand why this task matters.
- Explain what you're trying to accomplish and why, what you've already learned or ruled out, and enough context for the agent to make judgment calls.
- If you need a short response, say so ("report in under 200 words").
- Lookups: hand over the exact command. Investigations: hand over the question — prescribed steps become dead weight when the premise is wrong.

Terse command-style prompts produce shallow, generic work.

**Never delegate understanding.** Don't write "based on your findings, fix the bug" or "based on the research, implement it." Write prompts that prove you understood: include file paths, line numbers, what specifically to change.
"#
    )
}

#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;
