use pretty_assertions::assert_eq;
use serde_json::json;

use super::Answer;
use super::Question;
use super::QuestionOption;
use super::answers;
use super::answers_json;
use super::request_user_input_arguments;
use super::result_text;
use super::validate;

fn question(text: &str, labels: &[&str], multi_select: bool) -> Question {
    Question {
        question: text.to_string(),
        header: "Header".to_string(),
        options: labels
            .iter()
            .map(|label| QuestionOption {
                label: label.to_string(),
                description: format!("About {label}"),
            })
            .collect(),
        multi_select,
    }
}

#[test]
fn questions_become_request_user_input_questions() {
    let questions = vec![
        question("Which library?", &["A", "B"], false),
        question("Which features?", &["X", "Y"], true),
    ];
    assert_eq!(
        request_user_input_arguments(&questions),
        json!({
            "questions": [
                {
                    "id": "question_1",
                    "header": "Header",
                    "question": "Which library?",
                    "options": [
                        { "label": "A", "description": "About A" },
                        { "label": "B", "description": "About B" },
                    ],
                },
                {
                    "id": "question_2",
                    "header": "Header",
                    "question": "Which features? (Several may apply: choose one and name the others in a note.)",
                    "options": [
                        { "label": "X", "description": "About X" },
                        { "label": "Y", "description": "About Y" },
                    ],
                },
            ]
        })
    );
}

#[test]
fn codex_answers_read_as_claude_answers() {
    let questions = vec![
        question("Which library?", &["A", "B"], false),
        question("Which features?", &["X", "Y"], true),
        question("Which color?", &["Red", "Blue"], false),
        question("Unanswered?", &["Yes", "No"], false),
    ];
    let response = json!({
        "answers": {
            "question_1": { "answers": ["B"] },
            "question_2": { "answers": ["X", "user_note: and Y"] },
            "question_3": { "answers": ["None of the above", "user_note: Green"] },
            "question_4": { "answers": [] },
        }
    });
    let answers = answers(&questions, &response);
    assert_eq!(
        answers,
        vec![
            Answer {
                question: "Which library?".to_string(),
                answer: "B".to_string(),
                notes: None,
            },
            Answer {
                question: "Which features?".to_string(),
                answer: "X".to_string(),
                notes: Some("and Y".to_string()),
            },
            Answer {
                question: "Which color?".to_string(),
                answer: "Green".to_string(),
                notes: None,
            },
        ]
    );
    assert_eq!(
        result_text(&answers),
        "User has answered your questions: \"Which library?\"=\"B\", \"Which features?\"=\"X\" user notes: and Y, \"Which color?\"=\"Green\". You can now continue with the user's answers in mind."
    );
    assert_eq!(
        answers_json(&answers[..1]),
        json!({ "Which library?": "B" })
    );
}

#[test]
fn questions_are_checked_as_in_claude_code() {
    assert_eq!(validate(&[question("Q?", &["A", "B"], false)]), Ok(()));
    assert!(validate(&[]).is_err());
    assert!(validate(&[question("Q?", &["A"], false)]).is_err());
    assert!(validate(&[question("Q?", &["A", "B", "C", "D", "E"], false)]).is_err());
    assert_eq!(
        validate(&[question("Q?", &["A", "A"], false)]),
        Err(
            "Question texts must be unique, option labels must be unique within each question"
                .to_string()
        )
    );
    assert!(
        validate(&[
            question("Q?", &["A", "B"], false),
            question("Q?", &["C", "D"], false)
        ])
        .is_err()
    );
}
