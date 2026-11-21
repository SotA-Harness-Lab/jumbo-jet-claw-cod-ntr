// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: the skills of `.claude/skills` folders below the working
//! directory.
//!
//! Claude Code does not load them at startup. The first time the model works on a file,
//! it looks for `.claude/skills` in each directory from the file's own up to, not
//! including, the working directory, leaves out one whose directory git ignores (such as
//! `node_modules/<package>/.claude/skills`), and keeps the folders it found for the rest
//! of the session. The skills turn up in the catalog from the next model request on.

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Mutex;
use std::sync::MutexGuard;

use codex_extension_api::ExtensionData;
use codex_utils_absolute_path::AbsolutePathBuf;

const CLAUDE_SKILLS_DIR: [&str; 2] = [".claude", "skills"];

/// The nested `.claude/skills` folders of one session.
#[derive(Default)]
struct NestedSkillDirs(Mutex<NestedSkillDirsState>);

#[derive(Default)]
struct NestedSkillDirsState {
    /// Every `.claude/skills` path looked at, found or not, so that each is looked at once.
    checked: HashSet<PathBuf>,
    /// The folders found, deepest first.
    found: Vec<AbsolutePathBuf>,
}

impl NestedSkillDirs {
    fn state(&self) -> MutexGuard<'_, NestedSkillDirsState> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// The `.claude/skills` folders below the working directory found so far in this session,
/// deepest first.
pub(crate) fn nested_skill_roots(session_data: &ExtensionData) -> Vec<AbsolutePathBuf> {
    session_data
        .get::<NestedSkillDirs>()
        .map(|dirs| dirs.state().found.clone())
        .unwrap_or_default()
}

/// Looks for `.claude/skills` folders in the directories between each of `files` and
/// `cwd`, and keeps those found for the session.
pub(crate) async fn discover(session_data: &ExtensionData, cwd: &Path, files: &[PathBuf]) {
    // As in newer versions of Claude Code, the folders inside installed plugins do not count.
    let plugins_dir =
        dirs::home_dir().map(|home| home.join(codex_claude_compat::CLAUDE_DIR).join("plugins"));
    let nested = session_data.get_or_init(NestedSkillDirs::default);
    let candidates = {
        let mut state = nested.state();
        files
            .iter()
            .flat_map(|file| candidate_dirs(cwd, file))
            .filter(|(dir, _)| {
                plugins_dir
                    .as_ref()
                    .is_none_or(|plugins_dir| !dir.starts_with(plugins_dir))
            })
            .filter(|(_, skills)| state.checked.insert(skills.clone()))
            .collect::<Vec<_>>()
    };
    let mut found = Vec::new();
    for (dir, skills) in candidates {
        let is_dir = tokio::fs::metadata(&skills)
            .await
            .is_ok_and(|metadata| metadata.is_dir());
        if !is_dir {
            continue;
        }
        if is_git_ignored(cwd, &dir).await {
            tracing::debug!(
                "skipped the skills of an ignored directory: {}",
                skills.display()
            );
            continue;
        }
        if let Ok(skills) = AbsolutePathBuf::from_absolute_path_checked(&skills) {
            found.push(skills);
        }
    }
    if found.is_empty() {
        return;
    }
    let mut state = nested.state();
    state.found.extend(found);
    state
        .found
        .sort_by_key(|path| std::cmp::Reverse(path.as_path().components().count()));
}

/// Each directory from `file`'s own up to, not including, `cwd`, with its `.claude/skills`
/// path. A file outside `cwd` has none.
fn candidate_dirs(cwd: &Path, file: &Path) -> Vec<(PathBuf, PathBuf)> {
    // `..` in the path is resolved first, so that it cannot lead out of `cwd`.
    let Ok(file) = AbsolutePathBuf::from_absolute_path_checked(file) else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    let mut dir = file.as_path().parent();
    while let Some(current) = dir {
        if current == cwd || !current.starts_with(cwd) {
            break;
        }
        let skills = CLAUDE_SKILLS_DIR
            .iter()
            .fold(current.to_path_buf(), |path, part| path.join(part));
        candidates.push((current.to_path_buf(), skills));
        dir = current.parent();
    }
    candidates
}

/// Whether git ignores `dir`, asked as Claude Code does (`git check-ignore`, run in
/// `cwd`). Outside a repository, or when git cannot be run, nothing is ignored.
async fn is_git_ignored(cwd: &Path, dir: &Path) -> bool {
    tokio::process::Command::new("git")
        .arg("check-ignore")
        .arg("-q")
        .arg("--")
        .arg(dir)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .await
        .is_ok_and(|status| status.success())
}

#[cfg(test)]
#[path = "claude_nested_skills_tests.rs"]
mod tests;
