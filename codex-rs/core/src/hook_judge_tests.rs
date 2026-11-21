// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use super::AGENT_INSTRUCTIONS;
use super::PROMPT_INSTRUCTIONS;

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
