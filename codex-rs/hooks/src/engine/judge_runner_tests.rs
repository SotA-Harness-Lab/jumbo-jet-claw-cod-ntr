// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::verdict_output;
use super::with_arguments;

#[test]
fn the_hook_input_replaces_arguments_or_follows_the_prompt() {
    assert_eq!(
        with_arguments("Check $ARGUMENTS twice: $ARGUMENTS", r#"{"a":1}"#),
        r#"Check {"a":1} twice: {"a":1}"#
    );
    assert_eq!(
        with_arguments("Did the tests pass?", r#"{"a":1}"#),
        "Did the tests pass?\n\nARGUMENTS: {\"a\":1}"
    );
}

#[test]
fn a_verdict_reads_as_a_command_hook_result() {
    assert_eq!(
        verdict_output("Prompt", r#" {"ok": true} "#),
        (0, String::new(), String::new())
    );
    assert_eq!(
        verdict_output("Prompt", r#"{"ok": false, "reason": "tests did not run"}"#),
        (
            2,
            String::new(),
            "Prompt hook condition was not met: tests did not run".to_string()
        )
    );
    assert_eq!(
        verdict_output("Agent", r#"{"ok": false}"#),
        (
            2,
            String::new(),
            "Agent hook condition was not met: no reason given".to_string()
        )
    );
    let (exit_code, stdout, stderr) = verdict_output("Prompt", "looks fine to me");
    assert_eq!((exit_code, stdout.as_str()), (1, "looks fine to me"));
    assert!(stderr.starts_with("JSON validation failed: "), "{stderr}");
}
