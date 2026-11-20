// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! What files held before the agent changed them, kept so that rewinding a conversation
//! can put them back.
//!
//! One checkpoint a turn, in `$CODEX_HOME/file-history/<thread>/<turn>.json`: for every file
//! an edit of that turn touched, its text before the first of them, or nothing when the
//! turn created it. To go back to before a turn, each file is given what the earliest
//! checkpoint from that turn on holds for it. Only edits made with the editing tools are
//! known here; what a shell command changed is not.

use std::collections::BTreeMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

const DIR: &str = "file-history";

/// A file larger than this is not kept; its turn is then marked as not fully restorable.
pub const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;

/// What one turn's edits replaced.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TurnCheckpoint {
    /// Each file the turn's edits touched, by its absolute path, with its text before the
    /// first of them; `None` for a file that did not exist.
    #[serde(default)]
    pub files: BTreeMap<PathBuf, Option<String>>,
    /// Whether an edit of the turn could not be recorded in full (a file that is not text,
    /// one that is too large, a write that failed half way).
    #[serde(default)]
    pub incomplete: bool,
}

/// What rewinding to before a turn would do to the files.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RestorePlan {
    /// The files that differ from what they held then, with what to put back; `None`
    /// removes a file created since.
    pub files: Vec<(PathBuf, Option<String>)>,
    /// Whether some edit since then is not covered, so that the files may not end up
    /// exactly as they were.
    pub incomplete: bool,
}

fn thread_dir(codex_home: &Path, thread_id: &str) -> PathBuf {
    codex_home.join(DIR).join(safe_name(thread_id))
}

fn checkpoint_path(codex_home: &Path, thread_id: &str, turn_id: &str) -> PathBuf {
    thread_dir(codex_home, thread_id).join(format!("{}.json", safe_name(turn_id)))
}

/// An id as a file name: ids are UUIDs, and anything else in one is replaced.
fn safe_name(id: &str) -> String {
    id.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn read_checkpoint(path: &Path) -> io::Result<Option<TurnCheckpoint>> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

/// Adds to the checkpoint of `turn_id` what an edit replaced: `before` gives, for each file
/// the edit touched, its text before the edit. A file the turn has touched before keeps
/// what was recorded then.
pub fn record(
    codex_home: &Path,
    thread_id: &str,
    turn_id: &str,
    before: impl IntoIterator<Item = (PathBuf, Option<String>)>,
    incomplete: bool,
) -> io::Result<()> {
    let path = checkpoint_path(codex_home, thread_id, turn_id);
    let mut checkpoint = read_checkpoint(&path)?.unwrap_or_default();
    checkpoint.incomplete |= incomplete;
    for (file, text) in before {
        if text.as_ref().is_some_and(|text| text.len() > MAX_FILE_BYTES) {
            checkpoint.incomplete = true;
            continue;
        }
        checkpoint.files.entry(file).or_insert(text);
    }
    if let Some(dir) = path.parent() {
        create_private_dir(dir)?;
    }
    // Written beside and moved into place, so that a reader never finds half a checkpoint.
    let temporary = path.with_extension("json.tmp");
    write_private(
        &temporary,
        &serde_json::to_vec(&checkpoint).map_err(io::Error::other)?,
    )?;
    std::fs::rename(&temporary, &path)
}

/// What going back to before the first of `turn_ids` would do: `turn_ids` are that turn and
/// every turn after it, oldest first. A file that already holds what it held then is left
/// out.
pub fn plan_restore(codex_home: &Path, thread_id: &str, turn_ids: &[String]) -> RestorePlan {
    let mut earliest = BTreeMap::new();
    let mut incomplete = false;
    for turn_id in turn_ids {
        match read_checkpoint(&checkpoint_path(codex_home, thread_id, turn_id)) {
            Ok(Some(checkpoint)) => {
                incomplete |= checkpoint.incomplete;
                for (file, text) in checkpoint.files {
                    earliest.entry(file).or_insert(text);
                }
            }
            Ok(None) => {}
            Err(_) => incomplete = true,
        }
    }
    let files = earliest
        .into_iter()
        .filter(|(file, before)| match std::fs::read(file) {
            Ok(now) => before.as_ref().is_none_or(|before| before.as_bytes() != now),
            Err(err) if err.kind() == io::ErrorKind::NotFound => before.is_some(),
            // Unreadable now: putting it back is still what was asked for.
            Err(_) => true,
        })
        .collect();
    RestorePlan { files, incomplete }
}

/// Puts the files of `plan` back and returns those that could not be, with why.
pub fn restore(plan: &RestorePlan) -> Vec<(PathBuf, io::Error)> {
    plan.files
        .iter()
        .filter_map(|(file, before)| {
            let result = match before {
                Some(text) => file
                    .parent()
                    .map_or(Ok(()), std::fs::create_dir_all)
                    .and_then(|()| std::fs::write(file, text)),
                None => match std::fs::remove_file(file) {
                    Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
                    result => result,
                },
            };
            result.err().map(|err| (file.clone(), err))
        })
        .collect()
}

fn create_private_dir(dir: &Path) -> io::Result<()> {
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(dir)
}

fn write_private(path: &Path, contents: &[u8]) -> io::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    io::Write::write_all(&mut options.open(path)?, contents)
}

#[cfg(test)]
#[path = "file_history_tests.rs"]
mod tests;
