// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use codex_claude_compat::tools::edit::apply_edit;
use codex_claude_compat::tools::patch::add_file_patch;
use codex_claude_compat::tools::patch::update_file_patch;
use codex_exec_server::LOCAL_FS;
use codex_tools::ToolName;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use serde_json::json;

use super::ToolSwap;
use super::bash::exec_command_arguments_for_test;
use super::edit;
use super::read;
use super::write;
use super::written_file_path;
use crate::tools::handlers::ApplyPatchHandler;
use crate::tools::handlers::ViewImageHandler;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolRegistry;

#[test]
fn bash_calls_become_one_shot_or_background_exec_commands() {
    let max_tokens = crate::unified_exec::resolve_max_tokens(None);
    assert_eq!(
        exec_command_arguments_for_test(r#"{"command":"ls","description":"List"}"#, false),
        json!({ "cmd": "ls", "max_output_tokens": max_tokens, "timeout_ms": 120_000 })
    );
    assert_eq!(
        exec_command_arguments_for_test(r#"{"command":"ls","timeout":9000000}"#, false),
        json!({ "cmd": "ls", "max_output_tokens": max_tokens, "timeout_ms": 600_000 })
    );
    assert_eq!(
        exec_command_arguments_for_test(r#"{"command":"make","run_in_background":true}"#, true),
        json!({ "cmd": "make", "max_output_tokens": max_tokens, "yield_time_ms": 1_000 })
    );
    assert_eq!(
        exec_command_arguments_for_test(
            r#"{"command":"curl x","description":"Fetch x","dangerouslyDisableSandbox":true}"#,
            false
        ),
        json!({
            "cmd": "curl x",
            "max_output_tokens": max_tokens,
            "timeout_ms": 120_000,
            "sandbox_permissions": "require_escalated",
            "justification": "Fetch x",
        })
    );
}

#[test]
fn file_tools_replace_apply_patch_and_view_image() {
    let mut registry = ToolRegistry::from_tools([
        Arc::new(ApplyPatchHandler::default()) as Arc<dyn CoreToolRuntime>,
        Arc::new(ViewImageHandler::default()) as Arc<dyn CoreToolRuntime>,
    ]);
    let mut swap = ToolSwap::new(&mut registry);
    read::install(&mut swap);
    write::install(&mut swap);
    edit::install(&mut swap);
    let names = registry
        .tool_names_for_test()
        .into_iter()
        .map(|name| name.name)
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["Edit", "Read", "Write"]);
    assert!(registry.tool(&ToolName::plain("apply_patch")).is_none());
}

#[tokio::test]
async fn edits_reach_the_file_through_apply_patch() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("notes.txt");
    let original = "first\r\n*** End Patch\r\nsecond\r\nthird\r\n";
    std::fs::write(&path, original).expect("write file");
    let cwd = PathUri::from_host_native_path(dir.path()).expect("cwd");
    let native = path.to_string_lossy().to_string();

    let normalized = original.replace("\r\n", "\n");
    let updated = apply_edit(Some(normalized.as_str()), "second\n", "2nd\nadded\n", false)
        .expect("edit applies");
    let patch = update_file_patch(&native, original, &updated).expect("a change");
    codex_apply_patch::apply_patch(
        &patch,
        &cwd,
        &mut Vec::new(),
        &mut Vec::new(),
        LOCAL_FS.as_ref(),
        /*sandbox*/ None,
    )
    .await
    .expect("patch applies");
    assert_eq!(
        std::fs::read_to_string(&path).expect("read file"),
        "first\r\n*** End Patch\r\n2nd\r\nadded\r\nthird\r\n"
    );

    let created = dir.path().join("sub/new.txt");
    codex_apply_patch::apply_patch(
        &add_file_patch(&created.to_string_lossy(), "a\nb"),
        &cwd,
        &mut Vec::new(),
        &mut Vec::new(),
        LOCAL_FS.as_ref(),
        /*sandbox*/ None,
    )
    .await
    .expect("patch applies");
    assert_eq!(
        std::fs::read_to_string(&created).expect("read file"),
        "a\nb\n"
    );
}

#[test]
fn the_file_a_write_or_edit_wrote_is_found_for_its_post_tool_use_hooks() {
    assert_eq!(
        written_file_path("Write", r#"{"file_path":"/a.rs","content":"x"}"#),
        Some("/a.rs".to_string())
    );
    assert_eq!(
        written_file_path(
            "Edit",
            r#"{"file_path":"/b.rs","old_string":"x","new_string":"y"}"#
        ),
        Some("/b.rs".to_string())
    );
    assert_eq!(written_file_path("Read", r#"{"file_path":"/a.rs"}"#), None);
}
