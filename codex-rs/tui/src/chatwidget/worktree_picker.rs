//! Worktree choices for local session commands.
//!
//! Shared picker presentation preserves request identity and action safety checks.

use super::*;
use crate::app_event::ManagedWorktreeMode;
use crate::bottom_pane::PickerSurface;
use crate::worktree_browser::Action;
use crate::worktree_browser::Entry;
use crate::worktree_browser::Owner;
use crate::worktree_browser::Request;

const BROWSER_VIEW_ID: &str = "managed-worktrees";

impl ChatWidget {
    pub(crate) fn managed_worktree_available(&self) -> bool {
        self.local_worktree_operations && get_git_repo_root(self.config.cwd.as_path()).is_some()
    }

    pub(super) fn show_session_checkout_picker(
        &mut self,
        mode: ManagedWorktreeMode,
        name: Option<String>,
    ) {
        if mode == ManagedWorktreeMode::New
            && self.config.features.enabled(Feature::TuiMultiBackend)
        {
            self.app_event_tx
                .send(AppEvent::ChooseNewSessionCheckout { name });
            return;
        }
        if !self.managed_worktree_available() {
            match mode {
                ManagedWorktreeMode::New => {
                    self.app_event_tx.send(AppEvent::NewSession { name });
                }
                ManagedWorktreeMode::Fork => {
                    self.app_event_tx
                        .send(AppEvent::ForkCurrentSession { name });
                }
            }
            return;
        }

        self.open_session_checkout_picker(mode, name);
    }

    pub(crate) fn open_session_checkout_picker(
        &mut self,
        mode: ManagedWorktreeMode,
        name: Option<String>,
    ) {
        let title = match mode {
            ManagedWorktreeMode::New => crate::i18n::tr!("worktree-picker-where-new"),
            ManagedWorktreeMode::Fork => crate::i18n::tr!("worktree-picker-where-fork"),
        };
        let current_name = name.clone();
        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(title.into()),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-use-current").to_string(),
                    description: Some(
                        crate::i18n::tr!("worktree-picker-use-current-desc").to_string(),
                    ),
                    actions: vec![Box::new(move |tx| match mode {
                        ManagedWorktreeMode::New => {
                            tx.send(AppEvent::NewSessionInCurrentWorktree {
                                name: current_name.clone(),
                            });
                        }
                        ManagedWorktreeMode::Fork => {
                            tx.send(AppEvent::ForkCurrentSession {
                                name: current_name.clone(),
                            });
                        }
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-create-new").to_string(),
                    description: Some(
                        crate::i18n::tr!("worktree-picker-create-new-desc").to_string(),
                    ),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::StartManagedWorktree {
                            mode,
                            name: name.clone(),
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::picker()
        });
        self.request_redraw();
    }

    pub(super) fn show_managed_worktree_picker(&mut self) {
        if !self.managed_worktree_available() {
            self.add_error_message(crate::i18n::tr!("worktree-picker-require-local").to_string());
            return;
        }

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(crate::i18n::tr!("worktree-picker-title").into()),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-continue").to_string(),
                    description: Some(
                        crate::i18n::tr!("worktree-picker-continue-desc").to_string(),
                    ),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::StartManagedWorktree {
                            mode: ManagedWorktreeMode::Fork,
                            name: None,
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-start-new").to_string(),
                    description: Some(
                        crate::i18n::tr!("worktree-picker-start-new-desc").to_string(),
                    ),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::StartManagedWorktree {
                            mode: ManagedWorktreeMode::New,
                            name: None,
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-browse").to_string(),
                    description: Some(crate::i18n::tr!("worktree-picker-browse-desc").to_string()),
                    actions: vec![Box::new(|tx| tx.send(AppEvent::BrowseManagedWorktrees))],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::picker()
        });
        self.request_redraw();
    }

    pub(crate) fn request_managed_worktrees(&mut self) -> Option<Request> {
        if !self.managed_worktree_available() {
            return None;
        }
        let request = Request {
            id: uuid::Uuid::new_v4(),
            cwd: self.config.cwd.to_path_buf(),
            thread_id: self.thread_id,
        };
        self.bottom_pane.dismiss_view_by_id(BROWSER_VIEW_ID);
        self.worktree_popup_request_id = Some(request.id);
        self.bottom_pane.show_selection_view(SelectionViewParams {
            view_id: Some(BROWSER_VIEW_ID),
            picker_surface: PickerSurface::Panel,
            title: Some(crate::i18n::tr!("worktree-picker-managed-title").into()),
            items: vec![SelectionItem {
                name: crate::i18n::tr!("worktree-picker-loading").to_string(),
                is_disabled: true,
                ..Default::default()
            }],
            ..Default::default()
        });
        Some(request)
    }

    pub(crate) fn worktree_request_is_current(&self, request: &Request) -> bool {
        self.worktree_popup_request_id == Some(request.id)
            && request.cwd == self.config.cwd.as_path()
            && request.thread_id == self.thread_id
            && self.managed_worktree_available()
    }

    pub(crate) fn on_managed_worktrees_loaded(
        &mut self,
        request: Request,
        result: Result<Vec<Entry>, String>,
    ) {
        if self.worktree_popup_request_id != Some(request.id) {
            return;
        }
        if !self.worktree_request_is_current(&request)
            || !self.bottom_pane.dismiss_active_view_if_id(BROWSER_VIEW_ID)
        {
            self.worktree_popup_request_id = None;
            self.bottom_pane.dismiss_view_by_id(BROWSER_VIEW_ID);
            return;
        }
        let entries = match result {
            Ok(entries) => entries,
            Err(error) => {
                self.worktree_popup_request_id = None;
                self.add_error_message(crate::i18n::tr!(
                    "worktree-picker-list-error",
                    error = error
                ));
                return;
            }
        };
        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(crate::i18n::tr!("worktree-picker-managed-title").to_string()),
            subtitle: Some(
                if entries.is_empty() {
                    crate::i18n::tr!("worktree-picker-empty")
                } else {
                    crate::i18n::tr!("worktree-picker-select")
                }
                .to_string(),
            ),
            is_searchable: true,
            items: entries
                .into_iter()
                .map(|entry| {
                    let request = request.clone();
                    let (name, description) = match &entry.owner {
                        Owner::None | Owner::Unavailable(_) => {
                            let status = if matches!(entry.owner, Owner::None) {
                                crate::i18n::tr!("worktree-picker-no-thread")
                            } else {
                                crate::i18n::tr!("worktree-picker-owner-unavailable")
                            };
                            (
                                entry.cwd.display().to_string(),
                                crate::i18n::tr!(
                                    "worktree-picker-status-line",
                                    status = status,
                                    cwd = entry.cwd.display().to_string()
                                ),
                            )
                        }
                        Owner::Archived(thread) | Owner::Resumable(thread) => {
                            let archived = match &entry.owner {
                                Owner::Archived(_) => true,
                                Owner::Resumable(_) => false,
                                Owner::None | Owner::Unavailable(_) => unreachable!(),
                            };
                            let ago = worktree_updated_ago(
                                thread.updated_at,
                                chrono::Utc::now().timestamp(),
                            );
                            let cwd = entry.cwd.display().to_string();
                            (
                                thread.title.clone(),
                                if archived {
                                    crate::i18n::tr!(
                                        "worktree-picker-archived-updated",
                                        ago = ago,
                                        cwd = cwd
                                    )
                                } else {
                                    crate::i18n::tr!(
                                        "worktree-picker-updated",
                                        ago = ago,
                                        cwd = cwd
                                    )
                                },
                            )
                        }
                    };
                    SelectionItem {
                        name: name.clone(),
                        search_value: Some(format!("{name} {}", entry.cwd.display())),
                        description: Some(description),
                        actions: vec![Box::new(move |tx| {
                            tx.send(AppEvent::ShowManagedWorktreeActions {
                                request: request.clone(),
                                entry: entry.clone(),
                            })
                        })],
                        dismiss_on_select: true,
                        ..Default::default()
                    }
                })
                .collect(),
            ..SelectionViewParams::picker()
        });
    }

    pub(crate) fn managed_worktree_action(
        &self,
        request: &Request,
        action: Action,
    ) -> Option<AppEvent> {
        if !self.worktree_request_is_current(request) {
            return None;
        }
        Some(match action {
            Action::Resume(owner) => AppEvent::ResumeSessionByIdOrName(owner.to_string()),
            Action::Copy(cwd) => AppEvent::CopySelection {
                text: cwd.to_str()?.into(),
                label: crate::i18n::tr!("worktree-picker-copy-label").to_string(),
                format: crate::clipboard_copy::CopyFormat::PlainText,
            },
            Action::Remove(root) => AppEvent::RemoveManagedWorktree {
                request: request.clone(),
                root,
            },
        })
    }

    pub(crate) fn show_managed_worktree_actions(&mut self, request: Request, entry: Entry) {
        if !self.worktree_request_is_current(&request) {
            return;
        }
        let mut items = Vec::new();
        if let Owner::Resumable(thread) = &entry.owner {
            let owner = thread.id;
            let request = request.clone();
            items.push(SelectionItem {
                name: crate::i18n::tr!("worktree-picker-resume-owner").to_string(),
                actions: vec![Box::new(move |tx| {
                    tx.send(AppEvent::ManagedWorktreeAction {
                        request: request.clone(),
                        action: Action::Resume(owner),
                    })
                })],
                dismiss_on_select: true,
                ..Default::default()
            });
        }
        let cwd = entry.cwd.clone();
        let copy_request = request.clone();
        items.push(SelectionItem {
            name: crate::i18n::tr!("worktree-picker-copy-cwd").to_string(),
            is_disabled: entry.cwd.to_str().is_none(),
            disabled_reason: entry
                .cwd
                .to_str()
                .is_none()
                .then(|| crate::i18n::tr!("worktree-picker-not-utf8").to_string()),
            actions: vec![Box::new(move |tx| {
                tx.send(AppEvent::ManagedWorktreeAction {
                    request: copy_request.clone(),
                    action: Action::Copy(cwd.clone()),
                })
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
        let can_delete = !request.cwd.starts_with(&entry.root);
        let root = entry.root.clone();
        let delete_request = request;
        items.push(SelectionItem {
            name: crate::i18n::tr!("worktree-picker-delete").to_string(),
            is_disabled: !can_delete,
            disabled_reason: (!can_delete)
                .then(|| crate::i18n::tr!("worktree-picker-switch-first").to_string()),
            actions: vec![Box::new(move |tx| {
                tx.send(AppEvent::ConfirmManagedWorktreeRemoval {
                    request: delete_request.clone(),
                    root: root.clone(),
                })
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
        let title = match &entry.owner {
            Owner::Archived(thread) | Owner::Resumable(thread) => {
                crate::i18n::tr!("worktree-picker-titled", title = thread.title.clone())
            }
            Owner::None | Owner::Unavailable(_) => {
                crate::i18n::tr!("worktree-picker-single").to_string()
            }
        };
        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(title),
            subtitle: Some(entry.cwd.display().to_string()),
            items,
            ..SelectionViewParams::picker()
        });
    }

    pub(crate) fn confirm_managed_worktree_removal(&mut self, request: Request, root: PathBuf) {
        if !self.worktree_request_is_current(&request) || request.cwd.starts_with(&root) {
            return;
        }
        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: Some(crate::i18n::tr!("worktree-picker-delete-title").to_string()),
            subtitle: Some(root.display().to_string()),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-cancel").to_string(),
                    actions: vec![],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("worktree-picker-delete").to_string(),
                    description: Some(crate::i18n::tr!("worktree-picker-delete-desc").to_string()),
                    actions: vec![Box::new(move |tx| {
                        tx.send(AppEvent::ManagedWorktreeAction {
                            request: request.clone(),
                            action: Action::Remove(root.clone()),
                        })
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::confirmation()
        });
    }
}

fn worktree_updated_ago(updated_at: i64, now: i64) -> String {
    let seconds = now.saturating_sub(updated_at).max(0);
    if seconds < 60 {
        crate::i18n::tr!("worktree-picker-just-now").to_string()
    } else if seconds < 3_600 {
        crate::i18n::tr!("worktree-picker-minutes-ago", minutes = seconds / 60)
    } else if seconds < 86_400 {
        crate::i18n::tr!("worktree-picker-hours-ago", hours = seconds / 3_600)
    } else {
        crate::i18n::tr!("worktree-picker-days-ago", days = seconds / 86_400)
    }
}
