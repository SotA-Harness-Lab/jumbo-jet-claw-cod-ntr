// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use pretty_assertions::assert_eq;

use super::footer;
use super::remove_stale_dirs;
use super::write_private;

#[test]
fn the_footer_says_whether_the_file_is_the_whole_output() {
    assert_eq!(
        footer(Path::new("/tmp/out.txt"), 0),
        "\n[Full output saved to: /tmp/out.txt - read or search that file for the part you need instead of running the command again]"
    );
    assert!(
        footer(Path::new("/tmp/out.txt"), 10)
            .starts_with("\n[Output as collected (its own middle was not kept) saved to: ")
    );
}

#[test]
fn output_is_written_for_the_user_alone() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let dir = tmp.path().join("root/session");
    let path = dir.join("chunk.txt");

    write_private(&dir, &path, b"all of it").expect("write");

    assert_eq!(std::fs::read_to_string(&path).expect("read"), "all of it");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = |path: &Path| {
            std::fs::metadata(path).expect("metadata").permissions().mode() & 0o777
        };
        assert_eq!((mode(&dir), mode(&path)), (0o700, 0o600));
    }
}

#[test]
fn only_other_sessions_long_gone_are_cleared_out() {
    let tmp = tempfile::tempdir().expect("temp dir");
    let old = tmp.path().join("old");
    let recent = tmp.path().join("recent");
    let current = tmp.path().join("current");
    for dir in [&old, &recent, &current] {
        std::fs::create_dir(dir).expect("create");
    }
    let long_ago = std::time::SystemTime::now() - std::time::Duration::from_secs(8 * 24 * 60 * 60);
    for dir in [&old, &current] {
        std::fs::File::open(dir)
            .and_then(|file| file.set_modified(long_ago))
            .expect("set modified");
    }

    remove_stale_dirs(tmp.path(), &current);

    assert_eq!(
        (old.exists(), recent.exists(), current.exists()),
        (false, true, true)
    );
}
