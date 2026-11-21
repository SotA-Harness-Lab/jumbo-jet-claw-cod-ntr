// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `Bash(...)` rules: how a rule's content is read (exact command, legacy
//! `prefix:*`, `*` wildcards) and how a command is matched against it, after Claude Code's
//! `bashPermissions.ts` and `shellRuleMatching.ts`.
//!
//! A command is split into its subcommands (`&&`, `||`, `;`, `|`, `&`, line breaks) and
//! each is matched on its own, as Claude Code does: a deny or ask rule for any subcommand
//! decides the whole command, an allow rule allows it only when every subcommand is allowed.
//! The commands nested in command substitutions, subshells, groups and control flow are
//! taken out too: deny and ask rules meet them, as in Claude Code, but no prefix or wildcard
//! allow rule covers such a command, which is left to Codex unless an exact rule names it. A
//! substitution that only reads a quoted heredoc with `cat` (`"$(cat <<'EOF' ... EOF)"`) is
//! an inert argument.

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::PoisonError;

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

/// A word that assigns a variable (`NAME=`, `NAME+=`, `NAME[i]=`), its name unquoted.
static ASSIGNMENT: LazyLock<Regex> =
    LazyLock::new(|| regex(r"^[A-Za-z_][A-Za-z0-9_]*(?:\[[^\]]*\])?\+?="));

/// The shell builtins and zsh's `noglob` Claude Code drops from the front of a command, like
/// the wrappers above.
const SAFE_BUILTIN_WRAPPERS: &[&str] = &["command", "builtin", "noglob"];

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

/// The wildcard patterns compiled so far, each once: they come from the settings files and
/// hook conditions, while every command is checked against all of them.
static WILDCARDS: LazyLock<Mutex<HashMap<String, Option<Arc<Regex>>>>> =
    LazyLock::new(Mutex::default);

/// How many compiled patterns are kept before they are compiled anew.
const MAX_WILDCARDS: usize = 4096;

/// Claude Code's wildcard match: `*` is any text (line breaks included), `\*` a literal
/// star, `\\` a backslash; the whole command must match. A pattern whose only wildcard is a
/// trailing ` *` also matches the command without it (`git *` matches `git`).
pub(crate) fn matches_wildcard(pattern: &str, command: &str) -> bool {
    compiled_wildcard(pattern).is_some_and(|regex| regex.is_match(command))
}

/// The regular expression of a wildcard pattern, compiled on its first use; `None` when it
/// does not compile.
pub(crate) fn compiled_wildcard(pattern: &str) -> Option<Arc<Regex>> {
    let mut wildcards = WILDCARDS.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(regex) = wildcards.get(pattern) {
        return regex.clone();
    }
    let regex = wildcard_regex(pattern).map(Arc::new);
    if wildcards.len() >= MAX_WILDCARDS {
        wildcards.clear();
    }
    wildcards.insert(pattern.to_string(), regex.clone());
    regex
}

/// The regular expression of a wildcard pattern; `None` when it does not compile.
fn wildcard_regex(pattern: &str) -> Option<Regex> {
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
    Regex::new(&format!("(?s)^{expression}$")).ok()
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
        for wrapper in SAFE_BUILTIN_WRAPPERS.iter().copied() {
            if let Some(rest) = stripped.strip_prefix(wrapper)
                && rest.starts_with([' ', '\t'])
            {
                let rest = rest.trim_start_matches([' ', '\t']);
                // `command -v` looks a command up instead of running it.
                if !rest.starts_with('-') {
                    stripped = rest.to_string();
                }
            }
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

/// How deep command substitutions, subshells and backticks may nest before a command is taken
/// for one that cannot be taken apart.
const MAX_NESTING: usize = 32;

/// `$(cat <<'EOF' ... EOF)`: a heredoc with a quoted delimiter read by a bare `cat`, which
/// Claude Code takes for the literal text it is (`extractSafeCatHeredoc`).
static SAFE_CAT_HEREDOC: LazyLock<Regex> =
    LazyLock::new(|| regex(r#"^cat[ \t]+<<-?[ \t]*(?:'[^'\n]+'|"[^"\n]+"|\\[^ \t\n'"]+)$"#));

/// What Claude Code does not take for inert text even in such a heredoc.
static DANGEROUS_HEREDOC_BODY: LazyLock<Regex> =
    LazyLock::new(|| regex(r"/proc/[^/\n]*/environ|\bsystem\s*\("));

/// A command taken apart at its operators.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SplitCommand {
    /// Each simple command, trimmed, with its redirections; heredoc bodies are left out. The
    /// commands of the line come first, the keywords of control flow dropped, then the
    /// commands nested in command and process substitutions, backticks and subshells.
    pub(crate) subcommands: Vec<String>,
    /// The command has a construct Claude Code does not split for allow rules (command or
    /// process substitution, a subshell or group, control flow). A substitution that is only
    /// a quoted heredoc read by `cat` does not count.
    pub(crate) complex: bool,
}

/// Splits a command at `&&`, `||`, `;`, `|`, `|&`, `&` and line breaks outside quotes, as
/// Claude Code takes a command into its simple commands, and takes out the commands nested in
/// it. `None` when a quote, a substitution or a subshell is not closed, or a `)` closes
/// nothing.
pub(crate) fn split_command(command: &str) -> Option<SplitCommand> {
    let chars = command.chars().collect::<Vec<_>>();
    let scanned = Scanner::new(&chars, 0).scan(false)?;
    let mut subcommands = scanned.own;
    subcommands.extend(scanned.nested);
    Some(SplitCommand {
        subcommands,
        complex: scanned.complex,
    })
}

/// What a scan of a list of commands found.
#[derive(Default)]
struct Scanned {
    /// The simple commands of the list.
    own: Vec<String>,
    /// The simple commands nested in its substitutions and subshells.
    nested: Vec<String>,
    complex: bool,
}

impl Scanned {
    /// Takes in what the scan of a substitution or subshell found; of an array or a pattern
    /// (`nested_only`), only what is substituted in it runs.
    fn absorb(&mut self, inner: Scanned, nested_only: bool) {
        if !nested_only {
            self.nested.extend(inner.own);
        }
        self.nested.extend(inner.nested);
        self.complex = true;
    }

    /// Ends the simple command in `current`. Control flow keywords in front of it are dropped,
    /// and so is a command that is only the header of a loop, a `case` or a function, or the
    /// end of a block.
    fn finish(&mut self, current: &mut String) {
        let mut command = current.trim();
        loop {
            let (word, rest) = command
                .split_once(char::is_whitespace)
                .unwrap_or((command, ""));
            if !COMPOUND_KEYWORDS.contains(&word) {
                break;
            }
            self.complex = true;
            command = match word {
                "if" | "then" | "elif" | "else" | "do" | "while" | "until" => rest.trim_start(),
                // The substitutions of a header were taken out already.
                _ => "",
            };
        }
        if !command.is_empty() {
            self.own.push(command.to_string());
        }
        current.clear();
    }
}

/// A heredoc whose body starts at the next line break.
struct Heredoc {
    delimiter: String,
    strip_tabs: bool,
    /// The delimiter is quoted, so nothing in the body is expanded.
    quoted: bool,
}

/// Whether nothing but control flow keywords comes before the next word of a command.
fn at_command_start(current: &str) -> bool {
    current
        .split_whitespace()
        .all(|word| word == "!" || COMPOUND_KEYWORDS.contains(&word))
}

/// Where an arithmetic expression starting at `from` (after its `((`) ends, past its `))`;
/// `None` when the parentheses do not close as an arithmetic expression's.
fn arithmetic_end(chars: &[char], from: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, &c) in chars.iter().enumerate().skip(from) {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            ')' => return (chars.get(index + 1) == Some(&')')).then_some(index + 2),
            _ => {}
        }
    }
    None
}

/// Whether a command substitution, with the scan of what is inside it, is a quoted heredoc
/// read by a bare `cat`.
fn is_safe_cat_heredoc(inner: &Scanned, text: &[char]) -> bool {
    !inner.complex
        && inner.nested.is_empty()
        && matches!(inner.own.as_slice(), [command] if SAFE_CAT_HEREDOC.is_match(command))
        && !DANGEROUS_HEREDOC_BODY.is_match(&text.iter().collect::<String>())
}

/// Reads a command character by character, as far as a shell's parser would to find its
/// simple commands.
struct Scanner<'a> {
    chars: &'a [char],
    index: usize,
    /// How many substitutions and subshells the scan is inside.
    depth: usize,
}

impl<'a> Scanner<'a> {
    fn new(chars: &'a [char], depth: usize) -> Self {
        Self {
            chars,
            index: 0,
            depth,
        }
    }

    fn at(&self, offset: usize) -> Option<char> {
        self.chars.get(self.index + offset).copied()
    }

    /// Scans a list of commands to the end of the input or, `in_parens`, to the `)` that
    /// closes it, which is consumed.
    fn scan(&mut self, in_parens: bool) -> Option<Scanned> {
        let mut out = Scanned::default();
        let mut current = String::new();
        let mut word_start = true;
        let mut heredocs: Vec<Heredoc> = Vec::new();
        // The `case` statements open, whether the words before their `in` are being read, and
        // whether a pattern comes next.
        let mut cases = 0usize;
        let mut case_header = false;
        let mut case_pattern = false;

        while let Some(c) = self.at(0) {
            let next = self.at(1);
            if word_start && c.is_ascii_alphabetic() {
                let word = self.peek_word();
                if word == "case" && at_command_start(&current) {
                    cases += 1;
                    case_header = true;
                    out.complex = true;
                    current.push_str(&word);
                    self.index += word.len();
                    word_start = false;
                    continue;
                }
                if word == "in" && case_header {
                    case_header = false;
                    case_pattern = true;
                    current.clear();
                    self.index += word.len();
                    continue;
                }
                if word == "esac" && cases > 0 && (case_pattern || at_command_start(&current)) {
                    out.finish(&mut current);
                    cases -= 1;
                    case_pattern = false;
                    self.index += word.len();
                    word_start = false;
                    continue;
                }
            }
            if case_pattern && word_start && !c.is_whitespace() {
                self.skip_case_pattern()?;
                case_pattern = false;
                continue;
            }
            match c {
                '\'' => {
                    let end = self.chars[self.index + 1..]
                        .iter()
                        .position(|c| *c == '\'')?
                        + self.index
                        + 1;
                    current.extend(&self.chars[self.index..=end]);
                    self.index = end + 1;
                    word_start = false;
                }
                '"' => {
                    self.double_quoted(&mut current, &mut out)?;
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
                    self.index += 2;
                }
                '#' if word_start => {
                    while self.at(0).is_some_and(|c| c != '\n') {
                        self.index += 1;
                    }
                }
                '\n' => {
                    out.finish(&mut current);
                    self.index += 1;
                    for heredoc in heredocs.drain(..) {
                        self.heredoc_body(&heredoc, &mut out)?;
                    }
                    word_start = true;
                }
                ';' => {
                    let mut length = 1;
                    if next == Some(';') {
                        out.complex = true;
                        length = if self.at(2) == Some('&') { 3 } else { 2 };
                    } else if next == Some('&') && cases > 0 {
                        length = 2;
                    }
                    if length > 1 && cases > 0 {
                        case_pattern = true;
                    }
                    out.finish(&mut current);
                    self.index += length;
                    word_start = true;
                }
                '&' if next == Some('&') => {
                    out.finish(&mut current);
                    self.index += 2;
                    word_start = true;
                }
                '|' if next == Some('|') => {
                    out.finish(&mut current);
                    self.index += 2;
                    word_start = true;
                }
                '|' => {
                    out.finish(&mut current);
                    self.index += if next == Some('&') { 2 } else { 1 };
                    word_start = true;
                }
                '&' if next == Some('>')
                    || matches!(
                        self.index.checked_sub(1).map(|i| self.chars[i]),
                        Some('>' | '<')
                    ) =>
                {
                    current.push(c);
                    self.index += 1;
                    word_start = false;
                }
                '&' => {
                    out.finish(&mut current);
                    self.index += 1;
                    word_start = true;
                }
                '<' if next == Some('<') && self.at(2) != Some('<') => {
                    heredocs.push(self.heredoc_operator(&mut current));
                    word_start = false;
                }
                '<' | '>' if next == Some('(') => {
                    let start = self.index;
                    self.index += 2;
                    let inner = self.nested()?;
                    current.extend(&self.chars[start..self.index]);
                    out.absorb(inner, false);
                    word_start = false;
                }
                '`' => {
                    self.backtick(&mut current, &mut out)?;
                    word_start = false;
                }
                '$' if next == Some('(') => {
                    self.substitution(&mut current, &mut out)?;
                    word_start = false;
                }
                '$' if next == Some('{') => {
                    self.parameter(&mut current, &mut out)?;
                    word_start = false;
                }
                '(' => {
                    let start = self.index;
                    if word_start
                        && next == Some('(')
                        && let Some(end) = arithmetic_end(self.chars, start + 2)
                    {
                        // `(( expression ))`
                        Self::substitutions_in(
                            &self.chars[start + 2..end - 2],
                            self.depth,
                            &mut out,
                        )?;
                        current.extend(&self.chars[start..end]);
                        self.index = end;
                        out.complex = true;
                        word_start = false;
                    } else if word_start {
                        // A subshell: its commands run.
                        self.index += 1;
                        let inner = self.nested()?;
                        out.absorb(inner, false);
                        word_start = true;
                    } else {
                        // An array (`a=(x y)`) or a pattern (`!(x|y)`): only what is
                        // substituted in it runs.
                        self.index += 1;
                        let inner = self.nested()?;
                        current.extend(&self.chars[start..self.index]);
                        out.absorb(inner, true);
                        word_start = false;
                    }
                }
                ')' => {
                    if !in_parens {
                        return None;
                    }
                    out.finish(&mut current);
                    self.index += 1;
                    return Some(out);
                }
                // A group, `{ ...; }`: its commands are taken as the line's.
                '{' | '}'
                    if word_start
                        && (next.is_none_or(char::is_whitespace)
                            || (c == '}'
                                && matches!(next, Some(';' | '&' | '|' | ')' | '<' | '>')))) =>
                {
                    out.complex = true;
                    out.finish(&mut current);
                    self.index += 1;
                    word_start = true;
                }
                c if c.is_whitespace() => {
                    current.push(c);
                    self.index += 1;
                    word_start = true;
                }
                c => {
                    current.push(c);
                    self.index += 1;
                    word_start = false;
                }
            }
        }
        if in_parens {
            return None;
        }
        out.finish(&mut current);
        Some(out)
    }

    /// The scan of a substitution or subshell whose `(` was just read.
    fn nested(&mut self) -> Option<Scanned> {
        if self.depth >= MAX_NESTING {
            return None;
        }
        self.depth += 1;
        let scanned = self.scan(true);
        self.depth -= 1;
        scanned
    }

    /// The plain word at the current position, or an empty string when the word goes on with
    /// other characters (`in=1`, `done.txt`).
    fn peek_word(&self) -> String {
        let rest = &self.chars[self.index..];
        let length = rest
            .iter()
            .take_while(|c| c.is_ascii_alphanumeric() || **c == '_')
            .count();
        match rest.get(length) {
            Some(&c)
                if !c.is_whitespace() && !matches!(c, ';' | '&' | '|' | '(' | ')' | '<' | '>') =>
            {
                String::new()
            }
            _ => rest[..length].iter().collect(),
        }
    }

    /// Skips a `case` pattern up to and with the `)` that ends it.
    fn skip_case_pattern(&mut self) -> Option<()> {
        if self.at(0) == Some('(') {
            self.index += 1;
        }
        loop {
            match self.at(0)? {
                ')' => {
                    self.index += 1;
                    return Some(());
                }
                '\\' => self.index += 2,
                quote @ ('\'' | '"') => {
                    let length = self.chars[self.index + 1..]
                        .iter()
                        .position(|c| *c == quote)?;
                    self.index += length + 2;
                }
                _ => self.index += 1,
            }
        }
    }

    /// Reads a `<<` or `<<-` operator and its delimiter into `current`.
    fn heredoc_operator(&mut self, current: &mut String) -> Heredoc {
        current.push_str("<<");
        self.index += 2;
        let strip_tabs = self.at(0) == Some('-');
        if strip_tabs {
            current.push('-');
            self.index += 1;
        }
        while let Some(c @ (' ' | '\t')) = self.at(0) {
            current.push(c);
            self.index += 1;
        }
        let mut delimiter = String::new();
        let mut quoted = false;
        while let Some(c) = self.at(0) {
            if c.is_whitespace() || matches!(c, ';' | '&' | '|' | '<' | '>' | '(' | ')') {
                break;
            }
            current.push(c);
            if matches!(c, '\'' | '"' | '\\') {
                quoted = true;
            } else {
                delimiter.push(c);
            }
            self.index += 1;
        }
        Heredoc {
            delimiter,
            strip_tabs,
            quoted,
        }
    }

    /// Skips a heredoc's body and its delimiter line, taking out the commands substituted in
    /// the body unless the delimiter is quoted.
    fn heredoc_body(&mut self, heredoc: &Heredoc, out: &mut Scanned) -> Option<()> {
        let start = self.index;
        let mut body_end = self.chars.len();
        while self.index < self.chars.len() {
            let line_start = self.index;
            let end = self.chars[line_start..]
                .iter()
                .position(|c| *c == '\n')
                .map_or(self.chars.len(), |offset| line_start + offset);
            let line = self.chars[line_start..end].iter().collect::<String>();
            self.index = (end + 1).min(self.chars.len());
            let line = if heredoc.strip_tabs {
                line.trim_start_matches('\t')
            } else {
                line.as_str()
            };
            if line == heredoc.delimiter {
                body_end = line_start;
                break;
            }
        }
        if heredoc.quoted {
            return Some(());
        }
        Self::substitutions_in(&self.chars[start..body_end], self.depth, out)
    }

    /// A double-quoted string at the current position.
    fn double_quoted(&mut self, current: &mut String, out: &mut Scanned) -> Option<()> {
        current.push('"');
        self.index += 1;
        loop {
            let c = self.at(0)?;
            match c {
                '\\' => {
                    current.push(c);
                    if let Some(escaped) = self.at(1) {
                        current.push(escaped);
                    }
                    self.index += 2;
                }
                '"' => {
                    current.push(c);
                    self.index += 1;
                    return Some(());
                }
                '`' => self.backtick(current, out)?,
                '$' if self.at(1) == Some('(') => self.substitution(current, out)?,
                '$' if self.at(1) == Some('{') => self.parameter(current, out)?,
                _ => {
                    current.push(c);
                    self.index += 1;
                }
            }
        }
    }

    /// A `$(...)` or `$((...))` at the current position: its text goes into `current`, the
    /// commands in it into `out`.
    fn substitution(&mut self, current: &mut String, out: &mut Scanned) -> Option<()> {
        let start = self.index;
        if self.at(2) == Some('(')
            && let Some(end) = arithmetic_end(self.chars, start + 3)
        {
            Self::substitutions_in(&self.chars[start + 3..end - 2], self.depth, out)?;
            current.extend(&self.chars[start..end]);
            self.index = end;
            out.complex = true;
            return Some(());
        }
        self.index += 2;
        let inner = self.nested()?;
        let text = &self.chars[start..self.index];
        current.extend(text);
        if !is_safe_cat_heredoc(&inner, text) {
            out.absorb(inner, false);
        }
        Some(())
    }

    /// A backtick substitution at the current position.
    fn backtick(&mut self, current: &mut String, out: &mut Scanned) -> Option<()> {
        let start = self.index;
        let mut content = Vec::new();
        self.index += 1;
        loop {
            match self.at(0)? {
                '`' => break,
                '\\' => {
                    let escaped = self.at(1)?;
                    if !matches!(escaped, '`' | '\\' | '$') {
                        content.push('\\');
                    }
                    content.push(escaped);
                    self.index += 2;
                }
                c => {
                    content.push(c);
                    self.index += 1;
                }
            }
        }
        self.index += 1;
        current.extend(&self.chars[start..self.index]);
        if self.depth >= MAX_NESTING {
            return None;
        }
        let inner = Scanner::new(&content, self.depth + 1).scan(false)?;
        out.absorb(inner, false);
        Some(())
    }

    /// A `${...}` expansion at the current position; what is substituted in it is taken out.
    fn parameter(&mut self, current: &mut String, out: &mut Scanned) -> Option<()> {
        let start = self.index;
        let mut index = start + 2;
        let mut depth = 0usize;
        loop {
            match *self.chars.get(index)? {
                '\\' => index += 2,
                '{' => {
                    depth += 1;
                    index += 1;
                }
                '}' if depth == 0 => break,
                '}' => {
                    depth -= 1;
                    index += 1;
                }
                _ => index += 1,
            }
        }
        self.index = index + 1;
        current.extend(&self.chars[start..self.index]);
        Self::substitutions_in(&self.chars[start + 2..index], self.depth, out)
    }

    /// Takes out the commands of the `$(...)` and backticks in text that is not itself read as
    /// commands: an unquoted heredoc's body, a `${...}` expansion, an arithmetic expression.
    fn substitutions_in(text: &[char], depth: usize, out: &mut Scanned) -> Option<()> {
        let mut scanner = Scanner::new(text, depth);
        let mut ignored = String::new();
        while let Some(c) = scanner.at(0) {
            match c {
                '\\' => scanner.index += 2,
                '$' if scanner.at(1) == Some('(') => scanner.substitution(&mut ignored, out)?,
                '`' => scanner.backtick(&mut ignored, out)?,
                _ => scanner.index += 1,
            }
        }
        Some(())
    }
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
        // Runs of blanks between words are one space to a rule.
        let collapsed = collapse_blanks(&candidate);
        let mut forms = vec![candidate];
        if collapsed != forms[0] {
            forms.push(collapsed);
        }
        for candidate in forms {
            let stripped = strip_safe_wrappers(&candidate);
            let differs = stripped != candidate;
            candidates.push(candidate);
            if differs {
                candidates.push(stripped);
            }
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
        for index in 0..candidates.len() {
            for form in word_forms(&candidates[index]) {
                if seen.insert(form.clone()) {
                    candidates.push(form);
                }
            }
        }
    }
    candidates
}

/// The forms of a command deny and ask rules meet, for hook conditions too: as written and as
/// Claude Code strips it, then as its words are read by the shell, without the wrappers in
/// front that run the rest.
pub(crate) fn deny_forms(command: &str) -> Vec<String> {
    candidates(command, MatchMode::Prefix, true)
}

/// The command with its words, quotes kept, one space apart.
fn collapse_blanks(command: &str) -> String {
    words(command)
        .into_iter()
        .map(|word| word.text)
        .collect::<Vec<_>>()
        .join(" ")
}

/// The forms of a simple command its words give, for deny and ask rules: the words as the
/// shell passes them on (quotes removed, escapes resolved, one space apart) without the
/// variable assignments in front, and again without each wrapper in front that runs the rest
/// as the command (`sudo rm -rf build` is also `rm -rf build`).
fn word_forms(command: &str) -> Vec<String> {
    let raw = words(command);
    let assignments = raw
        .iter()
        .take_while(|word| ASSIGNMENT.is_match(&word.text))
        .count();
    let mut words = raw[assignments..]
        .iter()
        .map(|word| unquote(&word.text))
        .collect::<Vec<_>>();
    let mut forms = vec![words.join(" ")];
    while let Some(rest) = strip_exec_wrapper(&words) {
        words = rest;
        forms.push(words.join(" "));
    }
    forms
}

/// A word as the shell passes it on: quotes removed, escapes resolved; expansions are left as
/// they are written.
fn unquote(word: &str) -> String {
    let mut text = String::new();
    let mut chars = word.chars().peekable();
    let mut quote = None;
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some(open), _) if c == open => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            // `$'...'` and `$"..."`.
            (None, '$') if matches!(chars.peek(), Some(&('\'' | '"'))) => quote = chars.next(),
            (None, '\\') => text.extend(chars.next()),
            (Some('"'), '\\') if matches!(chars.peek(), Some(&('$' | '`' | '"' | '\\'))) => {
                text.extend(chars.next());
            }
            _ => text.push(c),
        }
    }
    text
}

/// What a wrapper in front of a command runs, as words: the words after `!`, `sudo`, `doas`,
/// `env`, `command`, `builtin`, `exec`, `noglob`, `nohup`, `time`, `nice`, `timeout`,
/// `stdbuf`, `watch`, `ionice`, `setsid` or `flock` and their options. `None` when the command
/// has no such wrapper in front, or nothing after it.
fn strip_exec_wrapper(words: &[String]) -> Option<Vec<String>> {
    let (first, rest) = words.split_first()?;
    let rest = match first.as_str() {
        "!" | "builtin" | "noglob" | "nohup" | "time" | "setsid" => skip_options(rest, "", &[]),
        // `command -v` and `-V` look a command up instead of running it.
        "command"
            if rest
                .first()
                .is_some_and(|word| word.starts_with("-v") || word.starts_with("-V")) =>
        {
            return None;
        }
        "command" => skip_options(rest, "", &[]),
        "exec" => skip_options(rest, "a", &[]),
        "sudo" => skip_assignments(skip_options(
            rest,
            "CDRTUacgprtu",
            &[
                "auth-type",
                "chdir",
                "chroot",
                "close-from",
                "command-timeout",
                "group",
                "host",
                "login-class",
                "other-user",
                "prompt",
                "role",
                "type",
                "user",
            ],
        )),
        "doas" => skip_options(rest, "aCu", &[]),
        "env" => skip_assignments(skip_options(rest, "uCP", &["unset", "chdir"])),
        "nice" => skip_options(rest, "n", &["adjustment"]),
        "timeout" => {
            let rest = skip_options(rest, "ks", &["kill-after", "signal"]);
            let (duration, rest) = rest.split_first()?;
            if !duration.starts_with(|c: char| c.is_ascii_digit() || c == '.') {
                return None;
            }
            rest
        }
        "stdbuf" => skip_options(rest, "ioe", &["input", "output", "error"]),
        "watch" => skip_options(rest, "nq", &["interval", "equexit"]),
        "ionice" => skip_options(rest, "cnpPu", &["class", "classdata", "pid", "pgid", "uid"]),
        "flock" => {
            let rest = skip_options(rest, "wE", &["timeout", "wait", "conflict-exit-code"]);
            // The lock file, then the command or `-c` and a command line.
            let (_, rest) = rest.split_first()?;
            match rest.first().map(String::as_str) {
                Some("-c" | "--command") => &rest[1..],
                _ => rest,
            }
        }
        _ => return None,
    };
    (!rest.is_empty()).then(|| rest.to_vec())
}

/// The words after a wrapper's options: the words starting with `-`, up to `--` or the first
/// other word, with the value of each option that takes one (`short_with_value` lists the
/// letters, `long_with_value` the names).
fn skip_options<'w>(
    words: &'w [String],
    short_with_value: &str,
    long_with_value: &[&str],
) -> &'w [String] {
    let mut index = 0;
    while let Some(word) = words.get(index) {
        if word == "--" {
            index += 1;
            break;
        }
        if let Some(long) = word.strip_prefix("--") {
            index += 1;
            if !long.contains('=') && long_with_value.contains(&long) {
                index += 1;
            }
            continue;
        }
        let Some(flags) = word.strip_prefix('-').filter(|flags| !flags.is_empty()) else {
            break;
        };
        index += 1;
        // An option that takes a value takes the rest of its word, or else the next word.
        if let Some((position, flag)) = flags
            .char_indices()
            .find(|(_, flag)| short_with_value.contains(*flag))
            && position + flag.len_utf8() == flags.len()
        {
            index += 1;
        }
    }
    words.get(index..).unwrap_or_default()
}

/// The words after the variable assignments in front.
fn skip_assignments(words: &[String]) -> &[String] {
    let assignments = words
        .iter()
        .take_while(|word| ASSIGNMENT.is_match(word))
        .count();
    &words[assignments..]
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
        let Some(split) = split_command(command) else {
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
        if split.complex {
            // Deny and ask rules meet each command, those nested in substitutions,
            // subshells and control flow too. Claude Code asks about the rest unless an
            // exact rule decides; the asking is left to Codex.
            if let Some(rule) = first_match(&self.deny, command, MatchMode::Prefix, true) {
                return ShellDecision::Deny(rule);
            }
            let decisions = split
                .subcommands
                .iter()
                .map(|subcommand| self.subcommand(subcommand))
                .collect::<Vec<_>>();
            return decisions
                .iter()
                .find(|decision| matches!(decision, ShellDecision::Deny(_)))
                .or_else(|| {
                    decisions
                        .iter()
                        .find(|decision| matches!(decision, ShellDecision::Ask(_)))
                })
                .copied()
                .unwrap_or(exact);
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
