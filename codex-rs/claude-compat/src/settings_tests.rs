use std::path::Path;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use serde_json::json;

use super::BashPrefixRule;
use super::PermissionDecision;
use super::bash_prefix_rules;
use super::settings_env;
use super::settings_paths;

fn rule(prefix: &[&str], decision: PermissionDecision) -> BashPrefixRule {
    BashPrefixRule {
        prefix: prefix.iter().map(ToString::to_string).collect(),
        decision,
    }
}

#[test]
fn bash_rules_become_prefixes() {
    let settings = json!({
        "permissions": {
            "allow": ["Bash(git status)", "Bash(git diff*)", "Bash(gh pr *)", "Bash(npm run:*)", "Read(~/x)"],
            "ask": ["Bash(rm *)"],
            "deny": ["Bash(git push --force*)", "Bash(curl * | sh)", "Bash"]
        }
    });

    let (rules, unsupported) = bash_prefix_rules(&settings);

    assert_eq!(
        rules,
        vec![
            rule(&["git", "status"], PermissionDecision::Allow),
            rule(&["git", "diff"], PermissionDecision::Allow),
            rule(&["gh", "pr"], PermissionDecision::Allow),
            rule(&["npm", "run"], PermissionDecision::Allow),
            rule(&["rm"], PermissionDecision::Ask),
            rule(&["git", "push", "--force"], PermissionDecision::Deny),
        ]
    );
    assert_eq!(unsupported, vec!["Read(~/x)", "Bash(curl * | sh)", "Bash"]);
}

#[test]
fn env_values_are_strings() {
    let settings = json!({"env": {"A": "1", "B": 2, "C": true}});
    assert_eq!(
        settings_env(&settings),
        vec![
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string()),
            ("C".to_string(), "true".to_string()),
        ]
    );
}

#[test]
fn settings_paths_skip_the_home_directory_as_a_project() {
    let home = Path::new("/home/u/.claude");
    assert_eq!(
        settings_paths(Some(home), Some(Path::new("/home/u"))),
        vec![PathBuf::from("/home/u/.claude/settings.json")]
    );
    assert_eq!(
        settings_paths(Some(home), Some(Path::new("/work/p"))),
        vec![
            PathBuf::from("/home/u/.claude/settings.json"),
            PathBuf::from("/work/p/.claude/settings.json"),
            PathBuf::from("/work/p/.claude/settings.local.json"),
        ]
    );
}
