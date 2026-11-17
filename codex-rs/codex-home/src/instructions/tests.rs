use std::fs;
use std::path::Path;

use codex_extension_api::Instructions;
use codex_extension_api::LoadedUserInstructions;
use codex_extension_api::UserInstructionsProvider;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::CodexHomeUserInstructionsProvider;
use super::DEFAULT_AGENTS_MD_FILENAME;
use super::LOCAL_AGENTS_MD_FILENAME;

fn provider(home: &TempDir) -> CodexHomeUserInstructionsProvider {
    CodexHomeUserInstructionsProvider::new(
        AbsolutePathBuf::try_from(home.path().to_path_buf()).expect("absolute temp dir"),
    )
}

fn expected(
    home: &TempDir,
    filename: &str,
    text: &str,
    warnings: Vec<String>,
) -> LoadedUserInstructions {
    LoadedUserInstructions {
        instructions: Some(Instructions {
            text: text.to_string(),
            source: Some(
                AbsolutePathBuf::try_from(home.path().join(filename))
                    .expect("absolute source path"),
            ),
        }),
        warnings,
    }
}

#[cfg(unix)]
fn create_symlink_loop(path: &Path) {
    std::os::unix::fs::symlink(
        path.file_name().expect("override path should have a name"),
        path,
    )
    .expect("create symlink loop");
}

// WINDOWS-REMNANT
#[cfg(windows)]
fn create_symlink_loop(path: &Path) {
    std::os::windows::fs::symlink_file(
        path.file_name().expect("override path should have a name"),
        path,
    )
    .expect("create symlink loop");
}

#[tokio::test]
async fn missing_files_return_no_instructions() {
    let home = TempDir::new().expect("temp dir");

    assert_eq!(
        provider(&home).load_user_instructions().await,
        LoadedUserInstructions::default()
    );
}

#[tokio::test]
async fn override_takes_precedence_over_default() {
    let home = TempDir::new().expect("temp dir");
    fs::write(home.path().join(DEFAULT_AGENTS_MD_FILENAME), "default").expect("write default");
    fs::write(home.path().join(LOCAL_AGENTS_MD_FILENAME), "override").expect("write override");

    assert_eq!(
        provider(&home).load_user_instructions().await,
        expected(&home, LOCAL_AGENTS_MD_FILENAME, "override", Vec::new())
    );
}

#[tokio::test]
async fn empty_override_falls_back_to_trimmed_default() {
    let home = TempDir::new().expect("temp dir");
    fs::write(home.path().join(LOCAL_AGENTS_MD_FILENAME), " \n\t").expect("write override");
    fs::write(
        home.path().join(DEFAULT_AGENTS_MD_FILENAME),
        "\n  default instructions  \n",
    )
    .expect("write default");

    assert_eq!(
        provider(&home).load_user_instructions().await,
        expected(
            &home,
            DEFAULT_AGENTS_MD_FILENAME,
            "default instructions",
            Vec::new()
        )
    );
}

#[tokio::test]
async fn directory_override_falls_back_to_default() {
    let home = TempDir::new().expect("temp dir");
    fs::create_dir(home.path().join(LOCAL_AGENTS_MD_FILENAME)).expect("create override directory");
    fs::write(home.path().join(DEFAULT_AGENTS_MD_FILENAME), "default").expect("write default");

    assert_eq!(
        provider(&home).load_user_instructions().await,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "default", Vec::new())
    );
}

#[tokio::test]
async fn recoverable_override_read_error_warns_and_falls_back_to_default() {
    let home = TempDir::new().expect("temp dir");
    let provider = provider(&home);
    let override_path = home.path().join(LOCAL_AGENTS_MD_FILENAME);
    create_symlink_loop(&override_path);
    fs::write(home.path().join(DEFAULT_AGENTS_MD_FILENAME), "default").expect("write default");
    let read_error = fs::read(&override_path).expect_err("symlink loop should not be readable");
    let warning = format!(
        "Failed to read global AGENTS.md instructions from `{}`: {read_error}",
        override_path.display()
    );

    assert_eq!(
        provider.load_user_instructions().await,
        expected(
            &home,
            DEFAULT_AGENTS_MD_FILENAME,
            "default",
            vec![warning.clone()]
        )
    );
    assert_eq!(
        provider.clone().load_user_instructions().await,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "default", Vec::new())
    );

    fs::remove_file(&override_path).expect("remove broken symlink");
    assert_eq!(
        provider.load_user_instructions().await,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "default", Vec::new())
    );
    create_symlink_loop(&override_path);
    assert_eq!(
        provider.load_user_instructions().await,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "default", vec![warning])
    );
}

#[tokio::test]
async fn failed_refresh_retains_last_success_until_recovery_or_removal() {
    let home = TempDir::new().expect("temp dir");
    let provider = provider(&home);
    let path = home.path().join(DEFAULT_AGENTS_MD_FILENAME);
    fs::write(&path, "initial").expect("write initial instructions");
    let initial = provider.load_user_instructions().await;

    fs::remove_file(&path).expect("remove initial file");
    create_symlink_loop(&path);
    let failed = provider.load_user_instructions().await;
    assert_eq!(failed.instructions, initial.instructions);
    assert_eq!(failed.warnings.len(), 1);
    assert_eq!(provider.load_user_instructions().await, initial);

    fs::remove_file(&path).expect("remove broken symlink");
    fs::write(&path, "recovered").expect("restore instructions");
    assert_eq!(
        provider.load_user_instructions().await,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "recovered", Vec::new()),
    );

    fs::write(&path, " \n").expect("clear instructions");
    assert_eq!(
        provider.load_user_instructions().await,
        LoadedUserInstructions::default(),
    );
    fs::remove_file(&path).expect("remove cleared instructions");
    create_symlink_loop(&path);
    let failed_after_clear = provider.load_user_instructions().await;
    assert_eq!(failed_after_clear.instructions, None);
    assert_eq!(failed_after_clear.warnings.len(), 1);
    fs::remove_file(&path).expect("remove broken symlink");
    assert_eq!(
        provider.load_user_instructions().await,
        LoadedUserInstructions::default(),
    );
}

#[tokio::test]
async fn invalid_utf8_is_lossy() {
    let home = TempDir::new().expect("temp dir");
    let path = home.path().join(DEFAULT_AGENTS_MD_FILENAME);
    let mut invalid_utf8 = b"global".to_vec();
    invalid_utf8.push(0xff);
    invalid_utf8.extend_from_slice(b" doc");
    fs::write(&path, &invalid_utf8).expect("write invalid utf-8");

    let outcome = provider(&home).load_user_instructions().await;
    assert_eq!(
        outcome,
        expected(
            &home,
            DEFAULT_AGENTS_MD_FILENAME,
            "global\u{fffd} doc",
            Vec::new()
        )
    );
}

fn provider_with_claude(
    home: &TempDir,
    claude_home: &TempDir,
) -> CodexHomeUserInstructionsProvider {
    provider(home).with_claude_home(Some(
        AbsolutePathBuf::try_from(claude_home.path().to_path_buf()).expect("absolute temp dir"),
    ))
}

#[tokio::test]
async fn claude_instructions_follow_agents_md_and_track_edits() {
    let home = TempDir::new().expect("temp dir");
    let claude_home = TempDir::new().expect("temp dir");
    let rules = claude_home.path().join("rules");
    fs::create_dir_all(rules.join("nested")).expect("create rules");
    fs::write(home.path().join(DEFAULT_AGENTS_MD_FILENAME), "codex rule").expect("write");
    fs::write(claude_home.path().join("CLAUDE.md"), "claude rule\n").expect("write");
    fs::write(
        rules.join("b.md"),
        "---\npaths: [\"**/*.ipynb\"]\n---\n\nnotebook rule\n",
    )
    .expect("write");
    fs::write(rules.join("a.md"), "plain rule").expect("write");
    fs::write(
        rules.join("nested/c.md"),
        "---\npaths:\n  - \"src/**\"\n  - \"*.rs\"\n---\nlisted rule",
    )
    .expect("write");
    fs::write(rules.join("notes.txt"), "not a rule").expect("write");
    let provider = provider_with_claude(&home, &claude_home);

    let loaded = provider.load_user_instructions().await;
    let instructions = loaded.instructions.expect("instructions");
    let claude_md = claude_home.path().join("CLAUDE.md");
    let expected_tail = format!(
        "## {}\n\nclaude rule\n\n\
         ## {}\n\nplain rule\n\n\
         ## {}\n\nApplies only when working with files matching: [\"**/*.ipynb\"]\n\nnotebook rule\n\n\
         ## {}\n\nApplies only when working with files matching: \"src/**\", \"*.rs\"\n\nlisted rule",
        claude_md.display(),
        rules.join("a.md").display(),
        rules.join("b.md").display(),
        rules.join("nested/c.md").display(),
    );
    assert!(instructions.text.starts_with("codex rule\n\n"));
    assert!(
        instructions.text.ends_with(&expected_tail),
        "{}",
        instructions.text
    );
    assert_eq!(
        instructions.source.as_ref().map(AbsolutePathBuf::as_path),
        Some(home.path().join(DEFAULT_AGENTS_MD_FILENAME).as_path())
    );
    assert_eq!(loaded.warnings, Vec::<String>::new());

    // An edit or a removal on the Claude Code side shows at the next load.
    fs::write(&claude_md, "changed rule").expect("write");
    fs::remove_file(rules.join("a.md")).expect("remove");
    let text = provider
        .load_user_instructions()
        .await
        .instructions
        .expect("instructions")
        .text;
    assert!(text.contains("changed rule") && !text.contains("claude rule"));
    assert!(!text.contains("plain rule"));
}

#[tokio::test]
async fn claude_instructions_alone_report_claude_md_as_source() {
    let home = TempDir::new().expect("temp dir");
    let claude_home = TempDir::new().expect("temp dir");
    fs::write(claude_home.path().join("CLAUDE.md"), "claude rule").expect("write");

    let loaded = provider_with_claude(&home, &claude_home)
        .load_user_instructions()
        .await;

    let instructions = loaded.instructions.expect("instructions");
    assert!(instructions.text.ends_with("claude rule"));
    assert_eq!(
        instructions.source.as_ref().map(AbsolutePathBuf::as_path),
        Some(claude_home.path().join("CLAUDE.md").as_path())
    );
}

#[tokio::test]
async fn missing_claude_home_leaves_instructions_unchanged() {
    let home = TempDir::new().expect("temp dir");
    let claude_home = TempDir::new().expect("temp dir");
    fs::write(home.path().join(DEFAULT_AGENTS_MD_FILENAME), "codex rule").expect("write");

    let loaded = provider_with_claude(&home, &claude_home)
        .load_user_instructions()
        .await;

    assert_eq!(
        loaded,
        expected(&home, DEFAULT_AGENTS_MD_FILENAME, "codex rule", Vec::new())
    );
}
