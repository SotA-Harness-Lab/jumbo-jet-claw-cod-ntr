// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::plan_restore;
use super::record;
use super::restore;

fn turns(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| (*id).to_string()).collect()
}

/// A workspace where turn 1 edited `a` and created `b`, and turn 2 edited `a` again and
/// deleted `c`.
fn two_turns(tmp: &TempDir) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let home = tmp.path().join("home");
    let (a, b, c) = (
        tmp.path().join("work/a.txt"),
        tmp.path().join("work/b.txt"),
        tmp.path().join("work/c.txt"),
    );
    fs::create_dir_all(tmp.path().join("work")).expect("create");
    record(
        &home,
        "thread",
        "turn-1",
        [(a.clone(), Some("a0".to_string())), (b.clone(), None)],
        false,
    )
    .expect("record");
    // A second edit of `a` in the same turn does not replace what the turn started from.
    record(&home, "thread", "turn-1", [(a.clone(), Some("a1".to_string()))], false)
        .expect("record");
    record(
        &home,
        "thread",
        "turn-2",
        [(a.clone(), Some("a2".to_string())), (c.clone(), Some("c0".to_string()))],
        false,
    )
    .expect("record");
    fs::write(&a, "a3").expect("write");
    fs::write(&b, "b1").expect("write");
    (home, a, b, c)
}

#[test]
fn going_back_gives_each_file_what_the_earliest_turn_replaced() {
    let tmp = TempDir::new().expect("temp dir");
    let (home, a, b, c) = two_turns(&tmp);

    let plan = plan_restore(&home, "thread", &turns(&["turn-1", "turn-2"]));
    assert_eq!(
        plan.files,
        vec![
            (a.clone(), Some("a0".to_string())),
            (b.clone(), None),
            (c.clone(), Some("c0".to_string())),
        ]
    );
    assert_eq!(plan.incomplete, false);

    assert_eq!(restore(&plan).len(), 0);
    assert_eq!(fs::read_to_string(&a).expect("read"), "a0");
    assert_eq!(b.exists(), false);
    assert_eq!(fs::read_to_string(&c).expect("read"), "c0");
    // Nothing is left to do once the files are back.
    assert_eq!(
        plan_restore(&home, "thread", &turns(&["turn-1", "turn-2"])).files,
        Vec::new()
    );
}

#[test]
fn going_back_to_a_later_turn_leaves_earlier_edits_alone() {
    let tmp = TempDir::new().expect("temp dir");
    let (home, a, _b, c) = two_turns(&tmp);

    let plan = plan_restore(&home, "thread", &turns(&["turn-2"]));
    assert_eq!(
        plan.files,
        vec![(a, Some("a2".to_string())), (c, Some("c0".to_string()))]
    );
}

#[test]
fn a_turn_without_a_checkpoint_changes_nothing_and_a_flawed_one_is_named() {
    let tmp = TempDir::new().expect("temp dir");
    let home = tmp.path().join("home");
    assert_eq!(
        plan_restore(&home, "thread", &turns(&["turn-9"])),
        Default::default()
    );

    record(&home, "thread", "turn-1", [], true).expect("record");
    assert_eq!(plan_restore(&home, "thread", &turns(&["turn-1"])).incomplete, true);
}
