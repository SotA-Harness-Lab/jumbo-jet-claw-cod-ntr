// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Which files the session has read, for Claude Code's rule that an existing file is read
//! before it is edited or overwritten, and is not changed by someone else in between.
//! Codex's own tools have no such rule, so the middleware keeps it.

use std::collections::HashMap;
use std::sync::Mutex;

use codex_claude_compat::tools::read;
use codex_exec_server::FileMetadata;
use sha1::Digest;
use sha1::Sha1;

use crate::session::session::Session;

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
