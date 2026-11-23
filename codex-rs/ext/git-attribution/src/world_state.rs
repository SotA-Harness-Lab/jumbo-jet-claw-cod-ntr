use codex_extension_api::PreviousWorldStateSection;
use codex_extension_api::RenderedWorldStateFragment;
use codex_extension_api::WorldStateSectionContribution;
use codex_extension_api::input_text;
use serde_json::Value;

const WORLD_STATE_ID: &str = "git_attribution";
const START_MARKER: &str = "<git_attribution>";
const END_MARKER: &str = "</git_attribution>";
pub(super) const LEGACY_COMMIT_ATTRIBUTION_INSTRUCTIONS: &str = codex_prompt_texts::text!(
    "ext/git-attribution/world_state/legacy_commit_attribution_instructions.md"
);
pub(super) const ENABLED_INSTRUCTIONS: &str =
    codex_prompt_texts::text!("ext/git-attribution/world_state/enabled_instructions.md");
pub(super) const DISABLED_INSTRUCTIONS: &str =
    codex_prompt_texts::text!("ext/git-attribution/world_state/disabled_instructions.md");

pub(super) fn git_attribution_world_state_section(enabled: bool) -> WorldStateSectionContribution {
    let contribution = WorldStateSectionContribution::new(WORLD_STATE_ID, move |previous| {
        let fragment = match (enabled, previous) {
            (true, PreviousWorldStateSection::Known(Value::Bool(true)))
            | (true, PreviousWorldStateSection::Unknown) => None,
            (true, PreviousWorldStateSection::Absent)
            | (true, PreviousWorldStateSection::Known(_)) => Some(RenderedWorldStateFragment::new(
                "developer",
                (START_MARKER, END_MARKER),
                input_text(ENABLED_INSTRUCTIONS),
            )),
            (false, PreviousWorldStateSection::Known(Value::Bool(true)))
            | (false, PreviousWorldStateSection::Unknown) => Some(RenderedWorldStateFragment::new(
                "developer",
                (START_MARKER, END_MARKER),
                input_text(DISABLED_INSTRUCTIONS),
            )),
            (false, PreviousWorldStateSection::Absent)
            | (false, PreviousWorldStateSection::Known(_)) => None,
        };
        (Some(Value::Bool(enabled)), fragment)
    })
    .with_legacy_matcher(move |role, text| {
        is_enabled_fragment(role, text)
            || (!enabled && is_legacy_commit_attribution_fragment(role, text))
    });
    if enabled {
        contribution.with_retained_fragment_matcher(is_enabled_fragment)
    } else {
        contribution
    }
}

fn is_legacy_commit_attribution_fragment(role: &str, text: &str) -> bool {
    role == "developer" && text.trim() == LEGACY_COMMIT_ATTRIBUTION_INSTRUCTIONS
}

fn is_enabled_fragment(role: &str, text: &str) -> bool {
    role == "developer"
        && text.trim_start().starts_with(START_MARKER)
        && text.contains("Co-authored-by: Codex <noreply@openai.com>")
        && (text.contains("Generated with [Codex](https://openai.com/codex/).")
            || text.contains("Generated with Codex."))
        && text.trim_end().ends_with(END_MARKER)
}
