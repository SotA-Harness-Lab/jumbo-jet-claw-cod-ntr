// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::sync::Arc;

use codex_extension_api::ExtensionData;
use codex_protocol::protocol::SkillScope;
use codex_skills::SkillMetadata;
use codex_skills_extension::HostSkillsSnapshot;
use codex_skills_extension::SkillLoadOutcome;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;

use super::files_worked_on;
use super::offer_skills;

fn skill(dir: &Path, name: &str, frontmatter: &str) -> SkillMetadata {
    let path = dir.join(name).join("SKILL.md");
    std::fs::create_dir_all(dir.join(name)).expect("create skill");
    std::fs::write(
        &path,
        format!("---\nname: {name}\ndescription: {name}\n{frontmatter}---\nBody\n"),
    )
    .expect("write skill");
    SkillMetadata {
        name: name.to_string(),
        description: name.to_string(),
        short_description: None,
        interface: None,
        dependencies: None,
        policy: None,
        path_to_skills_md: PathUri::from_abs_path(
            &AbsolutePathBuf::from_absolute_path_checked(&path).expect("absolute"),
        ),
        scope: SkillScope::User,
        plugin_id: None,
        remote_plugin_id: None,
    }
}

#[tokio::test]
async fn a_skill_with_paths_is_offered_once_a_file_it_covers_is_worked_on() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let skills_dir = temp_dir.path().join("skills");
    let cwd = temp_dir.path().join("repo");
    let rust = skill(&skills_dir, "rust", "paths: src/**/*.rs\n");
    let always = skill(&skills_dir, "always", "");
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![rust.clone(), always.clone()];
    let snapshot = Arc::new(HostSkillsSnapshot::new(Arc::new(outcome)));
    let session_data = ExtensionData::new("session");
    let enabled = |offered: &HostSkillsSnapshot| {
        (
            offered.outcome().is_skill_enabled(&rust),
            offered.outcome().is_skill_enabled(&always),
        )
    };

    let offered = offer_skills(&session_data, &cwd, Arc::clone(&snapshot)).await;
    assert_eq!(enabled(&offered), (false, true));

    files_worked_on(
        &session_data,
        &cwd,
        &[cwd.join("docs/guide.md")],
        /*include_project*/ false,
    )
    .await;
    let offered = offer_skills(&session_data, &cwd, Arc::clone(&snapshot)).await;
    assert_eq!(enabled(&offered), (false, true));

    files_worked_on(
        &session_data,
        &cwd,
        &[cwd.join("src/net/client.rs")],
        /*include_project*/ false,
    )
    .await;
    let offered = offer_skills(&session_data, &cwd, Arc::clone(&snapshot)).await;
    assert_eq!(enabled(&offered), (true, true));
    // With nothing left to hold back, the snapshot is offered as it is.
    assert_eq!(Arc::ptr_eq(&offered, &snapshot), true);
}

#[tokio::test]
async fn files_worked_on_before_the_first_offer_count() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let cwd = temp_dir.path().join("repo");
    let rust = skill(
        &temp_dir.path().join("skills"),
        "rust",
        "paths: \"**/*.rs\"\n",
    );
    let mut outcome = SkillLoadOutcome::default();
    outcome.skills = vec![rust.clone()];
    let snapshot = Arc::new(HostSkillsSnapshot::new(Arc::new(outcome)));
    let session_data = ExtensionData::new("session");

    files_worked_on(
        &session_data,
        &cwd,
        &[cwd.join("main.rs")],
        /*include_project*/ false,
    )
    .await;
    let offered = offer_skills(&session_data, &cwd, snapshot).await;
    assert_eq!(offered.outcome().is_skill_enabled(&rust), true);
}
