use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::RuleFile;
use super::glob_matches;
use super::load_rules;
use super::rule_globs;
use crate::source::LocalFiles;

fn globs(frontmatter: &str) -> Option<Vec<String>> {
    rule_globs(frontmatter)
}

#[test]
fn reads_paths_in_every_accepted_form() {
    assert_eq!(
        globs("paths: [\"**/*.ipynb\"]\n"),
        Some(vec!["**/*.ipynb".into()])
    );
    assert_eq!(
        globs("paths: src/**, docs/*.md\n"),
        Some(vec!["src".into(), "docs/*.md".into()])
    );
    assert_eq!(
        globs("paths:\n  - \"src/**/*.{ts,tsx}\"\n  - lib/{a,b}/**\n"),
        Some(vec![
            "src/**/*.ts".into(),
            "src/**/*.tsx".into(),
            "lib/a".into(),
            "lib/b".into(),
        ])
    );
    // Unquoted globs are not valid YAML and are still read.
    assert_eq!(
        globs("paths: **/*.rs, *.md\n"),
        Some(vec!["**/*.rs".into(), "*.md".into()])
    );
}

#[test]
fn a_rule_without_real_globs_is_unconditional() {
    assert_eq!(globs("description: x\n"), None);
    assert_eq!(globs("paths: \"**\"\n"), None);
    assert_eq!(globs("paths: []\n"), None);
}

#[test]
fn matches_globs_like_gitignore_patterns() {
    let base = Path::new("/repo");
    let notebook = vec!["**/*.ipynb".to_string()];
    assert!(glob_matches(
        &notebook,
        base,
        Path::new("/repo/a/b/x.ipynb")
    ));
    assert!(glob_matches(&notebook, base, Path::new("/repo/x.ipynb")));
    assert!(!glob_matches(&notebook, base, Path::new("/repo/x.py")));
    assert!(!glob_matches(
        &notebook,
        base,
        Path::new("/elsewhere/x.ipynb")
    ));

    // A directory pattern covers everything below it.
    let source = vec!["src".to_string()];
    assert!(glob_matches(
        &source,
        base,
        Path::new("/repo/src/deep/main.rs")
    ));
    assert!(!glob_matches(
        &source,
        base,
        Path::new("/repo/tests/main.rs")
    ));
}

#[tokio::test]
async fn loads_rules_sorted_with_their_globs() {
    let dir = TempDir::new().expect("temp dir");
    let rules = dir.path().join("rules");
    std::fs::create_dir_all(rules.join("nested")).expect("create rules");
    std::fs::write(rules.join("b.md"), "---\npaths: \"*.py\"\n---\n\npython\n").expect("write");
    std::fs::write(rules.join("a.md"), "<!-- note -->\nalways\n").expect("write");
    std::fs::write(rules.join("nested/c.md"), "nested").expect("write");
    std::fs::write(rules.join("empty.md"), "---\npaths: x\n---\n\n").expect("write");
    std::fs::write(rules.join("notes.txt"), "not a rule").expect("write");

    assert_eq!(
        load_rules(&LocalFiles, &rules).await.expect("load rules"),
        vec![
            RuleFile {
                path: rules.join("a.md"),
                globs: None,
                content: "always".to_string(),
            },
            RuleFile {
                path: rules.join("b.md"),
                globs: Some(vec!["*.py".to_string()]),
                content: "python".to_string(),
            },
            RuleFile {
                path: rules.join("nested/c.md"),
                globs: None,
                content: "nested".to_string(),
            },
        ]
    );
    assert_eq!(
        load_rules(&LocalFiles, &dir.path().join("missing"))
            .await
            .expect("missing directory"),
        Vec::new()
    );
}
