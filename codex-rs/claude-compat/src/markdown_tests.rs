// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::Document;
use super::frontmatter_string;
use super::import_paths;
use super::parse_document;
use super::parse_frontmatter;
use super::strip_block_comments;

#[test]
fn splits_frontmatter_from_body() {
    assert_eq!(
        parse_document("---\npaths: a\n---\nbody\n"),
        Document {
            frontmatter: Some("paths: a\n"),
            body: "body\n",
        }
    );
    assert_eq!(
        parse_document("---\nnever closed\n"),
        Document {
            frontmatter: None,
            body: "---\nnever closed\n",
        }
    );
    assert_eq!(parse_document("plain").frontmatter, None);
}

#[test]
fn quotes_values_that_break_the_frontmatter_and_parses_again() {
    let value = parse_frontmatter(
        "name: x\ndescription: Triggers include: X, Y\nglob: **/*.rs\narguments: [topic]\nhint: \"keep\"\n",
    )
    .expect("parsed after quoting");
    assert_eq!(
        value.get("description").and_then(serde_yaml::Value::as_str),
        Some("Triggers include: X, Y")
    );
    assert_eq!(
        value.get("glob").and_then(serde_yaml::Value::as_str),
        Some("**/*.rs")
    );
    assert_eq!(
        value.get("arguments"),
        Some(&serde_yaml::Value::Sequence(vec!["topic".into()]))
    );
    assert_eq!(
        value.get("hint").and_then(serde_yaml::Value::as_str),
        Some("keep")
    );
    assert_eq!(parse_frontmatter("name: x\n- stray\n"), None);
}

#[test]
fn takes_bare_numbers_and_booleans_as_text() {
    let value = parse_frontmatter("name: 3000\nflag: true\nlist: [a]\n").expect("valid YAML");
    let text = |key: &str| value.get(key).and_then(frontmatter_string);
    assert_eq!(text("name"), Some("3000".to_string()));
    assert_eq!(text("flag"), Some("true".to_string()));
    assert_eq!(text("list"), None);
}

#[test]
fn strips_only_block_level_comments() {
    let body = "one\n<!-- gone\nstill gone -->\ntwo <!-- inline stays -->\n```\n<!-- in code stays -->\n```\n<!-- closes --> tail\n<!-- never closed\nend\n";
    assert_eq!(
        strip_block_comments(body),
        "one\ntwo <!-- inline stays -->\n```\n<!-- in code stays -->\n```\n tail\n<!-- never closed\nend\n"
    );
}

#[test]
fn finds_imports_outside_code() {
    let body = "See @./a.md and @~/b.md, also @docs/c.md#part and @/abs/d.txt.\n\
                Not these: mail@example.com `@./span.md` @@double @#hash @/\n\
                ```\n@./fenced.md\n```\n\
                @with\\ space.md\n- item @list.md\n";
    assert_eq!(
        import_paths(body),
        vec![
            "./a.md",
            "~/b.md,",
            "docs/c.md",
            "/abs/d.txt.",
            "with space.md",
            "list.md",
        ]
    );
}
