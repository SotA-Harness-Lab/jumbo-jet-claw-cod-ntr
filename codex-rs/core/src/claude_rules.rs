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
use std::sync::Arc;
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
use codex_utils_output_truncation::approx_token_count;
use serde::Deserialize;

use crate::hook_runtime::record_additional_contexts;
use crate::session::step_context::StepContext;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;

const APPLY_PATCH_FILE_MARKERS: &[&str] = &["*** Add File: ", "*** Update File: ", "*** Move to: "];

/// Claude Code's size limit for an instruction file, in characters.
const MAX_ON_DEMAND_CHARACTERS: usize = 40_000;

/// Instruction files already shown in this session, or left out of it as too large for
/// the context left.
#[derive(Default)]
struct ShownFiles(Mutex<HashSet<PathBuf>>);

/// Forgets which instruction files were shown, after the conversation was compacted: their
/// text is gone from it, so Claude Code shows them again the next time a file they cover
/// is worked on. The project's instructions, `MEMORY.md` included, are read again at the
/// next request, as Claude Code reads them after a compaction, so that edits made during
/// the session are seen.
pub(crate) async fn forget_shown_files(session: &crate::session::session::Session) {
    if let Some(shown) = session.services.session_extension_data.get::<ShownFiles>() {
        shown
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }
    if codex_claude_compat::enabled() {
        session.services.agents_md_manager.forget_repository().await;
    }
}

/// The step a compaction rebuilds the context from, with the project's instructions read
/// from disk again: Claude Code gives the context that follows a compaction the project's
/// `CLAUDE.md` and `MEMORY.md` as they are now, not as they were when the step began, so
/// that edits made during the session are in it from the start.
pub(crate) async fn with_fresh_instructions(
    session: &crate::session::session::Session,
    step_context: Arc<StepContext>,
) -> Arc<StepContext> {
    if !codex_claude_compat::enabled() {
        return step_context;
    }
    reload_instructions(session, step_context).await
}

async fn reload_instructions(
    session: &crate::session::session::Session,
    step_context: Arc<StepContext>,
) -> Arc<StepContext> {
    let manager = &session.services.agents_md_manager;
    manager.forget_repository().await;
    let (loaded, warnings) = manager
        .refresh(&step_context.turn.config, &step_context.environments)
        .await;
    session.emit_instruction_warnings(warnings).await;
    match loaded {
        Ok(loaded_agents_md) => Arc::new(StepContext {
            loaded_agents_md,
            ..(*step_context).clone()
        }),
        Err(err) => {
            tracing::warn!("failed to read the project instructions again for a compaction: {err}");
            step_context
        }
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
    crate::claude_skill_paths::files_worked_on(
        &invocation.session.services.session_extension_data,
        cwd.as_path(),
        &files,
        include_project,
    )
    .await;
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

    let tokens_left = crate::session::context_window::context_window_token_status(
        invocation.session.as_ref(),
        invocation.turn.as_ref(),
    )
    .await
    .base_window_tokens_remaining;
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
    let (new, too_large) = {
        let mut shown = shown
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        // A file too large is noted once, and tried again after a compaction or a resume.
        let new = found
            .into_iter()
            .filter(|instruction| shown.insert(instruction.path.clone()))
            .map(cut_to_limit)
            .collect::<Vec<_>>();
        fit_in_context(new, tokens_left)
    };
    let text = [render_instruction_files(&new), too_large_note(&too_large)]
        .into_iter()
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n");
    if text.is_empty() {
        return;
    }
    record_additional_contexts(
        &invocation.session,
        &invocation.turn,
        vec![codex_hooks::HookContext::harness(text)],
    )
    .await;
}

/// An instruction file left out because it is larger than the context left.
#[derive(Debug, PartialEq)]
struct TooLarge {
    path: PathBuf,
    tokens: usize,
    tokens_left: i64,
}

/// Keeps, in order, the files that fit in the `tokens_left` of the context once the files
/// kept before them are counted, and lists the others: attaching a file larger than what
/// is left would only force a compaction, after which it would be attached again.
fn fit_in_context(
    files: Vec<InstructionFile>,
    tokens_left: Option<i64>,
) -> (Vec<InstructionFile>, Vec<TooLarge>) {
    let Some(mut tokens_left) = tokens_left else {
        return (files, Vec::new());
    };
    let mut fit = Vec::new();
    let mut too_large = Vec::new();
    for file in files {
        let tokens = approx_token_count(&render_instruction_files(std::slice::from_ref(&file)));
        let cost = i64::try_from(tokens).unwrap_or(i64::MAX);
        if cost <= tokens_left {
            tokens_left -= cost;
            fit.push(file);
        } else {
            too_large.push(TooLarge {
                path: file.path,
                tokens,
                tokens_left,
            });
        }
    }
    (fit, too_large)
}

/// Tells the model which instruction files apply but were not attached, so that it knows
/// they exist and can read them itself when it needs them.
fn too_large_note(files: &[TooLarge]) -> String {
    files
        .iter()
        .map(|file| {
            format!(
                codex_prompt_texts::text!("core/claude_rules/too_large_note.md"),
                file.path.display(),
                file.tokens,
                file.tokens_left
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
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
    use super::TooLarge;
    use super::cut_to_limit;
    use super::fit_in_context;
    use super::patch_files;
    use super::searched_file;
    use super::shell_command_files;
    use super::shown_in_history;
    use super::too_large_note;
    use crate::context::ContextualUserFragment;
    use crate::context::HookAdditionalContext;
    use codex_claude_compat::InstructionFile;
    use codex_claude_compat::InstructionKind;
    use codex_claude_compat::render_instruction_files;
    use codex_protocol::models::ResponseItem;
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;
    use std::path::PathBuf;

    struct EditedInstructions(std::sync::Mutex<String>);

    impl codex_extension_api::UserInstructionsProvider for EditedInstructions {
        fn load_user_instructions(&self) -> codex_extension_api::LoadInstructionsFuture<'_> {
            let text = self.0.lock().expect("instructions").clone();
            Box::pin(async move {
                codex_extension_api::LoadedUserInstructions {
                    instructions: Some(codex_extension_api::Instructions { text, source: None }),
                    warnings: Vec::new(),
                }
            })
        }
    }

    #[tokio::test]
    async fn a_compaction_rebuilds_the_context_from_the_instructions_as_they_are_now() {
        let (mut session, turn) = crate::session::tests::make_session_and_context().await;
        let turn = std::sync::Arc::new(turn);
        let provider = std::sync::Arc::new(EditedInstructions(std::sync::Mutex::new(
            "BEFORE-COMPACTION-MARKER".to_string(),
        )));
        session.services.agents_md_manager =
            std::sync::Arc::new(crate::agents_md_manager::AgentsMdManager::new(
                crate::agents_md_manager::SessionInstructions {
                    user_provider: Some(provider.clone()),
                    ..Default::default()
                },
            ));
        let session = std::sync::Arc::new(session);
        let step = session
            .capture_step_context(
                std::sync::Arc::clone(&turn),
                &tokio_util::sync::CancellationToken::new(),
            )
            .await
            .expect("capture step");
        // The project's own files may follow the user's instructions.
        let marker = |step: &crate::session::step_context::StepContext| {
            let text = step.loaded_agents_md.as_ref().expect("instructions").text();
            ["BEFORE-COMPACTION-MARKER", "AFTER-COMPACTION-MARKER"]
                .into_iter()
                .filter(|wanted| text.contains(wanted))
                .collect::<Vec<_>>()
        };
        assert_eq!(marker(&step), vec!["BEFORE-COMPACTION-MARKER"]);

        *provider.0.lock().expect("instructions") = "AFTER-COMPACTION-MARKER".to_string();
        let fresh = super::reload_instructions(&session, std::sync::Arc::clone(&step)).await;

        assert_eq!(
            (marker(&step), marker(&fresh)),
            (
                vec!["BEFORE-COMPACTION-MARKER"],
                vec!["AFTER-COMPACTION-MARKER"]
            )
        );
    }

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
    fn a_file_larger_than_the_context_left_is_noted_instead_of_shown() {
        let file = |path: &str, content: String| InstructionFile {
            path: path.into(),
            kind: InstructionKind::Project,
            content,
        };
        let small = file("/repo/a/CLAUDE.md", "a".repeat(400));
        let large = file("/repo/b/CLAUDE.md", "b".repeat(4_000));
        let tokens = |file: &InstructionFile| {
            codex_utils_output_truncation::approx_token_count(&render_instruction_files(
                std::slice::from_ref(file),
            ))
        };
        let (small_tokens, large_tokens) = (tokens(&small), tokens(&large));

        let all = fit_in_context(
            vec![small.clone(), large.clone()],
            /*tokens_left*/ None,
        );
        assert_eq!(all, (vec![small.clone(), large.clone()], Vec::new()));

        let left = i64::try_from(small_tokens + 10).expect("tokens");
        assert_eq!(
            fit_in_context(vec![small.clone(), large], Some(left)),
            (
                vec![small],
                vec![TooLarge {
                    path: "/repo/b/CLAUDE.md".into(),
                    tokens: large_tokens,
                    tokens_left: 10,
                }]
            )
        );

        let note = too_large_note(&[TooLarge {
            path: "/repo/b/CLAUDE.md".into(),
            tokens: 1_000,
            tokens_left: 10,
        }]);
        assert_eq!(
            note,
            "Not shown: the instructions in /repo/b/CLAUDE.md apply to the files worked on, but \
             they take about 1000 tokens and only about 10 are left in the context. They are \
             shown when a file they cover is worked on after the conversation is compacted."
        );
        // The note is not read back as a file shown.
        let message: ResponseItem = ContextualUserFragment::into(HookAdditionalContext::new(
            codex_hooks::HookContext::harness(note),
        ));
        assert_eq!(shown_in_history([message].iter()), HashSet::new());
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
