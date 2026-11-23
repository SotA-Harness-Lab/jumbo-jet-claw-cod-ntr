use codex_tools::JsonSchema;
use codex_tools::LIST_AVAILABLE_PLUGINS_TO_INSTALL_TOOL_NAME;
use codex_tools::REQUEST_PLUGIN_INSTALL_TOOL_NAME;
use codex_tools::ResponsesApiTool;
use codex_tools::TOOL_SEARCH_TOOL_NAME;
use codex_tools::ToolSpec;
pub(crate) fn create_list_available_plugins_to_install_tool() -> ToolSpec {
    let description = format!(
        codex_prompt_texts::text!(
            "core/tools/handlers/list_available_plugins_to_install_spec/description.md"
        ),
        TOOL_SEARCH_TOOL_NAME = TOOL_SEARCH_TOOL_NAME,
        REQUEST_PLUGIN_INSTALL_TOOL_NAME = REQUEST_PLUGIN_INSTALL_TOOL_NAME
    );

    ToolSpec::Function(ResponsesApiTool {
        name: LIST_AVAILABLE_PLUGINS_TO_INSTALL_TOOL_NAME.to_string(),
        description,
        strict: false,
        defer_loading: None,
        parameters: JsonSchema::object(Default::default(), Some(Vec::new()), Some(false.into())),
        output_schema: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn create_list_available_plugins_to_install_tool_uses_expected_wire_shape() {
        assert_eq!(
            create_list_available_plugins_to_install_tool(),
            ToolSpec::Function(ResponsesApiTool {
                name: "list_available_plugins_to_install".to_string(),
                description: "# List plugin/connector install candidates\n\nUse this tool only when both are true:\n- The user explicitly asks to use a specific plugin or connector that is not already available in the current context or active `tools` list.\n- `tool_search` is not available, or it has already been called and did not find or make the requested tool callable.\n\nReturns known plugins and connectors that can be passed to `request_plugin_install`. When both a plugin and a connector match, prefer the plugin; use the connector only when its corresponding plugin is already installed.\n".to_string(),
                strict: false,
                defer_loading: None,
                parameters: JsonSchema::object(
                    Default::default(),
                    Some(Vec::new()),
                    Some(false.into()),
                ),
                output_schema: None,
            })
        );
    }
}
