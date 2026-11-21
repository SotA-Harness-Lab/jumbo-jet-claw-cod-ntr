// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;
use std::time::Instant;

use pretty_assertions::assert_eq;
use url::Url;

use super::CACHE_TTL;
use super::CommandShell;
use super::FetchCache;
use super::FetchOutput;
use super::FetchedContent;
use super::MAX_MARKDOWN_LENGTH;
use super::command_error_message;
use super::description;
use super::fetch_command;
use super::fetch_command_to_file;
use super::html_text;
use super::is_permitted_redirect;
use super::is_preapproved;
use super::parse_fetch_output;
use super::redirect_message;
use super::returns_content_unprocessed;
use super::secondary_model_answer;
use super::secondary_model_prompt;
use super::to_markdown;
use super::truncate_markdown;
use super::upgrade_to_https;
use super::validate_url;

fn url(text: &str) -> Url {
    Url::parse(text).unwrap_or_else(|err| panic!("invalid test URL {text}: {err}"))
}

#[test]
fn validation_follows_claude_code() {
    assert!(validate_url("https://example.com/page").is_some());
    assert!(validate_url("http://docs.example.org").is_some());
    assert!(validate_url("not a url").is_none());
    assert!(validate_url("https://localhost/admin").is_none());
    assert!(validate_url("https://user:secret@example.com/").is_none());
    let long = format!("https://example.com/{}", "a".repeat(2000));
    assert!(validate_url(&long).is_none());
}

#[test]
fn http_is_upgraded() {
    assert_eq!(
        upgrade_to_https(url("http://example.com/a?b=1")).as_str(),
        "https://example.com/a?b=1"
    );
    assert_eq!(
        upgrade_to_https(url("https://example.com/")).as_str(),
        "https://example.com/"
    );
}

#[test]
fn only_same_site_redirects_are_followed() {
    let original = url("https://example.com/a");
    assert!(is_permitted_redirect(
        &original,
        &url("https://example.com/b")
    ));
    assert!(is_permitted_redirect(
        &original,
        &url("https://www.example.com/a/")
    ));
    assert!(is_permitted_redirect(
        &url("https://www.example.com/"),
        &url("https://example.com/")
    ));
    assert!(!is_permitted_redirect(
        &original,
        &url("https://evil.com/a")
    ));
    assert!(!is_permitted_redirect(
        &original,
        &url("http://example.com/a")
    ));
    assert!(!is_permitted_redirect(
        &original,
        &url("https://example.com:8443/a")
    ));
    assert!(!is_permitted_redirect(
        &original,
        &url("https://user@example.com/a")
    ));
}

#[test]
fn redirect_message_names_both_urls() {
    assert_eq!(
        redirect_message("https://a.com/", "https://b.com/", 301, "summarize"),
        "REDIRECT DETECTED: The URL redirects to a different host.\n\nOriginal URL: https://a.com/\nRedirect URL: https://b.com/\nStatus: 301 Moved Permanently\n\nTo complete your request, I need to fetch content from the redirected URL. Please use WebFetch again with these parameters:\n- url: \"https://b.com/\"\n- prompt: \"summarize\""
    );
}

#[test]
fn fetch_command_quotes_for_each_shell() {
    let target = url("https://example.com/it's?q=a b");
    let posix = fetch_command(&target, CommandShell::Posix);
    // Quoted for zsh, which expands a leading `=`.
    assert!(posix.starts_with("curl --silent --show-error --globoff --proto '=https' "));
    assert!(posix.ends_with("'https://example.com/it'\\''s?q=a%20b'"));
    assert!(posix.contains("--max-filesize 10485760"));
    let powershell = fetch_command(&target, CommandShell::PowerShell);
    assert!(powershell.starts_with("curl.exe "));
    assert!(powershell.ends_with("'https://example.com/it''s?q=a%20b'"));
    let cmd = fetch_command(&target, CommandShell::Cmd);
    assert!(cmd.ends_with("\"https://example.com/it's?q=a%20b\""));
}

#[test]
fn fetch_command_can_write_the_body_to_a_file() {
    let target = url("https://example.com/");
    let posix = fetch_command_to_file(&target, CommandShell::Posix, "/tmp/ccn webfetch");
    assert!(
        posix.ends_with(" --output '/tmp/ccn webfetch' 'https://example.com/'"),
        "{posix}"
    );
    let cmd = fetch_command_to_file(&target, CommandShell::Cmd, r"C:\Temp\body");
    assert!(
        cmd.ends_with(r#" --output "C:\Temp\body" "https://example.com/""#),
        "{cmd}"
    );
}

#[test]
fn output_is_split_into_body_and_fields() {
    let output = "<p>hi</p>\n__CODEX_WEBFETCH__\t200\t9\t\ttext/html; charset=utf-8";
    assert_eq!(
        parse_fetch_output(output),
        Some(FetchOutput {
            status: 200,
            bytes: 9,
            redirect_url: "",
            content_type: "text/html; charset=utf-8",
            body: "<p>hi</p>",
        })
    );
    let redirect = "\n__CODEX_WEBFETCH__\t301\t0\thttps://other.org/\t";
    assert_eq!(
        parse_fetch_output(redirect).map(|output| (output.status, output.redirect_url)),
        Some((301, "https://other.org/"))
    );
    assert_eq!(parse_fetch_output("curl: (6) Could not resolve host"), None);
}

#[test]
fn curl_failures_become_messages() {
    assert_eq!(
        command_error_message(Some(6), "curl: (6) Could not resolve host: x.example\n"),
        "curl: (6) Could not resolve host: x.example"
    );
    assert_eq!(
        command_error_message(Some(63), ""),
        "Response exceeds the maximum size of 10485760 bytes"
    );
    assert_eq!(
        command_error_message(Some(28), "\n__CODEX_WEBFETCH__\t000\t0\t\t"),
        "timeout of 60000ms exceeded"
    );
}

#[test]
fn html_becomes_markdown_and_other_content_is_kept() {
    let markdown = to_markdown(
        "<html><head><script>x()</script></head><body><h1>Title</h1><p>Some <a href=\"https://a.com\">link</a></p></body></html>",
        "text/html; charset=utf-8",
    );
    assert!(markdown.contains("# Title"), "{markdown}");
    assert!(
        markdown.contains("Some [link](https://a.com)"),
        "{markdown}"
    );
    assert!(!markdown.contains("x()"), "{markdown}");
    assert_eq!(to_markdown("<b>raw</b>", "text/plain"), "<b>raw</b>");
}

#[test]
fn row_headers_keep_their_column() {
    let markdown = to_markdown(
        "<table><thead><tr><th>Country</th><th>Population</th></tr></thead>\
         <tbody><tr><TH scope=\"row\">France</TH><td>68M</td></tr>\
         <tr><th>Chad</th><td></td></tr></tbody></table>",
        "text/html",
    );
    assert_eq!(
        markdown,
        "| Country | Population |\n| ------- | ---------- |\n| France  | 68M        |\n| Chad    |            |"
    );
    // Without `<thead>` every row is a body row; the row headers stay in the first column.
    let markdown = to_markdown(
        "<table><tr><td></td><th>2025</th></tr><tr><th>Sales</th><td>9</td></tr></table>",
        "text/html",
    );
    assert_eq!(markdown, "|       | 2025 |\n| Sales | 9    |");
}

#[test]
fn deeply_nested_html_becomes_plain_text() {
    let html = format!("{}<b>deep</b> &amp; done", "<div>".repeat(100_000));
    assert_eq!(to_markdown(&html, "text/html"), "deep & done");

    // End tags of elements that are not open close nothing.
    let html = format!("{}<b>deep</b>", "<div></span>".repeat(2_000));
    assert_eq!(to_markdown(&html, "text/html"), "deep");
}

#[test]
fn nesting_the_parser_undoes_is_converted() {
    // Paragraphs and list items without end tags, closed elements, `<br>` and the shapes
    // inside an SVG do not nest.
    let html = format!(
        "{}{}{}<svg>{}</svg><b>bold</b>",
        "<p>a".repeat(2_000),
        "<ul><li>b</ul>".repeat(2_000),
        "<span>c<br></span>".repeat(2_000),
        "<path d=\"M0 0\"/>".repeat(2_000),
    );
    assert!(to_markdown(&html, "text/html").ends_with("**bold**"));

    let html = format!("{}<b>bold</b>", "<div>".repeat(200));
    assert_eq!(to_markdown(&html, "text/html"), "**bold**");
}

#[test]
fn plain_text_of_html_keeps_blocks_and_skips_scripts() {
    assert_eq!(
        html_text(
            "<!DOCTYPE html><title>T</title><script>if (a < b) x()</script><!-- <div> -->\
             <div>a &lt; b&nbsp;&#x41;</div><p class=\"x>y\">c  d</p>1 < 2"
        ),
        "T\na < b A\nc d\n1 < 2"
    );
}

#[test]
fn preapproved_hosts_match_whole_segments() {
    assert!(is_preapproved(&url("https://docs.python.org/3/library/")));
    assert!(is_preapproved(&url("https://github.com/anthropics")));
    assert!(is_preapproved(&url(
        "https://github.com/anthropics/claude-code"
    )));
    assert!(!is_preapproved(&url(
        "https://github.com/anthropics-evil/x"
    )));
    assert!(!is_preapproved(&url("https://github.com/openai")));
    assert!(!is_preapproved(&url("https://example.com/")));
}

#[test]
fn preapproved_markdown_skips_the_secondary_model() {
    assert!(returns_content_unprocessed(true, "text/markdown", "# Doc"));
    assert!(!returns_content_unprocessed(
        false,
        "text/markdown",
        "# Doc"
    ));
    assert!(!returns_content_unprocessed(true, "text/html", "# Doc"));
}

#[test]
fn secondary_prompt_carries_content_prompt_and_guidelines() {
    let prompt = secondary_model_prompt("# Doc", "What is it?", false);
    assert!(prompt.starts_with("\nWeb page content:\n---\n# Doc\n---\n\nWhat is it?\n\n"));
    assert!(prompt.contains("125-character maximum"));
    let preapproved = secondary_model_prompt("# Doc", "What is it?", true);
    assert!(preapproved.contains("Include relevant details, code examples"));
}

#[test]
fn long_markdown_is_truncated() {
    let long = "é".repeat(MAX_MARKDOWN_LENGTH + 5);
    let truncated = truncate_markdown(&long);
    assert!(truncated.ends_with("\n\n[Content truncated due to length...]"));
    assert_eq!(
        truncated.chars().count(),
        MAX_MARKDOWN_LENGTH + "\n\n[Content truncated due to length...]".len()
    );
    assert_eq!(truncate_markdown("short"), "short");
}

#[test]
fn cache_expires_after_fifteen_minutes() {
    let mut cache = FetchCache::default();
    let start = Instant::now();
    let page = FetchedContent {
        bytes: 3,
        code: 200,
        code_text: "OK".to_string(),
        content: "abc".to_string(),
        content_type: "text/plain".to_string(),
    };
    cache.insert("http://a.com/".to_string(), page.clone(), start);
    assert_eq!(
        cache.get("http://a.com/", start + Duration::from_secs(60)),
        Some(page)
    );
    assert_eq!(cache.get("http://a.com/", start + CACHE_TTL), None);
}

#[test]
fn expired_pages_are_dropped_without_a_fetch_of_their_own() {
    let mut cache = FetchCache::default();
    let start = Instant::now();
    let page = FetchedContent {
        bytes: 3,
        code: 200,
        code_text: "OK".to_string(),
        content: "abc".to_string(),
        content_type: "text/plain".to_string(),
    };
    cache.insert("http://a.com/".to_string(), page.clone(), start);
    let later = start + Duration::from_secs(10 * 60);
    cache.insert("http://b.com/".to_string(), page.clone(), later);

    // A lookup of another page drops the expired one.
    assert_eq!(cache.get("http://c.com/", start + CACHE_TTL), None);
    assert_eq!(cache.entries.len(), 1);
    assert_eq!(cache.size, 3);
    assert_eq!(cache.get("http://b.com/", start + CACHE_TTL), Some(page));
    cache.remove_expired(later + CACHE_TTL);
    assert_eq!((cache.entries.len(), cache.size), (0, 0));
}

#[test]
fn description_points_to_bash_only_when_offered() {
    assert!(description(&["Bash"]).contains("gh CLI via Bash"));
    assert!(!description(&[]).contains("gh CLI"));
}

#[test]
fn an_empty_answer_fails_the_call() {
    assert_eq!(
        secondary_model_answer("The page says hi.".to_string()),
        Ok("The page says hi.".to_string())
    );
    let error = secondary_model_answer(" \n".to_string()).unwrap_err();
    assert!(
        error.starts_with("The page could not be summarized"),
        "{error}"
    );
    assert!(error.contains("curl through Bash"), "{error}");
}
