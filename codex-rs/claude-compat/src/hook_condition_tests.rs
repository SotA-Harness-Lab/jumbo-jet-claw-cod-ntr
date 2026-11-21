// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;

use super::hook_condition_matches;
use crate::permissions::file_pattern_matches;

const CWD: &str = "/repo";

fn check(actual: bool, expected: bool) {
    assert_eq!(actual, expected);
}

fn condition_matches(condition: &str, tool_names: &[&str], tool_input: &Value) -> bool {
    hook_condition_matches(condition, tool_names, tool_input, Path::new(CWD))
}

#[test]
fn tool_name_alone_covers_every_call_of_the_tool() {
    let input = json!({"command": "ls"});
    check(condition_matches("Bash", &["Bash"], &input), true);
    check(condition_matches("Bash(*)", &["Bash"], &input), true);
    check(condition_matches("Edit", &["Bash"], &input), false);
}

#[test]
fn a_tool_is_also_known_by_its_claude_code_names() {
    let input = json!({"file_path": "/repo/src/main.rs"});
    check(
        condition_matches("Edit(*.rs)", &["apply_patch", "Write", "Edit"], &input),
        true,
    );
    check(
        condition_matches("Read(*.rs)", &["apply_patch", "Write", "Edit"], &input),
        false,
    );
    // `Task` is the old name of `Agent`.
    check(
        condition_matches("Task", &["spawn_agent", "Agent"], &json!({})),
        true,
    );
}

#[test]
fn bash_content_is_matched_against_each_simple_command() {
    let matches = |condition: &str, command: &str| {
        condition_matches(condition, &["Bash"], &json!({"command": command}))
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
fn bash_content_is_matched_against_the_words_the_shell_runs() {
    let matches = |condition: &str, command: &str| {
        condition_matches(condition, &["Bash"], &json!({"command": command}))
    };
    check(matches("Bash(rm *)", "sudo rm -rf build"), true);
    check(matches("Bash(rm *)", "env -i rm -rf build"), true);
    check(matches("Bash(rm *)", "\"rm\" -rf build"), true);
    check(matches("Bash(rm *)", "TZ=\"$HOME\" rm -rf build"), true);
    check(matches("Bash(rm:*)", "timeout 5 rm -rf build"), true);
    check(matches("Bash(git push *)", "git  push --force"), true);
    check(matches("Bash(rm *)", "sudo ls"), false);
}

#[test]
fn a_command_that_cannot_be_taken_apart_runs_the_hook() {
    let matches =
        |command: &str| condition_matches("Bash(git *)", &["Bash"], &json!({"command": command}));
    check(matches("echo 'unclosed"), true);
    check(matches("echo $(git push"), true);
}

#[test]
fn bash_content_is_matched_against_nested_commands_only_where_they_match() {
    let matches = |condition: &str, command: &str| {
        condition_matches(condition, &["Bash"], &json!({"command": command}))
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
        condition_matches(
            "Write(*.md)",
            &["Write"],
            &json!({"file_path": "/repo/README.md"}),
        ),
        true,
    );
    check(
        condition_matches(
            "Write(*.md)",
            &["Write"],
            &json!({"file_path": "/repo/main.rs"}),
        ),
        false,
    );
    check(
        condition_matches("Grep(TODO*)", &["Grep"], &json!({"pattern": "TODO: fix"})),
        true,
    );
    check(
        condition_matches("Glob(*.ts)", &["Glob"], &json!({"pattern": "src/**/*.rs"})),
        false,
    );
}

#[test]
fn content_for_another_tool_never_matches() {
    check(
        condition_matches(
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
        condition_matches(
            "Edit(*.rs)",
            &["apply_patch", "Write", "Edit"],
            &json!({"command": "*** Begin Patch"}),
        ),
        true,
    );
}

#[test]
fn file_paths_are_matched_as_path_rules_match_them() {
    let read = |condition: &str, path: &str| {
        condition_matches(condition, &["Read"], &json!({"file_path": path}))
    };
    check(read("Read(.env)", "/repo/.env"), true);
    check(read("Read(.env)", "/repo/app/.env"), true);
    check(read("Read(.env)", "/elsewhere/.env"), false);
    check(read("Read(.env)", ".env"), true);
    check(read("Read(//etc/**)", "/etc/hosts"), true);
    check(read("Read(/src/*.rs)", "/repo/src/main.rs"), true);
    let edit = |condition: &str, path: &str| {
        condition_matches(condition, &["Edit"], &json!({"file_path": path}))
    };
    check(edit("Edit(src/**)", "/repo/src/a.rs"), true);
    check(edit("Edit(src/**)", "src/a.rs"), true);
    check(edit("Edit(src/**)", "/repo/vendor/src/a.rs"), false);
    check(edit("Edit(**/src/**)", "/repo/vendor/src/a.rs"), true);
    check(edit("Edit(*.RS)", "/repo/src/a.rs"), true);
    // A pattern that is no glob runs the hook.
    check(edit("Edit(src/{a)", "/repo/b.rs"), true);
}

#[test]
fn home_patterns_are_in_the_home_directory() {
    let home = Some(Path::new("/home/u"));
    let matches = |pattern: &str, path: &str| {
        file_pattern_matches(pattern, Path::new(path), Path::new(CWD), home)
    };
    check(matches("~/.ssh/**", "/home/u/.ssh/id_rsa"), true);
    check(matches("~/.ssh/**", "/repo/.ssh/id_rsa"), false);
    check(
        file_pattern_matches(
            "~/.ssh/**",
            Path::new("/home/u/.ssh/id_rsa"),
            Path::new(CWD),
            None,
        ),
        false,
    );
}
