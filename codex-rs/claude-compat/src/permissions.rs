// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's permission rules: `permissions.allow`, `ask` and `deny` of the settings
//! files, read and matched the way Claude Code reads and matches them
//! (`permissionRuleParser.ts`, `permissions.ts`, `filesystem.ts`, the tools' own checks).
//!
//! A rule is `Tool` (the whole tool; `Tool()` and `Tool(*)` too) or `Tool(content)`, with
//! `\(`, `\)` and `\\` escaped in the content. What the content means depends on the tool:
//!
//! - `Bash`: a command, exactly; a legacy prefix `cmd:*`; or a pattern with `*` wildcards,
//!   matched against each simple command of a compound one (`permissions_bash.rs`).
//! - `Read` and `Edit`: gitignore patterns for paths. `//path` is absolute, `~/path` is in the
//!   home directory, `/path` is relative to the settings file's root (`~/.claude` for the
//!   user's settings, the project directory for the project's), anything else matches
//!   anywhere under the working directory. `Read` rules also cover `Glob` and `Grep`, `Edit`
//!   rules also cover `Write`; an `Edit` allow rule allows reading too.
//! - `WebFetch`: `domain:<host>`.
//! - Anything else (`Agent(<type>)`): the content, exactly.
//!
//! An MCP tool is named `mcp__<server>__<tool>`; `mcp__<server>` and `mcp__<server>__*`
//! name every tool of the server.
//!
//! Deny wins over ask, ask over allow. A rule for the whole tool is looked at before the
//! tool's own rules for a deny or an ask, after them for an allow.

use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use ignore::Match;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use serde_json::Value;

use crate::CLAUDE_DIR;
use crate::settings::PermissionDecision;
use crate::settings::read_settings;

#[path = "permissions_bash.rs"]
pub(crate) mod bash;

use bash::ShellDecision;
use bash::ShellRules;

/// Patterns grouped by the directory they are relative to, each with the index of its
/// rule.
type PatternsByRoot = (Option<PathBuf>, Vec<(String, usize)>);

/// The tool whose rules govern every file read (`Read`, `Glob`, `Grep`).
const FILE_READ_TOOL: &str = "Read";
/// The tool whose rules govern every file edit (`Edit`, `Write`).
const FILE_EDIT_TOOL: &str = "Edit";
/// Claude Code's tool for fetching web pages.
const WEB_FETCH_TOOL: &str = "WebFetch";

/// Old tool names that rules may still use, and the tools they mean now.
const LEGACY_TOOL_NAMES: &[(&str, &str)] = &[
    ("Task", "Agent"),
    ("KillShell", "TaskStop"),
    ("AgentOutputTool", "TaskOutput"),
    ("BashOutputTool", "TaskOutput"),
];

/// Files Claude Code never lets an allow rule edit without asking.
const DANGEROUS_FILES: &[&str] = &[
    ".gitconfig",
    ".gitmodules",
    ".bashrc",
    ".bash_profile",
    ".zshrc",
    ".zprofile",
    ".profile",
    ".ripgreprc",
    ".mcp.json",
    ".claude.json",
];

/// Directories whose files Claude Code never lets an allow rule edit without asking.
const DANGEROUS_DIRECTORIES: &[&str] = &[".git", ".vscode", ".idea", ".claude"];

/// The settings file a rule comes from, which decides what `/path` is relative to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSource {
    /// `~/.claude/settings.json`: `/path` is in `~/.claude`.
    User,
    /// `.claude/settings.json` of the project: `/path` is in the project directory.
    Project,
    /// `.claude/settings.local.json` of the project: as for `Project`.
    Local,
}

/// One rule of a settings file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PermissionRule {
    /// The tool, under its current name.
    pub tool: String,
    /// What the rule says about the tool's input, unescaped; `None` for the whole tool.
    pub content: Option<String>,
    pub decision: PermissionDecision,
    pub source: RuleSource,
    /// The rule as written in the settings file.
    pub text: String,
}

/// What a tool call is matched on.
#[derive(Clone, Copy, Debug)]
pub enum ToolInput<'a> {
    /// Nothing but the tool's name (`TodoWrite`, `WebSearch`, an MCP tool).
    Tool,
    /// A `Bash` command, run in `cwd`.
    Command { command: &'a str, cwd: &'a Path },
    /// A file or directory read (`Read`, `Glob`, `Grep`): the absolute path, and the path
    /// with symbolic links resolved if it differs; relative patterns are under `cwd`. A
    /// deny or ask rule for any of the paths decides; an allow rule has to cover them all.
    Read { paths: &'a [PathBuf], cwd: &'a Path },
    /// A file written (`Edit`, `Write`), as for `Read`.
    Edit { paths: &'a [PathBuf], cwd: &'a Path },
    /// A `WebFetch` URL.
    Url(&'a str),
    /// Content matched exactly, such as the subagent type of `Agent`.
    Content(&'a str),
}

/// What the rules decide for a tool call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleVerdict {
    /// Run without asking; `rule` as written.
    Allow { rule: String },
    /// Ask the user first.
    Ask { rule: String },
    /// Refuse, giving the model Claude Code's `message`.
    Deny { rule: String, message: String },
    /// No rule speaks for this call.
    NoMatch,
}

/// The permission rules of the merged settings files.
#[derive(Clone, Debug, Default)]
pub struct PermissionRules {
    rules: Vec<PermissionRule>,
    home_dir: Option<PathBuf>,
    project_dir: Option<PathBuf>,
    /// The matchers of the `Read` and `Edit` rules, built once from `rules`.
    file_matchers: Vec<FileMatcher>,
}

/// The `Read` or `Edit` rules of one decision anchored at one place, as one gitignore
/// matcher.
#[derive(Clone, Debug)]
struct FileMatcher {
    tool: &'static str,
    decision: PermissionDecision,
    /// Where the patterns are anchored; `None` for the working directory of the call.
    root: Option<PathBuf>,
    matcher: Gitignore,
    /// Each line of the matcher, with the index of the rule it was made from.
    lines: Vec<(String, usize)>,
}

impl PermissionRules {
    /// Reads the rules of the user's `~/.claude/settings.json` and of the project's
    /// `.claude/settings.json` and `.claude/settings.local.json` in `project_dir`. Files
    /// that cannot be read are returned with their error and skipped.
    pub fn load(
        home_dir: Option<&Path>,
        project_dir: Option<&Path>,
    ) -> (Self, Vec<(PathBuf, io::Error)>) {
        let claude_home = home_dir.map(|home| home.join(CLAUDE_DIR));
        let mut files = Vec::new();
        if let Some(claude_home) = &claude_home {
            files.push((RuleSource::User, claude_home.join("settings.json")));
        }
        if let Some(project_dir) = project_dir {
            let dot_claude = project_dir.join(CLAUDE_DIR);
            // In the home directory, `.claude` is the user's own configuration.
            if claude_home.as_deref() != Some(dot_claude.as_path()) {
                files.push((RuleSource::Project, dot_claude.join("settings.json")));
                files.push((RuleSource::Local, dot_claude.join("settings.local.json")));
            }
        }
        let mut settings = Vec::new();
        let mut errors = Vec::new();
        for (source, path) in files {
            match read_settings(&path) {
                Ok(Some(value)) => settings.push((source, value)),
                Ok(None) => {}
                Err(err) => errors.push((path, err)),
            }
        }
        (Self::new(&settings, home_dir, project_dir), errors)
    }

    /// The rules of settings already read, lowest precedence first.
    pub fn new(
        settings: &[(RuleSource, Value)],
        home_dir: Option<&Path>,
        project_dir: Option<&Path>,
    ) -> Self {
        let mut rules = Vec::new();
        for (source, settings) in settings {
            let Some(permissions) = settings.get("permissions") else {
                continue;
            };
            for (key, decision) in [
                ("allow", PermissionDecision::Allow),
                ("ask", PermissionDecision::Ask),
                ("deny", PermissionDecision::Deny),
            ] {
                let Some(entries) = permissions.get(key).and_then(Value::as_array) else {
                    continue;
                };
                for text in entries.iter().filter_map(Value::as_str) {
                    let (tool, content) = parse_rule(text);
                    rules.push(PermissionRule {
                        tool,
                        content,
                        decision,
                        source: *source,
                        text: text.to_string(),
                    });
                }
            }
        }
        let mut this = Self {
            rules,
            home_dir: home_dir.map(Path::to_path_buf),
            project_dir: project_dir.map(Path::to_path_buf),
            file_matchers: Vec::new(),
        };
        this.file_matchers = this.build_file_matchers(/*warn*/ true);
        this
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn rules(&self) -> &[PermissionRule] {
        &self.rules
    }

    /// These rules without the allow rules of the project's own settings files, for a
    /// project that is not trusted: its files come with the repository, which must not
    /// be able to allow its own commands. What they deny or ask about is kept.
    pub fn without_project_allow_rules(mut self) -> Self {
        self.rules.retain(|rule| {
            rule.decision != PermissionDecision::Allow || rule.source == RuleSource::User
        });
        // What was wrong with the rules left was said when they were first built.
        self.file_matchers = self.build_file_matchers(/*warn*/ false);
        self
    }

    /// What the rules decide for a call of `tool` with `input`.
    pub fn evaluate(&self, tool: &str, input: ToolInput<'_>) -> RuleVerdict {
        let tool_name = normalize_tool_name(tool);
        let whole_tool = |decision| {
            self.rules.iter().find(|rule| {
                rule.decision == decision && rule.content.is_none() && tool_matches(rule, tool_name)
            })
        };
        if let Some(rule) = whole_tool(PermissionDecision::Deny) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("Permission to use {tool} has been denied."),
            };
        }
        if let Some(rule) = whole_tool(PermissionDecision::Ask) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        let verdict = match input {
            ToolInput::Tool => RuleVerdict::NoMatch,
            ToolInput::Command { command, cwd } => self.command(tool, tool_name, command, cwd),
            ToolInput::Read { paths, cwd } => self.read(paths, cwd),
            ToolInput::Edit { paths, cwd } => self.edit(paths, cwd),
            ToolInput::Url(url) => self.url(tool, url),
            ToolInput::Content(content) => self.content(tool, tool_name, content),
        };
        if verdict != RuleVerdict::NoMatch {
            return verdict;
        }
        // Claude Code asks about these files before it looks at a rule allowing the whole
        // tool, so such a rule does not cover them either.
        if let ToolInput::Edit { paths, .. } = &input
            && paths.iter().any(|path| is_dangerous_to_edit(path))
        {
            return RuleVerdict::NoMatch;
        }
        match whole_tool(PermissionDecision::Allow) {
            Some(rule) => RuleVerdict::Allow {
                rule: rule.text.clone(),
            },
            None => RuleVerdict::NoMatch,
        }
    }

    /// The rules of `tool` with content, for one decision, in order.
    fn with_content(
        &self,
        tool: &str,
        decision: PermissionDecision,
    ) -> impl Iterator<Item = &PermissionRule> {
        self.rules.iter().filter(move |rule| {
            rule.decision == decision && rule.tool == tool && rule.content.is_some()
        })
    }

    /// The rule whose content is `content`, exactly; later rules win, as Claude Code keeps
    /// rules in a map by their content.
    fn exact_content(
        &self,
        tool: &str,
        decision: PermissionDecision,
        content: &str,
    ) -> Option<&PermissionRule> {
        self.with_content(tool, decision)
            .filter(|rule| rule.content.as_deref() == Some(content))
            .last()
    }

    fn command(&self, tool: &str, tool_name: &str, command: &str, cwd: &Path) -> RuleVerdict {
        // Later rules with the same content replace earlier ones, as in Claude Code's map.
        let contents = |decision| {
            let mut contents: Vec<(&str, &PermissionRule)> = Vec::new();
            for rule in self.with_content(tool_name, decision) {
                let content = rule.content.as_deref().unwrap_or_default();
                match contents.iter_mut().find(|(seen, _)| *seen == content) {
                    Some(entry) => entry.1 = rule,
                    None => contents.push((content, rule)),
                }
            }
            contents
        };
        let deny = contents(PermissionDecision::Deny);
        let ask = contents(PermissionDecision::Ask);
        let allow = contents(PermissionDecision::Allow);
        let shell_rules = ShellRules {
            deny: deny.iter().map(|(content, _)| *content).collect(),
            ask: ask.iter().map(|(content, _)| *content).collect(),
            allow: allow.iter().map(|(content, _)| *content).collect(),
        };
        let text = |rules: &[(&str, &PermissionRule)], content: &str| {
            rules
                .iter()
                .find(|(seen, _)| *seen == content)
                .map(|(_, rule)| rule.text.clone())
                .unwrap_or_default()
        };
        match shell_rules.decide(command, cwd) {
            ShellDecision::Deny(content) => RuleVerdict::Deny {
                rule: text(&deny, content),
                message: format!(
                    "Permission to use {tool} with command {command} has been denied."
                ),
            },
            ShellDecision::Ask(content) => RuleVerdict::Ask {
                rule: text(&ask, content),
            },
            ShellDecision::Allow(content) => RuleVerdict::Allow {
                rule: text(&allow, content),
            },
            ShellDecision::NoMatch => RuleVerdict::NoMatch,
        }
    }

    fn read(&self, paths: &[PathBuf], cwd: &Path) -> RuleVerdict {
        let Some(path) = paths.first() else {
            return RuleVerdict::NoMatch;
        };
        let any = |tool, decision| {
            paths
                .iter()
                .find_map(|path| self.file_rule(tool, decision, path, cwd))
        };
        if let Some(rule) = any(FILE_READ_TOOL, PermissionDecision::Deny) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("Permission to read {} has been denied.", path.display()),
            };
        }
        if let Some(rule) = any(FILE_READ_TOOL, PermissionDecision::Ask) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        // What may be edited may be read, unless an edit rule says otherwise.
        if let RuleVerdict::Allow { rule } = self.edit(paths, cwd) {
            return RuleVerdict::Allow { rule };
        }
        match self.allow_rule_for_all(FILE_READ_TOOL, paths, cwd) {
            Some(rule) => RuleVerdict::Allow {
                rule: rule.text.clone(),
            },
            None => RuleVerdict::NoMatch,
        }
    }

    /// The `tool` allow rule of the first of `paths`, when every one of them has one: a
    /// path reached through a symbolic link is allowed only when the link and what it
    /// leads to both are.
    fn allow_rule_for_all(
        &self,
        tool: &str,
        paths: &[PathBuf],
        cwd: &Path,
    ) -> Option<&PermissionRule> {
        let mut first = None;
        for path in paths {
            let rule = self.file_rule(tool, PermissionDecision::Allow, path, cwd)?;
            first.get_or_insert(rule);
        }
        first
    }

    fn edit(&self, paths: &[PathBuf], cwd: &Path) -> RuleVerdict {
        let Some(path) = paths.first() else {
            return RuleVerdict::NoMatch;
        };
        let any = |decision| {
            paths
                .iter()
                .find_map(|path| self.file_rule(FILE_EDIT_TOOL, decision, path, cwd))
        };
        if let Some(rule) = any(PermissionDecision::Deny) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("Permission to edit {} has been denied.", path.display()),
            };
        }
        if let Some(rule) = any(PermissionDecision::Ask) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        // Claude Code asks before editing these whatever the allow rules say; the asking is
        // left to Codex.
        if paths.iter().any(|path| is_dangerous_to_edit(path)) {
            return RuleVerdict::NoMatch;
        }
        match self.allow_rule_for_all(FILE_EDIT_TOOL, paths, cwd) {
            Some(rule) => RuleVerdict::Allow {
                rule: rule.text.clone(),
            },
            None => RuleVerdict::NoMatch,
        }
    }

    fn url(&self, tool: &str, address: &str) -> RuleVerdict {
        if normalize_tool_name(tool) != WEB_FETCH_TOOL {
            return self.content(tool, normalize_tool_name(tool), address);
        }
        let Some(host) = url::Url::parse(address)
            .ok()
            .and_then(|url| url.host_str().map(str::to_string))
        else {
            return RuleVerdict::NoMatch;
        };
        let host = normalize_host(&host);
        let content = format!("domain:{host}");
        // The rule's host and the URL's are compared without regard to case or to one
        // trailing dot (`evil.com.` is `evil.com`); later rules win, as in `exact_content`.
        let rule_for = |decision| {
            self.with_content(WEB_FETCH_TOOL, decision)
                .filter(|rule| {
                    rule.content
                        .as_deref()
                        .and_then(|rule_content| rule_content.strip_prefix("domain:"))
                        .is_some_and(|rule_host| normalize_host(rule_host) == host)
                })
                .last()
        };
        if let Some(rule) = rule_for(PermissionDecision::Deny) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("{tool} denied access to {content}."),
            };
        }
        if let Some(rule) = rule_for(PermissionDecision::Ask) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        match rule_for(PermissionDecision::Allow) {
            Some(rule) => RuleVerdict::Allow {
                rule: rule.text.clone(),
            },
            None => RuleVerdict::NoMatch,
        }
    }

    fn content(&self, tool: &str, tool_name: &str, content: &str) -> RuleVerdict {
        if let Some(rule) = self.exact_content(tool_name, PermissionDecision::Deny, content) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("Permission to use {tool} has been denied."),
            };
        }
        if let Some(rule) = self.exact_content(tool_name, PermissionDecision::Ask, content) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        match self.exact_content(tool_name, PermissionDecision::Allow, content) {
            Some(rule) => RuleVerdict::Allow {
                rule: rule.text.clone(),
            },
            None => RuleVerdict::NoMatch,
        }
    }

    /// Where a file pattern is anchored, and the pattern relative to that place
    /// (`patternWithRoot`); `None` for the working directory. `None` overall when the
    /// place is unknown (no home directory).
    fn pattern_root(&self, pattern: &str, source: RuleSource) -> Option<(String, Option<PathBuf>)> {
        if let Some(absolute) = pattern.strip_prefix('/')
            && absolute.starts_with('/')
        {
            return Some((absolute.to_string(), Some(PathBuf::from("/"))));
        }
        if let Some(in_home) = pattern.strip_prefix('~')
            && in_home.starts_with('/')
        {
            return Some((in_home.to_string(), Some(self.home_dir.clone()?)));
        }
        if pattern.starts_with('/') {
            let root = match source {
                RuleSource::User => self.home_dir.as_ref()?.join(CLAUDE_DIR),
                RuleSource::Project | RuleSource::Local => self.project_dir.clone()?,
            };
            return Some((pattern.to_string(), Some(root)));
        }
        let relative = pattern.strip_prefix("./").unwrap_or(pattern);
        Some((relative.to_string(), None))
    }

    /// The gitignore matchers of the `Read` and `Edit` rules, one for each tool, decision
    /// and place the patterns are anchored at. A pattern that is no usable glob guards the
    /// path it spells in a deny or ask rule, and allows nothing in an allow rule, as in
    /// Claude Code; with `warn`, each such rule is logged.
    fn build_file_matchers(&self, warn: bool) -> Vec<FileMatcher> {
        let mut matchers = Vec::new();
        for tool in [FILE_READ_TOOL, FILE_EDIT_TOOL] {
            for decision in [
                PermissionDecision::Deny,
                PermissionDecision::Ask,
                PermissionDecision::Allow,
            ] {
                // Patterns by the place they are anchored at, each pattern once (the last
                // rule with it), in the order Claude Code builds its maps.
                let mut roots: Vec<PatternsByRoot> = Vec::new();
                for (index, rule) in self.rules.iter().enumerate() {
                    if rule.decision != decision || rule.tool != tool {
                        continue;
                    }
                    let Some(content) = rule.content.as_deref() else {
                        continue;
                    };
                    let Some((pattern, root)) = self.pattern_root(content, rule.source) else {
                        continue;
                    };
                    let position = match roots.iter().position(|(seen, _)| *seen == root) {
                        Some(position) => position,
                        None => {
                            roots.push((root, Vec::new()));
                            roots.len() - 1
                        }
                    };
                    let patterns = &mut roots[position].1;
                    match patterns.iter_mut().find(|(seen, _)| *seen == pattern) {
                        Some(entry) => entry.1 = index,
                        None => patterns.push((pattern, index)),
                    }
                }
                for (root, patterns) in roots {
                    let root = root.as_deref().map(normalize_path);
                    // Paths are matched relative to the root, so the matcher's own root only
                    // has to be one that no relative path starts with.
                    let mut builder =
                        GitignoreBuilder::new(root.as_deref().unwrap_or(Path::new("/")));
                    // Claude Code's `ignore` library compares case-insensitively.
                    if builder.case_insensitive(true).is_err() {
                        continue;
                    }
                    let mut lines = Vec::new();
                    for (pattern, index) in patterns {
                        let line = gitignore_line(&pattern, decision);
                        let error = match builder.add_line(None, &line) {
                            Ok(_) => {
                                lines.push((line, index));
                                continue;
                            }
                            Err(error) => error,
                        };
                        let rule = self.rules.get(index).map_or("", |rule| rule.text.as_str());
                        if decision == PermissionDecision::Allow {
                            if warn {
                                tracing::warn!(
                                    "Claude Code permission rule `{rule}` allows nothing: {error}"
                                );
                            }
                            continue;
                        }
                        if warn {
                            tracing::warn!(
                                "Claude Code permission rule `{rule}` guards only the path it \
                                 spells: {error}"
                            );
                        }
                        let literal = escape_glob(&line);
                        if builder.add_line(None, &literal).is_ok() {
                            lines.push((literal, index));
                        }
                    }
                    let Ok(matcher) = builder.build() else {
                        continue;
                    };
                    matchers.push(FileMatcher {
                        tool,
                        decision,
                        root,
                        matcher,
                        lines,
                    });
                }
            }
        }
        matchers
    }

    /// The `tool` rule (`Read` or `Edit`) of `decision` whose pattern matches `path`
    /// (`matchingRuleForInput`).
    fn file_rule(
        &self,
        tool: &str,
        decision: PermissionDecision,
        path: &Path,
        cwd: &Path,
    ) -> Option<&PermissionRule> {
        let path = normalize_path(path);
        let cwd = normalize_path(cwd);
        for matcher in self
            .file_matchers
            .iter()
            .filter(|matcher| matcher.tool == tool && matcher.decision == decision)
        {
            let root = matcher.root.as_deref().unwrap_or(&cwd);
            let Ok(relative) = path.strip_prefix(root) else {
                continue;
            };
            if relative.as_os_str().is_empty() {
                continue;
            }
            if let Match::Ignore(glob) = matcher
                .matcher
                .matched_path_or_any_parents(relative, /*is_dir*/ false)
            {
                let original = glob.original();
                return matcher
                    .lines
                    .iter()
                    // The matcher keeps a line without its trailing spaces.
                    .find(|(line, _)| line == original || line.trim_end() == original)
                    .and_then(|(_, index)| self.rules.get(*index));
            }
        }
        None
    }
}

/// Splits a rule into its tool and content (`permissionRuleValueFromString`).
pub fn parse_rule(text: &str) -> (String, Option<String>) {
    let whole = || (normalize_tool_name(text).to_string(), None);
    let Some(open) = find_unescaped(text, '(', /*last*/ false) else {
        return whole();
    };
    let Some(close) = find_unescaped(text, ')', /*last*/ true) else {
        return whole();
    };
    if close <= open || close != text.len() - 1 {
        return whole();
    }
    let tool = &text[..open];
    if tool.is_empty() {
        return whole();
    }
    let raw = &text[open + 1..close];
    let tool = normalize_tool_name(tool).to_string();
    if raw.is_empty() || raw == "*" {
        return (tool, None);
    }
    let content = raw
        .replace("\\(", "(")
        .replace("\\)", ")")
        .replace("\\\\", "\\");
    (tool, Some(content))
}

/// The byte index of the first (or last) `target` not escaped by an odd number of
/// backslashes.
fn find_unescaped(text: &str, target: char, last: bool) -> Option<usize> {
    let bytes = text.as_bytes();
    let unescaped = |index: usize| {
        bytes[..index]
            .iter()
            .rev()
            .take_while(|byte| **byte == b'\\')
            .count()
            % 2
            == 0
    };
    let mut found = text
        .char_indices()
        .filter(|(index, c)| *c == target && unescaped(*index))
        .map(|(index, _)| index);
    if last {
        found.next_back()
    } else {
        found.next()
    }
}

pub(crate) fn normalize_tool_name(name: &str) -> &str {
    LEGACY_TOOL_NAMES
        .iter()
        .find(|(legacy, _)| *legacy == name)
        .map_or(name, |(_, current)| *current)
}

/// `mcp__<server>__<tool>` as server and tool.
fn mcp_name(name: &str) -> Option<(&str, Option<&str>)> {
    let rest = name.strip_prefix("mcp__")?;
    let (server, tool) = match rest.split_once("__") {
        Some((server, tool)) => (server, Some(tool)),
        None => (rest, None),
    };
    (!server.is_empty()).then_some((server, tool))
}

/// Whether a rule for a whole tool covers `tool` (`toolMatchesRule`).
fn tool_matches(rule: &PermissionRule, tool: &str) -> bool {
    if rule.tool == tool {
        return true;
    }
    match (mcp_name(&rule.tool), mcp_name(tool)) {
        (Some((rule_server, rule_tool)), Some((server, _))) => {
            matches!(rule_tool, None | Some("*")) && rule_server == server
        }
        _ => false,
    }
}

/// The gitignore line for a file pattern of a rule of `decision`, the pattern relative to
/// its anchor. `dir/**` is `dir`, the directory and everything in it; in an allow rule a
/// single directory (`src/**`) is anchored, so that it covers `<anchor>/src` only, while
/// a deny or ask rule covers a directory of that name at any depth, as in Claude Code.
/// Nothing left (`/**`, `~/**`, `//**`) is everything under the anchor.
fn gitignore_line(pattern: &str, decision: PermissionDecision) -> String {
    let Some(directory) = pattern.strip_suffix("/**") else {
        return pattern.to_string();
    };
    if directory.is_empty() {
        return "**".to_string();
    }
    if decision == PermissionDecision::Allow && !directory.contains('/') {
        return format!("/{directory}");
    }
    directory.to_string()
}

/// `line` with every character a glob gives a meaning to escaped, so that it matches the
/// path it spells; `/` keeps its meaning in gitignore.
fn escape_glob(line: &str) -> String {
    let mut escaped = String::with_capacity(line.len());
    for c in line.chars() {
        if matches!(c, '\\' | '*' | '?' | '[' | ']' | '{' | '}') {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// A host name as `WebFetch(domain:...)` rules compare it: in lower case, without one
/// trailing dot.
fn normalize_host(host: &str) -> String {
    host.strip_suffix('.').unwrap_or(host).to_ascii_lowercase()
}

/// The path with `.` and `..` resolved, without touching the filesystem.
pub(crate) fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

/// Whether Claude Code asks before editing `path` whatever its allow rules say
/// (`isDangerousFilePathToAutoEdit`): files in `.git`, `.vscode`, `.idea` or `.claude`
/// (but `.claude/worktrees`), shell profiles and a few configuration files.
fn is_dangerous_to_edit(path: &Path) -> bool {
    let text = path.to_string_lossy();
    if text.starts_with("//") || text.starts_with("\\\\") {
        return true;
    }
    let segments = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_lowercase())
        .collect::<Vec<_>>();
    for (index, segment) in segments.iter().enumerate() {
        if !DANGEROUS_DIRECTORIES.contains(&segment.as_str()) {
            continue;
        }
        if segment.as_str() == ".claude"
            && segments.get(index + 1).map(String::as_str) == Some("worktrees")
        {
            continue;
        }
        return true;
    }
    segments
        .last()
        .is_some_and(|name| DANGEROUS_FILES.contains(&name.as_str()))
}

#[cfg(test)]
#[path = "permissions_tests.rs"]
mod tests;
