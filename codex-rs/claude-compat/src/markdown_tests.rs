// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use pretty_assertions::assert_eq;

use super::Document;
use super::import_paths;
use super::parse_document;
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
