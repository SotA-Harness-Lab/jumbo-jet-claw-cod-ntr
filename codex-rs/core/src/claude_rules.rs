// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: instructions that apply to a file once the model works on it.
//!
//! Claude Code loads rules limited to some files by `paths:`, and the `CLAUDE.md` files of
//! directories below the working directory, only when the model reads a file they cover,
//! and shows each of them once per session. Codex does the same after a tool call: the
//! files a shell command reads and the files a patch changes count as worked on (Claude
//! Code's own editing tools require the file to have been read first).

use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use codex_claude_compat::InstructionFile;
use codex_claude_compat::LocalFiles;
use codex_claude_compat::file_instructions;
use codex_claude_compat::render_instruction_files;
use codex_claude_compat::rendered_instruction_paths;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::parse_command::ParsedCommand;
use codex_shell_command::parse_command::parse_command;
use serde::Deserialize;

use crate::hook_runtime::record_additional_contexts;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;

const APPLY_PATCH_FILE_MARKERS: &[&str] = &["*** Add File: ", "*** Update File: ", "*** Move to: "];

/// Claude Code's size limit for an instruction file, in characters.
const MAX_ON_DEMAND_CHARACTERS: usize = 40_000;

/// Instruction files already shown in this session.
#[derive(Default)]
struct ShownFiles(Mutex<HashSet<PathBuf>>);

/// Forgets which instruction files were shown, after the conversation was compacted: their
/// text is gone from it, so Claude Code shows them again the next time a file they cover
/// is worked on.
pub(crate) fn forget_shown_files(session: &crate::session::session::Session) {
    if let Some(shown) = session.services.session_extension_data.get::<ShownFiles>() {
        shown
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
}

#[derive(Deserialize)]
struct BashPaths {
    command: String,
}

#[derive(Deserialize)]
struct FilePathArgs {
    file_path: String,
}

#[derive(Deserialize)]
struct ExecCommandPaths {
    cmd: String,
    #[serde(default)]
    workdir: Option<String>,
}

/// Shows the model the Claude Code instructions that apply to the files `invocation`
/// worked on and that it has not seen yet in this session.
pub(crate) async fn record_for_tool(invocation: &ToolInvocation) {
    if cfg!(test) || !codex_claude_compat::enabled() {
        return;
    }
    let Some(environment) = invocation.turn.initial_environments.primary() else {
        return;
    };
    // Instruction files are read from this host, so a remote executor is out of reach.
    if environment.environment().is_remote() {
        return;
    }
    let Ok(cwd) = environment.cwd().to_abs_path() else {
        return;
    };
    let files = touched_files(invocation, cwd.as_path());
    if files.is_empty() {
        return;
    }

    let home_dir = dirs::home_dir();
    let user_rules_dir = home_dir
        .as_ref()
        .map(|home| home.join(codex_claude_compat::CLAUDE_DIR).join("rules"));
    let include_project = !invocation.turn.config.active_project.is_untrusted();
    let mut found = Vec::new();
    for file in files {
        match file_instructions(
            &LocalFiles,
            user_rules_dir.as_deref(),
            home_dir.as_deref(),
            cwd.as_path(),
            &file,
            include_project,
        )
        .await
        {
            Ok(instructions) => found.extend(instructions),
            Err(err) => tracing::warn!(
                "failed to read Claude Code instructions for `{}`: {err}",
                file.display()
            ),
        }
    }

    let extension_data = &invocation.session.services.session_extension_data;
    let shown = match extension_data.get::<ShownFiles>() {
        Some(shown) => shown,
        None => {
            // A resumed or forked session already holds the files shown before.
            let history = invocation.session.clone_history().await;
            let seed = shown_in_history(history.raw_items());
            extension_data.get_or_init::<ShownFiles>(|| ShownFiles(Mutex::new(seed)))
        }
    };
    let new = {
        let mut shown = shown
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        found
            .into_iter()
            .filter(|instruction| shown.insert(instruction.path.clone()))
            .map(cut_to_limit)
            .collect::<Vec<_>>()
    };
    if new.is_empty() {
        return;
    }
    record_additional_contexts(
        &invocation.session,
        &invocation.turn,
        vec![codex_hooks::HookContext::harness(render_instruction_files(
            &new,
        ))],
    )
    .await;
}

/// The instruction files the developer messages of `history` show, read back from their
/// headings. After a compaction the history holds only what was shown since.
fn shown_in_history<'a>(history: impl Iterator<Item = &'a ResponseItem>) -> HashSet<PathBuf> {
    history
        .filter_map(|item| match item {
            ResponseItem::Message { role, content, .. } if role == "developer" => Some(content),
            _ => None,
        })
        .flatten()
        .filter_map(|content| match content {
            ContentItem::InputText { text } => Some(text),
            _ => None,
        })
        .flat_map(|text| rendered_instruction_paths(text))
        .collect()
}

/// Keeps the start of a file over Claude Code's size limit for an instruction file, so
/// that a generated, very large `CLAUDE.md` does not fill the context each time a file
/// below it is worked on, and again after each compaction.
fn cut_to_limit(mut instruction: InstructionFile) -> InstructionFile {
    if let Some((end, _)) = instruction
        .content
        .char_indices()
        .nth(MAX_ON_DEMAND_CHARACTERS)
    {
        instruction.content.truncate(end);
        instruction.content.push_str(&format!(
            "\n\n(Cut: only the first {MAX_ON_DEMAND_CHARACTERS} characters of this file are shown.)"
        ));
    }
    instruction
}

/// The files a shell command read or a patch changed, as absolute paths.
fn touched_files(invocation: &ToolInvocation, cwd: &Path) -> Vec<PathBuf> {
    if !invocation.tool_name.is_default_namespace() {
        return Vec::new();
    }
    let mut files = Vec::new();
    match (invocation.tool_name.name.as_str(), &invocation.payload) {
        ("exec_command", ToolPayload::Function { arguments }) => {
            let Ok(args) = serde_json::from_str::<ExecCommandPaths>(arguments) else {
                return Vec::new();
            };
            let base = args
                .workdir
                .map(|workdir| cwd.join(workdir))
                .unwrap_or_else(|| cwd.to_path_buf());
            files.extend(shell_command_files(args.cmd, &base));
        }
        ("apply_patch", ToolPayload::Custom { input }) => {
            files.extend(patch_files(input).map(|path| cwd.join(path)));
        }
        // Claude Code's own tools, offered with `[claude] tools` in ccn-config.toml.
        ("Bash", ToolPayload::Function { arguments }) => {
            let Ok(args) = serde_json::from_str::<BashPaths>(arguments) else {
                return Vec::new();
            };
            files.extend(shell_command_files(args.command, cwd));
        }
        ("Read" | "Edit" | "Write", ToolPayload::Function { arguments }) => {
            if let Ok(args) = serde_json::from_str::<FilePathArgs>(arguments) {
                files.push(cwd.join(args.file_path));
            }
        }
        _ => {}
    }
    let mut seen = HashSet::new();
    files.retain(|file| seen.insert(file.clone()));
    files
}

/// The files a shell script reads, as Codex's command parser recognises them (`cat`,
/// `sed -n`, `head` and the like), and the one file a `grep` or `rg` of a single file
/// searches, which Claude Code counts as viewing it.
fn shell_command_files(script: String, base: &Path) -> Vec<PathBuf> {
    // The parser follows `cd` for the files it reads, not for those it searches.
    let has_cd = shlex::split(&script).is_some_and(|words| words.iter().any(|word| word == "cd"));
    let command = ["bash".to_string(), "-lc".to_string(), script];
    let mut files = Vec::new();
    for parsed in parse_command(&command) {
        match parsed {
            ParsedCommand::Read { path, .. } => files.push(base.join(path)),
            // The parser shortens a search's path for display, so it is read from `cmd`.
            ParsedCommand::Search { cmd, .. } if !has_cd => {
                files.extend(
                    searched_file(&cmd)
                        .map(|path| base.join(path))
                        .filter(|path| path.is_file()),
                );
            }
            _ => {}
        }
    }
    files
}

/// The path operand of `grep`, `egrep`, `fgrep` or `rg` when there is exactly one.
fn searched_file(cmd: &str) -> Option<String> {
    const VALUE_FLAGS: &[&str] = &[
        "-m",
        "--max-count",
        "-A",
        "--after-context",
        "-B",
        "--before-context",
        "-C",
        "--context",
        "-g",
        "--glob",
        "--iglob",
        "-t",
        "--type",
        "--type-add",
        "--type-not",
        "--max-depth",
    ];
    let words = shlex::split(cmd)?;
    let (program, args) = words.split_first()?;
    if !matches!(program.as_str(), "grep" | "egrep" | "fgrep" | "rg") {
        return None;
    }
    let mut has_pattern = false;
    let mut operands = Vec::new();
    let mut after_double_dash = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if after_double_dash {
            operands.push(arg);
        } else if matches!(arg.as_str(), "|" | "||" | "&&" | ";" | "&") {
            break;
        } else if arg == "--" {
            after_double_dash = true;
        } else if matches!(arg.as_str(), "-e" | "--regexp" | "-f" | "--file") {
            has_pattern = true;
            args.next();
        } else if VALUE_FLAGS.contains(&arg.as_str()) {
            args.next();
        } else if !arg.starts_with('-') {
            operands.push(arg);
        }
    }
    let paths = if has_pattern {
        &operands[..]
    } else {
        operands.get(1..)?
    };
    match paths {
        [path] => Some(path.to_string()),
        _ => None,
    }
}

fn patch_files(patch: &str) -> impl Iterator<Item = &str> {
    patch.lines().filter_map(|line| {
        APPLY_PATCH_FILE_MARKERS
            .iter()
            .find_map(|marker| line.strip_prefix(marker))
            .map(str::trim)
            .filter(|path| !path.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::MAX_ON_DEMAND_CHARACTERS;
    use super::cut_to_limit;
    use super::patch_files;
    use super::searched_file;
    use super::shell_command_files;
    use super::shown_in_history;
    use crate::context::ContextualUserFragment;
    use crate::context::HookAdditionalContext;
    use codex_claude_compat::InstructionFile;
    use codex_claude_compat::InstructionKind;
    use codex_claude_compat::render_instruction_files;
    use codex_protocol::models::ResponseItem;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn files_shown_before_a_resume_are_read_back_from_the_history() {
        let shown = InstructionFile {
            path: "/repo/sub/CLAUDE.md".into(),
            kind: InstructionKind::Project,
            content: "sub doc".to_string(),
        };
        let message: ResponseItem = ContextualUserFragment::into(HookAdditionalContext::new(
            codex_hooks::HookContext::harness(render_instruction_files(&[shown])),
        ));

        assert_eq!(
            shown_in_history([message].iter()),
            HashSet::from([PathBuf::from("/repo/sub/CLAUDE.md")])
        );
    }

    #[test]
    fn a_file_over_the_size_limit_is_cut_to_its_start() {
        let file = |content: String| InstructionFile {
            path: "/repo/sub/CLAUDE.md".into(),
            kind: InstructionKind::Project,
            content,
        };
        let small = file("é".repeat(MAX_ON_DEMAND_CHARACTERS));
        assert_eq!(cut_to_limit(small.clone()), small);

        let cut = cut_to_limit(file(format!(
            "{}tail",
            "é".repeat(MAX_ON_DEMAND_CHARACTERS)
        )));
        assert_eq!(
            cut.content,
            format!(
                "{}\n\n(Cut: only the first {MAX_ON_DEMAND_CHARACTERS} characters of this file \
                 are shown.)",
                "é".repeat(MAX_ON_DEMAND_CHARACTERS)
            )
        );
    }

    #[test]
    fn a_search_of_one_file_counts_as_reading_it() {
        assert_eq!(
            searched_file("grep -n TODO core/src/spawn.rs"),
            Some("core/src/spawn.rs".to_string())
        );
        assert_eq!(
            searched_file("rg -e TODO -g '*.rs' src/a.rs"),
            Some("src/a.rs".to_string())
        );
        assert_eq!(searched_file("grep -R TODO src lib"), None);
        assert_eq!(searched_file("grep TODO"), None);
        assert_eq!(searched_file("cat a.rs"), None);

        let dir = tempfile::tempdir().expect("temp dir");
        std::fs::create_dir(dir.path().join("src")).expect("create");
        std::fs::write(dir.path().join("src/a.rs"), "x").expect("write");
        assert_eq!(
            shell_command_files("grep -n x src/a.rs".to_string(), dir.path()),
            vec![dir.path().join("src/a.rs")]
        );
        // A directory is searched, not viewed.
        assert_eq!(
            shell_command_files("grep -rn x src".to_string(), dir.path()),
            Vec::<std::path::PathBuf>::new()
        );
    }

    #[test]
    fn patch_files_lists_added_updated_and_moved_files() {
        let patch = "*** Begin Patch\n*** Add File: a.rs\n+x\n*** Update File: src/b.rs\n\
                     *** Move to: src/c.rs\n@@\n-y\n+z\n*** Delete File: d.rs\n*** End Patch\n";
        assert_eq!(
            patch_files(patch).collect::<Vec<_>>(),
            vec!["a.rs", "src/b.rs", "src/c.rs"]
        );
    }
}
