# Request plugin/connector install

Use this tool only after `{LIST_AVAILABLE_PLUGINS_TO_INSTALL_TOOL_NAME}` returns a plugin or connector that exactly matches the user's explicit request.

Do not use it for adjacent capabilities, broad recommendations, or tools that merely seem useful. Pass the returned `tool_type` through directly, and pass the returned `id` as `tool_id`.

IMPORTANT: DO NOT call this tool in parallel with other tools.