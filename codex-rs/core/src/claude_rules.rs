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

use codex_claude_compat::LocalFiles;
use codex_claude_compat::file_instructions;
use codex_claude_compat::render_instruction_files;
use codex_protocol::parse_command::ParsedCommand;
use codex_shell_command::parse_command::parse_command;
use serde::Deserialize;

use crate::hook_runtime::record_additional_contexts;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;

const APPLY_PATCH_FILE_MARKERS: &[&str] = &["*** Add File: ", "*** Update File: ", "*** Move to: "];

/// Instruction files already shown in this session.
#[derive(Default)]
struct ShownFiles(Mutex<HashSet<PathBuf>>);

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

    let shown = invocation
        .session
        .services
        .session_extension_data
        .get_or_init::<ShownFiles>(ShownFiles::default);
    let new = {
        let mut shown = shown
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        found
            .into_iter()
            .filter(|instruction| shown.insert(instruction.path.clone()))
            .collect::<Vec<_>>()
    };
    if new.is_empty() {
        return;
    }
    record_additional_contexts(
        &invocation.session,
        &invocation.turn,
        vec![codex_hooks::HookContext::harness(
            render_instruction_files(&new),
        )],
    )
    .await;
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
            let command = ["bash".to_string(), "-lc".to_string(), args.cmd];
            for parsed in parse_command(&command) {
                if let ParsedCommand::Read { path, .. } = parsed {
                    files.push(base.join(path));
                }
            }
        }
        ("apply_patch", ToolPayload::Custom { input }) => {
            files.extend(patch_files(input).map(|path| cwd.join(path)));
        }
        _ => {}
    }
    let mut seen = HashSet::new();
    files.retain(|file| seen.insert(file.clone()));
    files
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
    use super::patch_files;
    use pretty_assertions::assert_eq;

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
