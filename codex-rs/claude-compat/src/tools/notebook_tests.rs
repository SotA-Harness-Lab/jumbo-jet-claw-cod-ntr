// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;
use serde_json::json;

use super::NotebookPart;
use super::OutputImage;
use super::read_cells;
use super::render;

fn notebook(cells: serde_json::Value, language: Option<&str>) -> String {
    let metadata = match language {
        Some(name) => json!({ "language_info": { "name": name } }),
        None => json!({}),
    };
    json!({ "metadata": metadata, "cells": cells, "nbformat": 4 }).to_string()
}

#[test]
fn cells_are_tagged_with_their_id_type_and_outputs() {
    let json = notebook(
        json!([
            { "cell_type": "markdown", "id": "intro", "source": ["# Title\n", "text"] },
            {
                "cell_type": "code",
                "source": "print(1)\n1 + 1",
                "execution_count": 3,
                "outputs": [
                    { "output_type": "stream", "name": "stdout", "text": ["1\n"] },
                    {
                        "output_type": "execute_result",
                        "data": { "text/plain": "2", "image/png": "iVBO\nRw0K" }
                    }
                ]
            }
        ]),
        None,
    );
    let cells = read_cells(&json).expect("a notebook");
    assert_eq!(cells[1].cell_id, "cell-1");
    assert_eq!(cells[1].execution_count, Some(3));
    assert_eq!(
        render(&cells),
        vec![
            NotebookPart::Text(
                "<cell id=\"intro\"><cell_type>markdown</cell_type># Title\ntext</cell id=\"intro\">\n<cell id=\"cell-1\">print(1)\n1 + 1</cell id=\"cell-1\">\n\n1\n\n\n2"
                    .to_string()
            ),
            NotebookPart::Image(OutputImage {
                image_data: "iVBORw0K".to_string(),
                media_type: "image/png".to_string(),
            }),
        ]
    );
}

#[test]
fn other_languages_are_named_and_errors_show_their_traceback() {
    let json = notebook(
        json!([{
            "cell_type": "code",
            "source": "x",
            "outputs": [{
                "output_type": "error",
                "ename": "NameError",
                "evalue": "x",
                "traceback": ["line 1", "line 2"]
            }]
        }]),
        Some("julia"),
    );
    let cells = read_cells(&json).expect("a notebook");
    assert_eq!(
        render(&cells),
        vec![NotebookPart::Text(
            "<cell id=\"cell-0\"><language>julia</language>x</cell id=\"cell-0\">\n\nNameError: x\nline 1\nline 2"
                .to_string()
        )]
    );
}

#[test]
fn large_outputs_point_to_jq() {
    let json = notebook(
        json!([{
            "cell_type": "code",
            "source": "",
            "outputs": [{ "output_type": "stream", "text": "y".repeat(10_001) }]
        }]),
        None,
    );
    let cells = read_cells(&json).expect("a notebook");
    let outputs = cells[0].outputs.as_ref().expect("outputs");
    assert_eq!(
        outputs[0].text,
        "Outputs are too large to include. Use Bash with: cat <notebook_path> | jq '.cells[0].outputs'"
    );
}

#[test]
fn a_file_that_is_not_a_notebook_fails() {
    assert!(read_cells("not json").is_err());
}

#[test]
fn a_byte_order_mark_before_the_json_is_skipped() {
    let json = format!(
        "\u{feff}{}",
        notebook(
            json!([{ "cell_type": "code", "source": "x", "outputs": [] }]),
            None
        )
    );
    let cells = read_cells(&json).expect("a notebook");
    assert_eq!(cells.len(), 1);
}
