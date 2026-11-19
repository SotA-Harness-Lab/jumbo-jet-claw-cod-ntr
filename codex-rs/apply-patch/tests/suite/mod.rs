// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: marked the remaining Windows-only code

mod cli;
#[cfg(unix)]
mod no_follow;
mod scenarios;
// WINDOWS-REMNANT
#[cfg(not(target_os = "windows"))]
mod tool;
