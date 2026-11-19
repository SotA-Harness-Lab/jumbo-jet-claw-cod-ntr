//! Windows sandbox prompts and warning surfaces for `ChatWidget`.

use super::*;
// WINDOWS-REMNANT
#[cfg(any(target_os = "windows", test))]
use crate::render::renderable::ColumnRenderable;

impl ChatWidget {
    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(super) fn elevated_windows_sandbox_setup_required(&self) -> bool {
        self.windows_sandbox_config.requires_elevated()
            && (self.windows_sandbox_host != crate::app::WindowsSandboxHost::Local
                || !self.windows_sandbox_elevated_setup_complete)
    }

    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(crate) fn open_windows_sandbox_enable_prompt(
        &mut self,
        preset: ApprovalPreset,
        profile_selection: Option<PermissionProfileSelection>,
    ) {
        use ratatui_macros::line;

        self.session_telemetry.counter(
            "codex.windows_sandbox.elevated_prompt_shown",
            /*inc*/ 1,
            &[],
        );

        let allow_unelevated = self
            .windows_sandbox_config
            .allows(WindowsSandboxSetupMode::Unelevated);
        let setup_choice_is_required =
            !allow_unelevated || self.elevated_windows_sandbox_setup_required();
        let mut header = ColumnRenderable::new();
        header.push(*Box::new(crate::terminal_hyperlinks::HyperlinkText::new(
            if allow_unelevated {
                vec![line![crate::i18n::tr!("windows-sandbox-setup-intro")]]
            } else {
                vec![
                    line![crate::i18n::tr!("windows-sandbox-org-requires")],
                    line![crate::i18n::tr!("windows-sandbox-learn-more")],
                ]
            },
        )));

        let accept_otel = self.session_telemetry.clone();
        let legacy_otel = self.session_telemetry.clone();
        let legacy_preset = preset.clone();
        let legacy_profile_selection = profile_selection.clone();
        let quit_otel = self.session_telemetry.clone();
        let retry_preset = preset.clone();
        let retry_profile_selection = profile_selection.clone();
        let elevated_item = SelectionItem {
            name: crate::i18n::tr!("windows-sandbox-setup-default").to_string(),
            description: None,
            actions: vec![Box::new(move |tx| {
                accept_otel.counter(
                    "codex.windows_sandbox.elevated_prompt_accept",
                    /*inc*/ 1,
                    &[],
                );
                tx.send(AppEvent::BeginWindowsSandboxElevatedSetup {
                    preset: preset.clone(),
                    profile_selection: profile_selection.clone(),
                });
            })],
            dismiss_on_select: true,
            ..Default::default()
        };
        let mut items = if self
            .windows_sandbox_config
            .allows(WindowsSandboxSetupMode::Elevated)
        {
            vec![elevated_item]
        } else {
            Vec::new()
        };
        if allow_unelevated {
            items.push(SelectionItem {
                name: crate::i18n::tr!("windows-sandbox-use-non-admin").to_string(),
                description: None,
                actions: vec![Box::new(move |tx| {
                    legacy_otel.counter(
                        "codex.windows_sandbox.elevated_prompt_use_legacy",
                        /*inc*/ 1,
                        &[],
                    );
                    tx.send(AppEvent::BeginWindowsSandboxLegacySetup {
                        preset: legacy_preset.clone(),
                        profile_selection: legacy_profile_selection.clone(),
                    });
                })],
                dismiss_on_select: true,
                require_explicit_confirmation: true,
                ..Default::default()
            });
        }
        items.push(SelectionItem {
            name: crate::i18n::tr!("windows-sandbox-quit").to_string(),
            description: None,
            actions: vec![Box::new(move |tx| {
                quit_otel.counter(
                    "codex.windows_sandbox.elevated_prompt_quit",
                    /*inc*/ 1,
                    &[],
                );
                tx.send(AppEvent::Exit(ExitMode::ShutdownFirst));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: None,
            footer_hint: Some(standard_popup_hint_line()),
            items,
            header: Box::new(header),
            on_cancel: setup_choice_is_required.then(|| {
                Box::new(move |tx: &AppEventSender| {
                    tx.send(AppEvent::OpenWindowsSandboxEnablePrompt {
                        preset: retry_preset.clone(),
                        profile_selection: retry_profile_selection.clone(),
                    });
                }) as _
            }),
            ..SelectionViewParams::picker()
        });
    }

    // WINDOWS-REMNANT
    #[cfg(all(not(target_os = "windows"), not(test)))]
    pub(crate) fn open_windows_sandbox_enable_prompt(
        &mut self,
        _preset: ApprovalPreset,
        _profile_selection: Option<PermissionProfileSelection>,
    ) {
    }

    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(crate) fn open_windows_sandbox_fallback_prompt(
        &mut self,
        preset: ApprovalPreset,
        profile_selection: Option<PermissionProfileSelection>,
    ) {
        use ratatui_macros::line;

        let allow_unelevated = self
            .windows_sandbox_config
            .allows(WindowsSandboxSetupMode::Unelevated);
        let setup_choice_is_required =
            !allow_unelevated || self.elevated_windows_sandbox_setup_required();
        let mut lines = Vec::new();
        lines.push(line![
            crate::i18n::tr!("windows-sandbox-fallback-title").bold()
        ]);
        lines.push(line![""]);
        if allow_unelevated {
            lines.push(line![crate::i18n::tr!("windows-sandbox-fallback-still")]);
        } else {
            lines.push(line![crate::i18n::tr!("windows-sandbox-fallback-org")]);
        }
        lines.push(line![crate::i18n::tr!("windows-sandbox-learn-more")]);

        let mut header = ColumnRenderable::new();
        header.push(crate::terminal_hyperlinks::HyperlinkText::new(lines));

        let elevated_preset = preset.clone();
        let legacy_preset = preset;
        let retry_preset = elevated_preset.clone();
        let retry_profile_selection = profile_selection.clone();
        let elevated_profile_selection = profile_selection.clone();
        let legacy_profile_selection = profile_selection;
        let quit_otel = self.session_telemetry.clone();
        let elevated_item = SelectionItem {
            name: crate::i18n::tr!("windows-sandbox-retry-admin").to_string(),
            description: None,
            actions: vec![Box::new({
                let otel = self.session_telemetry.clone();
                let preset = elevated_preset;
                move |tx| {
                    otel.counter(
                        "codex.windows_sandbox.fallback_retry_elevated",
                        /*inc*/ 1,
                        &[],
                    );
                    tx.send(AppEvent::BeginWindowsSandboxElevatedSetup {
                        preset: preset.clone(),
                        profile_selection: elevated_profile_selection.clone(),
                    });
                }
            })],
            dismiss_on_select: true,
            ..Default::default()
        };
        let mut items = if self
            .windows_sandbox_config
            .allows(WindowsSandboxSetupMode::Elevated)
        {
            vec![elevated_item]
        } else {
            Vec::new()
        };
        if allow_unelevated {
            items.push(SelectionItem {
                name: crate::i18n::tr!("windows-sandbox-use-codex-non-admin").to_string(),
                description: None,
                actions: vec![Box::new({
                    let otel = self.session_telemetry.clone();
                    let preset = legacy_preset;
                    move |tx| {
                        otel.counter(
                            "codex.windows_sandbox.fallback_use_legacy",
                            /*inc*/ 1,
                            &[],
                        );
                        tx.send(AppEvent::BeginWindowsSandboxLegacySetup {
                            preset: preset.clone(),
                            profile_selection: legacy_profile_selection.clone(),
                        });
                    }
                })],
                dismiss_on_select: true,
                require_explicit_confirmation: true,
                ..Default::default()
            });
        }
        items.push(SelectionItem {
            name: crate::i18n::tr!("windows-sandbox-quit").to_string(),
            description: None,
            actions: vec![Box::new(move |tx| {
                quit_otel.counter(
                    "codex.windows_sandbox.fallback_prompt_quit",
                    /*inc*/ 1,
                    &[],
                );
                tx.send(AppEvent::Exit(ExitMode::ShutdownFirst));
            })],
            dismiss_on_select: true,
            ..Default::default()
        });

        self.bottom_pane.show_selection_view(SelectionViewParams {
            title: None,
            footer_hint: Some(standard_popup_hint_line()),
            items,
            header: Box::new(header),
            on_cancel: setup_choice_is_required.then(|| {
                Box::new(move |tx: &AppEventSender| {
                    tx.send(AppEvent::OpenWindowsSandboxFallbackPrompt {
                        preset: retry_preset.clone(),
                        profile_selection: retry_profile_selection.clone(),
                    });
                }) as _
            }),
            ..SelectionViewParams::picker()
        });
    }

    // WINDOWS-REMNANT
    #[cfg(all(not(target_os = "windows"), not(test)))]
    pub(crate) fn open_windows_sandbox_fallback_prompt(
        &mut self,
        _preset: ApprovalPreset,
        _profile_selection: Option<PermissionProfileSelection>,
    ) {
    }

    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(crate) fn maybe_prompt_windows_sandbox_enable(&mut self, show_now: bool) {
        let setup_is_required = !self.windows_sandbox_config.is_enabled()
            || self.elevated_windows_sandbox_setup_required();
        if show_now
            && setup_is_required
            && let Some(mut preset) = builtin_approval_presets()
                .into_iter()
                .find(|preset| preset.id == "auto")
        {
            if self.config.active_project.trust_level.is_none()
                && self.config.config_layer_stack.is_projectless()
            {
                preset.approval = self.config.permissions.approval_policy.value();
            }
            self.open_windows_sandbox_enable_prompt(preset, /*profile_selection*/ None);
        }
    }

    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(crate) fn show_windows_sandbox_setup_status(&mut self) {
        // While elevated sandbox setup runs, prevent typing so the user doesn't
        // accidentally queue messages that will run under an unexpected mode.
        self.bottom_pane.set_composer_input_enabled(
            /*enabled*/ false,
            Some(crate::i18n::tr!("windows-sandbox-input-disabled").to_string()),
        );
        self.bottom_pane.reset_status_timer(Duration::ZERO);
        self.bottom_pane.ensure_status_indicator();
        self.bottom_pane
            .set_interrupt_hint_visible(/*visible*/ false);
        self.set_status(
            crate::i18n::tr!("windows-sandbox-setting-up").to_string(),
            Some(crate::i18n::tr!("windows-sandbox-hang-tight").to_string()),
            StatusDetailsCapitalization::CapitalizeFirst,
            STATUS_DETAILS_DEFAULT_MAX_LINES,
        );
        self.request_redraw();
    }

    // WINDOWS-REMNANT
    #[cfg(not(any(target_os = "windows", test)))]
    #[allow(dead_code)]
    pub(crate) fn show_windows_sandbox_setup_status(&mut self) {}

    // WINDOWS-REMNANT
    #[cfg(any(target_os = "windows", test))]
    pub(crate) fn clear_windows_sandbox_setup_status(&mut self) {
        self.bottom_pane
            .set_composer_input_enabled(/*enabled*/ true, /*placeholder*/ None);
        self.bottom_pane.hide_status_indicator();
        self.request_redraw();
    }

    // WINDOWS-REMNANT
    #[cfg(not(any(target_os = "windows", test)))]
    pub(crate) fn clear_windows_sandbox_setup_status(&mut self) {}
}
