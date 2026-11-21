// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Reading Claude Code's configuration files the way Claude Code does.
//!
//! Codex uses these files in place: nothing here writes, copies or converts a file on
//! disk. The parsing is pure; the loaders reach the filesystem through [`FileSource`], so
//! that the same code serves the user's home directory and a project on an executor.

mod env_file;
pub mod file_history;
mod hook_condition;
mod instructions;
mod markdown;
mod mcp;
mod memory;
mod memory_instructions;
mod mod_config;
mod permissions;
mod plugin_vars;
mod plugins;
mod rules;
mod settings;
mod skill_frontmatter;
mod skill_invocation;
mod source;
mod subagents;
pub mod tools;

pub use env_file::EnvAssignment;
pub use env_file::apply_env_assignments;
pub use env_file::hook_env_dir;
pub use env_file::hook_env_file;
pub use env_file::parse_env_file;
pub use env_file::read_hook_env_dir;
pub use hook_condition::hook_condition_matches;
pub use instructions::INSTRUCTIONS_HEADER;
pub use instructions::InstructionFile;
pub use instructions::InstructionKind;
pub use instructions::directory_instructions;
pub use instructions::file_instructions;
pub use instructions::load_instruction_file;
pub use instructions::render_instruction_files;
pub use instructions::rendered_instruction_paths;
pub use instructions::user_instructions;
pub use markdown::Document;
pub use markdown::import_paths;
pub use markdown::parse_document;
pub use markdown::strip_block_comments;
pub use mcp::HEADERS_HELPER;
pub use mcp::McpTimeouts;
pub use mcp::claude_json_project_root;
pub use mcp::headers_helper_script;
pub use mcp::expand_mcp_server;
pub use mcp::mcp_json_paths;
pub use mcp::mcp_server_declarations;
pub use mcp::mcp_timeouts;
pub use memory::auto_memory_dir;
pub use memory::auto_memory_project_root;
pub use memory::truncate_memory_index;
pub use memory_instructions::auto_memory_base_dir;
pub use memory_instructions::auto_memory_dir_override;
pub use memory_instructions::auto_memory_enabled;
pub use memory_instructions::auto_memory_instructions;
pub use mod_config::ClaudeModConfig;
pub use mod_config::ClaudePluginsModConfig;
pub use mod_config::HarnessModConfig;
pub use mod_config::HooksModConfig;
pub use mod_config::MOD_CONFIG_FILE;
pub use mod_config::ModConfig;
pub use mod_config::UiModConfig;
pub use permissions::MAX_LINK_DEPTH;
pub use permissions::PermissionRule;
pub use permissions::PermissionRules;
pub use permissions::RuleSource;
pub use permissions::RuleVerdict;
pub use permissions::ToolInput;
pub use permissions::link_target;
pub use permissions::parse_rule as parse_permission_rule;
pub use plugin_vars::MappedCommand;
pub use plugin_vars::PluginVariables;
pub use plugin_vars::mapped_plugin_command;
pub use plugin_vars::plugin_root_of;
pub use plugin_vars::substitute_in_plugin_content;
pub use plugins::ClaudePlugin;
pub use plugins::INSTALLED_PLUGINS_FILE;
pub use plugins::applicable_plugins;
pub use plugins::enabled_plugin_ids;
pub use plugins::incompatibility;
pub use plugins::installed_enabled_plugins;
pub use plugins::installed_plugin_paths;
pub use plugins::load_claude_plugins;
pub use plugins::load_claude_plugins_for;
pub use rules::RuleFile;
pub use rules::glob_matches;
pub use rules::load_rules;
pub use rules::rule_globs;
pub use settings::BashPrefixRule;
pub use settings::PermissionDecision;
pub use settings::SettingsStamp;
pub use settings::bash_prefix_rules;
pub use settings::local_settings_paths;
pub use settings::read_settings;
pub use settings::settings_effort_level;
pub use settings::settings_env;
pub use settings::settings_paths;
pub use settings::settings_stamp;
pub use skill_frontmatter::SkillFrontmatter;
pub use skill_frontmatter::skill_frontmatter;
pub use skill_invocation::argument_names;
pub use skill_invocation::parse_arguments;
pub use skill_invocation::render_invoked_skill;
pub use skill_invocation::slash_invocation_arguments;
pub use skill_invocation::substitute_arguments;
pub use skill_invocation::substitute_skill_variables;
pub use source::FileSource;
pub use source::LocalFiles;
pub use source::SourceFuture;
pub use subagents::Subagent;
pub use subagents::parse_subagent;
pub use subagents::parse_subagent_at;

/// Name of Claude Code's configuration directory, in the home directory and in a project.
pub const CLAUDE_DIR: &str = ".claude";

const COMPAT_ENV: &str = "CCN_CLAUDE_COMPAT";

/// Whether Claude Code's configuration is shared with Codex: on unless
/// `CCN_CLAUDE_COMPAT=0`.
pub fn enabled() -> bool {
    std::env::var_os(COMPAT_ENV).is_none_or(|value| value != "0")
}
