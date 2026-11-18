//! Rule files: the Markdown files under a `rules` directory, some of them limited to
//! files matching the globs in their `paths:` frontmatter.

use std::collections::HashSet;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use ignore::gitignore::GitignoreBuilder;

use crate::markdown::parse_document;
use crate::markdown::strip_block_comments;
use crate::source::FileSource;

/// Bounds the walk through nested rule folders.
const MAX_RULES_DEPTH: usize = 16;

/// One rule file, ready to be shown to the model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleFile {
    pub path: PathBuf,
    /// The globs the rule is limited to; `None` for a rule that always applies.
    pub globs: Option<Vec<String>>,
    /// The body, without frontmatter and block comments.
    pub content: String,
}

/// Loads every `*.md` file under `rules_dir`, subfolders included, sorted by path. Files
/// with an empty body are left out.
pub async fn load_rules(source: &dyn FileSource, rules_dir: &Path) -> io::Result<Vec<RuleFile>> {
    let mut files = Vec::new();
    let mut pending = vec![(rules_dir.to_path_buf(), 0usize)];
    let mut visited = HashSet::new();
    while let Some((dir, depth)) = pending.pop() {
        if !visited.insert(dir.clone()) {
            continue;
        }
        for (path, is_dir) in source.list(&dir).await? {
            if is_dir {
                if depth < MAX_RULES_DEPTH {
                    pending.push((path, depth + 1));
                }
            } else if path.extension().is_some_and(|extension| extension == "md") {
                files.push(path);
            }
        }
    }
    files.sort();

    let mut rules = Vec::new();
    for path in files {
        let Some(contents) = source.read(&path).await? else {
            continue;
        };
        let document = parse_document(&contents);
        let content = strip_block_comments(document.body).trim().to_string();
        if content.is_empty() {
            continue;
        }
        rules.push(RuleFile {
            path,
            globs: document.frontmatter.and_then(rule_globs),
            content,
        });
    }
    Ok(rules)
}

/// Reads `paths:` from a rule's frontmatter: a comma-separated string or a YAML list,
/// brace-expanded. A trailing `/**` is dropped from each glob, and a rule whose globs
/// all match everything is not conditional, so `None` is returned for it.
pub fn rule_globs(frontmatter: &str) -> Option<Vec<String>> {
    let raw = match serde_yaml::from_str::<serde_yaml::Value>(frontmatter) {
        Ok(value) => paths_from_yaml(value.get("paths")?),
        // Unquoted globs such as `paths: **/*.rs, *.md` are not valid YAML.
        Err(_) => paths_from_lines(frontmatter)?,
    };
    let globs = raw
        .iter()
        .flat_map(|pattern| expand_braces(pattern))
        .map(|pattern| {
            pattern
                .strip_suffix("/**")
                .map(str::to_string)
                .unwrap_or(pattern)
        })
        .filter(|pattern| !pattern.is_empty())
        .collect::<Vec<_>>();
    if globs.is_empty() || globs.iter().all(|pattern| pattern == "**") {
        return None;
    }
    Some(globs)
}

fn paths_from_yaml(value: &serde_yaml::Value) -> Vec<String> {
    match value {
        serde_yaml::Value::String(text) => split_outside_braces(text),
        serde_yaml::Value::Sequence(items) => items
            .iter()
            .filter_map(serde_yaml::Value::as_str)
            .flat_map(split_outside_braces)
            .collect(),
        _ => Vec::new(),
    }
}

fn paths_from_lines(frontmatter: &str) -> Option<Vec<String>> {
    let mut lines = frontmatter.lines();
    let inline = lines
        .by_ref()
        .find_map(|line| line.strip_prefix("paths:"))?
        .trim();
    if !inline.is_empty() {
        let inline = inline.trim_start_matches('[').trim_end_matches(']');
        return Some(split_outside_braces(inline));
    }
    Some(
        lines
            .map_while(|line| line.trim_start().strip_prefix("- "))
            .flat_map(split_outside_braces)
            .collect(),
    )
}

/// Splits on commas that are not inside `{}` and trims quotes and blanks.
fn split_outside_braces(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for character in text.chars() {
        match character {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(character);
    }
    parts.push(current);
    parts
        .iter()
        .map(|part| part.trim().trim_matches(['"', '\'']).trim().to_string())
        .filter(|part| !part.is_empty())
        .collect()
}

/// Expands `a/{b,c}/d` into `a/b/d` and `a/c/d`, nested groups included.
fn expand_braces(pattern: &str) -> Vec<String> {
    let Some(open) = pattern.find('{') else {
        return vec![pattern.to_string()];
    };
    let mut depth = 0usize;
    let mut close = None;
    for (index, character) in pattern[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(open + index);
                    break;
                }
            }
            _ => {}
        }
    }
    let Some(close) = close else {
        return vec![pattern.to_string()];
    };
    let (head, tail) = (&pattern[..open], &pattern[close + 1..]);
    let inner = &pattern[open + 1..close];
    let mut alternatives = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    for character in inner.chars() {
        match character {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                alternatives.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(character);
    }
    alternatives.push(current);
    if alternatives.len() < 2 {
        // `{x}` is not a group: leave it for the glob matcher.
        return expand_braces(tail)
            .into_iter()
            .map(|rest| format!("{head}{{{inner}}}{rest}"))
            .collect();
    }
    alternatives
        .iter()
        .flat_map(|alternative| expand_braces(&format!("{head}{alternative}{tail}")))
        .collect()
}

/// Whether `file` matches one of `globs`, read as gitignore patterns relative to `base`.
/// A file outside `base` never matches.
pub fn glob_matches(globs: &[String], base: &Path, file: &Path) -> bool {
    let Ok(relative) = file.strip_prefix(base) else {
        return false;
    };
    if relative.as_os_str().is_empty() {
        return false;
    }
    let mut builder = GitignoreBuilder::new(base);
    for glob in globs {
        if builder.add_line(None, glob).is_err() {
            return false;
        }
    }
    let Ok(matcher) = builder.build() else {
        return false;
    };
    matcher
        .matched_path_or_any_parents(relative, /*is_dir*/ false)
        .is_ignore()
}

#[cfg(test)]
#[path = "rules_tests.rs"]
mod tests;
