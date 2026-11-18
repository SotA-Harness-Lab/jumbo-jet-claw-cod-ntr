use pretty_assertions::assert_eq;
use serde_json::json;

use super::GrepInput;
use super::OutputMode;
use super::description;
use super::format_result;
use super::rg_args;
use super::split_glob;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

const COMMON: &[&str] = &[
    "--hidden",
    "--glob",
    "!.git",
    "--glob",
    "!.svn",
    "--glob",
    "!.hg",
    "--glob",
    "!.bzr",
    "--glob",
    "!.jj",
    "--glob",
    "!.sl",
    "--max-columns",
    "500",
];

fn with_common(rest: &[&str]) -> Vec<String> {
    strings(COMMON).into_iter().chain(strings(rest)).collect()
}

#[test]
fn defaults_list_files_newest_first() {
    let input = GrepInput::parse(json!({ "pattern": "fn main" })).unwrap();
    assert_eq!(input.output_mode, OutputMode::FilesWithMatches);
    assert_eq!(
        rg_args(&input),
        with_common(&["-l", "--sortr=modified", "fn main"])
    );
}

#[test]
fn content_mode_takes_flags_context_type_and_globs() {
    let input = GrepInput::parse(json!({
        "pattern": "-x",
        "output_mode": "content",
        "-i": "true",
        "multiline": true,
        "-B": 2,
        "-A": "3",
        "type": "rust",
        "glob": "*.rs,*.toml *.{ts,tsx}",
    }))
    .unwrap();
    assert_eq!(
        rg_args(&input),
        with_common(&[
            "-U",
            "--multiline-dotall",
            "-i",
            "-n",
            "-B",
            "2",
            "-A",
            "3",
            "-e",
            "-x",
            "--type",
            "rust",
            "--glob",
            "*.rs",
            "--glob",
            "*.toml",
            "--glob",
            "*.{ts,tsx}",
        ])
    );

    // `context` wins over `-C`, which wins over `-B` and `-A`; `-n: false` drops numbers.
    let input = GrepInput::parse(json!({
        "pattern": "x",
        "output_mode": "content",
        "-n": false,
        "-C": 1,
        "context": 4,
        "-A": 9,
    }))
    .unwrap();
    assert_eq!(rg_args(&input), with_common(&["-C", "4", "x"]));

    // Context is for content mode only.
    let input =
        GrepInput::parse(json!({ "pattern": "x", "output_mode": "count", "-C": 1 })).unwrap();
    assert_eq!(rg_args(&input), with_common(&["-c", "x"]));
}

#[test]
fn invalid_input_is_reported_as_claude_code_does() {
    assert_eq!(
        GrepInput::parse(json!({ "path": 3, "extra": 1 })),
        Err("InputValidationError: Grep failed due to the following issues:\nAn unexpected parameter `extra` was provided\nThe required parameter `pattern` is missing\nThe parameter `path` type is expected as `string` but provided as `number`".to_string())
    );
    assert!(
        GrepInput::parse(json!({ "pattern": "x", "output_mode": "lines" }))
            .unwrap_err()
            .contains("`output_mode` is expected as one of")
    );
    assert!(GrepInput::parse(json!({ "pattern": "x", "head_limit": -1 })).is_err());
}

#[test]
fn glob_splits_on_spaces_and_on_commas_outside_braces() {
    assert_eq!(
        split_glob("*.js, *.{a,b} src/**,,x"),
        strings(&["*.js", "*.{a,b}", "src/**", "x"])
    );
}

#[test]
fn files_mode_lists_files_with_pagination() {
    let input = GrepInput::parse(json!({ "pattern": "x", "head_limit": 2, "offset": 1 })).unwrap();
    let result = format_result(
        &input,
        strings(&["/w/a.rs", "/w/b.rs", "/elsewhere/c.rs", "/w/d.rs"]),
        "/w",
    );
    assert_eq!(
        result.text,
        "Found 2 files limit: 2, offset: 1\nb.rs\n/elsewhere/c.rs"
    );
    assert_eq!(
        result.tool_response,
        json!({
            "mode": "files_with_matches",
            "numFiles": 2,
            "filenames": ["b.rs", "/elsewhere/c.rs"],
            "appliedLimit": 2,
            "appliedOffset": 1,
        })
    );

    let input = GrepInput::parse(json!({ "pattern": "x" })).unwrap();
    assert_eq!(
        format_result(&input, strings(&["/w/a.rs"]), "/w").text,
        "Found 1 file\na.rs"
    );
    assert_eq!(
        format_result(&input, Vec::new(), "/w").text,
        "No files found"
    );
}

#[test]
fn content_mode_shows_lines() {
    let input = GrepInput::parse(json!({ "pattern": "x", "output_mode": "content" })).unwrap();
    let result = format_result(
        &input,
        strings(&["/w/a.rs:1:x", "/w/a.rs-2-y", "--", "/w/b.rs:9:x"]),
        "/w",
    );
    assert_eq!(result.text, "a.rs:1:x\na.rs-2-y\n--\nb.rs:9:x");
    assert_eq!(result.tool_response["numLines"], json!(4));
    assert_eq!(
        format_result(&input, Vec::new(), "/w").text,
        "No matches found"
    );

    let input = GrepInput::parse(json!({
        "pattern": "x",
        "output_mode": "content",
        "head_limit": 1,
    }))
    .unwrap();
    assert_eq!(
        format_result(&input, strings(&["/w/a:1:x", "/w/a:2:x"]), "/w").text,
        "a:1:x\n\n[Showing results with pagination = limit: 1]"
    );
}

#[test]
fn count_mode_sums_matches() {
    let input = GrepInput::parse(json!({ "pattern": "x", "output_mode": "count" })).unwrap();
    let result = format_result(&input, strings(&["/w/a.rs:3", "/w/b:c.rs:1"]), "/w");
    assert_eq!(
        result.text,
        "a.rs:3\nb:c.rs:1\n\nFound 4 total occurrences across 2 files."
    );
    assert_eq!(result.tool_response["numMatches"], json!(4));
    assert_eq!(
        format_result(&input, strings(&["/w/a.rs:1"]), "/w").text,
        "a.rs:1\n\nFound 1 total occurrence across 1 file."
    );
    assert_eq!(
        format_result(&input, Vec::new(), "/w").text,
        "No matches found\n\nFound 0 total occurrences across 0 files."
    );
}

#[test]
fn description_points_to_agent_only_when_offered() {
    assert!(!description(&["Bash", "Grep"]).contains("Agent tool"));
    assert!(description(&["Bash", "Grep", "Agent"]).contains("Use Agent tool for open-ended"));
    assert!(description(&[]).contains(r#""function\s+\w+""#));
}
