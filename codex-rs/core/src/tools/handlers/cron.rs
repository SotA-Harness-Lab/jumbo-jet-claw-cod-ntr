// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! `CronCreate`, `CronList` and `CronDelete`: the model schedules prompts for itself, by
//! the names, parameters and answers of Claude Code's tools. Jobs last as long as the
//! session; see `scheduled_prompts`.

use std::collections::BTreeMap;

use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;

use crate::function_tool::FunctionCallError;
use crate::scheduled_prompts::CronJob;
use crate::scheduled_prompts::CronJobs;
use crate::scheduled_prompts::describe_next_run;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;

const CRON_CREATE: &str = "CronCreate";
const CRON_LIST: &str = "CronList";
const CRON_DELETE: &str = "CronDelete";

const CREATE_DESCRIPTION: &str =
    codex_prompt_texts::text!("core/tools/handlers/cron/create_description.md");

/// The answer when the arguments are not a function call's.
fn arguments(payload: ToolPayload, tool: &str) -> Result<String, FunctionCallError> {
    match payload {
        ToolPayload::Function { arguments } => Ok(arguments),
        _ => Err(FunctionCallError::RespondToModel(format!(
            "{tool} handler received unsupported payload"
        ))),
    }
}

fn function_tool(
    name: &str,
    description: &str,
    properties: BTreeMap<String, JsonSchema>,
    required: &[&str],
) -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: name.to_string(),
        description: description.to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(required.iter().map(|name| (*name).to_string()).collect()),
            /*additional_properties*/ Some(false.into()),
        ),
        output_schema: None,
    })
}

fn text(text: String) -> Box<dyn crate::tools::context::ToolOutput> {
    boxed_tool_output(FunctionToolOutput::from_text(text, Some(true)))
}

fn kind(job: &CronJob) -> &'static str {
    if job.recurring {
        "recurring"
    } else {
        "one-shot"
    }
}

pub struct CronCreateHandler;

#[derive(Debug, Deserialize)]
struct CronCreateArgs {
    cron: String,
    prompt: String,
    #[serde(default)]
    recurring: Option<bool>,
}

impl ToolExecutor<ToolInvocation> for CronCreateHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(CRON_CREATE)
    }

    fn spec(&self) -> ToolSpec {
        function_tool(
            CRON_CREATE,
            CREATE_DESCRIPTION,
            BTreeMap::from([
                (
                    "cron".to_string(),
                    JsonSchema::string(Some(
                        "5-field cron expression in local time: minute hour day-of-month month day-of-week."
                            .to_string(),
                    )),
                ),
                (
                    "prompt".to_string(),
                    JsonSchema::string(Some(
                        "The prompt to deliver at each time the expression matches.".to_string(),
                    )),
                ),
                (
                    "recurring".to_string(),
                    JsonSchema::boolean(Some(
                        codex_prompt_texts::text!("core/tools/handlers/cron/recurring_description.md")
                            .to_string(),
                    )),
                ),
            ]),
            &["cron", "prompt"],
        )
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let args: CronCreateArgs =
                parse_arguments(&arguments(invocation.payload, CRON_CREATE)?)?;
            if args.prompt.trim().is_empty() {
                return Err(FunctionCallError::RespondToModel(
                    "prompt must not be empty".to_string(),
                ));
            }
            let jobs = CronJobs::of(&invocation.session);
            let job = jobs
                .create(
                    &args.cron,
                    args.prompt,
                    args.recurring.unwrap_or(true),
                    chrono::Local::now().naive_local(),
                )
                .await
                .map_err(FunctionCallError::RespondToModel)?;
            jobs.ensure_scheduler(&invocation.session);
            let next_run = describe_next_run(&job);
            Ok(text(if job.recurring {
                format!(
                    codex_prompt_texts::text!("core/tools/handlers/cron/scheduled_recurring.md"),
                    job.id,
                    job.cron,
                    next_run = next_run
                )
            } else {
                format!(
                    codex_prompt_texts::text!("core/tools/handlers/cron/scheduled_one_shot.md"),
                    job.id,
                    job.cron,
                    next_run = next_run
                )
            }))
        })
    }
}

impl CoreToolRuntime for CronCreateHandler {}

pub struct CronListHandler;

impl ToolExecutor<ToolInvocation> for CronListHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(CRON_LIST)
    }

    fn supports_parallel_tool_calls(&self) -> bool {
        true
    }

    fn spec(&self) -> ToolSpec {
        function_tool(
            CRON_LIST,
            "List the jobs scheduled with CronCreate in this session.",
            BTreeMap::new(),
            &[],
        )
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let jobs = CronJobs::of(&invocation.session).list().await;
            if jobs.is_empty() {
                return Ok(text("No scheduled jobs.".to_string()));
            }
            Ok(text(
                jobs.iter()
                    .map(|job| {
                        let prompt = job.prompt.chars().take(80).collect::<String>();
                        format!(
                            "{} - {} ({}), next at {}: {prompt}",
                            job.id,
                            job.cron,
                            kind(job),
                            describe_next_run(job)
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ))
        })
    }
}

impl CoreToolRuntime for CronListHandler {}

pub struct CronDeleteHandler;

#[derive(Debug, Deserialize)]
struct CronDeleteArgs {
    id: String,
}

impl ToolExecutor<ToolInvocation> for CronDeleteHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(CRON_DELETE)
    }

    fn spec(&self) -> ToolSpec {
        function_tool(
            CRON_DELETE,
            "Cancel a job scheduled with CronCreate.",
            BTreeMap::from([(
                "id".to_string(),
                JsonSchema::string(Some("Job ID returned by CronCreate.".to_string())),
            )]),
            &["id"],
        )
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let args: CronDeleteArgs =
                parse_arguments(&arguments(invocation.payload, CRON_DELETE)?)?;
            if CronJobs::of(&invocation.session).delete(&args.id).await {
                Ok(text(format!("Cancelled job {}.", args.id)))
            } else {
                Err(FunctionCallError::RespondToModel(format!(
                    "No scheduled job with id '{}'",
                    args.id
                )))
            }
        })
    }
}

impl CoreToolRuntime for CronDeleteHandler {}
