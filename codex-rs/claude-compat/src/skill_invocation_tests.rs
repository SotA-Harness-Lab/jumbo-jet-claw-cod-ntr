use pretty_assertions::assert_eq;

use super::argument_names;
use super::parse_arguments;
use super::render_invoked_skill;
use super::slash_invocation_arguments;
use super::substitute_arguments;
use super::substitute_skill_variables;

fn names(names: &[&str]) -> Vec<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

#[test]
fn finds_arguments_of_a_slash_invocation() {
    assert_eq!(slash_invocation_arguments("/demo", "demo"), Some(""));
    assert_eq!(
        slash_invocation_arguments("/demo  a b \n", "demo"),
        Some("a b")
    );
    assert_eq!(
        slash_invocation_arguments("/demo\nsecond line", "demo"),
        Some("second line")
    );
    assert_eq!(slash_invocation_arguments("/demos a", "demo"), None);
    assert_eq!(slash_invocation_arguments("$demo a", "demo"), None);
    assert_eq!(slash_invocation_arguments("run /demo a", "demo"), None);
}

#[test]
fn splits_arguments_like_a_shell() {
    assert_eq!(parse_arguments(""), Vec::<String>::new());
    assert_eq!(
        parse_arguments("  foo bar\tbaz "),
        names(&["foo", "bar", "baz"])
    );
    assert_eq!(
        parse_arguments(r#"foo "hello world" 'a b' c\ d"#),
        names(&["foo", "hello world", "a b", "c d"])
    );
    assert_eq!(
        parse_arguments(r#""say \"hi\" \n" $HOME ''"#),
        names(&[r#"say "hi" \n"#, "$HOME", ""])
    );
    assert_eq!(
        parse_arguments("'unclosed quote"),
        names(&["unclosed quote"])
    );
}

#[test]
fn reads_argument_names_from_frontmatter() {
    assert_eq!(
        argument_names("name: x\narguments: file  mode\n"),
        names(&["file", "mode"])
    );
    assert_eq!(
        argument_names("arguments:\n  - file\n  - \"1\"\n  - ''\n  - mode\n"),
        names(&["file", "mode"])
    );
    assert_eq!(argument_names("arguments: 2 file\n"), names(&["file"]));
    assert_eq!(argument_names("name: x\n"), Vec::<String>::new());
    assert_eq!(argument_names(": not yaml ["), Vec::<String>::new());
}

#[test]
fn substitutes_every_kind_of_placeholder() {
    let content = "all=$ARGUMENTS first=$ARGUMENTS[0] second=$1 third=$2 file=$file mode=$mode";
    assert_eq!(
        substitute_arguments(content, r#"a.txt "b c""#, &names(&["file", "mode"])),
        r#"all=a.txt "b c" first=a.txt second=b c third= file=a.txt mode=b c"#
    );
}

#[test]
fn leaves_lookalike_placeholders_alone() {
    let content = "$1a $file_name $file[0] $12x cost $ 5 $ARGUMENTS[x]";
    assert_eq!(
        substitute_arguments(content, "one", &names(&["file"])),
        "$1a $file_name $file[0] $12x cost $ 5 one[x]"
    );
}

#[test]
fn empty_arguments_clear_placeholders() {
    assert_eq!(
        substitute_arguments("[$ARGUMENTS] [$0] [$ARGUMENTS[3]]", "", &[]),
        "[] [] []"
    );
    assert_eq!(
        substitute_arguments("no placeholder", "", &[]),
        "no placeholder"
    );
}

#[test]
fn appends_arguments_when_there_is_no_placeholder() {
    assert_eq!(
        substitute_arguments("Do the thing.", "now please", &[]),
        "Do the thing.\n\nARGUMENTS: now please"
    );
}

#[test]
fn substitutes_skill_variables() {
    assert_eq!(
        substitute_skill_variables(
            "run ${CLAUDE_SKILL_DIR}/x.sh in ${CLAUDE_SESSION_ID}",
            r"C:\skills\demo",
            "thread-1",
        ),
        "run C:/skills/demo/x.sh in thread-1"
    );
    assert_eq!(
        substitute_skill_variables("${CLAUDE_SKILL_DIR}", "", "s"),
        "${CLAUDE_SKILL_DIR}"
    );
}

#[test]
fn renders_an_invoked_skill_like_claude_code() {
    let contents = "---\nname: demo\narguments: [target]\n---\nCheck $target with ${CLAUDE_SKILL_DIR}/check.sh (${CLAUDE_SESSION_ID}).\n";
    assert_eq!(
        render_invoked_skill(contents, "/home/u/.claude/skills/demo", "thread-1", "src"),
        "Base directory for this skill: /home/u/.claude/skills/demo\n\nCheck src with /home/u/.claude/skills/demo/check.sh (thread-1).\n"
    );
}

#[test]
fn renders_an_invoked_skill_without_placeholders() {
    assert_eq!(
        render_invoked_skill("Body.\n", "/s/demo", "t", "some args"),
        "Base directory for this skill: /s/demo\n\nBody.\n\n\nARGUMENTS: some args"
    );
    assert_eq!(
        render_invoked_skill("Body.\n", "/s/demo", "t", ""),
        "Base directory for this skill: /s/demo\n\nBody.\n"
    );
}
