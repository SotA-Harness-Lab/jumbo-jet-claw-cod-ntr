// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

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
    let mcp_json = [json!({"mcpServers": {
        "b": {"command": "project-b", "args": ["--token=${TOKEN}", "${MISSING}", "${PORT:-80}"]},
        "c": {"command": "project-c"},
        "d": {"command": "project-d"}
    }})];

    let servers = mcp_server_declarations(
        Some(&claude_json),
        &settings,
        &mcp_json,
        Path::new("/work/p"),
        /*project_trusted*/ true,
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
    let servers = mcp_server_declarations(
        None,
        &settings,
        &mcp_json,
        Path::new("/work/p"),
        /*project_trusted*/ true,
        &env,
    );
    assert_eq!(servers.keys().collect::<Vec<_>>(), vec!["x"]);
}

#[test]
fn servers_turned_off_in_mcp_are_left_out() {
    let claude_json = json!({
        "mcpServers": {"a": {"command": "user-a"}, "b": {"command": "user-b"}},
        "projects": {"/work/p": {"disabledMcpServers": ["b"]}}
    });

    let servers = mcp_server_declarations(
        Some(&claude_json),
        &[],
        &[],
        Path::new("/work/p"),
        /*project_trusted*/ true,
        &env,
    );

    assert_eq!(servers.keys().collect::<Vec<_>>(), vec!["a"]);
}

#[test]
fn credentials_read_as_empty_in_a_remote_server_s_address() {
    let credentials = |name: &str| match name {
        "OPENAI_API_KEY" => Some("sk-secret".to_string()),
        "TOKEN" => Some("plain".to_string()),
        _ => None,
    };
    let mcp_json = [json!({"mcpServers": {
        "remote": {
            "url": "https://example.com/mcp?key=${OPENAI_API_KEY}&t=${TOKEN}",
            "headers": {"Authorization": "Bearer ${OPENAI_API_KEY:-fallback}"}
        },
        "local": {"command": "srv", "env": {"KEY": "${OPENAI_API_KEY}"}}
    }})];
    let settings = [json!({"enableAllProjectMcpServers": true})];

    let servers = mcp_server_declarations(
        None,
        &settings,
        &mcp_json,
        Path::new("/work/p"),
        /*project_trusted*/ true,
        &credentials,
    );

    assert_eq!(
        servers["remote"],
        json!({
            "url": "https://example.com/mcp?key=&t=plain",
            "headers": {"Authorization": "Bearer "}
        })
    );
    // A local server's own environment is the user's to give.
    assert_eq!(servers["local"]["env"]["KEY"], json!("sk-secret"));
}

#[test]
fn a_project_s_headers_helper_runs_only_in_a_trusted_project() {
    let claude_json = json!({
        "mcpServers": {
            "user": {"url": "https://u.example", "headersHelper": "user-helper ${TOKEN}"},
            "shadowed": {"url": "https://s.example", "headersHelper": "user-helper"}
        },
        "projects": {"/work/p": {"mcpServers": {
            "local": {"url": "https://l.example", "headersHelper": "local-helper"}
        }}}
    });
    let settings = [json!({"enableAllProjectMcpServers": true})];
    let mcp_json = [json!({"mcpServers": {
        "project": {"url": "https://p.example", "headersHelper": "project-helper"},
        "shadowed": {"url": "https://s.example", "headersHelper": "project-helper"}
    }})];
    let declarations = |project_trusted| {
        mcp_server_declarations(
            Some(&claude_json),
            &settings,
            &mcp_json,
            Path::new("/work/p"),
            project_trusted,
            &env,
        )
    };

    let trusted = declarations(true);
    // The shell that runs the helper expands its variables, not the declaration.
    assert_eq!(
        trusted["user"]["headersHelper"],
        json!("user-helper ${TOKEN}")
    );
    assert_eq!(trusted["local"]["headersHelper"], json!("local-helper"));
    assert_eq!(trusted["project"]["headersHelper"], json!("project-helper"));
    assert_eq!(
        trusted["shadowed"]["headersHelper"],
        json!("project-helper")
    );

    let untrusted = declarations(false);
    assert_eq!(
        untrusted["user"]["headersHelper"],
        json!("user-helper ${TOKEN}")
    );
    for name in ["local", "project", "shadowed"] {
        assert_eq!(untrusted[name].get("headersHelper"), None, "{name}");
        assert!(untrusted[name].get("url").is_some(), "{name}");
    }
}

#[test]
fn a_headers_helper_script_sets_claude_code_s_variables() {
    assert_eq!(
        super::headers_helper_script("get-headers", "it's", "https://x.example/mcp", None),
        "export CLAUDE_CODE_MCP_SERVER_NAME='it'\\''s' \
         CLAUDE_CODE_MCP_SERVER_URL='https://x.example/mcp'\nget-headers"
    );
    assert_eq!(
        super::headers_helper_script(
            "./bin/headers",
            "srv",
            "https://x.example",
            Some(Path::new("/plugins/p"))
        ),
        "export CLAUDE_CODE_MCP_SERVER_NAME='srv' CLAUDE_CODE_MCP_SERVER_URL='https://x.example'\n\
         export CLAUDE_PLUGIN_ROOT='/plugins/p'\ncd -- '/plugins/p' || exit 1\n./bin/headers"
    );
}

#[test]
fn the_walk_stops_below_the_root() {
    assert_eq!(
        super::mcp_json_paths(Path::new("/work/p")),
        vec![
            Path::new("/work/.mcp.json").to_path_buf(),
            Path::new("/work/p/.mcp.json").to_path_buf(),
        ]
    );
}

#[test]
fn a_linked_worktree_is_known_by_its_main_checkout() {
    let temp = tempfile::tempdir().expect("tempdir");
    let base = std::fs::canonicalize(temp.path()).expect("canonical tempdir");
    let main = base.join("main");
    let worktree = base.join("wt");
    let forged = base.join("forged");
    let worktree_git_dir = main.join(".git").join("worktrees").join("wt");
    std::fs::create_dir_all(&worktree_git_dir).expect("worktree git dir");
    std::fs::create_dir_all(worktree.join("sub")).expect("worktree");
    std::fs::create_dir_all(&forged).expect("forged");
    std::fs::write(worktree_git_dir.join("commondir"), "../..\n").expect("commondir");
    std::fs::write(
        worktree_git_dir.join("gitdir"),
        format!("{}\n", worktree.join(".git").display()),
    )
    .expect("gitdir");
    let gitfile = format!("gitdir: {}\n", worktree_git_dir.display());
    std::fs::write(worktree.join(".git"), &gitfile).expect("worktree .git");
    // A clone whose `.git` names the worktree's entry, which does not point back to it.
    std::fs::write(forged.join(".git"), &gitfile).expect("forged .git");

    assert_eq!(super::claude_json_project_root(&main), main);
    assert_eq!(super::claude_json_project_root(&worktree.join("sub")), main);
    assert_eq!(super::claude_json_project_root(&forged), forged);
}

#[test]
fn mcp_timeouts_read_milliseconds_as_parse_int_does() {
    use std::time::Duration;

    let env = |name: &str| match name {
        "MCP_TIMEOUT" => Some(" 45000ms".to_string()),
        "MCP_TOOL_TIMEOUT" => Some("600000".to_string()),
        _ => None,
    };
    assert_eq!(
        super::mcp_timeouts(&env),
        super::McpTimeouts {
            startup: Some(Duration::from_secs(45)),
            tool: Some(Duration::from_secs(600)),
        }
    );

    for unset in ["", "0", "abc", "-5"] {
        let env = |_: &str| Some(unset.to_string());
        assert_eq!(super::mcp_timeouts(&env), super::McpTimeouts::default());
    }
    assert_eq!(
        super::mcp_timeouts(&|_| None),
        super::McpTimeouts::default()
    );
}
