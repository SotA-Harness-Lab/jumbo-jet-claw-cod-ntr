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
    match index {
        0 => crate::i18n::tr!("agent-center-filter-all"),
        1 => crate::i18n::tr!("agent-center-filter-needs-you"),
        2 => crate::i18n::tr!("agent-center-filter-working"),
        3 => crate::i18n::tr!("agent-center-filter-ready"),
        _ => crate::i18n::tr!("agent-center-filter-inactive"),
    }
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
