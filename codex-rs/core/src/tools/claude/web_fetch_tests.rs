// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use codex_exec_server::LOCAL_FS;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;
use url::Url;

use super::BODY_FILE_NAME;
use super::prepare_body_file;
use super::remove_body_directory;

fn temporary_directory() -> (tempfile::TempDir, PathUri) {
    let directory = tempfile::tempdir().expect("tempdir");
    let uri = PathUri::from_host_native_path(directory.path()).expect("absolute path");
    (directory, uri)
}

fn native(uri: &PathUri) -> PathBuf {
    PathBuf::from(uri.inferred_native_path_string())
}

fn url() -> Url {
    Url::parse("https://example.com/page").expect("url")
}

#[tokio::test]
async fn body_file_is_in_a_directory_of_its_own_that_is_created_anew() {
    let (directory, uri) = temporary_directory();
    let file = prepare_body_file(LOCAL_FS.as_ref(), &uri, &url())
        .await
        .expect("body file");
    let own = native(&file.parent().expect("parent"));
    assert_eq!(
        own.parent().map(std::path::Path::to_path_buf),
        Some(std::fs::canonicalize(directory.path()).expect("canonical"))
    );
    assert!(own.is_dir());
    assert_eq!(
        native(&file).file_name().and_then(|name| name.to_str()),
        Some(BODY_FILE_NAME)
    );

    // A body an earlier fetch left behind is cleared, and the same URL gets the same path.
    std::fs::write(native(&file), "stale").expect("write body");
    let again = prepare_body_file(LOCAL_FS.as_ref(), &uri, &url())
        .await
        .expect("body file");
    assert_eq!(again, file);
    assert!(own.is_dir());
    assert!(!native(&again).exists());

    remove_body_directory(LOCAL_FS.as_ref(), &file.parent().expect("parent")).await;
    assert!(!own.exists());
}

#[cfg(unix)]
#[tokio::test]
async fn body_file_is_not_offered_where_someone_else_put_a_link_first() {
    let (directory, uri) = temporary_directory();
    let file = prepare_body_file(LOCAL_FS.as_ref(), &uri, &url())
        .await
        .expect("body file");
    let own = native(&file.parent().expect("parent"));
    std::fs::remove_dir(&own).expect("remove own directory");

    let victim = directory.path().join("victim");
    std::fs::create_dir(&victim).expect("victim directory");
    std::os::unix::fs::symlink(&victim, &own).expect("plant link");

    assert_eq!(
        prepare_body_file(LOCAL_FS.as_ref(), &uri, &url()).await,
        None
    );
    assert!(
        std::fs::symlink_metadata(&own)
            .expect("link")
            .file_type()
            .is_symlink()
    );
    assert!(!victim.join(BODY_FILE_NAME).exists());
}
