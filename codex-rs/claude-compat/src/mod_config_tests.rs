use std::fs;

use pretty_assertions::assert_eq;

use super::ClaudeModConfig;
use super::ClaudePluginsModConfig;
use super::MOD_CONFIG_FILE;
use super::ModConfig;

#[test]
fn parses_every_key() {
    let config = ModConfig::parse(
        r#"
[claude]
tools = true

[claude.plugins]
enable = ["claude-hud@claude-hud", "other@m"]
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
fn unknown_keys_and_wrong_types_are_errors() {
    assert!(ModConfig::parse("[claude]\ntool = true\n").is_err());
    assert!(ModConfig::parse("[other]\n").is_err());
    assert!(ModConfig::parse("[claude]\ntools = \"yes\"\n").is_err());
    assert!(ModConfig::parse("[claude.plugins]\nenable = \"a@m\"\n").is_err());
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
