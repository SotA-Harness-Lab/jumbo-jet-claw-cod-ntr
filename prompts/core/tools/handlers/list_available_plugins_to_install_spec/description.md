# List plugin/connector install candidates

Use this tool only when both are true:
- The user explicitly asks to use a specific plugin or connector that is not already available in the current context or active `tools` list.
- `{TOOL_SEARCH_TOOL_NAME}` is not available, or it has already been called and did not find or make the requested tool callable.

Returns known plugins and connectors that can be passed to `{REQUEST_PLUGIN_INSTALL_TOOL_NAME}`. When both a plugin and a connector match, prefer the plugin; use the connector only when its corresponding plugin is already installed.
