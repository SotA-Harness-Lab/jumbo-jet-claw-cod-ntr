//! Permission and approval popup flows for `ChatWidget`.
//!
//! This module presents permission choices and confirmations in the shared picker;
//! Windows-specific sandbox prompting lives beside it in
//! `windows_sandbox_prompts`.

use super::permissions_menu::permission_preset_description;
use super::*;
use crate::style::accent_color;
use codex_protocol::openai_models::MODEL_SPECIALTY_CYBER;

impl ChatWidget {
    /// Open the permissions popup.
    pub(crate) fn open_approvals_popup(&mut self) {
        self.open_permissions_popup();
    }

    /// Open a popup to choose the permissions mode.
    pub(crate) fn open_permissions_popup(&mut self) {
        if self.thread_id.is_some()
            || self.config.explicit_permission_profile_mode
            || self.permission_profiles_menu_opened
            || self
                .config
                .permissions
                .active_permission_profile()
                .is_some_and(|profile| !profile.id.starts_with(':'))
        {
            self.request_permission_profiles();
            return;
        }

        self.open_legacy_permissions_popup();
    }

    pub(super) fn open_legacy_permissions_popup(&mut self) {
        // WINDOWS-REMNANT
        let include_read_only = cfg!(target_os = "windows");
        let current_approval =
            AskForApproval::from(self.config.permissions.approval_policy.value());
        let current_permission_profile = self.config.permissions.permission_profile().clone();
        let guardian_approval_enabled = self.config.features.enabled(Feature::GuardianApproval);
        let current_review_policy = self.config.approvals_reviewer;
        let mut items: Vec<SelectionItem> = Vec::new();
        let presets: Vec<ApprovalPreset> = builtin_approval_presets();

        // WINDOWS-REMNANT
        #[cfg(target_os = "windows")]
        let windows_sandbox_level = self.windows_sandbox_config.level();
        // WINDOWS-REMNANT
        #[cfg(target_os = "windows")]
        let windows_degraded_sandbox_enabled =
            matches!(windows_sandbox_level, WindowsSandboxLevel::RestrictedToken)
                && self.windows_sandbox_local_server
                && self.windows_sandbox_host == crate::app::WindowsSandboxHost::Local;
        // WINDOWS-REMNANT
        #[cfg(not(target_os = "windows"))]
        let windows_degraded_sandbox_enabled = false;

        let show_elevate_sandbox_hint = windows_degraded_sandbox_enabled
            && self
                .windows_sandbox_config
                .allows(WindowsSandboxSetupMode::Elevated)
            && presets.iter().any(|preset| preset.id == "auto");

        let guardian_disabled_reason = |enabled: bool| {
            let mut next_features = self.config.features.get().clone();
            next_features.set_enabled(Feature::GuardianApproval, enabled);
            self.config
                .features
                .can_set(&next_features)
                .err()
                .map(|err| err.to_string())
        };

        for preset in presets.into_iter() {
            if !include_read_only && preset.id == "read-only" {
                continue;
            }
            let base_name = if preset.id == "auto" && windows_degraded_sandbox_enabled {
                crate::i18n::tr!(
                    "permission-popups-non-admin-sandbox",
                    label = ASK_FOR_APPROVAL_LABEL
                )
            } else if preset.id == "auto" {
                ASK_FOR_APPROVAL_LABEL.to_string()
            } else {
                preset.label.to_string()
            };
            let base_description = Some(permission_preset_description(&preset).to_string());
            let approval_disabled_reason = match self
                .config
                .permissions
                .approval_policy
                .can_set(&preset.approval)
            {
                Ok(()) => None,
                Err(err) => Some(err.to_string()),
            };
            let default_disabled_reason = approval_disabled_reason
                .clone()
                .or_else(|| guardian_disabled_reason(false));
            let default_actions = self.permission_mode_actions(
                &preset,
                base_name.clone(),
                ApprovalsReviewer::User,
                /*profile_selection*/ None,
                /*return_to_permissions*/ !include_read_only,
            );
            if preset.id == "auto" {
                items.push(SelectionItem {
                    name: base_name.clone(),
                    description: base_description.clone(),
                    is_current: current_review_policy == ApprovalsReviewer::User
                        && Self::preset_matches_current(
                            current_approval,
                            &current_permission_profile,
                            self.config.cwd.as_path(),
                            &preset,
                        ),
                    actions: default_actions,
                    dismiss_on_select: true,
                    disabled_reason: default_disabled_reason,
                    ..Default::default()
                });

                if guardian_approval_enabled {
                    items.push(SelectionItem {
                        name: APPROVE_FOR_ME_LABEL.to_string(),
                        description: Some(AUTO_REVIEW_DESCRIPTION.to_string()),
                        is_current: current_review_policy == ApprovalsReviewer::AutoReview
                            && (Self::preset_matches_current(
                                current_approval,
                                &current_permission_profile,
                                self.config.cwd.as_path(),
                                &preset,
                            ) || (current_approval == AskForApproval::OnRequest
                                && self
                                    .config
                                    .config_layer_stack
                                    .requirements()
                                    .auto_review_required_for_model(self.current_model()))),
                        actions: self.permission_mode_actions(
                            &preset,
                            APPROVE_FOR_ME_LABEL.to_string(),
                            ApprovalsReviewer::AutoReview,
                            /*profile_selection*/ None,
                            /*return_to_permissions*/ !include_read_only,
                        ),
                        dismiss_on_select: true,
                        disabled_reason: approval_disabled_reason
                            .or_else(|| guardian_disabled_reason(true)),
                        ..Default::default()
                    });
                }
            } else {
                items.push(SelectionItem {
                    name: base_name,
                    description: base_description,
                    is_current: Self::preset_matches_current(
                        current_approval,
                        &current_permission_profile,
                        self.config.cwd.as_path(),
                        &preset,
                    ),
                    actions: default_actions,
                    dismiss_on_select: true,
                    disabled_reason: default_disabled_reason,
                    ..Default::default()
                });
            }
        }

        let footer_note = show_elevate_sandbox_hint.then(|| {
            vec![
                crate::i18n::tr!("permission-popups-non-admin-sandbox-note").dim(),
                "/setup-default-sandbox".fg(accent_color()),
                ".".dim(),
            ]
            .into()
        });

        self.bottom_pane.show_selection_view(SelectionViewParams {
            footer_note,
            items,
            title: Some(crate::i18n::tr!("permission-popups-update-title").into()),
            ..SelectionViewParams::picker()
        });
    }

    pub(crate) fn open_auto_review_denials_popup(&mut self) {
        if self.review.recent_auto_review_denials.is_empty() {
            self.add_info_message(
                crate::i18n::tr!("permission-popups-no-denials").to_string(),
                Some(crate::i18n::tr!("permission-popups-denials-recorded").to_string()),
            );
            return;
        }
        let Some(thread_id) = self.thread_id() else {
            self.add_error_message(
                crate::i18n::tr!("permission-popups-thread-unavailable").to_string(),
            );
            return;
        };

        let mut items = vec![SelectionItem {
            name: crate::i18n::tr!("permission-popups-action").to_string(),
            description: Some(crate::i18n::tr!("permission-popups-rationale").to_string()),
            is_disabled: true,
            search_value: Some(String::new()),
            ..Default::default()
        }];
        items.extend(
            self.review
                .recent_auto_review_denials
                .entries()
                .map(|event| {
                    let id = event.id.clone();
                    let summary = auto_review_denials::action_summary(&event.action);
                    let rationale = event
                        .rationale
                        .as_deref()
                        .unwrap_or(crate::i18n::tr!("permission-popups-no-rationale"));
                    SelectionItem {
                        name: summary.clone(),
                        description: Some(rationale.to_string()),
                        selected_description: Some(rationale.to_string()),
                        search_value: Some(format!("{summary} {rationale}")),
                        actions: vec![Box::new(move |tx| {
                            tx.send(AppEvent::ApproveRecentAutoReviewDenial {
                                thread_id,
                                id: id.clone(),
                            });
                        })],
                        dismiss_on_select: true,
                        ..Default::default()
                    }
                }),
        );

        self.bottom_pane.show_selection_view(SelectionViewParams {
            header: Box::new(
                Paragraph::new(vec![
                    Line::from(crate::i18n::tr!("permission-popups-denials-title").bold()),
                    Line::from(crate::i18n::tr!("permission-popups-select-denied").dim()),
                ])
                .wrap(Wrap { trim: false }),
            ),
            items,
            is_searchable: true,
            // Denial rationales remain visible before authorizing a retry.
            description_layout: crate::bottom_pane::SelectionDescriptionLayout::Columns,
            ..SelectionViewParams::picker()
        });
        self.request_redraw();
    }

    pub(crate) fn approve_recent_auto_review_denial(&mut self, thread_id: ThreadId, id: String) {
        let Some(event) = self.review.recent_auto_review_denials.take(&id) else {
            self.add_error_message(
                crate::i18n::tr!("permission-popups-denial-unavailable").to_string(),
            );
            return;
        };

        self.app_event_tx.send(AppEvent::SubmitThreadOp {
            thread_id,
            op: AppCommand::approve_guardian_denied_action(event),
        });
        self.add_info_message(
            crate::i18n::tr!("permission-popups-approval-recorded").to_string(),
            Some(crate::i18n::tr!("permission-popups-approval-context").to_string()),
        );
    }

    pub(super) fn approval_preset_actions(
        approval: AskForApproval,
        permission_profile: PermissionProfile,
        active_permission_profile: ActivePermissionProfile,
        label: String,
        approvals_reviewer: ApprovalsReviewer,
    ) -> Vec<SelectionAction> {
        vec![Box::new(move |tx| {
            tx.send(AppEvent::CodexOp(AppCommand::override_turn_context(
                /*cwd*/ None,
                Some(approval),
                Some(approvals_reviewer),
                Some(permission_profile.clone()),
                Some(active_permission_profile.clone()),
                /*model*/ None,
                /*effort*/ None,
                /*summary*/ None,
                /*service_tier*/ None,
                /*collaboration_mode*/ None,
            )));
            tx.send(AppEvent::UpdateAskForApprovalPolicy(approval));
            tx.send(AppEvent::UpdateActivePermissionProfile(
                active_permission_profile.clone(),
            ));
            tx.send(AppEvent::UpdateApprovalsReviewer(approvals_reviewer));
            tx.send(AppEvent::InsertHistoryCell(Box::new(
                history_cell::new_info_event(
                    crate::i18n::tr!(
                        "permission-popups-permissions-updated",
                        label = label.to_string()
                    ),
                    /*hint*/ None,
                ),
            )));
        })]
    }

    pub(super) fn permission_profile_selection_actions(
        selection: PermissionProfileSelection,
    ) -> Vec<SelectionAction> {
        vec![Box::new(move |tx| {
            tx.send(AppEvent::SelectPermissionProfile(selection.clone()));
        })]
    }

    pub(super) fn permission_mode_actions(
        &self,
        preset: &ApprovalPreset,
        label: String,
        approvals_reviewer: ApprovalsReviewer,
        profile_selection: Option<PermissionProfileSelection>,
        return_to_permissions: bool,
    ) -> Vec<SelectionAction> {
        let profile_selection = profile_selection.or_else(|| {
            self.thread_id.map(|_| PermissionProfileSelection {
                profile_id: preset.active_permission_profile.id.clone(),
                approval_policy: Some(AskForApproval::from(preset.approval)),
                approvals_reviewer: Some(approvals_reviewer),
                display_label: label.clone(),
            })
        });
        let apply_actions = || {
            profile_selection.clone().map_or_else(
                || {
                    Self::approval_preset_actions(
                        AskForApproval::from(preset.approval),
                        preset.permission_profile.clone(),
                        preset.active_permission_profile.clone(),
                        label.clone(),
                        approvals_reviewer,
                    )
                },
                Self::permission_profile_selection_actions,
            )
        };
        let requires_confirmation = preset.id == "full-access";
        // WINDOWS-REMNANT
        #[cfg(target_os = "windows")]
        if preset.id == "auto"
            && matches!(
                self.windows_sandbox_host,
                crate::app::WindowsSandboxHost::Mixed | crate::app::WindowsSandboxHost::Unknown
            )
        {
            let preset = preset.clone();
            return vec![Box::new(move |tx| {
                tx.send(AppEvent::OpenWindowsSandboxEnablePrompt {
                    preset: preset.clone(),
                    profile_selection: profile_selection.clone(),
                });
            })];
        }
        if requires_confirmation {
            let preset = preset.clone();
            return vec![Box::new(move |tx| {
                tx.send(AppEvent::OpenFullAccessConfirmation {
                    preset: preset.clone(),
                    return_to_permissions,
                    profile_selection: profile_selection.clone(),
                });
            })];
        }
        if approvals_reviewer == ApprovalsReviewer::User && preset.id == "auto" {
            // WINDOWS-REMNANT
            #[cfg(target_os = "windows")]
            {
                if self.windows_sandbox_host == crate::app::WindowsSandboxHost::Remote {
                    // The remote server owns the permission choice. Its executor
                    // cannot be set up from this TUI's Windows account.
                    return apply_actions();
                }
                if !self.windows_sandbox_config.is_enabled() {
                    let preset = preset.clone();
                    return vec![Box::new(move |tx| {
                        tx.send(AppEvent::OpenWindowsSandboxEnablePrompt {
                            preset: preset.clone(),
                            profile_selection: profile_selection.clone(),
                        });
                    })];
                }
            }
        }
        apply_actions()
    }

    pub(super) fn preset_matches_current(
        current_approval: AskForApproval,
        current_permission_profile: &PermissionProfile,
        cwd: &std::path::Path,
        preset: &ApprovalPreset,
    ) -> bool {
        let preset_approval = AskForApproval::from(preset.approval);
        if current_approval != preset_approval {
            return false;
        }

        match preset.id {
            "full-access" => matches!(current_permission_profile, PermissionProfile::Disabled),
            "read-only" => {
                let file_system_policy = current_permission_profile.file_system_sandbox_policy();
                matches!(
                    current_permission_profile,
                    PermissionProfile::Managed { .. }
                ) && !file_system_policy.has_full_disk_write_access()
                    && file_system_policy
                        .get_writable_roots_with_cwd(cwd)
                        .is_empty()
                    && current_permission_profile.network_sandbox_policy()
                        == preset.permission_profile.network_sandbox_policy()
            }
            "auto" => {
                let file_system_policy = current_permission_profile.file_system_sandbox_policy();
                matches!(
                    current_permission_profile,
                    PermissionProfile::Managed { .. }
                ) && file_system_policy.can_write_local_path_with_cwd(cwd, cwd)
                    && !file_system_policy.has_full_disk_write_access()
            }
            _ => current_permission_profile == &preset.permission_profile,
        }
    }

    pub(crate) fn open_full_access_confirmation(
        &mut self,
        preset: ApprovalPreset,
        return_to_permissions: bool,
        profile_selection: Option<PermissionProfileSelection>,
    ) {
        let selected_name = preset.label.to_string();
        let approval = AskForApproval::from(preset.approval);
        let is_cyber_model = self.model_catalog.try_list_models().is_ok_and(|models| {
            models.iter().any(|model| {
                model.model == self.current_model()
                    && model.model_specialty.as_deref() == Some(MODEL_SPECIALTY_CYBER)
            })
        });
        let title_line =
            Line::from(crate::i18n::tr!("permission-popups-enable-full-access")).bold();
        let info_lines = if is_cyber_model {
            let recommendation = if auto_review_available(&self.config) {
                crate::i18n::tr!("permission-popups-recommend-approve-for-me")
            } else {
                crate::i18n::tr!("permission-popups-recommend-ask")
            };
            vec![
                Line::default(),
                Line::from(crate::i18n::tr!("permission-popups-full-access-warning")),
                Line::default(),
                Line::from(vec![
                    crate::i18n::tr!("permission-popups-cyber-risk").red(),
                    crate::i18n::tr!("permission-popups-cyber-safeguards").into(),
                    recommendation.into(),
                ]),
            ]
        } else {
            vec![Line::from(vec![
                crate::i18n::tr!("permission-popups-full-access-warning-sp").into(),
                crate::i18n::tr!("permission-popups-exercise-caution").red(),
            ])]
        };
        let header = Paragraph::new(
            std::iter::once(title_line)
                .chain(info_lines)
                .collect::<Vec<_>>(),
        )
        .wrap(Wrap { trim: false });

        let accept_actions = profile_selection.map_or_else(
            || {
                Self::approval_preset_actions(
                    approval,
                    preset.permission_profile,
                    preset.active_permission_profile,
                    selected_name,
                    ApprovalsReviewer::User,
                )
            },
            Self::permission_profile_selection_actions,
        );

        let deny_actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
            if return_to_permissions {
                tx.send(AppEvent::OpenPermissionsPopup);
            } else {
                tx.send(AppEvent::OpenApprovalsPopup);
            }
        })];

        let items = vec![
            SelectionItem {
                name: crate::i18n::tr!("permission-popups-yes-continue").to_string(),
                description: Some(
                    crate::i18n::tr!("permission-popups-apply-full-access").to_string(),
                ),
                actions: accept_actions,
                dismiss_on_select: true,
                ..Default::default()
            },
            SelectionItem {
                name: crate::i18n::tr!("permission-popups-cancel").to_string(),
                description: Some(crate::i18n::tr!("permission-popups-go-back").to_string()),
                actions: deny_actions,
                dismiss_on_select: true,
                ..Default::default()
            },
        ];

        self.bottom_pane.show_selection_view(SelectionViewParams {
            items,
            header: Box::new(header),
            ..SelectionViewParams::confirmation()
        });
    }
}
