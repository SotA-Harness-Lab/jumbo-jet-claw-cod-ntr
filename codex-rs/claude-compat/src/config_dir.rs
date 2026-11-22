// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use crate::CLAUDE_CONFIG_DIR_ENV;
use crate::CLAUDE_DIR;

/// The user's Claude Code directory for this process.
pub fn user_config_dir() -> Option<PathBuf> {
    user_config_dir_for(
        dirs::home_dir().as_deref(),
        std::env::var_os(CLAUDE_CONFIG_DIR_ENV).as_deref(),
    )
}

/// Resolve a launcher-provided directory independently of any project settings.
pub fn user_config_dir_for(home: Option<&Path>, override_dir: Option<&OsStr>) -> Option<PathBuf> {
    configured_dir(override_dir).or_else(|| home.map(|home| home.join(CLAUDE_DIR)))
}

/// Claude Code's global state file, which also contains user MCP servers.
pub fn user_state_file() -> Option<PathBuf> {
    configured_dir(std::env::var_os(CLAUDE_CONFIG_DIR_ENV).as_deref())
        .or_else(dirs::home_dir)
        .map(|dir| dir.join(".claude.json"))
}

fn configured_dir(value: Option<&OsStr>) -> Option<PathBuf> {
    let path = Path::new(value.filter(|path| !path.is_empty())?);
    std::path::absolute(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_override_is_independent_of_home() {
        let home = std::env::temp_dir().join("home");
        let profile = std::env::temp_dir().join("profiles").join("work");
        assert_eq!(
            user_config_dir_for(None, Some(profile.as_os_str())),
            Some(profile)
        );
        assert_eq!(
            user_config_dir_for(Some(&home), Some(OsStr::new(""))),
            Some(home.join(CLAUDE_DIR))
        );
        assert_eq!(user_config_dir_for(None, None), None);
    }
}
