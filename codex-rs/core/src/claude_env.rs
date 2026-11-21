// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code compatibility: the variables Claude Code's `SessionStart` hooks write into
//! `CLAUDE_ENV_FILE`, for the session's later commands.

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::PoisonError;

use codex_claude_compat::EnvAssignment;

use crate::session::session::Session;

/// The assignments of the session's hook files, as last read.
#[derive(Default)]
struct HookEnv(Mutex<Vec<EnvAssignment>>);

/// Reads what the session's `SessionStart` hooks wrote into their files.
pub(crate) fn load_session_start_env(session: &Session) {
    if !codex_claude_compat::enabled() {
        return;
    }
    let dir = codex_claude_compat::hook_env_dir(&session.session_id().to_string());
    let variables = codex_claude_compat::read_hook_env_dir(&dir);
    if variables.is_empty() {
        return;
    }
    let env = session
        .services
        .session_extension_data
        .get_or_init::<HookEnv>(HookEnv::default);
    // The directory holds every file the session's hooks wrote, so what it holds now
    // replaces what was read before.
    *env.0.lock().unwrap_or_else(PoisonError::into_inner) = variables;
}

/// Adds those variables to the environment of a command, where they win, as the file's text
/// runs before the command in Claude Code; a value naming a variable (`$PATH`) takes it from
/// the command's environment.
pub(crate) fn apply_session_start_env(session: &Session, env: &mut HashMap<String, String>) {
    if let Some(variables) = session.services.session_extension_data.get::<HookEnv>() {
        codex_claude_compat::apply_env_assignments(
            &variables.0.lock().unwrap_or_else(PoisonError::into_inner),
            env,
        );
    }
}
