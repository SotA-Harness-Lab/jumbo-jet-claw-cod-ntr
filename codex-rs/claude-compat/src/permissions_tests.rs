// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use pretty_assertions::assert_eq;
use serde_json::Value;
use serde_json::json;

use super::PermissionRules;
use super::RuleSource;
use super::RuleVerdict;
use super::ToolInput;
use super::bash::compiled_wildcard;
use super::bash::matches_wildcard;
use super::bash::split_command;
use super::bash::strip_output_redirections;
use super::bash::strip_safe_wrappers;
use super::link_target;
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
    PermissionRules::new(
        settings,
        Some(&Path::new(HOME).join(".claude")),
        Some(Path::new(HOME)),
        Some(Path::new(PROJECT)),
    )
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
fn wildcard_patterns_are_compiled_once() {
    let first = compiled_wildcard("cargo test * --release");
    let second = compiled_wildcard("cargo test * --release");
    assert!(
        first
            .zip(second)
            .is_some_and(|(first, second)| Arc::ptr_eq(&first, &second))
    );
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
    assert_eq!(
        split("# comment\nls # trailing"),
        Some((owned(&["ls"]), false))
    );
    assert_eq!(split("echo 'open"), None);
    assert_eq!(split("echo $(ls"), None);
    assert_eq!(split("ls )"), None);
    assert_eq!(split("echo {a,b}").map(|(_, complex)| complex), Some(false));
}

#[test]
fn nested_commands_are_taken_out() {
    let split =
        |command: &str| split_command(command).map(|split| (split.subcommands, split.complex));
    let owned = |parts: &[&str]| parts.iter().map(ToString::to_string).collect::<Vec<_>>();
    let cases: &[(&str, &[&str])] = &[
        ("echo $(rm -rf /)", &["echo $(rm -rf /)", "rm -rf /"]),
        ("echo `id`", &["echo `id`", "id"]),
        ("echo \"a $(b; c) d\"", &["echo \"a $(b; c) d\"", "b", "c"]),
        (
            "echo $(echo $(id))",
            &["echo $(echo $(id))", "echo $(id)", "id"],
        ),
        ("(cd x && make)", &["cd x", "make"]),
        ("true && { rm -rf build; }", &["true", "rm -rf build"]),
        ("for f in $(ls); do rm $f; done", &["rm $f", "ls"]),
        (
            "if test -f a; then cat a; elif true; then :; else ls; fi > out",
            &["test -f a", "cat a", "true", ":", "ls"],
        ),
        (
            "while read l; do echo $l; done < in",
            &["read l", "echo $l"],
        ),
        ("case $x in a|b) rm a;; (c) ls ;; esac", &["rm a", "ls"]),
        (
            "diff <(ls a) <(ls b)",
            &["diff <(ls a) <(ls b)", "ls a", "ls b"],
        ),
        ("a=($(ls) x y)", &["a=($(ls) x y)", "ls"]),
        ("echo ${x:-$(id)}", &["echo ${x:-$(id)}", "id"]),
        ("echo $((1 + $(id)))", &["echo $((1 + $(id)))", "id"]),
        ("cat <<EOF\n$(id)\nEOF", &["cat <<EOF", "id"]),
    ];
    for &(command, parts) in cases {
        assert_eq!(split(command), Some((owned(parts), true)), "{command}");
    }
    assert_eq!(
        split("cat <<'EOF'\n$(id)\nEOF"),
        Some((owned(&["cat <<'EOF'"]), false))
    );
    // A quoted heredoc read by `cat` is the text it holds.
    let commit = "git commit -m \"$(cat <<'EOF'\nFix it (it's broken)\nEOF\n)\"";
    assert_eq!(split(commit), Some((owned(&[commit]), false)));
    assert_eq!(
        split("echo \"$(cat <<'EOF'\n/proc/self/environ\nEOF\n)\"").map(|(_, complex)| complex),
        Some(true)
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
    assert_eq!(strip_safe_wrappers("command git status"), "git status");
    assert_eq!(strip_safe_wrappers("builtin cd x"), "cd x");
    assert_eq!(strip_safe_wrappers("noglob ls *"), "ls *");
    assert_eq!(strip_safe_wrappers("command -v git"), "command -v git");
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
fn deny_and_ask_rules_meet_the_words_the_shell_runs() {
    let rules = local_rules(&[], &["Bash(git push:*)"], &["Bash(rm:*)"]);
    let deny = |command: &str| denied_command("Bash(rm:*)", command);
    for command in [
        "sudo rm -rf build",
        "sudo -u root -E rm -rf build",
        "doas -u root rm -rf build",
        "env rm -rf build",
        "env -i -- FOO=1 rm -rf build",
        "command rm -rf build",
        "builtin rm -rf build",
        "exec rm -rf build",
        "! rm -rf build",
        "\"rm\" -rf build",
        "'rm' -rf build",
        "\\rm -rf build",
        "rm\t-rf build",
        "TZ=\"$HOME\" rm -rf build",
        "X=$PWD rm -rf build",
        "nice -n 5 rm -rf build",
        "timeout --signal KILL 5 rm -rf build",
        "watch -n 1 rm -rf build",
        "ionice -c 3 rm -rf build",
        "setsid -f rm -rf build",
        "flock /tmp/lock rm -rf build",
        "flock -w 5 /tmp/lock -c 'rm -rf build'",
        "sudo env FOO=1 nohup rm -rf build",
        "ls; sudo rm -rf build",
    ] {
        assert_eq!(bash(&rules, command), deny(command), "{command}");
    }
    for command in [
        "git  push --force",
        "git\tpush",
        "sudo git push",
        "\"git\" push",
    ] {
        assert_eq!(bash(&rules, command), ask("Bash(git push:*)"), "{command}");
    }
    // A lookup is not a run, and other words are other commands.
    for command in ["command -v rm", "echo rm -rf build", "rmdir build"] {
        assert_eq!(bash(&rules, command), RuleVerdict::NoMatch, "{command}");
    }
}

#[test]
fn broad_bash_rules_do_not_preapprove_exec_wrappers_or_find_actions() {
    let rules = local_rules(
        &[
            "Bash(watch:*)",
            "Bash(setsid *)",
            "Bash(flock *)",
            "Bash(find *)",
            "Bash(find build -type f -delete)",
            "Bash(watch -n 5 date)",
        ],
        &[],
        &[],
    );
    for command in [
        "watch -n 1 date",
        "setsid date",
        "flock lock date",
        "find build -delete",
        "find build -fprint manifest",
        "find -files0-from source -print",
        "nice find build -delete",
        "xargs find build -delete",
        "find build -print && watch -n 1 date",
    ] {
        assert_eq!(bash(&rules, command), RuleVerdict::NoMatch, "{command}");
    }
    assert_eq!(bash(&rules, "find build -print"), allow("Bash(find *)"));
    assert_eq!(
        bash(&rules, "find build -name '-delete' -print"),
        allow("Bash(find *)")
    );
    assert_eq!(
        bash(&rules, "find build -type f -delete"),
        allow("Bash(find build -type f -delete)")
    );
    assert_eq!(
        bash(&rules, "watch -n 5 date"),
        allow("Bash(watch -n 5 date)")
    );
}

#[test]
fn tool_name_wildcards_match_full_names_for_ask_and_deny() {
    let rules = local_rules(&["*"], &["Web*"], &["WebF*tch", "mcp__*"]);
    assert!(matches!(
        rules.evaluate("WebFetch", ToolInput::Tool),
        RuleVerdict::Deny { .. }
    ));
    assert_eq!(rules.evaluate("WebSearch", ToolInput::Tool), ask("Web*"));
    assert!(matches!(
        rules.evaluate("mcp__example__read", ToolInput::Tool),
        RuleVerdict::Deny { .. }
    ));
    assert_eq!(
        rules.evaluate("Read", ToolInput::Tool),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        rules.evaluate("MyWebSearch", ToolInput::Tool),
        RuleVerdict::NoMatch
    );
}

#[test]
fn parameter_rules_match_explicit_scalar_values_before_normalization() {
    let rules = local_rules(
        &["Agent", "Bash"],
        &["Agent(model: opus*)", "Bash(description:build *)"],
        &[
            "Agent(model:opus-preview)",
            "Agent(isolation:*)",
            "Bash(run_in_background:true)",
            "Bash(command:git status)",
        ],
    );
    let agent = |arguments| {
        rules.evaluate_with_arguments("Agent", ToolInput::Content("reviewer"), Some(arguments))
    };
    assert_eq!(agent(r#"{"model":"opus"}"#), ask("Agent(model: opus*)"));
    assert!(matches!(
        agent(r#"{"model":"opus-preview"}"#),
        RuleVerdict::Deny { .. }
    ));
    assert_eq!(agent(r#"{"model":"claude-opus-4-7"}"#), allow("Agent"));
    assert_eq!(
        agent(r#"{"isolation":{"type":"worktree"}}"#),
        allow("Agent")
    );
    assert_eq!(agent("{}"), allow("Agent"));
    let bash = |arguments| {
        rules.evaluate_with_arguments(
            "Bash",
            ToolInput::Command {
                command: "git status",
                cwd: Path::new(PROJECT),
            },
            Some(arguments),
        )
    };
    assert!(matches!(
        bash(r#"{"command":"git status","run_in_background":true}"#),
        RuleVerdict::Deny { .. }
    ));
    assert_eq!(
        bash(r#"{"command":"git status","description":"build project"}"#),
        ask("Bash(description:build *)")
    );
    assert_eq!(
        bash(r#"{"command":"git status","description":"build"}"#),
        allow("Bash")
    );
}

#[test]
fn allow_rules_meet_blanks_and_builtin_wrappers_but_not_unquoted_words() {
    let rules = local_rules(&["Bash(git commit:*)", "Bash(npm test *)"], &[], &[]);
    assert_eq!(
        bash(&rules, "git  commit\t-m x"),
        allow("Bash(git commit:*)")
    );
    assert_eq!(
        bash(&rules, "npm   test --watch"),
        allow("Bash(npm test *)")
    );
    assert_eq!(bash(&rules, "command npm test"), allow("Bash(npm test *)"));
    assert_eq!(bash(&rules, "sudo npm test"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "\"npm\" test"), RuleVerdict::NoMatch);
}

#[test]
fn deny_and_ask_rules_meet_nested_commands() {
    let rules = local_rules(&["Bash(echo:*)"], &["Bash(git push:*)"], &["Bash(rm:*)"]);
    let deny = |command: &str| denied_command("Bash(rm:*)", command);
    for command in [
        "(rm -rf build)",
        "true && { rm -rf build; }",
        "echo $(date); rm -rf build",
        "echo $(rm -rf build)",
        "echo `rm -rf build`",
        "for f in a; do rm -rf $f; done",
        "if true; then rm -rf build; fi",
        "git push origin \"$(rm -rf build)\"",
    ] {
        assert_eq!(bash(&rules, command), deny(command), "{command}");
    }
    assert_eq!(
        bash(&rules, "git push origin \"$(git branch --show-current)\""),
        ask("Bash(git push:*)")
    );
    assert_eq!(
        bash(&rules, "echo \"$(git push)\""),
        ask("Bash(git push:*)")
    );
    // Allow rules still cover no such command.
    assert_eq!(bash(&rules, "echo $(date)"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "(echo a)"), RuleVerdict::NoMatch);
}

#[test]
fn a_quoted_heredoc_read_by_cat_is_an_inert_argument() {
    let commit = "git commit -m \"$(cat <<'EOF'\nFix the parser (it's $(broken))\nEOF\n)\"";
    let rules = local_rules(&["Bash(git commit:*)"], &[], &[]);
    assert_eq!(bash(&rules, commit), allow("Bash(git commit:*)"));
    let rules = local_rules(&[], &["Bash(git commit:*)"], &[]);
    assert_eq!(bash(&rules, commit), ask("Bash(git commit:*)"));
    // An unquoted heredoc is expanded, so it is not inert.
    let rules = local_rules(&["Bash(git commit:*)"], &[], &[]);
    assert_eq!(
        bash(&rules, "git commit -m \"$(cat <<EOF\nmsg\nEOF\n)\""),
        RuleVerdict::NoMatch
    );
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
fn single_directory_allow_rules_cover_only_the_top_directory() {
    let rules = local_rules(&["Edit(src/**)"], &["Edit(gen/**)"], &["Read(secrets/**)"]);
    assert_eq!(
        edit(&rules, "Edit", "/work/project/src/main.rs"),
        allow("Edit(src/**)")
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/vendor/foo/src/x.rs"),
        RuleVerdict::NoMatch
    );
    // Deny and ask rules still cover the directory at any depth.
    assert_eq!(
        edit(&rules, "Edit", "/work/project/a/gen/x.rs"),
        ask("Edit(gen/**)")
    );
    assert_eq!(
        read(&rules, "/work/project/a/secrets/key"),
        RuleVerdict::Deny {
            rule: "Read(secrets/**)".to_string(),
            message: "Permission to read /work/project/a/secrets/key has been denied.".to_string(),
        }
    );
}

#[test]
fn rules_for_everything_under_an_anchor_match() {
    let rules = rules_from(&[
        (
            RuleSource::User,
            json!({ "permissions": { "deny": ["Read(~/**)", "Read(/**)"] } }),
        ),
        (
            RuleSource::Local,
            json!({ "permissions": { "allow": ["Edit(/**)"], "ask": ["Read(//**)"] } }),
        ),
    ]);
    let denied = |rule: &str, path: &str| RuleVerdict::Deny {
        rule: rule.to_string(),
        message: format!("Permission to read {path} has been denied."),
    };
    assert_eq!(
        read(&rules, "/home/user/notes/a.txt"),
        denied("Read(~/**)", "/home/user/notes/a.txt")
    );
    assert_eq!(read(&rules, "/etc/hosts"), ask("Read(//**)"));
    assert_eq!(
        edit(&rules, "Edit", "/work/project/src/main.rs"),
        allow("Edit(/**)")
    );
    assert_eq!(
        edit(&rules, "Edit", "/elsewhere/main.rs"),
        RuleVerdict::NoMatch
    );
}

#[test]
fn read_deny_rules_become_globs_relative_to_the_search_directory() {
    let rules = rules_from(&[
        (
            RuleSource::User,
            json!({ "permissions": {
                "deny": ["Read(~/.ssh/**)", "Read(//etc/shadow)", "Read(//**/*.key)"],
                "allow": ["Read(*.pem)"]
            } }),
        ),
        (
            RuleSource::Project,
            json!({ "permissions": {
                "deny": ["Read(.env)", "Read(/build/**)", "Read(./src/secret/*.json)", "Read(!x)"],
                "ask": ["Read(*.log)"]
            } }),
        ),
    ]);
    let globs = |search_dir: &str| rules.read_deny_globs(Path::new(search_dir), Path::new(PROJECT));
    assert_eq!(
        globs(PROJECT),
        vec![
            "/**/*.key".to_string(),
            "**/.env".to_string(),
            "/src/secret/*.json".to_string(),
            "/build".to_string(),
        ]
    );
    assert_eq!(
        globs("/work/project/src"),
        vec![
            "/**/*.key".to_string(),
            "**/.env".to_string(),
            "/secret/*.json".to_string(),
        ]
    );
    assert_eq!(
        globs("/work"),
        vec![
            "/**/*.key".to_string(),
            "/project/**/.env".to_string(),
            "/project/src/secret/*.json".to_string(),
            "/project/build".to_string(),
        ]
    );
    assert_eq!(
        globs(HOME),
        vec!["/.ssh".to_string(), "/**/*.key".to_string()]
    );
    assert_eq!(
        globs("/home/user/.ssh/keys"),
        vec!["**".to_string(), "/**/*.key".to_string()]
    );
    assert_eq!(
        globs("/etc"),
        vec!["/shadow".to_string(), "/**/*.key".to_string()]
    );
}

#[test]
fn a_path_through_a_symbolic_link_is_checked_where_it_leads_too() {
    let rules = local_rules(&["Read(docs/**)", "Edit(*.txt)"], &[], &["Read(~/.ssh/**)"]);
    let link = PathBuf::from("/work/project/docs/key");
    let target = PathBuf::from("/home/user/.ssh/id_rsa");
    let paths = [link.clone(), target.clone()];
    // A deny rule for what the link leads to denies the link.
    assert_eq!(
        rules.evaluate(
            "Read",
            ToolInput::Read {
                paths: &paths,
                cwd: Path::new(PROJECT),
            },
        ),
        RuleVerdict::Deny {
            rule: "Read(~/.ssh/**)".to_string(),
            message: "Permission to read /work/project/docs/key has been denied.".to_string(),
        }
    );
    // An allow rule has to cover both the link and what it leads to.
    let rules = local_rules(&["Read(docs/**)", "Edit(*.txt)"], &[], &[]);
    let read_paths = |paths: &[PathBuf]| {
        rules.evaluate(
            "Read",
            ToolInput::Read {
                paths,
                cwd: Path::new(PROJECT),
            },
        )
    };
    assert_eq!(read_paths(&[link.clone()]), allow("Read(docs/**)"));
    assert_eq!(read_paths(&[link, target]), RuleVerdict::NoMatch);
    assert_eq!(
        read_paths(&[
            PathBuf::from("/work/project/docs/a.md"),
            PathBuf::from("/work/project/docs/b.md"),
        ]),
        allow("Read(docs/**)")
    );
    let edit_paths = |paths: &[PathBuf]| {
        rules.evaluate(
            "Edit",
            ToolInput::Edit {
                paths,
                cwd: Path::new(PROJECT),
            },
        )
    };
    assert_eq!(
        edit_paths(&[
            PathBuf::from("/work/project/notes.txt"),
            PathBuf::from("/work/project/src/main.rs"),
        ]),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        edit_paths(&[
            PathBuf::from("/work/project/notes.txt"),
            PathBuf::from("/work/project/.vscode/tasks.json"),
        ]),
        RuleVerdict::NoMatch
    );
}

#[test]
fn a_pattern_that_is_no_glob_guards_the_path_it_spells() {
    let rules = local_rules(
        &["Read(docs/{draft)", "Read(*.md)"],
        &["Edit(out/[z-a].txt)"],
        &["Read(data/{raw/secret.csv)"],
    );
    assert_eq!(
        read(&rules, "/work/project/data/{raw/secret.csv"),
        RuleVerdict::Deny {
            rule: "Read(data/{raw/secret.csv)".to_string(),
            message: "Permission to read /work/project/data/{raw/secret.csv has been denied."
                .to_string(),
        }
    );
    assert_eq!(
        read(&rules, "/work/project/data/raw/secret.csv"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/out/[z-a].txt"),
        ask("Edit(out/[z-a].txt)")
    );
    // An allow rule that is no glob allows nothing, and the other rules still work.
    assert_eq!(
        read(&rules, "/work/project/docs/{draft"),
        RuleVerdict::NoMatch
    );
    assert_eq!(read(&rules, "/work/project/README.md"), allow("Read(*.md)"));
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
        "/home/user/.config/git/hooks/pre-commit",
        "/work/project/.cargo/config.toml",
        "/work/project/.devcontainer/devcontainer.json",
        "/work/project/.husky/pre-commit",
        "/work/project/.npmrc",
        "/work/project/.envrc",
    ] {
        assert_eq!(edit(&rules, "Edit", path), RuleVerdict::NoMatch, "{path}");
    }
    assert_eq!(
        edit(&rules, "Edit", "/work/project/.claude/worktrees/x/src/a.rs"),
        allow("Edit(**)")
    );
}

#[test]
fn a_rule_for_the_whole_edit_tool_does_not_cover_sensitive_files_either() {
    let rules = local_rules(&["Edit", "Write"], &[], &[]);
    for tool in ["Edit", "Write"] {
        assert_eq!(
            edit(&rules, tool, "/work/project/.git/hooks/pre-commit"),
            RuleVerdict::NoMatch,
            "{tool}"
        );
        assert_eq!(edit(&rules, tool, "/work/project/src/a.rs"), allow(tool));
    }
}

#[test]
fn only_the_user_can_set_deny_instead_of_prompting() {
    let settings = |value| json!({ "permissions": { "denyInsteadOfPrompting": value } });
    assert!(
        rules_from(&[
            (RuleSource::User, settings(true)),
            (RuleSource::Local, settings(false)),
        ])
        .deny_instead_of_prompting()
    );
    assert!(
        !rules_from(&[
            (RuleSource::Project, settings(true)),
            (RuleSource::Local, settings(true)),
        ])
        .deny_instead_of_prompting()
    );
}

#[test]
fn an_untrusted_project_keeps_its_deny_and_ask_rules_but_not_its_allow_rules() {
    let rules = rules_from(&[
        (
            RuleSource::User,
            json!({ "permissions": { "allow": ["Bash(ls)"] } }),
        ),
        (
            RuleSource::Project,
            json!({ "permissions": { "allow": ["Bash(make *)"], "deny": ["Bash(curl *)"] } }),
        ),
        (
            RuleSource::Local,
            json!({ "permissions": { "allow": ["Bash"], "ask": ["Bash(git push *)"] } }),
        ),
    ])
    .without_project_allow_rules();

    assert_eq!(bash(&rules, "ls"), allow("Bash(ls)"));
    assert_eq!(bash(&rules, "make test"), RuleVerdict::NoMatch);
    assert_eq!(bash(&rules, "rm -rf x"), RuleVerdict::NoMatch);
    assert_eq!(
        bash(&rules, "curl example.com"),
        denied_command("Bash(curl *)", "curl example.com")
    );
    assert_eq!(bash(&rules, "git push origin"), ask("Bash(git push *)"));
}

#[test]
fn path_rules_left_after_dropping_project_allow_rules_still_match() {
    let rules = rules_from(&[
        (
            RuleSource::User,
            json!({ "permissions": { "allow": ["Read(~/notes/**)"] } }),
        ),
        (
            RuleSource::Project,
            json!({ "permissions": {
                "allow": ["Edit(src/**)", "Read(docs/**)"],
                "deny": ["Read(.env)"]
            } }),
        ),
        (
            RuleSource::Local,
            json!({ "permissions": { "allow": ["Read(*.md)"], "ask": ["Edit(*.lock)"] } }),
        ),
    ]);
    // The matchers are built once; reading the same path again gives the same rule.
    for _ in 0..2 {
        assert_eq!(
            read(&rules, "/work/project/docs/a.txt"),
            allow("Read(docs/**)")
        );
    }
    let rules = rules.without_project_allow_rules();
    assert_eq!(
        read(&rules, "/work/project/app/.env"),
        RuleVerdict::Deny {
            rule: "Read(.env)".to_string(),
            message: "Permission to read /work/project/app/.env has been denied.".to_string(),
        }
    );
    assert_eq!(
        read(&rules, "/work/project/docs/a.txt"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        read(&rules, "/work/project/README.md"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/src/main.rs"),
        RuleVerdict::NoMatch
    );
    assert_eq!(
        edit(&rules, "Edit", "/work/project/Cargo.lock"),
        ask("Edit(*.lock)")
    );
    assert_eq!(
        read(&rules, "/home/user/notes/todo.txt"),
        allow("Read(~/notes/**)")
    );
}

#[test]
fn skill_prefix_rules_match_local_names_and_keep_deny_precedence() {
    let rules = local_rules(
        &["Skill(release *)", "Skill(acme:*)"],
        &[],
        &["Skill(acme:deploy)", "Skill(claude-ai:*)"],
    );
    let skill = |name| rules.evaluate("Skill", ToolInput::Content(name));
    assert_eq!(skill("release"), allow("Skill(release *)"));
    assert_eq!(skill("acme:docs"), allow("Skill(acme:*)"));
    assert!(matches!(skill("acme:deploy"), RuleVerdict::Deny { .. }));
    assert_eq!(skill("acme-two:docs"), RuleVerdict::NoMatch);
    assert_eq!(skill("claude-ai:local"), RuleVerdict::NoMatch);
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
fn web_fetch_domain_wildcards_respect_label_boundaries_and_precedence() {
    let rules = local_rules(
        &["WebFetch(domain:*)"],
        &["WebFetch(domain:docs.*)"],
        &["WebFetch(domain:*.Example.COM.)"],
    );
    let fetch = |url: &str| rules.evaluate("WebFetch", ToolInput::Url(url));
    assert!(matches!(
        fetch("https://a.b.example.com./"),
        RuleVerdict::Deny { .. }
    ));
    assert_eq!(fetch("https://example.com"), allow("WebFetch(domain:*)"));
    assert_eq!(fetch("https://notexample.com"), allow("WebFetch(domain:*)"));
    assert_eq!(fetch("https://docs.rs"), ask("WebFetch(domain:docs.*)"));
    assert_eq!(fetch("https://docs.evil.org"), allow("WebFetch(domain:*)"));
}

#[test]
fn web_fetch_hosts_match_without_case_or_a_trailing_dot() {
    let rules = local_rules(
        &["WebFetch(domain:Docs.RS.)"],
        &[],
        &["WebFetch(domain:Evil.com)"],
    );
    let fetch = |url: &str| rules.evaluate("WebFetch", ToolInput::Url(url));
    let denied = RuleVerdict::Deny {
        rule: "WebFetch(domain:Evil.com)".to_string(),
        message: "WebFetch denied access to domain:evil.com.".to_string(),
    };
    assert_eq!(fetch("https://evil.com/x"), denied);
    assert_eq!(fetch("https://evil.com./x"), denied);
    assert_eq!(fetch("https://EVIL.COM./x"), denied);
    assert_eq!(
        fetch("https://docs.rs/serde"),
        allow("WebFetch(domain:Docs.RS.)")
    );
    assert_eq!(fetch("https://sub.evil.com/x"), RuleVerdict::NoMatch);
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
fn the_local_rules_of_the_repository_root_apply_in_a_subdirectory() {
    let home = tempfile::tempdir().expect("tempdir");
    let repo = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(repo.path().join(".git")).expect("create .git");
    let root = repo.path().to_path_buf();
    let project = root.join("sub");
    for (dir, rule) in [(&project, "WebSearch"), (&root, "WebFetch")] {
        let dot_claude = dir.join(".claude");
        std::fs::create_dir_all(&dot_claude).expect("create .claude");
        std::fs::write(
            dot_claude.join("settings.local.json"),
            json!({ "permissions": { "deny": [rule] } }).to_string(),
        )
        .expect("write settings");
    }
    let (rules, errors) = PermissionRules::load(
        Some(&home.path().join(".claude")),
        Some(home.path()),
        Some(&project),
    );
    assert!(errors.is_empty());
    let sources = rules
        .rules()
        .iter()
        .map(|rule| (rule.source, rule.text.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        sources,
        vec![
            (RuleSource::Local, "WebSearch"),
            (RuleSource::Local, "WebFetch"),
        ]
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
    let (rules, errors) = PermissionRules::load(
        Some(&home.path().join(".claude")),
        Some(home.path()),
        Some(project.path()),
    );
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

#[test]
fn deny_and_ask_rules_through_a_linked_directory_meet_its_real_location() {
    // `/tmp` leads to `/private/tmp` and `~/vault` to `/mnt/vault`, as on macOS; nothing
    // else is a link.
    let resolve = |path: &Path| -> Option<PathBuf> {
        let text = path.to_str()?;
        if let Some(rest) = text.strip_prefix("/tmp") {
            return Some(PathBuf::from(format!("/private/tmp{rest}")));
        }
        if let Some(rest) = text.strip_prefix("/home/user/vault") {
            return Some(PathBuf::from(format!("/mnt/vault{rest}")));
        }
        matches!(text, "/" | "/home" | "/home/user").then(|| path.to_path_buf())
    };
    let rules = local_rules(
        &["Read(//tmp/open/**)"],
        &["Edit(~/vault/*.txt)"],
        &["Read(//tmp/x/**)", "Read(//etc/**)"],
    );
    // Without resolving, the real location is not covered.
    assert_eq!(read(&rules, "/private/tmp/x/a"), RuleVerdict::NoMatch);

    let rules = rules.with_resolved_rule_places(resolve);
    let denied = |rule: &str, path: &str| RuleVerdict::Deny {
        rule: rule.to_string(),
        message: format!("Permission to read {path} has been denied."),
    };
    assert_eq!(
        read(&rules, "/private/tmp/x/a"),
        denied("Read(//tmp/x/**)", "/private/tmp/x/a")
    );
    assert_eq!(
        read(&rules, "/tmp/x/a"),
        denied("Read(//tmp/x/**)", "/tmp/x/a")
    );
    // A rule whose place is no link stays as it is, and so do allow rules.
    assert_eq!(
        read(&rules, "/etc/passwd"),
        denied("Read(//etc/**)", "/etc/passwd")
    );
    assert_eq!(read(&rules, "/private/etc/passwd"), RuleVerdict::NoMatch);
    assert_eq!(read(&rules, "/private/tmp/open/a"), RuleVerdict::NoMatch);
    assert_eq!(
        edit(&rules, "Edit", "/mnt/vault/notes.txt"),
        ask("Edit(~/vault/*.txt)")
    );
}

#[test]
fn link_targets_are_taken_relative_to_the_link() {
    assert_eq!(
        link_target(Path::new("/w/docs/link"), "../secret/key"),
        PathBuf::from("/w/secret/key")
    );
    assert_eq!(
        link_target(Path::new("/w/link"), "/etc/passwd"),
        PathBuf::from("/etc/passwd")
    );
    assert_eq!(
        link_target(Path::new("/w/link"), "./same"),
        PathBuf::from("/w/same")
    );
}

#[test]
fn granted_allow_rules_allow_what_no_deny_or_ask_rule_holds_back() {
    let rules = local_rules(&[], &["Bash(git push *)"], &["Bash(rm *)"])
        .with_granted_allow_rules(&["Bash".to_string(), "Read".to_string()]);
    assert_eq!(bash(&rules, "ls -la"), allow("Bash"));
    assert_eq!(bash(&rules, "git push origin"), ask("Bash(git push *)"));
    assert_eq!(
        bash(&rules, "rm -rf x"),
        denied_command("Bash(rm *)", "rm -rf x")
    );
    assert_eq!(read(&rules, "/work/project/a.rs"), allow("Read"));
}
