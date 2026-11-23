Writes a file to the local filesystem.

Usage:
- This tool will overwrite the existing file if there is one at the provided path.
- If this is an existing file, you MUST use the {READ} tool first to read the file's contents. This tool will fail if you did not read the file first.
- Prefer the {EDIT} tool for modifying existing files — it only sends the diff. Only use this tool to create new files or for complete rewrites.
- NEVER create documentation files (*.md) or README files unless explicitly requested by the User.
- Only use emojis if the user explicitly requests it. Avoid writing emojis to files unless asked.
- The file_path must be a distinct file path, not a directory path. If the path resolves to an existing directory, the tool will reject it with a clear error message. Use a path that includes a filename with an appropriate extension (e.g., `my-docs/analysis/api/report.md`).