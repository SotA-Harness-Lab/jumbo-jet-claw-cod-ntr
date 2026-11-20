// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `Read` of a PDF: its pages as images from poppler's `pdftoppm` (100 dpi JPEG, as Claude
//! Code renders them) and their text from `pdftotext`. The file is read through the
//! executor filesystem under the turn's sandbox, as any other read; poppler then works on
//! a copy in a temporary directory.

use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::Output;
use std::process::Stdio;
use std::time::Duration;

use codex_claude_compat::tools::read;
use codex_protocol::models::FunctionCallOutputContentItem;
use serde_json::json;

use super::super::failed;
use super::super::files::FileTarget;
use super::super::output::ClaudeToolOutput;
use super::super::rejected;
use super::image_item;
use super::model_takes_images;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::boxed_tool_output;

const PDFINFO_TIMEOUT: Duration = Duration::from_secs(10);
const RENDER_TIMEOUT: Duration = Duration::from_secs(120);

pub(super) async fn read(
    invocation: &ToolInvocation,
    target: &FileTarget<'_>,
    file_path: &str,
    pages: Option<(u32, Option<u32>)>,
) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
    let Some(metadata) = target.metadata().await? else {
        return Err(failed(codex_claude_compat::tools::file_not_found_message(
            &target.cwd_display(),
        )));
    };
    if metadata.is_directory {
        return Err(rejected(&read::directory_message(file_path)));
    }
    if metadata.size == 0 {
        return Err(failed(read::pdf_empty_message(file_path)));
    }
    if metadata.size > read::PDF_MAX_EXTRACT_SIZE {
        return Err(failed(read::pdf_too_large_message()));
    }
    let bytes = target.read_bytes().await?;
    if !bytes.starts_with(b"%PDF-") {
        return Err(failed(read::not_a_pdf_message(file_path)));
    }
    let directory = tempfile::Builder::new()
        .prefix("ccn-pdf-")
        .tempdir()
        .map_err(|err| failed(format!("unable to render {file_path}: {err}")))?;
    let input = directory.path().join("input.pdf");
    tokio::fs::write(&input, &bytes)
        .await
        .map_err(|err| failed(format!("unable to render {file_path}: {err}")))?;

    let (first, last) = match pages {
        Some(range) => range,
        None => match page_count(&input).await {
            Some(count) if count > read::PDF_INLINE_MAX_PAGES => {
                return Err(failed(read::too_many_pages_message(count)));
            }
            Some(_) => (1, None),
            // Without pdfinfo the length is unknown: no more than one read's worth of pages.
            None => (1, Some(read::PDF_MAX_PAGES_PER_READ)),
        },
    };

    let images = model_takes_images(invocation);
    let rendered = if images {
        Some(render_pages(directory.path(), &input, first, last).await?)
    } else {
        None
    };
    let texts = page_texts(&input, first, last).await;
    if !images && texts.is_none() {
        return Err(failed(read::PDFTOTEXT_MISSING_MESSAGE.to_string()));
    }

    // Pages by number: the images when the model takes them, the text otherwise.
    let numbers: Vec<u32> = match (&rendered, &texts) {
        (Some(rendered), _) => rendered.iter().map(|(number, _)| *number).collect(),
        (None, Some(texts)) => (0..texts.len())
            .filter_map(|index| u32::try_from(index).ok().map(|index| first + index))
            .collect(),
        (None, None) => Vec::new(),
    };
    let header = read::pdf_pages_message(numbers.len(), file_path, metadata.size);
    let mut log_text = header.clone();
    let mut content = vec![FunctionCallOutputContentItem::InputText { text: header }];
    for (position, number) in numbers.iter().enumerate() {
        let text = texts.as_ref().and_then(|texts| {
            let index = usize::try_from(number - first).ok()?;
            texts.get(index).map(|text| text.trim_end())
        });
        let label = match text {
            Some(text) if !text.trim().is_empty() => format!("\nPage {number}:\n{text}"),
            _ => format!("\nPage {number}:"),
        };
        log_text.push_str(&label);
        content.push(FunctionCallOutputContentItem::InputText { text: label });
        if let Some((_, jpeg)) = rendered.as_ref().and_then(|rendered| rendered.get(position)) {
            content.push(image_item("image/jpeg", jpeg));
        }
    }
    let tool_response = json!({
        "type": "parts",
        "file": {
            "filePath": file_path,
            "originalSize": metadata.size,
            "count": numbers.len(),
        }
    });
    Ok(boxed_tool_output(ClaudeToolOutput::with_content(
        log_text,
        content,
        tool_response,
    )))
}

async fn run(program: &str, args: Vec<OsString>, timeout: Duration) -> io::Result<Output> {
    let child = tokio::process::Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;
    tokio::time::timeout(timeout, child.wait_with_output())
        .await
        .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, format!("{program} timed out")))?
}

fn page_args(first: u32, last: Option<u32>) -> Vec<OsString> {
    let mut args = vec!["-f".into(), first.to_string().into()];
    if let Some(last) = last {
        args.extend(["-l".into(), last.to_string().into()]);
    }
    args
}

/// The PDF's page count from `pdfinfo`, if it is installed and can tell.
async fn page_count(input: &Path) -> Option<u32> {
    let output = run("pdfinfo", vec![input.into()], PDFINFO_TIMEOUT)
        .await
        .ok()?;
    output
        .status
        .success()
        .then(|| read::pdfinfo_page_count(&String::from_utf8_lossy(&output.stdout)))
        .flatten()
}

/// The selected pages as base64 JPEG images, by page number.
async fn render_pages(
    directory: &Path,
    input: &Path,
    first: u32,
    last: Option<u32>,
) -> Result<Vec<(u32, String)>, FunctionCallError> {
    let mut args: Vec<OsString> = vec!["-jpeg".into(), "-r".into(), "100".into()];
    args.extend(page_args(first, last));
    args.extend([input.into(), directory.join("page").into()]);
    let output = match run("pdftoppm", args, RENDER_TIMEOUT).await {
        Ok(output) => output,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            return Err(failed(read::PDFTOPPM_MISSING_MESSAGE.to_string()));
        }
        Err(err) => return Err(failed(format!("pdftoppm failed: {err}"))),
    };
    if !output.status.success() {
        return Err(failed(read::pdftoppm_failed_message(
            String::from_utf8_lossy(&output.stderr).trim(),
        )));
    }
    let mut pages = Vec::new();
    let mut entries = tokio::fs::read_dir(directory)
        .await
        .map_err(|err| failed(format!("pdftoppm failed: {err}")))?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let Some(number) = entry
            .file_name()
            .to_str()
            .and_then(read::rendered_page_number)
        else {
            continue;
        };
        let jpeg = tokio::fs::read(entry.path())
            .await
            .map_err(|err| failed(format!("pdftoppm failed: {err}")))?;
        pages.push((number, jpeg));
    }
    if pages.is_empty() {
        return Err(failed(read::PDF_NO_PAGES_MESSAGE.to_string()));
    }
    pages.sort_by_key(|(number, _)| *number);
    Ok(pages
        .into_iter()
        .map(|(number, jpeg)| {
            use base64::Engine as _;
            (
                number,
                base64::engine::general_purpose::STANDARD.encode(jpeg),
            )
        })
        .collect())
}

/// The text of each selected page, from `pdftotext`; `None` when it is not installed or
/// fails.
async fn page_texts(input: &Path, first: u32, last: Option<u32>) -> Option<Vec<String>> {
    let mut args: Vec<OsString> = vec!["-layout".into(), "-enc".into(), "UTF-8".into()];
    args.extend(page_args(first, last));
    args.extend([input.into(), "-".into()]);
    let output = run("pdftotext", args, RENDER_TIMEOUT).await.ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    // Each page ends with a form feed.
    let mut pages: Vec<String> = text.split('\u{c}').map(str::to_string).collect();
    if text.ends_with('\u{c}') {
        pages.pop();
    }
    Some(pages)
}
