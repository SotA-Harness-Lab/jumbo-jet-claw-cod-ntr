# Claude Code compatibility

This fork reads the Claude Code configuration where Claude Code keeps it, in the home directory
and in projects, so skills and instructions are maintained in one place and used by both tools.
Nothing is copied or generated, and there is nothing to set up: when the files below exist, they
are used.

The user's Claude Code configuration directory is `~/.claude`. Setting `CODEX_CLAUDE_COMPAT=0`
in the environment turns the whole mode off.

## What is shared

| Claude Code | In Codex | When a change shows |
| --- | --- | --- |
| `~/.claude/skills/` | A user skill root, beside `~/.agents/skills` | Like any other skill root |
| `.claude/skills/` of a project | A repository skill root, beside `.agents/skills`, in each directory from the project root down to the working directory | Like any other skill root |
| `~/.claude/CLAUDE.md` | Appended to the global instructions, after `$CODEX_HOME/AGENTS.md` | At the next request |
| `~/.claude/rules/**/*.md` without `paths:` | Appended after `CLAUDE.md`, sorted by path | At the next request |
| `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/**/*.md` without `paths:`, `CLAUDE.local.md` | Project instructions, after the project's `AGENTS.md` files | When project instructions are loaded |
| `~/.claude/projects/<project>/memory/` | Claude Code's instructions for writing memories and the `MEMORY.md` index, as project instructions, last; the directory is writable in the sandbox | When project instructions are loaded |
| Rules with `paths:`, and the instruction files of directories below the working directory | Shown to the model once a file they cover is worked on | At once |
| `hooks` in `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json` | Hooks, beside Codex's own | At the next session |
| `env` in the same files | Added to the environment of the commands Codex runs | At the next session |
| Bash rules in `permissions` of `~/.claude/settings.json` | Command rules (execpolicy) | At the next session |
| `~/.claude/agents/**/*.md`, `.claude/agents/**/*.md` | Agent roles | Description at the next session, instructions at each spawn |
| `mcpServers` of `~/.claude.json`, approved servers of `.mcp.json` | MCP servers, beside Codex's own | When MCP servers are started |

- Skills are found as Codex finds its own: nested folders up to six levels deep, directory
  symlinks followed, hidden folders (such as `.trash`) skipped. A skill that is reachable from
  two roots through a symlink is listed once.
- Instruction files are found and shown as Claude Code shows them:
  - Project files are read in every directory from the top of the filesystem down to the
    working directory, in that order, and in each directory in the order of the table. In the
    home directory, `.claude` is the user's own configuration and is not read a second time.
  - Each file is a section headed `Contents of <path> (<kind>):`, under Claude Code's own
    line that these instructions override default behaviour.
  - YAML frontmatter and HTML comment blocks are removed; files that are left empty are skipped.
  - `@<path>` in a file includes another file after it (`@./x`, `@~/x`, `@/abs/x` or
    `@x`), up to five levels deep, not inside code blocks or code spans. A project file may
    include only files under the working directory; `~/.claude/CLAUDE.md` may include any.
  - They are not cut to `project_doc_max_bytes`, which still applies to `AGENTS.md`.
- Project files are skipped, like `AGENTS.md`, while the project is not trusted.
- A short preamble tells the model that the global instructions come from Claude Code, that
  `/<name>` means the skill of that name, and that the hooks they mention may not run in Codex.

## Rules for some files

Claude Code shows a rule whose frontmatter has `paths:`, and the `CLAUDE.md`,
`.claude/CLAUDE.md`, `CLAUDE.local.md` and rules of a directory below the working directory,
only when the model reads a file they cover. Codex does the same after each tool call, for the
files a shell command reads (as Codex's command parser recognises them: `cat`, `sed -n`,
`head` and the like), the files a patch adds, changes or moves to (Claude Code's editing
tools require the file to have been read first), and, with the Claude tool set, the file
`Read`, `Write` or `Edit` names. Each file is shown once per session, as a
developer message headed like the others. The globs follow Claude Code: gitignore syntax,
relative to the working directory for the user's rules and to the directory holding `.claude`
for a project's; `{a,b}` is expanded, and a list or a comma-separated string are both read.
This needs the session's environment to run on this machine.

## Hooks

Codex's hook engine takes the same shape as Claude Code's `hooks` key (events, `matcher`,
`type: "command"`, `timeout`, `async`, `statusMessage`), so the hooks of the settings files
run as they are, with Claude Code's input on stdin and Claude Code's outputs understood.

- The user's own hooks run without Codex's per-hook trust prompt, as they do in Claude Code. A
  project's hooks wait for Codex's trust like any other project hook. `disableAllHooks: true`
  in any of the files turns all of them off.
- Hooks see `CLAUDE_PROJECT_DIR` (the working directory) and `CLAUDE_CODE_ENTRYPOINT`: `cli`
  in the TUI, `sdk-cli` under `codex exec` (as for `claude -p`), `claude-vscode` for the
  IDE; other sessions, subagents among them, get none. `CLAUDE_ENV_FILE` is not provided.
- Input: `session_id` (the Codex thread id), `transcript_path`, `cwd`, `permission_mode`
  (`default`, or `bypassPermissions` when Codex never asks) and `hook_event_name` for every
  event; `tool_name`, `tool_input` (`{"command": ...}` for shell commands) and
  `tool_response` for tool events; `prompt` for `UserPromptSubmit`; `source` (`startup`,
  `resume`, `clear`, `compact`) for `SessionStart`; `stop_hook_active` and
  `last_assistant_message` for `Stop`. `transcript_path` is Codex's rollout file, whose lines
  are Codex's own records, not Claude Code's: a hook that parses the transcript finds none of
  the messages it looks for and should read `last_assistant_message` instead.
- Output: exit code 2 blocks with stderr as the reason, `hookSpecificOutput.permissionDecision`
  for `PreToolUse`, `decision: "block"` with `reason` for `Stop` and `PostToolUse`,
  `additionalContext` for `SessionStart`, `UserPromptSubmit`, `PreToolUse` and
  `PostToolUse`, plain stdout as context for `SessionStart` and `UserPromptSubmit`,
  `systemMessage`, `continue` and `stopReason`.
- Tool names: shell commands are `Bash`; `apply_patch` is also matched by `Write` and `Edit`,
  `spawn_agent` by `Agent` and `Task`, `request_user_input` by `AskUserQuestion` (its input has
  the same `questions[].question`), `update_plan` by `TodoWrite`. Hook input keeps Codex's
  name. `Read`, `MultiEdit`, `Grep`, `Glob`, `WebFetch`, `WebSearch` and `ExitPlanMode` have
  no Codex tool and never match; Codex's plan mode ends with a plan in the final answer, which
  only `Stop` sees.
  With the Claude tool set on (below), `Bash`, `Read`, `Write` and `Edit` are the tools'
  own names: hook input carries that name and Claude Code's input (`command`, `timeout`,
  `description`; `file_path`, `old_string`, `new_string`; ...), and `PostToolUse` gets
  Claude Code's `tool_response` object (`stdout`, `stderr`, `interrupted` for `Bash`).
- Events Codex's own configuration does not have run at the closest Codex moment, for command
  handlers only, and are not listed among Codex's hooks:
  - `Notification` when Codex asks the user: an approval of a command (`permission_prompt`,
    message `Codex needs your permission to use Bash`), of a patch (`... to use
    apply_patch`) or of more permissions (`... to use request_permissions`), and an MCP
    elicitation (`elicitation_dialog`, or `elicitation_url_dialog` for a URL). The matcher is
    matched against `notification_type`. `idle_prompt` and the other types are never sent.
  - `PostToolUseFailure` when a tool call fails, with `error` set to what went wrong. As in
    Claude Code, a shell command that exits with a non-zero code is a failure: its `error` is
    `Exit code N` and the output, and the settings files' `PostToolUse` hooks do not run for
    it (Codex's own `PostToolUse` hooks still do). `additionalContext`, or stderr with exit
    code 2, reaches the model. `is_interrupt` is always `false`: a cancelled call runs no hook.
  - `StopFailure` when a turn ends on an error instead of an answer, with `error` set to
    Claude Code's kind (`rate_limit`, `billing_error`, `authentication_failed`,
    `invalid_request`, `server_error`, `unknown`), matched by the matcher, and
    `error_details` to the message Codex shows.
  `Notification` and `StopFailure` hooks run in the background and their output is ignored,
  as in Claude Code. A project's hooks of these three events run only with
  `--dangerously-bypass-hook-trust`, since Codex's trust prompt has no place for them; without
  it they are skipped with a warning.
- Other events Codex does not have (`PermissionDenied`, `Setup`, `CwdChanged` and the rest)
  are ignored; handlers of type `http` are skipped with a warning; `prompt` and `agent`
  handlers are skipped as for Codex's own hooks.

## Claude tool set

Behind a switch, Codex offers the model Claude Code's own tools instead of its own, so that
the model works with the tools it knows, and hook matchers and tool names in Claude Code's
configuration match one to one. It is off by default; turn it on in `config.toml`:

```toml
[features]
claude_tools = true
```

`CODEX_CLAUDE_COMPAT=0` turns it off again, with the rest of the compatibility.

The Claude tools are a thin layer over Codex's tools. Each takes Claude Code's parameters
and description, hands the work to the Codex tool it replaces, and gives the model Claude
Code's result text. Codex keeps the execution: approval prompts and policies, the sandbox,
command rules (including the Bash rules of `~/.claude/settings.json`), events, the TUI and
the turn's diff are those of `exec_command` and `apply_patch`. A Claude Code feature is added
in the layer only where the Codex tool has nothing for it.

| Claude tool | Replaces | Delegated to Codex | Added by the layer |
| --- | --- | --- | --- |
| `Bash` | `exec_command` | Running the command (one-shot with the timeout; `run_in_background` as a process left running), shell, approval, sandbox, command rules, output truncation, events | Parameters (`command`, `timeout` in ms, default 2 minutes, at most 10; `description`; `run_in_background`; `dangerouslyDisableSandbox`, sent as `sandbox_permissions: "require_escalated"` with the description as justification), the result text: output trimmed, `Exit code N` and the output for a failed command, `(Bash completed with no output)` |
| `Read` | `view_image` | Reading through the executor filesystem under the turn's sandbox; images (`png`, `jpg`, `jpeg`, `gif`, `webp`) to `view_image` | Parameters (`file_path`, `offset`, `limit`), `N<TAB>line` numbering from 1, the 256 KB limit without `limit`, the 25,000-token limit, refusal of binary files and directories, the empty-file and short-file reminders, the "file does not exist" message |
| `Write` | `apply_patch` | Writing, as a patch: approval, sandbox, patch events, the turn's diff | Parameters (`file_path`, `content`), the rule that an existing file is read first and not changed since, messages |
| `Edit` | `apply_patch` | Writing, as a patch, as for `Write` | Parameters (`file_path`, `old_string`, `new_string`, `replace_all`), exact matching (refused when not found, or found more than once without `replace_all`), the read-first rule, messages |

`write_stdin` stays, to read the output of a background command and to stop it. Every other
Codex tool (the plan tool, `request_user_input`, the agent tools, web search, MCP tools and
the rest) is offered as before until its Claude Code counterpart is added.

Differences from Claude Code:

- Each `Bash` command starts in the session's working directory; `cd` does not carry over to
  the next command, as it does in Claude Code. The description says so.
- A background command is read and stopped with `write_stdin`, not through an output file
  and the `TaskOutput` and `TaskStop` tools; the model is not notified when it ends.
- The description of `Bash` keeps Claude Code's instructions for commits and pull requests,
  without attribution lines, and drops Claude Code's sandbox settings for a short note on
  Codex's sandbox.
- `Read` does not render PDFs or notebooks: a PDF is refused as a binary file, a notebook is
  read as its JSON. It does not shorten long lines, and reads the whole file when it is
  within the limits (as Claude Code does now; its description still says 2000 lines).
  Images need the model to take images and Codex's `view_image` feature.
- `Write` and `Edit` go through `apply_patch`, which ends every file with a line break and
  gives new lines the file's first line ending; untouched lines keep theirs. A file that is
  not UTF-8 cannot be edited or overwritten. The patch holds the whole file as context, so it
  can only apply to the file as it was read; a file changed in between makes the write fail.
- The read-first rule follows the session: a file read or written in the session may be
  edited until it changes on disk. Claude Code's "file unchanged" short answer for a second
  read of the same range is not reproduced.
- `MultiEdit` is not offered (current Claude Code no longer has it).
- Hooks and Codex's own `hooks.json` see the Claude names, so a Codex hook matching
  `exec_command` or `apply_patch` does not run for these tools.
- Auto-review (guardian) of past tool calls recognises only Codex's tool names.

### Adding a tool

The layer is in `codex-rs/core/src/tools/claude/`, one file a tool; what does not depend on
Codex's types (descriptions, parameter texts, result formatting, the edit itself) is in
`codex-rs/claude-compat/src/tools/`, also one file a tool, with its tests beside it. A new
tool:

1. Adds `<tool>.rs` with an `install(swap: &mut ToolSwap)` that takes the Codex runtime it
   delegates to with `swap.take("<codex tool>")` (which removes that tool from what the model
   sees) and adds a handler named after the Claude Code tool.
2. Adds one `<tool>::install(&mut swap)` line to `replace_codex_tools` in `mod.rs`, and its
   name to `OFFERED_TOOLS` so that the other descriptions point to it.
3. Builds the Codex call with `delegated(&invocation, payload)`, which keeps the session,
   turn, cancellation and diff tracker of the model's call, and returns a `ClaudeToolOutput`
   with Claude Code's text and `tool_response`.

Hook names need nothing more: a function tool reports its own name. `Bash` alone is
registered from `add_shell_tools`, which owns the shell options.

## Settings

- `env`: the user's file, then the project's and the local one, later files winning, are
  added to the environment of the commands Codex runs. `shell_environment_policy.set` in
  `config.toml` wins over them.
- `permissions`: a Bash rule that names a command prefix (`Bash(git status)`,
  `Bash(git diff*)`, `Bash(gh pr *)`, `Bash(npm run:*)`) becomes a Codex command rule of the
  same prefix, word by word: `allow` runs without asking (and, as for any Codex allow rule,
  outside the sandbox), `ask` asks, `deny` forbids; the strictest matching rule wins, as in
  Claude Code. A word cut short by `*` (`--force*`) matches only that whole word. Only the
  user's file is read. Rules for other tools, wildcards inside a command, `defaultMode` and
  `additionalDirectories` are not applied.

## Subagents

A Claude Code subagent becomes an agent role of the same name: its `description` is the
role's description, its body the role's developer instructions, and its `effort` the
reasoning effort (`max` as `xhigh`). `tools`, `model` and the other fields are left out.
Codex's own roles win over subagents of the same name, and a project's subagents over the
user's. Only `.claude/agents` of the working directory is read for the project.

## MCP servers

The servers are taken as Claude Code takes them: the user's `mcpServers` of `~/.claude.json`,
then the `.mcp.json` files from the top of the filesystem down to the working directory (a
server there only once approved by `enabledMcpjsonServers` or `enableAllProjectMcpServers`,
and not listed in `disabledMcpjsonServers`, in the settings files or in the project's entry of
`~/.claude.json`), then the servers `~/.claude.json` keeps for the project. `${VAR}` and
`${VAR:-default}` are expanded. A server of a name Codex configures itself is left to Codex's
configuration. Codex speaks stdio and streamable HTTP; an `sse` server does not start.

## Auto memory

Codex keeps the project's auto memory with Claude Code, in the same directory and the same way,
so a memory saved by one tool is recalled by the other.

- The directory is `~/.claude/projects/<project>/memory/`, `<project>` being the path of the
  main checkout of the Git repository (or of the working directory) with every character but
  letters and digits replaced by `-`. As in Claude Code, `autoMemoryDirectory` in
  `~/.claude/settings.json` or the project's `.claude/settings.local.json` puts it elsewhere
  (`~/` is expanded; the project's shared `.claude/settings.json` cannot move it), and so do
  `CLAUDE_COWORK_MEMORY_PATH_OVERRIDE` and `CLAUDE_CODE_REMOTE_MEMORY_DIR`.
- Claude Code's instructions for the memory, with the directory filled in, are shown after the
  project's instructions, followed by the index, `MEMORY.md`, cut to 200 lines and 25,000 bytes
  as in Claude Code. The instructions are Claude Code's own: the four types of memory, one file
  per memory with `name`, `description` and `type` frontmatter, a one-line pointer to it in
  `MEMORY.md`, what not to save, when to read memory and how far to trust it. Where Claude Code
  names its own tools, they name Codex's: files are written with `apply_patch`, the task list
  is `update_plan`, and `AGENTS.md` is named beside `CLAUDE.md` among the files whose content is
  not to be saved again. `CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES` is appended, as in Claude Code.
  Claude Code puts the instructions in its system prompt and the index among the `CLAUDE.md`
  files; Codex shows both together in the project instructions.
- The directory is created when the session's configuration is loaded, as Claude Code creates
  it at the start of a session. When the session may write its workspace (`workspace-write`),
  the directory is added to the sandbox's writable roots, so memories are written without an
  approval, as in Claude Code; a read-only session stays read-only, and full access needs
  nothing. Requirements that do not allow the extra root leave the sandbox as it was. Changing
  the permissions during a session (`/permissions`) drops the extra root; to keep it whatever
  the mode, add it to `config.toml` yourself:

  ```toml
  [sandbox_workspace_write]
  writable_roots = ["<MEMORY_DIR>"]
  ```

- Auto memory is off, as in Claude Code, when `CLAUDE_CODE_DISABLE_AUTO_MEMORY` is `1` or
  `true` (`0` or `false` turns it on whatever the settings say), when `CLAUDE_CODE_SIMPLE` is
  set, on Claude Code on the web without `CLAUDE_CODE_REMOTE_MEMORY_DIR`, or when the last of
  the settings files to set `autoMemoryEnabled` sets it to `false`. The variables are taken from
  the `env` of the settings files first, then from Codex's own environment. When it is off,
  neither the instructions nor the index are shown and nothing is made writable. Like the other
  project files, nothing is shown or made writable while the project is not trusted.
- Not reproduced: Claude Code's background extraction of memories at the end of a turn, its
  `/memory` and `/remember` commands, team memory, the daily logs of its assistant mode, and the
  search of past transcripts that some of its builds add to the instructions. `CLAUDE_CONFIG_DIR`
  does not move the memory, as Codex always reads `~/.claude`.

## Skill frontmatter

Codex reads these Claude Code fields of `SKILL.md`, in skills from any root:

| Field | In Codex |
| --- | --- |
| `disable-model-invocation: true` | The skill is not offered to the model; `$<name>` still runs it. An explicit `allow_implicit_invocation` in the skill's `agents/openai.yaml` wins |
| `description` missing or empty | The first non-empty line of the body stands in, cut to 100 characters |
| `when_to_use` | Appended to the description as ` - <text>` |
| `arguments` | Names for the arguments of a skill invoked as `/<name> <args>` (see below) |

To hide a Claude Code skill from Codex altogether (one that only makes sense in Claude Code,
or one that duplicates a Codex skill), disable it in `config.toml`:

```toml
[[skills.config]]
name = "<SKILL_NAME>"
enabled = false
```

## Invoking skills

In the TUI, `/<name> <args>` invokes the skill `<name>` as in Claude Code, for any skill Codex
lists (Claude Code's and Codex's own). A built-in command of the same name wins. Once a name is
typed after `/`, the slash popup lists the matching skills after the commands; Enter or Tab
completes the name. The message is sent as typed, with the skill attached as `$<name>` would
attach it. This works with `CODEX_CLAUDE_COMPAT=0` too, since such a name was an error before;
only the expansion below is turned off.

When the skill's `SKILL.md` is a skill on this machine and Codex itself puts it into the
conversation, it is expanded as Claude Code expands it:

- Invoked as `/<name> <args>` (the message starts with it): the frontmatter is left out and the
  body is headed `Base directory for this skill: <dir>`. `$ARGUMENTS` becomes the arguments as
  typed; `$ARGUMENTS[N]` and `$N` the N-th word (counted from 0, split as a shell splits words,
  with quotes); `$<name>` the word at the position of `<name>` in the `arguments` frontmatter (a
  list, or names separated by spaces). A placeholder past the last word becomes empty. When the
  body has no placeholder and arguments were given, they are appended as `ARGUMENTS: <args>`.
- `${CLAUDE_SKILL_DIR}` becomes the skill's directory and `${CLAUDE_SESSION_ID}` the thread's
  id, also when the skill is mentioned as `$<name>`. A `$<name>` mention keeps the rest of the
  text as it is, `$ARGUMENTS` included.

Not done:

- A skill the model opens by itself, reading `SKILL.md` with a shell command, is read as the
  file is: Codex does not see the read as an invocation, so nothing is expanded. Claude Code
  expands these too, because its model invokes skills through a Skill tool with arguments.
- Shell injection (```` ```! ```` blocks and inline ``!`command` ``) is not run: Codex puts the
  skill into the conversation before the turn starts, where no command runs under the session's
  sandbox and approval rules. The text stays as written, so the model sees the commands and can
  run them itself, with the usual approvals.
- Skills that are not files on this machine (cloud and remote-environment skills) are not
  expanded. Shell operators and globs in the arguments are kept as words, where Claude Code
  drops them.
- `/<name>` is a TUI feature: `codex exec` and other clients attach a skill with `$<name>`.
  They get the `/<name> <args>` expansion only when they send the text together with the skill
  as a structured skill input, as the TUI does.
- `user-invocable: false` does not hide a skill from the slash popup.

## What is not shared

- Slash commands (`commands/**/*.md`) and plugins.
- Other skill frontmatter (`allowed-tools`, `model`, `context`, `hooks`, `user-invocable`,
  `argument-hint` and the like) is ignored.
- Managed (enterprise) Claude Code files under `/etc/claude-code`.

## Size

The global instructions are never truncated. When they are larger than `project_doc_max_bytes`
(32 KiB by default), Codex prints a warning at startup; raise that setting to silence it and to
leave room for project instructions.
