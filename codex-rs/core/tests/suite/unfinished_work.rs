// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `[harness] continue_unfinished`: a turn about to end with its work left undone goes on.

use codex_core::TurnInputRequest;
use codex_protocol::protocol::EventMsg;
use codex_protocol::user_input::UserInput;
use core_test_support::responses::ResponseMock;
use core_test_support::responses::ev_assistant_message;
use core_test_support::responses::ev_completed;
use core_test_support::responses::ev_function_call;
use core_test_support::responses::ev_response_created;
use core_test_support::responses::mount_sse_sequence;
use core_test_support::responses::sse;
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::test_codex;
use core_test_support::wait_for_event;
use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;

const CONTINUATION: &str = "<unfinished_work_continuation>";

fn commentary(index: usize, text: &str) -> String {
    sse(vec![
        ev_response_created(&format!("resp-{index}")),
        json!({
            "type": "response.output_item.done",
            "item": {
                "type": "message",
                "role": "assistant",
                "id": format!("msg-{index}"),
                "phase": "commentary",
                "content": [{"type": "output_text", "text": text}]
            }
        }),
        ev_completed(&format!("resp-{index}")),
    ])
}

fn answer(index: usize, text: &str) -> String {
    sse(vec![
        ev_response_created(&format!("resp-{index}")),
        ev_assistant_message(&format!("msg-{index}"), text),
        ev_completed(&format!("resp-{index}")),
    ])
}

fn plan_call(index: usize, plan: Value) -> String {
    sse(vec![
        ev_response_created(&format!("resp-{index}")),
        ev_function_call(
            &format!("plan-{index}"),
            "update_plan",
            &json!({ "plan": plan }).to_string(),
        ),
        ev_completed(&format!("resp-{index}")),
    ])
}

/// Runs one turn against `responses`, returning the requests made and the warnings shown.
async fn run_turn(
    responses: Vec<String>,
    continue_unfinished: u32,
    judge: bool,
) -> anyhow::Result<(ResponseMock, Vec<String>)> {
    let server = start_mock_server().await;
    let response_mock = mount_sse_sequence(&server, responses).await;
    let test = test_codex()
        .with_config(move |config| {
            config.mod_config.harness.continue_unfinished = Some(continue_unfinished);
            config.mod_config.harness.unfinished_judge = Some(judge);
            config.update_plan_enabled = true;
            config.model_provider.stream_max_retries = Some(0);
            config.model_provider.request_max_retries = Some(0);
        })
        .build_with_auto_env(&server)
        .await?;
    test.codex
        .start_or_steer_turn(TurnInputRequest::user_input(vec![UserInput::Text {
            text: "fix the bug and update the docs".into(),
            text_elements: Vec::new(),
        }]))
        .await?;
    let mut warnings = Vec::new();
    wait_for_event(&test.codex, |event| {
        if let EventMsg::Warning(warning) = event {
            warnings.push(warning.message.clone());
        }
        matches!(event, EventMsg::TurnComplete(_))
    })
    .await;
    Ok((response_mock, warnings))
}

fn body(mock: &ResponseMock, index: usize) -> String {
    mock.requests()[index].body_json().to_string()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_turn_that_ends_on_commentary_goes_on() -> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let (mock, warnings) = run_turn(
        vec![
            commentary(1, "I'll run the tests now."),
            answer(2, "The tests pass."),
        ],
        /*continue_unfinished*/ 3,
        /*judge*/ false,
    )
    .await?;

    assert_eq!(mock.requests().len(), 2);
    assert!(!body(&mock, 0).contains(CONTINUATION));
    let second = body(&mock, 1);
    assert!(second.contains(CONTINUATION), "{second}");
    assert!(second.contains("no tool call followed it"), "{second}");
    assert_eq!(
        warnings,
        vec![
            "The model stopped before its work was done (it ended on what it would do next); \
             asking it to continue (1/3)."
                .to_string()
        ]
    );
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_turn_goes_on_no_more_often_than_allowed() -> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let (mock, warnings) = run_turn(
        vec![
            commentary(1, "Next I'll read the code."),
            commentary(2, "Next I'll read the code."),
        ],
        /*continue_unfinished*/ 1,
        /*judge*/ false,
    )
    .await?;

    assert_eq!(mock.requests().len(), 2);
    assert_eq!(warnings.len(), 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nothing_goes_on_when_it_is_turned_off() -> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let (mock, warnings) = run_turn(
        vec![commentary(1, "I'll run the tests now.")],
        /*continue_unfinished*/ 0,
        /*judge*/ true,
    )
    .await?;

    assert_eq!(mock.requests().len(), 1);
    assert_eq!(warnings, Vec::<String>::new());
    // Nor is the model told to finish the work.
    assert!(!body(&mock, 0).contains("# Finishing the work"));
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_plan_with_steps_left_sends_the_turn_back_once() -> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let (mock, warnings) = run_turn(
        vec![
            plan_call(
                1,
                json!([
                    {"step": "fix the bug", "status": "completed"},
                    {"step": "update the docs", "status": "pending"},
                ]),
            ),
            answer(2, "Fixed the bug."),
            // Sent back, the model answers again without touching the plan: that version of
            // the plan has been asked about, so the turn ends.
            answer(3, "The docs need your decision on the wording."),
        ],
        /*continue_unfinished*/ 3,
        /*judge*/ false,
    )
    .await?;

    assert_eq!(mock.requests().len(), 3);
    assert!(body(&mock, 0).contains("# Finishing the work"));
    let third = body(&mock, 2);
    assert!(third.contains(CONTINUATION), "{third}");
    assert!(third.contains("- update the docs"), "{third}");
    assert!(!third.contains("- fix the bug"), "{third}");
    assert_eq!(warnings.len(), 1);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_judge_sends_back_a_turn_that_left_work_it_could_do() -> anyhow::Result<()> {
    skip_if_no_network!(Ok(()));
    let (mock, warnings) = run_turn(
        vec![
            plan_call(1, json!([{"step": "fix the bug", "status": "completed"}])),
            answer(2, "Fixed the bug. Next I'll update the docs."),
            answer(
                3,
                r#"{"verdict":"unfinished","remaining":"Update the docs, as the user asked."}"#,
            ),
            answer(4, "Updated the docs."),
            answer(5, r#"{"verdict":"done","remaining":null}"#),
        ],
        /*continue_unfinished*/ 3,
        /*judge*/ true,
    )
    .await?;

    assert_eq!(mock.requests().len(), 5);
    // The judge reads the request and the last answer, under its own instructions.
    let judged = body(&mock, 2);
    assert!(judged.contains("You review the last message a coding agent wrote"), "{judged}");
    assert!(judged.contains("fix the bug and update the docs"), "{judged}");
    assert!(judged.contains("Next I'll update the docs."), "{judged}");
    let continued = body(&mock, 3);
    assert!(continued.contains(CONTINUATION), "{continued}");
    assert!(continued.contains("Update the docs, as the user asked."), "{continued}");
    // Nothing of the judge's own request reaches the session's history.
    assert!(!continued.contains("You review the last message"), "{continued}");
    assert_eq!(
        warnings,
        vec![
            "The model stopped before its work was done (its last answer leaves work it can \
             do); asking it to continue (1/3)."
                .to_string()
        ]
    );
    Ok(())
}
