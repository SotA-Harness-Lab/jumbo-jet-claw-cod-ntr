// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Persist a provider choice on its originating backend, leaving running threads untouched.

use super::App;
use super::AppEvent;
use super::AppServerSession;
use super::BackendId;
use super::Event;
use super::LIST_VIEW;
use super::LOADING_VIEW;
use super::edit_view;
use super::editor;
use super::loading_with_notice;
use super::progress;
use super::rpc;
use codex_app_server_client::AppServerRequestHandle;
use codex_app_server_client::TypedRequestError;
use codex_app_server_protocol::ModelProviderConfigWriteParams;
use codex_app_server_protocol::WriteStatus;
use tokio::sync::watch;

const SAVING_VIEW: &str = "model_provider_saving";

#[derive(derive_more::Debug, Clone)]
pub(crate) enum Write {
    Default {
        id: String,
        name: String,
    },
    #[debug("Settings(..)")]
    Settings(Box<(editor::Draft, ModelProviderConfigWriteParams)>),
}

#[derive(Debug)]
pub(crate) struct Completion {
    backend: BackendId,
    dismissed: watch::Receiver<()>,
    write: Write,
    result: Result<SaveNotice, String>,
}

#[derive(Debug)]
enum SaveNotice {
    Saved(String),
    UnconfirmedCreate,
}

impl Write {
    pub(super) fn start(self, app: &mut App, server: &AppServerSession) {
        let backend = server.backend_id();
        let write = app
            .provider_writes
            .entry(backend)
            .or_insert_with(|| app.model_catalog.provider_write.clone());
        let Ok(mut provider_changed) = write.clone().try_lock_owned() else {
            app.chat_widget
                .add_error_message(crate::i18n::tr!("provider-write-already-saving").into());
            return;
        };
        let (view, dismissed) = progress(SAVING_VIEW, crate::i18n::tr!("provider-write-saving"));
        let source = match self {
            Self::Default { .. } => LIST_VIEW,
            Self::Settings(_) => editor::VIEW_ID,
        };
        if !app
            .chat_widget
            .replace_selection_view_if_present(source, view)
        {
            return;
        }
        let handle = server.request_handle();
        let tx = app.app_event_tx.clone();
        let offline = app.reconnect.offline;
        tokio::spawn(async move {
            let result = if offline {
                Err(crate::i18n::tr!("provider-write-offline").into())
            } else {
                self.run(&handle, &mut provider_changed).await
            };
            drop(provider_changed);
            tx.send(AppEvent::Provider(Event::Written(Completion {
                backend,
                dismissed,
                write: self,
                result,
            })));
        });
    }

    async fn run(
        &self,
        handle: &AppServerRequestHandle,
        provider_changed: &mut bool,
    ) -> Result<SaveNotice, String> {
        match self {
            Self::Settings(settings) => {
                rpc::write_provider(handle, settings.1.clone())
                    .await
                    .map(|result| match result {
                        rpc::ProviderWrite::Saved(_) => SaveNotice::Saved(
                            crate::i18n::tr!("provider-write-saved-config").into(),
                        ),
                        rpc::ProviderWrite::UnconfirmedCreate => SaveNotice::UnconfirmedCreate,
                    })
            }
            Self::Default { id, name } => {
                let name = if name.trim().is_empty() {
                    id.as_str()
                } else {
                    name.trim()
                };
                let response = rpc::set_default_provider(handle, id)
                    .await
                    .map_err(|error| match error {
                        TypedRequestError::Server { .. } if rpc::unsupported_method(&error) => {
                            crate::i18n::tr!("provider-write-update-server").into()
                        }
                        TypedRequestError::Server { .. } => {
                            crate::i18n::tr!(
                                "provider-write-save-failed",
                                error = error.to_string()
                            )
                        }
                        TypedRequestError::Transport { .. }
                        | TypedRequestError::Deserialize { .. } => {
                            // The server may have committed the write before its reply was lost.
                            *provider_changed = true;
                            crate::i18n::tr!(
                                "provider-write-confirm-failed",
                                name = name,
                                error = error.to_string()
                            )
                        }
                    })?;
                match response.status {
                    WriteStatus::Ok => {
                        *provider_changed = true;
                        Ok(SaveNotice::Saved(crate::i18n::tr!(
                            "provider-write-now-use",
                            name = name
                        )))
                    }
                    WriteStatus::OkOverridden => {
                        let reason = rpc::override_reason(response);
                        Ok(SaveNotice::Saved(crate::i18n::tr!(
                            "provider-write-saved-name",
                            name = name,
                            reason = reason
                        )))
                    }
                }
            }
        }
    }
}

impl Completion {
    pub(super) fn finish(self, app: &mut App, backend: BackendId) {
        if backend != self.backend {
            return;
        }
        let present = matches!(self.dismissed.has_changed(), Ok(false));
        if app.model_catalog.provider_may_have_changed() {
            app.chat_widget.dismiss_stale_model_picker();
        }
        match self.result {
            Ok(notice) => {
                let message = match notice {
                    SaveNotice::Saved(message) => {
                        if !present {
                            app.chat_widget
                                .add_info_message(message.clone(), /*hint*/ None);
                        }
                        message
                    }
                    SaveNotice::UnconfirmedCreate => {
                        let message = rpc::unconfirmed_create().to_string();
                        app.chat_widget.add_error_message(message.clone());
                        message
                    }
                };
                if present {
                    let view = match self.write {
                        Write::Default { .. } => SAVING_VIEW,
                        Write::Settings(_) => {
                            app.chat_widget.dismiss_provider_view(SAVING_VIEW);
                            LIST_VIEW
                        }
                    };
                    reload(app, view, Some(message));
                } else if !reload(app, LIST_VIEW, /*notice*/ None) {
                    // A reopened picker may have started reading before this write committed.
                    reload(app, LOADING_VIEW, /*notice*/ None);
                }
            }
            Err(error) if present => {
                if let Write::Settings(settings) = self.write {
                    let view = edit_view(app, backend, settings.0, Some(error));
                    app.chat_widget
                        .replace_bottom_pane_view_if_present(SAVING_VIEW, view);
                    return;
                }
                app.chat_widget.add_error_message(error.clone());
                reload(app, SAVING_VIEW, Some(error));
            }
            Err(error) => app.chat_widget.add_error_message(error),
        }
    }
}

fn reload(app: &mut App, view_id: &'static str, notice: Option<String>) -> bool {
    let (view, load) = loading_with_notice(notice);
    if app
        .chat_widget
        .replace_selection_view_if_present(view_id, view)
    {
        app.app_event_tx.send(AppEvent::Provider(load));
        return true;
    }
    false
}
