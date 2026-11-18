//! The tools of an agent spawned from a Claude Code subagent definition. With Claude Code's
//! tools offered, the agent keeps only the tools its `tools` frontmatter lists and its
//! `disallowedTools` does not, by the names the model sees (Claude Code's names for the
//! Claude tools, `mcp__<server>__<tool>` for MCP tools). Codex tools that only serve a
//! Claude tool (`write_stdin` for `Bash`, the other agent tools for `Agent`) go with it.

use codex_claude_compat::tools::agent::ToolList;
use codex_claude_compat::tools::agent::owner_of;

use crate::config::AgentRoleConfig;
use crate::session::turn_context::TurnContext;
use crate::tools::registry::ToolRegistry;

/// Removes from the registry the tools the turn's Claude subagent definition does not give
/// its agent. Nothing changes without Claude Code's tools, or for an agent of another role.
pub(crate) fn restrict_to_subagent_tools(turn_context: &TurnContext, registry: &mut ToolRegistry) {
    if !super::enabled(turn_context) {
        return;
    }
    let Some(role_name) = turn_context.session_source.get_agent_role() else {
        return;
    };
    let Some(role) = crate::agent::role::resolve_role_config(&turn_context.config, &role_name)
    else {
        return;
    };
    if let Some(tools) = role_tool_list(role).filter(|tools| !tools.is_unrestricted()) {
        restrict(registry, &tools);
    }
}

/// The tool list of a role read from a Claude Code subagent file (`agents/*.md`); `None`
/// for any other role.
pub(super) fn role_tool_list(role: &AgentRoleConfig) -> Option<ToolList> {
    let path = role.config_file.as_ref()?;
    if path.extension().is_none_or(|extension| extension != "md") {
        return None;
    }
    let contents = std::fs::read_to_string(path)
        .inspect_err(|err| {
            tracing::warn!("cannot read the subagent file {}: {err}", path.display());
        })
        .ok()?;
    Some(codex_claude_compat::parse_subagent(&contents)?.tools)
}

/// Keeps in the registry only the tools `tools` permits.
pub(super) fn restrict(registry: &mut ToolRegistry, tools: &ToolList) {
    let removed = registry
        .entries()
        .map(|tool| tool.runtime.tool_name())
        .filter(|name| {
            let model_name = name.to_string();
            let owner = if model_name.starts_with("mcp__") {
                None
            } else {
                owner_of(&name.name)
            };
            !tools.permits(&model_name, owner)
        })
        .collect::<Vec<_>>();
    for name in removed {
        registry.remove(&name);
    }
}

#[cfg(test)]
#[path = "subagent_tools_tests.rs"]
mod tests;
