// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Goal summary for the bare `/goal` command.

use super::*;
use crate::goal_display::format_goal_elapsed_seconds;
use crate::goal_files;
use crate::status::format_tokens_compact;

impl ChatWidget {
    pub(crate) fn show_goal_summary(&mut self, goal: AppThreadGoal) {
        self.add_plain_history_lines(goal_summary_lines(&goal));
    }

    pub(crate) fn show_goal_edit_prompt(&mut self, thread_id: ThreadId, goal: AppThreadGoal) {
        let tx = self.app_event_tx.clone();
        let status = edited_goal_status(goal.status);
        let token_budget = goal.token_budget;
        let view = CustomPromptView::new(
            crate::i18n::tr!("goal-menu-edit-goal").to_string(),
            crate::i18n::tr!("goal-menu-type-objective").to_string(),
            goal.objective,
            /*context_label*/ None,
            Box::new(move |objective: String| {
                tx.send(AppEvent::SetThreadGoalDraft {
                    thread_id,
                    draft: goal_files::GoalDraft {
                        objective,
                        ..Default::default()
                    },
                    mode: crate::app_event::ThreadGoalSetMode::UpdateExisting {
                        status,
                        token_budget,
                    },
                });
            }),
        );
        self.bottom_pane.show_text_prompt(view);
    }

    pub(crate) fn show_resume_paused_goal_prompt(
        &mut self,
        thread_id: ThreadId,
        objective: String,
    ) {
        let resume_actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
            tx.send(AppEvent::SetThreadGoalStatus {
                thread_id,
                status: AppThreadGoalStatus::Active,
            });
        })];
        self.show_selection_view(SelectionViewParams {
            title: Some(crate::i18n::tr!("goal-menu-resume-paused-title").to_string()),
            subtitle: Some(crate::i18n::tr!(
                "goal-menu-goal-objective",
                objective = objective
            )),
            footer_hint: Some(standard_popup_hint_line()),
            initial_selected_idx: Some(0),
            items: vec![
                SelectionItem {
                    name: crate::i18n::tr!("goal-menu-resume-goal").to_string(),
                    description: Some(crate::i18n::tr!("goal-menu-resume-goal-desc").to_string()),
                    actions: resume_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: crate::i18n::tr!("goal-menu-leave-paused").to_string(),
                    description: Some(crate::i18n::tr!("goal-menu-leave-paused-desc").to_string()),
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..SelectionViewParams::confirmation()
        });
    }

    pub(crate) fn on_thread_goal_cleared(&mut self, thread_id: &str) {
        if self
            .thread_id
            .is_some_and(|active_thread_id| active_thread_id.to_string() == thread_id)
        {
            self.current_goal_status = None;
            self.update_collaboration_mode_indicator();
        }
    }
}

fn goal_summary_lines(goal: &AppThreadGoal) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(crate::i18n::tr!("goal-menu-goal").bold()),
        Line::from(vec![
            crate::i18n::tr!("goal-menu-status").dim(),
            goal_status_label(goal.status).to_string().into(),
        ]),
        Line::from(vec![
            crate::i18n::tr!("goal-menu-objective").dim(),
            goal.objective.clone().into(),
        ]),
        Line::from(vec![
            crate::i18n::tr!("goal-menu-time-used").dim(),
            format_goal_elapsed_seconds(goal.time_used_seconds).into(),
        ]),
        Line::from(vec![
            crate::i18n::tr!("goal-menu-tokens-used").dim(),
            format_tokens_compact(goal.tokens_used).into(),
        ]),
    ];
    if let Some(token_budget) = goal.token_budget {
        lines.push(Line::from(vec![
            crate::i18n::tr!("goal-menu-token-budget").dim(),
            format_tokens_compact(token_budget).into(),
        ]));
    }
    let command_hint = match goal.status {
        AppThreadGoalStatus::Active => crate::i18n::tr!("goal-menu-commands-active"),
        AppThreadGoalStatus::Paused
        | AppThreadGoalStatus::Blocked
        | AppThreadGoalStatus::UsageLimited => crate::i18n::tr!("goal-menu-commands-paused"),
        AppThreadGoalStatus::BudgetLimited | AppThreadGoalStatus::Complete => {
            crate::i18n::tr!("goal-menu-commands-done")
        }
    };
    lines.push(Line::default());
    lines.push(Line::from(command_hint.dim()));
    lines
}

fn goal_status_label(status: AppThreadGoalStatus) -> &'static str {
    match status {
        AppThreadGoalStatus::Active => crate::i18n::tr!("goal-menu-status-active"),
        AppThreadGoalStatus::Paused => crate::i18n::tr!("goal-menu-status-paused"),
        AppThreadGoalStatus::Blocked => crate::i18n::tr!("goal-menu-status-stalled"),
        AppThreadGoalStatus::UsageLimited => crate::i18n::tr!("goal-menu-status-usage-limited"),
        AppThreadGoalStatus::BudgetLimited => crate::i18n::tr!("goal-menu-status-budget-limited"),
        AppThreadGoalStatus::Complete => crate::i18n::tr!("goal-menu-status-complete"),
    }
}

fn edited_goal_status(status: AppThreadGoalStatus) -> AppThreadGoalStatus {
    match status {
        AppThreadGoalStatus::Active => AppThreadGoalStatus::Active,
        AppThreadGoalStatus::Paused
        | AppThreadGoalStatus::Blocked
        | AppThreadGoalStatus::UsageLimited => status,
        AppThreadGoalStatus::BudgetLimited | AppThreadGoalStatus::Complete => {
            AppThreadGoalStatus::Active
        }
    }
}
