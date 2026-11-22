// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: renamed the program

use predicates::prelude::*;

#[test]
fn missing_session_fails_before_delete_confirmation() -> anyhow::Result<()> {
    let codex_home = tempfile::tempdir()?;
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("jjcn")?);
    cmd.env("CODEX_HOME", codex_home.path())
        .args(["delete", "123e4567-e89b-12d3-a456-426614174000"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "No active or archived session found matching",
        ))
        .stderr(predicate::str::contains("cannot confirm").not());
    Ok(())
}
