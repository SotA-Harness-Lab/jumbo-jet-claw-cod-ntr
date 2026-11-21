// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::AgentType;
use super::NO_OUTPUT_MESSAGE;
use super::ToolList;
use super::completed_text;
use super::description;
use super::owner_of;
use super::role_for;
use super::task_name;
use super::unknown_agent_type_message;
use super::zero_tools_message;

fn tool_list(tools: Option<&str>, disallowed: Option<&str>) -> ToolList {
    let tools = tools.map(|text| serde_yaml::from_str(text).expect("yaml"));
    let disallowed = disallowed.map(|text| serde_yaml::from_str(text).expect("yaml"));
    ToolList::from_frontmatter(tools.as_ref(), disallowed.as_ref())
}

#[test]
fn tool_lists_read_as_in_claude_code() {
    assert_eq!(tool_list(None, None), ToolList::default());
    assert_eq!(
        tool_list(Some("Read, Grep Glob"), None).allowed,
        Some(vec![
            "Read".to_string(),
            "Grep".to_string(),
            "Glob".to_string()
        ])
    );
    assert_eq!(
        tool_list(Some("[Read, \"Bash(git log, git diff)\", Task]"), None).allowed,
        Some(vec![
            "Read".to_string(),
            "Bash".to_string(),
            "Agent".to_string()
        ])
    );
    assert_eq!(tool_list(Some("Read, *"), None).allowed, None);
    assert_eq!(tool_list(Some("\"\""), None).allowed, Some(Vec::new()));
    assert_eq!(tool_list(Some("~"), None).allowed, Some(Vec::new()));
    assert_eq!(
        tool_list(None, Some("Write, Edit")).disallowed,
        vec!["Write".to_string(), "Edit".to_string()]
    );
}

#[test]
fn tool_lists_keep_the_listed_tools_and_their_helpers() {
    let read_only = tool_list(Some("Read, Grep"), None);
    assert!(read_only.permits("Read", None));
    assert!(!read_only.permits("Write", None));
    assert!(!read_only.permits("write_stdin", owner_of("write_stdin")));
    assert!(!read_only.permits("mcp__github__create_issue", None));

    let with_bash = tool_list(Some("Bash, mcp__github__create_issue"), Some("Read"));
    assert!(with_bash.permits("write_stdin", owner_of("write_stdin")));
    assert!(with_bash.permits("mcp__github__create_issue", None));
    assert!(!with_bash.permits("Read", None));

    let no_writes = tool_list(None, Some("Write, Bash"));
    assert!(no_writes.permits("Read", None));
    assert!(!no_writes.permits("Write", None));
    assert!(!no_writes.permits("write_stdin", owner_of("write_stdin")));
    assert!(no_writes.permits("update_plan", owner_of("update_plan")));
}

#[test]
fn tool_lists_name_whole_mcp_servers() {
    let github = tool_list(Some("Read, mcp__github__*"), None);
    assert!(github.permits("mcp__github__create_issue", None));
    assert!(!github.permits("mcp__gitlab__create_issue", None));
    assert!(!github.permits("mcp__github_enterprise__create_issue", None));
    assert!(tool_list(Some("mcp__github"), None).permits("mcp__github__create_issue", None));
    // Without `__*`, `mcp__reports__v2` names the tool `v2` of the server `reports`.
    let reports = tool_list(Some("mcp__reports__v2"), None);
    assert!(!reports.permits("mcp__reports__v2__run_query", None));
    assert!(
        tool_list(Some("mcp__reports__v2__*"), None).permits("mcp__reports__v2__run_query", None)
    );
    // `mcp__*` gives no tool, but takes every MCP tool away.
    assert!(!tool_list(Some("mcp__*"), None).permits("mcp__github__create_issue", None));

    for denied in ["mcp__github", "mcp__github__*"] {
        let no_github = tool_list(None, Some(denied));
        assert!(!no_github.permits("mcp__github__create_issue", None));
        assert!(no_github.permits("mcp__gitlab__create_issue", None));
        assert!(no_github.permits("Read", None));
    }
    let no_reports = tool_list(None, Some("mcp__reports"));
    assert!(!no_reports.permits("mcp__reports__v2__run_query", None));
    let no_mcp = tool_list(None, Some("mcp__*"));
    assert!(!no_mcp.permits("mcp__github__create_issue", None));
    assert!(no_mcp.permits("Read", None));
}

#[test]
fn tool_lists_that_give_no_tool_are_refused() {
    let available = vec![
        ("Read".to_string(), None),
        ("Bash".to_string(), None),
        ("write_stdin".to_string(), owner_of("write_stdin")),
        ("mcp__github__create_issue".to_string(), None),
    ];
    let unresolved = |tools: Option<&str>, disallowed: Option<&str>| {
        tool_list(tools, disallowed).unresolved_entries(&available)
    };
    assert_eq!(
        unresolved(Some("Raed, Grpe, NotebookEdit, mcp__slack__*"), None),
        Some(vec![
            "Raed".to_string(),
            "Grpe".to_string(),
            "NotebookEdit".to_string(),
            "mcp__slack__*".to_string()
        ])
    );
    // One entry that resolves is enough, and a missing or emptied list is not refused.
    assert_eq!(unresolved(Some("Raed, Read"), None), None);
    assert_eq!(unresolved(Some("mcp__github__*"), None), None);
    assert_eq!(unresolved(None, None), None);
    assert_eq!(unresolved(Some("\"\""), None), None);
    assert_eq!(unresolved(Some("Read"), Some("Read")), None);

    assert_eq!(
        zero_tools_message(
            "code-reviewer",
            &[
                "Grpe".to_string(),
                "NotebookEdit".to_string(),
                "mcp__slack__*".to_string()
            ]
        ),
        "Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]; matched no tools in this session [NotebookEdit, mcp__slack__*]. Fix the agent's tools frontmatter or pass a different subagent_type."
    );
    assert_eq!(
        zero_tools_message("code-reviewer", &["Grpe".to_string()]),
        "Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type."
    );
}

#[test]
fn tool_lists_are_summed_up_as_in_claude_code() {
    assert_eq!(tool_list(None, None).summary(), "All tools");
    assert_eq!(
        tool_list(None, Some("Write, Edit")).summary(),
        "All tools except Write, Edit"
    );
    assert_eq!(
        tool_list(Some("Read, Write"), Some("Write")).summary(),
        "Read"
    );
    assert_eq!(tool_list(Some("Write"), Some("Write")).summary(), "None");
}

#[test]
fn subagent_types_map_to_roles() {
    let no_roles = |_: &str| false;
    assert_eq!(role_for(None, no_roles), None);
    assert_eq!(role_for(Some(" "), no_roles), None);
    assert_eq!(role_for(Some("general-purpose"), no_roles), None);
    assert_eq!(
        role_for(Some("general-purpose"), |name| name == "general-purpose"),
        Some("general-purpose")
    );
    assert_eq!(role_for(Some("reviewer"), no_roles), Some("reviewer"));
}

#[test]
fn agent_texts_follow_claude_code() {
    let agent_types = vec![
        AgentType {
            name: "general-purpose".to_string(),
            description: "Does anything.".to_string(),
            tools: ToolList::default(),
        },
        AgentType {
            name: "reviewer".to_string(),
            description: "Reviews code.".to_string(),
            tools: tool_list(Some("Read, Grep"), None),
        },
    ];
    let text = description(&agent_types);
    assert!(text.contains(
        "- general-purpose: Does anything. (Tools: All tools)\n- reviewer: Reviews code. (Tools: Read, Grep)\n"
    ));
    assert_eq!(
        unknown_agent_type_message("x", &agent_types),
        "Agent type 'x' not found. Available agents: general-purpose, reviewer"
    );
    assert_eq!(completed_text(Some("  Done.\n")), "Done.");
    assert_eq!(completed_text(Some("")), NO_OUTPUT_MESSAGE);
    assert_eq!(completed_text(None), NO_OUTPUT_MESSAGE);
}

#[test]
fn task_names_are_agent_path_segments() {
    assert_eq!(
        task_name("Review the API (v2)!", "call_AbC123xyz"),
        "review_the_api_v2_123xyz"
    );
    assert_eq!(task_name("", ""), "agent");
    assert_eq!(task_name("Root", "c1"), "agent_c1");
}
