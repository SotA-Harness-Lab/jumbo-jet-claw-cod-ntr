// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The working directory a `Bash` call's `cd` leaves for the next call, as in Claude Code:
//! it carries over while it stays inside the project (the session's working directory);
//! a command that ends outside puts the next one back in the project, and its result says
//! so. Subagents, like Claude Code's, always start in the project.
//!
//! The command reports where it ended through a file written after it (see
//! `codex_claude_compat::tools::bash::cwd_trailer`), which the exec runtime adds after
//! approval and lets the sandbox write. Commands on a remote executor are not followed.
//! The relative paths and default directories of `Read`, `Glob`, `Grep` and the other file
//! tools follow the same directory, as Claude Code's do.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::PoisonError;

use codex_protocol::protocol::SessionSource;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;

use crate::session::session::Session;
use crate::session::turn_context::TurnEnvironment;
use crate::tools::context::ToolInvocation;

/// The directory the next `Bash` call starts in, when it is not the project.
struct Carried {
    environment_id: String,
    project: AbsolutePathBuf,
    cwd: AbsolutePathBuf,
}

#[derive(Default)]
struct ShellCwd(Mutex<Option<Carried>>);

/// Files the exec runtime writes a `Bash` command's final directory to, by call id.
#[derive(Default)]
struct CwdReports(Mutex<HashMap<String, AbsolutePathBuf>>);

/// Takes the file a `Bash` call asked its command to report its directory to, if any.
pub(crate) fn take_cwd_report(session: &Session, call_id: &str) -> Option<AbsolutePathBuf> {
    session
        .services
        .session_extension_data
        .get_or_init::<CwdReports>(CwdReports::default)
        .0
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .remove(call_id)
}

/// Where the session's `Bash` commands start, for one call.
pub(super) struct Tracking {
    environment_id: String,
    project: AbsolutePathBuf,
    /// Where the call starts, when not in the project.
    pub(super) start: Option<AbsolutePathBuf>,
}

impl Tracking {
    /// `None` when the call's directory is not followed: in a subagent, or on a remote
    /// executor.
    pub(super) fn for_call(
        invocation: &ToolInvocation,
        environment: &TurnEnvironment,
    ) -> Option<Self> {
        if matches!(
            invocation.step_context.turn.session_source,
            SessionSource::SubAgent(_)
        ) || environment.environment().is_remote()
        {
            return None;
        }
        let project = environment.cwd().to_abs_path().ok()?;
        let environment_id = environment.selection.environment_id.clone();
        let start = carried(&invocation.session, &environment_id, &project);
        Some(Self {
            environment_id,
            project,
            start,
        })
    }

    /// Asks the exec runtime to report where the command `call_id` ends, to a file in a
    /// directory made for it; the directory is removed when the returned guard is dropped.
    pub(super) fn request_report(
        &self,
        session: &Session,
        call_id: &str,
    ) -> Option<(tempfile::TempDir, PathBuf)> {
        let directory = tempfile::Builder::new()
            .prefix("ccn-bash-cwd-")
            .tempdir()
            .ok()?;
        let file = directory.path().join("cwd");
        let absolute = AbsolutePathBuf::from_absolute_path(&file).ok()?;
        session
            .services
            .session_extension_data
            .get_or_init::<CwdReports>(CwdReports::default)
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(call_id.to_string(), absolute);
        Some((directory, file))
    }

    /// Records where a command that succeeded ended (`reported`, its `pwd -P`). Returns the
    /// message for the model when the command ended outside the project.
    pub(super) fn finish(&self, session: &Session, reported: &str) -> Option<String> {
        let project_display = self.project.as_path().display().to_string();
        let reported = Path::new(reported);
        let inside = |root: &Path| reported.starts_with(root);
        let physical_project = std::fs::canonicalize(self.project.as_path()).ok();
        let at_project = reported == self.project.as_path()
            || physical_project.as_deref() == Some(reported);
        let (next, reset) = if at_project {
            (None, false)
        } else if inside(self.project.as_path()) || physical_project.as_deref().is_some_and(inside)
        {
            (AbsolutePathBuf::from_absolute_path(reported).ok(), false)
        } else {
            (None, true)
        };
        *state(session)
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = next.map(|cwd| Carried {
            environment_id: self.environment_id.clone(),
            project: self.project.clone(),
            cwd,
        });
        reset.then(|| codex_claude_compat::tools::bash::shell_reset_message(&project_display))
    }
}

fn state(session: &Session) -> std::sync::Arc<ShellCwd> {
    session
        .services
        .session_extension_data
        .get_or_init::<ShellCwd>(ShellCwd::default)
}

/// The directory carried over in `environment_id` for `project`, if it still exists.
fn carried(
    session: &Session,
    environment_id: &str,
    project: &AbsolutePathBuf,
) -> Option<AbsolutePathBuf> {
    let state = state(session);
    let state = state.0.lock().unwrap_or_else(PoisonError::into_inner);
    state
        .as_ref()
        .filter(|carried| {
            carried.environment_id == environment_id
                && &carried.project == project
                && carried.cwd.as_path().is_dir()
        })
        .map(|carried| carried.cwd.clone())
}

/// The directory relative paths of the Claude file tools resolve against: where `Bash`'s
/// `cd` left the session, or the environment's working directory.
pub(super) fn working_directory(
    invocation: &ToolInvocation,
    environment: &TurnEnvironment,
) -> PathUri {
    Tracking::for_call(invocation, environment)
        .and_then(|tracking| tracking.start)
        .map(|cwd| PathUri::from_abs_path(&cwd))
        .unwrap_or_else(|| environment.cwd().clone())
}
