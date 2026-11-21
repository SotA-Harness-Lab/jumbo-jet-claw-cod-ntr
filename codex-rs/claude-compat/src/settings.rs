// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `settings.json`: the parts Codex can apply, `env`, `effortLevel` and the
//! Bash permission rules.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::CLAUDE_DIR;

/// What a Bash permission rule decides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionDecision {
    Allow,
    Ask,
    Deny,
}

/// A Bash permission rule Codex can enforce: commands starting with `prefix`, word by word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BashPrefixRule {
    pub prefix: Vec<String>,
    pub decision: PermissionDecision,
}

/// The settings files Claude Code reads, lowest precedence first: the user's
/// `~/.claude/settings.json`, then the project's `.claude/settings.json` in `project_dir`
/// and its `.claude/settings.local.json` files (see [`local_settings_paths`]).
pub fn settings_paths(claude_home: Option<&Path>, project_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(claude_home) = claude_home {
        paths.push(claude_home.join("settings.json"));
    }
    if let Some(project_dir) = project_dir {
        let dot_claude = project_dir.join(CLAUDE_DIR);
        // In the home directory, `.claude` is the user's own configuration.
        if claude_home != Some(dot_claude.as_path()) {
            paths.push(dot_claude.join("settings.json"));
            paths.extend(local_settings_paths(
                project_dir,
                claude_home.and_then(Path::parent),
            ));
        }
    }
    paths
}

const LOCAL_SETTINGS_FILE: &str = "settings.local.json";

/// The `.claude/settings.local.json` files Claude Code reads for a session in `project_dir`,
/// lowest precedence first. Below the root of a Git repository (the main checkout's root,
/// from a linked worktree) Claude Code keeps the file at that root, and still reads one an
/// earlier version left in `project_dir`, the root's file winning where both set a key. The
/// file is read from `project_dir` alone outside a repository, when the root is the home
/// directory `home_dir`, on Windows, and when the root, its `.git` or its `.claude` is not
/// owned by the user.
pub fn local_settings_paths(project_dir: &Path, home_dir: Option<&Path>) -> Vec<PathBuf> {
    let mut paths = vec![project_dir.join(CLAUDE_DIR).join(LOCAL_SETTINGS_FILE)];
    if let Some(root) = local_settings_root(project_dir, home_dir)
        && root != project_dir
    {
        paths.push(root.join(CLAUDE_DIR).join(LOCAL_SETTINGS_FILE));
    }
    paths
}

/// The repository root whose `.claude/settings.local.json` Claude Code reads for
/// `project_dir`, when it reads one there.
fn local_settings_root(project_dir: &Path, home_dir: Option<&Path>) -> Option<PathBuf> {
    if cfg!(windows) {
        return None;
    }
    let root = crate::mcp::git_repository_root(project_dir)?;
    if home_dir == Some(root.as_path()) {
        return None;
    }
    [root.clone(), root.join(".git"), root.join(CLAUDE_DIR)]
        .iter()
        .all(|path| owned_by_user(path))
        .then_some(root)
}

/// Whether `path` (not what a link there points to) belongs to the user Codex runs as; a
/// missing entry passes.
#[cfg(unix)]
fn owned_by_user(path: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match std::fs::symlink_metadata(path) {
        // SAFETY: geteuid has no preconditions and cannot fail.
        Ok(metadata) => metadata.uid() == unsafe { libc::geteuid() },
        Err(err) => err.kind() == io::ErrorKind::NotFound,
    }
}

#[cfg(not(unix))]
fn owned_by_user(_path: &Path) -> bool {
    true
}

/// The largest settings or `.mcp.json` file read; a larger one is an error.
const MAX_SETTINGS_BYTES: u64 = 16 * 1024 * 1024;

/// Reads a settings file; `Ok(None)` when it does not exist or is not a regular file
/// (a FIFO or a link to a device would hang or never end). A leading UTF-8 byte-order
/// mark is ignored, as Claude Code does.
pub fn read_settings(path: &Path) -> io::Result<Option<Value>> {
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_file() => {
            if metadata.len() > MAX_SETTINGS_BYTES {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("larger than {MAX_SETTINGS_BYTES} bytes"),
                ));
            }
        }
        Ok(_) => {
            tracing::warn!("ignoring {}: not a regular file", path.display());
            return Ok(None);
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    }
    let mut contents = String::new();
    match std::fs::File::open(path) {
        Ok(file) => {
            // The file may have grown since it was measured.
            io::Read::read_to_string(
                &mut io::Read::take(file, MAX_SETTINGS_BYTES),
                &mut contents,
            )?;
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    }
    let contents = contents.strip_prefix('\u{feff}').unwrap_or(&contents);
    serde_json::from_str(contents)
        .map(Some)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

/// The `env` entries of a settings file. Values that are not strings are written as JSON.
pub fn settings_env(settings: &Value) -> Vec<(String, String)> {
    let Some(env) = settings.get("env").and_then(Value::as_object) else {
        return Vec::new();
    };
    env.iter()
        .map(|(name, value)| {
            let value = match value {
                Value::String(text) => text.clone(),
                other => other.to_string(),
            };
            (name.clone(), value)
        })
        .collect()
}

/// The `effortLevel` of a settings file as a Codex reasoning effort: Claude Code's `low`,
/// `medium`, `high` and `xhigh` are Codex's levels of the same name, and `max` becomes `xhigh`,
/// as a subagent's `effort` does. Any other value is passed over.
pub fn settings_effort_level(settings: &Value) -> Option<&'static str> {
    match settings.get("effortLevel").and_then(Value::as_str)? {
        "low" => Some("low"),
        "medium" => Some("medium"),
        "high" => Some("high"),
        "xhigh" | "max" => Some("xhigh"),
        _ => None,
    }
}

/// The Bash rules of `permissions.allow`, `ask` and `deny` that name a command prefix,
/// and, separately, the rules that cannot be expressed as one (other tools, wildcards in
/// the middle of a command).
pub fn bash_prefix_rules(settings: &Value) -> (Vec<BashPrefixRule>, Vec<String>) {
    let mut rules = Vec::new();
    let mut unsupported = Vec::new();
    let Some(permissions) = settings.get("permissions") else {
        return (rules, unsupported);
    };
    for (key, decision) in [
        ("allow", PermissionDecision::Allow),
        ("ask", PermissionDecision::Ask),
        ("deny", PermissionDecision::Deny),
    ] {
        let Some(entries) = permissions.get(key).and_then(Value::as_array) else {
            continue;
        };
        for rule in entries.iter().filter_map(Value::as_str) {
            match bash_prefix(rule) {
                Some(prefix) => rules.push(BashPrefixRule { prefix, decision }),
                None => unsupported.push(rule.to_string()),
            }
        }
    }
    (rules, unsupported)
}

/// `Bash(git push*)`, `Bash(git push *)`, `Bash(git push:*)` and `Bash(git status)` all
/// become the prefix `git push` or `git status`. A word cut short by `*` is matched whole.
fn bash_prefix(rule: &str) -> Option<Vec<String>> {
    let content = rule.trim().strip_prefix("Bash(")?.strip_suffix(')')?.trim();
    let content = content
        .strip_suffix(":*")
        .or_else(|| content.strip_suffix('*'))
        .unwrap_or(content)
        .trim();
    if content.contains([
        '*', '\\', '"', '\'', '`', '$', '|', '&', ';', '<', '>', '(', ')',
    ]) {
        return None;
    }
    let words = content
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();
    (!words.is_empty()).then_some(words)
}

#[cfg(test)]
#[path = "settings_tests.rs"]
mod tests;
