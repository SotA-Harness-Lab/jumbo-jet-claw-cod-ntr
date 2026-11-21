// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `CLAUDE_ENV_FILE`: a file a `SessionStart` hook of Claude Code writes variables into, for
//! the commands run later in the session.
//!
//! Claude Code puts the files' text in front of every command it runs. Here a file's
//! assignments (`NAME=value`, `export NAME=value` and `declare -x NAME=value` as `export -p`
//! prints them, a line holding one or more) are added to the environment of the session's
//! commands. Their values are read as the shell reads them: quotes, backslashes, `~`, and
//! `$NAME` and `${NAME}`, which are looked up in the command's environment when it runs, so
//! `export PATH="$PATH:./node_modules/.bin"` adds to the command's own `PATH`. A line with
//! anything else (other commands, command substitution, `${NAME:-...}`) is left out whole.

use std::collections::HashMap;
use std::iter::Peekable;
use std::path::Path;
use std::path::PathBuf;
use std::str::Chars;

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

/// One assignment of a hook's file, whose value may name variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvAssignment {
    pub name: String,
    parts: Vec<ValuePart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValuePart {
    Text(String),
    Variable(String),
}

impl EnvAssignment {
    /// The value, each variable it names taken from `lookup` (empty when unset).
    pub fn value(&self, lookup: impl Fn(&str) -> Option<String>) -> String {
        self.parts
            .iter()
            .map(|part| match part {
                ValuePart::Text(text) => text.clone(),
                ValuePart::Variable(name) => lookup(name).unwrap_or_default(),
            })
            .collect()
    }
}

/// Applies `assignments` to `env` in their order, each value expanded against `env` as the
/// earlier ones left it.
pub fn apply_env_assignments(assignments: &[EnvAssignment], env: &mut HashMap<String, String>) {
    for assignment in assignments {
        let value = assignment.value(|name| env.get(name).cloned());
        env.insert(assignment.name.clone(), value);
    }
}

/// The assignments of every file in `dir`, in the order of the files' names.
pub fn read_hook_env_dir(dir: &Path) -> Vec<EnvAssignment> {
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

/// The assignments of a shell script, line by line; see the module's documentation.
pub fn parse_env_file(text: &str) -> Vec<EnvAssignment> {
    text.lines().filter_map(parse_line).flatten().collect()
}

fn parse_line(line: &str) -> Option<Vec<EnvAssignment>> {
    let mut rest = line.trim_start();
    let mut declaration = false;
    for keyword in ["export", "declare -x"] {
        if let Some(after) = rest.strip_prefix(keyword)
            && after.starts_with([' ', '\t'])
        {
            rest = after;
            declaration = true;
            break;
        }
    }
    let mut chars = rest.chars().peekable();
    let mut assignments = Vec::new();
    loop {
        while chars.next_if(|ch| matches!(ch, ' ' | '\t')).is_some() {}
        if matches!(chars.peek(), None | Some('#')) {
            break;
        }
        let name = take_name(&mut chars);
        if !is_name(&name) {
            return None;
        }
        match chars.peek().copied() {
            Some('=') => {
                chars.next();
            }
            // `export NAME` exports a variable that is already set.
            None | Some(' ' | '\t') if declaration => continue,
            _ => return None,
        }
        let parts = parse_value(&mut chars)?;
        assignments.push(EnvAssignment { name, parts });
    }
    (!assignments.is_empty()).then_some(assignments)
}

/// One shell word, up to an unquoted blank; `None` for what is not a plain value.
fn parse_value(chars: &mut Peekable<Chars<'_>>) -> Option<Vec<ValuePart>> {
    let mut parts = Vec::new();
    let mut text = String::new();
    // `~` is the home directory at the start of the value and after an unquoted `:`.
    let mut tilde_allowed = true;
    while let Some(ch) = chars.next_if(|ch| !matches!(ch, ' ' | '\t')) {
        let tilde_here = std::mem::replace(&mut tilde_allowed, false);
        match ch {
            '\\' => text.push(chars.next()?),
            '\'' => loop {
                match chars.next()? {
                    '\'' => break,
                    ch => text.push(ch),
                }
            },
            '"' => loop {
                match chars.next()? {
                    '"' => break,
                    '\\' => match chars.next()? {
                        escaped @ ('$' | '`' | '"' | '\\') => text.push(escaped),
                        other => {
                            text.push('\\');
                            text.push(other);
                        }
                    },
                    '$' => dollar(chars, &mut parts, &mut text)?,
                    '`' => return None,
                    other => text.push(other),
                }
            },
            '$' => dollar(chars, &mut parts, &mut text)?,
            '`' | ';' | '|' | '&' | '<' | '>' | '(' | ')' => return None,
            '~' if tilde_here && matches!(chars.peek(), None | Some('/' | ':' | ' ' | '\t')) => {
                push_variable(&mut parts, &mut text, "HOME".to_string());
            }
            ':' => {
                text.push(':');
                tilde_allowed = true;
            }
            other => text.push(other),
        }
    }
    if !text.is_empty() {
        parts.push(ValuePart::Text(text));
    }
    Some(parts)
}

/// What follows a `$`: a variable (`$NAME`, `${NAME}`), or `$` itself before anything that
/// does not start one; `None` for an expansion that is not a plain variable.
fn dollar(
    chars: &mut Peekable<Chars<'_>>,
    parts: &mut Vec<ValuePart>,
    text: &mut String,
) -> Option<()> {
    match chars.peek().copied() {
        Some('{') => {
            chars.next();
            let name = take_name(chars);
            if chars.next()? != '}' || !is_name(&name) {
                return None;
            }
            push_variable(parts, text, name);
        }
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => {
            let name = take_name(chars);
            push_variable(parts, text, name);
        }
        Some(ch) if ch.is_ascii_digit() || "(?!#@*$-".contains(ch) => return None,
        _ => text.push('$'),
    }
    Some(())
}

fn push_variable(parts: &mut Vec<ValuePart>, text: &mut String, name: String) {
    if !text.is_empty() {
        parts.push(ValuePart::Text(std::mem::take(text)));
    }
    parts.push(ValuePart::Variable(name));
}

fn take_name(chars: &mut Peekable<Chars<'_>>) -> String {
    let mut name = String::new();
    while let Some(ch) = chars.next_if(|ch| ch.is_ascii_alphanumeric() || *ch == '_') {
        name.push(ch);
    }
    name
}

fn is_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
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
#[path = "env_file_tests.rs"]
mod tests;
