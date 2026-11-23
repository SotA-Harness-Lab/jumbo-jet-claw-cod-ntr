use crate::catalog::SkillSourceKind;

const SKILLS_INTRO_WITH_SOURCE_LOCATORS: &str =
    codex_prompt_texts::text!("ext/skills/catalog_prompt/skills_intro_with_source_locators.md");
const SKILLS_INTRO_WITH_HOST_ALIASES: &str =
    codex_prompt_texts::text!("ext/skills/catalog_prompt/skills_intro_with_host_aliases.md");
const SKILLS_INTRO_WITH_RESOURCE_ALIASES: &str =
    codex_prompt_texts::text!("ext/skills/catalog_prompt/skills_intro_with_resource_aliases.md");
const RESOURCE_ALIAS_INSTRUCTIONS: &str =
    codex_prompt_texts::text!("ext/skills/catalog_prompt/resource_alias_instructions.md");
const SKILLS_HOW_TO_USE_WITH_SOURCE_LOCATORS: &str = codex_prompt_texts::text!(
    "ext/skills/catalog_prompt/skills_how_to_use_with_source_locators.md"
);
const SKILLS_HOW_TO_USE_WITH_HOST_ALIASES: &str =
    codex_prompt_texts::text!("ext/skills/catalog_prompt/skills_how_to_use_with_host_aliases.md");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SkillPromptKind {
    Unaliased,
    HostAliases,
    ResourceAliases,
    ResourceAliasesV2,
}

impl SkillPromptKind {
    pub(crate) fn for_aliased_source(source: &SkillSourceKind) -> Self {
        match source {
            SkillSourceKind::Host => Self::HostAliases,
            SkillSourceKind::Executor | SkillSourceKind::Cloud => Self::ResourceAliases,
            SkillSourceKind::Custom(_) => Self::Unaliased,
        }
    }

    fn intro(self) -> &'static str {
        match self {
            Self::Unaliased => SKILLS_INTRO_WITH_SOURCE_LOCATORS,
            Self::HostAliases => SKILLS_INTRO_WITH_HOST_ALIASES,
            Self::ResourceAliases | Self::ResourceAliasesV2 => SKILLS_INTRO_WITH_RESOURCE_ALIASES,
        }
    }

    pub(crate) fn alias_instructions(self) -> Option<&'static str> {
        match self {
            Self::ResourceAliases | Self::ResourceAliasesV2 => Some(RESOURCE_ALIAS_INSTRUCTIONS),
            Self::Unaliased | Self::HostAliases => None,
        }
    }
}

/// Prompt layout and description policy shared by rendering and budget accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SkillPrompt {
    pub(crate) kind: SkillPromptKind,
    pub(crate) omit_descriptions: bool,
}

impl SkillPrompt {
    pub(crate) fn usage_instructions(self) -> String {
        let instructions = match self.kind {
            SkillPromptKind::Unaliased
            | SkillPromptKind::ResourceAliases
            | SkillPromptKind::ResourceAliasesV2 => SKILLS_HOW_TO_USE_WITH_SOURCE_LOCATORS,
            SkillPromptKind::HostAliases => SKILLS_HOW_TO_USE_WITH_HOST_ALIASES,
        };
        if self.omit_descriptions {
            instructions
                .replace("name + description +", "name +")
                .replace("description shown above", "name shown above")
        } else {
            instructions.to_owned()
        }
    }
}

pub(crate) fn render_available_skills_body(
    prompt: SkillPrompt,
    skill_root_lines: &[String],
    skill_lines: &[String],
) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("## Skills".to_string());
    let intro = prompt.kind.intro();
    lines.push(if prompt.omit_descriptions {
        intro.replace("name, description, and", "name and")
    } else {
        intro.to_string()
    });
    if !skill_root_lines.is_empty() {
        lines.push("### Skill roots".to_string());
        if prompt.kind == SkillPromptKind::ResourceAliasesV2 {
            lines.push(
                codex_prompt_texts::text!("ext/skills/catalog_prompt/skill_roots_mapping_note.md")
                    .to_string(),
            );
        }
        lines.extend(skill_root_lines.iter().cloned());
    }
    if skill_lines
        .iter()
        .any(|line| line.contains("(executor package: ") || line.contains("(cloud package: "))
    {
        lines.push(
            codex_prompt_texts::text!(
                "ext/skills/catalog_prompt/skill_package_read_instructions.md"
            )
            .to_string(),
        );
    }
    lines.push("### Available skills".to_string());
    lines.extend(skill_lines.iter().cloned());

    format!("\n{}\n", lines.join("\n"))
}
