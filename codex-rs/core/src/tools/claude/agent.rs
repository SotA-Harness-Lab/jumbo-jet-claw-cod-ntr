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
use super::check_permission;
use super::ToolSwap;
use super::ask_user_question::output_json;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::subagent_tools::role_tool_list;
use crate::agent::agent_resolver::resolve_agent_target;
use crate::agent::role::DEFAULT_ROLE_NAME;
use crate::agent::status::is_final;
use crate::config::Config;
use crate::function_tool::FunctionCallError;
use crate::session::session::Session;
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

        let (agent_id, thread_id) = self.spawn(&invocation, &args, role).await?;
        if args.run_in_background {
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

        let status = final_status(&invocation.session, thread_id).await;
        if self.version == MultiAgentVersion::V1 {
            // A finished V1 agent keeps its thread slot until it is closed; Claude Code's
            // subagent ends with its answer.
            let _ = invocation
                .session
                .services
                .local_agent_runtime
                .control(invocation.session.session_id())
                .close_agent(thread_id)
                .await;
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
        arguments.insert("message".to_string(), json!(args.prompt));
        if let Some(role) = role {
            arguments.insert("agent_type".to_string(), json!(role));
        }
        if let Some(model) = args.model.as_deref().filter(|model| !model.is_empty()) {
            arguments.insert("model".to_string(), json!(model));
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

/// The agent's status once it is final, followed through Codex's status subscription as
/// `wait_agent` follows it.
async fn final_status(session: &Arc<Session>, thread_id: ThreadId) -> AgentStatus {
    let control = session
        .services
        .local_agent_runtime
        .control(session.session_id());
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
