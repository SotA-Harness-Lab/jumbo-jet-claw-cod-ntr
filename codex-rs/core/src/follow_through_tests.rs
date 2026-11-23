// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use codex_protocol::ThreadId;
use codex_protocol::plan_tool::PlanItemArg;
use codex_protocol::plan_tool::StepStatus;
use codex_protocol::plan_tool::UpdatePlanArgs;
use codex_protocol::protocol::InternalSessionSource;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use pretty_assertions::assert_eq;

use super::applies_to;
use super::judge_prompt;
use super::parse_verdict;
use super::tail;
use super::unfinished_steps;

#[test]
fn only_sessions_that_do_the_user_s_work_are_kept_going() {
    assert!(applies_to(&SessionSource::Cli));
    assert!(applies_to(&SessionSource::Exec));
    assert!(applies_to(&SessionSource::SubAgent(
        SubAgentSource::ThreadSpawn {
            parent_thread_id: ThreadId::new(),
            depth: 1,
            agent_path: None,
            agent_nickname: None,
            agent_role: None,
        }
    )));
    assert!(!applies_to(&SessionSource::Internal(
        InternalSessionSource::MemoryConsolidation
    )));
    assert!(!applies_to(&SessionSource::SubAgent(SubAgentSource::Review)));
    assert!(!applies_to(&SessionSource::SubAgent(SubAgentSource::Compact)));
    assert!(!applies_to(&SessionSource::SubAgent(SubAgentSource::Other(
        "hook_judge".to_string()
    ))));
}

#[test]
fn a_plan_s_unfinished_steps_are_the_ones_not_completed() {
    let step = |step: &str, status| PlanItemArg {
        step: step.to_string(),
        status,
    };
    let args = UpdatePlanArgs {
        explanation: None,
        plan: vec![
            step("read the code", StepStatus::Completed),
            step("change it", StepStatus::InProgress),
            step(" ", StepStatus::Pending),
            step("test it", StepStatus::Pending),
        ],
    };
    assert_eq!(unfinished_steps(&args), vec!["change it", "test it"]);
}

#[test]
fn only_an_unfinished_verdict_continues_the_turn() {
    assert_eq!(
        parse_verdict(r#"{"verdict":"unfinished","remaining":" Run the tests. "}"#),
        Some("Run the tests.".to_string())
    );
    assert_eq!(
        parse_verdict("```json\n{\"verdict\":\"unfinished\",\"remaining\":null}\n```"),
        Some("Finish what the user asked for.".to_string())
    );
    assert_eq!(
        parse_verdict(r#"{"verdict":"needs_user","remaining":null}"#),
        None
    );
    assert_eq!(parse_verdict(r#"{"verdict":"done","remaining":null}"#), None);
    // An answer that is not a verdict lets the turn end, as it would have.
    assert_eq!(parse_verdict("unfinished"), None);
}

#[test]
fn the_judge_reads_the_ends_of_long_texts() {
    assert_eq!(tail("  short  ", 10), "short");
    assert_eq!(tail("一二三四五六", 3), "[...]四五六");
}

#[test]
fn the_judge_is_given_the_request_the_answer_and_the_plan_left() {
    let prompt = judge_prompt(
        Some("fix the bug"),
        "Fixed the parser. Next I'll update the tests.",
        &["update the tests".to_string()],
    );
    assert_eq!(
        prompt,
        "<user_request>\nfix the bug\n</user_request>\n\n\
         <agent_last_message>\nFixed the parser. Next I'll update the tests.\n</agent_last_message>\n\
         \n<unfinished_plan_steps>\n- update the tests\n</unfinished_plan_steps>\n"
    );
    assert!(judge_prompt(None, "Done.", &[]).contains("(not available)"));
}
