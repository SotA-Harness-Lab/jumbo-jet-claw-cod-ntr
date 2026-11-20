// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::fs;

use pretty_assertions::assert_eq;

use super::ClaudeModConfig;
use super::ClaudePluginsModConfig;
use super::MOD_CONFIG_FILE;
use super::ModConfig;
use super::UiModConfig;

#[test]
fn parses_every_key() {
    let config = ModConfig::parse(
        r#"
[claude]
tools = true

[claude.plugins]
enable = ["claude-hud@claude-hud", "other@m"]

[ui]
language = "zh-CN"
"#,
    )
    .expect("parse");

    assert_eq!(
        config,
        ModConfig {
            claude: ClaudeModConfig {
                tools: true,
                plugins: ClaudePluginsModConfig {
                    enable: vec!["claude-hud@claude-hud".to_string(), "other@m".to_string()],
                },
            },
            ui: UiModConfig {
                language: Some("zh-CN".to_string()),
            },
            ..Default::default()
        }
    );
}

#[test]
fn missing_keys_are_the_defaults() {
    assert_eq!(ModConfig::parse("").expect("parse"), ModConfig::default());
    assert_eq!(
        ModConfig::parse("[claude]\n").expect("parse"),
        ModConfig::default()
    );
    assert!(!ModConfig::default().claude.tools);
}

#[test]
fn wrong_types_are_errors() {
    assert!(ModConfig::parse("[claude]\ntools = \"yes\"\n").is_err());
    assert!(ModConfig::parse("[claude.plugins]\nenable = \"a@m\"\n").is_err());
}

#[test]
fn unknown_keys_and_sections_are_left_out_and_named() {
    let (config, unknown) = ModConfig::parse_with_unknown_keys(
        "[claude]\ntools = true\ntool = 1\n\n[harness]\noutput_limit_continuations = 3\n\n[claude.plugins]\nextra = []\n",
    )
    .expect("a file with settings of another build parses");

    assert_eq!(
        config,
        ModConfig {
            claude: ClaudeModConfig {
                tools: true,
                ..Default::default()
            },
            ..Default::default()
        }
    );
    assert_eq!(
        unknown,
        vec!["claude.plugins.extra", "claude.tool", "harness"]
    );
}

#[test]
fn a_file_with_unknown_keys_loads_without_a_warning() {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(
        tmp.path().join(MOD_CONFIG_FILE),
        "[claude]\ntools = true\n\n[harness]\nanything = 1\n",
    )
    .expect("write");

    let (config, warning) = ModConfig::load(tmp.path());

    assert!(config.claude.tools);
    assert_eq!(warning, None);
}

#[test]
fn a_missing_file_is_the_defaults_without_a_warning() {
    let tmp = tempfile::tempdir().expect("tempdir");

    assert_eq!(ModConfig::load(tmp.path()), (ModConfig::default(), None));
}

#[test]
fn a_file_that_does_not_parse_is_the_defaults_with_a_warning() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join(MOD_CONFIG_FILE);
    fs::write(&path, "[claude]\ntools = tru\n").expect("write");

    let (config, warning) = ModConfig::load(tmp.path());

    assert_eq!(config, ModConfig::default());
    let warning = warning.expect("warning");
    assert!(
        warning.starts_with(&format!(
            "Ignoring {}, which is not valid: ",
            path.display()
        )),
        "{warning}"
    );
}

#[test]
fn a_valid_file_is_read() {
    let tmp = tempfile::tempdir().expect("tempdir");
    fs::write(tmp.path().join(MOD_CONFIG_FILE), "[claude]\ntools = true\n").expect("write");

    let (config, warning) = ModConfig::load(tmp.path());

    assert!(config.claude.tools);
    assert_eq!(warning, None);
}

#[test]
fn a_hook_s_claude_model_is_mapped_by_name_or_by_part_of_it() {
    let config = ModConfig::parse(
        "[hooks.models]\nhaiku = \"small\"\n\"claude-sonnet-4-6\" = \"medium\"\nsonnet = \"other\"\ndefault = \"tiny\"\nopus = \" \"\n",
    )
    .expect("parse");
    let hooks = &config.hooks;

    // The whole name first, then the longest name it contains.
    assert_eq!(hooks.model_for(Some("claude-sonnet-4-6")), Some("medium"));
    assert_eq!(hooks.model_for(Some("claude-sonnet-5")), Some("other"));
    assert_eq!(hooks.model_for(Some("Claude-Haiku-4-5")), Some("small"));
    assert_eq!(hooks.model_for(Some("haiku")), Some("small"));
    // A handler that names no model takes `default`, which names no part of a model.
    assert_eq!(hooks.model_for(None), Some("tiny"));
    assert_eq!(hooks.model_for(Some(" ")), Some("tiny"));
    // No entry, or an empty one: the handler's model is ignored.
    assert_eq!(hooks.model_for(Some("claude-opus-5")), None);
    assert_eq!(hooks.model_for(Some("gemini-default-x")), None);
    assert_eq!(ModConfig::default().hooks.model_for(Some("haiku")), None);
    assert_eq!(ModConfig::default().hooks.model_for(None), None);
}
