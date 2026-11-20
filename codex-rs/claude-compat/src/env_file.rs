// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `CLAUDE_ENV_FILE`: a file a `SessionStart` hook of Claude Code writes variables into, for
//! the commands run later in the session.
//!
//! Claude Code puts the files' text in front of every command it runs. Here a file's
//! assignments (`NAME=value` and `export NAME=value`, one a line, the value as written or in
//! quotes, without expansion) are added to the environment of the session's commands, and
//! anything else in it is left out.

use std::path::Path;
use std::path::PathBuf;

/// The directory the `SessionStart` hooks of the session `session_id` are given their
/// files in.
pub fn hook_env_dir(session_id: &str) -> PathBuf {
    std::env::temp_dir()
        .join("ccn-hook-env")
        .join(sanitize(session_id))
}

/// The file the hook at `index` among the session's `SessionStart` hooks writes.
pub fn hook_env_file(session_id: &str, index: i64) -> PathBuf {
    hook_env_dir(session_id).join(format!("sessionstart-hook-{index}.sh"))
}

/// The assignments of every file in `dir`, in the order of the files' names.
pub fn read_hook_env_dir(dir: &Path) -> Vec<(String, String)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .iter()
        .filter_map(|path| std::fs::read_to_string(path).ok())
        .flat_map(|text| parse_env_file(&text))
        .collect()
}

/// The assignments of a shell script: `NAME=value` or `export NAME=value` lines.
pub fn parse_env_file(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            let line = line.strip_prefix("export ").map_or(line, str::trim_start);
            let (name, value) = line.split_once('=')?;
            let valid_name = name
                .chars()
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
                && name
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_');
            if !valid_name {
                return None;
            }
            Some((name.to_string(), unquote(value.trim())))
        })
        .collect()
}

fn unquote(value: &str) -> String {
    for quote in ['"', '\''] {
        if value.len() >= 2 && value.starts_with(quote) && value.ends_with(quote) {
            return value[1..value.len() - 1].to_string();
        }
    }
    value.to_string()
}

fn sanitize(session_id: &str) -> String {
    session_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn assignments_are_read_and_other_shell_code_is_left_out() {
        let text = "# set up\nexport PATH_EXTRA=/opt/bin\nNODE_ENV=\"test\"\nexport  QUOTED='a b'\nsource ~/.nvm/nvm.sh\n1BAD=x\nif true; then X=1; fi\n";
        assert_eq!(
            parse_env_file(text),
            vec![
                ("PATH_EXTRA".to_string(), "/opt/bin".to_string()),
                ("NODE_ENV".to_string(), "test".to_string()),
                ("QUOTED".to_string(), "a b".to_string()),
            ]
        );
    }
}
