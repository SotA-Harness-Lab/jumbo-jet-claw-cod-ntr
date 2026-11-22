// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Shows saved Cloud environments before starting a conversation on the Cloud backend.

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use codex_app_server_client::SavedEnvironment;
use codex_protocol::ThreadId;

pub(super) const LOADING_VIEW_ID: &str = "cloud_environment_loading";

pub(super) fn loading_view() -> SelectionViewParams {
    SelectionViewParams {
        view_id: Some(LOADING_VIEW_ID),
        title: Some("Cloud environment".into()),
        subtitle: Some("Loading saved environments…".into()),
        ..SelectionViewParams::picker()
    }
}

pub(super) fn cloud_environment_picker(
    result: Result<Vec<SavedEnvironment>, String>,
    source: ThreadId,
) -> SelectionViewParams {
    let (environments, subtitle) = match result {
        Ok(environments) => (environments, None),
        Err(error) => (
            Vec::new(),
            Some(tr!("{error}. You can still use Default.", error)),
        ),
    };
    let choices = std::iter::once((
        tr!("Default").to_string(),
        tr!("Use the default environment with the current GitHub project, if available"),
        None,
    ))
    .chain(environments.into_iter().map(|environment| {
        (
            environment.name.clone(),
            if environment.workspace_shared {
                tr!("Workspace environment")
            } else {
                tr!("Personal environment")
            },
            Some(environment),
        )
    }));
    let items = choices
        .map(|(name, description, saved_environment)| SelectionItem {
            search_value: Some(name.clone()),
            name,
            description: Some(description.to_string()),
            actions: vec![Box::new(move |tx| {
                tx.send(AppEvent::StartCloudConversation {
                    source,
                    saved_environment: saved_environment.clone(),
                });
            })],
            dismiss_on_select: true,
            ..Default::default()
        })
        .collect();
    SelectionViewParams {
        title: Some("Cloud environment".into()),
        subtitle,
        items,
        is_searchable: true,
        search_placeholder: Some("Search environments".to_string()),
        ..SelectionViewParams::picker()
    }
}

#[cfg(test)]
#[path = "cloud_environments_tests.rs"]
mod tests;
