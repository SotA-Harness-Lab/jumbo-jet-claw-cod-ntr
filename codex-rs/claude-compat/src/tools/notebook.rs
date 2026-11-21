// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! How Claude Code's `Read` shows a Jupyter notebook: each cell as
//! `<cell id="...">...</cell id="...">` with its outputs after it, images from the outputs
//! as images, and a cell whose outputs are too large replaced by a pointer to `jq`.

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::BASH;

/// Outputs of one cell larger than this (characters of text and of image data) are left
/// out of a read of the whole notebook (Claude Code's `LARGE_OUTPUT_THRESHOLD`).
const LARGE_OUTPUT_THRESHOLD: usize = 10_000;

#[derive(Deserialize)]
struct Notebook {
    #[serde(default)]
    metadata: NotebookMetadata,
    #[serde(default)]
    cells: Vec<Cell>,
}

#[derive(Deserialize, Default)]
struct NotebookMetadata {
    #[serde(default)]
    language_info: Option<LanguageInfo>,
}

#[derive(Deserialize)]
struct LanguageInfo {
    #[serde(default)]
    name: Option<String>,
}

#[derive(Deserialize)]
struct Cell {
    #[serde(default)]
    id: Option<String>,
    cell_type: String,
    #[serde(default)]
    source: Value,
    #[serde(default)]
    execution_count: Option<i64>,
    #[serde(default)]
    outputs: Vec<Value>,
}

/// A cell as Claude Code hands it to hooks (`tool_response.file.cells`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NotebookCell {
    #[serde(rename = "cellType")]
    pub cell_type: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_count: Option<i64>,
    pub cell_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<CellOutput>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CellOutput {
    pub output_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<OutputImage>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OutputImage {
    /// Base64 data, without whitespace.
    pub image_data: String,
    pub media_type: String,
}

/// A piece of what the model receives for a notebook.
#[derive(Debug, Clone, PartialEq)]
pub enum NotebookPart {
    Text(String),
    Image(OutputImage),
}

/// The cells of a notebook, processed as Claude Code reads a whole notebook. Fails when the
/// file is not a notebook's JSON; a byte order mark before it is skipped.
pub fn read_cells(json: &str) -> Result<Vec<NotebookCell>, String> {
    let json = json.strip_prefix('\u{feff}').unwrap_or(json);
    let notebook: Notebook = serde_json::from_str(json).map_err(|err| err.to_string())?;
    let language = notebook
        .metadata
        .language_info
        .and_then(|info| info.name)
        .unwrap_or_else(|| "python".to_string());
    Ok(notebook
        .cells
        .into_iter()
        .enumerate()
        .map(|(index, cell)| process_cell(cell, index, &language))
        .collect())
}

fn joined(text: &Value) -> String {
    match text {
        Value::String(text) => text.clone(),
        Value::Array(parts) => parts.iter().filter_map(Value::as_str).collect(),
        _ => String::new(),
    }
}

fn process_cell(cell: Cell, index: usize, language: &str) -> NotebookCell {
    let is_code = cell.cell_type == "code";
    let mut processed = NotebookCell {
        source: joined(&cell.source),
        execution_count: if is_code {
            cell.execution_count.filter(|count| *count != 0)
        } else {
            None
        },
        cell_id: cell.id.unwrap_or_else(|| format!("cell-{index}")),
        language: is_code.then(|| language.to_string()),
        outputs: None,
        cell_type: cell.cell_type,
    };
    if is_code && !cell.outputs.is_empty() {
        let outputs: Vec<CellOutput> = cell.outputs.iter().filter_map(process_output).collect();
        let size: usize = outputs
            .iter()
            .map(|output| {
                output.text.len()
                    + output
                        .image
                        .as_ref()
                        .map_or(0, |image| image.image_data.len())
            })
            .sum();
        processed.outputs = Some(if size > LARGE_OUTPUT_THRESHOLD {
            vec![CellOutput {
                output_type: "stream".to_string(),
                text: format!(
                    "Outputs are too large to include. Use {BASH} with: cat <notebook_path> | jq '.cells[{index}].outputs'"
                ),
                image: None,
            }]
        } else {
            outputs
        });
    }
    processed
}

fn process_output(output: &Value) -> Option<CellOutput> {
    let output_type = output.get("output_type")?.as_str()?.to_string();
    let (text, image) = match output_type.as_str() {
        "stream" => (joined(output.get("text").unwrap_or(&Value::Null)), None),
        "execute_result" | "display_data" => {
            let data = output.get("data");
            let text = data
                .and_then(|data| data.get("text/plain"))
                .map(joined)
                .unwrap_or_default();
            (text, data.and_then(output_image))
        }
        "error" => {
            let field = |name| {
                output
                    .get(name)
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string()
            };
            let traceback = output
                .get("traceback")
                .and_then(Value::as_array)
                .map(|lines| {
                    lines
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .unwrap_or_default();
            (
                format!("{}: {}\n{traceback}", field("ename"), field("evalue")),
                None,
            )
        }
        _ => return None,
    };
    Some(CellOutput {
        output_type,
        text,
        image,
    })
}

fn output_image(data: &Value) -> Option<OutputImage> {
    ["image/png", "image/jpeg"]
        .into_iter()
        .find_map(|media_type| {
            let encoded = data.get(media_type)?.as_str()?;
            Some(OutputImage {
                image_data: encoded.chars().filter(|c| !c.is_whitespace()).collect(),
                media_type: media_type.to_string(),
            })
        })
}

/// What the model receives: each cell's text, then its outputs' text and images, with
/// adjacent text joined by a line break.
pub fn render(cells: &[NotebookCell]) -> Vec<NotebookPart> {
    let mut parts: Vec<NotebookPart> = Vec::new();
    let mut push = |part: NotebookPart| match (parts.last_mut(), part) {
        (Some(NotebookPart::Text(previous)), NotebookPart::Text(text)) => {
            previous.push('\n');
            previous.push_str(&text);
        }
        (_, part) => parts.push(part),
    };
    for cell in cells {
        let mut metadata = String::new();
        if cell.cell_type != "code" {
            metadata.push_str(&format!("<cell_type>{}</cell_type>", cell.cell_type));
        }
        if let Some(language) = cell.language.as_deref().filter(|name| *name != "python") {
            metadata.push_str(&format!("<language>{language}</language>"));
        }
        push(NotebookPart::Text(format!(
            "<cell id=\"{id}\">{metadata}{source}</cell id=\"{id}\">",
            id = cell.cell_id,
            source = cell.source
        )));
        for output in cell.outputs.iter().flatten() {
            if !output.text.is_empty() {
                push(NotebookPart::Text(format!("\n{}", output.text)));
            }
            if let Some(image) = &output.image {
                push(NotebookPart::Image(image.clone()));
            }
        }
    }
    parts
}

/// Claude Code's refusal of a notebook too large to read at once.
pub fn too_large_message(size: &str, max_size: &str, file_path: &str) -> String {
    format!(
        "Notebook content ({size}) exceeds maximum allowed size ({max_size}). Use {BASH} with jq to read specific portions:\n  cat \"{file_path}\" | jq '.cells[:20]' # First 20 cells\n  cat \"{file_path}\" | jq '.cells[100:120]' # Cells 100-120\n  cat \"{file_path}\" | jq '.cells | length' # Count total cells\n  cat \"{file_path}\" | jq '.cells[] | select(.cell_type==\"code\") | .source' # All code sources"
    )
}

#[cfg(test)]
#[path = "notebook_tests.rs"]
mod tests;
