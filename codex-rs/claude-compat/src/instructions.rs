// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Instruction files (`CLAUDE.md` and unconditional rules), loaded and rendered the way
//! Claude Code shows them to the model.

use std::collections::HashSet;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use crate::CLAUDE_DIR;
use crate::markdown::import_paths;
use crate::markdown::parse_document;
use crate::markdown::strip_block_comments;
use crate::rules::glob_matches;
use crate::rules::load_rules;
use crate::source::FileSource;

/// The line Claude Code puts above the instruction files.
pub const INSTRUCTIONS_HEADER: &str = "Codebase and user instructions are shown below. Be sure to adhere to these instructions. IMPORTANT: These instructions OVERRIDE any default behavior and you MUST follow them exactly as written.";

const CLAUDE_MD: &str = "CLAUDE.md";
const CLAUDE_LOCAL_MD: &str = "CLAUDE.local.md";
const RULES_DIR: &str = "rules";
/// How deep `@` imports may nest.
const MAX_IMPORT_DEPTH: usize = 4;
/// Extensions an imported file may have; one without an extension is accepted too.
const IMPORT_EXTENSIONS: &[&str] = &[
    "md",
    "markdown",
    "mdx",
    "txt",
    "text",
    "rst",
    "adoc",
    "org",
    "json",
    "jsonc",
    "json5",
    "yaml",
    "yml",
    "toml",
    "ini",
    "cfg",
    "conf",
    "env",
    "xml",
    "html",
    "htm",
    "css",
    "scss",
    "csv",
    "tsv",
    "log",
    "lock",
    "sh",
    "bash",
    "zsh",
    "fish",
    "ps1",
    "py",
    "rs",
    "go",
    "js",
    "jsx",
    "ts",
    "tsx",
    "mjs",
    "cjs",
    "java",
    "kt",
    "swift",
    "c",
    "h",
    "cc",
    "cpp",
    "hpp",
    "cs",
    "rb",
    "php",
    "pl",
    "lua",
    "r",
    "sql",
    "graphql",
    "proto",
    "tex",
    "nix",
    "dockerfile",
    "makefile",
    "gradle",
];

/// Where an instruction file comes from, which decides how it is introduced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionKind {
    /// The user's own files under `~/.claude`.
    User,
    /// Files checked into a project.
    Project,
    /// `CLAUDE.local.md`, the user's private notes on a project.
    Local,
    /// `MEMORY.md` of the project's auto memory.
    AutoMemory,
}

impl InstructionKind {
    fn description(self) -> &'static str {
        match self {
            Self::User => " (user's private global instructions for all projects)",
            Self::Project => " (project instructions, checked into the codebase)",
            Self::Local => " (user's private project instructions, not checked in)",
            Self::AutoMemory => " (user's auto-memory, persists across conversations)",
        }
    }
}

/// One instruction file with its imports already resolved into separate entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionFile {
    pub path: PathBuf,
    pub kind: InstructionKind,
    pub content: String,
}

/// Renders files as `Contents of <path> (<kind>):` sections, without the header.
pub fn render_instruction_files(files: &[InstructionFile]) -> String {
    files
        .iter()
        .map(|file| {
            format!(
                "Contents of {}{}:\n\n{}",
                file.path.display(),
                file.kind.description(),
                file.content
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The user's global instructions: `<claude_home>/CLAUDE.md`, then the rules under
/// `<claude_home>/rules` that are not limited to some files.
pub async fn user_instructions(
    source: &dyn FileSource,
    claude_home: &Path,
    home_dir: Option<&Path>,
) -> io::Result<Vec<InstructionFile>> {
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    // The user's own file may import from anywhere.
    load_instruction_file(
        source,
        &claude_home.join(CLAUDE_MD),
        InstructionKind::User,
        home_dir,
        /*import_root*/ None,
        &mut seen,
        &mut files,
    )
    .await?;
    push_unconditional_rules(
        source,
        &claude_home.join(RULES_DIR),
        InstructionKind::User,
        home_dir,
        /*import_root*/ None,
        &mut seen,
        &mut files,
    )
    .await?;
    Ok(files)
}

/// The instructions one directory contributes: `CLAUDE.md`, `.claude/CLAUDE.md`, the
/// unconditional rules under `.claude/rules`, then `CLAUDE.local.md`. Imports may not
/// leave `import_root`. In the home directory, `.claude` is the user's configuration and
/// is left to [`user_instructions`].
pub async fn directory_instructions(
    source: &dyn FileSource,
    dir: &Path,
    home_dir: Option<&Path>,
    import_root: &Path,
    seen: &mut HashSet<PathBuf>,
) -> io::Result<Vec<InstructionFile>> {
    let mut files = Vec::new();
    let dot_claude = dir.join(CLAUDE_DIR);
    // In the home directory, `.claude` holds the user's own files, loaded as such.
    let paths = if home_dir == Some(dir) {
        vec![dir.join(CLAUDE_MD)]
    } else {
        vec![dir.join(CLAUDE_MD), dot_claude.join(CLAUDE_MD)]
    };
    for path in paths {
        load_instruction_file(
            source,
            &path,
            InstructionKind::Project,
            home_dir,
            Some(import_root),
            seen,
            &mut files,
        )
        .await?;
    }
    if home_dir != Some(dir) {
        push_unconditional_rules(
            source,
            &dot_claude.join(RULES_DIR),
            InstructionKind::Project,
            home_dir,
            Some(import_root),
            seen,
            &mut files,
        )
        .await?;
    }
    load_instruction_file(
        source,
        &dir.join(CLAUDE_LOCAL_MD),
        InstructionKind::Local,
        home_dir,
        Some(import_root),
        seen,
        &mut files,
    )
    .await?;
    Ok(files)
}

/// The instructions Claude Code adds when the model reads `file`, beyond those loaded at
/// the start: the user's rules whose `paths:` match it (relative to `cwd`); for each
/// directory below `cwd` down to the file's own, its `CLAUDE.md`, `.claude/CLAUDE.md`,
/// `CLAUDE.local.md`, unconditional rules and matching conditional rules; then, for each
/// directory from the top of the filesystem down to `cwd`, its matching conditional rules.
/// Project files are left out unless `include_project`.
pub async fn file_instructions(
    source: &dyn FileSource,
    user_rules_dir: Option<&Path>,
    home_dir: Option<&Path>,
    cwd: &Path,
    file: &Path,
    include_project: bool,
) -> io::Result<Vec<InstructionFile>> {
    let file = file_seen_from(cwd, file);
    let file = file.as_path();
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    if let Some(rules_dir) = user_rules_dir {
        push_matching_rules(
            source,
            rules_dir,
            cwd,
            file,
            InstructionKind::User,
            &mut seen,
            &mut files,
        )
        .await?;
    }
    if !include_project {
        return Ok(files);
    }

    let file_dir = file.parent().unwrap_or(file);
    let mut nested = file_dir
        .ancestors()
        .take_while(|dir| *dir != cwd)
        .filter(|dir| dir.starts_with(cwd))
        .collect::<Vec<_>>();
    nested.reverse();
    for dir in nested {
        let dot_claude = dir.join(CLAUDE_DIR);
        for (path, kind) in [
            (dir.join(CLAUDE_MD), InstructionKind::Project),
            (dot_claude.join(CLAUDE_MD), InstructionKind::Project),
            (dir.join(CLAUDE_LOCAL_MD), InstructionKind::Local),
        ] {
            load_instruction_file(
                source,
                &path,
                kind,
                home_dir,
                Some(cwd),
                &mut seen,
                &mut files,
            )
            .await?;
        }
        let rules_dir = dot_claude.join(RULES_DIR);
        push_unconditional_rules(
            source,
            &rules_dir,
            InstructionKind::Project,
            home_dir,
            Some(cwd),
            &mut seen,
            &mut files,
        )
        .await?;
        push_matching_rules(
            source,
            &rules_dir,
            dir,
            file,
            InstructionKind::Project,
            &mut seen,
            &mut files,
        )
        .await?;
    }

    let mut cwd_level = cwd
        .ancestors()
        .filter(|dir| dir.parent().is_some() && home_dir != Some(*dir))
        .collect::<Vec<_>>();
    cwd_level.reverse();
    for dir in cwd_level {
        push_matching_rules(
            source,
            &dir.join(CLAUDE_DIR).join(RULES_DIR),
            dir,
            file,
            InstructionKind::Project,
            &mut seen,
            &mut files,
        )
        .await?;
    }
    Ok(files)
}

/// `file` as the instruction loaders compare it with `cwd` and its directories: `.` and
/// `..` resolved, and, when it is then not under `cwd`, its real path written from the
/// nearest of `cwd`'s directories it lies under, so that a file reached through a symlink
/// (or a `cwd` that is one) still counts as inside.
fn file_seen_from(cwd: &Path, file: &Path) -> PathBuf {
    let file = normalize(file);
    if file.starts_with(cwd) {
        return file;
    }
    // A file about to be written does not exist yet; its directory does.
    let real_file =
        std::fs::canonicalize(&file).or_else(|err| match (file.parent(), file.file_name()) {
            (Some(parent), Some(name)) => {
                std::fs::canonicalize(parent).map(|parent| parent.join(name))
            }
            _ => Err(err),
        });
    let Ok(real_file) = real_file else {
        return file;
    };
    for dir in cwd.ancestors() {
        if let Ok(real_dir) = std::fs::canonicalize(dir)
            && let Ok(rest) = real_file.strip_prefix(&real_dir)
        {
            return dir.join(rest);
        }
    }
    real_file
}

async fn push_matching_rules(
    source: &dyn FileSource,
    rules_dir: &Path,
    base: &Path,
    file: &Path,
    kind: InstructionKind,
    seen: &mut HashSet<PathBuf>,
    files: &mut Vec<InstructionFile>,
) -> io::Result<()> {
    for rule in load_rules(source, rules_dir).await? {
        let Some(globs) = &rule.globs else {
            continue;
        };
        if glob_matches(globs, base, file) && seen.insert(rule.path.clone()) {
            files.push(InstructionFile {
                path: rule.path,
                kind,
                content: rule.content,
            });
        }
    }
    Ok(())
}

/// The rules under `rules_dir` that are not limited to some files, each followed by the
/// files it imports with `@path`, as Claude Code loads a rule like any instruction file.
/// Imports may not leave `import_root`, when there is one.
#[allow(clippy::too_many_arguments)]
async fn push_unconditional_rules(
    source: &dyn FileSource,
    rules_dir: &Path,
    kind: InstructionKind,
    home_dir: Option<&Path>,
    import_root: Option<&Path>,
    seen: &mut HashSet<PathBuf>,
    files: &mut Vec<InstructionFile>,
) -> io::Result<()> {
    for rule in load_rules(source, rules_dir).await? {
        if rule.globs.is_none() && !seen.contains(&rule.path) {
            load_instruction_file(source, &rule.path, kind, home_dir, import_root, seen, files)
                .await?;
        }
    }
    Ok(())
}

/// Loads `path` and the files it imports with `@path`, the importing file first. Missing
/// files, files already in `seen` and imports more than four files down the chain are skipped,
/// as in Claude Code.
/// With an `import_root`, imports outside it are skipped as well.
pub async fn load_instruction_file(
    source: &dyn FileSource,
    path: &Path,
    kind: InstructionKind,
    home_dir: Option<&Path>,
    import_root: Option<&Path>,
    seen: &mut HashSet<PathBuf>,
    files: &mut Vec<InstructionFile>,
) -> io::Result<()> {
    // Depth-first with an explicit stack; each entry's imports are pushed in reverse so
    // that they come out in the order they are written.
    let mut stack = vec![(path.to_path_buf(), 0usize)];
    while let Some((path, depth)) = stack.pop() {
        if !seen.insert(path.clone()) {
            continue;
        }
        let Some(contents) = source.read(&path).await? else {
            continue;
        };
        let body = strip_block_comments(parse_document(&contents).body);
        if depth < MAX_IMPORT_DEPTH {
            let base = path.parent().unwrap_or(Path::new("/"));
            let imports = import_paths(&body);
            let imports = imports
                .iter()
                .filter_map(|import| resolve_import(import, base, home_dir))
                .filter(|import| import_root.is_none_or(|root| import.starts_with(root)))
                .filter(|import| has_text_extension(import));
            stack.extend(imports.rev().map(|import| (import, depth + 1)));
        }
        let content = body.trim();
        if !content.is_empty() {
            files.push(InstructionFile {
                path,
                kind,
                content: content.to_string(),
            });
        }
    }
    Ok(())
}

fn resolve_import(import: &str, base: &Path, home_dir: Option<&Path>) -> Option<PathBuf> {
    let path = if let Some(rest) = import.strip_prefix("~/") {
        home_dir?.join(rest)
    } else if import.starts_with('/') {
        PathBuf::from(import)
    } else {
        base.join(import)
    };
    Some(normalize(&path))
}

/// Resolves `.` and `..` without touching the filesystem.
fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn has_text_extension(path: &Path) -> bool {
    match path.extension().and_then(|extension| extension.to_str()) {
        None => true,
        Some(extension) => IMPORT_EXTENSIONS.contains(&extension.to_ascii_lowercase().as_str()),
    }
}

#[cfg(test)]
#[path = "instructions_tests.rs"]
mod tests;
