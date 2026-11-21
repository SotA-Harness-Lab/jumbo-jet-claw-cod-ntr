// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: skills whose frontmatter has `paths:`.
//!
//! Claude Code keeps such a skill out of the model's list, and out of reach of the `Skill`
//! tool, until a file its globs cover (relative to the working directory, read as a
//! rule's `paths:`) is worked on; from then on it is offered for the rest of the session.
//! Here the skills of a step are offered with those not brought up yet disabled.

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use codex_extension_api::ExtensionData;
use codex_skills_extension::HostSkillsSnapshot;
use codex_skills_extension::SkillLoadOutcome;
use codex_utils_path_uri::PathUri;

/// How many files worked on are kept until the skills are first offered.
const MAX_PENDING_FILES: usize = 1_000;

/// The conditional skills of one session.
#[derive(Default)]
struct ConditionalSkills(Mutex<ConditionalSkillsState>);

#[derive(Default)]
struct ConditionalSkillsState {
    /// The snapshot `globs` was read from, kept so that its address stays its own.
    source: Option<Arc<HostSkillsSnapshot>>,
    /// The globs of each skill of `source` that has `paths:`.
    globs: HashMap<PathUri, Vec<String>>,
    /// Files worked on before the skills were first offered.
    touched: Vec<PathBuf>,
    /// The conditional skills a file was worked on for.
    activated: HashSet<PathUri>,
    /// The snapshot last offered for `source`, with the number of skills it disables.
    offered: Option<(usize, Arc<HostSkillsSnapshot>)>,
}

impl ConditionalSkills {
    fn state(&self) -> MutexGuard<'_, ConditionalSkillsState> {
        self.0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// Notes the files a tool call worked on, for the skills with `paths:`, and looks for the
/// `.claude/skills` folders between them and the working directory, in a trusted project.
pub(crate) async fn files_worked_on(
    session_data: &ExtensionData,
    cwd: &Path,
    files: &[PathBuf],
    include_project: bool,
) {
    {
        let conditional = session_data.get_or_init(ConditionalSkills::default);
        let mut state = conditional.state();
        if state.source.is_some() {
            let activated = activated_skills(&state.globs, &state.activated, cwd, files);
            state.activated.extend(activated);
        } else if state.touched.len() < MAX_PENDING_FILES {
            state.touched.extend(files.iter().cloned());
        }
    }
    if include_project {
        crate::claude_nested_skills::discover(session_data, cwd, files).await;
    }
}

/// The skills of `snapshot` to offer now: those with `paths:` for which no file has been
/// worked on yet in this session are disabled.
pub(crate) async fn offered_skills(
    session_data: &ExtensionData,
    cwd: &Path,
    snapshot: Arc<HostSkillsSnapshot>,
) -> Arc<HostSkillsSnapshot> {
    if !codex_claude_compat::enabled() {
        return snapshot;
    }
    offer_skills(session_data, cwd, snapshot).await
}

async fn offer_skills(
    session_data: &ExtensionData,
    cwd: &Path,
    snapshot: Arc<HostSkillsSnapshot>,
) -> Arc<HostSkillsSnapshot> {
    let conditional = session_data.get_or_init(ConditionalSkills::default);
    let known = conditional
        .state()
        .source
        .as_ref()
        .is_some_and(|source| std::ptr::eq(source.outcome(), snapshot.outcome()));
    if !known {
        let globs = read_globs(snapshot.outcome()).await;
        let mut state = conditional.state();
        state.source = Some(Arc::clone(&snapshot));
        state.globs = globs;
        state.offered = None;
    }

    let mut state = conditional.state();
    let touched = std::mem::take(&mut state.touched);
    let activated = activated_skills(&state.globs, &state.activated, cwd, &touched);
    state.activated.extend(activated);
    let hidden = state
        .globs
        .keys()
        .filter(|path| !state.activated.contains(*path))
        .cloned()
        .collect::<HashSet<_>>();
    if hidden.is_empty() {
        return snapshot;
    }
    if let Some((count, offered)) = &state.offered
        && *count == hidden.len()
    {
        return Arc::clone(offered);
    }
    let mut outcome = snapshot.outcome().clone();
    outcome.disabled_paths.extend(hidden.iter().cloned());
    let offered = Arc::new(HostSkillsSnapshot::new(Arc::new(outcome)));
    state.offered = Some((hidden.len(), Arc::clone(&offered)));
    offered
}

/// The conditional skills not yet `activated` that one of `touched` brings up.
fn activated_skills(
    globs: &HashMap<PathUri, Vec<String>>,
    activated: &HashSet<PathUri>,
    cwd: &Path,
    touched: &[PathBuf],
) -> Vec<PathUri> {
    if touched.is_empty() {
        return Vec::new();
    }
    globs
        .iter()
        .filter(|(path, _)| !activated.contains(*path))
        .filter(|(_, globs)| {
            touched
                .iter()
                .any(|file| codex_claude_compat::glob_matches(globs, cwd, file))
        })
        .map(|(path, _)| path.clone())
        .collect()
}

/// The `paths:` globs of the enabled skills of `outcome` that have them, read from their
/// `SKILL.md` on this host. A file that cannot be read here is offered from the start.
async fn read_globs(outcome: &SkillLoadOutcome) -> HashMap<PathUri, Vec<String>> {
    let mut globs = HashMap::new();
    for skill in outcome
        .skills
        .iter()
        .filter(|skill| outcome.is_skill_enabled(skill))
    {
        let Ok(path) = skill.path_to_skills_md.to_abs_path() else {
            continue;
        };
        let Ok(contents) = tokio::fs::read_to_string(path.as_path()).await else {
            continue;
        };
        if let Some(paths) = codex_claude_compat::skill_frontmatter(&contents).paths {
            globs.insert(skill.path_to_skills_md.clone(), paths);
        }
    }
    globs
}

#[cfg(test)]
#[path = "claude_skill_paths_tests.rs"]
mod tests;
