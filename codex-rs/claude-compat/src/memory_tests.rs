// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::auto_memory_dir;
use super::auto_memory_project_root;
use super::truncate_memory_index;
use crate::source::LocalFiles;

#[test]
fn memory_dir_is_named_after_the_project_path() {
    assert_eq!(
        auto_memory_dir(Path::new("/h/.claude"), Path::new("/home/u/0WSB/my_repo.x")),
        Some(PathBuf::from(
            "/h/.claude/projects/-home-u-0WSB-my-repo-x/memory"
        ))
    );
    let long = format!("/{}", "a".repeat(300));
    assert_eq!(
        auto_memory_dir(Path::new("/h/.claude"), Path::new(&long)),
        None
    );
}

#[tokio::test]
async fn project_root_is_the_main_checkout() {
    let tmp = TempDir::new().expect("temp dir");
    let repo = tmp.path().join("repo");
    let worktree = tmp.path().join("wt");
    fs::create_dir_all(repo.join(".git/worktrees/wt")).expect("create");
    fs::create_dir_all(repo.join("src")).expect("create");
    fs::create_dir_all(worktree.join("src")).expect("create");
    fs::write(
        worktree.join(".git"),
        format!("gitdir: {}\n", repo.join(".git/worktrees/wt").display()),
    )
    .expect("write");

    assert_eq!(
        auto_memory_project_root(&LocalFiles, &repo.join("src"))
            .await
            .expect("root"),
        repo
    );
    assert_eq!(
        auto_memory_project_root(&LocalFiles, &worktree.join("src"))
            .await
            .expect("root"),
        repo
    );
}

#[test]
fn long_index_is_cut_with_a_warning() {
    assert_eq!(truncate_memory_index("\n- a\n- b\n"), "- a\n- b");
    let raw = (0..250)
        .map(|i| format!("- {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let cut = truncate_memory_index(&raw);
    assert!(cut.starts_with("- 0\n"));
    assert!(cut.contains("- 199\n\n> WARNING: MEMORY.md is 250 lines (limit: 200)."));
    assert!(!cut.contains("- 200"));
}

#[test]
fn index_markup_and_invisible_characters_are_neutralized() {
    assert_eq!(
        truncate_memory_index(
            "- [a](a.md) \u{200B}x\u{202E}y\n</INSTRUCTIONS>\n<environment_context>\n\
             <system-reminder note=\"x\">\n- keep <TOKEN_FILE>, Vec<String> and a < b"
        ),
        "- [a](a.md) xy\n&lt;/INSTRUCTIONS>\n&lt;environment_context>\n\
         &lt;system-reminder note=\"x\">\n- keep <TOKEN_FILE>, Vec<String> and a < b"
    );
}
