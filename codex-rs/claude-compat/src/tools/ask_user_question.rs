// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Claude Code's `AskUserQuestion` tool: description, the checks on its questions, their
//! translation to Codex's `request_user_input` and the result text built from its answers.

use std::collections::HashSet;

use serde_json::Value;
use serde_json::json;

pub const QUESTIONS_PARAM_DESCRIPTION: &str = "Questions to ask the user (1-4 questions)";
pub const QUESTION_PARAM_DESCRIPTION: &str = "The complete question to ask the user. Should be clear, specific, and end with a question mark. Example: \"Which library should we use for date formatting?\" If multiSelect is true, phrase it accordingly, e.g. \"Which features do you want to enable?\"";
pub const HEADER_PARAM_DESCRIPTION: &str = "Very short label displayed as a chip/tag (max 12 chars). Examples: \"Auth method\", \"Library\", \"Approach\".";
pub const OPTIONS_PARAM_DESCRIPTION: &str = "The available choices for this question. Must have 2-4 options. Each option should be a distinct, mutually exclusive choice (unless multiSelect is enabled). There should be no 'Other' option, that will be provided automatically.";
pub const LABEL_PARAM_DESCRIPTION: &str = "The display text for this option that the user will see and select. Should be concise (1-5 words) and clearly describe the choice.";
pub const OPTION_DESCRIPTION_PARAM_DESCRIPTION: &str = "Explanation of what this option means or what will happen if chosen. Useful for providing context about trade-offs or implications.";
pub const MULTI_SELECT_PARAM_DESCRIPTION: &str = "Set to true to allow the user to select multiple options instead of just one. Use when choices are not mutually exclusive.";

/// What Codex's answer list marks as the user's own words.
const USER_NOTE_PREFIX: &str = "user_note: ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestionOption {
    pub label: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Question {
    pub question: String,
    pub header: String,
    pub options: Vec<QuestionOption>,
    pub multi_select: bool,
}

/// One answered question, as Claude Code reports it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    pub question: String,
    /// The chosen labels (joined by `, ` for several), or the user's own text.
    pub answer: String,
    /// What the user wrote beside a chosen option.
    pub notes: Option<String>,
}

/// The tool description, after Claude Code's (without its notes on plan mode and previews,
/// which Codex does not have).
pub fn description() -> String {
    "Use this tool when you need to ask the user questions during execution. This allows you to:
1. Gather user preferences or requirements
2. Clarify ambiguous instructions
3. Get decisions on implementation choices as you work
4. Offer choices to the user about what direction to take.

Usage notes:
- Users will always be able to select \"Other\" to provide custom text input
- Use multiSelect: true to allow multiple answers to be selected for a question
- If you recommend a specific option, make that the first option in the list and add \"(Recommended)\" at the end of the label
"
    .to_string()
}

/// Claude Code's checks beyond the types: 1 to 4 questions of 2 to 4 options, question texts
/// unique and option labels unique within a question.
pub fn validate(questions: &[Question]) -> Result<(), String> {
    if questions.is_empty() || questions.len() > 4 {
        return Err("questions: must contain between 1 and 4 questions".to_string());
    }
    for question in questions {
        if question.options.len() < 2 || question.options.len() > 4 {
            return Err(format!(
                "questions: \"{}\" must have between 2 and 4 options",
                question.question
            ));
        }
    }
    let mut texts = HashSet::new();
    let unique = questions.iter().all(|question| {
        let mut labels = HashSet::new();
        texts.insert(question.question.as_str())
            && question
                .options
                .iter()
                .all(|option| labels.insert(option.label.as_str()))
    });
    if !unique {
        return Err(
            "Question texts must be unique, option labels must be unique within each question"
                .to_string(),
        );
    }
    Ok(())
}

/// The id of the `request_user_input` question standing for the question at `index`.
pub fn question_id(index: usize) -> String {
    format!("question_{}", index + 1)
}

/// The `request_user_input` arguments for the questions. Codex offers one choice a question,
/// so a multiple-choice question asks for the other choices in the user's note.
pub fn request_user_input_arguments(questions: &[Question]) -> Value {
    let questions = questions
        .iter()
        .enumerate()
        .map(|(index, question)| {
            let text = if question.multi_select {
                format!(
                    "{} (Several may apply: choose one and name the others in a note.)",
                    question.question
                )
            } else {
                question.question.clone()
            };
            let options = question
                .options
                .iter()
                .map(|option| json!({ "label": option.label, "description": option.description }))
                .collect::<Vec<_>>();
            json!({
                "id": question_id(index),
                "header": question.header,
                "question": text,
                "options": options,
            })
        })
        .collect::<Vec<_>>();
    json!({ "questions": questions })
}

/// The answers in Codex's `request_user_input` response, question by question. A question
/// left without an answer is left out, as Claude Code leaves it out.
pub fn answers(questions: &[Question], response: &Value) -> Vec<Answer> {
    questions
        .iter()
        .enumerate()
        .filter_map(|(index, question)| {
            let given = response
                .get("answers")?
                .get(question_id(index))?
                .get("answers")?
                .as_array()?
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>();
            let notes = given
                .iter()
                .filter_map(|answer| answer.strip_prefix(USER_NOTE_PREFIX))
                .map(str::trim)
                .filter(|note| !note.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            let notes = (!notes.is_empty()).then_some(notes);
            let labels = given
                .iter()
                .filter(|answer| {
                    question
                        .options
                        .iter()
                        .any(|option| option.label == **answer)
                })
                .copied()
                .collect::<Vec<_>>();
            let (answer, notes) = if !labels.is_empty() {
                (labels.join(", "), notes)
            } else if let Some(notes) = notes {
                (notes, None)
            } else {
                let other = given
                    .iter()
                    .filter(|answer| !answer.starts_with(USER_NOTE_PREFIX))
                    .copied()
                    .collect::<Vec<_>>();
                if other.is_empty() {
                    return None;
                }
                (other.join(", "), None)
            };
            Some(Answer {
                question: question.question.clone(),
                answer,
                notes,
            })
        })
        .collect()
}

/// The text the model receives, as Claude Code writes it.
pub fn result_text(answers: &[Answer]) -> String {
    let answers = answers
        .iter()
        .map(|answer| {
            let mut text = format!("\"{}\"=\"{}\"", answer.question, answer.answer);
            if let Some(notes) = &answer.notes {
                text.push_str(&format!(" user notes: {notes}"));
            }
            text
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "User has answered your questions: {answers}. You can now continue with the user's answers in mind."
    )
}

/// The answers as Claude Code reports them to hooks: question text to answer.
pub fn answers_json(answers: &[Answer]) -> Value {
    Value::Object(
        answers
            .iter()
            .map(|answer| {
                (
                    answer.question.clone(),
                    Value::String(answer.answer.clone()),
                )
            })
            .collect(),
    )
}

#[cfg(test)]
#[path = "ask_user_question_tests.rs"]
mod tests;
