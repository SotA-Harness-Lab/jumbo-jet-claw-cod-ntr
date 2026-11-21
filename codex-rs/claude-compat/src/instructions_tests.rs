// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::fs;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::InstructionFile;
use super::InstructionKind;
use super::directory_instructions;
use super::file_instructions;
use super::render_instruction_files;
use super::user_instructions;
use crate::source::LocalFiles;

fn file(path: std::path::PathBuf, kind: InstructionKind, content: &str) -> InstructionFile {
    InstructionFile {
        path,
        kind,
        content: content.to_string(),
    }
}

#[tokio::test]
async fn user_instructions_follow_imports_and_skip_conditional_rules() {
    let home = TempDir::new().expect("temp dir");
    let claude = home.path().join(".claude");
    fs::create_dir_all(claude.join("rules")).expect("create");
    fs::write(
        claude.join("CLAUDE.md"),
        "---\nx: y\n---\nmain @extra.md and @~/notes.md\n`@not.md`\n",
    )
    .expect("write");
    fs::write(claude.join("extra.md"), "extra @CLAUDE.md @deeper.md").expect("write");
    fs::write(claude.join("deeper.md"), "deeper").expect("write");
    fs::write(claude.join("not.md"), "never loaded").expect("write");
    fs::write(home.path().join("notes.md"), "notes").expect("write");
    fs::write(claude.join("rules/always.md"), "always").expect("write");
    fs::write(
        claude.join("rules/py.md"),
        "---\npaths: \"*.py\"\n---\npython",
    )
    .expect("write");

    let files = user_instructions(&LocalFiles, &claude, Some(home.path()))
        .await
        .expect("load");

    assert_eq!(
        files,
        vec![
            file(
                claude.join("CLAUDE.md"),
                InstructionKind::User,
                "main @extra.md and @~/notes.md\n`@not.md`"
            ),
            file(
                claude.join("extra.md"),
                InstructionKind::User,
                "extra @CLAUDE.md @deeper.md"
            ),
            file(claude.join("deeper.md"), InstructionKind::User, "deeper"),
            file(home.path().join("notes.md"), InstructionKind::User, "notes"),
            file(
                claude.join("rules/always.md"),
                InstructionKind::User,
                "always"
            ),
        ]
    );
}

#[tokio::test]
async fn directory_instructions_load_each_kind_and_keep_imports_inside_the_project() {
    let root = TempDir::new().expect("temp dir");
    let project = root.path().join("project");
    fs::create_dir_all(project.join(".claude/rules")).expect("create");
    fs::write(project.join("CLAUDE.md"), "project @../outside.md @docs.md").expect("write");
    fs::write(project.join("docs.md"), "docs").expect("write");
    fs::write(root.path().join("outside.md"), "outside").expect("write");
    fs::write(project.join(".claude/CLAUDE.md"), "dot claude").expect("write");
    fs::write(project.join(".claude/rules/r.md"), "rule").expect("write");
    fs::write(
        project.join(".claude/rules/c.md"),
        "---\npaths: src/**\n---\nconditional",
    )
    .expect("write");
    fs::write(project.join("CLAUDE.local.md"), "local").expect("write");

    let mut seen = HashSet::new();
    let files = directory_instructions(&LocalFiles, &project, None, &project, &mut seen)
        .await
        .expect("load");

    assert_eq!(
        files,
        vec![
            file(
                project.join("CLAUDE.md"),
                InstructionKind::Project,
                "project @../outside.md @docs.md"
            ),
            file(project.join("docs.md"), InstructionKind::Project, "docs"),
            file(
                project.join(".claude/CLAUDE.md"),
                InstructionKind::Project,
                "dot claude"
            ),
            file(
                project.join(".claude/rules/r.md"),
                InstructionKind::Project,
                "rule"
            ),
            file(
                project.join("CLAUDE.local.md"),
                InstructionKind::Local,
                "local"
            ),
        ]
    );
    assert_eq!(
        render_instruction_files(&files[3..]),
        format!(
            "Contents of {} (project instructions, checked into the codebase):\n\nrule\n\n\
             Contents of {} (user's private project instructions, not checked in):\n\nlocal",
            project.join(".claude/rules/r.md").display(),
            project.join("CLAUDE.local.md").display(),
        )
    );
}

#[tokio::test]
async fn file_instructions_follow_claude_code_phases() {
    let tmp = TempDir::new().expect("temp dir");
    let user_rules = tmp.path().join("home/.claude/rules");
    let top = tmp.path().join("top");
    let cwd = top.join("repo");
    let sub = cwd.join("sub");
    let target = sub.join("deep/main.rs");
    fs::create_dir_all(&user_rules).expect("create");
    fs::create_dir_all(top.join(".claude/rules")).expect("create");
    fs::create_dir_all(cwd.join(".claude/rules")).expect("create");
    fs::create_dir_all(sub.join(".claude/rules")).expect("create");
    fs::create_dir_all(target.parent().unwrap()).expect("create");
    fs::write(
        user_rules.join("rs.md"),
        "---\npaths: \"**/*.rs\"\n---\nuser rs",
    )
    .expect("write");
    fs::write(
        user_rules.join("py.md"),
        "---\npaths: \"**/*.py\"\n---\nuser py",
    )
    .expect("write");
    fs::write(user_rules.join("always.md"), "user always").expect("write");
    fs::write(
        top.join(".claude/rules/top.md"),
        "---\npaths: repo/sub/**\n---\ntop rule",
    )
    .expect("write");
    fs::write(
        cwd.join(".claude/rules/repo.md"),
        "---\npaths: sub/deep/*.rs\n---\nrepo rule",
    )
    .expect("write");
    fs::write(cwd.join(".claude/rules/plain.md"), "repo always").expect("write");
    fs::write(cwd.join("CLAUDE.md"), "already loaded").expect("write");
    fs::write(sub.join("CLAUDE.md"), "sub doc").expect("write");
    fs::write(sub.join("CLAUDE.local.md"), "sub local").expect("write");
    fs::write(sub.join(".claude/rules/sub.md"), "sub always").expect("write");
    fs::write(
        sub.join(".claude/rules/scoped.md"),
        "---\npaths: deep/**\n---\nsub scoped",
    )
    .expect("write");
    fs::write(
        sub.join(".claude/rules/other.md"),
        "---\npaths: other/**\n---\nnever",
    )
    .expect("write");

    let files = file_instructions(
        &LocalFiles,
        Some(&user_rules),
        /*home_dir*/ None,
        &cwd,
        &target,
        /*include_project*/ true,
    )
    .await
    .expect("load");

    assert_eq!(
        files,
        vec![
            file(user_rules.join("rs.md"), InstructionKind::User, "user rs"),
            file(sub.join("CLAUDE.md"), InstructionKind::Project, "sub doc"),
            file(
                sub.join("CLAUDE.local.md"),
                InstructionKind::Local,
                "sub local"
            ),
            file(
                sub.join(".claude/rules/sub.md"),
                InstructionKind::Project,
                "sub always"
            ),
            file(
                sub.join(".claude/rules/scoped.md"),
                InstructionKind::Project,
                "sub scoped"
            ),
            file(
                top.join(".claude/rules/top.md"),
                InstructionKind::Project,
                "top rule"
            ),
            file(
                cwd.join(".claude/rules/repo.md"),
                InstructionKind::Project,
                "repo rule"
            ),
        ]
    );

    let user_only = file_instructions(
        &LocalFiles,
        Some(&user_rules),
        /*home_dir*/ None,
        &cwd,
        &target,
        /*include_project*/ false,
    )
    .await
    .expect("load");
    assert_eq!(
        user_only,
        vec![file(
            user_rules.join("rs.md"),
            InstructionKind::User,
            "user rs"
        )]
    );
}

#[cfg(unix)]
#[tokio::test]
async fn file_instructions_see_through_symlinks_and_dot_dot() {
    let tmp = TempDir::new().expect("temp dir");
    let cwd = tmp.path().join("real");
    let sub = cwd.join("src");
    fs::create_dir_all(cwd.join(".claude/rules")).expect("create");
    fs::create_dir_all(&sub).expect("create");
    fs::write(
        cwd.join(".claude/rules/rs.md"),
        "---\npaths: src/**\n---\nsrc rule",
    )
    .expect("write");
    fs::write(sub.join("CLAUDE.md"), "src doc").expect("write");
    let link = tmp.path().join("link");
    std::os::unix::fs::symlink(&cwd, &link).expect("symlink");
    let expected = vec![
        file(sub.join("CLAUDE.md"), InstructionKind::Project, "src doc"),
        file(
            cwd.join(".claude/rules/rs.md"),
            InstructionKind::Project,
            "src rule",
        ),
    ];

    for (cwd, target) in [
        // The file through a link to the working directory.
        (cwd.clone(), link.join("src/main.rs")),
        // The file through `..`.
        (cwd.clone(), cwd.join("../real/src/main.rs")),
        // The working directory through the link, the file by its real path.
        (link.clone(), sub.join("main.rs")),
    ] {
        let files = file_instructions(
            &LocalFiles,
            /*user_rules_dir*/ None,
            /*home_dir*/ None,
            &cwd,
            &target,
            /*include_project*/ true,
        )
        .await
        .expect("load");
        let expected = expected
            .iter()
            .map(|expected| InstructionFile {
                path: cwd.join(expected.path.strip_prefix(tmp.path().join("real")).unwrap()),
                ..expected.clone()
            })
            .collect::<Vec<_>>();
        assert_eq!(files, expected, "{}", target.display());
    }
}

#[tokio::test]
async fn a_rule_s_includes_follow_it_and_the_chain_stops_four_files_down() {
    let home = TempDir::new().expect("temp dir");
    let claude = home.path().join(".claude");
    fs::create_dir_all(claude.join("rules")).expect("create");
    fs::write(claude.join("rules/always.md"), "always @../one.md").expect("write");
    fs::write(claude.join("one.md"), "one @two.md").expect("write");
    fs::write(claude.join("two.md"), "two @three.md").expect("write");
    fs::write(claude.join("three.md"), "three @four.md").expect("write");
    fs::write(claude.join("four.md"), "four @five.md").expect("write");
    fs::write(claude.join("five.md"), "five").expect("write");

    let files = user_instructions(&LocalFiles, &claude, Some(home.path()))
        .await
        .expect("load");

    assert_eq!(
        files
            .iter()
            .map(|file| file.content.split(' ').next().unwrap_or_default())
            .collect::<Vec<_>>(),
        vec!["always", "one", "two", "three", "four"]
    );
}
