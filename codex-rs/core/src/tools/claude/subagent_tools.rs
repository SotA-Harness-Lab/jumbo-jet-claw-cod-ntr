// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The tools of an agent spawned from a Claude Code subagent definition. With Claude Code's
//! tools offered, the agent keeps only the tools its `tools` frontmatter lists and its
//! `disallowedTools` does not, by the names the model sees (Claude Code's names for the
//! Claude tools, `mcp__<server>__<tool>` for MCP tools). Codex tools that only serve a
//! Claude tool (`write_stdin` for `Bash`, the other agent tools for `Agent`) go with it.

use codex_claude_compat::Subagent;
use codex_claude_compat::tools::agent::ToolList;
use codex_claude_compat::tools::agent::owner_of;
use codex_protocol::protocol::AskForApproval;
use codex_tools::ToolName;

use crate::config::AgentRoleConfig;
use crate::config::Config;
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
    Some(role_subagent(role)?.tools)
}

/// The Claude Code subagent definition a role was read from (`agents/*.md`), read again from
/// its file; `None` for any other role.
pub(crate) fn role_subagent(role: &AgentRoleConfig) -> Option<Subagent> {
    let path = role.config_file.as_ref()?;
    if path.extension().is_none_or(|extension| extension != "md") {
        return None;
    }
    let contents = std::fs::read_to_string(path)
        .inspect_err(|err| {
            tracing::warn!("cannot read the subagent file {}: {err}", path.display());
        })
        .ok()?;
    codex_claude_compat::parse_subagent_at(&contents, path)
}

/// `maxTurns` of the Claude Code subagent definition the turn's agent was started from: how
/// many model requests a turn of it may make. `None` for any other agent.
pub(crate) fn subagent_max_turns(turn_context: &TurnContext) -> Option<u32> {
    let role_name = turn_context.session_source.get_agent_role()?;
    let role = crate::agent::role::resolve_role_config(&turn_context.config, &role_name)?;
    role_subagent(role)?.max_turns
}

/// Applies the `permissionMode` of the Claude Code subagent definition of `role_name` to the
/// configuration of an agent about to start, where Codex has a counterpart that asks no
/// more than the parent does: `dontAsk`, which denies what would be asked, runs the agent
/// with the approval policy `never`. `default` (and `manual`) keeps the parent's, as Claude
/// Code does under a parent in default mode; `acceptEdits`, `plan`, `auto` and
/// `bypassPermissions` have no counterpart and keep the parent's too.
pub(crate) fn apply_subagent_permission_mode(config: &mut Config, role_name: Option<&str>) {
    let Some(subagent) = role_name
        .and_then(|role_name| crate::agent::role::resolve_role_config(config, role_name))
        .and_then(role_subagent)
    else {
        return;
    };
    if subagent.permission_mode.as_deref() == Some("dontAsk")
        && let Err(err) = config
            .permissions
            .approval_policy
            .set(AskForApproval::Never)
    {
        tracing::warn!(
            "agent {}: permissionMode dontAsk is not allowed here: {err}",
            subagent.name
        );
    }
}

/// Keeps in the registry only the tools `tools` permits.
pub(super) fn restrict(registry: &mut ToolRegistry, tools: &ToolList) {
    let removed = registry
        .entries()
        .map(|tool| tool.runtime.tool_name())
        .filter(|name| {
            let (model_name, owner) = list_entry(name);
            !tools.permits(&model_name, owner)
        })
        .collect::<Vec<_>>();
    for name in removed {
        registry.remove(&name);
    }
}

/// A tool as a subagent's tool list sees it: the name the model sees, and the Claude tool it
/// serves, if any.
pub(super) fn list_entry(name: &ToolName) -> (String, Option<&'static str>) {
    let model_name = name.to_string();
    let owner = if model_name.starts_with("mcp__") {
        None
    } else {
        owner_of(&name.name)
    };
    (model_name, owner)
}

#[cfg(test)]
#[path = "subagent_tools_tests.rs"]
mod tests;
