// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: read Claude Code's configuration in place; write Claude Code auto memory

//! AGENTS.md discovery and user instruction assembly.
//!
//! Project-level documentation is primarily stored in files named `AGENTS.md`.
//! Additional fallback filenames can be configured via `project_doc_fallback_filenames`.
//! Fallback entries containing path syntax for the executor's OS are ignored
//! before any filesystem probes use them.
//! We include the concatenation of all files found along the path from the
//! project root to the current working directory as follows:
//!
//! 1.  Determine the project root by walking upwards from the current working
//!     directory until a configured `project_root_markers` entry is found.
//!     When `project_root_markers` is unset, the default marker list is used
//!     (`.git`). If no marker is found, only the current working directory is
//!     considered. An empty marker list disables parent traversal.
//! 2.  Collect every `AGENTS.md` found from the project root down to the
//!     current working directory (inclusive) and concatenate their contents in
//!     that order.
//! 3.  We do **not** walk past the project root.

use crate::config::Config;
use crate::context::UserInstructions as ContextUserInstructions;
use crate::environment_selection::TurnEnvironmentSnapshot;
use codex_config::ConfigLayerSource;
use codex_config::default_project_root_markers;
use codex_config::merge_toml_values;
use codex_config::project_root_markers_from_config;
use codex_exec_server::ExecutorFileSystem;
use codex_exec_server::GetMetadataOptions;
use codex_exec_server::ReadFileOptions;
use codex_extension_api::Instructions;
use codex_file_system::FileSystemSandboxContext;
use codex_file_system::FindUpErrorPolicy;
use codex_file_system::find_nearest_ancestor_with_markers;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathConvention;
use codex_utils_path_uri::PathUri;
use futures::StreamExt;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use toml::Value as TomlValue;
use tracing::error;

/// Default filename scanned for AGENTS.md instructions.
pub const DEFAULT_AGENTS_MD_FILENAME: &str = "AGENTS.md";
/// Preferred local override for AGENTS.md instructions.
pub const LOCAL_AGENTS_MD_FILENAME: &str = "AGENTS.override.md";

/// When both user and project AGENTS.md docs are present, they will be
/// concatenated with the following separator.
const AGENTS_MD_SEPARATOR: &str = "\n\n--- project-doc ---\n\n";

const PROJECT_DOC_BUDGET_GUIDANCE: &str = "Trim your project instructions to fit the budget. If you need to keep them all, increase `project_doc_max_bytes`.";

// Metadata probes are cheap and the exec-server transport already bounds total in-flight calls.
// This covers typical project hierarchies in one remote round trip without monopolizing that
// transport when independent startup discovery runs concurrently.
const MAX_CONCURRENT_ANCESTOR_PROBES: usize = 256;

/// Loads repository instructions across the selected environments.
pub(crate) async fn load_project_instructions(
    config: &Config,
    environments: &TurnEnvironmentSnapshot,
    warnings: &mut Vec<String>,
) -> io::Result<LoadedAgentsMd> {
    let mut loaded = LoadedAgentsMd::default();
    if config.active_project.is_untrusted() || config.project_doc_max_bytes == 0 {
        return Ok(loaded);
    }

    let mut remaining = config.project_doc_max_bytes;
    for turn_environment in environments.turn_environments() {
        if remaining == 0 {
            warnings.push(format!(
                "The {}-byte project instruction budget was exhausted before checking environment `{}` or any later environments. {PROJECT_DOC_BUDGET_GUIDANCE}",
                config.project_doc_max_bytes,
                turn_environment.selection.environment_id
            ));
            break;
        }

        let filesystem = turn_environment.environment().get_filesystem();
        let sandbox = (!turn_environment
            .permission_profile()
            .file_system_sandbox_policy()
            .has_full_disk_read_access())
        .then(|| turn_environment.sandbox_context(/*additional_permissions*/ None));
        match read_agents_md(
            config,
            filesystem.as_ref(),
            &turn_environment.selection.environment_id,
            turn_environment.cwd(),
            remaining,
            sandbox.as_ref(),
            warnings,
        )
        .await
        {
            Ok(Some(docs)) => {
                for entry in docs.entries {
                    remaining = remaining.saturating_sub(entry.contents.len());
                    loaded.entries.push(entry);
                }
            }
            Ok(None) => {}
            Err(error) if sandbox.is_none() => {
                error!(
                    environment_id = turn_environment.selection.environment_id,
                    "error trying to find AGENTS.md docs: {error:#}"
                );
            }
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!(
                        "failed to load AGENTS.md instructions for environment `{}`: {error}",
                        turn_environment.selection.environment_id
                    ),
                ));
            }
        }
    }

    Ok(loaded)
}

/// Attempt to locate and load AGENTS.md documentation.
///
/// On success returns `Ok(Some(loaded))` where `loaded` contains every
/// discovered doc. If no documentation file is found the function returns
/// `Ok(None)`. Unexpected I/O failures bubble up as `Err` so callers can
/// decide how to handle them.
#[tracing::instrument(name = "agents_md.load", skip_all, fields(max_total = max_total))]
async fn read_agents_md(
    config: &Config,
    fs: &dyn ExecutorFileSystem,
    environment_id: &str,
    cwd: &PathUri,
    max_total: usize,
    sandbox: Option<&FileSystemSandboxContext>,
    warnings: &mut Vec<String>,
) -> io::Result<Option<LoadedAgentsMd>> {
    let paths = agents_md_paths(config, cwd, fs, sandbox).await?;
    let mut remaining = max_total;
    let mut loaded = LoadedAgentsMd::default();

    for p in paths {
        if remaining == 0 {
            warnings.push(format!(
                "Skipped project instructions in environment `{environment_id}` starting at `{}` because the {}-byte budget was exhausted. {PROJECT_DOC_BUDGET_GUIDANCE}",
                p.inferred_native_path_string(),
                config.project_doc_max_bytes
            ));
            break;
        }

        let mut data = match fs.read_file(&p, ReadFileOptions::default(), sandbox).await {
            Ok(data) => data,
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        };
        let size = data.len();
        if size > remaining {
            data.truncate(remaining);
            warnings.push(format!(
                "Loaded only {remaining} of {size} bytes from `{}` in environment `{environment_id}` under the {}-byte project instruction budget. {PROJECT_DOC_BUDGET_GUIDANCE}",
                p.inferred_native_path_string(),
                config.project_doc_max_bytes
            ));
            tracing::warn!(
                path = %p,
                remaining_bytes = remaining,
                "project doc exceeds remaining budget; truncating"
            );
        }

        let text = String::from_utf8_lossy(&data).to_string();
        if !text.trim().is_empty() {
            loaded.entries.push(InstructionEntry {
                contents: text,
                provenance: InstructionProvenance::Project {
                    source_path: p,
                    environment_id: environment_id.to_string(),
                    cwd: cwd.clone(),
                },
            });
            remaining -= data.len();
        }
    }

    // Unit tests run against mock filesystems and real ancestor directories; they call
    // `claude_project_entries` directly instead.
    if !cfg!(test) && codex_claude_compat::enabled() {
        loaded.entries.extend(
            claude_project_entries(
                fs,
                environment_id,
                cwd,
                sandbox,
                config.active_project.is_trusted(),
                warnings,
            )
            .await,
        );
    }

    if loaded.is_empty() {
        Ok(None)
    } else {
        Ok(Some(loaded))
    }
}

/// Claude Code's project instructions, found as Claude Code finds them: in every directory
/// from the top of the filesystem down to `cwd`, `CLAUDE.md`, `.claude/CLAUDE.md`, the
/// rules under `.claude/rules` that are not limited to some files, and `CLAUDE.local.md`.
/// Unlike AGENTS.md they are not cut to the byte budget, since Claude Code never cuts them.
async fn claude_project_entries(
    fs: &dyn ExecutorFileSystem,
    environment_id: &str,
    cwd: &PathUri,
    sandbox: Option<&FileSystemSandboxContext>,
    project_trusted: bool,
    warnings: &mut Vec<String>,
) -> Vec<InstructionEntry> {
    // An executor with other path conventions than the host's is out of reach.
    let Ok(cwd_path) = cwd.to_abs_path() else {
        return Vec::new();
    };
    let source = ExecutorFiles { fs, sandbox };
    let home_dir = dirs::home_dir();
    let mut directories = cwd_path
        .as_path()
        .ancestors()
        .filter(|directory| directory.parent().is_some())
        .collect::<Vec<_>>();
    directories.reverse();
    let mut seen = std::collections::HashSet::new();
    let mut entries = Vec::new();
    for directory in directories {
        let files = match codex_claude_compat::directory_instructions(
            &source,
            directory,
            home_dir.as_deref(),
            cwd_path.as_path(),
            &mut seen,
        )
        .await
        {
            Ok(files) => files,
            Err(err) => {
                warnings.push(format!(
                    "Failed to read Claude Code instructions in `{}`: {err}",
                    directory.display()
                ));
                continue;
            }
        };
        for file in files {
            let Ok(path) = AbsolutePathBuf::from_absolute_path_checked(&file.path) else {
                continue;
            };
            entries.push(InstructionEntry {
                contents: codex_claude_compat::render_instruction_files(std::slice::from_ref(
                    &file,
                )),
                provenance: InstructionProvenance::Project {
                    source_path: PathUri::from_abs_path(&path),
                    environment_id: environment_id.to_string(),
                    cwd: cwd.clone(),
                },
            });
        }
    }
    // Unit tests never read the real home directory's Claude Code settings; they test
    // `crate::claude_memory` with a home directory of their own.
    let memory_home = if cfg!(test) {
        None
    } else {
        home_dir.as_deref()
    };
    if let Some(memory) = crate::claude_memory::claude_auto_memory_for(
        &source,
        cwd_path.as_path(),
        memory_home,
        &crate::claude_memory::process_env,
        project_trusted,
    )
    .await
    {
        let index_path = memory.dir.join(AUTO_MEMORY_INDEX);
        // Claude Code puts the instructions in its system prompt and the index among the
        // CLAUDE.md files; Codex keeps both together, the instructions first.
        let mut contents = memory.instructions;
        if let Some(file) = auto_memory_index(&source, &index_path).await {
            contents.push_str("\n\n");
            contents.push_str(&codex_claude_compat::render_instruction_files(
                std::slice::from_ref(&file),
            ));
        }
        if let Ok(path) = AbsolutePathBuf::from_absolute_path_checked(&index_path) {
            entries.push(InstructionEntry {
                contents,
                provenance: InstructionProvenance::Project {
                    source_path: PathUri::from_abs_path(&path),
                    environment_id: environment_id.to_string(),
                    cwd: cwd.clone(),
                },
            });
        }
    }
    entries
}

/// The name of the auto memory's index in its directory.
const AUTO_MEMORY_INDEX: &str = "MEMORY.md";

/// The index of the project's Claude Code auto memory, cut as Claude Code cuts it.
async fn auto_memory_index(
    source: &ExecutorFiles<'_>,
    path: &Path,
) -> Option<codex_claude_compat::InstructionFile> {
    use codex_claude_compat::FileSource as _;
    let path = path.to_path_buf();
    let contents = source.read(&path).await.ok()??;
    let content = codex_claude_compat::truncate_memory_index(&contents);
    (!content.is_empty()).then_some(codex_claude_compat::InstructionFile {
        path,
        kind: codex_claude_compat::InstructionKind::AutoMemory,
        content,
    })
}

/// An executor's filesystem as the Claude Code loaders read it. Paths the sandbox keeps
/// out of reach read as missing.
struct ExecutorFiles<'a> {
    fs: &'a dyn ExecutorFileSystem,
    sandbox: Option<&'a FileSystemSandboxContext>,
}

impl ExecutorFiles<'_> {
    fn uri(path: &Path) -> io::Result<PathUri> {
        let path = AbsolutePathBuf::from_absolute_path_checked(path)?;
        Ok(PathUri::from_abs_path(&path))
    }
}

fn missing_or_denied(err: &io::Error) -> bool {
    matches!(
        err.kind(),
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory | io::ErrorKind::PermissionDenied
    )
}

impl codex_claude_compat::FileSource for ExecutorFiles<'_> {
    fn read<'a>(&'a self, path: &'a Path) -> codex_claude_compat::SourceFuture<'a, Option<String>> {
        Box::pin(async move {
            let uri = Self::uri(path)?;
            match self
                .fs
                .get_metadata(&uri, GetMetadataOptions::default(), self.sandbox)
                .await
            {
                Ok(metadata) if metadata.is_file => {}
                Ok(_) => return Ok(None),
                Err(err) if missing_or_denied(&err) => return Ok(None),
                Err(err) => return Err(err),
            }
            match self
                .fs
                .read_file(&uri, ReadFileOptions::default(), self.sandbox)
                .await
            {
                Ok(data) => Ok(Some(String::from_utf8_lossy(&data).into_owned())),
                Err(err) if missing_or_denied(&err) => Ok(None),
                Err(err) => Err(err),
            }
        })
    }

    fn list<'a>(
        &'a self,
        dir: &'a Path,
    ) -> codex_claude_compat::SourceFuture<'a, Vec<(PathBuf, bool)>> {
        Box::pin(async move {
            let uri = Self::uri(dir)?;
            let entries = match self.fs.read_directory(&uri, self.sandbox).await {
                Ok(entries) => entries,
                Err(err) if missing_or_denied(&err) => return Ok(Vec::new()),
                Err(err) => return Err(err),
            };
            let mut listed = Vec::new();
            for entry in entries {
                let path = dir.join(&entry.file_name);
                if entry.is_directory || entry.is_file {
                    listed.push((path, entry.is_directory));
                    continue;
                }
                // A symlink: follow it, as Claude Code does; a dangling one is skipped.
                if let Ok(metadata) = self
                    .fs
                    .get_metadata(
                        &Self::uri(&path)?,
                        GetMetadataOptions::default(),
                        self.sandbox,
                    )
                    .await
                    && (metadata.is_directory || metadata.is_file)
                {
                    listed.push((path, metadata.is_directory));
                }
            }
            Ok(listed)
        })
    }
}

/// Discovers AGENTS.md files from the project root to the current working
/// directory, inclusive. Symlinks are allowed.
#[tracing::instrument(name = "agents_md.discover", skip_all)]
async fn agents_md_paths(
    config: &Config,
    cwd: &PathUri,
    fs: &dyn ExecutorFileSystem,
    sandbox: Option<&FileSystemSandboxContext>,
) -> io::Result<Vec<PathUri>> {
    let dir = cwd.clone();

    let mut merged = TomlValue::Table(toml::map::Map::new());
    for layer in config.config_layer_stack.layers_low_to_high() {
        if matches!(layer.name, ConfigLayerSource::Project { .. }) {
            continue;
        }
        merge_toml_values(&mut merged, &layer.config);
    }
    let project_root_markers = match project_root_markers_from_config(&merged) {
        Ok(Some(markers)) => markers,
        Ok(None) => default_project_root_markers(),
        Err(err) => {
            tracing::warn!("invalid project_root_markers: {err}");
            default_project_root_markers()
        }
    };
    let project_root = find_nearest_ancestor_with_markers(
        fs,
        &dir,
        project_root_markers,
        FindUpErrorPolicy::Ignore,
        sandbox,
    )
    .await?;
    let search_dirs = if let Some(root) = project_root {
        let mut dirs = Vec::new();
        let mut cursor = dir.clone();
        loop {
            dirs.push(cursor.clone());
            if cursor == root {
                break;
            }
            let Some(parent) = cursor.parent() else {
                break;
            };
            cursor = parent;
        }
        dirs.reverse();
        dirs
    } else {
        vec![dir]
    };

    let candidate_filenames = candidate_filenames(config, cwd);
    let candidate_filenames = &candidate_filenames;
    let mut results = futures::stream::iter(search_dirs)
        .map(|directory| async move {
            for name in candidate_filenames {
                let candidate = directory
                    .join(name)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
                match fs
                    .get_metadata(&candidate, GetMetadataOptions::default(), sandbox)
                    .await
                {
                    Ok(metadata) if metadata.is_file => return Ok(Some(candidate)),
                    Ok(_) => {}
                    Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                    Err(err) => return Err(err),
                }
            }
            Ok(None)
        })
        .buffered(MAX_CONCURRENT_ANCESTOR_PROBES);
    let mut found = Vec::new();
    while let Some(result) = results.next().await {
        if let Some(candidate) = result? {
            found.push(candidate);
        }
    }
    Ok(found)
}

fn candidate_filenames<'a>(config: &'a Config, cwd: &PathUri) -> Vec<&'a str> {
    let mut names: Vec<&str> = Vec::with_capacity(2 + config.project_doc_fallback_filenames.len());
    names.push(LOCAL_AGENTS_MD_FILENAME);
    names.push(DEFAULT_AGENTS_MD_FILENAME);
    for candidate in &config.project_doc_fallback_filenames {
        let candidate = candidate.as_str();
        if candidate.is_empty() {
            continue;
        }
        // Use the executor's path convention, not the host's: resolving a Windows
        // network path can send ambient credentials even during metadata probes.
        if matches!(candidate, "." | "..")
            || candidate.contains(['/', '\0'])
            || cwd.infer_path_convention() == Some(PathConvention::Windows)
                && candidate.contains(['\\', ':'])
        {
            tracing::warn!("ignoring project_doc_fallback_filenames entry that is not a filename");
            continue;
        }
        if !names.contains(&candidate) {
            names.push(candidate);
        }
    }
    names
}

/// Model-visible instructions loaded from AGENTS.md files and internal
/// guidance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoadedAgentsMd {
    /// Account- or home-scoped instructions supplied by the host.
    pub(crate) user_instructions: Option<Instructions>,

    /// Thread-scoped instructions supplied by the host.
    pub(crate) thread_instructions: Option<Instructions>,

    /// Ordered instructions and their provenance.
    entries: Vec<InstructionEntry>,
}

impl LoadedAgentsMd {
    /// Creates loaded instructions containing one user-level AGENTS.md entry.
    pub fn new_user(contents: String, path: AbsolutePathBuf) -> Self {
        if contents.trim().is_empty() {
            return Self::default();
        }
        Self {
            user_instructions: Some(Instructions {
                text: contents,
                source: Some(path),
            }),
            thread_instructions: None,
            entries: Vec::new(),
        }
    }

    pub(crate) fn with_instructions(
        mut self,
        user_instructions: Option<Instructions>,
        thread_instructions: Option<Instructions>,
    ) -> Option<Self> {
        self.user_instructions =
            user_instructions.filter(|instructions| !instructions.text.trim().is_empty());
        self.thread_instructions =
            thread_instructions.filter(|instructions| !instructions.text.trim().is_empty());
        (!self.is_empty()).then_some(self)
    }

    /// Creates source-less user instructions for tests.
    ///
    /// This cannot be gated with `#[cfg(test)]` because integration tests
    /// compile `codex-core` as a normal dependency without that configuration.
    pub fn from_text_for_testing(contents: impl Into<String>) -> Self {
        let contents = contents.into();
        if contents.trim().is_empty() {
            return Self::default();
        }
        Self {
            user_instructions: None,
            thread_instructions: None,
            entries: vec![InstructionEntry {
                contents,
                provenance: InstructionProvenance::Internal,
            }],
        }
    }

    fn is_empty(&self) -> bool {
        self.user_instructions.is_none()
            && self.thread_instructions.is_none()
            && self
                .entries
                .iter()
                .all(|entry| entry.contents.trim().is_empty())
    }

    /// Returns the concatenated model-visible instruction text.
    pub fn text(&self) -> String {
        if self.has_multiple_project_environments() {
            self.environment_labeled_text()
        } else {
            self.legacy_text()
        }
    }

    fn legacy_text(&self) -> String {
        let mut output = String::new();
        let mut has_previous = false;
        let mut previous_was_project = false;
        for instructions in self
            .user_instructions
            .iter()
            .chain(self.thread_instructions.iter())
        {
            if has_previous {
                output.push_str("\n\n");
            }
            output.push_str(&instructions.text);
            has_previous = true;
        }
        for entry in &self.entries {
            let is_project = matches!(&entry.provenance, InstructionProvenance::Project { .. });
            if has_previous {
                // The project-doc marker tells the model where workspace-scoped
                // instructions begin, so it is only needed on the transition
                // from user or internal instructions to project instructions.
                let separator = if is_project && !previous_was_project {
                    AGENTS_MD_SEPARATOR
                } else {
                    "\n\n"
                };
                output.push_str(separator);
            }
            output.push_str(&entry.contents);
            has_previous = true;
            previous_was_project = is_project;
        }
        output
    }

    fn environment_labeled_text(&self) -> String {
        let mut output = String::new();
        let mut has_previous = false;
        let mut previous_environment: Option<(&str, &PathUri)> = None;
        for instructions in self
            .user_instructions
            .iter()
            .chain(self.thread_instructions.iter())
        {
            if has_previous {
                output.push_str("\n\n");
            }
            output.push_str(&instructions.text);
            has_previous = true;
        }
        for entry in &self.entries {
            match &entry.provenance {
                InstructionProvenance::Project {
                    environment_id,
                    cwd,
                    ..
                } => {
                    if has_previous {
                        output.push_str("\n\n");
                    }
                    // One environment can contribute several hierarchical AGENTS.md files from
                    // its project root through its cwd. Label that environment once for the
                    // complete group rather than repeating the label before every file.
                    let environment = (environment_id.as_str(), cwd);
                    if previous_environment != Some(environment) {
                        output.push_str(&format!(
                            "for `{}` with root {}\n\n",
                            environment_id,
                            cwd.inferred_native_path_string()
                        ));
                    }
                    output.push_str(&entry.contents);
                    previous_environment = Some(environment);
                }
                InstructionProvenance::Internal => {
                    if has_previous {
                        output.push_str("\n\n");
                    }
                    output.push_str(&entry.contents);
                    previous_environment = None;
                }
            }
            has_previous = true;
        }
        output
    }

    pub(crate) fn contextual_user_fragment(&self) -> ContextUserInstructions {
        // One contributing project environment retains the legacy cwd wrapper. With two or more,
        // the body labels every contributing environment itself, so the outer cwd is omitted.
        let directory = if self.has_multiple_project_environments() {
            None
        } else {
            self.single_project_cwd()
                .map(PathUri::inferred_native_path_string)
        };
        ContextUserInstructions {
            directory,
            text: self.text(),
        }
    }

    /// Returns the AGENTS.md files that supplied instruction entries.
    pub fn sources(&self) -> impl Iterator<Item = PathUri> + '_ {
        self.user_instructions
            .iter()
            .chain(self.thread_instructions.iter())
            .filter_map(|instructions| instructions.source.as_ref().map(PathUri::from_abs_path))
            .chain(
                self.entries
                    .iter()
                    .filter_map(|entry| entry.provenance.path()),
            )
    }

    fn has_multiple_project_environments(&self) -> bool {
        let mut first_environment_id = None;
        self.entries.iter().any(|entry| {
            let InstructionProvenance::Project { environment_id, .. } = &entry.provenance else {
                return false;
            };
            match first_environment_id {
                Some(first_environment_id) => first_environment_id != environment_id,
                None => {
                    first_environment_id = Some(environment_id);
                    false
                }
            }
        })
    }

    fn single_project_cwd(&self) -> Option<&PathUri> {
        self.entries
            .iter()
            .find_map(|entry| match &entry.provenance {
                InstructionProvenance::Project { cwd, .. } => Some(cwd),
                InstructionProvenance::Internal => None,
            })
    }
}

/// One model-visible instruction and its provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
struct InstructionEntry {
    /// Model-visible instruction text.
    contents: String,

    /// Origin of the instruction.
    provenance: InstructionProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InstructionProvenance {
    /// Workspace instructions discovered from project AGENTS.md files.
    Project {
        /// Exact AGENTS.md file, distinct from the environment's selected cwd.
        source_path: PathUri,
        environment_id: String,
        cwd: PathUri,
    },

    /// Instructions without a file source, including internally defined guidance.
    Internal,
}

impl InstructionProvenance {
    fn path(&self) -> Option<PathUri> {
        match self {
            Self::Project { source_path, .. } => Some(source_path.clone()),
            Self::Internal => None,
        }
    }
}

#[cfg(test)]
#[path = "agents_md_tests.rs"]
mod tests;
