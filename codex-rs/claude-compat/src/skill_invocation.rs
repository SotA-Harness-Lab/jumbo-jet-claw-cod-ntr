// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The text Claude Code gives the model when a skill is invoked: the `Base directory for
//! this skill` line, `$ARGUMENTS` and the other argument placeholders, `${CLAUDE_SKILL_DIR}`
//! and `${CLAUDE_SESSION_ID}`, substituted as Claude Code substitutes them.
//!
//! Shell injection (```` ```! ```` blocks and inline ``!`cmd` ``) is not done here: it runs
//! commands, and the text is left as it is.

use crate::markdown::parse_document;

const SKILL_DIR_PLACEHOLDER: &str = "${CLAUDE_SKILL_DIR}";
const SESSION_ID_PLACEHOLDER: &str = "${CLAUDE_SESSION_ID}";
const ARGUMENTS_PLACEHOLDER: &str = "$ARGUMENTS";

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
/// (which would clash with `$0`, `$1`) are dropped.
pub fn argument_names(frontmatter: &str) -> Vec<String> {
    let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(frontmatter) else {
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

/// Substitutes argument placeholders as Claude Code does, in this order: named arguments
/// (`$<name>` not followed by `[` or a word character), `$ARGUMENTS[N]`, `$N` (not followed
/// by a word character), then `$ARGUMENTS`. A named argument past the last argument
/// becomes empty; `$ARGUMENTS[N]` and `$N` past the last argument stay as written, as in
/// newer versions of Claude Code, so that the `$1` of an `awk` program is not lost. When
/// nothing was replaced and `arguments` is not empty, `ARGUMENTS: <arguments>` is appended
/// after a blank line.
pub fn substitute_arguments(content: &str, arguments: &str, names: &[String]) -> String {
    let parsed = parse_arguments(arguments);
    let argument = |index: usize| parsed.get(index).map(String::as_str);

    let mut result = content.to_string();
    for (index, name) in names.iter().enumerate() {
        result = replace_named(&result, name, argument(index).unwrap_or(""));
    }
    result = replace_indexed(
        &result,
        &format!("{ARGUMENTS_PLACEHOLDER}["),
        "]",
        &argument,
    );
    result = replace_shorthand(&result, &argument);
    result = replace_whole_arguments(&result, arguments);

    if result == content && !arguments.is_empty() {
        result.push_str("\n\nARGUMENTS: ");
        result.push_str(arguments);
    }
    result
}

fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// Replaces `$<name>` where it is not followed by `[` or a word character.
fn replace_named(content: &str, name: &str, value: &str) -> String {
    if name.is_empty() {
        return content.to_string();
    }
    let pattern = format!("${name}");
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find(&pattern) {
        let after = &rest[start + pattern.len()..];
        out.push_str(&rest[..start]);
        if after
            .chars()
            .next()
            .is_some_and(|next| next == '[' || is_word_char(next))
        {
            out.push_str(&pattern);
        } else {
            out.push_str(value);
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

/// Replaces `<open><digits><close>` with the argument at that index.
fn replace_indexed<'a>(
    content: &str,
    open: &str,
    close: &str,
    argument: &impl Fn(usize) -> Option<&'a str>,
) -> String {
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find(open) {
        let after_open = &rest[start + open.len()..];
        let digits = after_open.len()
            - after_open
                .trim_start_matches(|ch: char| ch.is_ascii_digit())
                .len();
        out.push_str(&rest[..start]);
        if digits > 0 && after_open[digits..].starts_with(close) {
            let end = digits + close.len();
            match index_value(&after_open[..digits], argument) {
                Some(value) => out.push_str(value),
                None => {
                    out.push_str(open);
                    out.push_str(&after_open[..end]);
                }
            }
            rest = &after_open[end..];
        } else {
            out.push_str(open);
            rest = after_open;
        }
    }
    out.push_str(rest);
    out
}

/// Replaces `$<digits>` not followed by a word character with the argument at that index.
fn replace_shorthand<'a>(content: &str, argument: &impl Fn(usize) -> Option<&'a str>) -> String {
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find('$') {
        let after_sigil = &rest[start + 1..];
        let digits = after_sigil.len()
            - after_sigil
                .trim_start_matches(|ch: char| ch.is_ascii_digit())
                .len();
        out.push_str(&rest[..start]);
        let followed_by_word = after_sigil[digits..]
            .chars()
            .next()
            .is_some_and(is_word_char);
        if digits > 0 && !followed_by_word {
            match index_value(&after_sigil[..digits], argument) {
                Some(value) => out.push_str(value),
                None => out.push_str(&rest[start..start + 1 + digits]),
            }
            rest = &after_sigil[digits..];
        } else {
            out.push('$');
            rest = after_sigil;
        }
    }
    out.push_str(rest);
    out
}

/// The argument at the index `digits` names; `None` when there is none.
fn index_value<'a>(digits: &str, argument: &impl Fn(usize) -> Option<&'a str>) -> Option<&'a str> {
    digits.parse::<usize>().ok().and_then(argument)
}

/// Replaces `$ARGUMENTS` with the whole argument string, except where it begins an
/// `$ARGUMENTS[N]` that was kept as written.
fn replace_whole_arguments(content: &str, arguments: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find(ARGUMENTS_PLACEHOLDER) {
        let after = &rest[start + ARGUMENTS_PLACEHOLDER.len()..];
        out.push_str(&rest[..start]);
        let kept_index = after.strip_prefix('[').is_some_and(|inner| {
            let index = inner.trim_start_matches(|ch: char| ch.is_ascii_digit());
            index.len() < inner.len() && index.starts_with(']')
        });
        if kept_index {
            out.push_str(ARGUMENTS_PLACEHOLDER);
        } else {
            out.push_str(arguments);
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
#[path = "skill_invocation_tests.rs"]
mod tests;
