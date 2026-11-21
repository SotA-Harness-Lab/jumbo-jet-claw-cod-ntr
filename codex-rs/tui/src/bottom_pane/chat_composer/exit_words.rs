// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Words that end the session when submitted on their own, as in Claude Code: `exit`, `quit`,
//! Vim's `:q`, `:q!`, `:wq` and `:wq!`, and the `/exit` and `/quit` commands. They are handed to
//! the app at once, also while a turn is running, instead of being sent to the model or queued
//! behind the turn; the app asks before quitting when work is still running.

use super::ChatComposer;
use crate::slash_command::SlashCommand;

const EXIT_WORDS: &[&str] = &["exit", "quit", ":q", ":q!", ":wq", ":wq!", "/exit", "/quit"];

pub(super) fn is_exit_word(text: &str) -> bool {
    EXIT_WORDS.contains(&text.trim())
}

impl ChatComposer {
    /// Clears the draft and returns the exit command when the draft is one of the exit words.
    ///
    /// A submission that will be queued has not flushed the paste burst yet, so the word may
    /// still be in it: flush it first, as the queueing path would.
    pub(super) fn take_exit_word(&mut self, should_queue: bool) -> Option<SlashCommand> {
        if !self.slash_commands_enabled() {
            return None;
        }
        if should_queue && let Some(pasted) = self.draft.paste_burst.flush_before_modified_input() {
            self.apply_paste(pasted);
        }
        if self.draft.is_bash_mode || !is_exit_word(self.draft.textarea.text()) {
            return None;
        }
        self.draft.textarea.set_text_clearing_elements("");
        Some(SlashCommand::Exit)
    }
}

#[cfg(test)]
mod tests {
    use super::super::InputResult;
    use super::super::tests::new_test_composer;
    use super::is_exit_word;
    use crate::slash_command::SlashCommand;

    #[test]
    fn matches_whole_words_only() {
        for word in ["exit", " quit ", ":q", ":q!", ":wq", ":wq!", "/exit", "/quit\n"] {
            assert!(is_exit_word(word), "{word:?}");
        }
        for text in ["Exit", "exit now", "please quit", ":x", "/exit now", "", "exit\nmore"] {
            assert!(!is_exit_word(text), "{text:?}");
        }
    }

    #[test]
    fn exit_word_quits_also_while_a_turn_runs() {
        for running in [false, true] {
            for word in ["exit", ":wq", "/exit"] {
                let (mut composer, _rx) = new_test_composer();
                composer.set_task_running(running);
                composer.set_text_content(word.to_string(), Vec::new(), Vec::new());
                let (result, _) = composer.handle_submission(/*should_queue*/ running);
                assert_eq!(result, InputResult::Command(SlashCommand::Exit), "{word:?}");
                assert!(composer.is_empty());
            }
        }
    }

    #[test]
    fn other_text_is_still_submitted() {
        let (mut composer, _rx) = new_test_composer();
        composer.set_text_content("exit now".to_string(), Vec::new(), Vec::new());
        let (result, _) = composer.handle_submission(/*should_queue*/ false);
        assert_eq!(
            result,
            InputResult::Submitted {
                text: "exit now".to_string(),
                text_elements: Vec::new(),
            }
        );
    }
}
