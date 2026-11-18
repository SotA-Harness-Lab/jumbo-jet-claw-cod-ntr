//! Which files the session has read, for Claude Code's rule that an existing file is read
//! before it is edited or overwritten, and is not changed by someone else in between.
//! Codex's own tools have no such rule, so the middleware keeps it.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::session::session::Session;

/// What the session knows of a file it read or wrote.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FileRecord {
    /// Modification time when the session last read or wrote the file.
    pub(crate) modified_at_ms: i64,
    /// The whole content (`\r\n` as `\n`) the session wrote, when it wrote the file: a newer
    /// modification time with the same content does not count as a change.
    pub(crate) content: Option<String>,
}

/// Files read or written in a session, by path URI.
#[derive(Default)]
pub(crate) struct ReadFiles(Mutex<HashMap<String, FileRecord>>);

impl ReadFiles {
    pub(crate) fn of(session: &Session) -> std::sync::Arc<Self> {
        session
            .services
            .session_extension_data
            .get_or_init::<ReadFiles>(ReadFiles::default)
    }

    pub(crate) fn get(&self, key: &str) -> Option<FileRecord> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(key)
            .cloned()
    }

    pub(crate) fn insert(&self, key: String, record: FileRecord) {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(key, record);
    }
}

/// Why a write of an existing file is refused, if it is: the session never read it, or the
/// file changed since (a newer modification time, unless the content is what the session
/// itself wrote).
pub(crate) fn write_refusal(
    record: Option<&FileRecord>,
    modified_at_ms: i64,
    current_content: &str,
) -> Option<&'static str> {
    let Some(record) = record else {
        return Some(codex_claude_compat::tools::edit::NOT_READ_MESSAGE);
    };
    if modified_at_ms > record.modified_at_ms && record.content.as_deref() != Some(current_content)
    {
        return Some(codex_claude_compat::tools::edit::MODIFIED_SINCE_READ_MESSAGE);
    }
    None
}
