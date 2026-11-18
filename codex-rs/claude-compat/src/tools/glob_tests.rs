use pretty_assertions::assert_eq;
use serde_json::json;

use super::GlobInput;
use super::MAX_RESULTS;
use super::TRUNCATED_NOTE;
use super::description;
use super::format_result;
use super::rg_args;
use super::split_absolute_pattern;

fn pair(directory: &str, pattern: &str) -> Option<(String, String)> {
    Some((directory.to_string(), pattern.to_string()))
}

#[test]
fn absolute_patterns_split_before_the_first_wildcard() {
    assert_eq!(split_absolute_pattern("**/*.rs"), None);
    assert_eq!(split_absolute_pattern("src/*.rs"), None);
    assert_eq!(
        split_absolute_pattern("/w/src/**/*.rs"),
        pair("/w/src", "**/*.rs")
    );
    assert_eq!(
        split_absolute_pattern("/w/a/{b,c}/x"),
        pair("/w/a", "{b,c}/x")
    );
    assert_eq!(split_absolute_pattern("/*.txt"), pair("/", "*.txt"));
    assert_eq!(
        split_absolute_pattern("/w/Cargo.toml"),
        pair("/w", "Cargo.toml")
    );
    assert_eq!(split_absolute_pattern(r"C:\w\*.rs"), pair(r"C:\w", "*.rs"));
    assert_eq!(split_absolute_pattern("C:/*.rs"), pair(r"C:\", "*.rs"));
}

#[test]
fn files_are_listed_oldest_first_with_hidden_and_ignored_ones() {
    assert_eq!(
        rg_args("**/*.rs"),
        vec![
            "--files",
            "--glob",
            "**/*.rs",
            "--sort=modified",
            "--no-ignore",
            "--hidden"
        ]
    );
}

#[test]
fn input_is_validated() {
    assert_eq!(
        GlobInput::parse(json!({ "pattern": "*.rs", "path": "" })),
        Ok(GlobInput {
            pattern: "*.rs".to_string(),
            path: None,
        })
    );
    assert_eq!(
        GlobInput::parse(json!({})),
        Err("InputValidationError: Glob failed due to the following issue:\nThe required parameter `pattern` is missing".to_string())
    );
}

#[test]
fn results_are_limited_and_shortened() {
    let result = format_result(vec!["/w/a.rs".to_string(), "/x/b.rs".to_string()], "/w", 7);
    assert_eq!(result.text, "a.rs\n/x/b.rs");
    assert_eq!(
        result.tool_response,
        json!({
            "durationMs": 7,
            "numFiles": 2,
            "filenames": ["a.rs", "/x/b.rs"],
            "truncated": false,
        })
    );

    let many: Vec<String> = (0..=MAX_RESULTS).map(|i| format!("/w/{i}")).collect();
    let result = format_result(many, "/w", 0);
    assert_eq!(result.text.lines().count(), MAX_RESULTS + 1);
    assert!(result.text.ends_with(TRUNCATED_NOTE));
    assert_eq!(result.tool_response["truncated"], json!(true));

    assert_eq!(format_result(Vec::new(), "/w", 0).text, "No files found");
}

#[test]
fn description_points_to_agent_only_when_offered() {
    assert!(!description(&["Glob"]).contains("Agent"));
    assert!(description(&["Glob", "Agent"]).ends_with("use the Agent tool instead"));
}
