// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: fill in a Claude Code plugin's variables

use codex_config::McpServerAuth;
use codex_config::McpServerConfig;
use codex_config::McpServerEnvVar;
use codex_config::McpServerTransportConfig;
use codex_utils_path_uri::LegacyAppPathString;
use codex_utils_path_uri::PathUri;
use serde::Deserialize;
use serde_json::Map as JsonMap;
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use std::path::Path;
use tracing::warn;

#[path = "agent_plugin_config.rs"]
mod agent_plugin_config;

pub use agent_plugin_config::parse_agent_plugin_mcp_config;

#[derive(Clone, Copy, Debug)]
enum PluginMcpSource<'a> {
    Host {
        root: &'a Path,
    },
    Environment {
        root: &'a PathUri,
        environment_id: &'a str,
    },
}

/// One plugin MCP server that could not be normalized into runtime configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginMcpServerParseError {
    pub name: String,
    pub message: String,
}

/// Valid servers and per-server errors parsed from one plugin MCP file.
#[derive(Debug, Default, PartialEq)]
pub struct PluginMcpConfigParseOutcome {
    pub servers: BTreeMap<String, McpServerConfig>,
    pub errors: Vec<PluginMcpServerParseError>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PluginMcpServersFile {
    mcp_servers: BTreeMap<String, JsonValue>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PluginMcpFile {
    McpServersObject(PluginMcpServersFile),
    ServerMap(BTreeMap<String, JsonValue>),
}

impl PluginMcpFile {
    fn into_mcp_servers(self) -> BTreeMap<String, JsonValue> {
        match self {
            Self::McpServersObject(file) => file.mcp_servers,
            Self::ServerMap(mcp_servers) => mcp_servers,
        }
    }
}

/// Parses wrapped or plain plugin MCP declarations without runtime normalization.
pub fn parse_plugin_mcp_declarations(
    contents: &str,
) -> Result<BTreeMap<String, JsonValue>, serde_json::Error> {
    serde_json::from_str::<PluginMcpFile>(contents).map(PluginMcpFile::into_mcp_servers)
}

/// Parses the two supported plugin MCP file shapes and normalizes each server.
///
/// Native plugin HTTP servers share the regular MCP transport configuration;
/// relative helper commands therefore use the session's local process cwd.
///
/// Invalid individual servers are returned as errors without discarding valid
/// siblings. A malformed top-level document fails the whole parse.
pub fn parse_plugin_mcp_config(
    plugin_root: &Path,
    contents: &str,
) -> Result<PluginMcpConfigParseOutcome, serde_json::Error> {
    parse_plugin_mcp_config_from(contents, PluginMcpSource::Host { root: plugin_root })
}

/// Parses executor-owned plugin MCP config without interpreting the plugin root
/// as a path on the orchestrator host.
pub fn parse_executor_plugin_mcp_config(
    plugin_root: &PathUri,
    contents: &str,
    environment_id: &str,
) -> Result<PluginMcpConfigParseOutcome, serde_json::Error> {
    parse_plugin_mcp_config_from(
        contents,
        PluginMcpSource::Environment {
            root: plugin_root,
            environment_id,
        },
    )
}

/// Normalizes already-parsed declarations for one executor installation.
pub fn normalize_executor_plugin_mcp_declarations(
    plugin_root: &PathUri,
    declarations: BTreeMap<String, JsonValue>,
    environment_id: &str,
) -> PluginMcpConfigParseOutcome {
    normalize_plugin_mcp_declarations(
        declarations,
        PluginMcpSource::Environment {
            root: plugin_root,
            environment_id,
        },
    )
}

impl PluginMcpSource<'_> {
    fn display(self) -> String {
        match self {
            Self::Host { root } => root.display().to_string(),
            Self::Environment { root, .. } => root.to_string(),
        }
    }
}

fn parse_plugin_mcp_config_from(
    contents: &str,
    source: PluginMcpSource<'_>,
) -> Result<PluginMcpConfigParseOutcome, serde_json::Error> {
    let declarations = parse_plugin_mcp_declarations(contents)?;
    Ok(normalize_plugin_mcp_declarations(declarations, source))
}

fn normalize_plugin_mcp_declarations(
    declarations: BTreeMap<String, JsonValue>,
    source: PluginMcpSource<'_>,
) -> PluginMcpConfigParseOutcome {
    let mut outcome = PluginMcpConfigParseOutcome::default();

    for (name, config_value) in declarations {
        match normalize_plugin_mcp_server(config_value, source) {
            Ok(config) => {
                outcome.servers.insert(name, config);
            }
            Err(message) => outcome
                .errors
                .push(PluginMcpServerParseError { name, message }),
        }
    }

    outcome
}

fn normalize_plugin_mcp_server(
    value: JsonValue,
    source: PluginMcpSource<'_>,
) -> Result<McpServerConfig, String> {
    let mut object = normalize_plugin_mcp_server_value(value, source);
    if let PluginMcpSource::Environment {
        root,
        environment_id,
    } = source
    {
        object.insert(
            "environment_id".to_string(),
            JsonValue::String(environment_id.to_string()),
        );
        if object.contains_key("command") {
            match object.remove("cwd") {
                Some(JsonValue::String(cwd)) => object.insert(
                    "cwd".to_string(),
                    JsonValue::String(environment_cwd(root, Some(&cwd))?.into_string()),
                ),
                Some(JsonValue::Null) | None => object.insert(
                    "cwd".to_string(),
                    JsonValue::String(
                        environment_cwd(root, /*configured_cwd*/ None)?.into_string(),
                    ),
                ),
                Some(value) => object.insert("cwd".to_string(), value),
            };
        }
    }

    let mut config = serde_json::from_value::<McpServerConfig>(JsonValue::Object(object))
        .map_err(|err| err.to_string())?;
    if matches!(config.auth, McpServerAuth::EmaAuth) {
        return Err(
            "plugin MCP declarations cannot select ema_auth; configure enterprise authentication in host policy"
                .to_string(),
        );
    }
    if matches!(source, PluginMcpSource::Environment { .. }) {
        bind_environment_env_vars(&mut config)?;
    }
    Ok(config)
}

fn environment_cwd(
    root: &PathUri,
    configured_cwd: Option<&str>,
) -> Result<LegacyAppPathString, String> {
    let Some(configured_cwd) = configured_cwd else {
        return Ok(root.clone().into());
    };
    let cwd = PathUri::parse(configured_cwd)
        .or_else(|_| root.join(configured_cwd))
        .map_err(|err| format!("invalid cwd `{configured_cwd}`: {err}"))?;
    if !cwd.starts_with(root) {
        return Err(format!(
            "cwd `{configured_cwd}` must remain within plugin root `{root}`"
        ));
    }
    Ok(cwd.into())
}

fn bind_environment_env_vars(config: &mut McpServerConfig) -> Result<(), String> {
    let is_local_environment = config.is_local_environment();
    let env_vars = match &mut config.transport {
        McpServerTransportConfig::Stdio { env_vars, .. } => env_vars,
        // Bearer credentials resolve on the executor; other header variables do not yet.
        McpServerTransportConfig::StreamableHttp {
            env_http_headers, ..
        } => {
            if is_local_environment {
                return Ok(());
            }
            if env_http_headers
                .as_ref()
                .is_some_and(|headers| !headers.is_empty())
            {
                return Err(
                    "`env_http_headers` requires executor-side environment resolution for an executor-owned HTTP MCP"
                        .to_string(),
                );
            }
            return Ok(());
        }
    };
    for env_var in env_vars {
        match env_var {
            McpServerEnvVar::Name(name) if !is_local_environment => {
                *env_var = McpServerEnvVar::Config {
                    name: std::mem::take(name),
                    source: Some("remote".to_string()),
                };
            }
            McpServerEnvVar::Name(_) => {}
            McpServerEnvVar::Config { name, source } => {
                match (is_local_environment, source.as_deref()) {
                    (true, None | Some("local")) | (false, Some("remote")) => {}
                    (true, Some("remote")) => {
                        return Err(format!(
                            "env_vars entry `{name}` cannot use source `remote` in a local environment"
                        ));
                    }
                    (false, None) => *source = Some("remote".to_string()),
                    (false, Some("local")) => {
                        return Err(format!(
                            "env_vars entry `{name}` cannot use source `local` in an executor-owned plugin"
                        ));
                    }
                    (_, Some(source)) => unreachable!("validated env_vars source `{source}`"),
                }
            }
        }
    }
    Ok(())
}

fn normalize_plugin_mcp_server_value(
    value: JsonValue,
    source: PluginMcpSource<'_>,
) -> JsonMap<String, JsonValue> {
    let mut value = value;
    // A Claude Code plugin's server may refer to the plugin's install path and to the
    // options the user set for it (`${CLAUDE_PLUGIN_ROOT}`, `${user_config.KEY}`).
    if let PluginMcpSource::Host { root } = source
        && let Some(plugin) = codex_claude_compat::PluginVariables::load(root)
    {
        substitute_plugin_variables(&mut value, &plugin, &source.display());
    }
    let mut object = match value {
        JsonValue::Object(object) => object,
        _ => return JsonMap::new(),
    };

    if let Some(JsonValue::String(transport_type)) = object.remove("type") {
        match transport_type.as_str() {
            "http" | "streamable_http" | "streamable-http" | "stdio" => {}
            other => {
                let plugin_display = source.display();
                warn!(
                    plugin = %plugin_display,
                    transport = other,
                    "plugin MCP server uses an unknown transport type"
                );
            }
        }
    }

    if let Some(JsonValue::Object(mut oauth)) = object.remove("oauth") {
        if let Some(callback_url) = oauth.remove("callbackUrl") {
            oauth
                .entry("callback_url".to_string())
                .or_insert(callback_url);
        }

        if let Some(callback_port) = oauth.remove("callbackPort") {
            oauth
                .entry("callback_port".to_string())
                .or_insert(callback_port);
        }

        if let Some(client_id) = oauth.remove("clientId") {
            oauth.entry("client_id".to_string()).or_insert(client_id);
        }

        if let Some(client_secret) = oauth.remove("clientSecret") {
            oauth
                .entry("client_secret".to_string())
                .or_insert(client_secret);
        }

        if !oauth.is_empty() {
            object.insert("oauth".to_string(), JsonValue::Object(oauth));
        }
    }

    if let PluginMcpSource::Host { root } = source
        && let Some(JsonValue::String(cwd)) = object.get("cwd")
        && !Path::new(cwd).is_absolute()
    {
        object.insert(
            "cwd".to_string(),
            JsonValue::String(root.join(cwd).display().to_string()),
        );
    }

    object
}

/// Fills in a Claude Code plugin's variables in every string of a server's declaration. A
/// string that names an option without a value is left as written.
fn substitute_plugin_variables(
    value: &mut JsonValue,
    plugin: &codex_claude_compat::PluginVariables,
    plugin_display: &str,
) {
    match value {
        JsonValue::String(text) => match plugin.substitute(text) {
            Ok(substituted) => *text = substituted,
            Err(option) => warn!(
                plugin = %plugin_display,
                option = %option,
                "plugin MCP server names an option that has no value; set it in Claude Code"
            ),
        },
        JsonValue::Array(items) => {
            for item in items {
                substitute_plugin_variables(item, plugin, plugin_display);
            }
        }
        JsonValue::Object(fields) => {
            for field in fields.values_mut() {
                substitute_plugin_variables(field, plugin, plugin_display);
            }
        }
        JsonValue::Null | JsonValue::Bool(_) | JsonValue::Number(_) => {}
    }
}

#[cfg(test)]
#[path = "plugin_config_tests.rs"]
mod tests;
