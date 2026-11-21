// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::super::bash::BashInput;
use super::super::edit::EditInput;
use super::super::read::ReadInput;
use super::super::write::WriteInput;

#[test]
fn read_takes_numbers_sent_as_strings() {
    assert_eq!(
        ReadInput::parse(json!({ "file_path": "/a", "offset": " +120 ", "limit": "50" })),
        Ok(ReadInput {
            file_path: "/a".to_string(),
            offset: Some(120),
            limit: Some(50),
            pages: None,
        })
    );
}

#[test]
fn read_takes_path_for_file_path_and_ignores_null_and_unknown_parameters() {
    assert_eq!(
        ReadInput::parse(json!({ "path": "/a", "offset": null, "description": "x" })),
        Ok(ReadInput {
            file_path: "/a".to_string(),
            offset: None,
            limit: None,
            pages: None,
        })
    );
}

#[test]
fn every_bad_parameter_is_listed() {
    assert_eq!(
        ReadInput::parse(json!({ "offset": "ten", "limit": -1 })),
        Err("InputValidationError: Read failed due to the following issues:\nThe required parameter `file_path` is missing\nThe parameter `offset` type is expected as `number` but provided as `string`\nThe parameter `limit` is expected as a number not below 0".to_string())
    );
}

#[test]
fn edit_takes_a_boolean_sent_as_a_string() {
    assert_eq!(
        EditInput::parse(json!({
            "file_path": "/a",
            "old_string": "x",
            "new_string": "y",
            "replace_all": "true",
        })),
        Ok(EditInput {
            file_path: "/a".to_string(),
            old_string: "x".to_string(),
            new_string: "y".to_string(),
            replace_all: Some(true),
        })
    );
}

#[test]
fn write_takes_the_names_models_send_in_place_of_its_own() {
    let expected = Ok(WriteInput {
        file_path: "/a".to_string(),
        content: "text".to_string(),
    });
    assert_eq!(
        WriteInput::parse(json!({ "path": "/a", "file_text": "text" })),
        expected
    );
    assert_eq!(
        WriteInput::parse(json!({ "file_path": "/a", "file_content": "text" })),
        expected
    );
    assert_eq!(
        WriteInput::parse(json!({ "file_path": "/a", "content": "text", "path": "/b" })),
        expected
    );
}

#[test]
fn bash_takes_strings_for_numbers_and_booleans_and_command_description() {
    assert_eq!(
        BashInput::parse(json!({
            "command": "ls",
            "timeout": "60000",
            "command_description": "List",
            "run_in_background": "false",
            "dangerouslyDisableSandbox": "true",
        })),
        Ok(BashInput {
            command: "ls".to_string(),
            timeout: Some(60000.0),
            description: Some("List".to_string()),
            run_in_background: Some(false),
            dangerously_disable_sandbox: Some(true),
        })
    );
}
