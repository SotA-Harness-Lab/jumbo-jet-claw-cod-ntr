//! Claude Code compatibility: the user's global Claude Code instructions, read in place.
//!
//! The files stay where Claude Code keeps them and are read again on every refresh, so
//! an edit there reaches Codex at the next request without a copy to keep in sync.

use std::io;
use std::path::Path;

use codex_utils_absolute_path::AbsolutePathBuf;

const CLAUDE_COMPAT_ENV: &str = "CODEX_CLAUDE_COMPAT";
const CLAUDE_HOME_DIRNAME: &str = ".claude";
const CLAUDE_MD_FILENAME: &str = "CLAUDE.md";
const RULES_DIRNAME: &str = "rules";
/// Bounds the walk through nested rule folders and through symlink loops.
const MAX_RULES_DEPTH: usize = 8;

const PREAMBLE: &str = "\
The sections below are the user's global Claude Code instructions, shared with Codex and \
binding here as well. Each heading is the file a section comes from. They are maintained \
for Claude Code: do not edit those files unless the user asks. Where they name Claude Code \
features, `/<name>` and the Skill tool mean the skill of that name, and the hooks and \
`settings.json` entries they describe do not run in Codex, so follow the rules those \
would have enforced yourself.";

/// Returns the user's Claude Code configuration directory, `~/.claude`, or nothing when
/// `CODEX_CLAUDE_COMPAT=0` turns the compatibility mode off.
pub fn default_claude_home() -> Option<AbsolutePathBuf> {
    if std::env::var_os(CLAUDE_COMPAT_ENV).is_some_and(|value| value == "0") {
        return None;
    }
    let path = dirs::home_dir()?.join(CLAUDE_HOME_DIRNAME);
    AbsolutePathBuf::from_absolute_path_checked(path).ok()
}

/// Claude Code instructions rendered for Codex.
pub(super) struct ClaudeInstructions {
    pub text: String,
    /// The first file that contributed, reported as the source when Codex has no
    /// global AGENTS.md of its own.
    pub source: AbsolutePathBuf,
}

/// Loads `CLAUDE.md` and every rule file under `rules/`, in that order, rules sorted by
/// path. Missing files are not an error; unreadable ones are reported in `warnings`.
pub(super) async fn load(
    claude_home: &AbsolutePathBuf,
    warnings: &mut Vec<String>,
) -> Option<ClaudeInstructions> {
    let mut files = vec![claude_home.join(CLAUDE_MD_FILENAME)];
    files.extend(rule_files(&claude_home.join(RULES_DIRNAME), warnings).await);

    let mut sections = Vec::new();
    let mut source = None;
    for path in files {
        let data = match tokio::fs::read(path.as_path()).await {
            Ok(data) => data,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => {
                warnings.push(read_warning(path.as_path(), &err));
                continue;
            }
        };
        let contents = String::from_utf8_lossy(&data);
        let (paths, body) = split_frontmatter(&contents);
        let body = body.trim();
        if body.is_empty() {
            continue;
        }
        let mut section = format!("## {}\n\n", path.display());
        if let Some(paths) = paths {
            section.push_str(&format!(
                "Applies only when working with files matching: {paths}\n\n"
            ));
        }
        section.push_str(body);
        sections.push(section);
        source.get_or_insert(path);
    }

    let source = source?;
    Some(ClaudeInstructions {
        text: format!("{PREAMBLE}\n\n{}", sections.join("\n\n")),
        source,
    })
}

async fn rule_files(
    rules_dir: &AbsolutePathBuf,
    warnings: &mut Vec<String>,
) -> Vec<AbsolutePathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![(rules_dir.clone(), 0usize)];
    while let Some((dir, depth)) = pending.pop() {
        let mut entries = match tokio::fs::read_dir(dir.as_path()).await {
            Ok(entries) => entries,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => {
                warnings.push(read_warning(dir.as_path(), &err));
                continue;
            }
        };
        loop {
            let entry = match entries.next_entry().await {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(err) => {
                    warnings.push(read_warning(dir.as_path(), &err));
                    break;
                }
            };
            let path = dir.join(entry.file_name());
            // Follows symlinks, as Claude Code does for linked rule files and folders.
            let Ok(metadata) = tokio::fs::metadata(path.as_path()).await else {
                continue;
            };
            if metadata.is_dir() {
                if depth < MAX_RULES_DEPTH {
                    pending.push((path, depth + 1));
                }
            } else if metadata.is_file()
                && path.as_path().extension().is_some_and(|ext| ext == "md")
            {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Splits a leading YAML frontmatter block off a rule file, returning the value of its
/// `paths:` key (the globs the rule is limited to) and the remaining body.
fn split_frontmatter(contents: &str) -> (Option<String>, &str) {
    let Some(rest) = contents.strip_prefix("---") else {
        return (None, contents);
    };
    let Some(rest) = rest
        .strip_prefix('\n')
        .or_else(|| rest.strip_prefix("\r\n"))
    else {
        return (None, contents);
    };
    let mut frontmatter_len = None;
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end() == "---" {
            frontmatter_len = Some((offset, offset + line.len()));
            break;
        }
        offset += line.len();
    }
    let Some((frontmatter_end, body_start)) = frontmatter_len else {
        return (None, contents);
    };
    (
        paths_from_frontmatter(&rest[..frontmatter_end]),
        &rest[body_start..],
    )
}

/// Reads `paths:` in either form Claude Code accepts, an inline value or a list of
/// `- glob` items, and returns the globs as written.
fn paths_from_frontmatter(frontmatter: &str) -> Option<String> {
    let mut lines = frontmatter.lines();
    let inline = lines.by_ref().find_map(|line| {
        line.strip_prefix("paths:")
            .map(|value| value.trim().to_string())
    })?;
    if !inline.is_empty() {
        return Some(inline);
    }
    let items = lines
        .map_while(|line| line.trim_start().strip_prefix("- "))
        .map(|item| item.trim().to_string())
        .collect::<Vec<_>>();
    (!items.is_empty()).then(|| items.join(", "))
}

fn read_warning(path: &Path, err: &io::Error) -> String {
    format!(
        "Failed to read Claude Code instructions from `{}`: {err}",
        path.display()
    )
}
