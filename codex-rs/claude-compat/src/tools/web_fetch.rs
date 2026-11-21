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

/// Deepest nesting of elements converted to Markdown. The converter recurses once per level
/// and the HTML parser slows down with depth, so a deeper page is reduced to its text.
pub const MAX_HTML_DEPTH: usize = 1_000;

/// The page as Claude Code works on it: HTML is converted to Markdown, anything else is
/// kept as it came. HTML nested deeper than [`MAX_HTML_DEPTH`] becomes its plain text.
pub fn to_markdown(body: &str, content_type: &str) -> String {
    if !content_type.contains("text/html") {
        return body.to_string();
    }
    if html_depth_exceeds(body, MAX_HTML_DEPTH) {
        return html_text(body);
    }
    let converter = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style"])
        .build();
    converter.convert(body).unwrap_or_else(|_| body.to_string())
}

/// Elements that have no content, and never nest.
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "frame", "hr", "img", "input", "keygen", "link", "meta",
    "param", "source", "track", "wbr",
];
/// Elements the parser closes when the next one of their kind starts, so that a run of them
/// without end tags does not nest; they are left out of the depth.
const SELF_ENDING_ELEMENTS: &[&str] = &[
    "p", "li", "dt", "dd", "tr", "td", "th", "thead", "tbody", "tfoot", "option", "optgroup", "rb",
    "rt", "rtc", "rp", "caption", "colgroup", "html", "head", "body",
];
/// Elements whose content is text up to their end tag, not markup.
const RAW_TEXT_ELEMENTS: &[&str] = &[
    "script", "style", "textarea", "title", "xmp", "iframe", "noembed", "noframes",
];
/// Elements that end a line in the plain text of a page.
const BLOCK_ELEMENTS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "br",
    "dd",
    "div",
    "dl",
    "dt",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hr",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "table",
    "tr",
    "ul",
];

enum HtmlToken<'a> {
    Text(&'a str),
    /// A start tag, its name in lower case, and whether it ends with `/>`.
    Start(String, bool),
    End(String),
}

/// Walks the tags and text of `html` without building a tree: comments, doctypes and the
/// content of `<script>` and `<style>` are skipped; the content of the other raw-text
/// elements is text. `visit` returns `false` to stop.
fn scan_html(html: &str, mut visit: impl FnMut(HtmlToken<'_>) -> bool) {
    let bytes = html.as_bytes();
    let len = html.len();
    let mut pos = 0;
    let mut text_start = 0;
    while let Some(offset) = html[pos..].find('<') {
        let start = pos + offset;
        let rest = &html[start + 1..];
        let closing = rest.starts_with('/');
        let name_start = start + 1 + usize::from(closing);
        let name_len = bytes[name_start.min(len)..]
            .iter()
            .take_while(|byte| byte.is_ascii_alphanumeric() || matches!(**byte, b'-' | b':'))
            .count();
        let starts_with_letter = bytes.get(name_start).is_some_and(u8::is_ascii_alphabetic);
        if name_len == 0 || !starts_with_letter {
            let skip_to = if let Some(comment) = rest.strip_prefix("!--") {
                comment.find("-->").map(|end| start + 4 + end + 3)
            } else if rest.starts_with('!') || rest.starts_with('?') || closing {
                rest.find('>').map(|end| start + 1 + end + 1)
            } else {
                // A `<` that starts no tag is text.
                pos = start + 1;
                continue;
            };
            if !visit(HtmlToken::Text(&html[text_start..start])) {
                return;
            }
            pos = skip_to.unwrap_or(len);
            text_start = pos;
            continue;
        }
        let name = html[name_start..name_start + name_len].to_ascii_lowercase();
        // The end of the tag: the first `>` outside a quoted attribute value.
        let mut end = name_start + name_len;
        let mut quote = None;
        let mut after_equals = false;
        while end < len {
            let byte = bytes[end];
            match quote {
                Some(open) if byte == open => quote = None,
                Some(_) => {}
                None if byte == b'>' => break,
                None if after_equals && matches!(byte, b'"' | b'\'') => quote = Some(byte),
                None => {}
            }
            if !byte.is_ascii_whitespace() {
                after_equals = quote.is_none() && byte == b'=';
            }
            end += 1;
        }
        let self_closing = end < len && end > name_start && bytes[end - 1] == b'/';
        if !visit(HtmlToken::Text(&html[text_start..start])) {
            return;
        }
        let token = if closing {
            HtmlToken::End(name.clone())
        } else {
            HtmlToken::Start(name.clone(), self_closing)
        };
        if !visit(token) {
            return;
        }
        pos = (end + 1).min(len);
        text_start = pos;
        if !closing && RAW_TEXT_ELEMENTS.contains(&name.as_str()) {
            let close = find_end_tag(html, pos, &name).unwrap_or(len);
            if !matches!(name.as_str(), "script" | "style")
                && !visit(HtmlToken::Text(&html[pos..close]))
            {
                return;
            }
            pos = close;
            text_start = close;
        }
    }
    visit(HtmlToken::Text(&html[text_start..]));
}

/// Where the end tag of the raw-text element `name` starts, from `from` on.
fn find_end_tag(html: &str, from: usize, name: &str) -> Option<usize> {
    let mut pos = from;
    while let Some(offset) = html[pos..].find("</") {
        let start = pos + offset;
        let candidate = html.get(start + 2..start + 2 + name.len());
        if candidate.is_some_and(|candidate| candidate.eq_ignore_ascii_case(name)) {
            return Some(start);
        }
        pos = start + 2;
    }
    None
}

/// Whether the elements of `html` nest deeper than `limit`, as the parser would nest them:
/// an end tag closes the innermost open element of its name and those inside it, and is
/// ignored when none is open; `/>` closes an element only inside `<svg>` or `<math>`.
fn html_depth_exceeds(html: &str, limit: usize) -> bool {
    let mut stack: Vec<String> = Vec::new();
    let mut open: HashMap<String, usize> = HashMap::new();
    let mut foreign = 0usize;
    let mut exceeded = false;
    scan_html(html, |token| {
        match token {
            HtmlToken::Text(_) => {}
            HtmlToken::Start(name, self_closing) => {
                let name_str = name.as_str();
                if VOID_ELEMENTS.contains(&name_str)
                    || SELF_ENDING_ELEMENTS.contains(&name_str)
                    || (self_closing && (foreign > 0 || matches!(name_str, "svg" | "math")))
                {
                    return true;
                }
                if matches!(name_str, "svg" | "math") {
                    foreign += 1;
                }
                *open.entry(name.clone()).or_default() += 1;
                stack.push(name);
                if stack.len() > limit {
                    exceeded = true;
                    return false;
                }
            }
            HtmlToken::End(name) => {
                if open.get(&name).copied().unwrap_or(0) == 0 {
                    return true;
                }
                while let Some(popped) = stack.pop() {
                    if let Some(count) = open.get_mut(&popped) {
                        *count = count.saturating_sub(1);
                    }
                    if matches!(popped.as_str(), "svg" | "math") {
                        foreign = foreign.saturating_sub(1);
                    }
                    if popped == name {
                        break;
                    }
                }
            }
        }
        true
    });
    exceeded
}

/// The text of `html`, for a page too deeply nested to convert: one line for each block,
/// runs of spaces made one, and the common character references decoded.
fn html_text(html: &str) -> String {
    let mut raw = String::new();
    scan_html(html, |token| {
        match token {
            HtmlToken::Text(text) => raw.push_str(text),
            HtmlToken::Start(name, _) | HtmlToken::End(name) => {
                if BLOCK_ELEMENTS.contains(&name.as_str()) {
                    raw.push('\n');
                } else if matches!(name.as_str(), "td" | "th") {
                    raw.push(' ');
                }
            }
        }
        true
    });
    let mut text = String::new();
    for line in raw.split('\n') {
        let line =
            decode_character_references(&line.split_whitespace().collect::<Vec<_>>().join(" "));
        let line = line.trim();
        if !line.is_empty() {
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str(line);
        }
    }
    text
}

/// Decodes the named references HTML text uses most and numeric ones; others stay as
/// they are.
fn decode_character_references(text: &str) -> String {
    let mut decoded = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('&') {
        decoded.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        let reference = after
            .find(';')
            .filter(|end| *end <= 10)
            .map(|end| (&after[..end], end));
        let character = reference.and_then(|(name, _)| match name {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            "nbsp" => Some(' '),
            _ => name
                .strip_prefix("#x")
                .or_else(|| name.strip_prefix("#X"))
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .or_else(|| name.strip_prefix('#').and_then(|dec| dec.parse().ok()))
                .and_then(char::from_u32),
        });
        match (character, reference) {
            (Some(character), Some((_, end))) => {
                decoded.push(character);
                rest = &after[end + 1..];
            }
            _ => {
                decoded.push('&');
                rest = after;
            }
        }
    }
    decoded.push_str(rest);
    decoded
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
        self.remove_expired(now);
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
        self.remove_expired(now);
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

    /// Drops every entry older than [`CACHE_TTL`], so that expired pages do not stay in
    /// memory until the next fetch.
    pub fn remove_expired(&mut self, now: Instant) {
        self.entries
            .retain(|_, entry| now.duration_since(entry.stored_at) < CACHE_TTL);
        self.size = self
            .entries
            .values()
            .map(|entry| entry.content.content.len().max(1))
            .sum();
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
