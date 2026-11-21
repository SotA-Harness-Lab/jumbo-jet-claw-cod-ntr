// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use codex_claude_compat::tools::agent::ToolList;
use pretty_assertions::assert_eq;

use super::super::ToolSwap;
use super::super::ask_user_question;
use super::super::edit;
use super::super::read;
use super::super::todo_write;
use super::super::write;
use super::apply_subagent_permission_mode;
use super::restrict;
use super::role_tool_list;
use crate::config::AgentRoleConfig;
use crate::tools::handlers::ApplyPatchHandler;
use crate::tools::handlers::PlanHandler;
use crate::tools::handlers::RequestUserInputHandler;
use crate::tools::handlers::ViewImageHandler;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolRegistry;

fn claude_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::from_tools([
        Arc::new(ApplyPatchHandler::default()) as Arc<dyn CoreToolRuntime>,
        Arc::new(ViewImageHandler::default()) as Arc<dyn CoreToolRuntime>,
        Arc::new(PlanHandler) as Arc<dyn CoreToolRuntime>,
        Arc::new(RequestUserInputHandler {
            available_modes: Vec::new(),
        }) as Arc<dyn CoreToolRuntime>,
    ]);
    let mut swap = ToolSwap::new(&mut registry);
    read::install(&mut swap);
    write::install(&mut swap);
    edit::install(&mut swap);
    todo_write::install(&mut swap);
    ask_user_question::install(&mut swap);
    registry
}

fn names(registry: &ToolRegistry) -> Vec<String> {
    let mut names = registry
        .tool_names_for_test()
        .into_iter()
        .map(|name| name.name)
        .collect::<Vec<_>>();
    names.sort();
    names
}

#[test]
fn plan_and_question_tools_are_replaced() {
    let registry = claude_registry();
    assert_eq!(
        names(&registry),
        vec!["AskUserQuestion", "Edit", "Read", "TodoWrite", "Write"]
    );
}

#[test]
fn a_subagent_keeps_only_its_tools() {
    let mut registry = claude_registry();
    restrict(
        &mut registry,
        &ToolList {
            allowed: Some(vec!["Read".to_string(), "TodoWrite".to_string()]),
            disallowed: Vec::new(),
        },
    );
    assert_eq!(names(&registry), vec!["Read", "TodoWrite"]);

    let mut registry = claude_registry();
    restrict(
        &mut registry,
        &ToolList {
            allowed: None,
            disallowed: vec!["Write".to_string(), "Edit".to_string()],
        },
    );
    assert_eq!(
        names(&registry),
        vec!["AskUserQuestion", "Read", "TodoWrite"]
    );
}

#[test]
fn tool_lists_come_from_claude_subagent_files_only() {
    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("reviewer.md");
    std::fs::write(
        &file,
        "---\nname: reviewer\ndescription: Reviews.\ntools: Read, Grep\ndisallowedTools: Grep\n---\nReview.\n",
    )
    .expect("write subagent");
    let role = |config_file| AgentRoleConfig {
        description: None,
        config_file,
        nickname_candidates: None,
    };
    assert_eq!(
        role_tool_list(&role(Some(file))),
        Some(ToolList {
            allowed: Some(vec!["Read".to_string(), "Grep".to_string()]),
            disallowed: vec!["Grep".to_string()],
        })
    );
    assert_eq!(
        role_tool_list(&role(Some(dir.path().join("worker.toml")))),
        None
    );
    assert_eq!(role_tool_list(&role(None)), None);
}

#[tokio::test]
async fn a_subagent_set_to_dont_ask_runs_without_approvals() {
    use codex_protocol::protocol::AskForApproval;

    let dir = tempfile::tempdir().expect("tempdir");
    let mut config = crate::config::test_config().await;
    config
        .permissions
        .approval_policy
        .set(AskForApproval::OnRequest)
        .expect("on-request");
    for (name, mode) in [("quiet", "dontAsk"), ("planner", "plan")] {
        let file = dir.path().join(format!("{name}.md"));
        std::fs::write(
            &file,
            format!("---\nname: {name}\ndescription: d\npermissionMode: {mode}\n---\nWork.\n"),
        )
        .expect("write subagent");
        config.agent_roles.insert(
            name.to_string(),
            AgentRoleConfig {
                description: None,
                config_file: Some(file),
                nickname_candidates: None,
            },
        );
    }

    // Modes without a Codex counterpart keep the parent's policy.
    apply_subagent_permission_mode(&mut config, Some("planner"));
    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::OnRequest
    );
    apply_subagent_permission_mode(&mut config, None);
    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::OnRequest
    );

    apply_subagent_permission_mode(&mut config, Some("quiet"));
    assert_eq!(
        config.permissions.approval_policy.value(),
        AskForApproval::Never
    );
}
