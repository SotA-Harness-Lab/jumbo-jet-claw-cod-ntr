// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use pretty_assertions::assert_eq;

use super::apply_env_assignments;
use super::parse_env_file;

/// The variables `text` sets over `base`, sorted by name.
fn applied(text: &str, base: &[(&str, &str)]) -> Vec<(String, String)> {
    let mut env: HashMap<String, String> = base
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();
    apply_env_assignments(&parse_env_file(text), &mut env);
    let mut env = env.into_iter().collect::<Vec<_>>();
    env.sort();
    env
}

fn pairs(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect()
}

#[test]
fn assignments_are_read_and_other_shell_code_is_left_out() {
    let text = "# set up\nexport PATH_EXTRA=/opt/bin\nNODE_ENV=\"test\"\nexport  QUOTED='a b'\nsource ~/.nvm/nvm.sh\n1BAD=x\nif true; then X=1; fi\n";
    assert_eq!(
        applied(text, &[]),
        pairs(&[
            ("NODE_ENV", "test"),
            ("PATH_EXTRA", "/opt/bin"),
            ("QUOTED", "a b"),
        ])
    );
}

#[test]
fn variables_are_expanded_against_the_command_environment() {
    let text = "export PATH=\"$PATH:./node_modules/.bin\"\nexport PATH=${PATH}:/opt/x\nGREETING=\"hi $NAME!\"\nLITERAL='$PATH'\nUNSET=\"[$NOPE]\"\n";
    assert_eq!(
        applied(text, &[("PATH", "/usr/bin:/bin"), ("NAME", "you")]),
        pairs(&[
            ("GREETING", "hi you!"),
            ("LITERAL", "$PATH"),
            ("NAME", "you"),
            ("PATH", "/usr/bin:/bin:./node_modules/.bin:/opt/x"),
            ("UNSET", "[]"),
        ])
    );
}

#[test]
fn export_p_output_is_read() {
    let text = "declare -x NVM_DIR=\"/home/u/.nvm\"\ndeclare -x QUOTE=\"say \\\"hi\\\" for \\$5\"\ndeclare -x OLDPWD\n";
    assert_eq!(
        applied(text, &[]),
        pairs(&[("NVM_DIR", "/home/u/.nvm"), ("QUOTE", "say \"hi\" for $5")])
    );
}

#[test]
fn several_assignments_on_a_line_and_the_home_directory() {
    let text = "export A=1 B=two # note\nBIN=~/bin:~/.local/bin\n";
    assert_eq!(
        applied(text, &[("HOME", "/home/u")]),
        pairs(&[
            ("A", "1"),
            ("B", "two"),
            ("BIN", "/home/u/bin:/home/u/.local/bin"),
            ("HOME", "/home/u"),
        ])
    );
}

#[test]
fn lines_that_are_not_plain_assignments_are_left_out_whole() {
    let text = "export PATH=\"$(npm bin):$PATH\"\nX=`date`\nY=${Z:-fallback}\nA=1 some-command\nB=\"unclosed\nC=1; D=2\nE=$1\n";
    assert_eq!(
        applied(text, &[("PATH", "/bin")]),
        pairs(&[("PATH", "/bin")])
    );
}
