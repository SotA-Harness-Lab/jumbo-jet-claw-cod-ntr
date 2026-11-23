//! Resolves literal Apps wording with a UTF-8 byte limit and the existing bundled namespace.
//! Configured strings are never interpolated or trimmed.

use codex_protocol::openai_models::AppsMessages;

// Reserve 200 bytes for markers and transition notices within a 1,000-byte fragment.
const MAX_USAGE_INSTRUCTIONS_BYTES: usize = 800;

/// Bounded Apps wording and its source, independently of connector availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedAppsInstructions {
    pub usage_instructions: String,
    pub catalog_override: bool,
}

impl ResolvedAppsInstructions {
    pub(super) fn new(messages: Option<&AppsMessages>, mcp_server_name: &str) -> Self {
        let catalog = messages
            .and_then(|messages| messages.usage_instructions.as_deref())
            .filter(|text| {
                text.is_empty()
                    || (text.len() <= MAX_USAGE_INSTRUCTIONS_BYTES && !text.trim().is_empty())
            });
        Self {
            usage_instructions: catalog.map(str::to_owned).unwrap_or_else(|| {
                format!(
                    codex_prompt_texts::text!(
                        "templates/model_messages/apps/usage_instructions.md"
                    ),
                    mcp_server_name = mcp_server_name
                )
            }),
            catalog_override: catalog.is_some(),
        }
    }
}
