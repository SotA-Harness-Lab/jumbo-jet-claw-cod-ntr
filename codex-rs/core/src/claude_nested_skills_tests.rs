// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::process::Stdio;

use codex_extension_api::ExtensionData;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;

use super::candidate_dirs;
use super::discover;
use super::nested_skill_roots;

#[test]
fn directories_between_a_file_and_the_working_directory_are_looked_at() {
    let cwd = PathBuf::from("/repo");
    assert_eq!(
        candidate_dirs(&cwd, &cwd.join("apps/web/src/a.ts")),
        vec![
            (
                PathBuf::from("/repo/apps/web/src"),
                PathBuf::from("/repo/apps/web/src/.claude/skills"),
            ),
            (
                PathBuf::from("/repo/apps/web"),
                PathBuf::from("/repo/apps/web/.claude/skills"),
            ),
            (
                PathBuf::from("/repo/apps"),
                PathBuf::from("/repo/apps/.claude/skills"),
            ),
        ]
    );
    // The working directory's own skills are loaded at startup.
    assert_eq!(candidate_dirs(&cwd, &cwd.join("a.ts")), Vec::new());
    assert_eq!(
        candidate_dirs(&cwd, &PathBuf::from("/other/a.ts")),
        Vec::new()
    );
    assert_eq!(
        candidate_dirs(&cwd, &PathBuf::from("/repository/a/b.ts")),
        Vec::new()
    );
    assert_eq!(
        candidate_dirs(&cwd, &cwd.join("../etc/x/y.conf")),
        Vec::new()
    );
}

#[tokio::test]
async fn skills_below_the_working_directory_are_found_unless_git_ignores_them() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let cwd = temp_dir.path().to_path_buf();
    let git_init = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(&cwd)
        .stdin(Stdio::null())
        .status()
        .expect("run git init");
    assert_eq!(git_init.success(), true);
    std::fs::write(cwd.join(".gitignore"), "node_modules/\n").expect("write .gitignore");
    let web = cwd.join("apps/web");
    std::fs::create_dir_all(web.join(".claude/skills/deploy")).expect("create skills");
    std::fs::create_dir_all(web.join("src")).expect("create sources");
    let package = cwd.join("node_modules/pkg");
    std::fs::create_dir_all(package.join(".claude/skills/install")).expect("create skills");

    let session_data = ExtensionData::new("session");
    assert_eq!(nested_skill_roots(&session_data), Vec::new());
    discover(
        &session_data,
        &cwd,
        &[web.join("src/a.ts"), package.join("index.js")],
    )
    .await;
    assert_eq!(
        nested_skill_roots(&session_data),
        vec![
            AbsolutePathBuf::from_absolute_path_checked(web.join(".claude/skills"))
                .expect("absolute"),
        ]
    );

    // A folder looked at once is not looked at again, even once it exists.
    let later = cwd.join("apps/api");
    std::fs::create_dir_all(later.join("src")).expect("create sources");
    discover(&session_data, &cwd, &[later.join("src/b.ts")]).await;
    std::fs::create_dir_all(later.join(".claude/skills/serve")).expect("create skills");
    discover(&session_data, &cwd, &[later.join("src/b.ts")]).await;
    assert_eq!(nested_skill_roots(&session_data).len(), 1);
}
