// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::hook_condition_matches;

fn check(actual: bool, expected: bool) {
    assert_eq!(actual, expected);
}

#[test]
fn tool_name_alone_covers_every_call_of_the_tool() {
    let input = json!({"command": "ls"});
    check(hook_condition_matches("Bash", &["Bash"], &input), true);
    check(hook_condition_matches("Bash(*)", &["Bash"], &input), true);
    check(hook_condition_matches("Edit", &["Bash"], &input), false);
}

#[test]
fn a_tool_is_also_known_by_its_claude_code_names() {
    let input = json!({"file_path": "/repo/src/main.rs"});
    check(
        hook_condition_matches("Edit(*.rs)", &["apply_patch", "Write", "Edit"], &input),
        true,
    );
    check(
        hook_condition_matches("Read(*.rs)", &["apply_patch", "Write", "Edit"], &input),
        false,
    );
    // `Task` is the old name of `Agent`.
    check(
        hook_condition_matches("Task", &["spawn_agent", "Agent"], &json!({})),
        true,
    );
}

#[test]
fn bash_content_is_matched_against_each_simple_command() {
    let matches = |condition: &str, command: &str| {
        hook_condition_matches(condition, &["Bash"], &json!({"command": command}))
    };
    check(matches("Bash(git *)", "git push origin main"), true);
    check(matches("Bash(git *)", "git"), true);
    check(matches("Bash(git *)", "ls -la"), false);
    check(matches("Bash(git *)", "ls && git push"), true);
    check(matches("Bash(git *)", "FOO=bar git push"), true);
    check(matches("Bash(git push:*)", "git push --force"), true);
    check(matches("Bash(git push:*)", "git pushx"), false);
    check(matches("Bash(npm test)", "npm test"), true);
    check(matches("Bash(npm test)", "npm test --watch"), false);
    check(matches("Bash(python *)", "python x.py > out.txt"), true);
}

#[test]
fn a_command_that_cannot_be_taken_apart_runs_the_hook() {
    let matches = |command: &str| {
        hook_condition_matches("Bash(git *)", &["Bash"], &json!({"command": command}))
    };
    check(matches("echo 'unclosed"), true);
    check(matches("echo $(git push"), true);
}

#[test]
fn bash_content_is_matched_against_nested_commands_only_where_they_match() {
    let matches = |condition: &str, command: &str| {
        hook_condition_matches(condition, &["Bash"], &json!({"command": command}))
    };
    check(matches("Bash(git *)", "echo $(git push)"), true);
    check(matches("Bash(git *)", "echo `git push`"), true);
    check(matches("Bash(rm *)", "(cd build && rm -rf out)"), true);
    check(matches("Bash(rm *)", "for f in a; do rm $f; done"), true);
    check(matches("Bash(rm *)", "echo \"$(date)\" && { rm x; }"), true);
    check(matches("Bash(rm *)", "echo $(date)"), false);
    check(matches("Bash(rm *)", "for f in *; do echo $f; done"), false);
    check(matches("Bash(cat *)", "echo $(date) more"), false);
    check(
        matches(
            "Bash(git commit *)",
            "git commit -m \"$(cat <<'EOF'\nmsg\nEOF\n)\"",
        ),
        true,
    );
}

#[test]
fn file_and_search_tools_match_their_path_or_pattern() {
    check(
        hook_condition_matches(
            "Write(*.md)",
            &["Write"],
            &json!({"file_path": "/a/README.md"}),
        ),
        true,
    );
    check(
        hook_condition_matches(
            "Write(*.md)",
            &["Write"],
            &json!({"file_path": "/a/main.rs"}),
        ),
        false,
    );
    check(
        hook_condition_matches("Grep(TODO*)", &["Grep"], &json!({"pattern": "TODO: fix"})),
        true,
    );
    check(
        hook_condition_matches("Glob(*.ts)", &["Glob"], &json!({"pattern": "src/**/*.rs"})),
        false,
    );
}

#[test]
fn content_for_another_tool_never_matches() {
    check(
        hook_condition_matches(
            "WebFetch(domain:example.com)",
            &["WebFetch"],
            &json!({"url": "https://example.com"}),
        ),
        false,
    );
}

#[test]
fn input_without_the_matched_field_runs_the_hook() {
    check(
        hook_condition_matches(
            "Edit(*.rs)",
            &["apply_patch", "Write", "Edit"],
            &json!({"command": "*** Begin Patch"}),
        ),
        true,
    );
}
