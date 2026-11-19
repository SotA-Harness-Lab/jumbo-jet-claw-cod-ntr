// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: marked the remaining Windows-only code

#[cfg(unix)]
pub(crate) fn handle_exit_status(status: std::process::ExitStatus) -> ! {
    use std::os::unix::process::ExitStatusExt;

    // Use ExitStatus to derive the exit code.
    if let Some(code) = status.code() {
        std::process::exit(code);
    } else if let Some(signal) = status.signal() {
        std::process::exit(128 + signal);
    } else {
        std::process::exit(1);
    }
}

// WINDOWS-REMNANT
#[cfg(windows)]
pub(crate) fn handle_exit_status(status: std::process::ExitStatus) -> ! {
    if let Some(code) = status.code() {
        std::process::exit(code);
    } else {
        // Rare on Windows, but if it happens: use fallback code.
        std::process::exit(1);
    }
}
