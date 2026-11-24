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
    ".zshenv",
    ".zlogin",
    ".zlogout",
    ".bash_login",
    ".bash_aliases",
    ".bash_logout",
    ".envrc",
    ".npmrc",
    ".yarnrc",
    ".yarnrc.yml",
    ".pnp.cjs",
    ".pnp.loader.mjs",
    ".pnp.js",
    ".pnp.data.json",
    ".pnpmfile.cjs",
    "bunfig.toml",
    ".bunfig.toml",
    ".bazelrc",
    ".bazelversion",
    ".bazeliskrc",
    ".pre-commit-config.yaml",
    "lefthook.yml",
    ".lefthook.yml",
    "lefthook.yaml",
    ".lefthook.yaml",
    "gradle-wrapper.properties",
    "maven-wrapper.properties",
    ".devcontainer.json",
    "pyrightconfig.json",
];

/// Directories whose files Claude Code never lets an allow rule edit without asking.
const DANGEROUS_DIRECTORIES: &[&str] = &[
    ".git",
    ".vscode",
    ".idea",
    ".claude",
    ".husky",
    ".cargo",
    ".devcontainer",
    ".yarn",
    ".mvn",
];

/// The settings file a rule comes from, which decides what `/path` is relative to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleSource {
    /// `~/.claude/settings.json`: `/path` is in `~/.claude`.
    User,
    /// `.claude/settings.json` of the project: `/path` is in the project directory.
    Project,
    /// A `.claude/settings.local.json` of the project, in `project_dir` or at the root of its
    /// repository: as for `Project`, `/path` is in the project directory.
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
    /// A file or directory read (`Read`, `Glob`, `Grep`): the absolute path, and the paths
    /// its symbolic links lead to where they differ; relative patterns are under `cwd`. A
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
    claude_home: Option<PathBuf>,
    project_dir: Option<PathBuf>,
    /// The matchers of the `Read` and `Edit` rules, built once from `rules`.
    file_matchers: Vec<FileMatcher>,
    deny_instead_of_prompting: bool,
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
    /// Reads the rules of the user's `~/.claude/settings.json`, of the project's
    /// `.claude/settings.json` in `project_dir` and of its `.claude/settings.local.json`
    /// files, the one at the repository's root included (see
    /// [`crate::local_settings_paths`]). Files that cannot be read are returned with their
    /// error and skipped.
    pub fn load(
        claude_home: Option<&Path>,
        home_dir: Option<&Path>,
        project_dir: Option<&Path>,
    ) -> (Self, Vec<(PathBuf, io::Error)>) {
        let mut files = Vec::new();
        if let Some(claude_home) = claude_home {
            files.push((RuleSource::User, claude_home.join("settings.json")));
        }
        if let Some(project_dir) = project_dir.filter(|dir| Some(*dir) != home_dir) {
            let dot_claude = project_dir.join(CLAUDE_DIR);
            // In the home directory, `.claude` is the user's own configuration.
            if claude_home != Some(dot_claude.as_path()) {
                files.push((RuleSource::Project, dot_claude.join("settings.json")));
                files.extend(
                    crate::settings::local_settings_paths(project_dir, home_dir)
                        .into_iter()
                        .map(|path| (RuleSource::Local, path)),
                );
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
        (
            Self::new(&settings, claude_home, home_dir, project_dir),
            errors,
        )
    }

    /// The rules of settings already read, lowest precedence first.
    pub fn new(
        settings: &[(RuleSource, Value)],
        claude_home: Option<&Path>,
        home_dir: Option<&Path>,
        project_dir: Option<&Path>,
    ) -> Self {
        let mut rules = Vec::new();
        let mut deny_instead_of_prompting = false;
        for (source, settings) in settings {
            let Some(permissions) = settings.get("permissions") else {
                continue;
            };
            if *source == RuleSource::User
                && let Some(value) = permissions
                    .get("denyInsteadOfPrompting")
                    .and_then(Value::as_bool)
            {
                deny_instead_of_prompting = value;
            }
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
            claude_home: claude_home.map(Path::to_path_buf),
            project_dir: project_dir.map(Path::to_path_buf),
            file_matchers: Vec::new(),
            deny_instead_of_prompting,
        };
        this.file_matchers = this.build_file_matchers(/*warn*/ true);
        this
    }

    pub fn deny_instead_of_prompting(&self) -> bool {
        self.deny_instead_of_prompting
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

    /// These rules with each `Read` and `Edit` deny or ask rule anchored at a fixed place
    /// (`//tmp/x/**`, `~/.ssh/**`, `/secrets/**`) also matched where that place really is
    /// when symbolic links lead it elsewhere (`//private/tmp/x/**` on macOS), as newer
    /// versions of Claude Code apply such a rule to a path given by its real location. The
    /// place is the part of the pattern before its first wildcard; `resolve` gives the real
    /// path of an existing path, and a place not there yet is resolved through its nearest
    /// existing ancestor. Allow rules are matched as written only.
    pub fn with_resolved_rule_places(mut self, resolve: impl Fn(&Path) -> Option<PathBuf>) -> Self {
        let mut added = Vec::new();
        for rule in &self.rules {
            if !matches!(
                rule.decision,
                PermissionDecision::Deny | PermissionDecision::Ask
            ) || !(rule.tool == FILE_READ_TOOL || rule.tool == FILE_EDIT_TOOL)
            {
                continue;
            }
            let Some(content) = rule.content.as_deref() else {
                continue;
            };
            let Some((pattern, Some(root))) = self.pattern_root(content, rule.source) else {
                continue;
            };
            let components: Vec<&str> = pattern.trim_start_matches('/').split('/').collect();
            let literal = components
                .iter()
                .take_while(|component| !component.is_empty() && is_literal(component))
                .count();
            if literal == 0 {
                continue;
            }
            let place = normalize_path(
                &components[..literal]
                    .iter()
                    .fold(root, |place, component| place.join(component)),
            );
            let Some(real) = resolve_through_ancestors(&place, &resolve) else {
                continue;
            };
            let real = normalize_path(&real);
            if real == place || !real.is_absolute() {
                continue;
            }
            let mut resolved = format!("/{}", escape_glob(&real.to_string_lossy()));
            for component in &components[literal..] {
                resolved.push('/');
                resolved.push_str(component);
            }
            added.push(PermissionRule {
                tool: rule.tool.clone(),
                content: Some(resolved),
                decision: rule.decision,
                source: rule.source,
                text: rule.text.clone(),
            });
        }
        if added.is_empty() {
            return self;
        }
        self.rules.extend(added);
        // What was wrong with the rules was said when they were first built.
        self.file_matchers = self.build_file_matchers(/*warn*/ false);
        self
    }

    /// These rules with `texts` added as allow rules granted for a while, such as the
    /// `allowed-tools` of a skill for the turn that invokes it. Their `/path` patterns are
    /// relative to the project directory, and deny and ask rules still win over them.
    pub fn with_granted_allow_rules(mut self, texts: &[String]) -> Self {
        for text in texts {
            let (tool, content) = parse_rule(text);
            self.rules.push(PermissionRule {
                tool,
                content,
                decision: PermissionDecision::Allow,
                source: RuleSource::Local,
                text: text.clone(),
            });
        }
        self.file_matchers = self.build_file_matchers(/*warn*/ false);
        self
    }

    /// What the rules decide for a call of `tool` with `input`.
    pub fn evaluate(&self, tool: &str, input: ToolInput<'_>) -> RuleVerdict {
        self.evaluate_with_arguments(tool, input, /*arguments*/ None)
    }

    /// Also matches explicit scalar parameters in a built-in tool's raw JSON arguments.
    pub fn evaluate_with_arguments(
        &self,
        tool: &str,
        input: ToolInput<'_>,
        arguments: Option<&str>,
    ) -> RuleVerdict {
        let tool_name = normalize_tool_name(tool);
        let whole_tool = |decision| {
            self.rules.iter().find(|rule| {
                rule.decision == decision && rule.content.is_none() && tool_matches(rule, tool_name)
            })
        };
        let (arg_deny, arg_ask) = self.parameter_rules(tool_name, input, arguments);
        if let Some(rule) = whole_tool(PermissionDecision::Deny).or(arg_deny) {
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
        if let Some(rule) = arg_ask
            && !matches!(verdict, RuleVerdict::Deny { .. } | RuleVerdict::Ask { .. })
        {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        if verdict != RuleVerdict::NoMatch {
            return verdict;
        }
        // Claude Code asks about these files before it looks at a rule allowing the whole
        // tool, so such a rule does not cover them either.
        if let ToolInput::Edit { paths, .. } = &input
            && paths.iter().any(|path| self.asks_before_editing(path))
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

    fn parameter_rules<'a>(
        &'a self,
        tool: &str,
        input: ToolInput<'_>,
        arguments: Option<&str>,
    ) -> (Option<&'a PermissionRule>, Option<&'a PermissionRule>) {
        let Some(arguments) = arguments.filter(|_| !tool.starts_with("mcp__")) else {
            return (None, None);
        };
        // The tool strings are already parsed by their handlers. Only parse the raw
        // arguments again when a parameter rule could apply to the call.
        let decoded = std::cell::OnceCell::new();
        let mut ask = None;
        for rule in &self.rules {
            if rule.decision == PermissionDecision::Allow || !tool_matches(rule, tool) {
                continue;
            }
            let Some((field, pattern)) = rule
                .content
                .as_deref()
                .and_then(|text| text.split_once(':'))
            else {
                continue;
            };
            let field = field.trim();
            if field.is_empty() || is_primary_parameter(tool, field) {
                continue;
            }
            // For the shell tools `name:*` is also the prefix rule `name *`, and Claude Code 2.1.331
            // applies both readings. `timeout` is the one field that is a program too, and models set
            // it on ordinary calls, so `Bash(timeout:*)` stays about the program alone.
            if matches!(tool, "Bash" | "PowerShell") && field == "timeout" && pattern.trim() == "*" {
                continue;
            }
            let Some(fact) = decoded
                .get_or_init(|| serde_json::from_str::<Value>(arguments).ok())
                .as_ref()
                .and_then(|value| value.get(field))
            else {
                continue;
            };
            let pattern = pattern.trim();
            let matched = match fact {
                Value::String(text) if tool == crate::tools::SKILL && field == "skill" => {
                    parameter_value_matches(pattern, text)
                        || matches!(input, ToolInput::Content(name) if parameter_value_matches(pattern, name))
                }
                Value::String(text) => parameter_value_matches(pattern, text),
                Value::Bool(_) | Value::Number(_) => {
                    parameter_value_matches(pattern, &fact.to_string())
                }
                _ => false,
            };
            if matched {
                if rule.decision == PermissionDecision::Deny {
                    return (Some(rule), ask);
                }
                ask.get_or_insert(rule);
            }
        }
        (None, ask)
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

    /// Later matching rules win. Skill permissions also allow a name prefix.
    fn matching_content(
        &self,
        tool: &str,
        decision: PermissionDecision,
        content: &str,
    ) -> Option<&PermissionRule> {
        self.with_content(tool, decision)
            .filter(|rule| {
                rule.content.as_deref().is_some_and(|pattern| {
                    pattern == content
                        || tool == crate::tools::SKILL && skill_prefix_matches(pattern, content)
                })
            })
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

    /// The patterns of the `Read` deny rules as ripgrep globs for a search of `search_dir`,
    /// relative to it (`getFileReadIgnorePatterns`, `normalizePatternsToPath`), so that
    /// `Grep` and `Glob` leave out the files those rules cover, as in Claude Code. A glob
    /// that starts with `/` is anchored at `search_dir`; one that starts with `**/`
    /// matches at any depth. Patterns that cannot reach into `search_dir`, and negations,
    /// are left out. Relative patterns are under `cwd`.
    pub fn read_deny_globs(&self, search_dir: &Path, cwd: &Path) -> Vec<String> {
        let search_dir = normalize_path(search_dir);
        let cwd = normalize_path(cwd);
        let mut globs = Vec::new();
        for matcher in self.file_matchers.iter().filter(|matcher| {
            matcher.tool == FILE_READ_TOOL && matcher.decision == PermissionDecision::Deny
        }) {
            let root = matcher.root.as_deref().unwrap_or(&cwd);
            for (line, _) in &matcher.lines {
                if line.starts_with('!') {
                    continue;
                }
                if let Some(glob) = search_glob(root, line, &search_dir)
                    && !globs.contains(&glob)
                {
                    globs.push(glob);
                }
            }
        }
        globs
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

    /// Whether no allow rule covers editing `path`: a file `is_dangerous_to_edit` names,
    /// or one in the user's Claude directory when `CLAUDE_CONFIG_DIR` puts it somewhere
    /// not named `.claude`, where the settings, hooks and rules it holds would otherwise
    /// be open to a broad `Edit` rule.
    fn asks_before_editing(&self, path: &Path) -> bool {
        is_dangerous_to_edit(path)
            || self.claude_home.as_deref().is_some_and(|claude_home| {
                path.starts_with(claude_home) && !path.starts_with(claude_home.join("worktrees"))
            })
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
        if paths.iter().any(|path| self.asks_before_editing(path)) {
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
        // trailing dot (`evil.com.` is `evil.com`); later rules win within each decision.
        let rule_for = |decision| {
            self.with_content(WEB_FETCH_TOOL, decision)
                .filter(|rule| {
                    rule.content
                        .as_deref()
                        .and_then(|rule_content| rule_content.strip_prefix("domain:"))
                        .is_some_and(|rule_host| {
                            web_fetch_host_matches(&normalize_host(rule_host), &host)
                        })
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
        if let Some(rule) = self.matching_content(tool_name, PermissionDecision::Deny, content) {
            return RuleVerdict::Deny {
                rule: rule.text.clone(),
                message: format!("Permission to use {tool} has been denied."),
            };
        }
        if let Some(rule) = self.matching_content(tool_name, PermissionDecision::Ask, content) {
            return RuleVerdict::Ask {
                rule: rule.text.clone(),
            };
        }
        match self.matching_content(tool_name, PermissionDecision::Allow, content) {
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
                RuleSource::User => self.claude_home.clone()?,
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

/// Diagnostics for permission rules in one settings file, in configuration order.
pub fn permission_rule_warnings(settings: &Value) -> Vec<String> {
    let Some(permissions) = settings.get("permissions") else {
        return Vec::new();
    };
    ["allow", "ask", "deny"]
        .into_iter()
        .flat_map(|decision| {
            permissions
                .get(decision)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .enumerate()
                .filter_map(move |(index, rule)| {
                    let rule = rule.as_str()?;
                    permission_rule_warning(rule, decision == "allow")
                        .map(|message| format!("permissions.{decision}[{index}] {message}"))
                })
        })
        .collect()
}

fn permission_rule_warning(rule: &str, is_allow: bool) -> Option<&'static str> {
    if let Some(open) = find_unescaped(rule, '(', /*last*/ false)
        && find_unescaped(rule, ')', /*last*/ true)
            .is_none_or(|close| close <= open || !rule[close + 1..].is_empty())
    {
        return Some("must end at its closing parenthesis");
    }

    let (tool, content) = parse_rule(rule);
    let content = content.as_deref()?;
    // `Bash(command:*)` is the older spelling of the prefix rule `Bash(command *)`.
    if let Some((field, value)) = content.split_once(':')
        && is_primary_parameter(&tool, field.trim())
        && !(matches!(tool.as_str(), "Bash" | "PowerShell") && value.trim() == "*")
    {
        return Some(
            "must use the tool's command, path or domain rule syntax for its primary input",
        );
    }
    // A colon can also introduce a scalar parameter rule such as Write(content:example).
    let is_path_selector = !content.split_once(':').is_some_and(|(field, _)| {
        !is_allow && is_supported_secondary_parameter(&tool, field.trim())
    });
    if is_path_selector {
        match tool.as_str() {
            "Write" | "NotebookEdit" | "MultiEdit" => return Some("should use Edit(path)"),
            "Grep" | "Glob" => return Some("should use Read(path)"),
            _ => {}
        }
    }
    if is_allow
        && tool == "Bash"
        && let bash::ShellRule::Wildcard(pattern) = bash::parse_shell_rule(content)
    {
        let mut words = pattern.split_ascii_whitespace();
        let program = words.next()?;
        let first_arg = words.next();
        if program == "*" || first_arg == Some("*") && words.next().is_some() {
            return Some(
                "has a wildcard before the subcommand. Put the wildcard after the subcommand",
            );
        }
    }
    None
}

fn is_supported_secondary_parameter(tool: &str, field: &str) -> bool {
    matches!(
        (tool, field),
        ("Write", "content")
            | (
                "NotebookEdit",
                "cell_id" | "new_source" | "cell_type" | "edit_mode"
            )
            | ("Glob", "pattern")
            | (
                "Grep",
                "pattern"
                    | "glob"
                    | "type"
                    | "output_mode"
                    | "offset"
                    | "head_limit"
                    | "-A"
                    | "-B"
                    | "-C"
                    | "-i"
                    | "-n"
                    | "context"
                    | "multiline"
            )
    )
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
    if rule.decision != PermissionDecision::Allow
        && rule.tool.contains('*')
        && bash::matches_wildcard(&rule.tool, tool)
    {
        return true;
    }
    match (mcp_name(&rule.tool), mcp_name(tool)) {
        (Some((rule_server, rule_tool)), Some((server, _))) => {
            matches!(rule_tool, None | Some("*")) && rule_server == server
        }
        _ => false,
    }
}

fn is_primary_parameter(tool: &str, parameter: &str) -> bool {
    matches!(
        (tool, parameter),
        ("Bash" | "PowerShell", "command")
            | ("Read" | "Edit" | "Write", "file_path")
            | ("Glob" | "Grep", "path")
            | ("NotebookEdit", "notebook_path")
            | ("WebFetch", "url")
    )
}

fn parameter_value_matches(pattern: &str, value: &str) -> bool {
    let Some((prefix, rest)) = pattern.split_once('*') else {
        return pattern == value;
    };
    let Some(value) = value.strip_prefix(prefix) else {
        return false;
    };
    let Some((middle, suffix)) = rest.rsplit_once('*') else {
        return value.ends_with(rest);
    };
    let Some(mut value) = value.strip_suffix(suffix) else {
        return false;
    };
    for part in middle.split('*') {
        let Some((_, tail)) = value.split_once(part) else {
            return false;
        };
        value = tail;
    }
    true
}

/// Whether `pattern`, the content of a `Read(...)`, `Edit(...)` or `Write(...)` rule as a
/// hook's `if` writes it, covers `path` (relative paths are under `cwd`). It is matched as
/// an allow rule of the project's settings is: gitignore patterns without regard to case,
/// `//path` absolute, `~/path` in `home_dir`, `/path` and anything else under `cwd`, and a
/// single directory (`src/**`) only at the top. A pattern that is no usable glob covers
/// every path, so that a guard does not silently stop firing.
pub(crate) fn file_pattern_matches(
    pattern: &str,
    path: &Path,
    cwd: &Path,
    home_dir: Option<&Path>,
) -> bool {
    let anchors = PermissionRules {
        home_dir: home_dir.map(Path::to_path_buf),
        project_dir: Some(cwd.to_path_buf()),
        ..PermissionRules::default()
    };
    let Some((pattern, root)) = anchors.pattern_root(pattern, RuleSource::Project) else {
        return false;
    };
    let root = normalize_path(root.as_deref().unwrap_or(cwd));
    let path = normalize_path(&cwd.join(path));
    let Ok(relative) = path.strip_prefix(&root) else {
        return false;
    };
    if relative.as_os_str().is_empty() {
        return false;
    }
    let mut builder = GitignoreBuilder::new(&root);
    if builder.case_insensitive(true).is_err() {
        return true;
    }
    let line = gitignore_line(&pattern, PermissionDecision::Allow);
    if builder.add_line(None, &line).is_err() {
        return true;
    }
    let Ok(matcher) = builder.build() else {
        return true;
    };
    matcher
        .matched_path_or_any_parents(relative, /*is_dir*/ false)
        .is_ignore()
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

/// The gitignore `line` of a pattern anchored at `root` as a ripgrep glob relative to
/// `search_dir`: `/<path>` from `search_dir`, or `**/<name>` at any depth; `None` when
/// the pattern names nothing in `search_dir`. A directory of `search_dir` that a
/// wildcard of the pattern stands for is taken as matching it.
fn search_glob(root: &Path, line: &str, search_dir: &Path) -> Option<String> {
    let body = line.strip_prefix('/').unwrap_or(line);
    // As in gitignore, a line with a slash only at its end matches at any depth.
    let anchored = line.starts_with('/') || line.trim_end_matches('/').contains('/');
    if let Ok(rest) = root.strip_prefix(search_dir) {
        // The pattern's root is `search_dir` or a directory in it.
        let prefix = slash_path(rest);
        return Some(match (anchored, prefix.is_empty()) {
            (false, true) => format!("**/{body}"),
            (false, false) => format!("/{prefix}/**/{body}"),
            (true, true) => format!("/{body}"),
            (true, false) => format!("/{prefix}/{body}"),
        });
    }
    // `search_dir` is inside the pattern's root: the directories between them have to be
    // the pattern's first components.
    let rest = search_dir.strip_prefix(root).ok()?;
    if !anchored {
        return Some(format!("**/{body}"));
    }
    let components = body.split('/').collect::<Vec<_>>();
    let mut index = 0;
    for directory in rest.components() {
        let directory = directory.as_os_str().to_string_lossy();
        match components.get(index) {
            // The pattern covers a directory `search_dir` is in: all of it.
            None => return Some("**".to_string()),
            Some(&"**") => break,
            Some(component) if is_literal(component) => {
                if !component.eq_ignore_ascii_case(&directory) {
                    return None;
                }
                index += 1;
            }
            Some(_) => index += 1,
        }
    }
    let remaining = components[index..].join("/");
    Some(if remaining.is_empty() {
        "**".to_string()
    } else {
        format!("/{remaining}")
    })
}

/// Most symbolic links followed from one path, as Claude Code follows them
/// (`SYMLOOP_MAX`).
pub const MAX_LINK_DEPTH: usize = 40;

/// Where the symbolic link at `link` leads, given the `target` it holds: a relative target
/// is under the link's directory, and `.` and `..` are taken by name, as Claude Code takes
/// them.
pub fn link_target(link: &Path, target: &str) -> PathBuf {
    normalize_path(&link.parent().unwrap_or(link).join(target))
}

/// `place` resolved by `resolve`, or, when it does not exist, its nearest ancestor that
/// does resolved and the rest of it appended.
fn resolve_through_ancestors(
    place: &Path,
    resolve: &impl Fn(&Path) -> Option<PathBuf>,
) -> Option<PathBuf> {
    for ancestor in place.ancestors() {
        if let Some(real) = resolve(ancestor) {
            let rest = place.strip_prefix(ancestor).ok()?;
            return Some(if rest.as_os_str().is_empty() {
                real
            } else {
                real.join(rest)
            });
        }
    }
    None
}

/// Whether a component of a glob has no wildcard in it.
fn is_literal(component: &str) -> bool {
    !component.contains(['*', '?', '[', '{', '\\'])
}

/// The components of a relative path joined with `/`.
fn slash_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
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

/// The reserved claude.ai namespaces do not cover a local skill with that prefix.
fn skill_prefix_matches(pattern: &str, name: &str) -> bool {
    let Some(prefix) = pattern
        .strip_suffix(":*")
        .or_else(|| pattern.strip_suffix(" *"))
    else {
        return false;
    };
    !prefix.is_empty()
        && !matches!(prefix, "anthropic-skills" | "claude-ai")
        && (name == prefix
            || name
                .strip_prefix(prefix)
                .is_some_and(|tail| tail.starts_with(':')))
}

/// A leading wildcard can span multiple subdomain labels. Every other wildcard
/// is restricted to one label.
fn web_fetch_host_matches(pattern: &str, host: &str) -> bool {
    if pattern == "*" || pattern == host {
        return true;
    }
    let suffix = if let Some(suffix) = pattern.strip_prefix("*.") {
        suffix
    } else {
        return host_labels_match(pattern, host);
    };
    if !suffix.contains('*') {
        return host
            .strip_suffix(suffix)
            .is_some_and(|prefix| prefix.ends_with('.') && prefix.len() > 1);
    }
    let Some((_, rest)) = host.split_once('.') else {
        return false;
    };
    std::iter::once(rest)
        .chain(rest.match_indices('.').map(|(index, _)| &rest[index + 1..]))
        .any(|candidate| host_labels_match(suffix, candidate))
}

fn host_labels_match(pattern: &str, host: &str) -> bool {
    let mut labels = host.split('.');
    pattern.split('.').all(|label| {
        labels.next().is_some_and(|host_label| {
            !host_label.is_empty()
                && (label == host_label
                    || label.contains('*') && bash::matches_wildcard(label, host_label))
        })
    }) && labels.next().is_none()
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
        .windows(2)
        .any(|pair| pair[0] == ".config" && pair[1] == "git")
        || segments
            .last()
            .is_some_and(|name| DANGEROUS_FILES.contains(&name.as_str()))
}

#[cfg(test)]
#[path = "permissions_tests.rs"]
mod tests;
