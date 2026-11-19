// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: marked the remaining Windows-only code

#[cfg_attr(target_os = "linux", path = "bubblewrap.rs")]
#[cfg_attr(target_os = "macos", path = "seatbelt.rs")]
// WINDOWS-REMNANT
#[cfg_attr(target_os = "windows", path = "mxc.rs")]
mod backend;
mod dependencies;
mod runner;
mod telemetry;

pub use runner::run_integrity_checks;

#[cfg(test)]
#[path = "backend_tests.rs"]
mod backend_tests;
