// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Skill`: Claude Code's tool for invoking a skill by name, with arguments.
//!
//! Nothing is delegated: the skill is one of those Codex lists for the turn, read from this
//! machine and expanded as `/<name> <arguments>` expands it (frontmatter left out, headed by
//! its directory, arguments, `${CLAUDE_SKILL_DIR}` and a plugin's variables filled in).
//! Added: Claude Code's parameters, description, refusals and result text.

use std::collections::BTreeMap;

use codex_claude_compat::tools::SKILL;
use codex_claude_compat::tools::skill;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
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
    Ok(boxed_tool_output(ClaudeToolOutput::new(
        skill::result_text(&name, &expanded),
        json!({ "success": true, "commandName": name }),
    )))
}
