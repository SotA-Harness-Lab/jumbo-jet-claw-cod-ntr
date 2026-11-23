// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's auto memory: `~/.claude/projects/<project>/memory/MEMORY.md`, an index the
//! model keeps for a project, shown at the start of every session.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::permissions::normalize_path;
use crate::source::FileSource;

const MAX_INDEX_LINES: usize = 200;
const MAX_INDEX_BYTES: usize = 25_000;
/// Longer project names get a hash suffix in Claude Code, which Codex cannot reproduce.
const MAX_PROJECT_NAME_LEN: usize = 200;

/// The optional process-level name for a project directory under an alternate profile.
pub fn named_auto_memory_dir(base: &Path, name: &str) -> Option<PathBuf> {
    if name.is_empty()
        || name.len() > 64
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return None;
    }
    let lower = name.to_ascii_lowercase();
    let device = matches!(lower.as_str(), "con" | "prn" | "aux" | "nul")
        || lower
            .strip_prefix("com")
            .or_else(|| lower.strip_prefix("lpt"))
            .is_some_and(|suffix| suffix.len() == 1 && suffix.as_bytes()[0].is_ascii_digit());
    (!device).then(|| base.join("projects").join(name).join("memory"))
}

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
            let main = main_checkout(source, dir, &contents).await?;
            return Ok(main.unwrap_or_else(|| dir.to_path_buf()));
        }
    }
    Ok(cwd.to_path_buf())
}

/// The main checkout of the linked worktree `dir`, whose `.git` file holds `dot_git`, or of
/// a bare repository its common directory. As for the project's entry in `~/.claude.json`,
/// the file is followed only when it has the shape `git worktree add` gives it: its `gitdir`
/// a direct child of the common directory's `worktrees`, whose own `gitdir` file points back
/// to this `.git`. Otherwise a cloned repository could name another project's checkout and
/// so read and write that project's memory.
async fn main_checkout(
    source: &dyn FileSource,
    dir: &Path,
    dot_git: &str,
) -> io::Result<Option<PathBuf>> {
    let Some(gitdir) = dot_git.trim().strip_prefix("gitdir:").map(str::trim) else {
        return Ok(None);
    };
    let worktree_git_dir = normalize_path(&dir.join(gitdir));
    let Some(commondir) = source.read(&worktree_git_dir.join("commondir")).await? else {
        return Ok(None);
    };
    let common_dir = normalize_path(&worktree_git_dir.join(commondir.trim()));
    if worktree_git_dir.parent() != Some(common_dir.join("worktrees").as_path()) {
        return Ok(None);
    }
    let Some(backlink) = source.read(&worktree_git_dir.join("gitdir")).await? else {
        return Ok(None);
    };
    if normalize_path(&dir.join(backlink.trim())) != normalize_path(&dir.join(".git")) {
        return Ok(None);
    }
    if common_dir.file_name().is_some_and(|name| name != ".git") {
        return Ok(Some(common_dir));
    }
    Ok(common_dir.parent().map(Path::to_path_buf))
}

/// Cuts the index as Claude Code does, to 200 lines and 25,000 bytes, with a note when cut,
/// and neutralizes what could pass for the markup around it (see [`neutralize_markup`]).
pub fn truncate_memory_index(raw: &str) -> String {
    neutralize_markup(&cut_memory_index(raw))
}

/// Tags of the wrappers Codex and Claude Code put around what the model is shown.
const MARKUP_TAGS: &[&str] = &[
    // Codex's.
    "INSTRUCTIONS",
    "user_instructions",
    "environment_context",
    "environments_instructions",
    "apps_instructions",
    "catalog_apps_instructions",
    "skills_instructions",
    "plugins_instructions",
    "permissions_instructions",
    "developer_instructions",
    "managed_developer_instructions",
    "model_instructions",
    "collaboration_mode",
    "multi_agent_mode",
    "realtime_conversation",
    "context_window",
    "context_window_guidance",
    "codex_internal_context",
    "project_context",
    "startup_context",
    "goal_context",
    "persistent_mode",
    "side_conversation",
    "model_switch",
    "tools",
    "skill",
    "user_shell_command",
    "turn_aborted",
    "subagent_notification",
    // Claude Code's.
    "system-reminder",
    "command-name",
    "command-message",
    "command-args",
    "local-command-stdout",
    "local-command-stderr",
    "local-command-caveat",
    "user-prompt-submit-hook",
    "task-notification",
    "bash-input",
    "bash-stdout",
    "bash-stderr",
];

/// Characters that do not show: the zero-width space, the word joiner and the invisible
/// operators, the byte-order mark, and the bidirectional controls that can make text read
/// other than it is. The zero-width joiner and non-joiner stay, as some scripts and emoji
/// need them.
fn is_invisible(character: char) -> bool {
    matches!(
        character,
        '\u{200B}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{2069}'
            | '\u{FEFF}'
    )
}

/// Removes invisible characters and turns the `<` of a tag that imitates the markup
/// around the index (`</INSTRUCTIONS>`, `<system-reminder>`) into `&lt;`, as Claude Code
/// does for `MEMORY.md`, which holds text the model wrote, possibly from what it was shown.
fn neutralize_markup(text: &str) -> String {
    let text = text
        .chars()
        .filter(|character| !is_invisible(*character))
        .collect::<String>();
    let mut out = String::with_capacity(text.len());
    let mut rest = text.as_str();
    while let Some(at) = rest.find('<') {
        out.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let name_start = after.strip_prefix('/').unwrap_or(after);
        let name_len = name_start
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
            .unwrap_or(name_start.len());
        let (name, tail) = name_start.split_at(name_len);
        let is_tag = tail.starts_with(|c: char| c == '>' || c == '/' || c.is_whitespace());
        if is_tag && MARKUP_TAGS.iter().any(|tag| tag.eq_ignore_ascii_case(name)) {
            out.push_str("&lt;");
        } else {
            out.push('<');
        }
        rest = after;
    }
    out.push_str(rest);
    out
}

fn cut_memory_index(raw: &str) -> String {
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
        codex_prompt_texts::text!("claude-compat/memory/memory_index_warning.md"),
        cut = cut,
        reason = reason
    )
}

#[cfg(test)]
#[path = "memory_tests.rs"]
mod tests;
