use pretty_assertions::assert_eq;

use super::RgRun;
use super::Shell;
use super::apply_head_limit;
use super::command_line;
use super::relative_to;
use super::result_lines;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(ToString::to_string).collect()
}

#[test]
fn arguments_are_quoted_for_the_shell() {
    let args = strings(&["--glob", "!.git", "-e", "it's (a|b)"]);
    assert_eq!(
        command_line(&args, "/work/my dir", Shell::Posix),
        r"rg --no-config --glob '!.git' -e 'it'\''s (a|b)' '/work/my dir'"
    );
    assert_eq!(
        command_line(&args, r"C:\work", Shell::PowerShell),
        r"rg --no-config --glob '!.git' -e 'it''s (a|b)' 'C:\work'"
    );
    assert_eq!(
        command_line(&strings(&["a\"b"]), "C:/w", Shell::Cmd),
        r#"rg --no-config "a\"b" C:/w"#
    );
    assert_eq!(
        command_line(&strings(&[""]), "/w", Shell::Posix),
        "rg --no-config '' /w"
    );
    // zsh expands a word starting with `=` to a command's path
    assert_eq!(
        command_line(&strings(&["-e", "=x", "a=b"]), "/w", Shell::Posix),
        "rg --no-config -e '=x' a=b /w"
    );
}

#[test]
fn exit_codes_read_as_claude_code_reads_them() {
    let run = |output, exit_code, timed_out| RgRun {
        output,
        exit_code,
        timed_out,
        target: "/w",
    };
    assert_eq!(
        result_lines(&run("/w/a\r\n/w/b\n\n", Some(0), false)),
        Ok(strings(&["/w/a", "/w/b"]))
    );
    assert_eq!(result_lines(&run("", Some(1), false)), Ok(Vec::new()));
    // Errors beside results keep the results and drop ripgrep's messages.
    assert_eq!(
        result_lines(&run(
            "/w/a\nrg: /w/x: Permission denied (os error 13)\n",
            Some(2),
            false
        )),
        Ok(strings(&["/w/a"]))
    );
    assert_eq!(
        result_lines(&run("rg: regex parse error\n", Some(2), false)),
        Err("ripgrep failed with exit code 2\nrg: regex parse error".to_string())
    );
    // The lines after ripgrep's first line of a message are not results either.
    assert_eq!(
        result_lines(&run(
            "rg: regex parse error:\n    (\n    ^\nerror: unclosed group\n",
            Some(2),
            false
        )),
        Err("ripgrep failed with exit code 2\nrg: regex parse error:\n    (\n    ^\nerror: unclosed group".to_string())
    );
    // A timeout drops the last line, which may be cut off, and fails without results.
    assert_eq!(
        result_lines(&run("/w/a\n/w/b", None, true)),
        Ok(strings(&["/w/a"]))
    );
    assert!(
        result_lines(&run("/w/a", None, true))
            .unwrap_err()
            .starts_with("Ripgrep search timed out after 20 seconds.")
    );
    assert!(
        result_lines(&run("bash: rg: command not found", Some(127), false))
            .unwrap_err()
            .starts_with("ripgrep (rg) was not found.")
    );
    assert!(
        result_lines(&run(
            "rg : The term 'rg' is not recognized as a name of a cmdlet",
            Some(1),
            false
        ))
        .is_err()
    );
}

#[test]
fn paths_inside_the_working_directory_are_shortened() {
    assert_eq!(
        relative_to("/w/src/a.rs:12:fn a()", "/w"),
        "src/a.rs:12:fn a()"
    );
    assert_eq!(relative_to("/w/src/a.rs-13-}", "/w/"), "src/a.rs-13-}");
    assert_eq!(relative_to("/work/a.rs", "/w"), "/work/a.rs");
    assert_eq!(relative_to("/other/a.rs", "/w"), "/other/a.rs");
    assert_eq!(relative_to("/a.rs", "/"), "a.rs");
    assert_eq!(relative_to(r"C:\w\src\a.rs", r"C:\w"), r"src\a.rs");
    assert_eq!(relative_to("--", "/w"), "--");
}

#[test]
fn head_limit_defaults_to_250_and_zero_is_unlimited() {
    let items: Vec<usize> = (0..300).collect();
    let (kept, applied) = apply_head_limit(items.clone(), None, 0);
    assert_eq!((kept.len(), applied), (250, Some(250)));
    let (kept, applied) = apply_head_limit(items.clone(), Some(0), 10);
    assert_eq!((kept.len(), kept[0], applied), (290, 10, None));
    let (kept, applied) = apply_head_limit(items.clone(), Some(5), 295);
    assert_eq!((kept, applied), (vec![295, 296, 297, 298, 299], None));
    let (kept, applied) = apply_head_limit(items, Some(5), 294);
    assert_eq!((kept, applied), (vec![294, 295, 296, 297, 298], Some(5)));
}
