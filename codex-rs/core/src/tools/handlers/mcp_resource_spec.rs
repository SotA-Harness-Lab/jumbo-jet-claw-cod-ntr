use codex_protocol::openai_models::ToolMessage;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolSpec;
use codex_tools::with_catalog_tool_message;
use std::collections::BTreeMap;

pub fn create_list_mcp_resources_tool(
    messages: Option<&ToolMessage>,
    include_environment_id: bool,
) -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "server".to_string(),
            JsonSchema::string(Some(
                "MCP server name. Omit to list resources from every configured server.".to_string(),
            )),
        ),
        (
            "cursor".to_string(),
            JsonSchema::string(Some(
                "Opaque cursor from a previous list_mcp_resources call; omit for the first page."
                    .to_string(),
            )),
        ),
    ]);

    let tool = ResponsesApiTool {
        name: "list_mcp_resources".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/mcp_resource_spec/list_mcp_resources_description.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
        output_schema: None,
    };
    resource_tool_spec(tool, messages, include_environment_id)
}

pub fn create_list_mcp_resource_templates_tool(
    messages: Option<&ToolMessage>,
    include_environment_id: bool,
) -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "server".to_string(),
            JsonSchema::string(Some(
                "MCP server name. Omit to list resource templates from every configured server."
                    .to_string(),
            )),
        ),
        (
            "cursor".to_string(),
            JsonSchema::string(Some(
                "Opaque cursor from a previous list_mcp_resource_templates call; omit for the first page."
                    .to_string(),
            )),
        ),
    ]);

    let tool = ResponsesApiTool {
        name: "list_mcp_resource_templates".to_string(),
        description: codex_prompt_texts::text!(
            "core/tools/handlers/mcp_resource_spec/list_mcp_resource_templates_description.md"
        )
        .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(properties, /*required*/ None, Some(false.into())),
        output_schema: None,
    };
    resource_tool_spec(tool, messages, include_environment_id)
}

pub fn create_read_mcp_resource_tool(
    messages: Option<&ToolMessage>,
    include_environment_id: bool,
) -> ToolSpec {
    let properties = BTreeMap::from([
        (
            "server".to_string(),
            JsonSchema::string(Some(
                codex_prompt_texts::text!(
                    "core/tools/handlers/mcp_resource_spec/server_description.md"
                )
                .to_string(),
            )),
        ),
        (
            "uri".to_string(),
            JsonSchema::string(Some(
                "Resource URI to read. Must be one of the URIs returned by list_mcp_resources."
                    .to_string(),
            )),
        ),
    ]);

    let tool = ResponsesApiTool {
        name: "read_mcp_resource".to_string(),
        description:
            "Read a specific resource from an MCP server given the server name and resource URI."
                .to_string(),
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(
            properties,
            Some(vec!["server".to_string(), "uri".to_string()]),
            Some(false.into()),
        ),
        output_schema: None,
    };
    resource_tool_spec(tool, messages, include_environment_id)
}

fn resource_tool_spec(
    tool: ResponsesApiTool,
    messages: Option<&ToolMessage>,
    include_environment_id: bool,
) -> ToolSpec {
    let mut tool = with_catalog_tool_message(tool, messages);
    if include_environment_id {
        // Catalog schemas can replace the parameters, but cannot remove environment selection.
        tool.parameters.properties.get_or_insert_default().insert(
            "environment_id".to_string(),
            JsonSchema::string(Some(
                codex_prompt_texts::text!(
                    "core/tools/handlers/mcp_resource_spec/environment_id_description.md"
                )
                .to_string(),
            )),
        );
    }
    ToolSpec::Function(tool)
}

#[cfg(test)]
#[path = "mcp_resource_spec_tests.rs"]
mod tests;
