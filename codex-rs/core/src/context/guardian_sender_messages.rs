//! Reviewer-only snapshot of sender instructions and preceding assistant context.
//! Each exchange shares the existing user-message budget; user evidence takes priority.

use super::ContextualUserFragment;
use codex_guardian_context::GuardianRootMessage;
pub(crate) use codex_guardian_reviewer::GuardianSenderExchange;
use codex_protocol::ThreadId;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ContentItemKind;
use codex_protocol::models::input_text;

/// A bounded fragment rendered once at admission and never added to the worker prompt.
pub(crate) struct GuardianSenderMessages {
    pub source: Option<ThreadId>,
    pub delivery: String,
    pub from_storage: bool,
    pub messages: Vec<GuardianSenderExchange>,
}

impl ContextualUserFragment for GuardianSenderMessages {
    fn content_kind(&self) -> ContentItemKind {
        ContentItemKind("guardian.sender_messages".to_owned())
    }

    fn role(&self) -> &'static str {
        "user"
    }

    fn markers(&self) -> (&'static str, &'static str) {
        Self::type_markers()
    }

    fn type_markers() -> (&'static str, &'static str) {
        (
            ">>> SENDER USER MESSAGES START\n",
            ">>> SENDER USER MESSAGES END\n",
        )
    }

    fn body(&self) -> ContentItem {
        let source = self
            .source
            .map(|id| id.to_string())
            .unwrap_or_else(|| "unavailable".to_owned());
        let mut text = format!(
            codex_prompt_texts::text!(
                "core/context/guardian_sender_messages/body/received_message_source_thread_source.md"
            ),
            self.delivery,
            source = source,
        );
        if self.from_storage {
            text.push_str(codex_prompt_texts::text!(
                "core/context/guardian_sender_messages/body/host_read_from_persisted_sender.md"
            ));
        }
        if self.messages.is_empty() {
            text.push_str(codex_prompt_texts::text!(
                "core/context/guardian_sender_messages/body/host_no_sender_user_messages.md"
            ));
        }
        let mut assistant_omitted = false;
        for message in &self.messages {
            let user = message
                .user
                .as_ref()
                .map(|text| GuardianRootMessage::User(text.clone()).render())
                .filter(|text| text.len() <= 900)
                .unwrap_or_else(|| {
                    codex_prompt_texts::text!(
                        "core/context/guardian_sender_messages/sender_unavailable.md"
                    )
                    .to_owned()
                });
            if let Some(assistant) = &message.assistant {
                let assistant = assistant.clone().render();
                if assistant.len() + user.len() <= 900 {
                    text.push_str(&assistant);
                } else {
                    assistant_omitted = true;
                }
            }
            text.push_str(&user);
        }
        if assistant_omitted {
            text.push_str(&GuardianRootMessage::IncompleteAssistantContext.render());
        }
        input_text(text)
    }
}
