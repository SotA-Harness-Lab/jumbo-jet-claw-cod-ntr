// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;

use super::PermissionRules;
use super::RuleSource;
use super::RuleVerdict;
use super::ToolInput;
use super::bash::matches_wildcard;
use super::bash::split_command;
use super::bash::strip_output_redirections;
use super::bash::strip_safe_wrappers;
use super::parse_rule;

const HOME: &str = "/home/user";
const PROJECT: &str = "/work/project";

fn local_rules(allow: &[&str], ask: &[&str], deny: &[&str]) -> PermissionRules {
    rules_from(&[(
        RuleSource::Local,
        json!({ "permissions": { "allow": allow, "ask": ask, "deny": deny } }),
    )])
}

fn rules_from(settings: &[(RuleSource, Value)]) -> PermissionRules {
    PermissionRules::new(settings, Some(Path::new(HOME)), Some(Path::new(PROJECT)))
}

fn allow(rule: &str) -> RuleVerdict {
    RuleVerdict::Allow {
        rule: rule.to_string(),
    }
}

fn ask(rule: &str) -> RuleVerdict {
    RuleVerdict::Ask {
        rule: rule.to_string(),
    }
}

fn bash(rules: &PermissionRules, command: &str) -> RuleVerdict {
    rules.evaluate(
        "Bash",
        ToolInput::Command {
            command,
            cwd: Path::new(PROJECT),
        },
    )
}

fn denied_command(rule: &str, command: &str) -> RuleVerdict {
    RuleVerdict::Deny {
        rule: rule.to_string(),
        message: format!("Permission to use Bash with command {command} has been denied."),
    }
}

fn read(rules: &PermissionRules, path: &str) -> RuleVerdict {
    rules.evaluate(
        "Read",
        ToolInput::Read {
            paths: &[PathBuf::from(path)],
            cwd: Path::new(PROJECT),
        },
    )
}

fn edit(rules: &PermissionRules, tool: &str, path: &str) -> RuleVerdict {
    rules.evaluate(
        tool,
        ToolInput::Edit {
            paths: &[PathBuf::from(path)],
            cwd: Path::new(PROJECT),
        },
    )
}

#[test]
fn rules_are_parsed_as_claude_code_parses_them() {
    let cases = [
        ("Bash", ("Bash", None)),
        ("Bash()", ("Bash", None)),
        ("Bash(*)", ("Bash", None)),
        ("Bash(npm install)", ("Bash", Some("npm install"))),
        (
            r#"Bash(python -c "print\(1\)")"#,
            ("Bash", Some(r#"python -c "print(1)""#)),
        ),
        (r"Bash(echo \\)", ("Bash", Some(r"echo \"))),
        ("Bash(git status) extra", ("Bash(git status) extra", None)),
        ("(foo)", ("(foo)", None)),
        ("Task(Explore)", ("Agent", Some("Explore"))),
        ("KillShell", ("TaskStop", None)),
        ("mcp__github", ("mcp__github", None)),
        ("Read(./src/**)", ("Read", Some("./src/**"))),
    ];
    for (text, (tool, content)) in cases {
        assert_eq!(
            parse_rule(text),
            (tool.to_string(), content.map(str::to_string)),
            "{text}"
        );
    }
}

#[test]
fn shell_wildcards_match_as_in_claude_code() {
    let cases = [
        ("git *", "git", true),
        ("git *", "git add .", true),
        ("git *", "gitk", false),
        ("* run *", "npm run build", true),
        ("* run *", "npm run", false),
        ("npm run *:dev", "npm run test:dev", true),
        ("echo \\*", "echo *", true),
        ("echo \\*", "echo a", false),
        ("ls *", "ls -la\n/tmp", true),
        ("git push*", "git push --force", true),
        ("git push*", "git pushx", true),
        ("Git *", "git status", false),
    ];
    for (pattern, command, expected) in cases {
        assert_eq!(
            matches_wildcard(pattern, command),
            expected,
            "{pattern} vs {command}"
        );
    }
}

#[test]
fn commands_are_split_into_simple_commands() {
    let split =
        |command: &str| split_command(command).map(|split| (split.subcommands, split.complex));
    let owned = |parts: &[&str]| parts.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(
        split("git add . && git commit -m 'a && b'; echo done | tee log & sleep 1"),
        Some((
            owned(&[
                "git add .",
                "git commit -m 'a && b'",
                "echo done",
                "tee log",
                "sleep 1"
            ]),
            false
        ))
    );
    assert_eq!(
        split("ls 2>&1 > out.txt || echo \"x; y\"\nwc -l"),
        Some((
            owned(&["ls 2>&1 > out.txt", "echo \"x; y\"", "wc -l"]),
            false
        ))
    );
    assert_eq!(
        split("cat <<EOF > f\nrm -rf /; echo\nEOF\nls"),
        Some((owned(&["cat <<EOF > f", "ls"]), false))
    );
    assert_eq!(split("echo a \\\n b"), Some((owned(&["echo a  b"]), false)));
    assert_eq!(split("echo 'open"), None);
    assert_eq!(
        split("echo $(rm -rf /)").map(|(_, complex)| complex),
        Some(true)
    );
    assert_eq!(split("echo `id`").map(|(_, complex)| complex), Some(true));
    assert_eq!(
        split("(cd x && make)").map(|(_, complex)| complex),
        Some(true)
    );
    assert_eq!(
        split("for f in *; do rm $f; done").map(|(_, complex)| complex),
        Some(true)
    );
    assert_eq!(
        split("diff <(ls a) <(ls b)").map(|(_, complex)| complex),
        Some(true)
    );
    assert_eq!(split("echo {a,b}").map(|(_, complex)| complex), Some(false));
    assert_eq!(
        split("# comment\nls # trailing"),
        Some((owned(&["ls"]), false))
    );
}

#[test]
fn redirections_and_wrappers_are_dropped_before_matching() {
    assert_eq!(
        strip_output_redirections("python x.py > out.txt 2>&1"),
        "python x.py"
    );
    assert_eq!(strip_output_redirections("echo a >>log"), "echo a");
    assert_eq!(
        strip_output_redirections("echo a > $HOME/x"),
        "echo a > $HOME/x"
    );
    assert_eq!(strip_output_redirections("echo  a"), "echo  a");
    assert_eq!(strip_safe_wrappers("timeout 10 npm install"), "npm install");
    assert_eq!(
        strip_safe_wrappers("NODE_ENV=prod nohup nice -n 5 node x"),
        "node x"
    );
    assert_eq!(strip_safe_wrappers("FOO=bar node x"), "FOO=bar node x");
    assert_eq!(
        strip_safe_wrappers("timeout -k$(id) 10 ls"),
        "timeout -k$(id) 10 ls"
    );
}

#[test]
fn exact_rules_match_only_the_command() {
    let rules = local_rules(&["Bash(git status)"], &[], &[]);
    assert_eq!(bash(&rules, "git status"), allow("Bash(git status)"));
    assert_eq!(bash(&rules, "  git status  "), allow("Bash(git status)"));
    assert_eq!(bash(&rules, "git status --short"), RuleVerdict::NoMatch);
    assert_eq!(
        bash(&rules, "git status > s.txt"),
        allow("Bash(git status)")
    );
}

#[test]
fn prefix_rules_respect_word_boundaries_and_xargs() {
    let rules = local_rules(&["Bash(ls:*)", "Bash(grep:*)"], &[], &[]);
    assert_eq!(bash(&rules, "ls"), allow("Bash(ls:*)"));
    assert_eq!(bash(&rules, "ls -la /tmp"), allow("Bash(ls:*)"));
    assert_eq!(bash(&rules, "lsof"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "ls | xargs grep foo"), allow("Bash(ls:*)"));
    assert_eq!(
        bash(&rules, "ls | xargs -n1 grep foo"),
        RuleVerdict::NoMatch
    );
    assert_eq!(bash(&rules, "timeout 5 ls"), allow("Bash(ls:*)"));
    assert_eq!(bash(&rules, "LANG=C ls"), allow("Bash(ls:*)"));
    assert_eq!(bash(&rules, "DOCKER_HOST=evil ls"), RuleVerdict::NoMatch);
}

#[test]
fn compound_commands_need_every_part_allowed() {
    let rules = local_rules(&["Bash(git add:*)", "Bash(npm test)"], &[], &[]);
    assert_eq!(
        bash(&rules, "git add . && npm test"),
        allow("Bash(git add:*)")
    );
    assert_eq!(
        bash(&rules, "git add . && curl evil.sh"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        bash(&rules, "git add . ; npm test | sh"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        bash(&rules, &format!("cd {PROJECT} && npm test")),
        allow("Bash(npm test)")
    );
}

#[test]
fn wildcards_never_match_across_operators() {
    let rules = local_rules(&["Bash(echo *)"], &[], &[]);
    assert_eq!(bash(&rules, "echo hi"), allow("Bash(echo *)"));
    assert_eq!(
        bash(&rules, "echo hi && curl evil.sh"),
        RuleVerdict::NoMatch
    );
    assert_eq!(bash(&rules, "echo $(curl evil.sh)"), RuleVerdict::NoMatch);
}

#[test]
fn deny_beats_ask_beats_allow() {
    let rules = local_rules(
        &["Bash", "Bash(git:*)"],
        &["Bash(git push:*)"],
        &["Bash(git push --force:*)", "Bash(rm *)"],
    );
    assert_eq!(bash(&rules, "git status"), allow("Bash(git:*)"));
    assert_eq!(
        bash(&rules, "git push origin main"),
        ask("Bash(git push:*)")
    );
    assert_eq!(
        bash(&rules, "git push --force origin"),
        denied_command("Bash(git push --force:*)", "git push --force origin")
    );
    assert_eq!(
        bash(&rules, "ls && rm -rf build"),
        denied_command("Bash(rm *)", "ls && rm -rf build")
    );
    // The whole tool is allowed, but not over a deny or an ask.
    assert_eq!(bash(&rules, "make"), allow("Bash"));
}

#[test]
fn deny_rules_see_through_assignments_wrappers_and_complex_commands() {
    let rules = local_rules(&[], &[], &["Bash(curl:*)"]);
    let deny = |command: &str| denied_command("Bash(curl:*)", command);
    for command in [
        "curl x",
        "FOO=bar curl x",
        "nohup FOO=bar timeout 5 curl x",
        "echo a; curl x",
        "echo a | xargs curl",
    ] {
        assert_eq!(bash(&rules, command), deny(command), "{command}");
    }
    // A command Claude Code cannot take apart meets deny rules on the whole text.
    assert_eq!(bash(&rules, "curl $(cat url)"), deny("curl $(cat url)"));
}

#[test]
fn cd_with_git_is_never_allowed_by_rules() {
    let rules = local_rules(&["Bash(cd:*)", "Bash(git status)"], &[], &[]);
    assert_eq!(bash(&rules, "cd sub && git status"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "cd a && cd b"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "cd sub"), allow("Bash(cd:*)"));
}

#[test]
fn whole_tool_deny_and_ask_come_first() {
    let rules = local_rules(&["Bash(ls)"], &["Bash"], &[]);
    assert_eq!(bash(&rules, "ls"), ask("Bash"));
    let rules = super::PermissionRules::new(
        &[(
            RuleSource::User,
            json!({ "permissions": { "allow": ["Edit"], "deny": ["WebSearch"] } }),
        )],
        None,
        None,
    );
    assert_eq!(
        rules.evaluate("WebSearch", ToolInput::Tool),
        RuleVerdict::Deny {
            rule: "WebSearch".to_string(),
            message: "Permission to use WebSearch has been denied.".to_string(),
        }
    );
    assert_eq!(
        rules.evaluate("TodoWrite", ToolInput::Tool),
        RuleVerdict::NoMatch
    );
}

#[test]
fn file_patterns_follow_gitignore_relative_to_their_anchor() {
    let settings = [
        (
            RuleSource::User,
            json!({ "permissions": {
                "deny": ["Read(~/.ssh/**)", "Read(/secrets.json)"],
                "allow": ["Read(//etc/hosts)"]
            } }),
        ),
        (
            RuleSource::Project,
            json!({ "permissions": {
                "deny": ["Read(.env)", "Read(/build/**)"],
                "ask": ["Read(*.pem)"],
                "allow": ["Read(./docs/**)"]
            } }),
        ),
    ];
    let rules = rules_from(&settings);
    let denied = |rule: &str, path: &str| RuleVerdict::Deny {
        rule: rule.to_string(),
        message: format!("Permission to read {path} has been denied."),
    };
    let cases = [
        (
            "/home/user/.ssh/id_rsa",
            denied("Read(~/.ssh/**)", "/home/user/.ssh/id_rsa"),
        ),
        (
            "/home/user/.ssh",
            denied("Read(~/.ssh/**)", "/home/user/.ssh"),
        ),
        (
            "/home/user/.claude/secrets.json",
            denied("Read(/secrets.json)", "/home/user/.claude/secrets.json"),
        ),
        ("/work/project/secrets.json", RuleVerdict::NoMatch),
        (
            "/work/project/.env",
            denied("Read(.env)", "/work/project/.env"),
        ),
        (
            "/work/project/app/.ENV",
            denied("Read(.env)", "/work/project/app/.ENV"),
        ),
        (
            "/work/project/build/out/x.js",
            denied("Read(/build/**)", "/work/project/build/out/x.js"),
        ),
        ("/work/project/src/build/x.js", RuleVerdict::NoMatch),
        ("/work/project/certs/key.pem", ask("Read(*.pem)")),
        ("/work/project/docs/a/b.md", allow("Read(./docs/**)")),
        ("/work/project/src/../docs/x.md", allow("Read(./docs/**)")),
        ("/etc/hosts", allow("Read(//etc/hosts)")),
        ("/etc/passwd", RuleVerdict::NoMatch),
        ("/elsewhere/.env", RuleVerdict::NoMatch),
    ];
    for (path, expected) in cases {
        assert_eq!(read(&rules, path), expected, "{path}");
    }
}

#[test]
fn edit_rules_cover_write_and_allow_reading() {
    let rules = local_rules(
        &["Edit(src/**)"],
        &["Edit(src/generated/**)"],
        &["Edit(*.lock)"],
    );
    assert_eq!(
        edit(&rules, "Write", "/work/project/src/main.rs"),
        allow("Edit(src/**)")
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/src/main.rs"),
        allow("Edit(src/**)")
    );
    assert_eq!(
        read(&rules, "/work/project/src/main.rs"),
        allow("Edit(src/**)")
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/src/generated/x.rs"),
        ask("Edit(src/generated/**)")
    );
    assert_eq!(
        edit(&rules, "Write", "/work/project/src/Cargo.lock"),
        RuleVerdict::Deny {
            rule: "Edit(*.lock)".to_string(),
            message: "Permission to edit /work/project/src/Cargo.lock has been denied.".to_string(),
        }
    );
    // Reading what may not be edited is not allowed by the edit rules.
    assert_eq!(
        read(&rules, "/work/project/src/Cargo.lock"),
        RuleVerdict::NoMatch
    );
    // `Write(...)` rules do not govern file paths; only `Edit(...)` rules do.
    let rules = local_rules(&["Write(src/**)"], &[], &[]);
    assert_eq!(
        edit(&rules, "Write", "/work/project/src/main.rs"),
        RuleVerdict::NoMatch
    );
}

#[test]
fn allow_rules_never_cover_sensitive_files() {
    let rules = local_rules(&["Edit(**)"], &[], &[]);
    for path in [
        "/work/project/.git/config",
        "/work/project/.claude/settings.json",
        "/work/project/.VSCode/settings.json",
        "/home/user/.bashrc",
        "/work/project/.mcp.json",
    ] {
        assert_eq!(edit(&rules, "Edit", path), RuleVerdict::NoMatch, "{path}");
    }
    assert_eq!(
        edit(&rules, "Edit", "/work/project/.claude/worktrees/x/src/a.rs"),
        allow("Edit(**)")
    );
}

#[test]
fn web_fetch_rules_name_the_host() {
    let rules = local_rules(
        &["WebFetch(domain:docs.rs)"],
        &["WebFetch(domain:github.com)"],
        &["WebFetch(domain:evil.example)"],
    );
    let fetch = |url: &str| rules.evaluate("WebFetch", ToolInput::Url(url));
    assert_eq!(
        fetch("https://docs.rs/serde"),
        allow("WebFetch(domain:docs.rs)")
    );
    assert_eq!(
        fetch("https://DOCS.rs/serde"),
        allow("WebFetch(domain:docs.rs)")
    );
    assert_eq!(
        fetch("https://github.com/x"),
        ask("WebFetch(domain:github.com)")
    );
    assert_eq!(fetch("https://api.github.com/x"), RuleVerdict::NoMatch);
    assert_eq!(
        fetch("http://evil.example:8080/a"),
        RuleVerdict::Deny {
            rule: "WebFetch(domain:evil.example)".to_string(),
            message: "WebFetch denied access to domain:evil.example.".to_string(),
        }
    );
    assert_eq!(fetch("not a url"), RuleVerdict::NoMatch);
}

#[test]
fn mcp_rules_name_a_server_or_a_tool() {
    let rules = local_rules(
        &["mcp__github"],
        &["mcp__slack__*"],
        &["mcp__github__delete_repo"],
    );
    let call = |tool: &str| rules.evaluate(tool, ToolInput::Tool);
    assert_eq!(call("mcp__github__list_issues"), allow("mcp__github"));
    assert_eq!(call("mcp__slack__post"), ask("mcp__slack__*"));
    assert_eq!(
        call("mcp__github__delete_repo"),
        RuleVerdict::Deny {
            rule: "mcp__github__delete_repo".to_string(),
            message: "Permission to use mcp__github__delete_repo has been denied.".to_string(),
        }
    );
    assert_eq!(call("mcp__gitlab__x"), RuleVerdict::NoMatch);
}

#[test]
fn other_tools_match_their_content_exactly() {
    let rules = local_rules(&[], &[], &["Task(Explore)"]);
    assert_eq!(
        rules.evaluate("Agent", ToolInput::Content("Explore")),
        RuleVerdict::Deny {
            rule: "Task(Explore)".to_string(),
            message: "Permission to use Agent has been denied.".to_string(),
        }
    );
    assert_eq!(
        rules.evaluate("Task", ToolInput::Content("Plan")),
        RuleVerdict::NoMatch
    );
}

#[test]
fn rules_are_loaded_from_the_user_project_and_local_files() {
    let home = tempfile::tempdir().expect("tempdir");
    let project = tempfile::tempdir().expect("tempdir");
    let write = |dir: &Path, name: &str, value: Value| {
        let dot_claude = dir.join(".claude");
        std::fs::create_dir_all(&dot_claude).expect("create .claude");
        std::fs::write(dot_claude.join(name), value.to_string()).expect("write settings");
    };
    write(
        home.path(),
        "settings.json",
        json!({ "permissions": { "allow": ["Bash(ls)"] } }),
    );
    write(
        project.path(),
        "settings.json",
        json!({ "permissions": { "ask": ["Bash(ls)"] } }),
    );
    write(
        project.path(),
        "settings.local.json",
        json!({ "permissions": { "deny": ["WebSearch"] } }),
    );
    let (rules, errors) = PermissionRules::load(Some(home.path()), Some(project.path()));
    assert!(errors.is_empty());
    let sources = rules
        .rules()
        .iter()
        .map(|rule| (rule.source, rule.text.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        sources,
        vec![
            (RuleSource::User, "Bash(ls)"),
            (RuleSource::Project, "Bash(ls)"),
            (RuleSource::Local, "WebSearch"),
        ]
    );
    assert_eq!(
        rules.evaluate(
            "Bash",
            ToolInput::Command {
                command: "ls",
                cwd: project.path()
            }
        ),
        ask("Bash(ls)")
    );
}
