// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Skill`: Claude Code's tool for invoking a skill by name, with arguments.
//!
//! The skill is one of those Codex lists for the turn, read from this machine and expanded
//! as `/<name> <arguments>` expands it (frontmatter left out, headed by its directory,
//! arguments and `${CLAUDE_SKILL_DIR}` filled in). A `context: fork` skill is delegated to
//! the `Agent` tool, which runs the expanded text in a subagent. Added: Claude Code's
//! parameters, description, refusals and result texts.
//! its directory, arguments, `${CLAUDE_SKILL_DIR}` and a plugin's variables filled in).
//! Added: Claude Code's parameters, description, refusals and result text.

use std::collections::BTreeMap;
use std::sync::Arc;

use codex_claude_compat::SkillFrontmatter;
use codex_claude_compat::tools::AGENT;
use codex_claude_compat::tools::SKILL;
use codex_claude_compat::tools::skill;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use super::PermissionInput;
use super::ToolSwap;
use super::check_permission;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `Skill`, which no Codex tool stands for.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    swap.add(SkillHandler);
}

struct SkillHandler;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillArgs {
    skill: String,
    #[serde(default)]
    args: Option<String>,
}

impl ToolExecutor<ToolInvocation> for SkillHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(SKILL)
    }

    fn spec(&self) -> ToolSpec {
        let properties = BTreeMap::from([
            (
                "skill".to_string(),
                JsonSchema::string(Some(skill::SKILL_PARAM_DESCRIPTION.to_string())),
            ),
            (
                "args".to_string(),
                JsonSchema::string(Some(skill::ARGS_PARAM_DESCRIPTION.to_string())),
            ),
        ]);
        ToolSpec::Function(ResponsesApiTool {
            name: SKILL.to_string(),
            description: skill::DESCRIPTION.to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["skill".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(handle_call(invocation))
    }
}

impl CoreToolRuntime for SkillHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

/// The text of each skill a subagent's `skills` names, to start the agent with, as Claude
/// Code preloads them: the first [`skill::MAX_PRELOADED_SKILLS`] distinct names, each found
/// among the skills the model may invoke this turn by [`skill::resolve_preloaded_skill`] and
/// expanded as the `Skill` tool expands it. One that is not found or cannot be read is left
/// out, with a line in the log.
pub(super) async fn preloaded_skills(
    invocation: &ToolInvocation,
    agent_type: &str,
    names: &[String],
) -> Vec<String> {
    if names.is_empty() {
        return Vec::new();
    }
    let environments = invocation
        .session
        .services
        .turn_environments
        .snapshot()
        .await;
    let snapshot = invocation
        .session
        .host_skills_for_step(&invocation.turn, &environments)
        .await;
    let outcome = snapshot.outcome();
    let invocable = outcome
        .skills
        .iter()
        .filter(|candidate| {
            outcome.is_skill_enabled(candidate) && candidate.allows_implicit_invocation()
        })
        .collect::<Vec<_>>();
    let available = invocable
        .iter()
        .map(|candidate| candidate.name.as_str())
        .collect::<Vec<_>>();
    let session_id = invocation.session.session_id().to_string();
    let mut wanted = Vec::new();
    for name in names {
        let name = skill::normalize_name(name);
        if !name.is_empty() && !wanted.contains(&name) {
            wanted.push(name);
        }
    }
    wanted.truncate(skill::MAX_PRELOADED_SKILLS);
    let mut texts = Vec::new();
    for name in wanted {
        let Some(found) = skill::resolve_preloaded_skill(&available, agent_type, name)
            .and_then(|found| invocable.iter().find(|candidate| candidate.name == found))
        else {
            tracing::warn!("agent {agent_type}: skill {name} of its skills was not found");
            continue;
        };
        let Ok(path) = found.path_to_skills_md.to_abs_path() else {
            tracing::warn!("agent {agent_type}: skill {name} is not a file on this machine");
            continue;
        };
        let contents = match tokio::fs::read_to_string(path.as_path()).await {
            Ok(contents) => contents,
            Err(err) => {
                tracing::warn!("agent {agent_type}: cannot read skill {name}: {err}");
                continue;
            }
        };
        let skill_dir = path
            .as_path()
            .parent()
            .map(|dir| dir.to_string_lossy().into_owned())
            .unwrap_or_default();
        let expanded = codex_claude_compat::render_invoked_skill(
            &contents,
            &skill_dir,
            &session_id,
            /*arguments*/ "",
        );
        texts.push(skill::result_text(&found.name, &expanded));
    }
    texts
}

async fn handle_call(invocation: ToolInvocation) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let args: SkillArgs = parse_claude_arguments(SKILL, &invocation)?;
    let name = skill::normalize_name(&args.skill).to_string();

    let environments = invocation
        .session
        .services
        .turn_environments
        .snapshot()
        .await;
    let snapshot = invocation
        .session
        .host_skills_for_step(&invocation.turn, &environments)
        .await;
    let outcome = snapshot.outcome();
    let Some(found) = outcome
        .skills
        .iter()
        .find(|candidate| candidate.name == name && outcome.is_skill_enabled(candidate))
    else {
        return Err(rejected(&skill::unknown_skill_message(&name)));
    };
    if !found.allows_implicit_invocation() {
        return Err(rejected(&skill::model_invocation_disabled_message(&name)));
    }
    let _permission = check_permission(&invocation, SKILL, PermissionInput::Content(&name)).await?;

    let Ok(path) = found.path_to_skills_md.to_abs_path() else {
        return Err(failed(format!(
            "Skill {name} is not a file on this machine and cannot be loaded with the {SKILL} tool."
        )));
    };
    let contents = tokio::fs::read_to_string(path.as_path())
        .await
        .map_err(|err| failed(format!("Failed to read skill {name}: {err}")))?;
    let skill_dir = path
        .as_path()
        .parent()
        .map(|dir| dir.to_string_lossy().into_owned())
        .unwrap_or_default();
    let session_id = invocation.session.session_id().to_string();
    let expanded = codex_claude_compat::render_invoked_skill(
        &contents,
        &skill_dir,
        &session_id,
        args.args.as_deref().unwrap_or_default(),
    );
    let expanded = codex_claude_compat::substitute_in_plugin_content(expanded, path.as_path());
    let frontmatter = codex_claude_compat::skill_frontmatter(&contents);
    if frontmatter.context_fork
        && let Some(agent) = invocation
            .step_context
            .tool_router
            .registered_tool(&ToolName::plain(AGENT))
            .map(|tool| Arc::clone(&tool.runtime))
    {
        return run_forked(&invocation, agent, &name, &expanded, &frontmatter).await;
    }
    // The skill's `allowed-tools` run without asking for the rest of this turn.
    codex_skills_extension::grant_skill_tools(
        &invocation.turn.extension_data,
        codex_skills_extension::skill_tool_grants(&contents, &path.as_path().to_string_lossy()),
    );
    Ok(boxed_tool_output(ClaudeToolOutput::new(
        skill::result_text(&name, &expanded),
        json!({ "success": true, "commandName": name }),
    )))
}

/// Runs a `context: fork` skill as Claude Code runs it: its expanded text is the prompt of
/// a subagent of the type `agent` names (`general-purpose` when it names none), with the
/// skill's `model` (through `[hooks.models]` of `ccn-config.toml`) and `effort`, started and
/// waited for by the `Agent` tool; the subagent's answer is the result.
async fn run_forked(
    invocation: &ToolInvocation,
    agent: Arc<dyn CoreToolRuntime>,
    name: &str,
    prompt: &str,
    frontmatter: &SkillFrontmatter,
) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let mut arguments = Map::new();
    arguments.insert("description".to_string(), json!(format!("Skill {name}")));
    arguments.insert("prompt".to_string(), json!(prompt));
    if let Some(agent_type) = &frontmatter.agent {
        arguments.insert("subagent_type".to_string(), json!(agent_type));
    }
    if let Some(model) = frontmatter.model.as_deref().and_then(|model| {
        invocation
            .turn
            .config
            .mod_config
            .hooks
            .model_for(Some(model))
    }) {
        arguments.insert("model".to_string(), json!(model));
    }
    if let Some(effort) = &frontmatter.effort {
        arguments.insert("effort".to_string(), json!(effort));
    }
    let payload = ToolPayload::Function {
        arguments: Value::Object(arguments).to_string(),
    };
    let output = agent
        .handle(ToolInvocation {
            tool_name: ToolName::plain(AGENT),
            payload: payload.clone(),
            ..invocation.clone()
        })
        .await?;
    let result = match output.code_mode_result(&payload) {
        Value::String(text) => text,
        other => other.to_string(),
    };
    Ok(boxed_tool_output(ClaudeToolOutput::new(
        skill::forked_result_text(name, &result),
        json!({
            "success": true,
            "commandName": name,
            "status": "forked",
            "result": result,
        }),
    )))
}
