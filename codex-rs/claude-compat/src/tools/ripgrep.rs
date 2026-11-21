// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Running ripgrep for Claude Code's `Glob` and `Grep`: the command line handed to Codex's
//! command execution, and reading back what ripgrep printed, as Claude Code's `ripGrep`
//! helper reads it.

/// How long a search may run (Claude Code's default outside WSL).
pub const TIMEOUT_MS: u64 = 20_000;

/// Directories of version control systems that `Grep` leaves out (Claude Code's list).
pub const VCS_DIRECTORIES: &[&str] = &[".git", ".svn", ".hg", ".bzr", ".jj", ".sl"];

/// The shell Codex runs the command line with, which decides how arguments are quoted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    /// `sh`, `bash`, `zsh`.
    Posix,
    PowerShell,
    Cmd,
}

/// The command line running `rg` with `args` and then `target`, the file or directory
/// searched, as Claude Code passes it last. `--no-config` keeps a user's ripgrep
/// configuration from changing the output, as Claude Code's own ripgrep does.
pub fn command_line(args: &[String], target: &str, shell: Shell) -> String {
    let mut line = String::from("rg --no-config");
    for arg in args.iter().map(String::as_str).chain([target]) {
        line.push(' ');
        line.push_str(&quote(arg, shell));
    }
    line
}

/// Most bytes of results read back from a search that wrote them to a file, as Claude
/// Code keeps at most this much of ripgrep's output.
pub const MAX_OUTPUT_BYTES: usize = 20_000_000;

/// The size limit, in 512-byte blocks (1024-byte ones under bash, which then allows twice
/// as much), that keeps ripgrep from writing a file much larger than [`MAX_OUTPUT_BYTES`]:
/// it is stopped once the file reaches it.
const OUTPUT_FILE_BLOCKS: usize = MAX_OUTPUT_BYTES.div_ceil(512);

/// [`command_line`] for a POSIX shell, writing ripgrep's results to `file` instead of the
/// output, where only the first and last 512 KiB of them would come back. Messages still go
/// to the output.
pub fn command_line_to_file(args: &[String], target: &str, file: &str) -> String {
    format!(
        "(ulimit -f {OUTPUT_FILE_BLOCKS}; exec {} > {})",
        command_line(args, target, Shell::Posix),
        quote(file, Shell::Posix)
    )
}

/// Whether Codex's command execution cut `output` (stdout and stderr together) in the
/// middle, as it does past 1 MiB, putting a line `... <n> bytes omitted ...` in place of
/// what it left out.
pub fn output_was_cut(output: &str) -> bool {
    output.split('\n').any(|line| {
        line.strip_prefix("... ")
            .and_then(|rest| rest.strip_suffix(" bytes omitted ..."))
            .is_some_and(|count| !count.is_empty() && count.bytes().all(|b| b.is_ascii_digit()))
    })
}

/// The arguments that leave out of a search what `globs` match (`Read` deny patterns,
/// relative to the directory `rg` runs in), compared without regard to case as the
/// permission rules are.
pub fn exclusion_args(globs: &[String]) -> Vec<String> {
    globs
        .iter()
        .flat_map(|glob| ["--iglob".to_string(), format!("!{glob}")])
        .collect()
}

fn quote(arg: &str, shell: Shell) -> String {
    // A leading `=` stays quoted: zsh expands `=word` to the path of the command `word`.
    let plain = !arg.is_empty()
        && !arg.starts_with('=')
        && arg
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_=./:+".contains(c));
    if plain {
        return arg.to_string();
    }
    match shell {
        Shell::Posix => format!("'{}'", arg.replace('\'', r"'\''")),
        Shell::PowerShell => format!("'{}'", arg.replace('\'', "''")),
        Shell::Cmd => format!("\"{}\"", arg.replace('"', "\\\"")),
    }
}

/// What a finished `rg` command produced, as Codex's command execution reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgRun<'a> {
    /// Output of the command, stdout and stderr together.
    pub output: &'a str,
    /// ripgrep's results, when the command wrote them to a file instead of the output.
    pub results: Option<&'a str>,
    /// Whether the results were cut at [`MAX_OUTPUT_BYTES`].
    pub truncated: bool,
    /// Exit code; `None` when the command did not exit by itself.
    pub exit_code: Option<i32>,
    /// Whether the command was stopped at its timeout.
    pub timed_out: bool,
    /// The path searched, which starts every result when it is a directory.
    pub target: &'a str,
}

/// The lines ripgrep printed, or the error the model is told.
///
/// As in Claude Code: exit code 1 is "no matches"; other failures keep the results printed
/// before them, and a search stopped at its timeout loses its last, maybe partial, line
/// and fails only when nothing was found. Results cut at their size limit lose their last
/// line too, and never fail. ripgrep's own messages (`rg: ...`, on stderr, which Codex
/// returns mixed with stdout) are not results; after a failure only lines that start with
/// the path searched are. Unlike Claude Code, a failure without results
/// reports ripgrep's message (an invalid pattern, a path that cannot be read) instead of
/// "no matches".
pub fn result_lines(run: &RgRun<'_>) -> Result<Vec<String>, String> {
    let mut lines: Vec<String> = run
        .results
        .unwrap_or(run.output)
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .filter(|line| !line.is_empty() && !line.starts_with("rg: "))
        .map(str::to_string)
        .collect();

    if run.timed_out || run.truncated {
        lines.pop();
        if lines.is_empty() && !run.truncated {
            return Err(timeout_message());
        }
        return Ok(lines);
    }
    match run.exit_code {
        Some(0) => Ok(lines),
        Some(1) if is_missing_ripgrep(run.output) => Err(missing_ripgrep_message()),
        Some(1) => Ok(Vec::new()),
        Some(127) | Some(9009) => Err(missing_ripgrep_message()),
        exit_code => {
            lines.retain(|line| line.starts_with(run.target));
            if !lines.is_empty() {
                return Ok(lines);
            }
            let mut message = match exit_code {
                Some(code) => format!("ripgrep failed with exit code {code}"),
                None => "ripgrep did not finish".to_string(),
            };
            let output = run.output.trim();
            if !output.is_empty() {
                message.push('\n');
                message.push_str(output);
            }
            Err(message)
        }
    }
}

/// PowerShell exits with 1 when a command is not found, which is ripgrep's "no matches".
fn is_missing_ripgrep(output: &str) -> bool {
    output.contains("'rg' is not recognized")
}

/// Adds the note that the results are partial to a `Glob` or `Grep` result whose search
/// stopped at its timeout, so that the model does not take them for all there are.
pub fn note_timeout(text: &mut String, timed_out: bool) {
    if timed_out {
        text.push_str(&format!(
            "\n\n(The search timed out after {} seconds, so these results are partial. Try searching a more specific path or pattern.)",
            TIMEOUT_MS / 1000
        ));
    }
}

fn timeout_message() -> String {
    format!(
        "Ripgrep search timed out after {} seconds. The search may have matched files but did not complete in time. Try searching a more specific path or pattern.",
        TIMEOUT_MS / 1000
    )
}

fn missing_ripgrep_message() -> String {
    "ripgrep (rg) was not found. Glob and Grep run rg in the session's environment; install ripgrep there or put it on the PATH.".to_string()
}

/// `path` relative to `cwd` when it is inside it, as Claude Code shortens paths to save
/// tokens; otherwise `path` unchanged. Lines of results that start with a path (`path`,
/// `path:12:text`, `path-12-text`, `path:3`) are shortened the same way.
pub fn relative_to(path: &str, cwd: &str) -> String {
    let cwd = cwd.trim_end_matches(['/', '\\']);
    if cwd.is_empty() {
        // The root: every absolute path is inside it.
        return path
            .strip_prefix(['/', '\\'])
            .map(str::to_string)
            .unwrap_or_else(|| path.to_string());
    }
    match path
        .strip_prefix(cwd)
        .and_then(|rest| rest.strip_prefix(['/', '\\']))
    {
        Some(rest) => rest.to_string(),
        None => path.to_string(),
    }
}

/// Claude Code's `head_limit`: the result when no limit is given.
pub const DEFAULT_HEAD_LIMIT: usize = 250;

/// Skips `offset` items, then keeps `limit` of them (`DEFAULT_HEAD_LIMIT` when `None`,
/// all when 0), as Claude Code's `applyHeadLimit`. The second value is the limit when it
/// cut items off, so that the model knows there are more.
pub fn apply_head_limit<T>(
    items: Vec<T>,
    limit: Option<usize>,
    offset: usize,
) -> (Vec<T>, Option<usize>) {
    let rest = items.into_iter().skip(offset);
    match limit {
        Some(0) => (rest.collect(), None),
        limit => {
            let limit = limit.unwrap_or(DEFAULT_HEAD_LIMIT);
            let mut rest = rest.peekable();
            let kept: Vec<T> = rest.by_ref().take(limit).collect();
            let truncated = rest.peek().is_some();
            (kept, truncated.then_some(limit))
        }
    }
}

#[cfg(test)]
#[path = "ripgrep_tests.rs"]
mod tests;
