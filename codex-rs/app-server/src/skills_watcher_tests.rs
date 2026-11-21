// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::is_skill_change;

#[test]
fn only_changes_that_can_change_skills_count() {
    let home = TempDir::new().expect("temp dir");
    let root = home.path().join(".claude/skills");
    let system_root = home.path().join(".codex/skills/.system");
    let skill = root.join("demo");
    fs::create_dir_all(skill.join("scripts/__pycache__")).expect("create skill");
    fs::create_dir_all(root.join(".trash/old")).expect("create trash");
    fs::create_dir_all(skill.join(".pixi/envs")).expect("create environment");
    let roots = HashSet::from([root.clone()]);
    let counts = |path: PathBuf| is_skill_change(&path, &roots, &system_root);

    // The root itself lives under a hidden folder; only what is below it is looked at.
    assert_eq!(counts(root.clone()), true);
    assert_eq!(counts(skill.join("SKILL.md")), true);
    assert_eq!(counts(skill.join("reference/usage.md")), true);
    assert_eq!(counts(skill.clone()), true);
    assert_eq!(counts(root.join("removed-skill")), true);

    assert_eq!(counts(skill.join("scripts/run.py")), false);
    assert_eq!(counts(skill.join("scripts/__pycache__")), false);
    assert_eq!(
        counts(skill.join("scripts/__pycache__/run.cpython-312.pyc")),
        false
    );
    assert_eq!(counts(skill.join("node_modules/pkg/README.md")), false);
    assert_eq!(counts(skill.join(".pixi/envs")), false);
    assert_eq!(counts(skill.join(".pixi/envs/default/notes.md")), false);
    assert_eq!(counts(root.join(".trash/old/SKILL.md")), false);
    assert_eq!(counts(skill.join(".SKILL.md.swp")), false);
    assert_eq!(counts(system_root.join("demo/SKILL.md")), false);

    // A path under no known root is taken as a change.
    assert_eq!(counts(home.path().join("elsewhere/file.txt")), true);
}
