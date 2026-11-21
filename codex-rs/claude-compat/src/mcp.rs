// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's MCP servers: `mcpServers` of `~/.claude.json` (the user's, and the one
//! kept per project under `projects`) and of the project's `.mcp.json` files.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use serde_json::Map;
use serde_json::Value;

use crate::permissions::normalize_path;

/// The MCP servers Claude Code would start in `cwd`, as Claude Code declares them, with
/// `${VAR}` and `${VAR:-default}` expanded. Later sources win: the user's servers, then
/// the approved servers of the project's `.mcp.json` files (the one nearest `cwd` winning),
/// then the servers `~/.claude.json` keeps for the project. The servers turned off in
/// `/mcp` (`disabledMcpServers` of the project's entry in `~/.claude.json`) are left out.
///
/// In a remote server's `url` and `headers`, the credentials Claude Code keeps out of them
/// ([`CREDENTIAL_VARIABLES`]) read as empty, a `:-default` included, so that a project's
/// `.mcp.json` cannot send them to a server it names.
///
/// `claude_json` is `~/.claude.json`; `settings` are the parsed settings files, which
/// approve `.mcp.json` servers; `mcp_json_files` are the `.mcp.json` files from the top of
/// the filesystem down to `cwd`; `project_root` is the key of the project in `~/.claude.json`.
///
/// A `headersHelper` is a shell command, so, as in Claude Code, that of a server of
/// `.mcp.json` or of the project's entry in `~/.claude.json` is dropped unless
/// `project_trusted`: the server then connects with its static `headers` alone. The
/// command is left as written, for the shell that runs it to expand.
pub fn mcp_server_declarations(
    claude_json: Option<&Value>,
    settings: &[Value],
    mcp_json_files: &[Value],
    project_root: &Path,
    project_trusted: bool,
    env: &dyn Fn(&str) -> Option<String>,
) -> BTreeMap<String, Value> {
    let mut servers = BTreeMap::new();
    let mut project_servers = BTreeSet::new();
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
                project_servers.insert(name.clone());
                servers.insert(name, server);
            }
        }
    }
    if let Some(local) = project.and_then(servers_of) {
        project_servers.extend(local.keys().cloned());
        servers.extend(local);
    }
    if !project_trusted {
        for name in &project_servers {
            if let Some(Value::Object(server)) = servers.get_mut(name)
                && server.remove(HEADERS_HELPER).is_some()
            {
                tracing::warn!(
                    "headersHelper of MCP server {name} not run: the project is not trusted"
                );
            }
        }
    }
    if let Some(disabled) = project
        .and_then(|project| project.get("disabledMcpServers"))
        .and_then(Value::as_array)
    {
        for name in disabled.iter().filter_map(Value::as_str) {
            servers.remove(name);
        }
    }

    servers
        .into_iter()
        .map(|(name, server)| (name, expand_server(server, env)))
        .collect()
}

/// The environment variables that read as empty in a remote server's `url` and `headers`:
/// the examples Claude Code's documentation gives of the credentials it keeps out of them
/// (it does not list them all), and the credentials of this program's own providers.
pub const CREDENTIAL_VARIABLES: &[&str] = &[
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_AUTH_TOKEN",
    "CLAUDE_CODE_OAUTH_TOKEN",
    "AWS_BEARER_TOKEN_BEDROCK",
    "AWS_ACCESS_KEY_ID",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_SESSION_TOKEN",
    "HTTPS_PROXY",
    "HTTP_PROXY",
    "NPM_TOKEN",
    "OPENAI_API_KEY",
    "CODEX_API_KEY",
];

/// The key of a remote server's command that prints headers to send.
pub const HEADERS_HELPER: &str = "headersHelper";

/// The `sh` script that runs a server's `headersHelper` as Claude Code runs it: with
/// `CLAUDE_CODE_MCP_SERVER_NAME` and `CLAUDE_CODE_MCP_SERVER_URL` set and, for a plugin's
/// server, `CLAUDE_PLUGIN_ROOT` set and the plugin's root as its working directory.
pub fn headers_helper_script(
    command: &str,
    server_name: &str,
    url: &str,
    plugin_root: Option<&Path>,
) -> String {
    let mut script = format!(
        "export CLAUDE_CODE_MCP_SERVER_NAME={} CLAUDE_CODE_MCP_SERVER_URL={}\n",
        shell_quote(server_name),
        shell_quote(url)
    );
    if let Some(root) = plugin_root {
        let root = shell_quote(&root.to_string_lossy());
        script.push_str(&format!(
            "export CLAUDE_PLUGIN_ROOT={root}\ncd -- {root} || exit 1\n"
        ));
    }
    script.push_str(command);
    script
}

/// `text` in single quotes for `sh`.
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', r"'\''"))
}

/// A server's settings with `${VAR}` expanded as in `.mcp.json`, for a server declared
/// elsewhere (a subagent's `mcpServers`).
pub fn expand_mcp_server(server: Value, env: &dyn Fn(&str) -> Option<String>) -> Value {
    expand_server(server, env)
}

fn expand_server(server: Value, env: &dyn Fn(&str) -> Option<String>) -> Value {
    let Value::Object(fields) = server else {
        return expand_env(server, env);
    };
    let remote = fields.contains_key("url");
    Value::Object(
        fields
            .into_iter()
            .map(|(key, value)| {
                let value = if key == HEADERS_HELPER {
                    value
                } else if remote && (key == "url" || key == "headers") {
                    expand_env_with(value, env, /*credentials_empty*/ true)
                } else {
                    expand_env(value, env)
                };
                (key, value)
            })
            .collect(),
    )
}

/// The time limits Claude Code's environment sets for its MCP servers: `MCP_TIMEOUT`, how
/// long a server may take to start, and `MCP_TOOL_TIMEOUT`, how long a tool call may take,
/// both in milliseconds. A variable that is unset, not a number or zero gives `None`, as
/// Claude Code then uses its default; a server's own `timeout` wins over `tool`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct McpTimeouts {
    pub startup: Option<Duration>,
    pub tool: Option<Duration>,
}

/// Reads [`McpTimeouts`] through `env`, which should see the `env` of Claude Code's settings
/// files before the process environment, as Claude Code puts the former into its own.
pub fn mcp_timeouts(env: &dyn Fn(&str) -> Option<String>) -> McpTimeouts {
    let millis = |name: &str| {
        env(name)
            .and_then(|value| parse_int_millis(&value))
            .map(Duration::from_millis)
    };
    McpTimeouts {
        startup: millis("MCP_TIMEOUT"),
        tool: millis("MCP_TOOL_TIMEOUT"),
    }
}

/// The leading decimal digits of `value` after leading whitespace (JavaScript's `parseInt`,
/// which Claude Code uses), or `None` when there are none or they read as zero. A negative
/// value is taken as unset.
fn parse_int_millis(value: &str) -> Option<u64> {
    let value = value.trim_start();
    let value = value.strip_prefix('+').unwrap_or(value);
    let digits = value
        .find(|c: char| !c.is_ascii_digit())
        .map_or(value, |end| &value[..end]);
    digits.parse::<u64>().ok().filter(|millis| *millis > 0)
}

/// The key of the project `cwd` is in, in `projects` of `~/.claude.json`: the main checkout
/// of its Git repository, also from a linked worktree (Claude Code's `findCanonicalGitRoot`),
/// or `cwd` itself outside a repository.
pub fn claude_json_project_root(cwd: &Path) -> PathBuf {
    git_repository_root(cwd).unwrap_or_else(|| cwd.to_path_buf())
}

/// The root of the Git repository `cwd` is in, the main checkout's from a linked worktree
/// (Claude Code's `findCanonicalGitRoot`); `None` outside a repository.
pub(crate) fn git_repository_root(cwd: &Path) -> Option<PathBuf> {
    let git_root = cwd.ancestors().find(|dir| dir.join(".git").exists())?;
    Some(canonical_git_root(git_root).unwrap_or_else(|| git_root.to_path_buf()))
}

/// The main checkout of the linked worktree `git_root`, or of a bare repository its common
/// directory; `None` for any other working tree. As in Claude Code, a `.git` file is
/// followed only when it has the shape `git worktree add` gives it (its `gitdir` a direct
/// child of the common directory's `worktrees`, whose own `gitdir` points back to this
/// `.git`), so that a cloned repository cannot claim another project's entry.
fn canonical_git_root(git_root: &Path) -> Option<PathBuf> {
    let content = std::fs::read_to_string(git_root.join(".git")).ok()?;
    let gitdir = content.trim().strip_prefix("gitdir:")?.trim();
    let worktree_git_dir = normalize_path(&git_root.join(gitdir));
    let commondir = std::fs::read_to_string(worktree_git_dir.join("commondir")).ok()?;
    let common_dir = normalize_path(&worktree_git_dir.join(commondir.trim()));
    if worktree_git_dir.parent()? != common_dir.join("worktrees").as_path() {
        return None;
    }
    let backlink = std::fs::read_to_string(worktree_git_dir.join("gitdir")).ok()?;
    let backlink = std::fs::canonicalize(backlink.trim()).ok()?;
    if backlink != std::fs::canonicalize(git_root).ok()?.join(".git") {
        return None;
    }
    if common_dir.file_name()? != ".git" {
        return Some(common_dir);
    }
    common_dir.parent().map(Path::to_path_buf)
}

/// The `.mcp.json` files Claude Code reads for `cwd`, from the top of the filesystem down.
pub fn mcp_json_paths(cwd: &Path) -> Vec<PathBuf> {
    // As in Claude Code, the walk stops below the root of the filesystem.
    let mut paths = cwd
        .ancestors()
        .filter(|dir| dir.parent().is_some())
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
    expand_env_with(value, env, /*credentials_empty*/ false)
}

fn expand_env_with(
    value: Value,
    env: &dyn Fn(&str) -> Option<String>,
    credentials_empty: bool,
) -> Value {
    match value {
        Value::String(text) => Value::String(expand_text(&text, env, credentials_empty)),
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| expand_env_with(item, env, credentials_empty))
                .collect(),
        ),
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, item)| (key, expand_env_with(item, env, credentials_empty)))
                .collect(),
        ),
        other => other,
    }
}

/// Expands `${VAR}` and `${VAR:-default}`; a missing variable without a default stays as
/// written, as in Claude Code.
fn expand_text(
    text: &str,
    env: &dyn Fn(&str) -> Option<String>,
    credentials_empty: bool,
) -> String {
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
        let is_credential = credentials_empty
            && CREDENTIAL_VARIABLES
                .iter()
                .any(|credential| credential.eq_ignore_ascii_case(name));
        let value = if is_credential {
            Some(String::new())
        } else {
            env(name).or_else(|| default.map(str::to_string))
        };
        match value {
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
