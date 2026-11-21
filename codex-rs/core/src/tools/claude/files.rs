// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! File access shared by the Claude file tools. Reading goes through the turn
//! environment's executor filesystem under the turn's sandbox, as Codex's own tools read;
//! writing goes through Codex's `apply_patch` runtime.

use std::io;
use std::sync::Arc;

use codex_claude_compat::tools::read;
use codex_exec_server::ExecutorFileSystem;
use codex_exec_server::FileMetadata;
use codex_exec_server::FileSystemSandboxContext;
use codex_exec_server::GetMetadataOptions;
use codex_exec_server::ReadFileOptions;
use codex_utils_path_uri::PathUri;
use futures::StreamExt;

use super::delegated;
use super::failed;
use super::file_state::ContentDigest;
use super::file_state::FileRecord;
use super::file_state::FileStamp;
use super::file_state::ReadFiles;
use crate::function_tool::FunctionCallError;
use crate::session::turn_context::TurnEnvironment;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::handlers::selected_executor;
use crate::tools::registry::CoreToolRuntime;

/// A file a Claude tool works on, in the environment the call runs in.
pub(crate) struct FileTarget<'a> {
    pub(crate) environment: &'a TurnEnvironment,
    pub(crate) uri: PathUri,
    /// The directory a relative path resolves against.
    cwd: PathUri,
    fs: Arc<dyn ExecutorFileSystem>,
    sandbox: FileSystemSandboxContext,
}

impl<'a> FileTarget<'a> {
    /// Resolves `file_path` (absolute, or relative to the working directory, as Claude Code
    /// expands it: where `Bash`'s `cd` left the session) in the call's environment.
    pub(crate) fn resolve(
        invocation: &'a ToolInvocation,
        file_path: &str,
    ) -> Result<Self, FunctionCallError> {
        let environment = selected_executor(
            &invocation.environments,
            &invocation.step_context.environments,
        )?;
        let cwd = super::shell_cwd::working_directory(invocation, environment);
        let uri = cwd.join(file_path).map_err(|err| {
            failed(format!(
                "unable to resolve `{file_path}` against the working directory `{cwd}`: {err}"
            ))
        })?;
        Ok(Self {
            environment,
            uri,
            cwd,
            fs: environment.environment().get_filesystem(),
            sandbox: environment.sandbox_context(/*additional_permissions*/ None),
        })
    }

    /// Key of the file in the session's [`ReadFiles`].
    pub(crate) fn key(&self) -> String {
        self.uri.to_string()
    }

    /// The path as the executor's operating system writes it.
    pub(crate) fn native_path(&self) -> String {
        self.uri.inferred_native_path_string()
    }

    /// The working directory, for Claude Code's "file does not exist" message.
    pub(crate) fn cwd_display(&self) -> String {
        self.cwd.inferred_native_path_string()
    }

    /// The file's metadata, `None` when it does not exist.
    pub(crate) async fn metadata(&self) -> Result<Option<FileMetadata>, FunctionCallError> {
        match self
            .fs
            .get_metadata(
                &self.uri,
                GetMetadataOptions::default(),
                Some(&self.sandbox),
            )
            .await
        {
            Ok(metadata) => Ok(Some(metadata)),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(failed(format!(
                "unable to access {}: {err}",
                self.native_path()
            ))),
        }
    }

    pub(crate) async fn read_bytes(&self) -> Result<Vec<u8>, FunctionCallError> {
        self.fs
            .read_file(&self.uri, ReadFileOptions::default(), Some(&self.sandbox))
            .await
            .map_err(|err| failed(format!("unable to read {}: {err}", self.native_path())))
    }

    /// Selects lines of the file as [`read::slice_text`] does, reading it in chunks so that
    /// only the selected lines are held, as Claude Code reads a range of a large file.
    pub(crate) async fn read_lines(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<read::TextSlice, FunctionCallError> {
        let unreadable =
            |err: io::Error| failed(format!("unable to read {}: {err}", self.native_path()));
        let mut stream = self
            .fs
            .read_file_stream(&self.uri, Some(&self.sandbox))
            .await
            .map_err(unreadable)?;
        let mut slicer = read::LineSlicer::new(offset, limit);
        while let Some(chunk) = stream.next().await {
            slicer.push(&chunk.map_err(unreadable)?);
            if slicer.too_long() {
                break;
            }
        }
        slicer
            .finish()
            .map_err(|bytes| failed(read::token_limit_message(bytes.div_ceil(4))))
    }

    /// The file as text; a file that is not UTF-8 cannot be edited.
    pub(crate) async fn read_text(&self) -> Result<String, FunctionCallError> {
        String::from_utf8(self.read_bytes().await?).map_err(|_| {
            failed(format!(
                "{} is not a UTF-8 text file and cannot be edited.",
                self.native_path()
            ))
        })
    }

    /// Remembers the file as the session now knows it, after a write: its whole content.
    pub(crate) async fn record_written(&self, invocation: &ToolInvocation) {
        let Ok(Some(metadata)) = self.metadata().await else {
            return;
        };
        let record = match self.read_text().await {
            Ok(content) => FileRecord::Whole(ContentDigest::of(&content)),
            Err(_) => FileRecord::Part(FileStamp::of(&metadata)),
        };
        ReadFiles::of(&invocation.session).insert(self.key(), record);
    }
}

/// Applies `patch` through Codex's `apply_patch` runtime, under the model's call: the patch
/// is approved, sandboxed, shown and tracked as any `apply_patch` call.
pub(crate) async fn apply_patch(
    apply_patch: &Arc<dyn CoreToolRuntime>,
    invocation: &ToolInvocation,
    patch: String,
) -> Result<(), FunctionCallError> {
    apply_patch
        .handle(delegated(invocation, ToolPayload::Custom { input: patch }))
        .await
        .map(|_| ())
}
