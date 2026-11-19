// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Shared implementation for `codex archive`, `codex delete`, and `codex unarchive`.
//!
//! The CLI commands are thin app-server clients: resolve a user-provided UUID or exact session
//! name, then call the corresponding app-server RPC.
//! Explicit remote session commands, including queue, leave authentication and execution to
//! the selected server without initializing the caller's credentials or runtime.

use std::io::IsTerminal;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use crate::Cli;
use crate::app_server_session::AppServerSession;
use crate::app_server_session::ThreadParamsMode;
use crate::legacy_core::config::ConfigBuilder;
use crate::legacy_core::config::ConfigOverrides;
use crate::legacy_core::config::load_config_toml_with_layer_stack;
use crate::legacy_core::config::resolve_oss_provider;
use crate::legacy_core::config::resolve_profile_v2_config_path;
use crate::named_session_lookup::SessionCollection;
use crate::named_session_lookup::display_label;
use crate::named_session_lookup::lookup;
use codex_app_server_protocol::Thread as AppServerThread;
use codex_arg0::Arg0DispatchPaths;
use codex_config::CloudConfigBundleLoader;
use codex_config::ConfigLoadOptions;
use codex_config::LoaderOverrides;
use codex_exec_server::EnvironmentManager;
use codex_exec_server::ExecServerRuntimeOptions;
use codex_protocol::ThreadId;
use codex_utils_cli::CliConfigOverrides;
use codex_utils_home_dir::find_codex_home;
use codex_utils_oss::get_default_model_for_oss_provider;
use color_eyre::eyre::Result;
use color_eyre::eyre::WrapErr;
use color_eyre::eyre::eyre;

use super::RemoteAppServerEndpoint;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteConfirmation {
    Prompt,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionArchiveAction {
    Archive,
    Delete(DeleteConfirmation),
    Unarchive,
}

pub struct SessionArchiveCommandOptions {
    pub cli: Cli,
    pub arg0_paths: Arg0DispatchPaths,
    pub explicit_remote_endpoint: Option<RemoteAppServerEndpoint>,
}

fn success_message(
    action: SessionArchiveAction,
    session_id: ThreadId,
    session_name: Option<&str>,
) -> String {
    let session_id = session_id.to_string();
    match (action, session_name) {
        (SessionArchiveAction::Archive, Some(name)) => {
            crate::i18n::tr!(
                "session-archive-archived-named",
                name = name,
                id = session_id
            )
        }
        (SessionArchiveAction::Archive, None) => {
            crate::i18n::tr!("session-archive-archived", id = session_id)
        }
        (SessionArchiveAction::Delete(_), Some(name)) => {
            crate::i18n::tr!(
                "session-archive-deleted-named",
                name = name,
                id = session_id
            )
        }
        (SessionArchiveAction::Delete(_), None) => {
            crate::i18n::tr!("session-archive-deleted", id = session_id)
        }
        (SessionArchiveAction::Unarchive, Some(name)) => {
            crate::i18n::tr!(
                "session-archive-unarchived-named",
                name = name,
                id = session_id
            )
        }
        (SessionArchiveAction::Unarchive, None) => {
            crate::i18n::tr!("session-archive-unarchived", id = session_id)
        }
    }
}

struct ResolvedSessionTarget {
    session_id: ThreadId,
    session_name: Option<String>,
}

pub async fn run_session_archive_command(
    action: SessionArchiveAction,
    target: String,
    options: SessionArchiveCommandOptions,
) -> Result<String> {
    let codex_home = find_codex_home().wrap_err("failed to find Codex home")?;
    let mut app_server =
        start_app_server_for_session_command(options, codex_home.to_path_buf()).await?;
    run_session_archive_action_with_app_server(
        &mut app_server,
        codex_home.as_path(),
        action,
        &target,
    )
    .await
}

async fn run_session_archive_action_with_app_server(
    app_server: &mut AppServerSession,
    codex_home: &Path,
    action: SessionArchiveAction,
    target: &str,
) -> Result<String> {
    let resolved = resolve_session_target(app_server, codex_home, action, target).await?;
    let session_name = match action {
        SessionArchiveAction::Archive => {
            app_server.thread_archive(resolved.session_id).await?;
            resolved.session_name
        }
        SessionArchiveAction::Delete(confirmation) => {
            if matches!(confirmation, DeleteConfirmation::Prompt)
                && !confirm_session_delete(&resolved)?
            {
                return Ok(crate::i18n::tr!("session-archive-delete-cancelled").to_string());
            }
            app_server.thread_delete(resolved.session_id).await?;
            resolved.session_name
        }
        SessionArchiveAction::Unarchive => {
            let thread = app_server.thread_unarchive(resolved.session_id).await?;
            thread.name.or(resolved.session_name)
        }
    };
    Ok(success_message(
        action,
        resolved.session_id,
        session_name.as_deref(),
    ))
}

async fn resolve_session_target(
    app_server: &mut AppServerSession,
    codex_home: &Path,
    action: SessionArchiveAction,
    target: &str,
) -> Result<ResolvedSessionTarget> {
    if let Ok(session_id) = ThreadId::from_string(target) {
        if matches!(
            action,
            SessionArchiveAction::Delete(DeleteConfirmation::Prompt)
        ) {
            let thread = app_server
                .thread_read(session_id, /*include_turns*/ false)
                .await
                .with_context(|| {
                    crate::i18n::tr!(
                        "session-archive-not-found-active-or-archived",
                        target = target
                    )
                })?;
            return Ok(ResolvedSessionTarget {
                session_id,
                session_name: thread.name,
            });
        }
        return Ok(ResolvedSessionTarget {
            session_id,
            session_name: None,
        });
    }

    let (not_found_id, collections): (&str, &[SessionCollection]) = match action {
        SessionArchiveAction::Archive => (
            "session-archive-not-found-active",
            &[SessionCollection::Active],
        ),
        SessionArchiveAction::Delete(_) => (
            "session-archive-not-found-active-or-archived",
            &[SessionCollection::Active, SessionCollection::Archived],
        ),
        SessionArchiveAction::Unarchive => (
            "session-archive-not-found-archived",
            &[SessionCollection::Archived],
        ),
    };
    if let Some(thread) = lookup(
        app_server,
        codex_home,
        target,
        collections,
        &[super::resume_source_kinds(
            /*include_non_interactive*/ false,
        )],
        /*model_providers*/ None,
    )
    .await?
    {
        return session_target_from_app_server_thread(thread);
    }
    Err(eyre!(
        "{}",
        match not_found_id {
            "session-archive-not-found-active" => {
                crate::i18n::tr!("session-archive-not-found-active", target = target)
            }
            "session-archive-not-found-archived" => {
                crate::i18n::tr!("session-archive-not-found-archived", target = target)
            }
            _ => crate::i18n::tr!(
                "session-archive-not-found-active-or-archived",
                target = target
            ),
        }
    ))
}

fn session_target_from_app_server_thread(thread: AppServerThread) -> Result<ResolvedSessionTarget> {
    let session_id = ThreadId::from_string(&thread.id)
        .wrap_err_with(|| format!("app server returned invalid session id `{}`", thread.id))?;
    Ok(ResolvedSessionTarget {
        session_id,
        session_name: Some(display_label(&thread).to_string()),
    })
}

fn confirm_session_delete(target: &ResolvedSessionTarget) -> Result<bool> {
    if !(std::io::stdin().is_terminal() && std::io::stderr().is_terminal()) {
        return Err(eyre!(crate::i18n::tr!(
            "session-archive-confirm-needs-terminal"
        )));
    }

    let mut stderr = std::io::stderr().lock();
    match target.session_name.as_deref() {
        Some(name) => writeln!(
            stderr,
            "{}",
            crate::i18n::tr!(
                "session-archive-confirm-delete-named",
                name = name,
                id = target.session_id.to_string()
            )
        ),
        None => writeln!(
            stderr,
            "{}",
            crate::i18n::tr!(
                "session-archive-confirm-delete",
                id = target.session_id.to_string()
            )
        ),
    }?;
    writeln!(
        stderr,
        "{}",
        crate::i18n::tr!("session-archive-cannot-undo")
    )?;
    write!(
        stderr,
        "{}",
        crate::i18n::tr!("session-archive-continue-prompt")
    )?;
    stderr.flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let answer = input.trim();
    Ok(answer.eq_ignore_ascii_case("y") || answer.eq_ignore_ascii_case("yes"))
}

pub(super) async fn start_app_server_for_session_command(
    options: SessionArchiveCommandOptions,
    codex_home: PathBuf,
) -> Result<AppServerSession> {
    let SessionArchiveCommandOptions {
        cli,
        arg0_paths,
        explicit_remote_endpoint,
    } = options;
    if cli.no_daemon && explicit_remote_endpoint.is_some() {
        return Err(eyre!("--no-daemon cannot be used with --remote."));
    }
    let loader_overrides = LoaderOverrides::default();
    let strict_config = cli.strict_config;
    let raw_overrides = cli.config_overrides.raw_overrides.clone();
    let overrides_cli = CliConfigOverrides { raw_overrides };
    let cli_kv_overrides = overrides_cli
        .parse_overrides()
        .map_err(|err| eyre!("failed to parse -c overrides: {err}"))?;
    let mut launch_loader_overrides = loader_overrides.clone();
    if let Some(profile_v2) = cli.config_profile_v2.as_ref() {
        launch_loader_overrides.user_config_path = Some(resolve_profile_v2_config_path(
            codex_home.as_path(),
            profile_v2,
        ));
        launch_loader_overrides.user_config_profile = Some(profile_v2.clone());
    }

    if let Some(endpoint) = explicit_remote_endpoint {
        // Validate config before connecting, but leave authentication and execution
        // to the selected server even when this caller has workload identity set.
        launch_loader_overrides.ignore_login_requirements = true;
        ConfigBuilder::default()
            .codex_home(codex_home)
            .cli_overrides(cli_kv_overrides)
            .loader_overrides(launch_loader_overrides)
            .strict_config(strict_config)
            .build()
            .await
            .wrap_err("failed to load config.toml")?;
        return Ok(AppServerSession::new(
            crate::backend_id::BackendId::new(),
            super::connect_remote_app_server(endpoint).await?,
            ThreadParamsMode::Remote,
        )
        .with_remote_cwd_override(cli.cwd.clone()));
    }

    let workload_identity_selected = codex_login::is_workload_identity_selected();
    let reuse_implicit_local_daemon = !cli.no_daemon
        && !workload_identity_selected
        && super::daemon_startup::config_exclusion(
            &cli_kv_overrides,
            &launch_loader_overrides,
            strict_config,
            cli.bypass_hook_trust,
        )
        .is_none();
    let default_daemon = if explicit_remote_endpoint.is_none() && reuse_implicit_local_daemon {
        super::maybe_probe_default_daemon_socket(codex_home.as_path()).await
    } else {
        None
    };
    let mut app_server_target = super::app_server_target_for_launch(
        explicit_remote_endpoint,
        default_daemon,
        reuse_implicit_local_daemon,
        workload_identity_selected,
        std::env::var_os(codex_exec_server::CODEX_EXEC_SERVER_URL_ENV_VAR).as_deref(),
    )?;
    let remote_cwd_override = cli
        .cwd
        .clone()
        .filter(|_| app_server_target.uses_remote_workspace());

    let local_runtime_paths = ExecServerRuntimeOptions::from_optional_paths(
        arg0_paths.codex_self_exe.clone(),
        arg0_paths.codex_linux_sandbox_exe.clone(),
    )
    .wrap_err("failed to resolve local runtime paths")?;
    let prepared_environment_manager = EnvironmentManager::prepare_from_env()
        .await
        .wrap_err("failed to discover execution environments")?;
    let config_cwd = super::config_cwd_for_app_server_target(
        cli.cwd.as_deref(),
        &app_server_target,
        prepared_environment_manager.default_environment_is_remote(),
    )
    .wrap_err("failed to resolve config cwd")?;

    let mut loader_overrides = loader_overrides;
    if let Some(profile_v2) = cli.config_profile_v2.as_ref() {
        loader_overrides.user_config_path = Some(resolve_profile_v2_config_path(
            codex_home.as_path(),
            profile_v2,
        ));
        loader_overrides.user_config_profile = Some(profile_v2.clone());
    }
    loader_overrides.ignore_login_requirements = app_server_target.uses_remote_workspace();
    let embedded_network_policy =
        codex_app_server_client::EmbeddedNetworkPolicy::load(&loader_overrides).await;

    let bootstrap_config = load_config_toml_with_layer_stack(
        codex_home.as_path(),
        config_cwd.as_ref(),
        cli_kv_overrides.clone(),
        ConfigLoadOptions {
            loader_overrides: loader_overrides.clone(),
            strict_config,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await
    .wrap_err("failed to load config.toml")?;
    let config_toml = &bootstrap_config.config_toml;
    let cloud_config_bundle = super::cloud_config_bundle_for_app_server_target(
        &app_server_target,
        &bootstrap_config,
        codex_home.as_path(),
        &embedded_network_policy,
    )
    .await?;

    let model_provider = if cli.oss {
        resolve_oss_provider(cli.oss_provider.as_deref(), config_toml)
    } else {
        None
    };
    let model = cli.model.clone().or_else(|| {
        model_provider
            .as_deref()
            .and_then(get_default_model_for_oss_provider)
            .map(ToOwned::to_owned)
    });
    let cwd = cli.cwd.clone();
    let config = ConfigBuilder::default()
        .cli_overrides(cli_kv_overrides.clone())
        .harness_overrides(ConfigOverrides {
            model,
            cwd: if app_server_target.uses_remote_workspace() {
                None
            } else {
                cwd
            },
            model_provider,
            codex_self_exe: arg0_paths.codex_self_exe.clone(),
            codex_linux_sandbox_exe: arg0_paths.codex_linux_sandbox_exe.clone(),
            show_raw_agent_reasoning: cli.oss.then_some(true),
            bypass_hook_trust: cli.bypass_hook_trust.then_some(true),
            ..Default::default()
        })
        .loader_overrides(loader_overrides.clone())
        .strict_config(strict_config)
        .cloud_config_bundle(cloud_config_bundle.clone())
        .build()
        .await
        .wrap_err("failed to load configuration")?;
    let environment_manager = Arc::new(
        prepared_environment_manager
            .build(
                Some(local_runtime_paths),
                app_server_target
                    .environment_http_client_factory(&config, &embedded_network_policy),
            )
            .wrap_err("failed to initialize environment manager")?,
    );
    let mut state_db = super::init_state_db_for_app_server_target(&config, &app_server_target)
        .await
        .wrap_err("failed to initialize state database")?;
    let app_server = super::start_app_server(
        &mut app_server_target,
        arg0_paths,
        config,
        cli_kv_overrides,
        loader_overrides,
        strict_config,
        cloud_config_bundle,
        codex_feedback::CodexFeedback::new(),
        /*log_db*/ None,
        &mut state_db,
        environment_manager,
        embedded_network_policy,
    )
    .await?;
    Ok(AppServerSession::new(
        crate::backend_id::BackendId::new(),
        app_server,
        app_server_target.thread_params_mode(),
    )
    .with_remote_cwd_override(remote_cwd_override))
}

#[cfg(test)]
#[path = "session_archive_commands_tests.rs"]
mod tests;
