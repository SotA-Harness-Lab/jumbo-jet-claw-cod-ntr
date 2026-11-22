// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The text Claude Code gives the model when a skill is invoked: the `Base directory for
//! this skill` line, `$ARGUMENTS` and the other argument placeholders, `${CLAUDE_SKILL_DIR}`
//! and `${CLAUDE_SESSION_ID}`, substituted as Claude Code substitutes them.
//!
//! Shell injection (```` ```! ```` blocks and inline ``!`cmd` ``) is not done here: it runs
//! commands, and the text is left as it is.

use crate::markdown::parse_document;
use crate::markdown::parse_frontmatter;

const SKILL_DIR_PLACEHOLDER: &str = "${CLAUDE_SKILL_DIR}";
const SESSION_ID_PLACEHOLDER: &str = "${CLAUDE_SESSION_ID}";
const ARGUMENTS_WORD: &str = "ARGUMENTS";

pub const PROJECT_DIR_PLACEHOLDER: &str = "${CLAUDE_PROJECT_DIR}";

/// Values taken from the step invoking a skill.
#[derive(Default)]
pub struct SkillExecutionVariables {
    pub project_dir: Option<String>,
    pub effort: Option<String>,
}

impl SkillExecutionVariables {
    pub fn substitute(&self, mut content: String) -> String {
        if let Some(dir) = &self.project_dir
            && content.contains(PROJECT_DIR_PLACEHOLDER)
        {
            content = content.replace(PROJECT_DIR_PLACEHOLDER, &dir.replace('\\', "/"));
        }
        if let Some(effort) = &self.effort
            && content.contains("${CLAUDE_EFFORT}")
        {
            content = content.replace("${CLAUDE_EFFORT}", effort);
        }
        content
    }
}

/// The arguments of a skill invoked as `/<name> <arguments>`: the rest of `text` when it
/// starts with `/<name>` followed by whitespace or by nothing, trimmed. `None` when `text`
/// does not invoke the skill that way.
pub fn slash_invocation_arguments<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let rest = text.strip_prefix('/')?.strip_prefix(name)?;
    if rest.is_empty() {
        return Some(rest);
    }
    rest.starts_with(char::is_whitespace).then_some(rest.trim())
}

/// The text of a skill invoked as `/<name> <arguments>`, as Claude Code builds it: the
/// frontmatter is left out, the body is headed `Base directory for this skill: <dir>`, and
/// the argument placeholders, `${CLAUDE_SKILL_DIR}` and `${CLAUDE_SESSION_ID}` are
/// replaced. When the body has no argument placeholder and `arguments` is not empty, the
/// arguments are appended as `ARGUMENTS: <arguments>`.
pub fn render_invoked_skill(
    contents: &str,
    skill_dir: &str,
    session_id: &str,
    arguments: &str,
) -> String {
    let document = parse_document(contents);
    let names = document.frontmatter.map(argument_names).unwrap_or_default();
    let content = if skill_dir.is_empty() {
        document.body.to_string()
    } else {
        format!(
            "Base directory for this skill: {skill_dir}\n\n{}",
            document.body
        )
    };
    let content = substitute_arguments(&content, arguments, &names);
    substitute_skill_variables(&content, skill_dir, session_id)
}

/// Replaces `${CLAUDE_SKILL_DIR}` (unless `skill_dir` is empty) and `${CLAUDE_SESSION_ID}`,
/// the placeholders that need no arguments. Backslashes in `skill_dir` become forward
/// slashes, as in Claude Code, so that shell commands do not read them as escapes.
pub fn substitute_skill_variables(content: &str, skill_dir: &str, session_id: &str) -> String {
    let content = if skill_dir.is_empty() {
        content.to_string()
    } else {
        content.replace(SKILL_DIR_PLACEHOLDER, &skill_dir.replace('\\', "/"))
    };
    content.replace(SESSION_ID_PLACEHOLDER, session_id)
}

/// The names of a skill's arguments, from the `arguments` field of its frontmatter: a list,
/// or a string of names separated by whitespace. Empty names and names made only of digits
/// (which would clash with `$0`, `$1`) are dropped. The frontmatter is read as Claude Code
/// reads it, so a `description: Triggers include: X` beside it does not lose the names.
pub fn argument_names(frontmatter: &str) -> Vec<String> {
    let Some(value) = parse_frontmatter(frontmatter) else {
        return Vec::new();
    };
    let names: Vec<String> = match value.get("arguments") {
        Some(serde_yaml::Value::String(text)) => {
            text.split_whitespace().map(str::to_string).collect()
        }
        Some(serde_yaml::Value::Sequence(items)) => items
            .iter()
            .filter_map(serde_yaml::Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    };
    names
        .into_iter()
        .filter(|name| !name.trim().is_empty() && !name.bytes().all(|byte| byte.is_ascii_digit()))
        .collect()
}

/// Splits an argument string into words as a shell does: whitespace separates words, single
/// quotes keep everything literally, double quotes keep whitespace and let a backslash
/// escape `"`, `\`, `$` and `` ` ``, and a backslash outside quotes escapes the next
/// character. `$NAME` is kept as written. An unclosed quote runs to the end.
pub fn parse_arguments(arguments: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_word = false;
    let mut chars = arguments.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\'' => {
                in_word = true;
                for quoted in chars.by_ref() {
                    if quoted == '\'' {
                        break;
                    }
                    word.push(quoted);
                }
            }
            '"' => {
                in_word = true;
                while let Some(quoted) = chars.next() {
                    match quoted {
                        '"' => break,
                        '\\' => match chars.next() {
                            Some(escaped @ ('"' | '\\' | '$' | '`')) => word.push(escaped),
                            Some(other) => {
                                word.push('\\');
                                word.push(other);
                            }
                            None => word.push('\\'),
                        },
                        other => word.push(other),
                    }
                }
            }
            '\\' => {
                in_word = true;
                if let Some(escaped) = chars.next() {
                    word.push(escaped);
                }
            }
            ch if ch.is_whitespace() => {
                if in_word {
                    words.push(std::mem::take(&mut word));
                    in_word = false;
                }
            }
            other => {
                in_word = true;
                word.push(other);
            }
        }
    }
    if in_word {
        words.push(word);
    }
    words
}

/// Substitutes argument placeholders as Claude Code does, in one pass from left to right,
/// so that a value put in is never read again for placeholders. At each `$`, in this
/// order: a named argument (`$<name>` not followed by `[` or a word character),
/// `$ARGUMENTS[N]`, `$N` (not followed by a word character), then `$ARGUMENTS`. A named
/// argument past the last argument becomes empty; `$ARGUMENTS[N]` and `$N` past the last
/// argument stay as written, as in newer versions of Claude Code, so that the `$1` of an
/// `awk` program is not lost. When no placeholder received an argument (one kept as
/// written does not count) and `arguments` is not empty, `ARGUMENTS: <arguments>` is
/// appended after a blank line.
pub fn substitute_arguments(content: &str, arguments: &str, names: &[String]) -> String {
    let parsed = parse_arguments(arguments);
    let mut out = String::with_capacity(content.len());
    let mut received = false;
    let mut rest = content;
    while let Some(start) = rest.find('$') {
        out.push_str(&rest[..start]);
        let after_sigil = &rest[start + 1..];
        let (len, value) = match placeholder_at(after_sigil, names) {
            Some(Placeholder::Named { index, len }) => {
                (len, Some(parsed.get(index).map_or("", String::as_str)))
            }
            Some(Placeholder::Indexed { index, len }) => (
                len,
                index
                    .and_then(|index| parsed.get(index))
                    .map(String::as_str),
            ),
            Some(Placeholder::All { len }) => (len, Some(arguments)),
            None => (0, None),
        };
        match value {
            Some(value) => {
                out.push_str(value);
                received = true;
            }
            None => out.push_str(&rest[start..start + 1 + len]),
        }
        rest = &after_sigil[len..];
    }
    out.push_str(rest);

    if !received && !arguments.is_empty() {
        out.push_str("\n\nARGUMENTS: ");
        out.push_str(arguments);
    }
    out
}

/// A placeholder found after a `$`, with the length of its text after the `$`.
enum Placeholder {
    /// The `index`-th name of the `arguments` frontmatter.
    Named { index: usize, len: usize },
    /// `$ARGUMENTS[N]` or `$N`; `index` is `None` when `N` is too large to be one.
    Indexed { index: Option<usize>, len: usize },
    /// `$ARGUMENTS`.
    All { len: usize },
}

/// The placeholder that `after_sigil`, the text after a `$`, starts with.
fn placeholder_at(after_sigil: &str, names: &[String]) -> Option<Placeholder> {
    for (index, name) in names.iter().enumerate() {
        if name.is_empty() {
            continue;
        }
        if let Some(tail) = after_sigil.strip_prefix(name.as_str())
            && !tail
                .chars()
                .next()
                .is_some_and(|next| next == '[' || is_word_char(next))
        {
            return Some(Placeholder::Named {
                index,
                len: name.len(),
            });
        }
    }
    if let Some(tail) = after_sigil.strip_prefix(ARGUMENTS_WORD) {
        if let Some(inner) = tail.strip_prefix('[') {
            let digits = leading_digits(inner);
            if digits > 0 && inner[digits..].starts_with(']') {
                return Some(Placeholder::Indexed {
                    index: inner[..digits].parse().ok(),
                    len: ARGUMENTS_WORD.len() + digits + "[]".len(),
                });
            }
        }
        return Some(Placeholder::All {
            len: ARGUMENTS_WORD.len(),
        });
    }
    let digits = leading_digits(after_sigil);
    let followed_by_word = after_sigil[digits..]
        .chars()
        .next()
        .is_some_and(is_word_char);
    (digits > 0 && !followed_by_word).then(|| Placeholder::Indexed {
        index: after_sigil[..digits].parse().ok(),
        len: digits,
    })
}

/// The length of the ASCII digits `text` starts with.
fn leading_digits(text: &str) -> usize {
    text.len()
        - text
            .trim_start_matches(|ch: char| ch.is_ascii_digit())
            .len()
}

fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[cfg(test)]
#[path = "skill_invocation_tests.rs"]
mod tests;
