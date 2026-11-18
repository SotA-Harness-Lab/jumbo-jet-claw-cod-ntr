//! Claude Code's auto memory: `~/.claude/projects/<project>/memory/MEMORY.md`, an index the
//! model keeps for a project, shown at the start of every session.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::source::FileSource;

const MAX_INDEX_LINES: usize = 200;
const MAX_INDEX_BYTES: usize = 25_000;
/// Longer project names get a hash suffix in Claude Code, which Codex cannot reproduce.
const MAX_PROJECT_NAME_LEN: usize = 200;

/// The directory of a project's auto memory, `<claude_home>/projects/<name>/memory`, the
/// name being the project's path with every character but ASCII letters and digits
/// replaced by `-`. `None` for a path too long to be named that way.
pub fn auto_memory_dir(claude_home: &Path, project_root: &Path) -> Option<PathBuf> {
    let name = project_root
        .to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>();
    (name.len() <= MAX_PROJECT_NAME_LEN)
        .then(|| claude_home.join("projects").join(name).join("memory"))
}

/// The project whose memory a session in `cwd` uses: the main checkout of the Git
/// repository `cwd` is in (also from a worktree), or `cwd` itself outside one.
pub async fn auto_memory_project_root(source: &dyn FileSource, cwd: &Path) -> io::Result<PathBuf> {
    for dir in cwd.ancestors() {
        let dot_git = dir.join(".git");
        let Some((_, is_dir)) = source
            .list(dir)
            .await?
            .into_iter()
            .find(|(path, _)| path == &dot_git)
        else {
            continue;
        };
        if is_dir {
            return Ok(dir.to_path_buf());
        }
        if let Some(contents) = source.read(&dot_git).await? {
            // A worktree's `.git` file points into the main checkout's `.git/worktrees`.
            let main = contents
                .lines()
                .find_map(|line| line.strip_prefix("gitdir:"))
                .map(str::trim)
                .and_then(|gitdir| gitdir.split_once("/.git/worktrees/"))
                .map(|(main, _)| PathBuf::from(main));
            return Ok(main.unwrap_or_else(|| dir.to_path_buf()));
        }
    }
    Ok(cwd.to_path_buf())
}

/// Cuts the index as Claude Code does, to 200 lines and 25,000 bytes, with a note when cut.
pub fn truncate_memory_index(raw: &str) -> String {
    let trimmed = raw.trim();
    let lines = trimmed.split('\n').collect::<Vec<_>>();
    let too_many_lines = lines.len() > MAX_INDEX_LINES;
    let too_long = trimmed.len() > MAX_INDEX_BYTES;
    if !too_many_lines && !too_long {
        return trimmed.to_string();
    }
    let mut cut = if too_many_lines {
        lines[..MAX_INDEX_LINES].join("\n")
    } else {
        trimmed.to_string()
    };
    if cut.len() > MAX_INDEX_BYTES {
        let mut end = MAX_INDEX_BYTES;
        while !cut.is_char_boundary(end) {
            end -= 1;
        }
        let end = cut[..end].rfind('\n').filter(|at| *at > 0).unwrap_or(end);
        cut.truncate(end);
    }
    let reason = match (too_many_lines, too_long) {
        (false, true) => format!(
            "{} bytes (limit: {MAX_INDEX_BYTES}) — index entries are too long",
            trimmed.len()
        ),
        (true, false) => format!("{} lines (limit: {MAX_INDEX_LINES})", lines.len()),
        _ => format!("{} lines and {} bytes", lines.len(), trimmed.len()),
    };
    format!(
        "{cut}\n\n> WARNING: MEMORY.md is {reason}. Only part of it was loaded. Keep index entries to one line under ~200 chars; move detail into topic files."
    )
}

#[cfg(test)]
#[path = "memory_tests.rs"]
mod tests;
