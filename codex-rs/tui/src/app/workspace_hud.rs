//! Present pending shortcuts with workspace-specific labels when pane controls are active.
//! Dispatch stays with the originating control, including before the reveal delay elapses.

use super::App;
use crate::bottom_pane::ShortcutHud;
use ratatui::layout::Rect;
use ratatui::style::Stylize;

impl App {
    pub(super) fn workspace_hud(&self, tui: &crate::tui::Tui, area: Rect) -> Option<ShortcutHud> {
        let workspace =
            self.conversation_focus_available(tui) && !self.conversation_frame.has_caller_inset();
        if workspace && self.chat_widget.quick_dial_menu_open() {
            return Some(self.chat_widget.quick_dial_hud());
        }
        let pending = self
            .key_chord_matcher
            .visible_pending(&tui.frame_requester())?;
        if !workspace
            || (self.key_chord_matcher.workspace_leader().is_none()
                && !pending
                    .completions(&self.keymap.chords)
                    .any(|binding| binding.action.is_side_pane()))
        {
            return pending.guide(&self.keymap.chords, |_| true);
        }
        let side = self.sidebar_has_focus();
        let can_switch = self.panel_focus().is_some() || self.switchable_side_child().is_some();
        let can_size = super::side_pane::split_layout(self, tui, area).is_some();
        let can_resize = can_size && side && self.zoomed_thread.is_none();
        let mut groups = [
            (crate::i18n::tr!("workspace-hud-panes"), Vec::new()),
            (crate::i18n::tr!("workspace-hud-move"), Vec::new()),
            (crate::i18n::tr!("workspace-hud-this-pane"), Vec::new()),
        ];
        for (action, keys) in pending.actions(&self.keymap.chords) {
            let (group, label) = match action.action {
                "focus_command_center" => (0, crate::i18n::tr!("workspace-hud-command-center")),
                "focus_main_composer" => (0, crate::i18n::tr!("workspace-hud-main-composer")),
                "focus_sidebar_tabs" => (0, crate::i18n::tr!("workspace-hud-open-in-sidebar")),
                "focus_sidebar_content" if side => {
                    (0, crate::i18n::tr!("workspace-hud-hide-sidebar"))
                }
                "focus_sidebar_content" => (0, crate::i18n::tr!("workspace-hud-sidebar-content")),
                "resize_panes" if can_size => (2, crate::i18n::tr!("workspace-hud-resize-panes")),
                "resize_panes" => continue,
                "minimize_pane_width" if can_size => {
                    (2, crate::i18n::tr!("workspace-hud-minimum-width"))
                }
                "maximize_pane_width" if can_size => {
                    (2, crate::i18n::tr!("workspace-hud-maximum-width"))
                }
                "equalize_pane_widths" if can_size => {
                    (2, crate::i18n::tr!("workspace-hud-equal-widths"))
                }
                "minimize_pane_width" | "maximize_pane_width" | "equalize_pane_widths" => continue,
                "move_divider_left" if can_resize => {
                    (2, crate::i18n::tr!("workspace-hud-divider-left"))
                }
                "move_divider_right" if can_resize => {
                    (2, crate::i18n::tr!("workspace-hud-divider-right"))
                }
                "toggle_pane_zoom" if self.zoomed_thread.is_some() => {
                    (2, crate::i18n::tr!("workspace-hud-restore-layout"))
                }
                "toggle_pane_zoom" => (2, crate::i18n::tr!("workspace-hud-zoom-pane")),
                "move_divider_left" | "move_divider_right" => continue,
                "close_sidebar_tab" if self.active_side_parent_thread_id().is_some() => {
                    (2, crate::i18n::tr!("workspace-hud-close-tab"))
                }
                "close_sidebar_tab" => continue,
                "sidebar_up" => (1, crate::i18n::tr!("workspace-hud-previous-tab")),
                "sidebar_down" => (1, crate::i18n::tr!("workspace-hud-next-tab")),
                "hide_pane" if side => (2, crate::i18n::tr!("workspace-hud-hide-pane")),
                "focus_pane_left" if side && can_switch => {
                    (1, crate::i18n::tr!("workspace-hud-pane-left"))
                }
                "focus_pane_right" if !side && can_switch && self.open_pane_focus().is_some() => {
                    (1, crate::i18n::tr!("workspace-hud-pane-right"))
                }
                "hide_pane" | "focus_pane_left" | "focus_pane_right" => continue,
                "choose_cloud_environment" => {
                    (2, crate::i18n::tr!("workspace-hud-choose-environment"))
                }
                "open_presets" => (2, crate::i18n::tr!("workspace-hud-presets")),
                "find_transcript" => (2, crate::i18n::tr!("workspace-hud-find-text")),
                "select_transcript" => (2, crate::i18n::tr!("workspace-hud-select-transcript")),
                "mark_markdown" => (2, crate::i18n::tr!("workspace-hud-mark-table-cell")),
                "focus_conversation_composer" => (2, crate::i18n::tr!("workspace-hud-composer")),
                other => (2, other),
            };
            let key = keys.join(" / ").fg(crate::style::accent_color());
            groups[group].1.push((key, label.replace('_', " ")));
        }
        let origin = if side {
            crate::i18n::tr!("workspace-hud-sidebar")
        } else {
            crate::i18n::tr!("workspace-hud-conversation")
        };
        Some(ShortcutHud::leader(
            crate::i18n::tr!(
                "workspace-hud-leader-title",
                prefix = pending.prefix.display_label(),
                origin = origin
            ),
            groups
                .into_iter()
                .filter_map(|(title, items)| {
                    let width = items.iter().map(|(key, _)| key.width()).max()?.max(7);
                    let lines = items
                        .into_iter()
                        .map(|(key, label)| {
                            let gap = " ".repeat(width - key.width() + 1);
                            vec![key, gap.into(), label.into()].into()
                        })
                        .collect();
                    Some((title, lines))
                })
                .collect(),
        ))
    }
}
