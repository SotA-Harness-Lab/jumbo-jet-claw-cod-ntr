// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Catalog and accessors for keymap actions shown by `/keymap`.
//!
//! The catalog declaration generates both the UI-facing descriptors and the
//! matching root-config accessors. Each entry ties together the config path,
//! user-facing context label, stable action name, short description, and
//! optional feature gate used by the picker and action menu.
//!
//! Runtime lookups reuse the inventory owned by [`crate::keymap`]. A catalog
//! action must remain both writable in `TuiKeymap` and readable from the shared
//! runtime inventory.

use std::collections::BTreeSet;

use codex_config::types::KeybindingsSpec;
use codex_config::types::TuiKeymap;
use crossterm::event::KeyEvent;

use crate::keymap::RuntimeKeymap;
use crate::keymap::bindings_for_action;

#[derive(Clone, Copy, Debug)]
pub(super) struct KeymapActionDescriptor {
    /// Config context segment, such as `composer` in `tui.keymap.composer.submit`.
    pub(super) context: &'static str,
    /// Human-readable group label shown in the picker.
    pub(super) context_label: &'static str,
    /// Config action segment, such as `submit` in `tui.keymap.composer.submit`.
    pub(super) action: &'static str,
    /// Short user-facing explanation of what the action does.
    pub(super) description: &'static str,
    /// Feature required before the action appears in `/keymap`.
    required_feature: Option<KeymapActionFeature>,
}

#[derive(Clone, Copy, Debug)]
enum KeymapActionFeature {
    FastMode,
    MultiBackend,
    Workspace,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct KeymapActionFilter {
    pub(crate) fast_mode_enabled: bool,
    pub(crate) multi_backend_enabled: bool,
    pub(crate) workspace_enabled: bool,
}

impl KeymapActionDescriptor {
    pub(super) fn is_visible(self, filter: KeymapActionFilter) -> bool {
        match self.required_feature {
            None => true,
            Some(KeymapActionFeature::FastMode) => filter.fast_mode_enabled,
            Some(KeymapActionFeature::MultiBackend) => filter.multi_backend_enabled,
            Some(KeymapActionFeature::Workspace) => filter.workspace_enabled,
        }
    }
}

macro_rules! required_feature {
    () => {
        None
    };
    ($feature:ident) => {
        Some(KeymapActionFeature::$feature)
    };
}

macro_rules! define_keymap_actions {
    (
        $(
            $context:ident $context_label:literal {
                $(
                    $action:ident $( [$required_feature:ident] )? => $description:literal,
                )*
            }
        )*
    ) => {
        pub(super) static KEYMAP_ACTIONS: std::sync::LazyLock<Vec<KeymapActionDescriptor>> =
            std::sync::LazyLock::new(|| vec![
            $(
                $(
                    KeymapActionDescriptor {
                        context: stringify!($context),
                        context_label: crate::i18n::tr!($context_label),
                        action: stringify!($action),
                        description: crate::i18n::tr!($description),
                        required_feature: required_feature!($($required_feature)?),
                    },
                )*
            )*
        ]);

        /// Return the mutable root-config binding slot for one catalog action.
        ///
        /// The returned `Option<KeybindingsSpec>` distinguishes three states that the
        /// editor must preserve: absent means use fallback/default resolution, `Some`
        /// with one or more keys is a custom binding, and `Some(Many([]))` is an
        /// explicit unbind.
        pub(super) fn binding_slot<'a>(
            keymap: &'a mut TuiKeymap,
            context: &str,
            action: &str,
        ) -> Option<&'a mut Option<KeybindingsSpec>> {
            match (context, action) {
                $(
                    $(
                        (stringify!($context), stringify!($action)) => {
                            Some(&mut keymap.$context.$action)
                        }
                    )*
                )*
                _ => None,
            }
        }
    };
}

define_keymap_actions! {
    global "keymap-actions-ctx-global" {
        change_host [MultiBackend] => "keymap-actions-global-change-host",
        focus_main_composer [Workspace] => "keymap-actions-global-focus-main-composer",
        focus_sidebar_tabs [Workspace] => "keymap-actions-global-focus-sidebar-tabs",
        focus_sidebar_content [Workspace] => "keymap-actions-global-focus-sidebar-content",
        hide_pane [Workspace] => "keymap-actions-global-hide-pane",
        close_sidebar_tab [Workspace] => "keymap-actions-global-close-sidebar-tab",
        sidebar_up [Workspace] => "keymap-actions-global-sidebar-up",
        sidebar_down [Workspace] => "keymap-actions-global-sidebar-down",
        focus_pane_left [Workspace] => "keymap-actions-global-focus-pane-left",
        focus_pane_right [Workspace] => "keymap-actions-global-focus-pane-right",
        minimize_pane_width [Workspace] => "keymap-actions-global-minimize-pane-width",
        maximize_pane_width [Workspace] => "keymap-actions-global-maximize-pane-width",
        equalize_pane_widths [Workspace] => "keymap-actions-global-equalize-pane-widths",
        resize_panes [Workspace] => "keymap-actions-global-resize-panes",
        move_divider_left [Workspace] => "keymap-actions-global-move-divider-left",
        move_divider_right [Workspace] => "keymap-actions-global-move-divider-right",
        toggle_pane_zoom [Workspace] => "keymap-actions-global-toggle-pane-zoom",
        open_agents => "keymap-actions-global-open-agents",
        open_transcript => "keymap-actions-global-open-transcript",
        find_transcript => "keymap-actions-global-find-transcript",
        // Keep the new warnings action out of shared-config writes until older strict readers
        // accept it. Its built-in shortcuts remain available.
        focus_activity => "keymap-actions-global-focus-activity",
        open_external_editor => "keymap-actions-global-open-external-editor",
        copy => "keymap-actions-global-copy",
        clear_terminal => "keymap-actions-global-clear-terminal",
        toggle_vim_mode => "keymap-actions-global-toggle-vim-mode",
        toggle_fast_mode [FastMode] => "keymap-actions-global-toggle-fast-mode",
        toggle_raw_output => "keymap-actions-global-toggle-raw-output",
        toggle_side_conversation => "keymap-actions-global-toggle-side-conversation",
        leader => "keymap-actions-global-leader",
    }
    chat "keymap-actions-ctx-chat" {
        interrupt_turn => "keymap-actions-chat-interrupt-turn",
        decrease_reasoning_effort => "keymap-actions-chat-decrease-reasoning-effort",
        increase_reasoning_effort => "keymap-actions-chat-increase-reasoning-effort",
        previous_permission_mode => "keymap-actions-chat-previous-permission-mode",
        next_permission_mode => "keymap-actions-chat-next-permission-mode",
        edit_queued_message => "keymap-actions-chat-edit-queued-message",
        prompt_stack_back => "keymap-actions-chat-prompt-stack-back",
        skip_question => "keymap-actions-chat-skip-question",
        toggle_voice => "keymap-actions-chat-toggle-voice",
        open_presets => "keymap-actions-chat-open-presets",
        toggle_voice_mute => "keymap-actions-chat-toggle-voice-mute",
    }
    composer "keymap-actions-ctx-composer" {
        submit => "keymap-actions-composer-submit",
        queue => "keymap-actions-composer-queue",
        toggle_shortcuts => "keymap-actions-composer-toggle-shortcuts",
        history_search_previous => "keymap-actions-composer-history-search-previous",
        history_search_next => "keymap-actions-composer-history-search-next",
        undo => "keymap-actions-composer-undo",
        redo => "keymap-actions-composer-redo",
    }
    editor "keymap-actions-ctx-editor" {
        insert_newline => "keymap-actions-editor-insert-newline",
        move_left => "keymap-actions-editor-move-left",
        move_right => "keymap-actions-editor-move-right",
        move_up => "keymap-actions-editor-move-up",
        move_down => "keymap-actions-editor-move-down",
        move_word_left => "keymap-actions-editor-move-word-left",
        move_word_right => "keymap-actions-editor-move-word-right",
        move_line_start => "keymap-actions-editor-move-line-start",
        move_line_end => "keymap-actions-editor-move-line-end",
        delete_backward => "keymap-actions-editor-delete-backward",
        delete_forward => "keymap-actions-editor-delete-forward",
        delete_backward_word => "keymap-actions-editor-delete-backward-word",
        delete_forward_word => "keymap-actions-editor-delete-forward-word",
        kill_line_start => "keymap-actions-editor-kill-line-start",
        kill_whole_line => "keymap-actions-editor-kill-whole-line",
        kill_line_end => "keymap-actions-editor-kill-line-end",
        yank => "keymap-actions-editor-yank",
    }
    vim_normal "keymap-actions-ctx-vim-normal" {
        enter_insert => "keymap-actions-vim-normal-enter-insert",
        append_after_cursor => "keymap-actions-vim-normal-append-after-cursor",
        append_line_end => "keymap-actions-vim-normal-append-line-end",
        insert_line_start => "keymap-actions-vim-normal-insert-line-start",
        open_line_below => "keymap-actions-vim-normal-open-line-below",
        open_line_above => "keymap-actions-vim-normal-open-line-above",
        enter_replace_mode => "keymap-actions-vim-normal-enter-replace-mode",
        move_left => "keymap-actions-vim-normal-move-left",
        move_right => "keymap-actions-vim-normal-move-right",
        move_up => "keymap-actions-vim-normal-move-up",
        move_down => "keymap-actions-vim-normal-move-down",
        move_word_forward => "keymap-actions-vim-normal-move-word-forward",
        move_word_backward => "keymap-actions-vim-normal-move-word-backward",
        move_word_end => "keymap-actions-vim-normal-move-word-end",
        move_line_start => "keymap-actions-vim-normal-move-line-start",
        move_line_end => "keymap-actions-vim-normal-move-line-end",
        find_forward => "keymap-actions-vim-normal-find-forward",
        find_backward => "keymap-actions-vim-normal-find-backward",
        till_forward => "keymap-actions-vim-normal-till-forward",
        till_backward => "keymap-actions-vim-normal-till-backward",
        jump_top => "keymap-actions-vim-normal-jump-top",
        jump_bottom => "keymap-actions-vim-normal-jump-bottom",
        delete_char => "keymap-actions-vim-normal-delete-char",
        replace_char => "keymap-actions-vim-normal-replace-char",
        repeat_last_change => "keymap-actions-vim-normal-repeat-last-change",
        substitute_char => "keymap-actions-vim-normal-substitute-char",
        delete_to_line_end => "keymap-actions-vim-normal-delete-to-line-end",
        change_to_line_end => "keymap-actions-vim-normal-change-to-line-end",
        yank_line => "keymap-actions-vim-normal-yank-line",
        paste_after => "keymap-actions-vim-normal-paste-after",
        start_delete_operator => "keymap-actions-vim-normal-start-delete-operator",
        start_yank_operator => "keymap-actions-vim-normal-start-yank-operator",
        start_change_operator => "keymap-actions-vim-normal-start-change-operator",
        undo => "keymap-actions-vim-normal-undo",
        redo => "keymap-actions-vim-normal-redo",
        cancel_operator => "keymap-actions-vim-normal-cancel-operator",
    }
    vim_search "keymap-actions-ctx-vim-search" {
        forward => "keymap-actions-vim-search-forward",
        backward => "keymap-actions-vim-search-backward",
        next => "keymap-actions-vim-search-next",
        previous => "keymap-actions-vim-search-previous",
    }
    vim_operator "keymap-actions-ctx-vim-operator" {
        delete_line => "keymap-actions-vim-operator-delete-line",
        yank_line => "keymap-actions-vim-operator-yank-line",
        motion_left => "keymap-actions-vim-operator-motion-left",
        motion_right => "keymap-actions-vim-operator-motion-right",
        motion_up => "keymap-actions-vim-operator-motion-up",
        motion_down => "keymap-actions-vim-operator-motion-down",
        motion_word_forward => "keymap-actions-vim-operator-motion-word-forward",
        motion_word_backward => "keymap-actions-vim-operator-motion-word-backward",
        motion_word_end => "keymap-actions-vim-operator-motion-word-end",
        motion_line_start => "keymap-actions-vim-operator-motion-line-start",
        motion_line_end => "keymap-actions-vim-operator-motion-line-end",
        motion_find_forward => "keymap-actions-vim-operator-motion-find-forward",
        motion_find_backward => "keymap-actions-vim-operator-motion-find-backward",
        motion_till_forward => "keymap-actions-vim-operator-motion-till-forward",
        motion_till_backward => "keymap-actions-vim-operator-motion-till-backward",
        motion_jump_top => "keymap-actions-vim-operator-motion-jump-top",
        motion_jump_bottom => "keymap-actions-vim-operator-motion-jump-bottom",
        select_inner_text_object => "keymap-actions-vim-operator-select-inner-text-object",
        select_around_text_object => "keymap-actions-vim-operator-select-around-text-object",
        cancel => "keymap-actions-vim-operator-cancel",
    }
    vim_text_object "keymap-actions-ctx-vim-text-object" {
        word => "keymap-actions-vim-text-object-word",
        big_word => "keymap-actions-vim-text-object-big-word",
        parentheses => "keymap-actions-vim-text-object-parentheses",
        brackets => "keymap-actions-vim-text-object-brackets",
        braces => "keymap-actions-vim-text-object-braces",
        double_quote => "keymap-actions-vim-text-object-double-quote",
        single_quote => "keymap-actions-vim-text-object-single-quote",
        backtick => "keymap-actions-vim-text-object-backtick",
        cancel => "keymap-actions-vim-text-object-cancel",
    }
    pager "keymap-actions-ctx-pager" {
        scroll_up => "keymap-actions-pager-scroll-up",
        scroll_down => "keymap-actions-pager-scroll-down",
        page_up => "keymap-actions-pager-page-up",
        page_down => "keymap-actions-pager-page-down",
        half_page_up => "keymap-actions-pager-half-page-up",
        half_page_down => "keymap-actions-pager-half-page-down",
        jump_top => "keymap-actions-pager-jump-top",
        jump_bottom => "keymap-actions-pager-jump-bottom",
        close => "keymap-actions-pager-close",
        close_transcript => "keymap-actions-pager-close-transcript",
        find => "keymap-actions-pager-find",
        mark_markdown => "keymap-actions-pager-mark-markdown",
    }
    list "keymap-actions-ctx-list" {
        move_up => "keymap-actions-list-move-up",
        move_down => "keymap-actions-list-move-down",
        move_left => "keymap-actions-list-move-left",
        move_right => "keymap-actions-list-move-right",
        page_up => "keymap-actions-list-page-up",
        page_down => "keymap-actions-list-page-down",
        jump_top => "keymap-actions-list-jump-top",
        jump_bottom => "keymap-actions-list-jump-bottom",
        accept => "keymap-actions-list-accept",
        cancel => "keymap-actions-list-cancel",
    }
    agents "keymap-actions-ctx-agents" {
        resume => "keymap-actions-agents-resume",
        search => "keymap-actions-agents-search",
        new_task => "keymap-actions-agents-new-task",
        new_worktree => "keymap-actions-agents-new-worktree",
        fork => "keymap-actions-agents-fork",
        rename => "keymap-actions-agents-rename",
        stop => "keymap-actions-agents-stop",
        archive => "keymap-actions-agents-archive",
        delete => "keymap-actions-agents-delete",
        hide => "keymap-actions-agents-hide",
        toggle_grouping => "keymap-actions-agents-toggle-grouping",
    }
    approval "keymap-actions-ctx-approval" {
        open_fullscreen => "keymap-actions-approval-open-fullscreen",
        open_thread => "keymap-actions-approval-open-thread",
        approve => "keymap-actions-approval-approve",
        approve_for_session => "keymap-actions-approval-approve-for-session",
        approve_for_prefix => "keymap-actions-approval-approve-for-prefix",
        deny => "keymap-actions-approval-deny",
        decline => "keymap-actions-approval-decline",
        cancel => "keymap-actions-approval-cancel",
    }
}

/// Convert a stable action identifier into a display label.
///
/// This is intentionally presentation-only: the returned string must never be
/// parsed back into an action name, because underscores and casing are part of
/// the stable config contract.
pub(crate) fn action_label(action: &str) -> String {
    if action == "change_host" {
        return crate::i18n::tr!("keymap-actions-change-where-to-run").to_string();
    }
    action
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Format an action's active single-key and chord alternatives in config order.
///
/// Duplicate runtime variants that normalize to the same config spec are shown
/// once so compatibility defaults do not appear as separate user choices.
pub(super) fn format_action_binding_summary(
    runtime_keymap: &RuntimeKeymap,
    context: &str,
    action: &str,
) -> String {
    let specs = super::active_binding_specs(runtime_keymap, context, action).unwrap_or_else(|_| {
        bindings_for_action(runtime_keymap, context, action)
            .unwrap_or_default()
            .iter()
            .filter_map(|binding| super::binding_to_config_key_spec(*binding).ok())
            .collect()
    });
    let mut seen = BTreeSet::new();
    let specs = specs
        .into_iter()
        .filter(|spec| seen.insert(spec.clone()))
        .map(|spec| {
            let Some(completion) = spec.strip_prefix("leader ") else {
                return spec;
            };
            let resolved = runtime_keymap
                .chords
                .leader
                .iter()
                .filter_map(|prefix| super::binding_to_config_key_spec(*prefix).ok())
                .map(|prefix| format!("{prefix} {completion}"))
                .collect::<Vec<_>>();
            if resolved.is_empty() {
                crate::i18n::tr!("keymap-actions-binding-inactive", spec = spec)
            } else {
                crate::i18n::tr!(
                    "keymap-actions-binding-resolved",
                    spec = spec,
                    resolved = resolved.join(", ")
                )
            }
        })
        .collect::<Vec<_>>();
    if specs.is_empty() {
        crate::i18n::tr!("keymap-actions-unbound").to_string()
    } else {
        specs.join(", ")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum KeymapDebugBindingSource {
    Custom,
    CustomGlobal,
    Default,
}

impl KeymapDebugBindingSource {
    pub(super) fn label(&self) -> &'static str {
        match self {
            Self::Custom => crate::i18n::tr!("keymap-actions-source-custom"),
            Self::CustomGlobal => crate::i18n::tr!("keymap-actions-source-custom-global"),
            Self::Default => crate::i18n::tr!("keymap-actions-source-default"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct KeymapDebugActionMatch {
    pub(super) context: &'static str,
    pub(super) action: &'static str,
    pub(super) label: String,
    pub(super) description: &'static str,
    pub(super) source: KeymapDebugBindingSource,
}

pub(super) fn matching_actions_for_key_event(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    event: KeyEvent,
) -> Vec<KeymapDebugActionMatch> {
    KEYMAP_ACTIONS
        .iter()
        .filter_map(|descriptor| {
            let bindings =
                bindings_for_action(runtime_keymap, descriptor.context, descriptor.action)?;
            bindings
                .iter()
                .any(|binding| binding.is_press(event))
                .then(|| KeymapDebugActionMatch {
                    context: descriptor.context,
                    action: descriptor.action,
                    label: action_label(descriptor.action),
                    description: descriptor.description,
                    source: debug_binding_source(keymap_config, descriptor),
                })
        })
        .collect()
}

fn debug_binding_source(
    keymap_config: &TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> KeymapDebugBindingSource {
    let mut keymap_config = keymap_config.clone();
    let Some(slot) = binding_slot(&mut keymap_config, descriptor.context, descriptor.action) else {
        return KeymapDebugBindingSource::Default;
    };
    if slot.is_some() {
        return KeymapDebugBindingSource::Custom;
    }

    let Some(global_slot) = global_fallback_slot(&mut keymap_config, descriptor) else {
        return KeymapDebugBindingSource::Default;
    };
    if global_slot.is_some() {
        KeymapDebugBindingSource::CustomGlobal
    } else {
        KeymapDebugBindingSource::Default
    }
}

fn global_fallback_slot<'a>(
    keymap: &'a mut TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> Option<&'a mut Option<KeybindingsSpec>> {
    if descriptor.context != "composer" {
        return None;
    }

    match descriptor.action {
        "submit" => Some(&mut keymap.global.submit),
        "queue" => Some(&mut keymap.global.queue),
        "toggle_shortcuts" => Some(&mut keymap.global.toggle_shortcuts),
        _ => None,
    }
}
