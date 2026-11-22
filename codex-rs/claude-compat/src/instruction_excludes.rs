// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The `claudeMdExcludes` setting, applied before instruction files and imports are read.

use std::path::Path;
use std::path::PathBuf;

use globset::GlobBuilder;
use globset::GlobSet;
use globset::GlobSetBuilder;

use crate::FileSource;
use crate::SourceFuture;
use crate::read_settings;
use crate::settings_paths;

/// Exclusion patterns from the settings that apply to an instruction load.
///
/// Wrapping the instruction source also filters transitive imports and rule files
/// without affecting other reads or Codex's own instruction discovery.
pub struct InstructionExcludes<'a> {
    source: &'a dyn FileSource,
    patterns: GlobSet,
}

impl<'a> InstructionExcludes<'a> {
    pub fn load(
        source: &'a dyn FileSource,
        claude_home: Option<&Path>,
        home_dir: Option<&Path>,
        project_dir: Option<&Path>,
    ) -> Self {
        let mut builder = GlobSetBuilder::new();
        for path in settings_paths(claude_home, home_dir, project_dir) {
            let settings = match read_settings(&path) {
                Ok(Some(settings)) => settings,
                Ok(None) => continue,
                Err(err) => {
                    tracing::warn!(
                        "failed to read Claude Code settings {}: {err}",
                        path.display()
                    );
                    continue;
                }
            };
            if let Some(patterns) = settings
                .get("claudeMdExcludes")
                .and_then(serde_json::Value::as_array)
            {
                for pattern in patterns.iter().filter_map(serde_json::Value::as_str) {
                    match GlobBuilder::new(pattern).literal_separator(true).build() {
                        Ok(glob) => {
                            builder.add(glob);
                        }
                        Err(err) => tracing::warn!(
                            "ignoring invalid claudeMdExcludes glob in {}: {err}",
                            path.display()
                        ),
                    }
                }
            }
        }
        let patterns = builder.build().unwrap_or_else(|err| {
            tracing::warn!("could not compile claudeMdExcludes: {err}");
            GlobSet::empty()
        });
        Self { source, patterns }
    }
}

impl FileSource for InstructionExcludes<'_> {
    fn read<'a>(&'a self, path: &'a Path) -> SourceFuture<'a, Option<String>> {
        if self.patterns.is_empty() {
            return self.source.read(path);
        }
        Box::pin(async move {
            if self.patterns.is_match(path)
                || self
                    .source
                    .canonicalize(path)
                    .await
                    .ok()
                    .flatten()
                    .is_some_and(|target| self.patterns.is_match(target))
            {
                return Ok(None);
            }
            self.source.read(path).await
        })
    }

    fn list<'a>(&'a self, dir: &'a Path) -> SourceFuture<'a, Vec<(PathBuf, bool)>> {
        self.source.list(dir)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::*;
    use crate::LocalFiles;
    use crate::file_instructions;

    #[tokio::test]
    async fn user_and_project_patterns_filter_instructions_rules_and_imports() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("home");
        let profile = tmp.path().join("profile");
        let project = tmp.path().join("project");
        let source_file = project.join("src/main.rs");
        let instructions = project.join("src/CLAUDE.md");
        let imported = project.join("src/personal.md");
        let rule = project.join("src/.claude/rules/hidden.md");
        fs::create_dir_all(rule.parent().unwrap()).unwrap();
        fs::create_dir_all(project.join(".claude")).unwrap();
        fs::create_dir_all(&profile).unwrap();
        fs::write(&instructions, "keep this\n\n@personal.md").unwrap();
        fs::write(&imported, "no import").unwrap();
        fs::write(&rule, "---\npaths: '*.rs'\n---\nno rule").unwrap();
        fs::write(
            profile.join("settings.json"),
            json!({ "claudeMdExcludes": ["**/src/personal.md"] }).to_string(),
        )
        .unwrap();
        fs::write(
            project.join(".claude/settings.json"),
            json!({ "claudeMdExcludes": [rule] }).to_string(),
        )
        .unwrap();
        let source =
            InstructionExcludes::load(&LocalFiles, Some(&profile), Some(&home), Some(&project));
        let loaded = file_instructions(
            &source,
            None,
            Some(&home),
            &project,
            &source_file,
            /*include_project*/ true,
        )
        .await
        .unwrap();
        assert_eq!(
            loaded.into_iter().map(|file| file.path).collect::<Vec<_>>(),
            [instructions]
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn a_pattern_can_match_the_symlink_target() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("target.md");
        let link = tmp.path().join("CLAUDE.md");
        fs::write(&target, "instructions").unwrap();
        std::os::unix::fs::symlink(&target, &link).unwrap();
        fs::write(
            tmp.path().join("settings.json"),
            json!({ "claudeMdExcludes": ["**/target.md"] }).to_string(),
        )
        .unwrap();
        let source = InstructionExcludes::load(&LocalFiles, Some(tmp.path()), None, None);
        assert_eq!(source.read(&link).await.unwrap(), None);
    }
}
