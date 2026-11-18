//! Claude Code's `Read` tool: description, line selection and numbering, messages.

use super::BASH;
use super::format_file_size;

/// Without `limit`, a file larger than this is refused (Claude Code's default).
pub const MAX_SIZE_BYTES: u64 = 256 * 1024;
/// The most tokens a read may return (Claude Code's default).
pub const MAX_TOKENS: usize = 25_000;
/// Lines the description says a read returns by default.
pub const MAX_LINES_TO_READ: usize = 2000;

pub const FILE_PATH_PARAM_DESCRIPTION: &str = "The absolute path to the file to read";
pub const OFFSET_PARAM_DESCRIPTION: &str =
    "The line number to start reading from. Only provide if the file is too large to read at once";
pub const LIMIT_PARAM_DESCRIPTION: &str =
    "The number of lines to read. Only provide if the file is too large to read at once.";

/// Extensions Claude Code reads as images.
const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp"];

/// Extensions Claude Code refuses to read as text (its `BINARY_EXTENSIONS`). Images are
/// read as images; PDF is listed because Codex's Read does not render PDFs.
const BINARY_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "ico", "webp", "tiff", "tif", "mp4", "mov", "avi", "mkv",
    "webm", "wmv", "flv", "m4v", "mpeg", "mpg", "mp3", "wav", "ogg", "flac", "aac", "m4a", "wma",
    "aiff", "opus", "zip", "tar", "gz", "bz2", "7z", "rar", "xz", "z", "tgz", "iso", "exe", "dll",
    "so", "dylib", "bin", "o", "a", "obj", "lib", "app", "msi", "deb", "rpm", "pdf", "doc", "docx",
    "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "ttf", "otf", "woff", "woff2", "eot", "pyc",
    "pyo", "class", "jar", "war", "ear", "node", "wasm", "rlib", "sqlite", "sqlite3", "db", "mdb",
    "idx", "psd", "ai", "eps", "sketch", "fig", "xd", "blend", "3ds", "max", "swf", "fla", "lockb",
    "dat", "data",
];

/// The tool description, after Claude Code's. `images` says whether image files are shown
/// to the model; Codex's Read does not render PDFs or notebooks, so those lines are left out.
pub fn description(images: bool) -> String {
    let mut text = format!(
        "Reads a file from the local filesystem. You can access any file directly by using this tool.
Assume this tool is able to read all files on the machine. If the User provides a path to a file assume that path is valid. It is okay to read a file that does not exist; an error will be returned.

Usage:
- The file_path parameter must be an absolute path, not a relative path
- By default, it reads up to {MAX_LINES_TO_READ} lines starting from the beginning of the file
- You can optionally specify a line offset and limit (especially handy for long files), but it's recommended to read the whole file by not providing these parameters
- Results are returned using cat -n format, with line numbers starting at 1"
    );
    if images {
        text.push_str("\n- This tool allows you to read images (eg PNG, JPG, etc). When reading an image file the contents are presented visually.");
    }
    text.push_str(&format!(
        "\n- This tool can only read files, not directories. To read a directory, use an ls command via the {BASH} tool.
- You will regularly be asked to read screenshots. If the user provides a path to a screenshot, ALWAYS use this tool to view the file at the path. This tool will work with all temporary file paths.
- If you read a file that exists but has empty contents you will receive a system reminder warning in place of file contents."
    ));
    text
}

fn extension(path: &str) -> Option<String> {
    let name = path.rsplit(['/', '\\']).next()?;
    let (stem, extension) = name.rsplit_once('.')?;
    (!stem.is_empty()).then(|| extension.to_ascii_lowercase())
}

/// Whether Claude Code reads the file as an image.
pub fn is_image(path: &str) -> bool {
    extension(path).is_some_and(|extension| IMAGE_EXTENSIONS.contains(&extension.as_str()))
}

/// The extension (with its dot) of a file Claude Code will not read as text, if it is one.
pub fn binary_extension(path: &str) -> Option<String> {
    extension(path)
        .filter(|extension| BINARY_EXTENSIONS.contains(&extension.as_str()))
        .map(|extension| format!(".{extension}"))
}

/// The lines a read returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSlice {
    /// The selected lines, joined with `\n`, without `\r`.
    pub content: String,
    /// Number of the first selected line (1-based).
    pub start_line: usize,
    /// Number of selected lines.
    pub num_lines: usize,
    /// Number of lines in the file.
    pub total_lines: usize,
}

/// Selects lines as Claude Code does: the byte order mark is dropped, lines end at `\n`
/// (a `\r` before it is dropped), the text after the last `\n` is a line too, `offset` is
/// 1-based (0 reads from the start) and `limit` counts lines.
pub fn slice_text(text: &str, offset: Option<usize>, limit: Option<usize>) -> TextSlice {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let offset = offset.unwrap_or(1);
    let first = offset.saturating_sub(1);
    let end = limit.map_or(usize::MAX, |limit| first.saturating_add(limit));
    if text.is_empty() {
        return TextSlice {
            content: String::new(),
            start_line: offset.max(1),
            num_lines: 0,
            total_lines: 0,
        };
    }
    let mut selected = Vec::new();
    let mut total_lines = 0;
    for (index, line) in text.split('\n').enumerate() {
        total_lines = index + 1;
        if index >= first && index < end {
            selected.push(line.strip_suffix('\r').unwrap_or(line));
        }
    }
    TextSlice {
        content: selected.join("\n"),
        start_line: offset.max(1),
        num_lines: selected.len(),
        total_lines,
    }
}

/// Numbers lines `N<TAB>line`, Claude Code's compact `cat -n` format.
pub fn number_lines(content: &str, start_line: usize) -> String {
    content
        .split('\n')
        .enumerate()
        .map(|(index, line)| format!("{}\t{line}", start_line + index))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The text the model receives for a read of a text file.
pub fn render(slice: &TextSlice) -> String {
    if !slice.content.is_empty() {
        return number_lines(&slice.content, slice.start_line);
    }
    if slice.total_lines == 0 {
        "<system-reminder>Warning: the file exists but the contents are empty.</system-reminder>"
            .to_string()
    } else {
        format!(
            "<system-reminder>Warning: the file exists but is shorter than the provided offset ({}). The file has {} lines.</system-reminder>",
            slice.start_line, slice.total_lines
        )
    }
}

/// The token count Claude Code assumes before counting: four bytes a token.
pub fn estimated_tokens(content: &str) -> usize {
    content.len().div_ceil(4)
}

/// Whether the selected text is too large to return.
pub fn exceeds_token_limit(content: &str) -> bool {
    content.len() > MAX_TOKENS * 4
}

pub fn token_limit_message(tokens: usize) -> String {
    format!(
        "File content ({tokens} tokens) exceeds maximum allowed tokens ({MAX_TOKENS}). Use offset and limit parameters to read specific portions of the file, or search for specific content instead of reading the whole file."
    )
}

pub fn too_large_message(size: u64) -> String {
    format!(
        "File content ({}) exceeds maximum allowed size ({}). Use offset and limit parameters to read specific portions of the file, or search for specific content instead of reading the whole file.",
        format_file_size(size),
        format_file_size(MAX_SIZE_BYTES)
    )
}

pub fn directory_message(file_path: &str) -> String {
    format!(
        "Cannot read '{file_path}': the specified path is an existing directory. Use {BASH} ls to list files in this directory, or specify a filename path to read a specific file."
    )
}

pub fn binary_message(extension: &str) -> String {
    format!(
        "This tool cannot read binary files. The file appears to be a binary {extension} file. Please use appropriate tools for binary file analysis."
    )
}

#[cfg(test)]
#[path = "read_tests.rs"]
mod tests;
