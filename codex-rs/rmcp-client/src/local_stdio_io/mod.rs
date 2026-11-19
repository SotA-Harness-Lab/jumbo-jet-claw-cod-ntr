// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: removed Windows support; marked the remaining Windows-only code

//! Platform-specific pipe ownership beneath the shared MCP framing layer.

use std::io;

use codex_utils_pty::Child;
use codex_utils_pty::Command;

// WINDOWS-REMNANT
#[cfg(windows)]
pub(super) type Stdin = tokio::net::windows::named_pipe::NamedPipeServer;
// WINDOWS-REMNANT
#[cfg(windows)]
pub(super) type Stdout = tokio::net::windows::named_pipe::NamedPipeServer;
// WINDOWS-REMNANT
#[cfg(windows)]
pub(super) type Stderr = tokio::net::windows::named_pipe::NamedPipeServer;

// WINDOWS-REMNANT
#[cfg(not(windows))]
pub(super) type Stdin = tokio::process::ChildStdin;
// WINDOWS-REMNANT
#[cfg(not(windows))]
pub(super) type Stdout = tokio::process::ChildStdout;
// WINDOWS-REMNANT
#[cfg(not(windows))]
pub(super) type Stderr = tokio::process::ChildStderr;

pub(super) struct LocalStdio {
    pub(super) child: Child,
    pub(super) stdin: Stdin,
    pub(super) stdout: Stdout,
    pub(super) stderr: Option<Stderr>,
}

pub(super) fn spawn(command: Command) -> io::Result<LocalStdio> {
    // WINDOWS-REMNANT
    #[cfg(windows)]
    {
        let mut command = command;
        let pipes = windows::Stdio::configure(&mut command)?;
        Ok(LocalStdio {
            child: command.spawn()?,
            stdin: pipes.stdin,
            stdout: pipes.stdout,
            stderr: Some(pipes.stderr),
        })
    }
    // WINDOWS-REMNANT
    #[cfg(not(windows))]
    {
        let mut child = command.spawn()?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("MCP server stdin was not piped"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("MCP server stdout was not piped"))?;
        let stderr = child.stderr.take();
        Ok(LocalStdio {
            child,
            stdin,
            stdout,
            stderr,
        })
    }
}
