//! Claude Code's tool set, offered to the model in place of Codex's own tools when the
//! `claude_tools` feature is on (see `docs/claude-compat.md`, "Claude tool set").
//!
//! The Claude tools are a translation layer, not a second implementation: each one takes
//! Claude Code's parameters, turns them into a call of the Codex tool runtime it stands in
//! for (`exec_command`, `apply_patch`, `view_image`, the executor filesystem) and turns the
//! result into Claude Code's text. Approval, sandboxing, execution policy, events, the TUI
//! and diff tracking stay Codex's. Only what the Codex tool cannot do is added here.
//!
//! Each tool lives in its own file with an `install` (or `register`) function; the pure
//! parts (descriptions, formatting, edits) are in `codex_claude_compat::tools`. A new tool
//! is a new file plus one line in [`replace_codex_tools`].

mod agent;
mod ask_user_question;
mod bash;
mod edit;
mod file_state;
mod files;
mod output;
mod read;
mod subagent_tools;
mod todo_write;
mod write;

use std::collections::HashMap;
use std::sync::Arc;

use codex_features::Feature;
use codex_tools::ToolName;
use serde::Deserialize;

use crate::function_tool::FunctionCallError;
use crate::session::turn_context::TurnContext;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolRegistry;

pub(crate) use bash::register as register_bash;
pub(crate) use subagent_tools::restrict_to_subagent_tools;

/// Whether the model is offered Claude Code's tools instead of Codex's: the `claude_tools`
/// feature is on and `CODEX_CLAUDE_COMPAT` is not `0`.
pub(crate) fn enabled(turn_context: &TurnContext) -> bool {
    turn_context.config.features.enabled(Feature::ClaudeTools) && codex_claude_compat::enabled()
}

/// Names of the Claude Code tools offered next to each other, for descriptions that refer
/// to one another. A tool is listed once its file installs it.
pub(crate) const OFFERED_TOOLS: &[&str] = &[
    codex_claude_compat::tools::BASH,
    codex_claude_compat::tools::READ,
    codex_claude_compat::tools::WRITE,
    codex_claude_compat::tools::EDIT,
    codex_claude_compat::tools::TODO_WRITE,
    codex_claude_compat::tools::ASK_USER_QUESTION,
    codex_claude_compat::tools::AGENT,
];

/// Replaces Codex tools by Claude Code's in a registry that holds Codex's core tools.
/// `Bash` is registered by `add_shell_tools` itself, which owns the shell options.
pub(crate) fn replace_codex_tools(turn_context: &TurnContext, registry: &mut ToolRegistry) {
    if !enabled(turn_context) {
        return;
    }
    let mut swap = ToolSwap::new(registry);
    read::install(&mut swap);
    write::install(&mut swap);
    edit::install(&mut swap);
    todo_write::install(&mut swap);
    ask_user_question::install(&mut swap);
    agent::install(&mut swap, turn_context);
    // Next: glob, grep, web_fetch, web_search, todo_write, ask_user_question, agent; each
    // in its own file, taking the Codex tool it replaces through `swap.take`.
}

/// Takes Codex tools out of what the model is offered, keeping their runtimes for the
/// Claude tools that delegate to them.
pub(crate) struct ToolSwap<'a> {
    registry: &'a mut ToolRegistry,
    taken: HashMap<&'static str, Option<Arc<dyn CoreToolRuntime>>>,
}

impl<'a> ToolSwap<'a> {
    fn new(registry: &'a mut ToolRegistry) -> Self {
        Self {
            registry,
            taken: HashMap::new(),
        }
    }

    /// Removes the Codex tool `name` from the registry and returns its runtime, or `None`
    /// when this turn does not have it. Taking a tool again returns the same runtime, so
    /// that several Claude tools can delegate to one Codex tool.
    pub(crate) fn take(&mut self, name: &'static str) -> Option<Arc<dyn CoreToolRuntime>> {
        let registry = &mut *self.registry;
        self.taken
            .entry(name)
            .or_insert_with(|| registry.remove(&ToolName::plain(name)))
            .clone()
    }

    /// Removes a Codex tool given with its namespace (the multi-agent tools) and returns its
    /// runtime, or `None` when this turn does not have it.
    pub(crate) fn take_named(&mut self, name: ToolName) -> Option<Arc<dyn CoreToolRuntime>> {
        self.registry.remove(&name)
    }

    /// Whether the session's tool policy lets the model use the Codex tool `name`: a Claude
    /// tool that builds its own runtime for a missing Codex tool checks this first, so that
    /// a policy that removes a Codex tool removes its Claude counterpart too.
    pub(crate) fn allows(&self, name: &str) -> bool {
        self.registry.tool_policy.allows(&ToolName::plain(name))
    }

    pub(crate) fn add(&mut self, runtime: impl CoreToolRuntime + 'static) {
        self.registry.add(runtime);
    }
}

/// Parses a Claude tool's arguments; Claude Code reports an invalid input wrapped as a
/// tool use error.
pub(crate) fn parse_claude_arguments<T>(
    tool: &str,
    invocation: &ToolInvocation,
) -> Result<T, FunctionCallError>
where
    T: for<'de> Deserialize<'de>,
{
    let ToolPayload::Function { arguments } = &invocation.payload else {
        return Err(FunctionCallError::RespondToModel(format!(
            "{tool} received an unsupported payload"
        )));
    };
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(codex_claude_compat::tools::tool_use_error(&format!(
            "InputValidationError: {tool} failed due to the following issue: {err}"
        )))
    })
}

/// A refusal Claude Code makes before running the tool.
pub(crate) fn rejected(message: &str) -> FunctionCallError {
    FunctionCallError::RespondToModel(codex_claude_compat::tools::tool_use_error(message))
}

/// A failure while the tool ran, reported as Claude Code reports a thrown error.
pub(crate) fn failed(message: impl Into<String>) -> FunctionCallError {
    FunctionCallError::RespondToModel(message.into())
}

/// The same invocation with another payload, for the Codex runtime a Claude tool delegates
/// to. Everything else (session, turn, cancellation, diff tracker, call id) is kept, so the
/// Codex runtime reports its events under the model's call.
pub(crate) fn delegated(invocation: &ToolInvocation, payload: ToolPayload) -> ToolInvocation {
    ToolInvocation {
        payload,
        ..invocation.clone()
    }
}

#[cfg(test)]
#[path = "claude_tests.rs"]
mod tests;
