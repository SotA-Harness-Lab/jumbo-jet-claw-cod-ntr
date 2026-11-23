use crate::agent::child_config::MAX_SPAWN_AGENT_MODEL_OVERRIDES;
use crate::agent::child_config::model_supports_multi_agent_backend;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::protocol::MultiAgentVersion;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiNamespace;
use codex_tools::ResponsesApiNamespaceTool;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use serde_json::Value;
use serde_json::json;
use std::collections::BTreeMap;

pub const MULTI_AGENT_V1_NAMESPACE: &str = "multi_agent_v1";
const MULTI_AGENT_V1_NAMESPACE_DESCRIPTION: &str = "Tools for spawning and managing sub-agents.";

const SPAWN_AGENT_INHERITED_MODEL_GUIDANCE: &str = codex_prompt_texts::text!(
    "core/tools/handlers/multi_agents_spec/spawn_agent_inherited_model_guidance.md"
);
const SPAWN_AGENT_INHERITED_MODEL_GUIDANCE_V2: &str = codex_prompt_texts::text!(
    "core/tools/handlers/multi_agents_spec/spawn_agent_inherited_model_guidance_v2.md"
);
const SPAWN_AGENT_MODEL_CATALOG_GUIDANCE: &str =
    "Pick model overrides from the latest <model_catalog> listing.";
const SPAWN_AGENT_TYPE_OVERRIDE_DESCRIPTION_V1: &str = codex_prompt_texts::text!(
    "core/tools/handlers/multi_agents_spec/spawn_agent_type_override_description_v1.md"
);
const SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION: &str =
    "Model override for the new agent. Omit unless an explicit override is needed.";
const MAX_REASONING_EFFORT_CHARS_IN_SPAWN_AGENT_DESCRIPTION: usize = 64;

#[derive(Debug, Clone)]
pub struct SpawnAgentToolOptions {
    pub available_models: Vec<ModelPreset>,
    pub agent_type_description: String,
    pub expose_agent_type: bool,
    pub hide_agent_type_model_reasoning: bool,
    pub expose_spawn_agent_model_overrides: bool,
    pub multi_agent_version: MultiAgentVersion,
    pub model_catalog_in_context: bool,
    pub usage_hint_text: Option<String>,
}

impl Default for SpawnAgentToolOptions {
    fn default() -> Self {
        Self {
            available_models: Vec::new(),
            agent_type_description: String::new(),
            expose_agent_type: true,
            hide_agent_type_model_reasoning: false,
            expose_spawn_agent_model_overrides: false,
            multi_agent_version: MultiAgentVersion::Disabled,
            model_catalog_in_context: false,
            usage_hint_text: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitAgentTimeoutOptions {
    pub default_timeout_ms: i64,
    pub min_timeout_ms: i64,
    pub max_timeout_ms: i64,
}

impl Default for WaitAgentTimeoutOptions {
    fn default() -> Self {
        Self {
            default_timeout_ms: super::multi_agents_common::DEFAULT_WAIT_TIMEOUT_MS,
            min_timeout_ms: super::multi_agents_common::MIN_WAIT_TIMEOUT_MS,
            max_timeout_ms: super::multi_agents_common::MAX_WAIT_TIMEOUT_MS,
        }
    }
}

pub fn create_spawn_agent_tool_v1(options: SpawnAgentToolOptions) -> ToolSpec {
    let available_models_description = (!options.model_catalog_in_context
        && !options.hide_agent_type_model_reasoning)
        .then(|| {
            spawn_agent_models_description(&options.available_models, options.multi_agent_version)
        });
    let inherited_model_guidance =
        (!options.hide_agent_type_model_reasoning).then_some(SPAWN_AGENT_INHERITED_MODEL_GUIDANCE);
    let return_value_description =
        "Returns the spawned agent id plus the user-facing nickname when available.";
    let mut properties = spawn_agent_common_properties_v1(&options.agent_type_description);
    if !options.expose_agent_type {
        properties.remove("agent_type");
    }
    if options.hide_agent_type_model_reasoning {
        hide_spawn_agent_metadata_options(&mut properties);
    }

    ToolSpec::Namespace(ResponsesApiNamespace {
        name: MULTI_AGENT_V1_NAMESPACE.to_string(),
        description: MULTI_AGENT_V1_NAMESPACE_DESCRIPTION.to_string(),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: "spawn_agent".to_string(),
            description: spawn_agent_tool_description(
                available_models_description.as_deref(),
                options.model_catalog_in_context,
                inherited_model_guidance,
                return_value_description,
                options.usage_hint_text,
            ),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
            output_schema: Some(spawn_agent_output_schema_v1().into()),
        })],
    })
}

pub fn create_spawn_agent_tool_v2(
    options: SpawnAgentToolOptions,
    description_override: Option<&str>,
) -> ToolSpec {
    let available_models_description = (!options.model_catalog_in_context
        && options.expose_spawn_agent_model_overrides)
        .then(|| {
            spawn_agent_models_description(&options.available_models, options.multi_agent_version)
        });
    let inherited_model_guidance = if options.model_catalog_in_context {
        options
            .expose_spawn_agent_model_overrides
            .then_some(SPAWN_AGENT_INHERITED_MODEL_GUIDANCE_V2)
    } else {
        (options.expose_spawn_agent_model_overrides && !options.hide_agent_type_model_reasoning)
            .then_some(SPAWN_AGENT_INHERITED_MODEL_GUIDANCE)
    };
    let mut properties = spawn_agent_common_properties_v2(&options.agent_type_description);
    if !options.expose_agent_type {
        properties.remove("agent_type");
    }
    if !options.expose_spawn_agent_model_overrides {
        properties.remove("model");
        properties.remove("reasoning_effort");
    }
    properties.insert(
        "task_name".to_string(),
        JsonSchema::string(Some(
            "Task name for the new agent. Use lowercase letters, digits, and underscores."
                .to_string(),
        )),
    );

    ToolSpec::Function(ResponsesApiTool {
        name: "spawn_agent".to_string(),
        description: spawn_agent_tool_description_v2(
            available_models_description.as_deref(),
            options.model_catalog_in_context,
            options
                .expose_spawn_agent_model_overrides
                .then_some(SPAWN_AGENT_MODEL_CATALOG_GUIDANCE),
            inherited_model_guidance,
            options.usage_hint_text,
            description_override,
        ),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["task_name".to_string(), "message".to_string()]),
            Some(false.into()),
        ),
        output_schema: Some(
            spawn_agent_output_schema_v2(options.hide_agent_type_model_reasoning).into(),
        ),
    })
}

pub fn create_send_input_tool_v1() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some("Agent id to message (from spawn_agent).".to_string())),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Legacy plain-text message to send to the agent. Use either message or items."
                    .to_string(),
            )),
        ),
        ("items".to_string(), create_collab_input_items_schema()),
        (
            "interrupt".to_string(),
            JsonSchema::boolean(Some(
                "True interrupts the current task and handles this message immediately; false or omitted queues it."
                    .to_string(),
            )),
        ),
    ]);

    ToolSpec::Namespace(ResponsesApiNamespace {
        name: MULTI_AGENT_V1_NAMESPACE.to_string(),
        description: MULTI_AGENT_V1_NAMESPACE_DESCRIPTION.to_string(),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: "send_input".to_string(),
            description: codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/send_input_description_v1.md"
            )
            .to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["target".to_string()]),
                Some(false.into()),
            ),
            output_schema: Some(send_input_output_schema().into()),
        })],
    })
}

pub fn create_send_message_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some(
                "Relative or canonical task name to message (from spawn_agent).".to_string(),
            )),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Message text to queue on the target agent.".to_string(),
            ))
            .with_encrypted(),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: "send_message".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/multi_agents_spec/send_message_description.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["target".to_string(), "message".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

pub fn create_followup_task_tool() -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "target".to_string(),
            JsonSchema::string(Some(
                "Agent id or canonical task name to send a follow-up task to (from spawn_agent)."
                    .to_string(),
            )),
        ),
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Message text to send to the target agent.".to_string(),
            ))
            .with_encrypted(),
        ),
    ]);

    ToolSpec::Function(ResponsesApiTool {
        name: "followup_task".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/multi_agents_spec/followup_task_description.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["target".to_string(), "message".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    })
}

pub fn create_resume_agent_tool() -> ToolSpec {
    let properties = BTreeMap::from([(
        "id".to_string(),
        JsonSchema::string(Some("Agent id to resume.".to_string())),
    )]);

    ToolSpec::Namespace(ResponsesApiNamespace {
        name: MULTI_AGENT_V1_NAMESPACE.to_string(),
        description: MULTI_AGENT_V1_NAMESPACE_DESCRIPTION.to_string(),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: "resume_agent".to_string(),
            description:
                "Resume a previously closed agent by id so it can receive send_input and wait_agent calls."
                    .to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(properties, Some(vec!["id".to_string()]), Some(false.into())),
            output_schema: Some(resume_agent_output_schema().into()),
        })],
    })
}

pub fn create_wait_agent_tool_v1(options: WaitAgentTimeoutOptions) -> ToolSpec {
    ToolSpec::Namespace(ResponsesApiNamespace {
        name: MULTI_AGENT_V1_NAMESPACE.to_string(),
        description: MULTI_AGENT_V1_NAMESPACE_DESCRIPTION.to_string(),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: "wait_agent".to_string(),
            description: codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/wait_agent_description_v1.md"
            )
            .to_string(),
            strict: false,
            defer_loading: None,
            parameters: wait_agent_tool_parameters_v1(options),
            output_schema: Some(wait_output_schema_v1().into()),
        })],
    })
}

pub fn create_wait_agent_tool_v2(options: WaitAgentTimeoutOptions) -> ToolSpec {
    ToolSpec::Function(ResponsesApiTool {
        name: "wait_agent".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/multi_agents_spec/wait_agent_description_v2.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: wait_agent_tool_parameters_v2(options),
        output_schema: Some(wait_output_schema_v2().into()),
    })
}

pub fn create_list_agents_tool() -> ToolSpec {
    let properties = BTreeMap::from([(
        "path_prefix".to_string(),
        JsonSchema::string(Some(
            "Task-path prefix filter without a trailing slash. Omit to list all live agents."
                .to_string(),
        )),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "list_agents".to_string(),
        description:
            "List live agents in the current root thread tree. Optionally filter by task-path prefix."
                .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
        output_schema: Some(list_agents_output_schema().into()),
    })
}

pub fn create_close_agent_tool_v1() -> ToolSpec {
    let properties = BTreeMap::from([(
        "target".to_string(),
        JsonSchema::string(Some("Agent id to close (from spawn_agent).".to_string())),
    )]);

    ToolSpec::Namespace(ResponsesApiNamespace {
        name: MULTI_AGENT_V1_NAMESPACE.to_string(),
        description: MULTI_AGENT_V1_NAMESPACE_DESCRIPTION.to_string(),
        tools: vec![ResponsesApiNamespaceTool::Function(ResponsesApiTool {
            name: "close_agent".to_string(),
            description: codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/close_agent_description_v1.md"
            )
            .to_string(),
            strict: false,
            defer_loading: None,
            parameters: JsonSchema::object(
                properties,
                Some(vec!["target".to_string()]),
                Some(false.into()),
            ),
            output_schema: Some(
                agent_previous_status_output_schema(
                    "The agent status observed before shutdown was requested.",
                )
                .into(),
            ),
        })],
    })
}

pub fn create_interrupt_agent_tool_v2() -> ToolSpec {
    let properties = BTreeMap::from([(
        "target".to_string(),
        JsonSchema::string(Some(
            "Agent id or canonical task name to interrupt (from spawn_agent).".to_string(),
        )),
    )]);

    ToolSpec::Function(ResponsesApiTool {
        name: "interrupt_agent".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/multi_agents_spec/interrupt_agent_description_v2.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["target".to_string()]),
            Some(false.into()),
        ),
        output_schema: Some(
            agent_previous_status_output_schema(
                "The agent status observed before the interrupt request was handled.",
            )
            .into(),
        ),
    })
}

fn agent_status_output_schema() -> Value {
    json!({
        "oneOf": [
            {
                "type": "string",
                "enum": ["pending_init", "running", "interrupted", "shutdown", "not_found"]
            },
            {
                "type": "object",
                "properties": {
                    "completed": {
                        "type": ["string", "null"]
                    }
                },
                "required": ["completed"],
                "additionalProperties": false
            },
            {
                "type": "object",
                "properties": {
                    "errored": {
                        "type": "string"
                    }
                },
                "required": ["errored"],
                "additionalProperties": false
            }
        ]
    })
}

fn spawn_agent_output_schema_v1() -> Value {
    json!({
        "type": "object",
        "properties": {
            "agent_id": {
                "type": "string",
                "description": "Thread identifier for the spawned agent."
            },
            "nickname": {
                "type": ["string", "null"],
                "description": "User-facing nickname for the spawned agent when available."
            }
        },
        "required": ["agent_id", "nickname"],
        "additionalProperties": false
    })
}

fn spawn_agent_output_schema_v2(hide_agent_metadata: bool) -> Value {
    if hide_agent_metadata {
        return json!({
            "type": "object",
            "properties": {
                "task_name": {
                    "type": "string",
                    "description": "Canonical task name for the spawned agent."
                }
            },
            "required": ["task_name"],
            "additionalProperties": false
        });
    }

    json!({
        "type": "object",
        "properties": {
            "task_name": {
                "type": "string",
                "description": "Canonical task name for the spawned agent."
            },
            "nickname": {
                "type": ["string", "null"],
                "description": "User-facing nickname for the spawned agent when available."
            }
        },
        "required": ["task_name", "nickname"],
        "additionalProperties": false
    })
}

fn send_input_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "submission_id": {
                "type": "string",
                "description": "Identifier for the queued input submission."
            }
        },
        "required": ["submission_id"],
        "additionalProperties": false
    })
}

fn list_agents_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "agents": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "agent_name": {
                            "type": "string",
                            "description": "Canonical task name for the agent when available, otherwise the agent id."
                        },
                        "agent_status": {
                            "description": "Last known status of the agent.",
                            "allOf": [agent_status_output_schema()]
                        }
                    },
                    "required": ["agent_name", "agent_status"],
                    "additionalProperties": false
                },
                "description": "Live agents visible in the current root thread tree."
            }
        },
        "required": ["agents"],
        "additionalProperties": false
    })
}

fn resume_agent_output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "status": agent_status_output_schema()
        },
        "required": ["status"],
        "additionalProperties": false
    })
}

fn wait_output_schema_v1() -> Value {
    json!({
        "type": "object",
        "properties": {
            "status": {
                "type": "object",
                "description": "Final statuses keyed by agent id.",
                "additionalProperties": agent_status_output_schema()
            },
            "timed_out": {
                "type": "boolean",
                "description": "Whether the wait call returned due to timeout before any agent reached a final status."
            }
        },
        "required": ["status", "timed_out"],
        "additionalProperties": false
    })
}

fn wait_output_schema_v2() -> Value {
    json!({
        "type": "object",
        "properties": {
            "message": {
                "type": "string",
                "description": "Brief wait summary without the agent's final content, including any timeout adjustment."
            },
            "timed_out": {
                "type": "boolean",
                "description": "Whether the wait call returned because no mailbox update arrived before the timeout."
            }
        },
        "required": ["message", "timed_out"],
        "additionalProperties": false
    })
}

fn agent_previous_status_output_schema(previous_status_description: &str) -> Value {
    json!({
        "type": "object",
        "properties": {
            "previous_status": {
                "description": previous_status_description,
                "allOf": [agent_status_output_schema()]
            }
        },
        "required": ["previous_status"],
        "additionalProperties": false
    })
}

fn create_collab_input_items_schema() -> JsonSchema {
    let properties = BTreeMap::from([
        (
            "type".to_string(),
            JsonSchema::string(Some(
                "Input item type: text, image, local_image, audio, local_audio, skill, or mention."
                    .to_string(),
            )),
        ),
        (
            "text".to_string(),
            JsonSchema::string(Some("Text content when type is text.".to_string())),
        ),
        (
            "image_url".to_string(),
            JsonSchema::string(Some("Image URL when type is image.".to_string())),
        ),
        (
            "audio_url".to_string(),
            JsonSchema::string(Some("Audio data URL when type is audio.".to_string())),
        ),
        (
            "path".to_string(),
            JsonSchema::string(Some(
                codex_prompt_texts::text!(
                    "core/tools/handlers/multi_agents_spec/collab_input_path_description.md"
                )
                .to_string(),
            )),
        ),
        (
            "name".to_string(),
            JsonSchema::string(Some(
                "Display name when type is skill or mention.".to_string(),
            )),
        ),
    ]);

    JsonSchema::array(JsonSchema::object(properties, /*required*/ None, Some(false.into())), Some(
            "Structured input items. Use this to pass explicit mentions (for example app:// connector paths)."
                .to_string(),
        ))
}

fn spawn_agent_common_properties_v1(agent_type_description: &str) -> BTreeMap<String, JsonSchema> {
    BTreeMap::from([
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Initial plain-text task for the new agent. Use either message or items."
                    .to_string(),
            )),
        ),
        ("items".to_string(), create_collab_input_items_schema()),
        (
            "agent_type".to_string(),
            JsonSchema::string(Some(format!(
                "{SPAWN_AGENT_TYPE_OVERRIDE_DESCRIPTION_V1}\n{agent_type_description}"
            ))),
        ),
        (
            "fork_context".to_string(),
            JsonSchema::boolean(Some(
                codex_prompt_texts::text!(
                    "core/tools/handlers/multi_agents_spec/fork_context_description_v1.md"
                )
                .to_string(),
            )),
        ),
        (
            "model".to_string(),
            JsonSchema::string(Some(SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION.to_string())),
        ),
        (
            "reasoning_effort".to_string(),
            JsonSchema::string(Some(
                "Reasoning effort override for the new agent. Omit to inherit the parent effort."
                    .to_string(),
            )),
        ),
    ])
}

fn spawn_agent_common_properties_v2(agent_type_description: &str) -> BTreeMap<String, JsonSchema> {
    BTreeMap::from([
        (
            "message".to_string(),
            JsonSchema::string(Some(
                "Initial plain-text task for the new agent.".to_string(),
            ))
            .with_encrypted(),
        ),
        (
            "agent_type".to_string(),
            JsonSchema::string(Some(format!(
                codex_prompt_texts::text!(
                    "core/tools/handlers/multi_agents_spec/agent_type_description_v2.md"
                ),
                agent_type_description = agent_type_description
            ))),
        ),
        (
            "fork_turns".to_string(),
            JsonSchema::string(Some(
                codex_prompt_texts::text!(
                    "core/tools/handlers/multi_agents_spec/fork_turns_description_v2.md"
                )
                .to_string(),
            )),
        ),
        (
            "model".to_string(),
            JsonSchema::string(Some(SPAWN_AGENT_MODEL_OVERRIDE_DESCRIPTION.to_string())),
        ),
        (
            "reasoning_effort".to_string(),
            JsonSchema::string(Some(
                "Reasoning effort override for the new agent. Omit to inherit the parent effort."
                    .to_string(),
            )),
        ),
    ])
}

fn hide_spawn_agent_metadata_options(properties: &mut BTreeMap<String, JsonSchema>) {
    properties.remove("agent_type");
    properties.remove("model");
    properties.remove("reasoning_effort");
}

fn spawn_agent_tool_description(
    available_models_description: Option<&str>,
    model_catalog_in_context: bool,
    inherited_model_guidance: Option<&str>,
    return_value_description: &str,
    usage_hint_text: Option<String>,
) -> String {
    let model_catalog_guidance = inherited_model_guidance
        .map(|_| SPAWN_AGENT_MODEL_CATALOG_GUIDANCE)
        .unwrap_or_default();
    let inherited_model_guidance = inherited_model_guidance.unwrap_or_default();

    let tool_description = if model_catalog_in_context {
        format!(
            codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/spawn_agent_tool_description_catalog.md"
            ),
            return_value_description = return_value_description,
            inherited_model_guidance = inherited_model_guidance,
            model_catalog_guidance = model_catalog_guidance
        )
    } else {
        let available_models_description = available_models_description.unwrap_or_default();
        format!(
            codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/spawn_agent_tool_description_models.md"
            ),
            available_models_description = available_models_description,
            return_value_description = return_value_description,
            inherited_model_guidance = inherited_model_guidance
        )
    };

    if let Some(usage_hint_text) = usage_hint_text {
        return format!(
            codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/spawn_agent_tool_description/tool_description_usage_hint_text.md"
            ),
            tool_description = tool_description,
            usage_hint_text = usage_hint_text
        );
    }
    let agent_role_usage_hint = if model_catalog_in_context {
        ""
    } else if available_models_description.is_some() {
        codex_prompt_texts::text!("core/tools/handlers/multi_agents_spec/agent_role_usage_hint.md")
    } else {
        "\n"
    };
    format!(
        codex_prompt_texts::text!(
            "core/tools/handlers/multi_agents_spec/spawn_agent_description_v1.md"
        ),
        tool_description = tool_description,
        agent_role_usage_hint = agent_role_usage_hint
    )
}

fn spawn_agent_tool_description_v2(
    available_models_description: Option<&str>,
    model_catalog_in_context: bool,
    model_catalog_guidance: Option<&str>,
    inherited_model_guidance: Option<&str>,
    usage_hint_text: Option<String>,
    description: Option<&str>,
) -> String {
    let model_catalog_guidance = model_catalog_guidance.unwrap_or_default();
    let inherited_model_guidance = inherited_model_guidance.unwrap_or_default();

    let (catalog_prefix, catalog_suffix) = if model_catalog_in_context {
        (String::new(), format!("\n{model_catalog_guidance}"))
    } else {
        let available_models_description = available_models_description.unwrap_or_default();
        (
            format!("        {available_models_description}\n"),
            String::new(),
        )
    };
    let tool_description = if let Some(description) = description {
        format!(
            r#"
{catalog_prefix}        {description}
{inherited_model_guidance}{catalog_suffix}"#
        )
    } else {
        format!(
            codex_prompt_texts::text!(
                "core/tools/handlers/multi_agents_spec/spawn_agent_description_v2.md"
            ),
            catalog_prefix = catalog_prefix,
            inherited_model_guidance = inherited_model_guidance,
            catalog_suffix = catalog_suffix
        )
    };

    if let Some(usage_hint_text) = usage_hint_text {
        return format!(
            r#"
        {tool_description}
{usage_hint_text}"#
        );
    }
    tool_description
}

fn spawn_agent_models_description(
    models: &[ModelPreset],
    multi_agent_version: MultiAgentVersion,
) -> String {
    let visible_models: Vec<&ModelPreset> = models
        .iter()
        .filter(|model| model.show_in_picker)
        .filter(|model| model_supports_multi_agent_backend(model, multi_agent_version))
        .take(MAX_SPAWN_AGENT_MODEL_OVERRIDES)
        .collect();
    if visible_models.is_empty() {
        return "No picker-visible model overrides are currently loaded.".to_string();
    }

    let model_descriptions = visible_models
        .into_iter()
        .map(|model| {
            let default_reasoning_effort = &model.default_reasoning_effort;
            let efforts = model
                .supported_reasoning_efforts
                .iter()
                .map(|preset| {
                    let effort = preset.effort.as_str();
                    let effort = match effort
                        .char_indices()
                        .nth(MAX_REASONING_EFFORT_CHARS_IN_SPAWN_AGENT_DESCRIPTION)
                    {
                        Some((index, _)) => &effort[..index],
                        None => effort,
                    };
                    if &preset.effort == default_reasoning_effort {
                        format!("{effort} (default)")
                    } else {
                        effort.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            let reasoning_efforts_suffix = if efforts.is_empty() {
                String::new()
            } else {
                format!(" Reasoning efforts: {efforts}.")
            };
            let service_tiers = model
                .service_tiers
                .iter()
                .map(|tier| tier.id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let service_tiers_suffix = if service_tiers.is_empty() {
                String::new()
            } else {
                format!(" Service tiers: {service_tiers}.")
            };
            let model_slug = &model.model;
            let description = &model.description;
            format!(
                "- `{model_slug}`: {description}{reasoning_efforts_suffix}{service_tiers_suffix}"
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Available model overrides (optional; inherited parent model is preferred):\n{model_descriptions}"
    )
}

fn wait_agent_tool_parameters_v1(options: WaitAgentTimeoutOptions) -> JsonSchema {
    let properties = BTreeMap::from([
        (
            "targets".to_string(),
            JsonSchema::array(
                JsonSchema::string(/*description*/ None),
                Some(
                    "Agent ids to wait on. Pass multiple ids to wait for whichever finishes first."
                        .to_string(),
                ),
            ),
        ),
        (
            "timeout_ms".to_string(),
            JsonSchema::number(Some(format!(
                codex_prompt_texts::text!(
                    "core/tools/handlers/multi_agents_spec/wait_timeout_description_v1.md"
                ),
                options.default_timeout_ms, options.min_timeout_ms, options.max_timeout_ms,
            ))),
        ),
    ]);

    JsonSchema::object(
        properties,
        Some(vec!["targets".to_string()]),
        Some(false.into()),
    )
}

fn wait_agent_tool_parameters_v2(options: WaitAgentTimeoutOptions) -> JsonSchema {
    let properties = BTreeMap::from([(
        "timeout_ms".to_string(),
        JsonSchema::number(Some(format!(
            "Timeout in milliseconds. Defaults to {}, min {}, max {}.",
            options.default_timeout_ms, options.min_timeout_ms, options.max_timeout_ms,
        ))),
    )]);

    JsonSchema::object(properties, /*required*/ None, Some(false.into()))
}

#[cfg(test)]
#[path = "multi_agents_spec_tests.rs"]
mod tests;
