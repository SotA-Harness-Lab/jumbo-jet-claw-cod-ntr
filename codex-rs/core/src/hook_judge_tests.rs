// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use pretty_assertions::assert_eq;

use super::AGENT_INSTRUCTIONS;
use super::ModelFailed;
use super::PROMPT_INSTRUCTIONS;
use super::falls_back_to_session_model;

#[test]
fn a_failing_hooks_model_falls_back_to_the_session_s_model() {
    let refused = Err::<(), _>(std::io::Error::other("model_not_found"))
        .context(ModelFailed("the request to the model failed"))
        .unwrap_err();
    let no_answer = anyhow::Error::msg(ModelFailed("the model gave no answer"));
    let overran = anyhow::anyhow!("the verifier did not answer within 50 tool calls");

    assert!(falls_back_to_session_model(&refused, "gpt-typo", "gpt-5.5"));
    assert!(falls_back_to_session_model(
        &no_answer, "gpt-typo", "gpt-5.5"
    ));
    // Not the model's failure, or the session's model has already failed.
    assert!(!falls_back_to_session_model(
        &overran, "gpt-typo", "gpt-5.5"
    ));
    assert!(!falls_back_to_session_model(&refused, "gpt-5.5", "gpt-5.5"));
    // The message the hook shows stays the same.
    assert_eq!(refused.to_string(), "the request to the model failed");
}

#[test]
fn both_kinds_of_hook_read_a_rule_as_well_as_a_condition() {
    for instructions in [PROMPT_INSTRUCTIONS, AGENT_INSTRUCTIONS] {
        assert!(
            instructions.contains("a rule about what to block or allow")
                && instructions
                    .contains("when the input is what a rule blocks, the rule is not met")
                && instructions.contains("not met means the agent has to keep working"),
            "{instructions}"
        );
        assert!(
            instructions.ends_with(
                "2. If it is not met: {\"ok\": false, \"reason\": \"Reason for why it is not met\"}"
            ),
            "{instructions}"
        );
    }
}
