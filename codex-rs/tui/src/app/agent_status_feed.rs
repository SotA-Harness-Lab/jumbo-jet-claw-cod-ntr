// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

//! Bounded, best-effort previews for the v2 `/subagents` status output.

use super::ThreadBufferedEvent;
use super::ThreadEventStore;
use crate::history_cell::HistoryCell;
use crate::history_cell::plain_lines;
use crate::style::accent_color;
use crate::text_formatting::truncate_text;
use codex_app_server_protocol::CollabAgentTool;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::SubAgentActivityKind;
use codex_app_server_protocol::ThreadItemPayload;
use ratatui::style::Stylize;
use ratatui::text::Line;
use std::collections::HashSet;

const AGENT_STATUS_PREVIEW_LINES: usize = 3;
const AGENT_STATUS_PREVIEW_ITEMS: usize = 6;
const AGENT_STATUS_PREVIEW_GRAPHEMES: usize = 240;
const AGENT_STATUS_PREVIEW_INDENT: u16 = 4;

#[derive(Debug)]
pub(super) struct AgentStatusHistoryCell {
    entries: Vec<AgentStatusThreadPreview>,
}

impl AgentStatusHistoryCell {
    pub(super) fn new(entries: Vec<AgentStatusThreadPreview>) -> Self {
        Self { entries }
    }
}

impl HistoryCell for AgentStatusHistoryCell {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines: Vec<Line<'static>> = vec![
            "/subagents".magenta().into(),
            crate::i18n::tr!("agent-status-feed-heading").bold().into(),
            "".into(),
        ];

        if self.entries.is_empty() {
            lines.push(crate::i18n::tr!("agent-status-feed-none").italic().into());
            return lines;
        }

        for entry in &self.entries {
            lines.push(entry.title_line());
            let preview_width = width.saturating_sub(AGENT_STATUS_PREVIEW_INDENT).max(1);
            let preview_lines = entry.preview_lines(preview_width);
            if preview_lines.is_empty() {
                lines.push(
                    vec![
                        "    ".into(),
                        crate::i18n::tr!("agent-status-feed-no-activity")
                            .dim()
                            .italic(),
                    ]
                    .into(),
                );
            } else {
                lines.extend(preview_lines.into_iter().map(indent_preview_line));
            }
            lines.push("".into());
        }
        let _ = lines.pop();
        lines
    }

    fn raw_lines(&self) -> Vec<Line<'static>> {
        plain_lines(self.display_lines(u16::MAX))
    }
}

#[derive(Debug)]
pub(super) struct AgentStatusThreadPreview {
    agent_path: String,
    activity: Vec<String>,
}

impl AgentStatusThreadPreview {
    pub(super) fn from_store(agent_path: String, store: &ThreadEventStore) -> Self {
        Self::from_events(agent_path, store.buffer.iter().rev())
    }

    pub(super) fn empty(agent_path: String) -> Self {
        Self::from_events(agent_path, std::iter::empty())
    }

    fn from_events<'a>(
        agent_path: String,
        events: impl Iterator<Item = &'a ThreadBufferedEvent>,
    ) -> Self {
        let mut seen_item_ids = HashSet::new();
        let mut activity = Vec::new();
        for event in events {
            let item = match event {
                ThreadBufferedEvent::Notification(notification) => match notification.as_ref() {
                    ServerNotification::ItemCompleted(event) => &event.item,
                    ServerNotification::ItemStarted(event) => &event.item,
                    _ => continue,
                },
                ThreadBufferedEvent::Request(_)
                | ThreadBufferedEvent::HistoryEntryResponse(_)
                | ThreadBufferedEvent::FeedbackSubmission(_) => continue,
            };
            if !seen_item_ids.insert(item.id().to_string()) {
                continue;
            }
            if let Some(summary) = activity_summary(&item.payload) {
                activity.push(summary);
                if activity.len() == AGENT_STATUS_PREVIEW_ITEMS {
                    break;
                }
            }
        }
        activity.reverse();
        Self {
            agent_path,
            activity,
        }
    }

    fn title_line(&self) -> Line<'static> {
        vec![
            "  • ".dim(),
            format!("`{}`", self.agent_path).fg(accent_color()),
        ]
        .into()
    }

    fn preview_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut lines = self
            .activity
            .iter()
            .flat_map(|activity| textwrap::wrap(activity, width as usize))
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.into_owned().dim().into())
            .collect::<Vec<_>>();
        if lines.len() > AGENT_STATUS_PREVIEW_LINES {
            lines.drain(..lines.len() - AGENT_STATUS_PREVIEW_LINES);
        }
        lines
    }
}

fn activity_summary(item: &ThreadItemPayload) -> Option<String> {
    let summary = match item {
        ThreadItemPayload::AgentMessage { text, .. } | ThreadItemPayload::Plan { text, .. } => text,
        ThreadItemPayload::Reasoning { summary, .. } => summary.last()?,
        ThreadItemPayload::CommandExecution { command, .. } => {
            let command = truncate_text(
                command,
                AGENT_STATUS_PREVIEW_GRAPHEMES.saturating_sub("$ ".len()),
            );
            return bounded_summary(&format!("$ {command}"));
        }
        ThreadItemPayload::FileChange { changes, .. } => {
            return bounded_summary(&crate::i18n::tr!(
                "agent-status-feed-updated-files",
                count = changes.len()
            ));
        }
        ThreadItemPayload::McpToolCall { server, tool, .. } => {
            return bounded_summary(&format!("MCP {server}/{tool}"));
        }
        ThreadItemPayload::DynamicToolCall {
            namespace, tool, ..
        } => {
            let tool = namespace
                .as_ref()
                .map(|namespace| format!("{namespace}/{tool}"))
                .unwrap_or_else(|| tool.clone());
            return bounded_summary(&crate::i18n::tr!("agent-status-feed-tool", tool = tool));
        }
        ThreadItemPayload::CollabAgentToolCall { tool, .. } => {
            let action = match tool {
                CollabAgentTool::SendMessage
                | CollabAgentTool::FollowupTask
                | CollabAgentTool::InterruptAgent
                | CollabAgentTool::ListAgents => return None,
                CollabAgentTool::SpawnAgent => crate::i18n::tr!("agent-status-feed-spawned"),
                CollabAgentTool::SendInput => crate::i18n::tr!("agent-status-feed-sent-input"),
                CollabAgentTool::ResumeAgent => crate::i18n::tr!("agent-status-feed-resumed"),
                CollabAgentTool::Wait => crate::i18n::tr!("agent-status-feed-waited"),
                CollabAgentTool::CloseAgent => crate::i18n::tr!("agent-status-feed-closed"),
            };
            return Some(action.to_string());
        }
        ThreadItemPayload::SubAgentActivity {
            kind, agent_path, ..
        } => {
            let action = match kind {
                SubAgentActivityKind::Started => crate::i18n::tr!("agent-status-feed-started"),
                SubAgentActivityKind::Interacted => crate::i18n::tr!("agent-status-feed-contacted"),
                SubAgentActivityKind::Interrupted => {
                    crate::i18n::tr!("agent-status-feed-interrupted")
                }
                SubAgentActivityKind::Completed => crate::i18n::tr!("agent-status-feed-completed"),
            };
            return bounded_summary(&crate::i18n::tr!(
                "agent-status-feed-action-agent",
                action = action,
                agent_path = agent_path.to_string()
            ));
        }
        ThreadItemPayload::WebSearch(item) => {
            return bounded_summary(&crate::i18n::tr!(
                "agent-status-feed-web-search",
                query = item.query.to_string()
            ));
        }
        ThreadItemPayload::ImageView { path, .. } => {
            let path = path.render_for_ui();
            return bounded_summary(&crate::i18n::tr!("agent-status-feed-viewed", path = path));
        }
        ThreadItemPayload::ImageGeneration(_) => {
            return Some(crate::i18n::tr!("agent-status-feed-generated-image").to_string());
        }
        ThreadItemPayload::EnteredReviewMode { .. } => {
            return Some(crate::i18n::tr!("agent-status-feed-entered-review").to_string());
        }
        ThreadItemPayload::ExitedReviewMode { .. } => {
            return Some(crate::i18n::tr!("agent-status-feed-exited-review").to_string());
        }
        ThreadItemPayload::ContextCompaction { .. } => {
            return Some(crate::i18n::tr!("agent-status-feed-compacted").to_string());
        }
        ThreadItemPayload::UserMessage { .. }
        | ThreadItemPayload::HookPrompt { .. }
        | ThreadItemPayload::FunctionCallOutput { .. }
        | ThreadItemPayload::Sleep(_) => {
            return None;
        }
    };
    bounded_summary(summary)
}

fn bounded_summary(summary: &str) -> Option<String> {
    let summary = truncate_text(summary, AGENT_STATUS_PREVIEW_GRAPHEMES);
    let summary = summary.split_whitespace().collect::<Vec<_>>().join(" ");
    (!summary.is_empty()).then_some(summary)
}

fn indent_preview_line(mut line: Line<'static>) -> Line<'static> {
    line.spans.insert(0, "    ".into());
    line
}

#[cfg(test)]
#[path = "agent_status_feed_tests.rs"]
mod tests;
