// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `AskUserQuestion`: Claude Code's multiple-choice question tool.
//!
//! Delegated: asking, to Codex's `request_user_input` runtime (the TUI's question panel, the
//! modes it is offered in, the root-thread rule, cancellation and the recorded answers).
//! Added: Claude Code's parameters and checks, its description and its result text.

use std::collections::BTreeMap;
use std::sync::Arc;

use codex_claude_compat::tools::ASK_USER_QUESTION;
use codex_claude_compat::tools::ask_user_question;
use codex_claude_compat::tools::ask_user_question::Question;
use codex_claude_compat::tools::ask_user_question::QuestionOption;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolExposure;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use serde_json::Value;
use serde_json::json;

use super::PermissionInput;
use super::check_permission;
use super::ToolSwap;
use super::delegated;
use super::failed;
use super::output::ClaudeToolOutput;
use super::parse_claude_arguments;
use super::rejected;
use crate::function_tool::FunctionCallError;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

/// Offers `AskUserQuestion` in place of `request_user_input`, when the turn has it.
pub(super) fn install(swap: &mut ToolSwap<'_>) {
    if let Some(request_user_input) = swap.take("request_user_input") {
        swap.add(AskUserQuestionHandler { request_user_input });
    }
}

struct AskUserQuestionHandler {
    request_user_input: Arc<dyn CoreToolRuntime>,
}

/// Claude Code's input; `answers`, `annotations` and `metadata`, which its own permission
/// dialog fills in, are accepted and ignored.
#[derive(Deserialize)]
struct AskUserQuestionArgs {
    questions: Vec<QuestionArg>,
}

#[derive(Deserialize)]
struct QuestionArg {
    question: String,
    header: String,
    options: Vec<OptionArg>,
    #[serde(rename = "multiSelect", default)]
    multi_select: bool,
}

#[derive(Deserialize)]
struct OptionArg {
    label: String,
    description: String,
}

impl ToolExecutor<ToolInvocation> for AskUserQuestionHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(ASK_USER_QUESTION)
    }

    /// As `request_user_input`: offered to the model directly, not inside code mode.
    fn exposure(&self) -> ToolExposure {
        ToolExposure::DirectModelOnly
    }

    fn spec(&self) -> ToolSpec {
        let option = JsonSchema::object(
            BTreeMap::from([
                (
                    "label".to_string(),
                    JsonSchema::string(Some(
                        ask_user_question::LABEL_PARAM_DESCRIPTION.to_string(),
                    )),
                ),
                (
                    "description".to_string(),
                    JsonSchema::string(Some(
                        ask_user_question::OPTION_DESCRIPTION_PARAM_DESCRIPTION.to_string(),
                    )),
                ),
            ]),
            Some(vec!["label".to_string(), "description".to_string()]),
            Some(false.into()),
        );
        let question = JsonSchema::object(
            BTreeMap::from([
                (
                    "question".to_string(),
                    JsonSchema::string(Some(
                        ask_user_question::QUESTION_PARAM_DESCRIPTION.to_string(),
                    )),
                ),
                (
                    "header".to_string(),
                    JsonSchema::string(Some(
                        ask_user_question::HEADER_PARAM_DESCRIPTION.to_string(),
                    )),
                ),
                (
                    "options".to_string(),
                    JsonSchema::array(
                        option,
                        Some(ask_user_question::OPTIONS_PARAM_DESCRIPTION.to_string()),
                    ),
                ),
                (
                    "multiSelect".to_string(),
                    JsonSchema::boolean(Some(
                        ask_user_question::MULTI_SELECT_PARAM_DESCRIPTION.to_string(),
                    )),
                ),
            ]),
            Some(vec![
                "question".to_string(),
                "header".to_string(),
                "options".to_string(),
                "multiSelect".to_string(),
            ]),
            Some(false.into()),
        );
        let properties = BTreeMap::from([(
            "questions".to_string(),
            JsonSchema::array(
                question,
                Some(ask_user_question::QUESTIONS_PARAM_DESCRIPTION.to_string()),
            ),
        )]);
        ToolSpec::Function(ResponsesApiTool {
            name: ASK_USER_QUESTION.to_string(),
            description: ask_user_question::description(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["questions".to_string()]),
                Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(self.handle_call(invocation))
    }
}

impl CoreToolRuntime for AskUserQuestionHandler {
    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(payload, ToolPayload::Function { .. })
    }
}

impl AskUserQuestionHandler {
    async fn handle_call(
        &self,
        invocation: ToolInvocation,
    ) -> Result<Box<dyn ToolOutput>, FunctionCallError> {
        let args: AskUserQuestionArgs = parse_claude_arguments(ASK_USER_QUESTION, &invocation)?;
        let _permission =
            check_permission(&invocation, ASK_USER_QUESTION, PermissionInput::Tool).await?;
        let questions = args
            .questions
            .into_iter()
            .map(|question| Question {
                question: question.question,
                header: question.header,
                options: question
                    .options
                    .into_iter()
                    .map(|option| QuestionOption {
                        label: option.label,
                        description: option.description,
                    })
                    .collect(),
                multi_select: question.multi_select,
            })
            .collect::<Vec<_>>();
        ask_user_question::validate(&questions).map_err(|issue| {
            rejected(&format!(
                "InputValidationError: {ASK_USER_QUESTION} failed due to the following issue: {issue}"
            ))
        })?;

        let arguments = ask_user_question::request_user_input_arguments(&questions).to_string();
        let payload = ToolPayload::Function { arguments };
        let output = self
            .request_user_input
            .handle(delegated(&invocation, payload.clone()))
            .await?;
        let response = output_json(output.as_ref(), &payload)
            .ok_or_else(|| failed("request_user_input returned no answers"))?;
        let answers = ask_user_question::answers(&questions, &response);

        let tool_response = json!({
            "questions": questions
                .iter()
                .map(|question| json!({
                    "question": question.question,
                    "header": question.header,
                    "options": question
                        .options
                        .iter()
                        .map(|option| json!({
                            "label": option.label,
                            "description": option.description,
                        }))
                        .collect::<Vec<_>>(),
                    "multiSelect": question.multi_select,
                }))
                .collect::<Vec<_>>(),
            "answers": ask_user_question::answers_json(&answers),
        });
        Ok(boxed_tool_output(ClaudeToolOutput::new(
            ask_user_question::result_text(&answers),
            tool_response,
        )))
    }
}

/// The JSON result of a delegated Codex tool, whether it gives it as a value or as text.
pub(super) fn output_json(output: &dyn ToolOutput, payload: &ToolPayload) -> Option<Value> {
    match output.code_mode_result(payload) {
        Value::String(text) => serde_json::from_str(&text).ok(),
        Value::Null => None,
        value => Some(value),
    }
}
