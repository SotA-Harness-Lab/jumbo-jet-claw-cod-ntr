// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: marked the remaining Windows-only code;
// follow more of Claude Code's behaviour

use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::future::Future;
use std::io::ErrorKind;
// WINDOWS-REMNANT
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::Path;
#[cfg(not(unix))]
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;
use std::time::Instant;

use async_channel::Sender;
use codex_protocol::shell_environment::is_non_inheritable_env_var;
#[cfg(unix)]
use codex_utils_pty::Command;
// WINDOWS-REMNANT
#[cfg(windows)]
use codex_utils_pty::JobObject;
use futures::future::Either;
use futures::future::select;
use futures::future::try_join;
use serde_json::Value;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
#[cfg(not(unix))]
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::timeout;
use tracing::Span;

// WINDOWS-REMNANT
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use super::ClaudeHooksEngine;
use super::CommandShell;
use super::ConfiguredHandler;
use super::HandlerRunResult;
use super::dispatcher::ParsedHandler;
use super::dispatcher::hook_event_name_label;
use super::dispatcher::hook_execution_mode_label;
use super::dispatcher::hook_handler_type_label;
use super::dispatcher::hook_scope_label;
use super::dispatcher::hook_source_label;
use super::dispatcher::scope_for_event;
use crate::output_spill::HookOutputSpiller;
use codex_protocol::ThreadId;
use codex_protocol::protocol::HookCompletedEvent;
use codex_protocol::protocol::HookHandlerType;
use codex_protocol::protocol::HookOutputEntry;
use codex_protocol::protocol::HookOutputEntryKind;

const MAX_CONCURRENT_ASYNC_HOOKS: usize = 8;

/// How long output is still read once the hook's own process has exited. A process the hook
/// left in the background (`some-daemon &`) may keep the pipes open for as long as it runs, so
/// the hook ends shortly after its process does, as in Claude Code, not at end of file.
const OUTPUT_GRACE_AFTER_EXIT: Duration = Duration::from_millis(200);

/// Owns command execution and bounded asynchronous work for one session.
#[derive(Clone)]
pub(crate) struct CommandHookRuntime {
    pub(crate) environment_executor: Option<Arc<dyn crate::HookEnvironmentExecutor>>,
    shell: CommandShell,
    environment: Arc<Vec<(OsString, OsString)>>,
    result_sender: Sender<HookCompletedEvent>,
    state: Arc<Mutex<CommandHookRuntimeState>>,
    output_spiller: HookOutputSpiller,
}

struct CommandHookRuntimeState {
    concurrency_limit: Arc<Semaphore>,
    tasks: JoinSet<()>,
}

impl Default for CommandHookRuntimeState {
    fn default() -> Self {
        Self {
            concurrency_limit: Arc::new(Semaphore::new(MAX_CONCURRENT_ASYNC_HOOKS)),
            tasks: JoinSet::new(),
        }
    }
}

impl CommandHookRuntime {
    pub(crate) fn new(
        shell: CommandShell,
        environment: Arc<Vec<(OsString, OsString)>>,
        thread_id: ThreadId,
        result_sender: Sender<HookCompletedEvent>,
    ) -> Self {
        Self {
            environment_executor: None,
            shell,
            environment,
            result_sender,
            state: Arc::new(Mutex::new(CommandHookRuntimeState::default())),
            output_spiller: HookOutputSpiller::new(thread_id),
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, CommandHookRuntimeState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub(crate) fn reconfigured(&self, shell: CommandShell) -> Self {
        Self {
            environment_executor: self.environment_executor.clone(),
            shell,
            environment: Arc::clone(&self.environment),
            result_sender: self.result_sender.clone(),
            state: Arc::clone(&self.state),
            output_spiller: self.output_spiller.clone(),
        }
    }

    /// This runtime with `/bin/sh -c` as its shell, as Claude Code runs its hooks.
    pub(crate) fn with_posix_sh(&self) -> Self {
        self.reconfigured(CommandShell {
            program: "/bin/sh".to_string(),
            args: vec!["-c".to_string()],
        })
    }

    pub(crate) fn output_spiller(&self) -> &HookOutputSpiller {
        &self.output_spiller
    }

    pub(crate) fn schedule_async_hook<T: 'static>(
        &self,
        engine: ClaudeHooksEngine,
        handler: ConfiguredHandler,
        event_input: Value,
        cwd: std::path::PathBuf,
        turn_id: Option<String>,
        parse: impl Fn(&ConfiguredHandler, HandlerRunResult, Option<String>) -> ParsedHandler<T>
        + Send
        + 'static,
    ) {
        if self.result_sender.is_closed() {
            return;
        }

        let result_sender = self.result_sender.clone();
        let runtime = self.clone();
        self.schedule_async_task(async move {
            let result = super::dispatcher::execute_handler(
                &engine,
                &handler,
                &event_input,
                &cwd,
                /*metadata*/ None,
            )
            .await;
            let mut hook_result = parse(&handler, result, turn_id).completed;
            let mut entries = Vec::new();
            let mut warnings = Vec::new();

            for entry in std::mem::take(&mut hook_result.run.entries) {
                match entry.kind {
                    HookOutputEntryKind::Context => {
                        let text = runtime
                            .output_spiller
                            .maybe_spill_text_with_limit(
                                entry.text,
                                handler.additional_context_limit,
                            )
                            .await;
                        entries.push(HookOutputEntry {
                            kind: HookOutputEntryKind::Context,
                            text,
                        });
                    }
                    HookOutputEntryKind::Warning => warnings.push(entry),
                    HookOutputEntryKind::Error => entries.push(entry),
                    HookOutputEntryKind::Stop | HookOutputEntryKind::Feedback => {}
                }
            }

            entries.extend(warnings);
            hook_result.run.entries = entries;
            let _ = result_sender.try_send(hook_result);
        });
    }

    pub(crate) fn schedule_async_task(&self, task: impl Future<Output = ()> + Send + 'static) {
        let mut state = self.lock_state();
        if state.concurrency_limit.is_closed() {
            return;
        }

        while state.tasks.try_join_next().is_some() {}
        let concurrency_limit = Arc::clone(&state.concurrency_limit);
        state.tasks.spawn(async move {
            let Ok(_permit) = concurrency_limit.acquire_owned().await else {
                return;
            };
            task.await;
        });
    }

    pub(crate) async fn shutdown(&self) {
        let mut tasks = {
            let mut state = self.lock_state();
            state.concurrency_limit.close();
            std::mem::take(&mut state.tasks)
        };
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
    }
}

#[tracing::instrument(
    name = "codex.hooks.command",
    level = "trace",
    skip_all,
    fields(
        hook.event_name = hook_event_name_label(handler.event_name),
        hook.handler_type = hook_handler_type_label(HookHandlerType::Command),
        hook.execution_mode = hook_execution_mode_label(handler.execution_mode()),
        hook.scope = hook_scope_label(scope_for_event(handler.event_name)),
        hook.source = hook_source_label(handler.source),
        hook.display_order = handler.display_order,
        hook.timeout_sec = handler.timeout_sec,
        hook.command_outcome = tracing::field::Empty,
    )
)]
pub(crate) async fn run_command(
    runtime: &CommandHookRuntime,
    handler: &ConfiguredHandler,
    command_line: &str,
    env: &HashMap<String, String>,
    input_json: &str,
    cwd: &Path,
) -> HandlerRunResult {
    let started_at = chrono::Utc::now().timestamp();
    let started = Instant::now();
    let timeout_started = tokio::time::Instant::now();
    let duration = Duration::from_secs(handler.timeout_sec);
    if let Some(trust) = &handler.plugin_trust {
        let error = match timeout(duration, trust.verify()).await {
            Ok(Ok(())) => None,
            Ok(Err(err)) => Some(err.to_string()),
            Err(_) => Some(format!("hook timed out after {}s", handler.timeout_sec)),
        };
        if let Some(error) = error {
            return finish_command_run(
                started_at,
                started,
                CommandRunCompletion {
                    exit_code: None,
                    stdout: String::new(),
                    stderr: String::new(),
                    error: Some(error),
                    outcome: "trust_error",
                },
            );
        }
    }

    if let super::HandlerSourcePath::Environment { environment_id, .. } = &handler.source_path {
        let result = match &runtime.environment_executor {
            Some(executor) => {
                executor
                    .execute(
                        environment_id,
                        command_line,
                        env,
                        input_json,
                        Duration::from_secs(handler.timeout_sec),
                    )
                    .await
            }
            None => Err(anyhow::anyhow!("environment hook executor is unavailable")),
        };
        let (exit_code, stdout, stderr, error) = match result {
            Ok(output) => (Some(output.exit_code), output.stdout, output.stderr, None),
            Err(error) => (None, String::new(), String::new(), Some(error.to_string())),
        };
        return HandlerRunResult {
            structured_content: None,
            context_metadata: codex_protocol::models::ContentItemMetadata::command_hook(),
            started_at,
            completed_at: chrono::Utc::now().timestamp(),
            duration_ms: started.elapsed().as_millis().try_into().unwrap_or(i64::MAX),
            exit_code,
            stdout,
            stderr,
            error,
        };
    }

    let mut command = build_command(&runtime.shell, command_line, &runtime.environment, env);
    command.current_dir(cwd);

    // WINDOWS-REMNANT
    #[cfg(windows)]
    let (child, process_tree_job) = match JobObject::spawn_background(&mut command) {
        Ok((child, job)) => (Ok(child), job),
        Err(error) => (Err(error), None),
    };
    // WINDOWS-REMNANT
    #[cfg(not(windows))]
    let child = command.spawn();

    let mut child = match child {
        Ok(child) => child,
        Err(err) => {
            return finish_command_run(
                started_at,
                started,
                CommandRunCompletion {
                    exit_code: None,
                    stdout: String::new(),
                    stderr: String::new(),
                    error: Some(err.to_string()),
                    outcome: "spawn_error",
                },
            );
        }
    };

    let mut process_tree_guard = ProcessTreeGuard {
        process_id: child.id(),
        // WINDOWS-REMNANT
        #[cfg(windows)]
        job: process_tree_job,
    };

    let stdin = child.stdin.take();
    let write_stdin = async {
        if let Some(mut stdin) = stdin
            && let Err(err) = stdin.write_all(input_json.as_bytes()).await
            && err.kind() != ErrorKind::BrokenPipe
        {
            return Err(("stdin_error", format!("failed to write hook stdin: {err}")));
        }
        Ok(())
    };
    let mut stdout_pipe = child.stdout.take();
    let mut stderr_pipe = child.stderr.take();
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    // Drain output while sending input so neither pipe can block the other, and
    // include stdin writes in the deadline even when the hook never reads them.
    // The hook is done when its own process exits: input not yet taken is dropped
    // and output is read for a short grace period only, since a background process
    // it started may hold the pipes open.
    let wait_with_output = async {
        let exited = async {
            let write_stdin = std::pin::pin!(write_stdin);
            let wait = std::pin::pin!(child.wait());
            let status = match select(write_stdin, wait).await {
                Either::Left((written, wait)) => {
                    written?;
                    wait.await
                }
                // The hook exited without taking all of its input.
                Either::Right((status, _)) => status,
            };
            status.map_err(|err| ("wait_error", err.to_string()))
        };
        let exited = std::pin::pin!(exited);
        let drain_output = std::pin::pin!(try_join(
            drain_pipe(stdout_pipe.as_mut(), &mut stdout),
            drain_pipe(stderr_pipe.as_mut(), &mut stderr),
        ));
        match select(drain_output, exited).await {
            Either::Left((drained, exited)) => {
                drained.map_err(|err| ("wait_error", err.to_string()))?;
                exited.await
            }
            Either::Right((status, drain_output)) => {
                let status = status?;
                if let Ok(drained) = timeout(OUTPUT_GRACE_AFTER_EXIT, drain_output).await {
                    drained.map_err(|err| ("wait_error", err.to_string()))?;
                }
                Ok(status)
            }
        }
    };
    let outcome = timeout(
        duration.saturating_sub(timeout_started.elapsed()),
        wait_with_output,
    )
    .await;

    match outcome {
        Ok(Ok(status)) => {
            // Successfully completed hooks may intentionally leave detached helpers running.
            // WINDOWS-REMNANT
            #[cfg(windows)]
            if let Some(job) = process_tree_guard.job.as_ref() {
                let _ = job.preserve_descendants();
            }
            process_tree_guard.process_id = None;
            finish_command_run(
                started_at,
                started,
                CommandRunCompletion {
                    exit_code: status.code(),
                    stdout: String::from_utf8_lossy(&stdout).to_string(),
                    stderr: String::from_utf8_lossy(&stderr).to_string(),
                    error: None,
                    outcome: "completed",
                },
            )
        }
        Ok(Err((outcome, error))) => finish_command_run(
            started_at,
            started,
            CommandRunCompletion {
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(error),
                outcome,
            },
        ),
        Err(_) => finish_command_run(
            started_at,
            started,
            CommandRunCompletion {
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(format!("hook timed out after {}s", handler.timeout_sec)),
                outcome: "timeout",
            },
        ),
    }
}

/// Appends what `pipe` yields to `buf` until end of file. What was read stays in `buf` when
/// the future is dropped before that.
async fn drain_pipe<R: AsyncRead + Unpin>(
    pipe: Option<&mut R>,
    buf: &mut Vec<u8>,
) -> std::io::Result<()> {
    let Some(pipe) = pipe else {
        return Ok(());
    };
    let mut chunk = [0u8; 8192];
    loop {
        let read = pipe.read(&mut chunk).await?;
        if read == 0 {
            return Ok(());
        }
        buf.extend_from_slice(&chunk[..read]);
    }
}

// Needed only until command hooks move to the exec server, which owns process-tree cleanup.
struct ProcessTreeGuard {
    process_id: Option<u32>,
    // WINDOWS-REMNANT
    #[cfg(windows)]
    job: Option<JobObject>,
}

impl Drop for ProcessTreeGuard {
    fn drop(&mut self) {
        let Some(process_id) = self.process_id else {
            return;
        };

        #[cfg(unix)]
        {
            let _ = codex_utils_pty::process_group::kill_process_group(process_id);
        }

        // WINDOWS-REMNANT
        #[cfg(windows)]
        {
            if let Some(job) = self.job.as_ref() {
                let _ = job.terminate();
            } else {
                let _ = std::process::Command::new("taskkill")
                    .args(["/PID", &process_id.to_string(), "/T", "/F"])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn();
            }
        }
    }
}

struct CommandRunCompletion {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    error: Option<String>,
    outcome: &'static str,
}

fn finish_command_run(
    started_at: i64,
    started: Instant,
    completion: CommandRunCompletion,
) -> HandlerRunResult {
    Span::current().record("hook.command_outcome", completion.outcome);
    HandlerRunResult {
        structured_content: None,
        context_metadata: codex_protocol::models::ContentItemMetadata::command_hook(),
        started_at,
        completed_at: chrono::Utc::now().timestamp(),
        duration_ms: started.elapsed().as_millis().try_into().unwrap_or(i64::MAX),
        exit_code: completion.exit_code,
        stdout: completion.stdout,
        stderr: completion.stderr,
        error: completion.error,
    }
}

fn build_command(
    shell: &CommandShell,
    command_line: &str,
    environment: &[(OsString, OsString)],
    env: &HashMap<String, String>,
) -> Command {
    let mut command = if shell.program.is_empty() {
        Command::new(default_shell_program(environment))
    } else {
        Command::new(&shell.program)
    };
    if shell.program.is_empty() {
        // WINDOWS-REMNANT
        #[cfg(windows)]
        command.arg("/C");
        // WINDOWS-REMNANT
        #[cfg(not(windows))]
        command.arg("-lc");
    } else {
        command.args(&shell.args);
    }
    // WINDOWS-REMNANT
    #[cfg(windows)]
    if shell.program.is_empty() || shell.args.iter().any(|arg| arg.eq_ignore_ascii_case("/c")) {
        command.raw_arg(format!(r#""{command_line}""#));
    } else {
        command.arg(command_line);
    }
    // WINDOWS-REMNANT
    #[cfg(not(windows))]
    command.arg(command_line);

    #[cfg(unix)]
    command.process_mode(codex_utils_pty::ProcessMode::NewSession);
    // WINDOWS-REMNANT
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    #[cfg(not(unix))]
    command
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    // Both launchers start with an empty environment. Replay the session snapshot
    // before hook overrides, filtering restricted names from both sources.
    command.envs(
        environment
            .iter()
            .map(|(key, value)| (key.as_os_str(), value.as_os_str()))
            .chain(
                env.iter()
                    .map(|(key, value)| (OsStr::new(key), OsStr::new(value))),
            )
            .filter(|(key, _)| !key.to_str().is_some_and(is_non_inheritable_env_var)),
    );
    command
}

fn default_shell_program(environment: &[(OsString, OsString)]) -> OsString {
    // WINDOWS-REMNANT
    #[cfg(windows)]
    let (environment_variable, fallback_program) = ("COMSPEC", "cmd.exe");

    // WINDOWS-REMNANT
    #[cfg(not(windows))]
    let (environment_variable, fallback_program) = ("SHELL", "/bin/sh");

    environment
        .iter()
        .find(|(key, _)| {
            // WINDOWS-REMNANT
            #[cfg(windows)]
            {
                key.to_str()
                    .is_some_and(|key| key.eq_ignore_ascii_case(environment_variable))
            }

            // WINDOWS-REMNANT
            #[cfg(not(windows))]
            {
                key == OsStr::new(environment_variable)
            }
        })
        .map(|(_, value)| value.clone())
        .unwrap_or_else(|| OsString::from(fallback_program))
}

#[cfg(test)]
#[path = "command_runner_tests.rs"]
mod tests;
