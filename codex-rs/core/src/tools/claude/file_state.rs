// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Which files the session has read, for Claude Code's rule that an existing file is read
//! before it is edited or overwritten, and is not changed by someone else in between.
//! Codex's own tools have no such rule, so the middleware keeps it.

use std::collections::HashMap;
use std::sync::Mutex;

use codex_claude_compat::tools::EDIT;
use codex_claude_compat::tools::READ;
use codex_claude_compat::tools::WRITE;
use codex_claude_compat::tools::read;
use codex_claude_compat::tools::seen_files::PastCall;
use codex_claude_compat::tools::seen_files::seen_files;
use codex_exec_server::FileMetadata;
use codex_history::ResponseItemEnvelope;
use codex_protocol::models::ResponseItem;
use codex_utils_path_uri::PathUri;
use sha1::Digest;
use sha1::Sha1;

use crate::session::session::Session;
use crate::session::turn_context::TurnContext;

/// How many files the session remembers: the ones it used last, as Claude Code keeps its
/// `READ_FILE_STATE_CACHE_SIZE`. A file it forgot has to be read again before it is changed.
const MAX_RECORDS: usize = 100;

/// A digest of a file's content as `Read` shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContentDigest(sha1::digest::Output<Sha1>);

impl ContentDigest {
    /// The digest of `text` as `Read` shows it: without a byte order mark, with `\r\n` as
    /// `\n` and without the line break that ends the file.
    pub(crate) fn of(text: &str) -> Self {
        Self(Sha1::digest(
            read::slice_text(text, None, None).content.as_bytes(),
        ))
    }
}

/// A file's modification time and size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FileStamp {
    pub(crate) modified_at_ms: i64,
    pub(crate) size: u64,
}

impl FileStamp {
    pub(crate) fn of(metadata: &FileMetadata) -> Self {
        Self {
            modified_at_ms: metadata.modified_at_ms,
            size: metadata.size,
        }
    }
}

/// What the session knows of a file it read or wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FileRecord {
    /// The session saw the whole content: it read the whole file, or wrote it. The file has
    /// changed only if its content has, whatever its modification time says.
    Whole(ContentDigest),
    /// The session read some of its lines. The file has changed if its modification time or
    /// size is not what it was then.
    Part(FileStamp),
}

/// Files read or written in a session, by path URI, the ones used last.
#[derive(Default)]
pub(crate) struct ReadFiles(Mutex<Records>);

#[derive(Default)]
struct Records {
    /// Each record with the moment it was last used.
    by_key: HashMap<String, (FileRecord, u64)>,
    clock: u64,
}

impl Records {
    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }
}

impl ReadFiles {
    pub(crate) fn of(session: &Session) -> std::sync::Arc<Self> {
        session
            .services
            .session_extension_data
            .get_or_init::<ReadFiles>(ReadFiles::default)
    }

    fn records(&self) -> std::sync::MutexGuard<'_, Records> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn get(&self, key: &str) -> Option<FileRecord> {
        let mut records = self.records();
        let now = records.tick();
        let (record, used) = records.by_key.get_mut(key)?;
        *used = now;
        Some(record.clone())
    }

    /// Remembers `record`, forgetting the file used longest ago when there are too many.
    pub(crate) fn insert(&self, key: String, record: FileRecord) {
        let mut records = self.records();
        let now = records.tick();
        records.by_key.insert(key, (record, now));
        if records.by_key.len() > MAX_RECORDS {
            let oldest = records
                .by_key
                .iter()
                .min_by_key(|(_, (_, used))| *used)
                .map(|(key, _)| key.clone());
            if let Some(oldest) = oldest {
                records.by_key.remove(&oldest);
            }
        }
    }
}

/// Rebuilds what the session knows of the files the model read and wrote from `history`, the
/// conversation of a resumed or forked session, as Claude Code rebuilds it from the
/// transcript: each file the model knew whole there is known by that content, so that it may
/// be edited without a new read while its content is unchanged, and a file it last read in
/// part is known by the modification time and size kept with that read.
pub(crate) fn restore<'a>(
    session: &Session,
    turn_context: &TurnContext,
    history: impl IntoIterator<Item = &'a ResponseItemEnvelope>,
) {
    if !super::enabled(turn_context) {
        return;
    }
    let Some(environment) = turn_context.initial_environments.primary() else {
        return;
    };
    restore_into(&ReadFiles::of(session), environment.cwd(), history);
}

/// [`restore`] into `files`, with relative paths resolved against `cwd`.
fn restore_into<'a>(
    files: &ReadFiles,
    cwd: &PathUri,
    history: impl IntoIterator<Item = &'a ResponseItemEnvelope>,
) {
    let mut calls: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut past = Vec::new();
    // The reads of some lines, by their place in `past`, with the file as it was then.
    let mut partial_reads = Vec::new();
    for envelope in history {
        match &envelope.item {
            ResponseItem::FunctionCall {
                name,
                namespace: None,
                arguments,
                call_id,
                ..
            } => {
                calls.insert(call_id.as_str(), (name.as_str(), arguments.as_str()));
            }
            ResponseItem::FunctionCallOutput {
                call_id: Some(call_id),
                output,
                ..
            } => {
                if let (Some(&(tool, arguments)), Some(output)) =
                    (calls.get(call_id.as_str()), output.text_content())
                {
                    if let Some(stamp) = envelope
                        .metadata
                        .as_ref()
                        .and_then(|metadata| metadata.partial_read)
                        .filter(|_| tool == READ)
                    {
                        partial_reads.push((past.len(), stamp));
                    }
                    past.push(PastCall {
                        tool,
                        arguments,
                        output,
                    });
                }
            }
            _ => {}
        }
    }
    let key_of = |file_path: &str| cwd.join(file_path).ok().map(|uri| uri.to_string());
    // The call that used each file last: a read of some lines counts only when it was the last.
    let key_of_call = |call: &PastCall<'_>| {
        if ![READ, WRITE, EDIT].contains(&call.tool) {
            return None;
        }
        let arguments = serde_json::from_str::<serde_json::Value>(call.arguments).ok()?;
        key_of(arguments.get("file_path")?.as_str()?)
    };
    let mut last_use = HashMap::new();
    for (index, call) in past.iter().enumerate() {
        if let Some(key) = key_of_call(call) {
            last_use.insert(key, index);
        }
    }
    let read_in_part = partial_reads
        .into_iter()
        .filter_map(|(index, stamp)| {
            let key = key_of_call(&past[index])?;
            (last_use.get(&key) == Some(&index)).then_some((
                key,
                FileStamp {
                    modified_at_ms: stamp.modified_at_ms,
                    size: stamp.size,
                },
            ))
        })
        .collect::<Vec<_>>();
    for (key, content) in seen_files(past, key_of) {
        files.insert(key, FileRecord::Whole(ContentDigest::of(&content)));
    }
    for (key, stamp) in read_in_part {
        files.insert(key, FileRecord::Part(stamp));
    }
}

/// Why a write of an existing file is refused, if it is: the session never read it, or the
/// file changed since (see [`FileRecord`]). `stamp` and `current_content` are the file's now.
pub(crate) fn write_refusal(
    record: Option<&FileRecord>,
    stamp: FileStamp,
    current_content: &str,
) -> Option<&'static str> {
    let changed = match record {
        None => return Some(codex_claude_compat::tools::edit::NOT_READ_MESSAGE),
        Some(FileRecord::Whole(digest)) => *digest != ContentDigest::of(current_content),
        Some(FileRecord::Part(seen)) => *seen != stamp,
    };
    changed.then_some(codex_claude_compat::tools::edit::MODIFIED_SINCE_READ_MESSAGE)
}

#[cfg(test)]
#[path = "file_state_tests.rs"]
mod tests;
