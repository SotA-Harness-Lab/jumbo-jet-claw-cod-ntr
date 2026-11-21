// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The parts of a Claude Code Markdown file that carry meaning for loading it:
//! frontmatter, `@path` imports and block-level HTML comments.

/// A Markdown file split into its YAML frontmatter and its body.
#[derive(Debug, PartialEq, Eq)]
pub struct Document<'a> {
    /// The frontmatter text between the `---` lines, when the file starts with one.
    pub frontmatter: Option<&'a str>,
    pub body: &'a str,
}

/// Splits a leading `---` frontmatter block off a file. A file whose block is never
/// closed is all body. A UTF-8 byte order mark at the start is dropped, as Claude Code
/// drops it.
pub fn parse_document(contents: &str) -> Document<'_> {
    let contents = contents.strip_prefix('\u{feff}').unwrap_or(contents);
    let whole = Document {
        frontmatter: None,
        body: contents,
    };
    let mut lines = contents.split_inclusive('\n');
    let Some(first) = lines.next().filter(|line| line.trim_end() == "---") else {
        return whole;
    };
    let start = first.len();
    let mut offset = start;
    for line in lines {
        if line.trim_end() == "---" {
            return Document {
                frontmatter: Some(&contents[start..offset]),
                body: &contents[offset + line.len()..],
            };
        }
        offset += line.len();
    }
    whole
}

/// Parses frontmatter text as YAML the way Claude Code does: when it is not valid YAML,
/// the values of top-level `key: value` lines that hold characters YAML reads as syntax
/// (`: `, `#`, `*`, `[`, `{` and the like) are put in double quotes, and the text is
/// parsed again. Unlike Claude Code, a value that is valid YAML by itself, other than as
/// a `key: value` of its own, is not quoted (a list such as `[topic]` stays a list).
/// `None` when the second parse fails too.
pub fn parse_frontmatter(frontmatter: &str) -> Option<serde_yaml::Value> {
    serde_yaml::from_str(frontmatter)
        .ok()
        .or_else(|| serde_yaml::from_str(&quote_problematic_values(frontmatter)).ok())
}

/// A frontmatter value as text: a string as it is, a bare number or boolean (`name: 3000`)
/// as written, as Claude Code turns them into strings. `None` for anything else.
pub fn frontmatter_string(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::String(text) => Some(text.clone()),
        serde_yaml::Value::Number(number) => Some(number.to_string()),
        serde_yaml::Value::Bool(flag) => Some(flag.to_string()),
        _ => None,
    }
}

/// Claude Code's repair of a frontmatter that is not valid YAML: an unindented
/// `key: value` line whose value is not quoted, holds a character YAML reads as syntax and
/// is not valid YAML by itself gets the value in double quotes.
fn quote_problematic_values(frontmatter: &str) -> String {
    frontmatter
        .split('\n')
        .map(|line| quote_problematic_value(line).unwrap_or_else(|| line.to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn quote_problematic_value(line: &str) -> Option<String> {
    let line = line.strip_suffix('\r').unwrap_or(line);
    let (key, rest) = line.split_once(':')?;
    if key.is_empty()
        || !key
            .chars()
            .all(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '-')
        || !rest.starts_with(char::is_whitespace)
    {
        return None;
    }
    let value = rest.trim_start();
    if value.is_empty()
        || (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''))
    {
        return None;
    }
    let special = value.contains(": ") || value.contains(|ch: char| "{}[]*&#!|>%@`".contains(ch));
    // A value YAML reads by itself, other than as a `key: value` of its own, is not what
    // broke the frontmatter.
    let valid_alone = match serde_yaml::from_str::<serde_yaml::Value>(value) {
        Ok(serde_yaml::Value::Mapping(_)) => value.starts_with('{'),
        Ok(_) => true,
        Err(_) => false,
    };
    if !special || valid_alone {
        return None;
    }
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    Some(format!("{key}: \"{escaped}\""))
}

/// Removes block-level `<!-- ... -->` comments: those that open at the start of a line,
/// outside fenced code. A comment that is never closed, and comments inside a line of
/// text or inside code, stay.
pub fn strip_block_comments(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut fence: Option<Fence> = None;
    let mut rest = body;
    while !rest.is_empty() {
        let line_end = rest.find('\n').map_or(rest.len(), |index| index + 1);
        let line = &rest[..line_end];
        if let Some(open) = fence {
            if open.closed_by(line) {
                fence = None;
            }
        } else if let Some(open) = Fence::opened_by(line) {
            fence = Some(open);
        } else if is_block_comment_start(line)
            && let Some(end) = rest.find("-->")
        {
            // Text after the comment on its closing line is kept.
            let after = &rest[end + "-->".len()..];
            let after_line_end = after.find('\n').map_or(after.len(), |index| index + 1);
            let tail = &after[..after_line_end];
            if !tail.trim().is_empty() {
                out.push_str(tail);
            }
            rest = &after[after_line_end..];
            continue;
        }
        out.push_str(line);
        rest = &rest[line_end..];
    }
    out
}

fn is_block_comment_start(line: &str) -> bool {
    let indent = line.len() - line.trim_start_matches(' ').len();
    indent <= 3 && line[indent..].starts_with("<!--")
}

/// Paths named by `@path` imports in a file, as written, in order. Imports inside fenced
/// code blocks and inline code spans are not imports.
pub fn import_paths(body: &str) -> Vec<String> {
    let mut paths = Vec::new();
    let mut fence: Option<Fence> = None;
    for line in body.lines() {
        if let Some(open) = fence {
            if open.closed_by(line) {
                fence = None;
            }
            continue;
        }
        if let Some(open) = Fence::opened_by(line) {
            fence = Some(open);
            continue;
        }
        collect_line_imports(&without_code_spans(line), &mut paths);
    }
    paths
}

fn collect_line_imports(line: &str, paths: &mut Vec<String>) {
    let mut chars = line.char_indices().peekable();
    let mut at_boundary = true;
    while let Some((index, character)) = chars.next() {
        if character == '@' && at_boundary {
            let mut path = String::new();
            let mut rest = line[index + 1..].chars().peekable();
            while let Some(next) = rest.next() {
                if next == '\\' && rest.peek() == Some(&' ') {
                    path.push(' ');
                    rest.next();
                    chars.next();
                } else if next.is_whitespace() || next == '\\' {
                    break;
                } else {
                    path.push(next);
                }
                chars.next();
            }
            if let Some(path) = accepted_import(&path) {
                paths.push(path);
            }
            at_boundary = false;
            continue;
        }
        at_boundary = character.is_whitespace();
    }
}

/// Applies Claude Code's rule for what an import may look like and drops a `#fragment`.
fn accepted_import(raw: &str) -> Option<String> {
    let path = raw.split('#').next().unwrap_or_default();
    if path.is_empty() {
        return None;
    }
    let accepted = path.starts_with("./")
        || path.starts_with("~/")
        || (path.starts_with('/') && path != "/")
        || path
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_alphanumeric() || matches!(first, '.' | '_' | '-'));
    accepted.then(|| path.to_string())
}

/// Blanks out inline code spans so that nothing inside them is read as an import.
fn without_code_spans(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let ticks = rest[start..].chars().take_while(|c| *c == '`').count();
        let delimiter = &rest[start..start + ticks];
        let after = &rest[start + ticks..];
        match after.find(delimiter) {
            Some(end) => {
                out.push_str(&rest[..start]);
                out.push(' ');
                rest = &after[end + ticks..];
            }
            None => break,
        }
    }
    out.push_str(rest);
    out
}

/// An open fenced code block: its marker character and length.
#[derive(Clone, Copy)]
struct Fence {
    marker: char,
    len: usize,
}

impl Fence {
    fn opened_by(line: &str) -> Option<Self> {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            return None;
        }
        let text = &line[indent..];
        let marker = text.chars().next().filter(|c| matches!(c, '`' | '~'))?;
        let len = text.chars().take_while(|c| *c == marker).count();
        (len >= 3).then_some(Self { marker, len })
    }

    fn closed_by(self, line: &str) -> bool {
        let text = line.trim();
        let len = text.chars().take_while(|c| *c == self.marker).count();
        len >= self.len && text.chars().all(|c| c == self.marker)
    }
}

#[cfg(test)]
#[path = "markdown_tests.rs"]
mod tests;
