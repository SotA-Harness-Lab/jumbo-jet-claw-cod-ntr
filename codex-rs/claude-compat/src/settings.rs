// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `settings.json`: the parts Codex can apply, `env` and the Bash
//! permission rules.

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
/// `~/.claude/settings.json`, then the project's `.claude/settings.json` and
/// `.claude/settings.local.json` in `project_dir`.
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
            paths.push(dot_claude.join("settings.local.json"));
        }
    }
    paths
}

/// Reads a settings file; `Ok(None)` when it does not exist.
pub fn read_settings(path: &Path) -> io::Result<Option<Value>> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    serde_json::from_str(&contents)
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
    if content.contains(['*', '\\', '"', '\'', '`', '$', '|', '&', ';', '<', '>', '(', ')']) {
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
