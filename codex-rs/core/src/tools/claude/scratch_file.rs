// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Files a command writes for a Claude tool to read back (the body `WebFetch` fetches, the
//! results of a large `Grep`), in the executor's temporary directory, since a command's
//! output reaches the tool cut to its first and last 512 KiB.
//!
//! The temporary directory may be shared with other users (`/tmp`), and a file's name,
//! once seen in a command line, is known to them. So a command never writes to a path there
//! that someone else could have put a file or a link at first: each file is in a directory
//! of the tool's own, created anew for each command without following links, and the
//! creation fails when anything already stands at its path.

use std::sync::LazyLock;

use codex_exec_server::CreateDirectoryOptions;
use codex_exec_server::ExecutorFileSystem;
use codex_exec_server::ReadFileOptions;
use codex_exec_server::RemoveOptions;
use codex_utils_path_uri::PathUri;

use crate::session::turn_context::TurnEnvironment;

/// A namespace drawn once per process, so that the names cannot be guessed beforehand.
static NAMESPACE: LazyLock<uuid::Uuid> = LazyLock::new(uuid::Uuid::new_v4);

/// The name of the file inside its directory.
const FILE_NAME: &str = "output";

/// The file for one command, in a directory `<kind>-<id>` created for it in the executor's
/// temporary directory (`/tmp` where the executor names none and `posix` says its shell is
/// a POSIX one), or `None` when there is none or the directory cannot be created. The id
/// depends on `key` alone within a process, so that a command for the same `key` stays the
/// same and an approval given for it still applies to the next one.
pub(super) async fn create(
    environment: &TurnEnvironment,
    posix: bool,
    kind: &str,
    key: &[u8],
) -> Option<PathUri> {
    let directory = match environment
        .temporary_directories
        .as_ref()
        .and_then(|directories| directories.first())
    {
        Some(directory) => directory.clone(),
        None if posix => PathUri::from_host_native_path("/tmp").ok()?,
        None => return None,
    };
    let fs = environment.environment().get_filesystem();
    create_in(fs.as_ref(), &directory, kind, key).await
}

/// Reads the file a command wrote and removes it with its directory. They are the tool's
/// own, so this is not sandboxed; links are not followed.
pub(super) async fn take(
    environment: &TurnEnvironment,
    file: &PathUri,
) -> std::io::Result<Vec<u8>> {
    let fs = environment.environment().get_filesystem();
    let contents = fs
        .read_file(
            file,
            ReadFileOptions {
                follow_symlinks: false,
            },
            /*sandbox*/ None,
        )
        .await;
    if let Some(directory) = file.parent() {
        remove_directory(fs.as_ref(), &directory).await;
    }
    contents
}

/// [`create`] in `temporary_directory`.
async fn create_in(
    fs: &dyn ExecutorFileSystem,
    temporary_directory: &PathUri,
    kind: &str,
    key: &[u8],
) -> Option<PathUri> {
    // The links in the temporary directory's own path (`/tmp` to `/private/tmp` on macOS)
    // are the system's; the operations below follow none.
    let directory = fs
        .canonicalize(temporary_directory, /*sandbox*/ None)
        .await
        .ok()?;
    let own = directory
        .join(&format!("{kind}-{}", uuid::Uuid::new_v5(&NAMESPACE, key)))
        .ok()?;
    // What an earlier command left behind (a crash before the cleanup) is removed; what
    // someone else put there stays and makes the creation fail.
    remove_directory(fs, &own).await;
    fs.create_directory(
        &own,
        CreateDirectoryOptions {
            recursive: false,
            follow_symlinks: false,
        },
        /*sandbox*/ None,
    )
    .await
    .ok()?;
    own.join(FILE_NAME).ok()
}

/// Removes a directory of [`create`] and the file in it, following no link; anything else
/// in it keeps it in place.
async fn remove_directory(fs: &dyn ExecutorFileSystem, directory: &PathUri) {
    let options = RemoveOptions {
        recursive: false,
        force: true,
        follow_symlinks: false,
    };
    if let Ok(file) = directory.join(FILE_NAME) {
        let _ = fs.remove(&file, options, /*sandbox*/ None).await;
    }
    let _ = fs.remove(directory, options, /*sandbox*/ None).await;
}

#[cfg(test)]
#[path = "scratch_file_tests.rs"]
mod tests;
