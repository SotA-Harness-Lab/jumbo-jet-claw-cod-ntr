// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::pin::Pin;

/// Future returned by [`FileSource`] methods.
pub type SourceFuture<'a, T> = Pin<Box<dyn Future<Output = io::Result<T>> + Send + 'a>>;

/// The filesystem as the loaders need it. Symlinks are followed, as Claude Code does.
pub trait FileSource: Send + Sync {
    /// Reads a file as text, or returns `None` when it is missing or not a regular file.
    fn read<'a>(&'a self, path: &'a Path) -> SourceFuture<'a, Option<String>>;

    /// Resolves symlinks when the filesystem supports it.
    fn canonicalize<'a>(&'a self, _path: &'a Path) -> SourceFuture<'a, Option<PathBuf>> {
        Box::pin(async { Ok(None) })
    }

    /// Lists a directory as `(path, is_directory)` pairs, empty when it is missing.
    fn list<'a>(&'a self, dir: &'a Path) -> SourceFuture<'a, Vec<(PathBuf, bool)>>;
}

/// The local filesystem.
#[derive(Clone, Copy, Debug, Default)]
pub struct LocalFiles;

impl FileSource for LocalFiles {
    fn canonicalize<'a>(&'a self, path: &'a Path) -> SourceFuture<'a, Option<PathBuf>> {
        Box::pin(async move { Ok(std::fs::canonicalize(path).ok()) })
    }

    fn read<'a>(&'a self, path: &'a Path) -> SourceFuture<'a, Option<String>> {
        Box::pin(async move {
            match std::fs::metadata(path) {
                Ok(metadata) if metadata.is_file() => {}
                Ok(_) => return Ok(None),
                Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(err) => return Err(err),
            }
            match std::fs::read(path) {
                Ok(data) => Ok(Some(String::from_utf8_lossy(&data).into_owned())),
                Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
                Err(err) => Err(err),
            }
        })
    }

    fn list<'a>(&'a self, dir: &'a Path) -> SourceFuture<'a, Vec<(PathBuf, bool)>> {
        Box::pin(async move {
            let entries = match std::fs::read_dir(dir) {
                Ok(entries) => entries,
                Err(err)
                    if matches!(
                        err.kind(),
                        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                    ) =>
                {
                    return Ok(Vec::new());
                }
                Err(err) => return Err(err),
            };
            let mut listed = Vec::new();
            for entry in entries {
                let path = entry?.path();
                // `metadata` follows symlinks; a dangling link is skipped.
                if let Ok(metadata) = std::fs::metadata(&path) {
                    listed.push((path, metadata.is_dir()));
                }
            }
            Ok(listed)
        })
    }
}
