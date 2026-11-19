use codex_app_server_protocol::ExternalAgentConfigMigrationItem;
use codex_app_server_protocol::ExternalAgentConfigMigrationItemType;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub(crate) struct ExternalAgentConfigMigrationGroupModel {
    pub(crate) label: String,
    pub(crate) description: &'static str,
    pub(crate) item_indices: Vec<usize>,
}

pub(crate) fn external_agent_config_migration_groups(
    items: &[ExternalAgentConfigMigrationItem],
) -> Vec<ExternalAgentConfigMigrationGroupModel> {
    let tools_and_setup = items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            (item.cwd.is_none() && item.item_type != ExternalAgentConfigMigrationItemType::Sessions)
                .then_some(idx)
        })
        .collect::<Vec<_>>();
    let projects = items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            (item.cwd.is_some() && item.item_type != ExternalAgentConfigMigrationItemType::Sessions)
                .then_some(idx)
        })
        .collect::<Vec<_>>();
    let chat_sessions = items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            (item.item_type == ExternalAgentConfigMigrationItemType::Sessions).then_some(idx)
        })
        .collect::<Vec<_>>();

    let mut groups = Vec::new();
    if !tools_and_setup.is_empty() {
        groups.push(ExternalAgentConfigMigrationGroupModel {
            label: crate::i18n::tr!("external-agent-migration-group-tools").to_string(),
            description: crate::i18n::tr!("external-agent-migration-group-tools-desc"),
            item_indices: tools_and_setup,
        });
    }
    if !projects.is_empty() {
        let project_count = projects
            .iter()
            .filter_map(|idx| items[*idx].cwd.as_deref())
            .collect::<BTreeSet<_>>()
            .len();
        groups.push(ExternalAgentConfigMigrationGroupModel {
            label: if project_count == 1 {
                crate::i18n::tr!("external-agent-migration-group-current-project").to_string()
            } else {
                crate::i18n::tr!(
                    "external-agent-migration-group-projects",
                    project_count = project_count as i64
                )
            },
            description: crate::i18n::tr!("external-agent-migration-group-projects-desc"),
            item_indices: projects,
        });
    }
    if !chat_sessions.is_empty() {
        let session_count = chat_sessions
            .iter()
            .filter_map(|idx| items[*idx].details.as_ref())
            .map(|details| details.sessions.len())
            .sum::<usize>();
        groups.push(ExternalAgentConfigMigrationGroupModel {
            label: crate::i18n::tr!(
                "external-agent-migration-group-sessions",
                session_count = session_count as i64
            ),
            description: crate::i18n::tr!("external-agent-migration-group-sessions-desc"),
            item_indices: chat_sessions,
        });
    }
    groups
}

pub(crate) fn external_agent_config_migration_item_label(
    item: &ExternalAgentConfigMigrationItem,
) -> &'static str {
    match item.item_type {
        ExternalAgentConfigMigrationItemType::AgentsMd => {
            crate::i18n::tr!("external-agent-migration-type-instructions")
        }
        ExternalAgentConfigMigrationItemType::Config => {
            crate::i18n::tr!("external-agent-migration-type-settings")
        }
        ExternalAgentConfigMigrationItemType::Skills => {
            crate::i18n::tr!("external-agent-migration-type-skills")
        }
        ExternalAgentConfigMigrationItemType::Plugins => {
            crate::i18n::tr!("external-agent-migration-type-plugins")
        }
        ExternalAgentConfigMigrationItemType::McpServerConfig => {
            crate::i18n::tr!("external-agent-migration-type-mcp-servers")
        }
        ExternalAgentConfigMigrationItemType::Subagents => {
            crate::i18n::tr!("external-agent-migration-type-agents")
        }
        ExternalAgentConfigMigrationItemType::Hooks => {
            crate::i18n::tr!("external-agent-migration-type-hooks")
        }
        ExternalAgentConfigMigrationItemType::Commands => {
            crate::i18n::tr!("external-agent-migration-type-commands")
        }
        ExternalAgentConfigMigrationItemType::Memory => {
            crate::i18n::tr!("external-agent-migration-type-memory")
        }
        ExternalAgentConfigMigrationItemType::Sessions => {
            crate::i18n::tr!("external-agent-migration-item-sessions")
        }
    }
}

pub(crate) fn external_agent_config_migration_type_label(
    item_type: ExternalAgentConfigMigrationItemType,
) -> &'static str {
    match item_type {
        ExternalAgentConfigMigrationItemType::AgentsMd => {
            crate::i18n::tr!("external-agent-migration-type-instructions")
        }
        ExternalAgentConfigMigrationItemType::Config => {
            crate::i18n::tr!("external-agent-migration-type-settings")
        }
        ExternalAgentConfigMigrationItemType::Skills => {
            crate::i18n::tr!("external-agent-migration-type-skills")
        }
        ExternalAgentConfigMigrationItemType::Plugins => {
            crate::i18n::tr!("external-agent-migration-type-plugins")
        }
        ExternalAgentConfigMigrationItemType::McpServerConfig => {
            crate::i18n::tr!("external-agent-migration-type-mcp-servers")
        }
        ExternalAgentConfigMigrationItemType::Subagents => {
            crate::i18n::tr!("external-agent-migration-type-agents")
        }
        ExternalAgentConfigMigrationItemType::Hooks => {
            crate::i18n::tr!("external-agent-migration-type-hooks")
        }
        ExternalAgentConfigMigrationItemType::Commands => {
            crate::i18n::tr!("external-agent-migration-type-commands")
        }
        ExternalAgentConfigMigrationItemType::Memory => {
            crate::i18n::tr!("external-agent-migration-type-memory")
        }
        ExternalAgentConfigMigrationItemType::Sessions => {
            crate::i18n::tr!("external-agent-migration-type-sessions")
        }
    }
}

/// Summarizes the concrete objects represented by selected migration items.
///
/// Most detected item types carry the objects they will import in `details`; types without
/// details represent one importable file or source directory per migration item.
pub(crate) fn external_agent_config_migration_count_summary<'a>(
    items: impl IntoIterator<Item = &'a ExternalAgentConfigMigrationItem>,
) -> String {
    let mut counts = Vec::<(ExternalAgentConfigMigrationItemType, usize)>::new();
    for item in items {
        let count = external_agent_config_migration_item_count(item);
        if let Some((_, type_count)) = counts
            .iter_mut()
            .find(|(item_type, _)| *item_type == item.item_type)
        {
            *type_count += count;
        } else {
            counts.push((item.item_type, count));
        }
    }

    counts
        .into_iter()
        .map(|(item_type, count)| {
            crate::i18n::tr!(
                "external-agent-migration-summary-entry",
                label = external_agent_config_migration_type_label(item_type),
                count = count as i64
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn external_agent_config_migration_item_count(
    item: &ExternalAgentConfigMigrationItem,
) -> usize {
    match item.item_type {
        ExternalAgentConfigMigrationItemType::Plugins => {
            item.details.as_ref().map_or(1, |details| {
                details
                    .plugins
                    .iter()
                    .map(|plugin_group| plugin_group.plugin_names.len())
                    .sum()
            })
        }
        ExternalAgentConfigMigrationItemType::McpServerConfig => item
            .details
            .as_ref()
            .map_or(1, |details| details.mcp_servers.len()),
        ExternalAgentConfigMigrationItemType::Subagents => item
            .details
            .as_ref()
            .map_or(1, |details| details.subagents.len()),
        ExternalAgentConfigMigrationItemType::Hooks => item
            .details
            .as_ref()
            .map_or(1, |details| details.hooks.len()),
        ExternalAgentConfigMigrationItemType::Commands => item
            .details
            .as_ref()
            .map_or(1, |details| details.commands.len()),
        ExternalAgentConfigMigrationItemType::Memory => item
            .details
            .as_ref()
            .map_or(0, |details| details.memory.len()),
        ExternalAgentConfigMigrationItemType::Sessions => item
            .details
            .as_ref()
            .map_or(1, |details| details.sessions.len()),
        ExternalAgentConfigMigrationItemType::Skills => item
            .details
            .as_ref()
            .map_or(1, |details| details.skills.len()),
        ExternalAgentConfigMigrationItemType::AgentsMd
        | ExternalAgentConfigMigrationItemType::Config => 1,
    }
}

pub(crate) fn external_agent_config_migration_item_detail(
    item: &ExternalAgentConfigMigrationItem,
) -> Option<String> {
    let details = item.details.as_ref()?;
    match item.item_type {
        ExternalAgentConfigMigrationItemType::Plugins => None,
        ExternalAgentConfigMigrationItemType::Skills => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-skill",
                count = details.skills.len() as i64
            ),
            details.skills.iter().map(|skill| skill.name.as_str()),
        )),
        ExternalAgentConfigMigrationItemType::McpServerConfig => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-mcp-server",
                count = details.mcp_servers.len() as i64
            ),
            details
                .mcp_servers
                .iter()
                .map(|server| server.name.as_str()),
        )),
        ExternalAgentConfigMigrationItemType::Subagents => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-agent",
                count = details.subagents.len() as i64
            ),
            details.subagents.iter().map(|agent| agent.name.as_str()),
        )),
        ExternalAgentConfigMigrationItemType::Hooks => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-hook",
                count = details.hooks.len() as i64
            ),
            details.hooks.iter().map(|hook| hook.name.as_str()),
        )),
        ExternalAgentConfigMigrationItemType::Commands => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-command",
                count = details.commands.len() as i64
            ),
            details.commands.iter().map(|command| command.name.as_str()),
        )),
        ExternalAgentConfigMigrationItemType::Memory => {
            let memory = &details.memory;
            let count = memory.len();
            let summary = crate::i18n::tr!(
                "external-agent-migration-count-memory",
                count = count as i64
            );
            let names = memory
                .iter()
                .map(String::as_str)
                .take(4)
                .collect::<Vec<_>>();
            Some(if names.is_empty() {
                summary
            } else {
                crate::i18n::tr!(
                    "external-agent-migration-count-with-names",
                    summary = summary,
                    names = names.join(", ")
                )
            })
        }
        ExternalAgentConfigMigrationItemType::Sessions => Some(format_counted_details(
            crate::i18n::tr!(
                "external-agent-migration-count-session",
                count = details.sessions.len() as i64
            ),
            details
                .sessions
                .iter()
                .filter_map(|session| session.title.as_deref()),
        )),
        ExternalAgentConfigMigrationItemType::AgentsMd
        | ExternalAgentConfigMigrationItemType::Config => None,
    }
}

fn format_counted_details<'a>(summary: String, names: impl Iterator<Item = &'a str>) -> String {
    match names.take(4).collect::<Vec<_>>() {
        names if names.is_empty() => summary,
        names => crate::i18n::tr!(
            "external-agent-migration-count-with-names",
            summary = summary,
            names = names.join(", ")
        ),
    }
}
