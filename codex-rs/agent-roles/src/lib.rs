// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: read Claude Code's configuration in place

mod agent_role_config;
mod discovery;
mod loader;

pub use agent_role_config::AgentRoleConfig;
pub use agent_role_config::ResolvedAgentRoleFile;
pub use agent_role_config::parse_agent_role_file_contents;
pub use loader::load_agent_roles;
pub use loader::load_claude_code_agent_role_files;
pub use loader::load_claude_code_agent_roles;
