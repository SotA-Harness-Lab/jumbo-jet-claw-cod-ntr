// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use codex_claude_compat::LocalFiles;
use codex_protocol::models::PermissionProfile;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::permissions::NetworkSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;

use super::claude_auto_memory;
use super::claude_auto_memory_for;
use super::with_writable_memory_dir;

fn no_env(_: &str) -> Option<String> {
    None
}

/// A home directory and a Git repository with a subdirectory to work in.
fn fixture() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let home = tmp.path().join("home");
    let repo = tmp.path().join("repo");
    fs::create_dir_all(home.join(".claude")).unwrap();
    fs::create_dir_all(repo.join(".git")).unwrap();
    fs::create_dir_all(repo.join(".claude")).unwrap();
    fs::create_dir_all(repo.join("src")).unwrap();
    (tmp, home, repo)
}

fn default_dir(home: &Path, repo: &Path) -> PathBuf {
    codex_claude_compat::auto_memory_dir(&home.join(".claude"), repo).expect("memory dir")
}

#[tokio::test]
async fn memory_of_the_repository_root_by_default() {
    let (_tmp, home, repo) = fixture();
    let memory = claude_auto_memory(&LocalFiles, &repo.join("src"), Some(&home), &no_env)
        .await
        .expect("auto memory");
    let dir = default_dir(&home, &repo);
    assert_eq!(memory.dir, dir);
    assert_eq!(
        memory.instructions,
        codex_claude_compat::auto_memory_instructions(&dir, &no_env)
    );
}

#[tokio::test]
async fn no_memory_without_a_home_directory() {
    let (_tmp, _home, repo) = fixture();
    assert_eq!(
        claude_auto_memory(&LocalFiles, &repo, /*home_dir*/ None, &no_env).await,
        None
    );
}

#[tokio::test]
async fn settings_turn_memory_off_and_local_settings_win() {
    let (_tmp, home, repo) = fixture();
    fs::write(
        home.join(".claude/settings.json"),
        r#"{"autoMemoryEnabled": false}"#,
    )
    .unwrap();
    assert_eq!(
        claude_auto_memory(&LocalFiles, &repo, Some(&home), &no_env).await,
        None
    );

    fs::write(
        repo.join(".claude/settings.local.json"),
        r#"{"autoMemoryEnabled": true}"#,
    )
    .unwrap();
    assert!(
        claude_auto_memory(&LocalFiles, &repo, Some(&home), &no_env)
            .await
            .is_some()
    );
}

#[tokio::test]
async fn settings_env_overrides_the_process_environment() {
    let (_tmp, home, repo) = fixture();
    let enabled = |name: &str| (name == "CLAUDE_CODE_DISABLE_AUTO_MEMORY").then(|| "0".to_string());
    fs::write(
        repo.join(".claude/settings.json"),
        r#"{"env": {"CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1"}}"#,
    )
    .unwrap();
    assert_eq!(
        claude_auto_memory(&LocalFiles, &repo, Some(&home), &enabled).await,
        None
    );
}

#[tokio::test]
async fn only_trusted_settings_move_the_directory() {
    let (_tmp, home, repo) = fixture();
    fs::write(
        repo.join(".claude/settings.json"),
        r#"{"autoMemoryDirectory": "~/.ssh"}"#,
    )
    .unwrap();
    let memory = claude_auto_memory(&LocalFiles, &repo, Some(&home), &no_env)
        .await
        .expect("auto memory");
    assert_eq!(memory.dir, default_dir(&home, &repo));

    fs::write(
        home.join(".claude/settings.json"),
        r#"{"autoMemoryDirectory": "~/notes/memory"}"#,
    )
    .unwrap();
    let memory = claude_auto_memory(&LocalFiles, &repo, Some(&home), &no_env)
        .await
        .expect("auto memory");
    assert_eq!(memory.dir, home.join("notes/memory"));
}

#[tokio::test]
async fn an_untrusted_project_cannot_name_the_directory() {
    let (_tmp, home, repo) = fixture();
    fs::write(
        repo.join(".claude/settings.json"),
        r#"{"env": {"CLAUDE_COWORK_MEMORY_PATH_OVERRIDE": "/somewhere/else"}}"#,
    )
    .unwrap();
    fs::write(
        repo.join(".claude/settings.local.json"),
        r#"{"autoMemoryDirectory": "~/.ssh"}"#,
    )
    .unwrap();

    let memory = claude_auto_memory_for(
        &LocalFiles,
        &repo,
        Some(&home),
        &no_env,
        /*project_trusted*/ false,
    )
    .await
    .expect("auto memory");
    assert_eq!(memory.dir, default_dir(&home, &repo));

    // It can still turn the memory off.
    fs::write(
        repo.join(".claude/settings.json"),
        r#"{"autoMemoryEnabled": false}"#,
    )
    .unwrap();
    assert_eq!(
        claude_auto_memory_for(
            &LocalFiles,
            &repo,
            Some(&home),
            &no_env,
            /*project_trusted*/ false,
        )
        .await,
        None
    );
}

#[test]
fn memory_is_writable_where_the_workspace_is() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let cwd = tmp.path().join("repo");
    let memory_dir =
        AbsolutePathBuf::from_absolute_path(tmp.path().join("home/.claude/projects/-repo/memory"))
            .expect("absolute");
    fs::create_dir_all(&cwd).unwrap();
    fs::create_dir_all(memory_dir.as_path()).unwrap();

    // Without the temporary directories, which hold the test's own files.
    let workspace_write = PermissionProfile::workspace_write_with(
        &[],
        NetworkSandboxPolicy::Restricted,
        /*exclude_tmpdir_env_var*/ true,
        /*exclude_slash_tmp*/ true,
    )
    .file_system_sandbox_policy();
    assert!(!workspace_write.can_write_local_path_with_cwd(memory_dir.as_path(), &cwd));
    let granted = with_writable_memory_dir(&workspace_write, &cwd, &memory_dir)
        .expect("memory made writable");
    assert!(granted.can_write_local_path_with_cwd(memory_dir.as_path(), &cwd));
    assert!(granted.can_write_local_path_with_cwd(&cwd, &cwd));
    assert_eq!(with_writable_memory_dir(&granted, &cwd, &memory_dir), None);

    // Read-only sessions stay read-only; full access needs nothing.
    assert_eq!(
        with_writable_memory_dir(&FileSystemSandboxPolicy::read_only(), &cwd, &memory_dir),
        None
    );
    assert_eq!(
        with_writable_memory_dir(&FileSystemSandboxPolicy::unrestricted(), &cwd, &memory_dir),
        None
    );
}
