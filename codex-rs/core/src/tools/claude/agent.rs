// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Agent`: Claude Code's subagent tool (called `Task` before).
//!
//! Delegated: starting the agent, to Codex's `spawn_agent` runtime of the turn's multi-agent
//! version (roles, models, depth and thread limits, sandbox inheritance, TUI events), and
//! waiting for it, through Codex's agent status subscription that `wait_agent` uses; a
//! background agent's result reaches the conversation through Codex's completion notice.
//! Added: Claude Code's parameters, its description listing the agent types, the mapping of
//! `subagent_type` to a role, and its result texts. The other agent tools stay Codex's.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::Duration;

use codex_claude_compat::HooksModConfig;
use codex_claude_compat::tools::AGENT;
use codex_claude_compat::tools::agent;
use codex_claude_compat::tools::agent::AgentType;
use codex_claude_compat::tools::agent::ToolList;
use codex_protocol::ThreadId;
use codex_protocol::protocol::AgentStatus;
use codex_protocol::protocol::MultiAgentVersion;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use futures::StreamExt;
use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use super::PermissionInput;
use super::ToolSwap;
use super::ask_user_question::output_json;
use super::check_permission;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::skill::preloaded_skills;
use super::subagent_tools::list_entry;
use super::subagent_tools::role_subagent;
use super::subagent_tools::role_tool_list;
use crate::agent::LocalAgentControl;
use crate::agent::agent_resolver::resolve_agent_target;
use crate::agent::role::DEFAULT_ROLE_NAME;
use crate::agent::status::is_final;
use crate::config::Config;
use crate::function_tool::FunctionCallError;
use crate::session::turn_context::TurnContext;
use crate::tools::context::ToolCallSource;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::multi_agents_spec::MULTI_AGENT_V1_NAMESPACE;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `Agent` in place of `spawn_agent`, when the turn has the multi-agent tools.
pub(super) fn install(swap: &mut ToolSwap<'_>, turn_context: &TurnContext) {
    let version = turn_context.multi_agent_version;
    let spawn_agent = match version {
        MultiAgentVersion::V1 => ToolName::namespaced(MULTI_AGENT_V1_NAMESPACE, "spawn_agent"),
        MultiAgentVersion::V2 => ToolName::new(
            turn_context.config.multi_agent_v2.tool_namespace.clone(),
            "spawn_agent",
        ),
        MultiAgentVersion::Disabled => return,
    };
    let Some(spawn_agent) = swap.take_named(spawn_agent) else {
        return;
    };
    let (agent_types, roles) = agent_types(&turn_context.config);
    swap.add(AgentHandler {
        spawn_agent,
        version,
        agent_types,
        roles,
    });
}

/// The agent types offered to the model, after Claude Code's list, and the names of every
/// role. Codex's default role is offered as Claude Code's `general-purpose`, unless a role
/// has that name.
fn agent_types(config: &Config) -> (Vec<AgentType>, BTreeSet<String>) {
    let mut roles = crate::agent::role::built_in_configs()
        .iter()
        .chain(config.agent_roles.iter())
        .map(|(name, role)| (name.as_str(), role))
        .collect::<BTreeMap<_, _>>();
    let names = roles.keys().map(ToString::to_string).collect();
    let mut agent_types = Vec::new();
    if !roles.contains_key(agent::GENERAL_PURPOSE) {
        roles.remove(DEFAULT_ROLE_NAME);
        agent_types.push(AgentType {
            name: agent::GENERAL_PURPOSE.to_string(),
            description: agent::GENERAL_PURPOSE_DESCRIPTION.to_string(),
            tools: ToolList::default(),
        });
    }
    agent_types.extend(roles.into_iter().map(|(name, role)| AgentType {
        name: name.to_string(),
        description: role.description.clone().unwrap_or_default(),
        tools: role_tool_list(role).unwrap_or_default(),
    }));
    (agent_types, names)
}

struct AgentHandler {
    spawn_agent: Arc<dyn CoreToolRuntime>,
    version: MultiAgentVersion,
    agent_types: Vec<AgentType>,
    roles: BTreeSet<String>,
}

/// Claude Code's input; its team, isolation and permission-mode fields are not offered and
/// are ignored when sent.
#[derive(Deserialize)]
struct AgentArgs {
    description: String,
    prompt: String,
    subagent_type: Option<String>,
    model: Option<String>,
    #[serde(default)]
    run_in_background: bool,
    /// Not offered to the model: the effort level of a `context: fork` skill run through
    /// this tool, as Codex's reasoning effort.
    #[serde(default)]
    effort: Option<String>,
}

impl ToolExecutor<ToolInvocation> for AgentHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(AGENT)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "description".to_string(),
                JsonSchema::string(Some(agent::DESCRIPTION_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "prompt".to_string(),
                JsonSchema::string(Some(agent::PROMPT_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "subagent_type".to_string(),
                JsonSchema::string(Some(agent::SUBAGENT_TYPE_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "model".to_string(),
                JsonSchema::string(Some(agent::MODEL_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "run_in_background".to_string(),
                JsonSchema::boolean(Some(agent::RUN_IN_BACKGROUND_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: AGENT.to_string(),
            description: agent::description(&self.agent_types),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["description".to_string(), "prompt".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for AgentHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl AgentHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: AgentArgs = parse_claude_arguments(AGENT, &invocation)?;
        let _permission = check_permission(
            &invocation,
            AGENT,
            PermissionInput::Content(args.subagent_type.as_deref().unwrap_or("general-purpose")),
        )
        .await?;
        let role = agent::role_for(args.subagent_type.as_deref(), |name| {
            self.roles.contains(name)
        });
        if let Some(role) = role
            && !self.roles.contains(role)
        {
            return Err(failed(agent::unknown_agent_type_message(
                role,
                &self.agent_types,
            )));
        }
        // As Claude Code does, an agent whose `tools` names none of the tools there are is
        // not started.
        if let Some(role) = role
            && let Some(agent_type) = self
                .agent_types
                .iter()
                .find(|agent_type| agent_type.name == role)
        {
            let available = invocation
                .step_context
                .tool_router
                .registered_tool_names()
                .map(|name| list_entry(&name))
                .collect::<Vec<_>>();
            if let Some(entries) = agent_type.tools.unresolved_entries(&available) {
                return Err(failed(agent::zero_tools_message(role, &entries)));
            }
        }

        let control = invocation
            .session
            .services
            .local_agent_runtime
            .control(invocation.session.session_id());
        let (agent_id, thread_id) = self.spawn(&invocation, &args, role).await?;
        if args.run_in_background {
            if self.version == MultiAgentVersion::V1 {
                close_when_finished(control, thread_id);
            }
            return Ok(boxed_tool_output(ClaudeToolOutput::new(
                agent::async_launched_text(&agent_id),
                json!({
                    "status": "async_launched",
                    "agentId": agent_id,
                    "description": args.description,
                    "prompt": args.prompt,
                }),
            )));
        }

        // A finished V1 agent keeps its thread slot until it is closed; Claude Code's
        // subagent ends with its answer, or when the call is interrupted.
        let close = (self.version == MultiAgentVersion::V1).then(|| CloseOnDrop {
            control: Some(control.clone()),
            thread_id,
        });
        let status = final_status(&control, thread_id).await;
        if let Some(close) = close {
            close.close().await;
        }
        match status {
            AgentStatus::Completed(message) => {
                let text = agent::completed_text(message.as_deref());
                Ok(boxed_tool_output(ClaudeToolOutput::new(
                    text.clone(),
                    json!({
                        "status": "completed",
                        "agentId": agent_id,
                        "content": [{ "type": "text", "text": text }],
                        "prompt": args.prompt,
                    }),
                )))
            }
            AgentStatus::Errored(error) => Err(failed(error)),
            other => Err(failed(format!(
                "Agent {agent_id} ended without a result ({other:?})"
            ))),
        }
    }

    /// Starts the agent through Codex's `spawn_agent`; returns the id the model may use for
    /// it and its thread.
    async fn spawn(
        &self,
        invocation: &ToolInvocation,
        args: &AgentArgs,
        role: Option<&str>,
    ) -> Result<(String, ThreadId), FunctionCallError> {
        let mut arguments = Map::new();
        let subagent = role
            .and_then(|role| crate::agent::role::resolve_role_config(&invocation.turn.config, role))
            .and_then(role_subagent);
        // As in Claude Code, the skills the subagent names come with the prompt, in full.
        let preloaded = match (role, &subagent) {
            (Some(role), Some(subagent)) => {
                preloaded_skills(invocation, role, &subagent.skills).await
            }
            _ => Vec::new(),
        };
        let message = std::iter::once(args.prompt.clone())
            .chain(preloaded)
            .collect::<Vec<_>>()
            .join("\n\n");
        arguments.insert("message".to_string(), json!(message));
        if let Some(role) = role {
            arguments.insert("agent_type".to_string(), json!(role));
        }
        if let Some(model) = spawn_model(
            &invocation.turn.config.mod_config.hooks,
            args.model.as_deref(),
            subagent
                .as_ref()
                .and_then(|subagent| subagent.model.as_deref()),
        ) {
            arguments.insert("model".to_string(), json!(model));
        }
        if let Some(effort) = args.effort.as_deref() {
            arguments.insert("reasoning_effort".to_string(), json!(effort));
        }
        let source = if self.version == MultiAgentVersion::V2 {
            arguments.insert(
                "task_name".to_string(),
                json!(agent::task_name(&args.description, &invocation.call_id)),
            );
            arguments.insert("fork_turns".to_string(), json!("none"));
            // The prompt is the model's plain text, not a message encrypted for transport.
            ToolCallSource::DirectPlaintextMessage
        } else {
            invocation.source.clone()
        };
        let payload = ToolPayload::Function {
            arguments: Value::Object(arguments).to_string(),
        };
        let output = self
            .spawn_agent
            .handle(ToolInvocation {
                payload: payload.clone(),
                source,
                ..invocation.clone()
            })
            .await?;
        let spawned = output_json(output.as_ref(), &payload).unwrap_or(Value::Null);
        let field = |name: &str| {
            spawned
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_string)
        };
        if self.version == MultiAgentVersion::V2 {
            let task_name = field("task_name")
                .ok_or_else(|| failed("spawn_agent did not name the new agent"))?;
            let thread_id =
                resolve_agent_target(&invocation.session, &invocation.turn, &task_name).await?;
            Ok((task_name, thread_id))
        } else {
            let agent_id = field("agent_id")
                .ok_or_else(|| failed("spawn_agent did not name the new agent"))?;
            let thread_id = ThreadId::from_string(&agent_id)
                .map_err(|err| failed(format!("invalid agent id {agent_id}: {err:?}")))?;
            Ok((agent_id, thread_id))
        }
    }
}

/// The model to start the agent on, as Claude Code picks it: the call's `model`, then the
/// subagent's own, then (`None`) the parent's. A Claude model name is mapped to a Codex model
/// by `[hooks.models]` of `ccn-config.toml`; the call's `model` is otherwise passed on as a
/// model name Codex knows, while a subagent's that the table does not map, or `inherit`, is
/// left out.
fn spawn_model(
    hooks: &HooksModConfig,
    requested: Option<&str>,
    subagent: Option<&str>,
) -> Option<String> {
    let mapped = |model: &str| {
        (!model.eq_ignore_ascii_case("inherit"))
            .then(|| hooks.model_for(Some(model)))
            .flatten()
            .map(str::to_string)
    };
    if let Some(requested) = requested.map(str::trim).filter(|model| !model.is_empty()) {
        return Some(mapped(requested).unwrap_or_else(|| requested.to_string()));
    }
    subagent
        .map(str::trim)
        .filter(|model| !model.is_empty())
        .and_then(mapped)
}

/// The agent's status once it is final, followed through Codex's status subscription as
/// `wait_agent` follows it.
async fn final_status(control: &LocalAgentControl, thread_id: ThreadId) -> AgentStatus {
    if let Ok(mut updates) = control.subscribe_status(thread_id).await {
        while let Some(Ok(snapshot)) = updates.next().await {
            if let Some(status) = snapshot.status()
                && is_final(status)
            {
                return status.clone();
            }
        }
    }
    control.get_status(thread_id).await
}

/// How long a finished background agent waits for Codex's completion watcher before it is
/// closed anyway; the watcher reads one status change and injects one notice.
const COMPLETION_WATCHER_LIMIT: Duration = Duration::from_secs(60);

/// Closes a background V1 agent once it has finished, so that it gives back its thread slot
/// as a foreground one does. Its result still reaches the conversation through Codex's
/// completion notice: the close waits until the completion watcher has injected it. The
/// final status is kept before the close, so that `wait_agent` on the agent still returns it.
fn close_when_finished(control: LocalAgentControl, thread_id: ThreadId) {
    let Ok(membership) = control.runtime.admit_start() else {
        return;
    };
    let teardown = membership.into_teardown_guard("claude_agent_close", Some(thread_id));
    tokio::spawn(async move {
        let status = tokio::select! {
            biased;
            _ = control.runtime.shutdown.cancelled() => None,
            status = final_status(&control, thread_id) => Some(status),
        };
        if let Some(status @ (AgentStatus::Completed(_) | AgentStatus::Errored(_))) = status {
            // Closing turns the status into `Shutdown` and removes the thread; the
            // completion notice takes the final status first.
            let watcher = control.wait_for_completion_watcher(thread_id, COMPLETION_WATCHER_LIMIT);
            let noticed = tokio::select! {
                biased;
                _ = control.runtime.shutdown.cancelled() => None,
                noticed = watcher => Some(noticed),
            };
            if let Some(noticed) = noticed {
                if !noticed {
                    tracing::debug!(
                        "closing background agent {thread_id} before its completion notice"
                    );
                }
                control.record_closed_result(thread_id, status);
                let _ = control.close_agent(thread_id).await;
            }
        }
        teardown.complete();
    });
}

/// Closes a foreground V1 agent: awaited once its result is in, or, when the call is
/// dropped before (an interrupted turn), started in the background.
struct CloseOnDrop {
    control: Option<LocalAgentControl>,
    thread_id: ThreadId,
}

impl CloseOnDrop {
    async fn close(mut self) {
        if let Some(control) = self.control.take() {
            let _ = control.close_agent(self.thread_id).await;
        }
    }
}

impl Drop for CloseOnDrop {
    fn drop(&mut self) {
        if let Some(control) = self.control.take()
            && let Ok(runtime) = tokio::runtime::Handle::try_current()
        {
            let thread_id = self.thread_id;
            runtime.spawn(async move {
                let _ = control.close_agent(thread_id).await;
            });
        }
    }
}

#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;
