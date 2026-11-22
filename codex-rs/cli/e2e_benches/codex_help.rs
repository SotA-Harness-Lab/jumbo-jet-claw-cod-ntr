// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: renamed the program

#![allow(clippy::expect_used)]

use std::process::Command;

use divan::Bencher;

fn main() {
    divan::main();
}

/// Exercises the Bazel-backed end-to-end benchmark path with a cheap,
/// deterministic Codex invocation. Richer scenarios can add separate
/// benchmark binaries without making the shared harness depend on them.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn codex_help(bencher: Bencher) {
    let codex = codex_utils_cargo_bin::cargo_bin("jjcn")
        .expect("codex binary should be available through Bazel runfiles");

    bencher.bench_local(move || {
        let output = Command::new(&codex)
            .arg("--help")
            .output()
            .expect("codex --help should run");
        assert!(output.status.success(), "codex --help should succeed");
    });
}
