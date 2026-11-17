//! Claude Code's MCP servers: `mcpServers` of `~/.claude.json` (the user's, and the one
//! kept per project under `projects`) and of the project's `.mcp.json` files.

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Map;
use serde_json::Value;

/// The MCP servers Claude Code would start in `cwd`, as Claude Code declares them, with
/// `${VAR}` and `${VAR:-default}` expanded. Later sources win: the user's servers, then
/// the approved servers of the project's `.mcp.json` files (the one nearest `cwd` winning),
/// then the servers `~/.claude.json` keeps for the project.
///
/// `claude_json` is `~/.claude.json`; `settings` are the parsed settings files, which
/// approve `.mcp.json` servers; `mcp_json_files` are the `.mcp.json` files from the top of
/// the filesystem down to `cwd`; `project_root` is the key of the project in `~/.claude.json`.
pub fn mcp_server_declarations(
    claude_json: Option<&Value>,
    settings: &[Value],
    mcp_json_files: &[Value],
    project_root: &Path,
    env: &dyn Fn(&str) -> Option<String>,
) -> BTreeMap<String, Value> {
    let mut servers = BTreeMap::new();
    let project = claude_json
        .and_then(|config| config.get("projects"))
        .and_then(|projects| projects.get(project_root.to_string_lossy().as_ref()));

    if let Some(user) = claude_json.and_then(servers_of) {
        servers.extend(user);
    }
    let approval_sources = settings.iter().chain(project).collect::<Vec<_>>();
    for file in mcp_json_files {
        for (name, server) in servers_of(file).unwrap_or_default() {
            if mcp_json_server_approved(&name, &approval_sources) {
                servers.insert(name, server);
            }
        }
    }
    if let Some(local) = project.and_then(servers_of) {
        servers.extend(local);
    }

    servers
        .into_iter()
        .map(|(name, server)| (name, expand_env(server, env)))
        .collect()
}

/// The `.mcp.json` files Claude Code reads for `cwd`, from the top of the filesystem down.
pub fn mcp_json_paths(cwd: &Path) -> Vec<PathBuf> {
    let mut paths = cwd
        .ancestors()
        .map(|dir| dir.join(".mcp.json"))
        .collect::<Vec<_>>();
    paths.reverse();
    paths
}

fn servers_of(value: &Value) -> Option<Map<String, Value>> {
    value.get("mcpServers").and_then(Value::as_object).cloned()
}

/// A `.mcp.json` server runs only once approved: listed in `enabledMcpjsonServers` or
/// covered by `enableAllProjectMcpServers`, and not listed in `disabledMcpjsonServers`.
fn mcp_json_server_approved(name: &str, sources: &[&Value]) -> bool {
    let lists = |key: &str| {
        sources.iter().any(|source| {
            source
                .get(key)
                .and_then(Value::as_array)
                .is_some_and(|names| names.iter().any(|listed| listed.as_str() == Some(name)))
        })
    };
    if lists("disabledMcpjsonServers") {
        return false;
    }
    lists("enabledMcpjsonServers")
        || sources.iter().any(|source| {
            source
                .get("enableAllProjectMcpServers")
                .and_then(Value::as_bool)
                .unwrap_or(false)
        })
}

fn expand_env(value: Value, env: &dyn Fn(&str) -> Option<String>) -> Value {
    match value {
        Value::String(text) => Value::String(expand_text(&text, env)),
        Value::Array(items) => Value::Array(items.into_iter().map(|item| expand_env(item, env)).collect()),
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, item)| (key, expand_env(item, env)))
                .collect(),
        ),
        other => other,
    }
}

/// Expands `${VAR}` and `${VAR:-default}`; a missing variable without a default stays as
/// written, as in Claude Code.
fn expand_text(text: &str, env: &dyn Fn(&str) -> Option<String>) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let inner = &after[..end];
        let (name, default) = match inner.split_once(":-") {
            Some((name, default)) => (name, Some(default)),
            None => (inner, None),
        };
        match env(name).or_else(|| default.map(str::to_string)) {
            Some(value) => out.push_str(&value),
            None => out.push_str(&rest[start..start + 2 + end + 1]),
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
#[path = "mcp_tests.rs"]
mod tests;
