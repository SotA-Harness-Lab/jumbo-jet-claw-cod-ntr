// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Status filters and viewport state survive refreshes of the shared task projection.

mod hints;
mod input;
mod navigation;
mod render;
mod rows;

use super::*;

// Counts and filtering use the same status groups.
pub(super) const TASK_FILTERS: &[(&str, Option<AgentsOverviewGroup>)] = &[
    ("All", None),
    ("Needs you", Some(AgentsOverviewGroup::NeedsYou)),
    ("Working", Some(AgentsOverviewGroup::Working)),
    ("Ready", Some(AgentsOverviewGroup::Ready)),
    ("Inactive", Some(AgentsOverviewGroup::Finished)),
];

/// The label of the task filter at `index` of [`TASK_FILTERS`], in the TUI's language.
pub(super) fn task_filter_label(index: usize) -> &'static str {
    let label = match index {
        0 => "All",
        1 => "Needs you",
        2 => "Working",
        3 => "Ready",
        _ => "Inactive",
    };
    crate::i18n::translate_in("agent-filter-label", label)
}

impl AgentsOverviewView {
    pub(in crate::app::agents_overview_view) fn selectable_indices(&self) -> Vec<usize> {
        let mut indices = self.visible_indices();
        if self.state().more_available() {
            indices.push(usize::MAX);
        }
        indices
    }

    pub(in crate::app::agents_overview_view) fn reconcile_command_center_selection(&mut self) {
        let visible = self.selectable_indices();
        if !visible.contains(&self.selected) {
            self.selected = visible.first().copied().unwrap_or(usize::MAX);
            self.state().show_more_selected = false;
        }
    }
}
