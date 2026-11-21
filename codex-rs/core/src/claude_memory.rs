// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: the auto memory Claude Code keeps for a project. Codex finds
//! it where Claude Code does, shows the model Claude Code's instructions for writing it,
//! and lets the sandbox write there, so that both tools share one memory.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use codex_claude_compat::CLAUDE_DIR;
use codex_claude_compat::FileSource;
use codex_claude_compat::LocalFiles;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;

/// The auto memory of a project, when Claude Code would keep one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClaudeAutoMemory {
    /// The memory directory; its index is `MEMORY.md` in it.
    pub(crate) dir: PathBuf,
    /// Claude Code's instructions for writing the memory, with `dir` filled in.
    pub(crate) instructions: String,
}

/// The auto memory of a session in `cwd`, decided from the Claude Code settings of
/// `home_dir` and `cwd` and from the environment (`process_env`, under the `env` of the
/// settings, which Claude Code applies over its own environment). `None` when Claude Code
/// has auto memory turned off, or when there is no home directory.
///
/// In a project that is not trusted, the project's own settings files can still turn the
/// memory off, but neither their `env` nor their `autoMemoryDirectory` is taken: both can
/// name the directory, which is made writable, and they come with the repository.
pub(crate) async fn claude_auto_memory_for(
    source: &dyn FileSource,
    cwd: &Path,
    home_dir: Option<&Path>,
    process_env: &(dyn Fn(&str) -> Option<String> + Sync),
    project_trusted: bool,
) -> Option<ClaudeAutoMemory> {
    let home_dir = home_dir?;
    let claude_home = home_dir.join(CLAUDE_DIR);
    let user = read_settings(&claude_home.join("settings.json"));
    let dot_claude = cwd.join(CLAUDE_DIR);
    // In the home directory, `.claude` is the user's own configuration.
    // The local files are the one in `cwd` and the one at the repository's root.
    let (project, local) = if dot_claude == claude_home {
        (None, Vec::new())
    } else {
        (
            read_settings(&dot_claude.join("settings.json")),
            codex_claude_compat::local_settings_paths(cwd, Some(home_dir))
                .into_iter()
                .filter_map(|path| read_settings(&path))
                .collect::<Vec<Value>>(),
        )
    };
    let settings = user
        .iter()
        .chain(&project)
        .chain(&local)
        .collect::<Vec<&Value>>();
    let mut settings_env = HashMap::new();
    let env_settings = if project_trusted {
        settings.clone()
    } else {
        user.iter().collect()
    };
    for settings in env_settings {
        settings_env.extend(codex_claude_compat::settings_env(settings));
    }
    let env = |name: &str| {
        settings_env
            .get(name)
            .cloned()
            .or_else(|| process_env(name))
    };
    if !codex_claude_compat::auto_memory_enabled(&settings, &env) {
        return None;
    }
    // A project's shared settings never move the memory: a repository could otherwise
    // have Codex write wherever it names.
    let no_settings: &[Value] = &[];
    let trusted_settings = user
        .iter()
        .chain(if project_trusted {
            local.as_slice()
        } else {
            no_settings
        })
        .collect::<Vec<&Value>>();
    let dir = match codex_claude_compat::auto_memory_dir_override(
        &trusted_settings,
        Some(home_dir),
        &env,
    ) {
        Some(dir) => dir,
        None => {
            let project_root = codex_claude_compat::auto_memory_project_root(source, cwd)
                .await
                .ok()?;
            let base_dir = codex_claude_compat::auto_memory_base_dir(&claude_home, &env);
            codex_claude_compat::auto_memory_dir(&base_dir, &project_root)?
        }
    };
    let instructions = codex_claude_compat::auto_memory_instructions(&dir, &env);
    Some(ClaudeAutoMemory { dir, instructions })
}

/// [`claude_auto_memory_for`] in a trusted project.
#[cfg(test)]
pub(crate) async fn claude_auto_memory(
    source: &dyn FileSource,
    cwd: &Path,
    home_dir: Option<&Path>,
    process_env: &(dyn Fn(&str) -> Option<String> + Sync),
) -> Option<ClaudeAutoMemory> {
    claude_auto_memory_for(
        source,
        cwd,
        home_dir,
        process_env,
        /*project_trusted*/ true,
    )
    .await
}

/// The auto memory directory of a local session in `cwd`, created if missing, as Claude
/// Code creates it at the start of every session (its instructions tell the model the
/// directory exists). `None` when Claude Code compatibility or auto memory is off.
pub(crate) async fn local_auto_memory_dir(
    cwd: &Path,
    project_trusted: bool,
) -> Option<AbsolutePathBuf> {
    // Unit tests never read the real home directory's Claude Code settings.
    if cfg!(test) || !codex_claude_compat::enabled() {
        return None;
    }
    let home_dir = dirs::home_dir();
    let memory = claude_auto_memory_for(
        &LocalFiles,
        cwd,
        home_dir.as_deref(),
        &process_env,
        project_trusted,
    )
    .await?;
    if let Err(err) = tokio::fs::create_dir_all(&memory.dir).await {
        // The model's own write will show the real error.
        tracing::debug!(
            "failed to create Claude Code memory directory {}: {err}",
            memory.dir.display()
        );
    }
    AbsolutePathBuf::from_absolute_path_checked(&memory.dir).ok()
}

/// `policy` with `memory_dir` writable too, when `policy` lets the session write its
/// working directory `cwd` but not the whole disk: the memory is outside the workspace,
/// and Claude Code lets itself write it without asking. `None` when nothing changes.
pub(crate) fn with_writable_memory_dir(
    policy: &FileSystemSandboxPolicy,
    cwd: &Path,
    memory_dir: &AbsolutePathBuf,
) -> Option<FileSystemSandboxPolicy> {
    if policy.has_full_disk_write_access()
        || !policy.can_write_local_path_with_cwd(cwd, cwd)
        || policy.can_write_local_path_with_cwd(memory_dir.as_path(), cwd)
    {
        return None;
    }
    Some(
        policy
            .clone()
            .with_additional_writable_roots(cwd, std::slice::from_ref(memory_dir)),
    )
}

/// The process environment, for [`claude_auto_memory`].
pub(crate) fn process_env(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

fn read_settings(path: &Path) -> Option<Value> {
    match codex_claude_compat::read_settings(path) {
        Ok(settings) => settings,
        Err(err) => {
            tracing::warn!(
                "failed to read Claude Code settings {}: {err}",
                path.display()
            );
            None
        }
    }
}

#[cfg(test)]
#[path = "claude_memory_tests.rs"]
mod tests;
