// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::CommandRun;
use super::DEFAULT_TIMEOUT_MS;
use super::MAX_TIMEOUT_MS;
use super::background_message;
use super::backgrounds_on_timeout;
use super::description;
use super::format_result;
use super::moved_to_background_message;
use super::strip_empty_lines;
use super::timeout_ms;

fn run(output: &str, exit_code: Option<i32>) -> CommandRun<'_> {
    CommandRun {
        output,
        exit_code,
        timed_out: false,
        timeout_ms: DEFAULT_TIMEOUT_MS,
    }
}

#[test]
fn timeouts_default_and_are_capped() {
    assert_eq!(timeout_ms(None), DEFAULT_TIMEOUT_MS);
    assert_eq!(timeout_ms(Some(0.0)), DEFAULT_TIMEOUT_MS);
    assert_eq!(timeout_ms(Some(f64::NAN)), DEFAULT_TIMEOUT_MS);
    assert_eq!(timeout_ms(Some(5_000.0)), 5_000);
    assert_eq!(timeout_ms(Some(10_000_000.0)), MAX_TIMEOUT_MS);
}

#[test]
fn successful_output_is_trimmed() {
    assert_eq!(
        format_result(&run("\n  \nhello\nworld  \n\n", Some(0))),
        ("hello\nworld".to_string(), false)
    );
    assert_eq!(
        format_result(&run("", Some(0))),
        ("(Bash completed with no output)".to_string(), false)
    );
}

#[test]
fn a_non_zero_exit_is_an_error_with_its_code_first() {
    assert_eq!(
        format_result(&run("no such file\n", Some(2))),
        ("Exit code 2\nno such file".to_string(), true)
    );
    assert_eq!(
        format_result(&run("", Some(1))),
        ("Exit code 1".to_string(), true)
    );
}

#[test]
fn a_timeout_is_an_error() {
    let run = CommandRun {
        output: "partial",
        exit_code: None,
        timed_out: true,
        timeout_ms: 120_000,
    };
    assert_eq!(
        format_result(&run),
        ("Command timed out after 2m 0s\npartial".to_string(), true)
    );
}

#[test]
fn blank_lines_inside_the_output_stay() {
    assert_eq!(strip_empty_lines("a\n\nb\n"), "a\n\nb");
    assert_eq!(strip_empty_lines(" \n \n"), "");
}

#[test]
fn background_result_names_the_session() {
    assert!(background_message("7").starts_with("Command running in background with ID: 7."));
}

#[test]
fn description_points_only_to_offered_tools_and_has_no_attribution() {
    let text = description(&["Bash", "Read", "Edit", "Write"], true);
    assert!(text.contains("Read files: Use Read (NOT cat/head/tail)"));
    assert!(!text.contains("Use Glob"));
    assert!(!text.contains("TodoWrite"));
    assert!(text.contains("run_in_background"));
    assert!(!text.contains("Co-Authored-By"));
    assert!(!text.contains("Claude Code"));

    let without_background = description(&["Bash"], false);
    assert!(!without_background.contains("run_in_background"));
    assert!(!without_background.contains("Read files"));
}

#[test]
fn a_command_moved_to_the_background_reports_its_id_and_what_it_wrote() {
    assert_eq!(
        moved_to_background_message("1000", 120_000, "\nbuilding\n\n"),
        "Command did not complete within its 2m 0s timeout and was moved to the background with ID: 1000. It is still running. Read its further output with write_stdin, passing session_id 1000 and empty chars.\nOutput so far:\nbuilding"
    );
    assert!(
        moved_to_background_message("7", 5_000, "").ends_with("It has written no output so far.")
    );
}

#[test]
fn sleep_is_not_left_in_the_background() {
    assert!(backgrounds_on_timeout("cargo build --release"));
    assert!(backgrounds_on_timeout("FOO=1 sleep 5"));
    assert!(!backgrounds_on_timeout("sleep 600"));
    assert!(!backgrounds_on_timeout("  sleep 1 && echo done"));
}
