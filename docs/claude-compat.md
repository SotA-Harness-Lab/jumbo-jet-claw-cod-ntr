# Claude Code compatibility

This fork reads the Claude Code configuration where Claude Code keeps it, in the home directory
and in projects, so skills and instructions are maintained in one place and used by both tools.
Nothing is copied or generated, and there is nothing to set up: when the files below exist, they
are used.

The user's Claude Code configuration directory is `~/.claude`. Setting `CCN_CLAUDE_COMPAT=0`
in the environment turns the whole mode off.

## The fork's own settings (ccn-config.toml)

Settings that only this fork has are kept in their own file, `$CODEX_HOME/ccn-config.toml`
(`~/.codex/ccn-config.toml` by default), never in Codex's `config.toml`, which keeps only upstream
Codex's settings. The file is read once each time the configuration is loaded, at the start of a
session. Every key is optional; a missing file means the defaults. A file that does not parse,
or that names a key or section not listed here, is reported as one startup warning and the
defaults are used in its place; Codex still starts.

```toml
[claude]
# Offer the model Claude Code's tool set in place of Codex's (see "Claude tool set").
tools = false

[claude.plugins]
# Claude Code plugins to use even when they are found incompatible with Codex
# (see "Which plugins are used"), by their <plugin>@<marketplace> id.
enable = []
```

| Key | Type | Default | Effect |
| --- | --- | --- | --- |
| `claude.tools` | boolean | `false` | The Claude tool set in place of Codex's tools |
| `claude.plugins.enable` | list of strings | `[]` | Claude Code plugins used even when found incompatible |

`CCN_CLAUDE_COMPAT=0` overrides both: with it, neither the Claude tool set nor Claude Code's
plugins are used.

## What is shared

| Claude Code | In Codex | When a change shows |
| --- | --- | --- |
| `~/.claude/skills/` | A user skill root, beside `~/.agents/skills` | Like any other skill root |
| `.claude/skills/` of a project | A repository skill root, beside `.agents/skills`, in each directory from the project root down to the working directory | Like any other skill root |
| `~/.claude/commands/`, `.claude/commands/` of a project | Skills named after the file's path, see "Custom slash commands" | Like any other skill root |
| `~/.claude/CLAUDE.md` | Appended to the global instructions, after `$CODEX_HOME/AGENTS.md` | At the next request |
| `~/.claude/rules/**/*.md` without `paths:` | Appended after `CLAUDE.md`, sorted by path | At the next request |
| `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/**/*.md` without `paths:`, `CLAUDE.local.md` | Project instructions, after the project's `AGENTS.md` files | When project instructions are loaded |
| `~/.claude/projects/<project>/memory/` | Claude Code's instructions for writing memories and the `MEMORY.md` index, as project instructions, last; the directory is writable in the sandbox | When project instructions are loaded |
| Rules with `paths:`, and the instruction files of directories below the working directory | Shown to the model once a file they cover is worked on | At once |
| `hooks` in `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json` | Hooks, beside Codex's own | At the next session |
| `env` in the same files | Added to the environment of the commands Codex runs | At the next session |
| Bash rules in `permissions` of `~/.claude/settings.json` | Command rules (execpolicy), while the Claude tool set is off | At the next session |
| `permissions.allow`, `ask`, `deny` of the three settings files | Checked by the Claude tool set before each call, while it is on | At the next session |
| `~/.claude/agents/**/*.md`, `.claude/agents/**/*.md` | Agent roles | Description at the next session, instructions at each spawn |
| `mcpServers` of `~/.claude.json`, approved servers of `.mcp.json` | MCP servers, beside Codex's own | When MCP servers are started |
| Plugins installed and enabled in Claude Code | Codex plugins, loaded from their install paths: skills, MCP servers, hooks | At the next session |

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
  name. Without the Claude tool set, `Read`, `Grep`, `Glob`, `WebFetch` and `WebSearch` have
  no Codex tool and never match. `MultiEdit` (gone from Claude Code) and `ExitPlanMode` never
  match: Codex's plan mode ends with a plan in the final answer, which only `Stop` sees.
  With the Claude tool set on (below), `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`,
  `WebFetch`, `WebSearch`, `TodoWrite`, `AskUserQuestion` and `Agent` are the tools' own names: hook input carries that name and Claude Code's input (`command`, `timeout`,
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
configuration match one to one. It is off by default; turn it on in `$CODEX_HOME/ccn-config.toml`
(see "The fork's own settings"):

```toml
[claude]
tools = true
```

`CCN_CLAUDE_COMPAT=0` turns it off again, with the rest of the compatibility.

The Claude tools are a thin layer over Codex's tools. Each takes Claude Code's parameters
and description, hands the work to the Codex tool it replaces, and gives the model Claude
Code's result text. Codex keeps the execution: approval prompts and policies, the sandbox,
command rules, events, the TUI and the turn's diff are those of `exec_command` and
`apply_patch`. A Claude Code feature is added in the layer only where the Codex tool has
nothing for it; Claude Code's permission rules are one (see "Permission rules" below).

| Claude tool | Replaces | Delegated to Codex | Added by the layer |
| --- | --- | --- | --- |
| `Bash` | `exec_command` | Running the command (one-shot with the timeout; `run_in_background` as a process left running), shell, approval, sandbox, command rules, output truncation, events | Parameters (`command`, `timeout` in ms, default 2 minutes, at most 10; `description`; `run_in_background`; `dangerouslyDisableSandbox`, sent as `sandbox_permissions: "require_escalated"` with the description as justification), the result text: output trimmed, `Exit code N` and the output for a failed command, `(Bash completed with no output)` |
| `Read` | `view_image` | Reading through the executor filesystem under the turn's sandbox; images (`png`, `jpg`, `jpeg`, `gif`, `webp`) to `view_image` | Parameters (`file_path`, `offset`, `limit`), `N<TAB>line` numbering from 1, the 256 KB limit without `limit`, the 25,000-token limit, refusal of binary files and directories, the empty-file and short-file reminders, the "file does not exist" message |
| `Write` | `apply_patch` | Writing, as a patch: approval, sandbox, patch events, the turn's diff | Parameters (`file_path`, `content`), the rule that an existing file is read first and not changed since, messages |
| `Edit` | `apply_patch` | Writing, as a patch, as for `Write` | Parameters (`file_path`, `old_string`, `new_string`, `replace_all`), exact matching (refused when not found, or found more than once without `replace_all`), the read-first rule, messages |
| `Glob` | none (offered with `Bash`) | Listing and sorting, as `rg --files --glob <pattern> --sort=modified --no-ignore --hidden` run through `exec_command` (sandbox, approval, command rules, events); the directory check through the executor filesystem | Parameters (`pattern`, `path`), an absolute pattern split into its directory and the rest, the first 100 files and the truncation note, paths inside the working directory shortened, `No files found`, the "directory does not exist" and "not a directory" messages |
| `Grep` | none (offered with `Bash`) | Searching, as one `rg` command run through `exec_command` as for `Glob`, files listed newest first by `rg --sortr=modified`; the path check through the executor filesystem | Parameters (`pattern`, `path`, `glob`, `type`, `output_mode`, `-i`, `-n`, `-A`, `-B`, `-C`, `context`, `multiline`, `head_limit`, `offset`), Claude Code's ripgrep arguments (hidden files, VCS directories left out, `--max-columns 500`), `head_limit` (250 by default, 0 for all) and `offset` with the pagination note, paths shortened, the text of each output mode (`Found N files`, `No matches found`, `Found N total occurrences across M files.`), the "path does not exist" message |
| `WebSearch` | `web.run` (standalone search) | The search, its backend and settings (`web_search` mode, location, filters), its events and the TUI | Parameters (`query`, at least 2 characters; `allowed_domains`, sent as the search's domain filter; `blocked_domains`, sent as `-site:` operators in the query; not both), the result text with Claude Code's sources reminder |
| `WebFetch` | nothing (new) | The request, as a `curl` command through `exec_command`: sandbox, managed network proxy and its host approvals, command rules, approval prompts, events; the answer to `prompt` from the session's model client | Parameters (`url`, `prompt`), URL checks (at most 2000 characters, no credentials, a dotted host), HTTP upgraded to HTTPS, same-site redirects (`www.` added or removed) followed up to 10, other redirects reported in Claude Code's format, the 10 MB and 60-second limits, HTML to Markdown, the 15-minute cache, the secondary model's prompt with Claude Code's quoting rules and preapproved documentation hosts, the result text |
| `TodoWrite` | `update_plan` | The list, sent as the plan: the TUI's plan display, plan events, the refusal in Plan mode | Parameters (`todos` of `content`, `status`, `activeForm`), the checks (both forms non-empty), description, result text; the step in progress shows its `activeForm` |
| `AskUserQuestion` | `request_user_input` | Asking: the TUI's question panel, the modes it is offered in, the root-thread rule, cancellation, recorded answers | Parameters (1-4 `questions` of `question`, `header`, 2-4 `options` of `label` and `description`, `multiSelect`), the uniqueness checks, description, result text (`"question"="answer"`, with `user notes:`) |
| `Agent` | `spawn_agent` (multi-agent v1 or v2, whichever the turn has) | Starting the agent (roles, models, depth and thread limits, sandbox inheritance, TUI events), waiting through Codex's agent status subscription (as `wait_agent` does), the completion notice of a background agent | Parameters (`description`, `prompt`, `subagent_type`, `model`, `run_in_background`), the description listing the agent types with their tools, `general-purpose` as Codex's default role, Claude Code's message for an unknown type, result texts (the agent's final message) |

`write_stdin` stays, to read the output of a background command and to stop it. Codex's
other agent tools (`wait_agent`, `send_input`, `close_agent` and the rest, or v2's
`send_message`, `followup_task`, `list_agents`) stay beside `Agent`. Every other Codex tool
(the hosted web search, MCP tools and the rest) is offered as before.

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
- `Glob` and `Grep` run `rg` in the session's environment. Codex's own release packages ship
  one and put it on the path of every command Codex runs; a Codex built from source has none, so
  install ripgrep there. Without it they fail with a message saying so. Each search is a command
  of the turn, so it shows as one, and under an approval policy that asks for commands it is
  asked for. The search stops after 20 seconds, as in Claude Code.
- `Grep` lists files newest first through ripgrep's own sort, which also sorts files that do
  not match and searches on one thread; files with the same time keep ripgrep's order, not
  the order of their names. When ripgrep fails without a result (an invalid pattern, a path
  it cannot read), the model is told ripgrep's message where Claude Code says no match was
  found. The deny rules of Claude Code's `Read` permissions are not left out of the search,
  and paths are not suggested for one that does not exist.
- Output past Codex's capture of 1 MiB loses its middle before `Glob` and `Grep` see it, and
  a long result is truncated as any tool output in Codex's history, where Claude Code saves
  results over 20,000 characters to a file.
- Hooks and Codex's own `hooks.json` see the Claude names, so a Codex hook matching
  `exec_command` or `apply_patch` does not run for these tools.
- Auto-review (guardian) of past tool calls recognises only Codex's tool names.
- `WebSearch` is offered only where Codex runs its standalone search (`web.run`, with the
  `standalone_web_search` feature or a Responses Lite model). Codex's usual search is the
  hosted `web_search` tool, which the model provider runs inside the model's response: the
  model calls it in its own format, not as a function Codex could rename or give other
  parameters, so with it the model keeps Codex's `web_search`, without `allowed_domains`
  and `blocked_domains` (allowed domains can be set for every search with
  `[tools.web_search] filters.allowed_domains` in `config.toml`), and
  hooks matching `WebSearch` never run. Search MCP tools stay as they are.
- `WebSearch` results are Codex's search output wrapped in Claude Code's text, not Claude
  Code's list of links; `blocked_domains` is only as good as the engine's `-site:`
  support. Only plain web search is offered: opening pages goes through `WebFetch`.
- `WebFetch` needs `Bash` (the model may run commands) and `curl` on the machine that runs
  commands (`curl.exe` under PowerShell or `cmd`). The request goes out from there, under
  the session's rules: with network in the sandbox, or a managed network proxy (which
  allows, denies or asks per host), it runs in the sandbox; otherwise it asks to run
  outside it, as `sandbox_permissions: "require_escalated"`, which Codex approves or
  refuses by its approval policy (refused under `never`). The approval shows the `curl`
  command, and command rules for `curl` apply to it. `WebFetch(domain:...)` permission rules
  apply (see "Permission rules"); Claude Code's preapproved documentation hosts never allow a
  request by themselves.
- The answer to `prompt` comes from the session's model at low reasoning effort, not from
  a separate small model, and its tokens are not counted in the session's usage.
- `WebFetch` decodes every body as UTF-8 and does not save binary content (such as PDFs)
  to a file. Command output is capped at 1 MiB, so the middle of a larger page is lost
  before conversion. The cache is per session.
- Hooks see `WebFetch` and `WebSearch` under these names with Claude Code's input;
  `PostToolUse` gets Claude Code's `tool_response` (`bytes`, `code`, `codeText`,
  `result`, `durationMs`, `url`; `query`, `results`, `durationSeconds`).
- `TodoWrite` is offered even where Codex's own plan tool is off (`[tools.update_plan]` is off
  by default), since Claude Code always has it; a tool policy that removes `update_plan` removes
  it too.
- `TodoWrite` keeps a list whose items are all completed (Claude Code clears it), and its
  hook `tool_response` has `newTodos` but no `oldTodos`.
- `AskUserQuestion` asks only where Codex's `request_user_input` does: in Plan mode, in
  Default mode with the `default_mode_request_user_input` feature, and in the root agent;
  elsewhere the model gets Codex's refusal. Codex's panel takes one choice a question: a
  `multiSelect` question asks for the other choices in a note. Option `preview`s are not
  offered.
- `Agent` has no `isolation`, `name`, `team_name` or `mode`, and an agent cannot be
  continued with `SendMessage`; Codex's own agent tools stay for that. The `model` parameter
  takes a model name Codex knows, not `sonnet`, `opus` or `haiku`. A foreground agent of
  multi-agent v1 is closed once it has answered, to free its thread. A background agent's
  result is added to the conversation when it ends, without starting a turn; the model is
  told to use `wait_agent` to wait for it. Codex also adds its own completion notice after a
  foreground agent's answer. `Agent(type, ...)` in a tool list does not restrict the types.

### Permission rules

With the tool set on, each Claude tool call is checked against the `permissions.allow`, `ask`
and `deny` rules of `~/.claude/settings.json`, `.claude/settings.json` and
`.claude/settings.local.json` before it runs, matched as Claude Code matches them, and the
decision is handed to Codex's own approval path:

| Rule | What happens |
| --- | --- |
| `deny` | The tool does not run; the model gets Claude Code's denial (`Permission to use Bash with command ... has been denied.`, `Permission to read ... has been denied.`, ...). |
| `allow` | A `Bash`, `Write` or `Edit` call Codex would ask about runs without asking. It still runs in the sandbox: the rule settles the approval Codex's orchestrator decides for `exec_command` and `apply_patch`, not the sandbox, and it does not cover a request to leave the sandbox (`dangerouslyDisableSandbox`), a command Codex forbids, or a managed rule that wants a person. An MCP call skips Codex's MCP approval. Tools Codex never asks about (`Read`, ...) run as before. |
| `ask` | A `Bash`, `Write` or `Edit` call asks even where Codex would not, with the rule as the reason, and no "approve for the session" is remembered for later calls. A tool Codex has no approval for (`Read`, `Glob`, `Grep`, `WebFetch`, MCP tools, ...) asks with Codex's approval prompt, showing the tool and its input; declining gives the model Claude Code's rejection. When the approval policy cannot ask (`never`, or `granular` without rule approvals), the call is denied, as Claude Code denies in a non-interactive session. |
| none | Codex decides, as for its own tools. |

Deny wins over ask, ask over allow; a rule for a whole tool (`Bash`, `WebSearch`) is looked
at first for deny and ask, last for allow. The rules mean what they mean in Claude Code:

- `Bash(...)`: an exact command; a legacy prefix `npm run:*` (the word itself, or followed by a
  space, also after `xargs`); or a pattern with `*` wildcards (`git * main`; `\*` is a literal
  star; a single trailing ` *` also matches the bare command). A compound command is taken apart
  at `&&`, `||`, `;`, `|`, `&` and line breaks outside quotes, heredoc bodies left out, a leading
  `cd <working directory>` dropped: a deny or ask rule for any part decides the whole, an allow
  rule allows it only when every part is allowed (or the whole command is an exact rule).
  Output redirections to plain files, safe variable assignments (`LANG=C`, `NODE_ENV=...`, the
  list Claude Code keeps) and the wrappers `timeout`, `time`, `nice`, `stdbuf`, `nohup` are
  dropped before matching; deny and ask rules drop any variable assignment. A prefix or wildcard
  allow rule never matches a part that still looks compound. Several `cd`s, or `cd` with
  `git`, are never allowed by rules. A command Claude Code's parser does not take apart
  (`$(...)`, backticks, `<(...)`, subshells, groups, `if`/`for`/`while`/`case`) is decided only
  by an exact rule or a deny rule on the whole text; otherwise Codex decides (Claude Code would
  ask).
- `Read(...)` and `Edit(...)`: gitignore patterns, compared without regard to case. `//path` is
  absolute, `~/path` is in the home directory, `/path` is relative to the settings file's root
  (`~/.claude` for the user's file, the project directory for the project's and the local
  one), anything else (`*.env`, `./src/**`) is relative to the working directory, and a pattern
  without a slash matches at any depth. `dir/**` covers the directory and everything in it.
  `Read` rules govern `Read`, `Glob` and `Grep`; `Edit` rules govern `Edit` and `Write`
  (`Write(...)` rules with a path do nothing, as in Claude Code); an `Edit` allow rule also
  allows reading. Allow rules never cover files in `.git`, `.vscode`, `.idea` or `.claude`
  (except `.claude/worktrees`), shell profiles, `.gitconfig`, `.gitmodules`, `.ripgreprc`,
  `.mcp.json` and `.claude.json`: Claude Code asks about those whatever the rules say, and here
  Codex decides.
- `WebFetch(domain:<host>)`: the URL's host name, exactly.
- `mcp__<server>__<tool>`, `mcp__<server>`, `mcp__<server>__*`: an MCP tool, or every tool of a
  server. Codex's MCP tools are checked under the same names.
- `Agent(<type>)` and other tools' content: compared exactly. `Task` is read as `Agent`.

Not applied: `defaultMode` (Codex's approval policy and sandbox mode stand for it, and none of
Claude Code's modes is one of Codex's policies), `additionalDirectories` (in Claude Code they
let the model read there without asking but still ask before edits; Codex reads without asking
anyway, and making them writable roots would let a project's settings open the sandbox), the
managed (`policySettings`) file, rules given on the command line, Claude Code's own checks that
are not rules (its read-only command list, its path checks for commands, the classifier), and
the hiding of files denied to `Read` from `Glob` and `Grep` results. The settings files are read
once a session for each project directory. Paths are compared as written (made absolute and
normalised), without resolving symbolic links.

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
4. Before delegating, checks the permission rules with one line, keeping the guard until the
   delegated call is done: `let _permission = check_permission(&invocation, <TOOL>,
   PermissionInput::<input>).await?;`, where the input is `Tool` (nothing but the name:
   `TodoWrite`, `WebSearch`, `AskUserQuestion`), `ReadPath(&path)` (`Glob`, `Grep`: the
   absolute directory searched), `EditPath(&path)`, `Command(&command)`, `Url(&url)`
   (`WebFetch`) or `Content(&text)` (`Agent`: the subagent type).

Hook names need nothing more: a function tool reports its own name. `Bash` alone is
registered from `add_shell_tools`, which owns the shell options.

## Settings

- `env`: the user's file, then the project's and the local one, later files winning, are
  added to the environment of the commands Codex runs. `shell_environment_policy.set` in
  `config.toml` wins over them.
- `permissions`, with the Claude tool set on: all of `allow`, `ask` and `deny`, from the user's
  file, the project's and the local one, applied exactly as Claude Code applies them; see
  "Permission rules" under "Claude tool set" for what is enforced and how. The approximation
  below is then not loaded.
- `permissions`, with the Claude tool set off: a Bash rule that names a command prefix
  (`Bash(git status)`, `Bash(git diff*)`, `Bash(gh pr *)`, `Bash(npm run:*)`) becomes a Codex
  command rule of the same prefix, word by word: `allow` runs without asking (and, as for any
  Codex allow rule, outside the sandbox), `ask` asks, `deny` forbids; the strictest matching
  rule wins, as in Claude Code. A word cut short by `*` (`--force*`) matches only that whole
  word. Only the user's file is read. Rules for other tools, wildcards inside a command,
  `defaultMode` and `additionalDirectories` are not applied.

## Subagents

A Claude Code subagent becomes an agent role of the same name: its `description` is the
role's description, its body the role's developer instructions, and its `effort` the
reasoning effort (`max` as `xhigh`). `model` and the other fields are left out.
Codex's own roles win over subagents of the same name, and a project's subagents over the
user's. Only `.claude/agents` of the working directory is read for the project.

`tools` and `disallowedTools` apply only with the Claude tool set on (`[claude] tools`): an
agent spawned with the role then keeps only the tools its `tools` lists (all when absent,
none when empty, all with `*`) and its `disallowedTools` does not, by the names the model
sees: Claude Code's names, `mcp__<server>__<tool>` for MCP tools, Codex's own name for a
Codex tool with no Claude counterpart. `Bash(git *)` counts as `Bash`, `Task` as `Agent`;
a Codex tool that serves a Claude tool goes with it (`write_stdin` with `Bash`, Codex's
other agent tools with `Agent`). The list is read from the subagent file each time the
agent's tools are built, and applied after MCP and dynamic tools are added; hosted tools
(Codex's built-in web search) and the tools code mode and tool search add are not filtered.
Unlike Claude Code, a subagent may start agents of its own within Codex's depth limit.
Without the switch the two fields are ignored, as before. The `Agent` description lists
each type with its tools, as Claude Code does.

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

## Custom slash commands

Claude Code's custom slash commands are loaded in place as skills, by the same loader as the
skills, so they are listed, invoked and expanded as skills are:

| Claude Code | Name in Codex |
|---|---|
| `~/.claude/commands/<name>.md` | `<name>` (user) |
| `.claude/commands/<name>.md` of the project and the directories up to the working directory | `<name>` (repo) |
| `<plugin>/commands/<name>.md` of a Claude Code plugin that is used | `<plugin>:<name>` |

- A file in a subdirectory is named after its path, the directories joined with `:`:
  `commands/git/commit.md` is `git:commit`, invoked as `/git:commit`. Frontmatter `name` is
  ignored, as Claude Code ignores it.
- Frontmatter is optional. `description` (or else the first line of the body) is the description
  the model sees; `disable-model-invocation: true` keeps the command out of the model's list, so
  only `/<name>` runs it. `$ARGUMENTS`, `$1`... and `arguments` work as for skills.
- Plugins whose commands Codex migrated into skills when it installed them (Codex's own plugin
  format) keep those; the commands of Claude Code's plugins are never copied.
- `CCN_CLAUDE_COMPAT=0` leaves out the user's and the projects' commands with the skills.

## Invoking skills

In the TUI, `/<name> <args>` invokes the skill `<name>` as in Claude Code, for any skill Codex
lists (Claude Code's and Codex's own). A built-in command of the same name wins. Once a name is
typed after `/`, the slash popup lists the matching skills after the commands; Enter or Tab
completes the name. The message is sent as typed, with the skill attached as `$<name>` would
attach it. This works with `CCN_CLAUDE_COMPAT=0` too, since such a name was an error before;
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

## Plugins

Codex's plugin loader already reads Claude Code's plugin format (`.claude-plugin/plugin.json`,
`skills/`, `.mcp.json`, `hooks/hooks.json`, `${CLAUDE_PLUGIN_ROOT}`); what it does not know is
which plugins the user has. Those are taken from Claude Code's own records:

- Installed: `~/.claude/plugins/installed_plugins.json`. A plugin is read from the
  `installPath` of the first installation recorded for it, as Claude Code does at startup,
  whatever its scope.
- Enabled: `enabledPlugins` of `~/.claude/settings.json`, the project's `.claude/settings.json`
  and `.claude/settings.local.json`, merged in that order, a later file's value for a
  `<plugin>@<marketplace>` id winning. Only `true` enables; an id that is absent or `false` is
  not loaded, as in Claude Code. An enabled id that is not installed is reported at startup.

Each plugin that is both is handed to Codex's plugin loader as an enabled plugin of the same id,
at its install path: nothing is copied, and Codex's `config.toml` is not written. Codex then
loads it as one of its own: skills as `<plugin>:<skill>`, MCP servers, hooks. A plugin Codex
configures under the same id, or has installed under the same name from any marketplace, wins,
and the Claude Code one is left out. `[plugins]` policy in Codex's configuration applies to these
plugins as to Codex's own, and `CCN_CLAUDE_COMPAT=0` or turning Codex's `plugins` feature off
leaves them all out.

### Which plugins are used

Every plugin that is installed and enabled is used, whatever it contains: one that brings only
commands, only a script or only hooks is used like one with skills. A plugin is disabled whole,
automatically, only when it relies on a mechanism of Claude Code's that Codex does not have, so
that in Codex it could misbehave or change things wrongly. That is the case when either:

- it declares a component Codex has no counterpart for: output styles (`outputStyles` in
  `.claude-plugin/plugin.json`, or an `output-styles/` directory when the manifest declares
  none), LSP servers (`lspServers`, or a `.lsp.json` file) or channels (`channels`); or
- one of its files configures Claude Code's own interface. The files read are those under
  `commands/`, `agents/`, `skills/`, `hooks/` and `scripts/`, and under the paths the manifest
  declares for `commands`, `agents`, `skills` and `hooks`, up to six levels deep and 1 MiB each;
  README files, hidden directories and `node_modules` are not read, nor anything else in the
  plugin (its sources, its compiled code). A file configures the interface when it names one of
  Claude Code's settings files (`settings.json` or `settings.local.json`) and, as a whole word
  (no letter, digit or `_` on either side), one of the settings that only configure Claude
  Code's interface: `statusLine`, `statusLineEnabled`, `outputStyle`, `spinnerVerbs`,
  `spinnerTipsEnabled`, `spinnerTipsOverride`, `companyAnnouncements`, `prefersReducedMotion`,
  `syntaxHighlightingDisabled`, `terminalTitleFromRename`.

Each disabled plugin is reported with one startup warning that says why and how to turn it on,
for example claude-hud, whose `/claude-hud:setup` command merges a `statusLine` into Claude
Code's `settings.json`:

```
Claude Code plugin claude-hud@claude-hud disabled: its commands/setup.md configures Claude Code's
`statusLine` setting, which Codex does not have. To use it anyway, add "claude-hud@claude-hud"
to `enable` under `[claude.plugins]` in $CODEX_HOME/ccn-config.toml
```

A plugin listed in `enable` under `[claude.plugins]` of `$CODEX_HOME/ccn-config.toml` is used even when
it is found incompatible; the check is a plain text match and can take a plugin that only
mentions such a setting for one that sets it. A plugin is also left out with a warning when its
install path is gone, or when it has no readable `.claude-plugin/plugin.json`, which Codex's
loader needs; `enable` does not change that.

From a plugin that is used, Codex loads what its plugin loader knows: skills, MCP servers and
hooks, and its commands (`commands/`, or the directories `commands` of the manifest lists) as
skills named `<plugin>:<command>` (see "Custom slash commands"). The rest is not loaded,
without disabling the plugin:

- `agents/`: Codex's plugin loader has no plugin agents. Subagents in `~/.claude/agents` and
  `.claude/agents` are still read (see "Subagents").
- MCP bundles (`.mcpb`), the plugin's `settings.json` and `userConfig`.

### Hooks

Claude Code runs the hooks of an enabled plugin without asking. Codex does the same for a plugin
the user's own `~/.claude/settings.json` enables, as it does for the hooks in that file: no
Codex trust is needed. A plugin that only a project's settings enable waits for Codex's trust
like the project's own hooks: review it in Codex's hooks list (`/hooks`), or run with
`--dangerously-bypass-hook-trust`. `disableAllHooks: true` in any of the settings files turns
these plugins' hooks off too. The hooks see `CLAUDE_PLUGIN_ROOT` (the install path),
`CLAUDE_PLUGIN_DATA`, and `CLAUDE_PROJECT_DIR` and `CLAUDE_CODE_ENTRYPOINT` as described under
"Hooks".

### Differences from Claude Code

- Plugins are read at the start of a session; installing, enabling or disabling one in Claude
  Code shows in the next session. They are not installed in Codex, so Codex's own plugin
  commands (install, uninstall, enable) do not manage them; Claude Code does.
- The plugin's files are read as Codex reads its own plugins: a path the manifest declares for
  `hooks` or `mcpServers` replaces `hooks/hooks.json` or `.mcp.json`, where Claude Code loads
  both. An `mcpServers` list mixing paths and objects is not read.
- Hook events Codex does not have (`Notification`, `PostToolUseFailure`, `StopFailure` among
  them) do not run from plugins, and a plugin hooks file with a handler type Codex does not know
  (`http`) is not loaded, with a warning.
- MCP servers keep the names their files give them, where Claude Code names them
  `plugin:<plugin>:<server>`. Of two plugins' servers of the same name, the first plugin's (by
  id) is used.
- `CLAUDE_PLUGIN_DATA` is Codex's plugin data directory (`$CODEX_HOME/plugins/data/...`), not
  Claude Code's, so a plugin's saved state is not shared between the two.
- `${user_config.*}` values from a plugin's `userConfig` are not filled in.
- A plugin is taken from its recorded install path; one whose marketplace entry defines its
  components without a `plugin.json` (`strict: false`) is not read that way. Managed
  (`policySettings`) `enabledPlugins`, plugins from `--add-dir` and `--plugin-dir`, and Claude
  Code's built-in plugins are not read.

## What is not shared

- A plugin's `commands` entry that names a single file rather than a directory.
- Other skill frontmatter (`allowed-tools`, `model`, `context`, `hooks`, `user-invocable`,
  `argument-hint` and the like) is ignored.
- Managed (enterprise) Claude Code files under `/etc/claude-code`.

## Size

The global instructions are never truncated. When they are larger than `project_doc_max_bytes`
(32 KiB by default), Codex prints a warning at startup; raise that setting to silence it and to
leave room for project instructions.
