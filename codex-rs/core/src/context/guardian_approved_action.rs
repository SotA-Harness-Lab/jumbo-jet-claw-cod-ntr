use super::ContextualUserFragment;
use crate::guardian::AUTO_REVIEW_DENIED_ACTION_APPROVAL_DEVELOPER_PREFIX;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GuardianApprovedAction {
    approved_action_json: String,
}

impl GuardianApprovedAction {
    pub(crate) const KIND: &str = "guardian.approved_action";

    pub(crate) fn new(approved_action_json: String) -> Self {
        Self {
            approved_action_json,
        }
    }
}

impl ContextualUserFragment for GuardianApprovedAction {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind(Self::KIND.to_string())
    }

    fn role(&self) -> &'static str {
        "developer"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        ("", "")
    }

    fn body(&self) -> ContentItem {
        let approval_prefix = AUTO_REVIEW_DENIED_ACTION_APPROVAL_DEVELOPER_PREFIX;
        let approved_action_json = &self.approved_action_json;
        input_text(format!(
            codex_prompt_texts::text!("core/context/guardian_approved_action/body.md"),
            approval_prefix = approval_prefix,
            approved_action_json = approved_action_json
        ))
    }
}
