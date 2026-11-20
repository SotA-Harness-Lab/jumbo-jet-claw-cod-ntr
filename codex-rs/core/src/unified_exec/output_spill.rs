// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Keeps the output of a command that is too long for the model to be shown whole.
//!
//! The model sees the head and the tail of such an output and nothing of the middle, and
//! had to run the command again to see it. The output as collected is written to a file
//! instead, and the last line of what the model reads says where it is, so that it can
//! read or search the part it needs.

use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;

use codex_protocol::ThreadId;

use crate::tools::context::ExecCommandToolOutput;

/// The directory in the system's temporary directory that holds one directory a session.
const SPILL_DIR: &str = "ccn-tool-output";

/// A session's directory is removed once nothing in it was written for this long.
const STALE_AFTER: Duration = Duration::from_secs(7 * 24 * 60 * 60);

/// Writes the collected output to a file when the model will be shown less than all of it,
/// and appends the file's place to the output, where truncation keeps it: at the end.
///
/// A failure to write leaves the output as it was: the model then gets what it always got.
pub(crate) async fn spill_long_output(response: &mut ExecCommandToolOutput, thread_id: ThreadId) {
    let text = &response.raw_output.text;
    if text.len() <= response.model_visible_bytes() {
        return;
    }
    let dir = std::env::temp_dir()
        .join(SPILL_DIR)
        .join(thread_id.to_string());
    let name = if response.chunk_id.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        response.chunk_id.clone()
    };
    let path = dir.join(format!("{name}.txt"));
    let contents = text.clone();
    let written = tokio::task::spawn_blocking({
        let path = path.clone();
        move || write_private(&dir, &path, contents.as_bytes())
    })
    .await;
    match written {
        Ok(Ok(())) => {
            response
                .raw_output
                .text
                .push_str(&footer(&path, response.raw_output.omitted_bytes));
        }
        Ok(Err(err)) => {
            tracing::warn!("could not keep a long command output in {}: {err}", path.display());
        }
        Err(err) => tracing::warn!("could not keep a long command output: {err}"),
    }
}

/// The line that tells the model where the output is.
fn footer(path: &Path, omitted_bytes: usize) -> String {
    let kept = if omitted_bytes == 0 {
        "Full output saved to"
    } else {
        // Collection itself keeps only the head and the tail of a very large output.
        "Output as collected (its own middle was not kept) saved to"
    };
    format!(
        "\n[{kept}: {} - read or search that file for the part you need instead of running the command again]",
        path.display()
    )
}

/// Writes `contents` to `path` in `dir`, both for the user alone, after clearing out the
/// directories of sessions long gone.
fn write_private(dir: &Path, path: &Path, contents: &[u8]) -> std::io::Result<()> {
    if let Some(root) = dir.parent() {
        create_private_dir(root)?;
        remove_stale_dirs(root, dir);
    }
    create_private_dir(dir)?;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    std::io::Write::write_all(&mut options.open(path)?, contents)
}

fn create_private_dir(dir: &Path) -> std::io::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(dir)
}

/// Removes the directories under `root`, other than `keep`, that nothing was written to for
/// [`STALE_AFTER`]. Errors are left alone: this is housekeeping.
fn remove_stale_dirs(root: &Path, keep: &Path) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    let now = SystemTime::now();
    let stale = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path != keep)
        .filter(|path| {
            std::fs::metadata(path)
                .and_then(|metadata| metadata.modified())
                .ok()
                .and_then(|modified| now.duration_since(modified).ok())
                .is_some_and(|age| age > STALE_AFTER)
        })
        .collect::<Vec<PathBuf>>();
    for path in stale {
        let _ = std::fs::remove_dir_all(path);
    }
}

#[cfg(test)]
#[path = "output_spill_tests.rs"]
mod tests;
