// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Bash(...)` rules: how a rule's content is read (exact command, legacy
//! `prefix:*`, `*` wildcards) and how a command is matched against it, after Claude Code's
//! `bashPermissions.ts` and `shellRuleMatching.ts`.
//!
//! A command is split into its subcommands (`&&`, `||`, `;`, `|`, `&`, line breaks) and
//! each is matched on its own, as Claude Code does: a deny or ask rule for any subcommand
//! decides the whole command, an allow rule allows it only when every subcommand is allowed.
//! Commands Claude Code does not split but asks about (command substitution, subshells,
//! control flow) are left to Codex unless an exact rule or a deny rule names them.

use std::collections::HashSet;
use std::path::Path;
use std::sync::LazyLock;

use regex_lite::Regex;

/// Environment variables Claude Code drops from the front of a command before matching it
/// against allow rules (`SAFE_ENV_VARS`): they cannot run code or load libraries.
const SAFE_ENV_VARS: &[&str] = &[
    "GOEXPERIMENT",
    "GOOS",
    "GOARCH",
    "CGO_ENABLED",
    "GO111MODULE",
    "RUST_BACKTRACE",
    "RUST_LOG",
    "NODE_ENV",
    "PYTHONUNBUFFERED",
    "PYTHONDONTWRITEBYTECODE",
    "PYTEST_DISABLE_PLUGIN_AUTOLOAD",
    "PYTEST_DEBUG",
    "ANTHROPIC_API_KEY",
    "LANG",
    "LANGUAGE",
    "LC_ALL",
    "LC_CTYPE",
    "LC_TIME",
    "CHARSET",
    "TERM",
    "COLORTERM",
    "NO_COLOR",
    "FORCE_COLOR",
    "TZ",
    "LS_COLORS",
    "LSCOLORS",
    "GREP_COLOR",
    "GREP_COLORS",
    "GCC_COLORS",
    "TIME_STYLE",
    "BLOCK_SIZE",
    "BLOCKSIZE",
];

/// Words that start a shell construct Claude Code's parser does not take apart.
const COMPOUND_KEYWORDS: &[&str] = &[
    "if", "then", "elif", "else", "fi", "for", "while", "until", "do", "done", "case", "esac",
    "select", "function",
];

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap_or_else(|err| panic!("invalid pattern {pattern}: {err}"))
}

/// `timeout`, `time`, `nice`, `stdbuf` and `nohup` in front of a command
/// (`SAFE_WRAPPER_PATTERNS`).
static SAFE_WRAPPERS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"^timeout[ \t]+(?:(?:--(?:foreground|preserve-status|verbose)|--(?:kill-after|signal)=[A-Za-z0-9_.+-]+|--(?:kill-after|signal)[ \t]+[A-Za-z0-9_.+-]+|-v|-[ks][ \t]+[A-Za-z0-9_.+-]+|-[ks][A-Za-z0-9_.+-]+)[ \t]+)*(?:--[ \t]+)?\d+(?:\.\d+)?[smhd]?[ \t]+",
        r"^time[ \t]+(?:--[ \t]+)?",
        r"^nice(?:[ \t]+-n[ \t]+-?\d+|[ \t]+-\d+)?[ \t]+(?:--[ \t]+)?",
        r"^stdbuf(?:[ \t]+-[ioe][LN0-9]+)+[ \t]+(?:--[ \t]+)?",
        r"^nohup[ \t]+(?:--[ \t]+)?",
    ]
    .into_iter()
    .map(regex)
    .collect()
});

/// A safe variable assignment in front of a command.
static SAFE_ENV_VAR: LazyLock<Regex> =
    LazyLock::new(|| regex(r"^([A-Za-z_][A-Za-z0-9_]*)=([A-Za-z0-9_./:-]+)[ \t]+"));

/// Any variable assignment in front of a command, for deny and ask rules.
static ANY_ENV_VAR: LazyLock<Regex> = LazyLock::new(|| {
    regex(
        r#"^([A-Za-z_][A-Za-z0-9_]*(?:\[[^\]]*\])?)\+?=(?:'[^'\n\r]*'|"(?:\\.|[^"$`\\\n\r])*"|\\.|[^ \t\n\r$`;|&()<>\\'"])*[ \t]+"#,
    )
});

/// What a `Bash(...)` rule's content asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellRule<'a> {
    /// The command, exactly.
    Exact(&'a str),
    /// Legacy `prefix:*`: the command, or the command followed by a space and more.
    Prefix(&'a str),
    /// A pattern with `*` wildcards.
    Wildcard(&'a str),
}

pub(crate) fn parse_shell_rule(content: &str) -> ShellRule<'_> {
    if let Some(prefix) = content.strip_suffix(":*")
        && !prefix.is_empty()
    {
        return ShellRule::Prefix(prefix);
    }
    if has_wildcards(content) {
        ShellRule::Wildcard(content)
    } else {
        ShellRule::Exact(content)
    }
}

/// Whether a pattern has a `*` not escaped by a backslash (`:*` at the end is a prefix).
fn has_wildcards(pattern: &str) -> bool {
    if pattern.ends_with(":*") {
        return false;
    }
    let bytes = pattern.as_bytes();
    bytes.iter().enumerate().any(|(index, byte)| {
        *byte == b'*'
            && bytes[..index]
                .iter()
                .rev()
                .take_while(|byte| **byte == b'\\')
                .count()
                % 2
                == 0
    })
}

/// Claude Code's wildcard match: `*` is any text (line breaks included), `\*` a literal
/// star, `\\` a backslash; the whole command must match. A pattern whose only wildcard is a
/// trailing ` *` also matches the command without it (`git *` matches `git`).
pub(crate) fn matches_wildcard(pattern: &str, command: &str) -> bool {
    let pattern = pattern.trim();
    let chars = pattern.chars().collect::<Vec<_>>();
    let mut expression = String::new();
    let mut stars = 0;
    let mut index = 0;
    while index < chars.len() {
        let current = chars[index];
        if current == '\\'
            && let Some(next @ ('*' | '\\')) = chars.get(index + 1)
        {
            expression.push_str(&regex_lite::escape(&next.to_string()));
            index += 2;
            continue;
        }
        if current == '*' {
            expression.push_str(".*");
            stars += 1;
        } else {
            expression.push_str(&regex_lite::escape(&current.to_string()));
        }
        index += 1;
    }
    if stars == 1 && expression.ends_with(" .*") {
        expression.truncate(expression.len() - 3);
        expression.push_str("( .*)?");
    }
    Regex::new(&format!("(?s)^{expression}$")).is_ok_and(|regex| regex.is_match(command))
}

/// Drops full-line comments; a command of comments only is kept as it is.
fn strip_comment_lines(command: &str) -> String {
    let lines = command
        .split('\n')
        .filter(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#')
        })
        .collect::<Vec<_>>();
    if lines.is_empty() {
        command.to_string()
    } else {
        lines.join("\n")
    }
}

/// The command without safe variable assignments and wrapper commands in front
/// (`stripSafeWrappers`).
pub(crate) fn strip_safe_wrappers(command: &str) -> String {
    let mut stripped = command.to_string();
    loop {
        let previous = stripped.clone();
        stripped = strip_comment_lines(&stripped);
        if let Some(captures) = SAFE_ENV_VAR.captures(&stripped)
            && SAFE_ENV_VARS.contains(&&captures[1])
        {
            let end = captures.get(0).map_or(0, |found| found.end());
            stripped = stripped[end..].to_string();
        }
        if stripped == previous {
            break;
        }
    }
    loop {
        let previous = stripped.clone();
        stripped = strip_comment_lines(&stripped);
        for wrapper in SAFE_WRAPPERS.iter() {
            stripped = wrapper.replace(&stripped, "").into_owned();
        }
        if stripped == previous {
            break;
        }
    }
    stripped.trim().to_string()
}

/// The command without any variable assignments in front (`stripAllLeadingEnvVars`), so
/// that `FOO=bar rm x` still meets a deny rule for `rm`.
pub(crate) fn strip_all_leading_env_vars(command: &str) -> String {
    let mut stripped = command.to_string();
    loop {
        let previous = stripped.clone();
        stripped = strip_comment_lines(&stripped);
        if let Some(found) = ANY_ENV_VAR.find(&stripped) {
            stripped = stripped[found.end()..].to_string();
        }
        if stripped == previous {
            break;
        }
    }
    stripped.trim().to_string()
}

/// A command taken apart at its operators.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SplitCommand {
    /// Each simple command, trimmed, with its redirections; heredoc bodies are left out.
    pub(crate) subcommands: Vec<String>,
    /// The command has a construct Claude Code does not split (command or process
    /// substitution, a subshell or group, control flow).
    pub(crate) complex: bool,
}

/// Splits a command at `&&`, `||`, `;`, `|`, `|&`, `&` and line breaks outside quotes, as
/// Claude Code takes a command into its simple commands. `None` when a quote is not closed.
pub(crate) fn split_command(command: &str) -> Option<SplitCommand> {
    let chars = command.chars().collect::<Vec<_>>();
    let mut subcommands = Vec::new();
    let mut complex = false;
    let mut current = String::new();
    let mut word_start = true;
    let mut heredocs: Vec<(String, bool)> = Vec::new();
    let mut index = 0;

    let finish = |current: &mut String, subcommands: &mut Vec<String>| {
        let trimmed = current.trim();
        if !trimmed.is_empty() {
            subcommands.push(trimmed.to_string());
        }
        current.clear();
    };

    while index < chars.len() {
        let c = chars[index];
        let next = chars.get(index + 1).copied();
        match c {
            '\'' => {
                let end = chars[index + 1..].iter().position(|c| *c == '\'')? + index + 1;
                current.extend(&chars[index..=end]);
                index = end + 1;
                word_start = false;
            }
            '"' => {
                current.push('"');
                index += 1;
                loop {
                    let c = *chars.get(index)?;
                    match c {
                        '\\' => {
                            current.push(c);
                            if let Some(escaped) = chars.get(index + 1) {
                                current.push(*escaped);
                            }
                            index += 2;
                        }
                        '"' => {
                            current.push(c);
                            index += 1;
                            break;
                        }
                        '`' => {
                            complex = true;
                            current.push(c);
                            index += 1;
                        }
                        '$' if chars.get(index + 1) == Some(&'(') => {
                            complex = true;
                            current.push(c);
                            index += 1;
                        }
                        _ => {
                            current.push(c);
                            index += 1;
                        }
                    }
                }
                word_start = false;
            }
            '\\' => {
                match next {
                    // A line continuation joins the lines without a space.
                    Some('\n') => {}
                    Some(escaped) => {
                        current.push(c);
                        current.push(escaped);
                        word_start = false;
                    }
                    None => current.push(c),
                }
                index += 2;
            }
            '#' if word_start => {
                while index < chars.len() && chars[index] != '\n' {
                    index += 1;
                }
            }
            '\n' => {
                finish(&mut current, &mut subcommands);
                index += 1;
                for (delimiter, strip_tabs) in heredocs.drain(..) {
                    while index < chars.len() {
                        let end = chars[index..]
                            .iter()
                            .position(|c| *c == '\n')
                            .map_or(chars.len(), |offset| index + offset);
                        let line = chars[index..end].iter().collect::<String>();
                        index = (end + 1).min(chars.len());
                        let line = if strip_tabs {
                            line.trim_start_matches('\t')
                        } else {
                            line.as_str()
                        };
                        if line == delimiter {
                            break;
                        }
                    }
                }
                word_start = true;
            }
            ';' => {
                if next == Some(';') {
                    complex = true;
                }
                finish(&mut current, &mut subcommands);
                index += 1;
                word_start = true;
            }
            '&' if next == Some('&') => {
                finish(&mut current, &mut subcommands);
                index += 2;
                word_start = true;
            }
            '|' if next == Some('|') => {
                finish(&mut current, &mut subcommands);
                index += 2;
                word_start = true;
            }
            '|' => {
                finish(&mut current, &mut subcommands);
                index += if next == Some('&') { 2 } else { 1 };
                word_start = true;
            }
            '&' if next == Some('>')
                || matches!(index.checked_sub(1).map(|i| chars[i]), Some('>' | '<')) =>
            {
                current.push(c);
                index += 1;
                word_start = false;
            }
            '&' => {
                finish(&mut current, &mut subcommands);
                index += 1;
                word_start = true;
            }
            '<' if next == Some('<') && chars.get(index + 2) != Some(&'<') => {
                current.push_str("<<");
                index += 2;
                let strip_tabs = chars.get(index) == Some(&'-');
                if strip_tabs {
                    current.push('-');
                    index += 1;
                }
                while matches!(chars.get(index), Some(' ' | '\t')) {
                    current.push(chars[index]);
                    index += 1;
                }
                let mut delimiter = String::new();
                while let Some(&c) = chars.get(index) {
                    if c.is_whitespace() || matches!(c, ';' | '&' | '|' | '<' | '>' | '(' | ')') {
                        break;
                    }
                    current.push(c);
                    if !matches!(c, '\'' | '"' | '\\') {
                        delimiter.push(c);
                    }
                    index += 1;
                }
                heredocs.push((delimiter, strip_tabs));
                word_start = false;
            }
            '<' | '>' if next == Some('(') => {
                complex = true;
                current.push(c);
                index += 1;
                word_start = false;
            }
            '`' => {
                complex = true;
                current.push(c);
                index += 1;
                word_start = false;
            }
            '$' if next == Some('(') => {
                complex = true;
                current.push(c);
                index += 1;
                word_start = false;
            }
            '(' | ')' => {
                complex = true;
                current.push(c);
                index += 1;
                word_start = true;
            }
            '{' | '}' if word_start && next.is_none_or(char::is_whitespace) => {
                complex = true;
                current.push(c);
                index += 1;
            }
            c if c.is_whitespace() => {
                current.push(c);
                index += 1;
                word_start = true;
            }
            c => {
                current.push(c);
                index += 1;
                word_start = false;
            }
        }
    }
    finish(&mut current, &mut subcommands);

    complex |= subcommands.iter().any(|subcommand| {
        subcommand
            .split_whitespace()
            .next()
            .is_some_and(|word| COMPOUND_KEYWORDS.contains(&word))
    });
    Some(SplitCommand {
        subcommands,
        complex,
    })
}

/// A word of a command with where it is, quotes kept.
struct Word {
    text: String,
    start: usize,
    end: usize,
}

/// The words of a simple command, split at unquoted blanks.
fn words(command: &str) -> Vec<Word> {
    let mut words = Vec::new();
    let mut text = String::new();
    let mut start = None;
    let mut quote = None;
    let mut escaped = false;
    for (index, c) in command.char_indices() {
        if start.is_none() {
            if c == ' ' || c == '\t' {
                continue;
            }
            start = Some(index);
        }
        text.push(c);
        if escaped {
            escaped = false;
        } else if c == '\\' && quote != Some('\'') {
            escaped = true;
        } else if let Some(open) = quote {
            if c == open {
                quote = None;
            }
        } else if c == '\'' || c == '"' {
            quote = Some(c);
        } else if c == ' ' || c == '\t' {
            text.pop();
            words.push(Word {
                text: std::mem::take(&mut text),
                start: start.take().unwrap_or(index),
                end: index,
            });
        }
    }
    if let Some(start) = start {
        words.push(Word {
            text,
            start,
            end: command.len(),
        });
    }
    words
}

/// Whether a redirection target is a plain file name Claude Code drops before matching
/// (`isStaticRedirectTarget`).
fn is_static_redirect_target(target: &str) -> bool {
    !target.is_empty()
        && !target.contains(|c: char| c.is_whitespace() || c == '\'' || c == '"')
        && !target.starts_with(['#', '!', '=', '&'])
        && !target.contains(['$', '`', '*', '?', '[', '{', '~', '(', '<'])
}

/// The output redirection operator a word starts with, and the rest of the word.
fn output_redirection(word: &str) -> Option<&str> {
    let unnumbered = word.trim_start_matches(|c: char| c.is_ascii_digit());
    let numbered = unnumbered.len() < word.len();
    for operator in ["&>>", "&>", ">>", ">&", ">|", ">"] {
        if let Some(rest) = unnumbered.strip_prefix(operator) {
            // `2&>` is not an operator.
            if numbered && operator.starts_with('&') {
                return None;
            }
            return Some(rest);
        }
    }
    None
}

/// The command without its output redirections to plain files and file descriptors
/// (`extractOutputRedirections`), so that `Bash(python:*)` matches `python x.py > out`.
pub(crate) fn strip_output_redirections(command: &str) -> String {
    let words = words(command);
    let mut dropped = vec![false; words.len()];
    let mut index = 0;
    while index < words.len() {
        let Some(rest) = output_redirection(&words[index].text) else {
            index += 1;
            continue;
        };
        let (target, target_index) = if rest.is_empty() {
            match words.get(index + 1) {
                Some(word) => (word.text.as_str(), Some(index + 1)),
                None => {
                    index += 1;
                    continue;
                }
            }
        } else {
            (rest, None)
        };
        let target = target.strip_prefix('&').unwrap_or(target);
        let descriptor = matches!(target, "0" | "1" | "2" | "-");
        if descriptor || is_static_redirect_target(target) {
            dropped[index] = true;
            if let Some(target_index) = target_index {
                dropped[target_index] = true;
            }
        }
        index += target_index.map_or(1, |_| 2);
    }
    if !dropped.contains(&true) {
        return command.to_string();
    }
    words
        .iter()
        .zip(dropped)
        .filter(|(_, dropped)| !dropped)
        .map(|(word, _)| &command[word.start..word.end])
        .collect::<Vec<_>>()
        .join(" ")
}

/// How a rule is matched: against the whole command (`exact`, wildcards never match), or
/// against one simple command (`prefix`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum MatchMode {
    Exact,
    Prefix,
}

/// The forms of a command Claude Code tries against rules: as written, without output
/// redirections, without safe wrappers, and for deny and ask rules without any variable
/// assignments in front (`filterRulesByContentsMatchingInput`).
fn candidates(command: &str, mode: MatchMode, strip_all_env_vars: bool) -> Vec<String> {
    let command = command.trim();
    let without_redirections = strip_output_redirections(command);
    let for_matching = match mode {
        MatchMode::Exact => vec![command.to_string(), without_redirections],
        MatchMode::Prefix => vec![without_redirections],
    };
    let mut candidates = Vec::new();
    for candidate in for_matching {
        let stripped = strip_safe_wrappers(&candidate);
        let differs = stripped != candidate;
        candidates.push(candidate);
        if differs {
            candidates.push(stripped);
        }
    }
    if strip_all_env_vars {
        let mut seen = candidates.iter().cloned().collect::<HashSet<_>>();
        let mut start = 0;
        while start < candidates.len() {
            let end = candidates.len();
            for index in start..end {
                for stripped in [
                    strip_all_leading_env_vars(&candidates[index]),
                    strip_safe_wrappers(&candidates[index]),
                ] {
                    if seen.insert(stripped.clone()) {
                        candidates.push(stripped);
                    }
                }
            }
            start = end;
        }
    }
    candidates
}

fn rule_matches(rule: ShellRule<'_>, candidate: &str, mode: MatchMode, compound: bool) -> bool {
    match rule {
        ShellRule::Exact(command) => command == candidate,
        ShellRule::Prefix(prefix) => match mode {
            MatchMode::Exact => prefix == candidate,
            MatchMode::Prefix => {
                if compound {
                    return false;
                }
                let xargs = format!("xargs {prefix}");
                candidate == prefix
                    || candidate.starts_with(&format!("{prefix} "))
                    || candidate == xargs
                    || candidate.starts_with(&format!("{xargs} "))
            }
        },
        ShellRule::Wildcard(pattern) => {
            mode == MatchMode::Prefix && !compound && matches_wildcard(pattern, candidate)
        }
    }
}

/// The first rule (by its content) that matches `command`. Deny and ask rules are matched
/// harder to escape (`strict`): any variable assignment in front is dropped, and a rule also
/// meets a command that still looks compound.
fn first_match<'r>(
    rules: &[&'r str],
    command: &str,
    mode: MatchMode,
    strict: bool,
) -> Option<&'r str> {
    let candidates = candidates(command, mode, strict);
    let compound = candidates
        .iter()
        .map(|candidate| {
            mode == MatchMode::Prefix
                && !strict
                && split_command(candidate).is_none_or(|split| split.subcommands.len() > 1)
        })
        .collect::<Vec<_>>();
    rules.iter().copied().find(|content| {
        let rule = parse_shell_rule(content);
        candidates
            .iter()
            .zip(&compound)
            .any(|(candidate, compound)| rule_matches(rule, candidate, mode, *compound))
    })
}

/// The `Bash` rule contents of each behaviour.
pub(crate) struct ShellRules<'r> {
    pub(crate) deny: Vec<&'r str>,
    pub(crate) ask: Vec<&'r str>,
    pub(crate) allow: Vec<&'r str>,
}

/// What the rules decide for a command, with the content of the deciding rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ShellDecision<'r> {
    Deny(&'r str),
    Ask(&'r str),
    Allow(&'r str),
    NoMatch,
}

impl<'r> ShellRules<'r> {
    fn exact(&self, command: &str) -> ShellDecision<'r> {
        if let Some(rule) = first_match(&self.deny, command, MatchMode::Exact, true) {
            ShellDecision::Deny(rule)
        } else if let Some(rule) = first_match(&self.ask, command, MatchMode::Exact, true) {
            ShellDecision::Ask(rule)
        } else if let Some(rule) = first_match(&self.allow, command, MatchMode::Exact, false) {
            ShellDecision::Allow(rule)
        } else {
            ShellDecision::NoMatch
        }
    }

    /// One simple command (`bashToolCheckPermission`, without Claude Code's path checks).
    fn subcommand(&self, command: &str) -> ShellDecision<'r> {
        let exact = self.exact(command);
        if matches!(exact, ShellDecision::Deny(_) | ShellDecision::Ask(_)) {
            return exact;
        }
        if let Some(rule) = first_match(&self.deny, command, MatchMode::Prefix, true) {
            return ShellDecision::Deny(rule);
        }
        if let Some(rule) = first_match(&self.ask, command, MatchMode::Prefix, true) {
            return ShellDecision::Ask(rule);
        }
        if exact != ShellDecision::NoMatch {
            return exact;
        }
        match first_match(&self.allow, command, MatchMode::Prefix, false) {
            Some(rule) => ShellDecision::Allow(rule),
            None => ShellDecision::NoMatch,
        }
    }

    /// A whole command (`bashToolHasPermission`), run in `cwd`.
    pub(crate) fn decide(&self, command: &str, cwd: &Path) -> ShellDecision<'r> {
        let exact = self.exact(command);
        let split = split_command(command);
        let Some(split) = split.filter(|split| !split.complex) else {
            // Claude Code asks about what it cannot take apart, unless an exact rule or a
            // deny rule decides; the asking is left to Codex.
            if exact != ShellDecision::NoMatch {
                return exact;
            }
            return match first_match(&self.deny, command, MatchMode::Prefix, true) {
                Some(rule) => ShellDecision::Deny(rule),
                None => ShellDecision::NoMatch,
            };
        };
        if let ShellDecision::Deny(_) = exact {
            return exact;
        }

        // Models often start with `cd` into the working directory, which changes nothing.
        let cwd = cwd.to_string_lossy();
        let cd_cwd = format!("cd {cwd}");
        let subcommands = split
            .subcommands
            .iter()
            .filter(|subcommand| **subcommand != cd_cwd)
            .collect::<Vec<_>>();
        let decisions = subcommands
            .iter()
            .map(|subcommand| self.subcommand(subcommand))
            .collect::<Vec<_>>();
        if let Some(denied) = decisions
            .iter()
            .find(|decision| matches!(decision, ShellDecision::Deny(_)))
        {
            return *denied;
        }
        if let Some(asked) = decisions
            .iter()
            .find(|decision| matches!(decision, ShellDecision::Ask(_)))
        {
            return *asked;
        }
        if let ShellDecision::Ask(_) = exact {
            return exact;
        }

        // Claude Code asks about several `cd`s, or `cd` with `git`, whatever the allow rules
        // say: such a command is not allowed by them here either.
        let cds = subcommands
            .iter()
            .filter(|subcommand| is_cd_command(subcommand))
            .count();
        let has_git = subcommands
            .iter()
            .any(|subcommand| is_git_command(subcommand));
        if cds > 1 || (cds == 1 && has_git) {
            return ShellDecision::NoMatch;
        }
        if let ShellDecision::Allow(_) = exact {
            return exact;
        }
        let mut allowed = None;
        for decision in &decisions {
            match decision {
                ShellDecision::Allow(rule) => {
                    allowed.get_or_insert(*rule);
                }
                _ => return ShellDecision::NoMatch,
            }
        }
        allowed.map_or(ShellDecision::NoMatch, ShellDecision::Allow)
    }
}

fn first_word(command: &str) -> Option<String> {
    let stripped = strip_safe_wrappers(command);
    let word = words(&stripped).into_iter().next()?.text;
    Some(word.replace(['\'', '"', '\\'], ""))
}

fn is_cd_command(command: &str) -> bool {
    first_word(command).is_some_and(|word| matches!(word.as_str(), "cd" | "pushd" | "popd"))
}

fn is_git_command(command: &str) -> bool {
    if command == "git" || command.starts_with("git ") {
        return true;
    }
    let stripped = strip_safe_wrappers(command);
    let words = words(&stripped)
        .into_iter()
        .map(|word| word.text.replace(['\'', '"', '\\'], ""))
        .collect::<Vec<_>>();
    match words.first().map(String::as_str) {
        Some("git") => true,
        Some("xargs") => words.iter().any(|word| word == "git"),
        _ => false,
    }
}
