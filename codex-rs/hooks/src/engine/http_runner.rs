// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `http` hook handlers: the hook input is sent as the body of a POST
//! request, and the JSON the server answers with is read as a command hook's stdout is.
//!
//! As in Claude Code (`execHttpHook.ts`): header values may name environment variables
//! (`$NAME`, `${NAME}`), which are filled in only when the handler lists them in
//! `allowedEnvVars`; the settings keys `allowedHttpHookUrls` and `httpHookAllowedEnvVars`
//! narrow what any handler may reach; redirects are not followed; and without a proxy, a
//! host that resolves to a private or link-local address is refused, loopback excepted.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::sync::LazyLock;
use std::time::Duration;
use std::time::Instant;

use codex_protocol::models::ContentItemMetadata;
use regex::Regex;

use super::ConfiguredHandler;
use super::HandlerRunResult;

/// What the settings files allow `http` hooks to do. `None` leaves a thing unrestricted;
/// an empty list allows nothing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct HttpHookPolicy {
    /// `allowedHttpHookUrls`: patterns, with `*` for any text, one of which a URL has to match.
    pub(crate) allowed_urls: Option<Vec<String>>,
    /// `httpHookAllowedEnvVars`: the variables a handler's own `allowedEnvVars` may name.
    pub(crate) allowed_env_vars: Option<Vec<String>>,
}

/// One `http` handler as its hooks file declares it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HttpHook {
    pub(crate) url: String,
    pub(crate) headers: BTreeMap<String, String>,
    pub(crate) allowed_env_vars: Vec<String>,
    pub(crate) policy: HttpHookPolicy,
}

static ENV_VAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\$\{([A-Z_][A-Z0-9_]*)\}|\$([A-Z_][A-Z0-9_]*)")
        .unwrap_or_else(|err| panic!("valid environment variable pattern: {err}"))
});

/// Posts the hook input to the handler's URL. A response outside 2xx, a refused or failed
/// request and a timeout are errors that do not block, as in Claude Code.
pub(crate) async fn run_http(
    handler: &ConfiguredHandler,
    hook: &HttpHook,
    input_json: &str,
) -> HandlerRunResult {
    let started_at = chrono::Utc::now().timestamp();
    let started = Instant::now();
    let timeout = Duration::from_secs(handler.timeout_sec);
    let outcome = match tokio::time::timeout(timeout, post(hook, input_json)).await {
        Ok(outcome) => outcome,
        Err(_) => Err(format!(
            "HTTP hook timed out after {}s: {}",
            handler.timeout_sec, hook.url
        )),
    };
    let (exit_code, stdout, stderr) = match outcome {
        Ok(body) => match validate_body(&body) {
            Ok(()) => (0, body, String::new()),
            Err(error) => (1, body, format!("JSON validation failed: {error}")),
        },
        Err(error) => (1, String::new(), error),
    };
    HandlerRunResult {
        structured_content: None,
        context_metadata: ContentItemMetadata::command_hook(),
        started_at,
        completed_at: chrono::Utc::now().timestamp(),
        duration_ms: started.elapsed().as_millis().try_into().unwrap_or(i64::MAX),
        exit_code: Some(exit_code),
        stdout,
        stderr,
        error: None,
    }
}

async fn post(hook: &HttpHook, input_json: &str) -> Result<String, String> {
    if let Some(allowed) = &hook.policy.allowed_urls
        && !allowed
            .iter()
            .any(|pattern| url_matches_pattern(&hook.url, pattern))
    {
        return Err(format!(
            "HTTP hook blocked: {} does not match any pattern in allowedHttpHookUrls",
            hook.url
        ));
    }
    let url = reqwest::Url::parse(&hook.url)
        .map_err(|err| format!("invalid HTTP hook URL {}: {err}", hook.url))?;

    let mut builder = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());
    if !proxy_in_use() {
        // The address checked is the one connected to, so that a name cannot resolve to an
        // allowed address here and to a private one for the request.
        let (host, address) = guarded_address(&url).await?;
        builder = builder.resolve(&host, address);
    }
    let client = builder
        .build()
        .map_err(|err| format!("failed to set up the HTTP hook request: {err}"))?;

    let allowed_env_vars = hook
        .allowed_env_vars
        .iter()
        .filter(|name| {
            hook.policy
                .allowed_env_vars
                .as_ref()
                .is_none_or(|allowed| allowed.contains(name))
        })
        .map(String::as_str)
        .collect::<Vec<_>>();
    let mut request = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(input_json.to_string());
    for (name, value) in &hook.headers {
        request = request.header(
            name.as_str(),
            interpolate_env_vars(value, &allowed_env_vars, |name| std::env::var(name).ok()),
        );
    }
    let response = request
        .send()
        .await
        .map_err(|err| format!("HTTP hook request to {} failed: {err}", hook.url))?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP {} from {}", status.as_u16(), hook.url));
    }
    response.text().await.map_err(|err| {
        format!(
            "failed to read the HTTP hook response from {}: {err}",
            hook.url
        )
    })
}

/// Whether a proxy named in the environment carries the request, in which case the proxy
/// resolves the host and the address guard would only check the proxy's own address.
fn proxy_in_use() -> bool {
    [
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
        "ALL_PROXY",
        "all_proxy",
    ]
    .iter()
    .any(|name| std::env::var(name).is_ok_and(|value| !value.trim().is_empty()))
}

/// The host of `url` and one address of it that an `http` hook may reach.
async fn guarded_address(url: &reqwest::Url) -> Result<(String, SocketAddr), String> {
    let host = url
        .host_str()
        .ok_or_else(|| format!("HTTP hook URL {url} has no host"))?
        .to_string();
    let port = url
        .port_or_known_default()
        .ok_or_else(|| format!("HTTP hook URL {url} has no port"))?;
    let lookup_host = host
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_string();
    let addresses = tokio::net::lookup_host((lookup_host.as_str(), port))
        .await
        .map_err(|err| format!("failed to resolve HTTP hook host {host}: {err}"))?
        .collect::<Vec<_>>();
    if let Some(blocked) = addresses
        .iter()
        .find(|address| is_blocked_address(address.ip()))
    {
        return Err(format!(
            "HTTP hook blocked: {host} resolves to {}, a private or link-local address",
            blocked.ip()
        ));
    }
    let address = addresses
        .first()
        .copied()
        .ok_or_else(|| format!("HTTP hook host {host} has no address"))?;
    Ok((host, address))
}

/// Private, link-local, shared and unspecified addresses, which a hook of a project could
/// otherwise use to reach services inside the network. Loopback stays allowed, for hooks
/// served from this machine.
fn is_blocked_address(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(v4) => {
            if v4.is_loopback() {
                return false;
            }
            let [a, b, ..] = v4.octets();
            v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
                // Shared address space, 100.64.0.0/10.
                || (a == 100 && (64..128).contains(&b))
        }
        IpAddr::V6(v6) => {
            if v6.is_loopback() {
                return false;
            }
            if let Some(v4) = v6.to_ipv4_mapped() {
                return is_blocked_address(IpAddr::V4(v4));
            }
            let first = v6.segments()[0];
            v6.is_unspecified()
                // Unique local, fc00::/7.
                || (first & 0xfe00) == 0xfc00
                // Link-local, fe80::/10.
                || (first & 0xffc0) == 0xfe80
        }
    }
}

/// Whether `url` matches `pattern`, in which `*` stands for any text.
fn url_matches_pattern(url: &str, pattern: &str) -> bool {
    let mut parts = pattern.split('*');
    let Some(first) = parts.next() else {
        return false;
    };
    let Some(mut rest) = url.strip_prefix(first) else {
        return false;
    };
    let parts = parts.collect::<Vec<_>>();
    let Some((last, middle)) = parts.split_last() else {
        // No wildcard: the whole URL has to be the pattern.
        return rest.is_empty();
    };
    for part in middle {
        let Some(found) = rest.find(part) else {
            return false;
        };
        rest = &rest[found + part.len()..];
    }
    rest.ends_with(last)
}

/// `value` with `$NAME` and `${NAME}` filled in from `lookup` for the names in `allowed`;
/// any other reference becomes empty, so that a hooks file cannot send out a variable the
/// user did not release. Line breaks and NUL are dropped: a value may not add a header.
fn interpolate_env_vars(
    value: &str,
    allowed: &[&str],
    lookup: impl Fn(&str) -> Option<String>,
) -> String {
    let interpolated = ENV_VAR.replace_all(value, |captures: &regex::Captures<'_>| {
        let name = captures
            .get(1)
            .or_else(|| captures.get(2))
            .map_or("", |found| found.as_str());
        if allowed.contains(&name) {
            lookup(name).unwrap_or_default()
        } else {
            String::new()
        }
    });
    interpolated
        .chars()
        .filter(|ch| !matches!(ch, '\r' | '\n' | '\0'))
        .collect()
}

/// An `http` hook answers with a JSON object, or with nothing.
fn validate_body(body: &str) -> Result<(), String> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    if !trimmed.starts_with('{') {
        let shown = trimmed.chars().take(200).collect::<String>();
        return Err(format!(
            "HTTP hook must return JSON, but got non-JSON response body: {shown}"
        ));
    }
    serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(trimmed)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
#[path = "http_runner_tests.rs"]
mod tests;
