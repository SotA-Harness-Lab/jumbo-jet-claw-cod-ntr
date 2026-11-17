use std::path::Path;

use pretty_assertions::assert_eq;
use serde_json::json;

use super::mcp_server_declarations;

fn env(name: &str) -> Option<String> {
    (name == "TOKEN").then(|| "secret".to_string())
}

#[test]
fn sources_are_merged_in_claude_code_order() {
    let claude_json = json!({
        "mcpServers": {
            "a": {"command": "user-a"},
            "b": {"command": "user-b"}
        },
        "projects": {
            "/work/p": {
                "mcpServers": {"c": {"command": "local-c"}},
                "enabledMcpjsonServers": ["b"]
            }
        }
    });
    let settings = [json!({"disabledMcpjsonServers": ["d"]})];
    let mcp_json = [
        json!({"mcpServers": {
            "b": {"command": "project-b", "args": ["--token=${TOKEN}", "${MISSING}", "${PORT:-80}"]},
            "c": {"command": "project-c"},
            "d": {"command": "project-d"}
        }}),
    ];

    let servers = mcp_server_declarations(
        Some(&claude_json),
        &settings,
        &mcp_json,
        Path::new("/work/p"),
        &env,
    );

    assert_eq!(
        serde_json::to_value(&servers).expect("json"),
        json!({
            "a": {"command": "user-a"},
            "b": {"command": "project-b", "args": ["--token=secret", "${MISSING}", "80"]},
            "c": {"command": "local-c"}
        })
    );
}

#[test]
fn enable_all_approves_every_unlisted_server() {
    let settings = [json!({"enableAllProjectMcpServers": true, "disabledMcpjsonServers": ["y"]})];
    let mcp_json = [json!({"mcpServers": {"x": {"command": "x"}, "y": {"command": "y"}}})];
    let servers =
        mcp_server_declarations(None, &settings, &mcp_json, Path::new("/work/p"), &env);
    assert_eq!(servers.keys().collect::<Vec<_>>(), vec!["x"]);
}
