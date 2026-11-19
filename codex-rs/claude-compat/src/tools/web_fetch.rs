// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `WebFetch` tool: description, URL rules, the fetch command, HTML to
//! Markdown, the cache and the prompt of the secondary model.
//!
//! Codex has no fetch tool. The fetch runs as a `curl` command through Codex's command
//! execution (see `core/src/tools/claude/web_fetch.rs`), so that Codex's sandbox, network
//! policy and approvals decide whether it may reach the host; this module builds that
//! command and reads its output, and holds everything else that does not depend on Codex.

use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use url::Url;

use super::BASH;

/// Longest URL Claude Code fetches.
pub const MAX_URL_LENGTH: usize = 2000;
/// Largest response body Claude Code accepts, in bytes.
pub const MAX_HTTP_CONTENT_LENGTH: u64 = 10 * 1024 * 1024;
/// Timeout of one request, in seconds (Claude Code's default).
pub const FETCH_TIMEOUT_SECS: u64 = 60;
/// Same-host redirects followed before giving up.
pub const MAX_REDIRECTS: usize = 10;
/// Longest Markdown handed to the secondary model, in characters.
pub const MAX_MARKDOWN_LENGTH: usize = 100_000;
/// How long a fetched page is kept.
pub const CACHE_TTL: Duration = Duration::from_secs(15 * 60);
/// Most Markdown the cache holds, in bytes.
pub const MAX_CACHE_SIZE_BYTES: usize = 50 * 1024 * 1024;

pub const URL_PARAM_DESCRIPTION: &str = "The URL to fetch content from";
pub const PROMPT_PARAM_DESCRIPTION: &str = "The prompt to run on the fetched content";

/// The user agent of the fetch.
pub const USER_AGENT: &str = "Codex-WebFetch (Claude Code compatible)";

/// The tool description, after Claude Code's. `available` lists the Claude Code tools the
/// model is offered; the GitHub hint points to `Bash` only when it is there.
pub fn description(available: &[&str]) -> String {
    let github = if available.contains(&BASH) {
        format!(
            "\n  - For GitHub URLs, prefer using the gh CLI via {BASH} instead (e.g., gh pr view, gh issue view, gh api)."
        )
    } else {
        String::new()
    };
    format!(
        r#"IMPORTANT: WebFetch WILL FAIL for authenticated or private URLs. Before using this tool, check if the URL points to an authenticated service (e.g. Google Docs, Confluence, Jira, GitHub). If so, look for a specialized MCP tool that provides authenticated access.

- Fetches content from a specified URL and processes it using an AI model
- Takes a URL and a prompt as input
- Fetches the URL content, converts HTML to markdown
- Processes the content with the prompt using a small, fast model
- Returns the model's response about the content
- Use this tool when you need to retrieve and analyze web content

Usage notes:
  - IMPORTANT: If an MCP-provided web fetch tool is available, prefer using that tool instead of this one, as it may have fewer restrictions.
  - The URL must be a fully-formed valid URL
  - HTTP URLs will be automatically upgraded to HTTPS
  - The prompt should describe what information you want to extract from the page
  - This tool is read-only and does not modify any files
  - Results may be summarized if the content is very large
  - Includes a self-cleaning 15-minute cache for faster responses when repeatedly accessing the same URL
  - When a URL redirects to a different host, the tool will inform you and provide the redirect URL in a special format. You should then make a new WebFetch request with the redirect URL to fetch the content.
  - The request runs under the session's sandbox and network rules, and may ask the user for approval.{github}
"#
    )
}

/// Claude Code's message for a URL that cannot be parsed.
pub fn invalid_url_message(url: &str) -> String {
    format!("Error: Invalid URL \"{url}\". The URL provided could not be parsed.")
}

/// Parses a URL the way Claude Code checks it before fetching: at most
/// [`MAX_URL_LENGTH`] long, no user name or password, and a host name with at least two
/// labels (no `localhost` or other single-label names). `None` is Claude Code's
/// `Invalid URL` error.
pub fn validate_url(url: &str) -> Option<Url> {
    if url.len() > MAX_URL_LENGTH {
        return None;
    }
    let parsed = Url::parse(url).ok()?;
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return None;
    }
    let host = parsed.host_str()?;
    if host.split('.').count() < 2 {
        return None;
    }
    Some(parsed)
}

/// The URL actually requested: `http` becomes `https`.
pub fn upgrade_to_https(mut url: Url) -> Url {
    if url.scheme() == "http" {
        // Both are special schemes, so the change cannot fail.
        let _ = url.set_scheme("https");
    }
    url
}

/// Whether a redirect stays on the same site, so Claude Code follows it: the same scheme,
/// port and host, a `www.` added or removed, and no credentials.
pub fn is_permitted_redirect(original: &Url, redirect: &Url) -> bool {
    if original.scheme() != redirect.scheme() || original.port() != redirect.port() {
        return false;
    }
    if !redirect.username().is_empty() || redirect.password().is_some() {
        return false;
    }
    let strip_www = |url: &Url| {
        let host = url.host_str().unwrap_or_default();
        host.strip_prefix("www.").unwrap_or(host).to_string()
    };
    strip_www(original) == strip_www(redirect)
}

/// The status text Claude Code reports for a redirect it does not follow.
pub fn redirect_status_text(status: u16) -> &'static str {
    match status {
        301 => "Moved Permanently",
        308 => "Permanent Redirect",
        307 => "Temporary Redirect",
        _ => "Found",
    }
}

/// Claude Code's result for a redirect to another host.
pub fn redirect_message(original: &str, redirect: &str, status: u16, prompt: &str) -> String {
    format!(
        "REDIRECT DETECTED: The URL redirects to a different host.\n\nOriginal URL: {original}\nRedirect URL: {redirect}\nStatus: {status} {}\n\nTo complete your request, I need to fetch content from the redirected URL. Please use WebFetch again with these parameters:\n- url: \"{redirect}\"\n- prompt: \"{prompt}\"",
        redirect_status_text(status)
    )
}

/// Whether a status is a redirect Claude Code handles (others are errors).
pub fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 307 | 308)
}

/// Claude Code's error for a response that is neither a success nor a handled redirect.
pub fn status_error_message(status: u16) -> String {
    format!("Request failed with status code {status}")
}

/// The reason phrase of a status, as Node reports it for HTTP/1.1 responses.
pub fn status_text(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        203 => "Non-Authoritative Information",
        204 => "No Content",
        205 => "Reset Content",
        206 => "Partial Content",
        _ => "",
    }
}

/// How the executor's shell quotes words, for the fetch command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandShell {
    /// `sh`, `bash`, `zsh`.
    Posix,
    /// PowerShell, where `curl` may be an alias of `Invoke-WebRequest`.
    PowerShell,
    /// `cmd.exe`.
    Cmd,
}

/// Marks the end of the body in the fetch command's output; `curl --write-out` adds it
/// after the body, followed by the status, the body size, the redirect target and the
/// content type, separated by tabs.
pub const OUTPUT_MARKER: &str = "\n__CODEX_WEBFETCH__\t";
const WRITE_OUT: &str =
    "\\n__CODEX_WEBFETCH__\\t%{http_code}\\t%{size_download}\\t%{redirect_url}\\t%{content_type}";

/// The command that fetches `url` (already upgraded to HTTPS) without following
/// redirects: `curl` with Claude Code's limits (timeout, body size) and `Accept` header.
/// Proxy settings come from the environment Codex gives the command.
pub fn fetch_command(url: &Url, shell: CommandShell) -> String {
    let program = match shell {
        CommandShell::PowerShell | CommandShell::Cmd => "curl.exe",
        CommandShell::Posix => "curl",
    };
    let timeout = FETCH_TIMEOUT_SECS.to_string();
    let max_size = MAX_HTTP_CONTENT_LENGTH.to_string();
    let args = [
        "--silent",
        "--show-error",
        "--globoff",
        "--proto",
        "=https",
        "--max-time",
        timeout.as_str(),
        "--max-filesize",
        max_size.as_str(),
        "--header",
        "Accept: text/markdown, text/html, */*",
        "--user-agent",
        USER_AGENT,
        "--write-out",
        WRITE_OUT,
        url.as_str(),
    ];
    let mut command = program.to_string();
    for arg in args {
        command.push(' ');
        command.push_str(&quote(arg, shell));
    }
    command
}

fn quote(word: &str, shell: CommandShell) -> String {
    // A leading `=` stays quoted: zsh expands `=word` to the path of the command `word`.
    let plain = !word.is_empty()
        && !word.starts_with('=')
        && word
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '=' | '.' | '/'));
    if plain {
        return word.to_string();
    }
    match shell {
        CommandShell::Posix => format!("'{}'", word.replace('\'', "'\\''")),
        CommandShell::PowerShell => format!("'{}'", word.replace('\'', "''")),
        CommandShell::Cmd => format!("\"{}\"", word.replace('"', "\"\"")),
    }
}

/// What the fetch command printed: the body and what `--write-out` reported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FetchOutput<'a> {
    pub status: u16,
    pub bytes: u64,
    /// Where a redirect points, resolved against the request URL; empty without one.
    pub redirect_url: &'a str,
    pub content_type: &'a str,
    pub body: &'a str,
}

/// Splits the fetch command's output into the body and the reported fields; `None` when
/// the output does not end with them (curl failed before writing them).
pub fn parse_fetch_output(output: &str) -> Option<FetchOutput<'_>> {
    let start = output.rfind(OUTPUT_MARKER)?;
    let body = &output[..start];
    let mut fields = output[start + OUTPUT_MARKER.len()..].splitn(4, '\t');
    let status = fields.next()?.trim().parse().ok()?;
    let bytes = fields.next()?.trim().parse().unwrap_or(body.len() as u64);
    let redirect_url = fields.next()?.trim();
    let content_type = fields.next().unwrap_or_default().trim();
    Some(FetchOutput {
        status,
        bytes,
        redirect_url,
        content_type,
        body,
    })
}

/// Claude Code's message when curl itself failed, from its exit code and output.
pub fn command_error_message(exit_code: Option<i32>, output: &str) -> String {
    let output = match output.find(OUTPUT_MARKER) {
        Some(end) => &output[..end],
        None => output,
    };
    let detail = output.trim();
    match exit_code {
        Some(63) => format!("Response exceeds the maximum size of {MAX_HTTP_CONTENT_LENGTH} bytes"),
        Some(28) => format!("timeout of {}ms exceeded", FETCH_TIMEOUT_SECS * 1000),
        Some(code) if detail.is_empty() => format!("Fetch failed: curl exited with code {code}"),
        Some(_) => detail.to_string(),
        None => "Fetch did not finish".to_string(),
    }
}

/// The page as Claude Code works on it: HTML is converted to Markdown, anything else is
/// kept as it came.
pub fn to_markdown(body: &str, content_type: &str) -> String {
    if !content_type.contains("text/html") {
        return body.to_string();
    }
    let converter = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();
    converter.convert(body).unwrap_or_else(|_| body.to_string())
}

/// A fetched page, as cached.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FetchedContent {
    pub bytes: u64,
    pub code: u16,
    pub code_text: String,
    pub content: String,
    pub content_type: String,
}

struct CacheEntry {
    stored_at: Instant,
    used_at: Instant,
    content: FetchedContent,
}

/// Claude Code's URL cache: entries live [`CACHE_TTL`], the least recently used go first
/// once the content exceeds [`MAX_CACHE_SIZE_BYTES`]. Keyed by the URL as the model gave
/// it, not the upgraded or redirected one.
#[derive(Default)]
pub struct FetchCache {
    entries: HashMap<String, CacheEntry>,
    size: usize,
}

impl FetchCache {
    pub fn get(&mut self, url: &str, now: Instant) -> Option<FetchedContent> {
        let expired = self
            .entries
            .get(url)
            .is_some_and(|entry| now.duration_since(entry.stored_at) >= CACHE_TTL);
        if expired {
            self.remove(url);
            return None;
        }
        let entry = self.entries.get_mut(url)?;
        entry.used_at = now;
        Some(entry.content.clone())
    }

    pub fn insert(&mut self, url: String, content: FetchedContent, now: Instant) {
        self.remove(&url);
        let size = content.content.len().max(1);
        if size > MAX_CACHE_SIZE_BYTES {
            return;
        }
        self.entries
            .retain(|_, entry| now.duration_since(entry.stored_at) < CACHE_TTL);
        self.size = self
            .entries
            .values()
            .map(|entry| entry.content.content.len().max(1))
            .sum();
        while self.size + size > MAX_CACHE_SIZE_BYTES {
            let Some(oldest) = self
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.used_at)
                .map(|(key, _)| key.clone())
            else {
                break;
            };
            self.remove(&oldest);
        }
        self.size += size;
        self.entries.insert(
            url,
            CacheEntry {
                stored_at: now,
                used_at: now,
                content,
            },
        );
    }

    fn remove(&mut self, url: &str) {
        if let Some(entry) = self.entries.remove(url) {
            self.size = self.size.saturating_sub(entry.content.content.len().max(1));
        }
    }
}

/// Hosts (and host/path prefixes) Claude Code trusts for code documentation. Codex's own
/// rules decide whether they may be fetched; here they only change how the secondary model
/// may quote them, and let their Markdown through unprocessed.
pub const PREAPPROVED_HOSTS: &[&str] = &[
    "platform.claude.com",
    "code.claude.com",
    "modelcontextprotocol.io",
    "github.com/anthropics",
    "agentskills.io",
    "docs.python.org",
    "en.cppreference.com",
    "docs.oracle.com",
    "learn.microsoft.com",
    "developer.mozilla.org",
    "go.dev",
    "pkg.go.dev",
    "www.php.net",
    "docs.swift.org",
    "kotlinlang.org",
    "ruby-doc.org",
    "doc.rust-lang.org",
    "www.typescriptlang.org",
    "react.dev",
    "angular.io",
    "vuejs.org",
    "nextjs.org",
    "expressjs.com",
    "nodejs.org",
    "bun.sh",
    "jquery.com",
    "getbootstrap.com",
    "tailwindcss.com",
    "d3js.org",
    "threejs.org",
    "redux.js.org",
    "webpack.js.org",
    "jestjs.io",
    "reactrouter.com",
    "docs.djangoproject.com",
    "flask.palletsprojects.com",
    "fastapi.tiangolo.com",
    "pandas.pydata.org",
    "numpy.org",
    "www.tensorflow.org",
    "pytorch.org",
    "scikit-learn.org",
    "matplotlib.org",
    "requests.readthedocs.io",
    "jupyter.org",
    "laravel.com",
    "symfony.com",
    "wordpress.org",
    "docs.spring.io",
    "hibernate.org",
    "tomcat.apache.org",
    "gradle.org",
    "maven.apache.org",
    "asp.net",
    "dotnet.microsoft.com",
    "nuget.org",
    "blazor.net",
    "reactnative.dev",
    "docs.flutter.dev",
    "developer.apple.com",
    "developer.android.com",
    "keras.io",
    "spark.apache.org",
    "huggingface.co",
    "www.kaggle.com",
    "www.mongodb.com",
    "redis.io",
    "www.postgresql.org",
    "dev.mysql.com",
    "www.sqlite.org",
    "graphql.org",
    "prisma.io",
    "docs.aws.amazon.com",
    "cloud.google.com",
    "kubernetes.io",
    "www.docker.com",
    "www.terraform.io",
    "www.ansible.com",
    "vercel.com/docs",
    "docs.netlify.com",
    "devcenter.heroku.com",
    "cypress.io",
    "selenium.dev",
    "docs.unity.com",
    "docs.unrealengine.com",
    "git-scm.com",
    "nginx.org",
    "httpd.apache.org",
];

/// Whether `url` is on one of [`PREAPPROVED_HOSTS`]; a path entry matches whole path
/// segments only (`/anthropics`, not `/anthropics-evil`).
pub fn is_preapproved(url: &Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    let path = url.path();
    PREAPPROVED_HOSTS
        .iter()
        .any(|entry| match entry.split_once('/') {
            None => *entry == host,
            Some((entry_host, entry_path)) => {
                let prefix = format!("/{entry_path}");
                entry_host == host && (path == prefix || path.starts_with(&format!("{prefix}/")))
            }
        })
}

/// Whether Claude Code returns the page as it is, without the secondary model: Markdown
/// from a preapproved host, short enough.
pub fn returns_content_unprocessed(preapproved: bool, content_type: &str, content: &str) -> bool {
    preapproved
        && content_type.contains("text/markdown")
        && content.chars().count() < MAX_MARKDOWN_LENGTH
}

/// The content handed to the secondary model: at most [`MAX_MARKDOWN_LENGTH`] characters.
pub fn truncate_markdown(content: &str) -> String {
    match content.char_indices().nth(MAX_MARKDOWN_LENGTH) {
        Some((end, _)) => format!(
            "{}\n\n[Content truncated due to length...]",
            &content[..end]
        ),
        None => content.to_string(),
    }
}

/// Claude Code's prompt for the secondary model.
pub fn secondary_model_prompt(markdown: &str, prompt: &str, preapproved: bool) -> String {
    let guidelines = if preapproved {
        "Provide a concise response based on the content above. Include relevant details, code examples, and documentation excerpts as needed."
    } else {
        "Provide a concise response based only on the content above. In your response:\n - Enforce a strict 125-character maximum for quotes from any source document. Open Source Software is ok as long as we respect the license.\n - Use quotation marks for exact language from articles; any language outside of the quotation should never be word-for-word the same.\n - You are not a lawyer and never comment on the legality of your own prompts and responses.\n - Never produce or reproduce exact song lyrics."
    };
    format!(
        "\nWeb page content:\n---\n{}\n---\n\n{prompt}\n\n{guidelines}\n",
        truncate_markdown(markdown)
    )
}

/// Claude Code's result when the secondary model answered nothing.
pub const NO_RESPONSE: &str = "No response from model";

#[cfg(test)]
#[path = "web_fetch_tests.rs"]
mod tests;
