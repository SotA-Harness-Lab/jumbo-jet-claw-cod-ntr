# Claude Code compatibility

This fork reads the Claude Code configuration where Claude Code keeps it, in the home directory
and in projects, so skills and instructions are maintained in one place and used by both tools.
Nothing is copied or generated, and there is nothing to set up: when the files below exist, they
are used.

The user's Claude Code configuration directory is `~/.claude`. Setting `CCN_CLAUDE_COMPAT=0`
in the environment turns the whole mode off.

How this document is arranged:

- "What is read" lists every Claude Code file and setting this fork uses and what it becomes.
- "Differences from Claude Code" is what works, but not quite as in Claude Code: area by
  area, a table puts Claude Code's behaviour beside what this fork does in its place.
- "Fully compatible" is what behaves as in Claude Code, with the mechanism that makes it so.
- "Not yet implemented" is what Claude Code has and this fork does not have yet, with the
  nearest thing Codex has.
- "Incompatible" is what cannot be made to behave as in Claude Code, because it contradicts
  how Codex works at its core, with what Codex does in its place.

The last column of each table, "Status", says where a row stands: "Done", with the day and the
commit that brought the behaviour the row describes (the last one, where several did); "In
progress"; "Planned"; or "Not planned". A row of the last two chapters that
[jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr), the fork built on this one, has done says so with its own day and commit.

A feature is in one chapter only. Where a feature works in part, the part that works is under
"Differences" or "Fully compatible" and the part that is missing under one of the last two.

Throughout, "Codex" is this program: upstream Codex with the compatibility layer.

What the tables say of Claude Code is dated: see "What Claude Code was compared" for the
version and the day, and bring that section up to date with every change to a table.

## What Claude Code was compared

Claude Code changes every few days, so a statement about it is only as good as its date. The
"Claude Code" column of every table was last checked on **2026-11-21**, against:

| Source | State |
| --- | --- |
| Claude Code's documentation at `https://code.claude.com/docs/en/` (the pages `hooks`, `skills`, `sub-agents`, `mcp`, `memory`, `permissions`, `permission-modes`, `settings`, `tools-reference`, `plugins-reference`, and the index `https://code.claude.com/docs/llms.txt`) | As served on 2026-11-20, when the newest release in its changelog (`https://code.claude.com/docs/en/changelog`) was **2.1.329** of 2026-11-19; the changelog itself read on 2026-11-21, up to **2.1.330** of 2026-11-20 |
| A reference copy of Claude Code's source | Its state of 2026-11-16. It names no release of Claude Code, but its oldest commit is of 2026-03-31, between 2.1.87 (2026-03-29) and 2.1.89 (2026-04-01), and its maintainers have brought in only a few later changes: the features the changelog adds from 2.1.89 on were looked for in it one by one |

Where the two differ, the documentation is followed and the cell says "newer versions" for
what only it has, with the release that brought the change where the documentation names
one. A cell that says nothing of versions was found the same in both, or in the source alone.

Keeping it current:

- Read the changelog from the release named above to the newest one, and the pages above
  for what changed; correct the rows it touches, add a row for each new feature, and move a
  row to another chapter when this fork or Claude Code changed sides.
- Then change the date and the release in this section, and only then: the date says that
  every table was read against that release.
- When this fork changes how it treats a Claude Code feature, change the row in the same
  commit, its Status included; the commit's own id goes in with the next commit, since a
  commit cannot name itself. A row that moves to "Fully compatible" or "Differences" moves
  with its new status.

## The fork's own settings (ccn-config.toml)

Settings that only this fork has are kept in their own file, `$CODEX_HOME/ccn-config.toml`
(`~/.codex/ccn-config.toml` by default), never in Codex's `config.toml`, which keeps only upstream
Codex's settings. The file is read once each time the configuration is loaded, at the start of a
session. Every key is optional; a missing file means the defaults. A file that does not parse,
or gives a key a value of the wrong type, is reported as one startup warning and the defaults
are used in its place; Codex still starts. A key or section not listed here is left alone and
only noted in the log, so that another build of the program, or a later version, can keep
settings of its own in the same file.

```toml
[claude]
# Offer the model Claude Code's tool set in place of Codex's (see "Claude tool set").
tools = false

[claude.plugins]
# Claude Code plugins to use even when they are found incompatible with Codex
# (see "Which plugins are used"), by their <plugin>@<marketplace> id.
enable = []

[hooks.models]
# Which Codex model judges a `prompt` or `agent` hook in the place of the Claude model its
# handler names (see "Hooks"). `default` is for a handler that names none.
# default = "gpt-5.5-mini"
# haiku = "gpt-5.5-mini"
# "claude-sonnet-4-6" = "gpt-5.5"

[ui]
# The TUI's language, a BCP 47 tag ("en", "zh-CN"); the system's language when left out.
# language = "zh-CN"
```

| Key | Type | Default | Effect |
| --- | --- | --- | --- |
| `claude.tools` | boolean | `false` | The Claude tool set in place of Codex's tools |
| `claude.plugins.enable` | list of strings | `[]` | Claude Code plugins used even when found incompatible |
| `hooks.models` | table of strings | empty | Claude model name, or part of one, to the Codex model that judges `prompt` and `agent` hooks naming it; `default` for hooks naming none |
| `ui.language` | string | the system's | The TUI's language: Simplified Chinese (`zh-CN`) or English; a language without a translation is English |

The TUI reads `ui.language` once, when it starts; the environment variable `CCN_LANGUAGE`, when set, comes before it
(the tests that drive the program are run with `CCN_LANGUAGE=en-US`, as `just test` does, since they look for English text). `CCN_CLAUDE_COMPAT=0` overrides the `claude` keys: with it, neither the Claude tool set nor Claude Code's
plugins are used.

## What is read

| Claude Code | In Codex | When a change shows |
| --- | --- | --- |
| `~/.claude/skills/` | A user skill root, beside `~/.agents/skills` | Like any other skill root |
| `.claude/skills/` of a project | A repository skill root, beside `.agents/skills`, in each directory from the project root down to the working directory | Like any other skill root |
| `~/.claude/commands/`, `.claude/commands/` of a project | Skills named after the file's path | Like any other skill root |
| `~/.claude/CLAUDE.md` | Appended to the global instructions, after `$CODEX_HOME/AGENTS.md` | At the next request |
| `~/.claude/rules/**/*.md` without `paths:` | Appended after `CLAUDE.md`, sorted by path | At the next request |
| `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/**/*.md` without `paths:`, `CLAUDE.local.md` | Project instructions, after the project's `AGENTS.md` files | When project instructions are loaded |
| `~/.claude/projects/<project>/memory/` | Claude Code's instructions for writing memories and the `MEMORY.md` index, as project instructions, last; the directory is writable in the sandbox | When project instructions are loaded |
| Rules with `paths:`, and the instruction files of directories below the working directory | Shown to the model once a file they cover is worked on | At once |
| `hooks` in `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json` (in the working directory and at the repository's root) | Hooks, beside Codex's own | At the next prompt or tool call |
| `env` in the same files | Added to the environment of the commands Codex runs | At the next session |
| `effortLevel` in the same files | The default reasoning effort, when Codex's own configuration sets none | At the next session |
| Bash rules in `permissions` of `~/.claude/settings.json` | Command rules (execpolicy), while the Claude tool set is off | At the next session |
| `permissions.allow`, `ask`, `deny` of the three settings files | Checked by the Claude tool set before each call, while it is on | At the next tool call |
| `~/.claude/agents/**/*.md`, `.claude/agents/**/*.md` | Agent roles | Description at the next session, instructions at each spawn |
| `mcpServers` of `~/.claude.json`, approved servers of `.mcp.json` | MCP servers, beside Codex's own | When MCP servers are started |
| Plugins installed and enabled in Claude Code | Codex plugins, loaded from their install paths: skills, commands, agents, MCP servers, hooks | At the next session |

## Differences from Claude Code

### Instructions and rules

`CLAUDE.md`, `CLAUDE.local.md` and the rule files. How they are found and laid out is the
same as in Claude Code (see "Fully compatible"); these rows are what is not.

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| Place in the model's context | A user message after the system prompt, headed `Codebase and user instructions are shown below` | The user's files after `$CODEX_HOME/AGENTS.md`, a project's after the project's `AGENTS.md` files, in Codex's instruction messages | Done 2026-11-17 `80c1279420` |
| Words around them | None of Codex's | A short preamble tells the model that the global instructions come from Claude Code, that `/<name>` means the skill of that name, and that the hooks they mention may not run in Codex | Done 2026-11-17 `13979cd030` |
| Size limit | None from Codex | They are not cut to `project_doc_max_bytes`, which still applies to `AGENTS.md`, and the global instructions are never truncated. When those are larger than `project_doc_max_bytes` (32 KiB by default), Codex prints a warning at startup; raise that setting to silence it and to leave room for project instructions | Done 2026-11-17 `13979cd030` |
| When a rule with `paths:`, or the instruction files of a directory below the working directory, are shown | When `Read` touches a file they cover; in newer versions also `Edit`, `Write` and a `cat`, `head`, `tail`, `sed -n` or `grep` of a single file (2.1.293) | After each tool call, for the files a shell command reads (as Codex's command parser recognises them: `cat`, `sed -n`, `head` and the like), the one file a `grep` or `rg` searches when it names a single file (not after a `cd` in the same command), the files a patch adds, changes or moves to, and, with the Claude tool set, the file `Read`, `Write` or `Edit` names | Done 2026-11-21 |
| How such a file is shown | With the read that triggered it | Once per session, as a developer message headed like the other instruction files | Done 2026-11-17 `80c1279420` |
| Where it works | Everywhere | Only when the session's environment runs on this machine | Done 2026-11-17 `80c1279420` |
| A project file's `@<path>` include outside the working directory | Loaded once the user has approved external includes for the project, asked once | Never loaded | Done 2026-11-17 `80c1279420` |
| Order of the files in `~/.claude/rules` | The order the directory lists them in | Sorted by path | Done 2026-11-17 `13979cd030` |
| `AGENTS.md` | Newer versions (2.1.277 on) read it where no `CLAUDE.md` is found | Always read, before the `CLAUDE.md` files: it is Codex's own instruction file | Done (upstream Codex) |
| A Git worktree inside its own repository (`claude -w`) | The repository's checked-in `CLAUDE.md` and rules above the worktree are skipped, the worktree having its own; `CLAUDE.local.md` is still read | Both are read | Done 2026-11-17 `80c1279420` |
| What else counts as working on a file | A file named with `@` in the prompt, and the file open in the IDE | Tool calls only | Done 2026-11-17 `80c1279420` |

### Skills, slash commands and invoking them

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| Which folders are skills | `<root>/<name>/SKILL.md`, one level below the root; directory symlinks followed; a skill reached twice is listed once | Codex's rules: nested folders up to six levels deep, directory symlinks followed, hidden folders (such as `.trash`) skipped; a skill reachable from two roots through a symlink is listed once | Done 2026-11-17 `13979cd030` |
| Frontmatter fields that are used | `name`, `description`, `when_to_use`, `arguments`, `argument-hint`, `allowed-tools`, `model`, `effort`, `context`, `agent`, `hooks`, `paths`, `disable-model-invocation`, `user-invocable`, `version`, `shell` | `disable-model-invocation`, `description`, `when_to_use` and `arguments` (see "Fully compatible"); for the others see "Not yet implemented" | Done 2026-11-17 `2471894159` |
| `disable-model-invocation: true` | The skill is not offered to the model | The same, unless an explicit `allow_implicit_invocation` in the skill's `agents/openai.yaml` says otherwise; `$<name>` still runs it | Done 2026-11-17 `2471894159` |
| Shell operators and globs in the arguments | Dropped | Kept as words | Done 2026-11-17 `0d118bdf09` |
| Skills that are not files on this machine (Codex's cloud and remote-environment skills) | Its own kinds, in some builds: skills of MCP servers and of the claude.ai account | Not expanded as Claude Code expands a skill | Done 2026-11-17 `0d118bdf09` |
| A built-in command and a skill of the same name | The skill replaces the built-in command (not its aliases) | The built-in command wins | Done 2026-11-17 `0d118bdf09` |
| `${CLAUDE_SESSION_ID}` | Claude Code's session id | The Codex thread's id | Done 2026-11-17 `0d118bdf09` |
| Commands of a plugin in Codex's own plugin format | Not applicable | Such plugins keep the skills Codex made of their commands when it installed them; the commands of Claude Code's plugins are never copied | Done 2026-11-18 `c58a3409c6` |
| How much of the skill list the model is shown | 1% of the context window (`SLASH_COMMAND_TOOL_CHAR_BUDGET`), an entry's description and `when_to_use` cut to 1,536 characters | Codex's own budget for its skill list | Done 2026-11-17 `aac85b1fcf` |
| Skills that come with the program | Claude Code's bundled skills; newer versions add `/code-review` (2.1.147, `--fix` from 2.1.152) and `/less-permission-prompts` (2.1.111) | Codex's own system skills; Claude Code's are not there | Done (upstream Codex) |
| A skill's name | In the reference source the folder's name, `name` in the frontmatter being only the name shown; the documentation describes `name` as what is typed; newer versions ignore a `name` longer than 256 characters (2.1.292) or holding `<`, `>`, hidden, direction-changing or blank-like characters, and do not offer a skill whose folder's name has them (2.1.299) | `name` of the frontmatter, the folder's name when it has none: a skill whose `name` differs from its folder is `/<name>` here. Neither the length nor those characters are checked | Done 2026-11-17 `2471894159` |
| A `SKILL.md` without frontmatter | Loaded, named after its folder, with the first line of the body as description | Not loaded, and reported as invalid; a command file without frontmatter is loaded | Done 2026-11-17 `2471894159` |
| No `description` and no text in the body | Listed as `Skill` or `Custom command` | Not loaded | Done 2026-11-17 `2471894159` |
| `SKILL.md` in a folder of a commands directory (`commands/<dir>/SKILL.md`) | One command named after `<dir>`; the other files of that folder are left out | Each file is a command named after its path (`<dir>:SKILL`) | Done 2026-11-18 `c58a3409c6` |
| A skill the model invokes itself | The model calls the `Skill` tool with the skill's name and its arguments, and gets the skill expanded as for `/<name> <args>` | The same with the Claude tool set on: `Skill` takes `skill` and `args`, refuses a skill with `disable-model-invocation` and one that is not a file on this machine, and returns `Launching skill: <name>` with the expanded text in its result, where Claude Code adds the text as a message of its own. With the tool set off there is no `Skill` tool: the model reads `SKILL.md` with a shell command, and nothing is expanded | Done 2026-11-20 `f987126bd2` |
| `Skill(<name>)` permission rules | The skill's name, or `<name>:*` for a prefix; newer versions also match a skill's alias and display name, and `Skill(anthropic-skills:*)` and `Skill(claude-ai:*)` cover only the skills synced from claude.ai (2.1.282, 2.1.283) | The skill's name exactly, with the Claude tool set on | Done 2026-11-20 `f987126bd2` |
| Quitting from the prompt | `exit`, `quit`, `:q`, `:q!`, `:wq`, `:wq!`, `/exit` or `/quit` on its own (spaces around it ignored) quits, also while a turn is running, instead of being sent or queued. When work is running, newer versions ask first in a `Background work is running` dialog that names it, offering to stop it and exit or to move the session to the background | The same words in the TUI, also with `CCN_CLAUDE_COMPAT=0`; Codex alone had `/exit` and `/quit`, and queued them behind a running turn. When nothing runs, it quits at once. With the embedded app server (the default) it otherwise asks in a dialog that names what quitting stops (the turn in progress, other sessions and subagents, side conversations, background terminals), offering Stay or Exit and stop tasks; with a local daemon a running turn gets Codex's own menu (cancel it, run it in the background, or stop it and exit) | Done 2026-11-20 `3ff4bc95e1` |
| Yes and no in the frontmatter | Newer versions (2.1.218) read `yes`, `no`, `on`, `off`, `1` and `0`, in any case, as booleans in skills and plugins, beside `true` and `false` | Only `true`, as a boolean or a string, is true: `disable-model-invocation: yes` leaves the skill offered to the model | Done 2026-11-17 `2471894159` |
| Several skills at the start of a prompt (`/skill-a /skill-b <text>`) | Newer versions (2.1.199) load every leading skill, up to 5 | The first skill expands, and the rest of the line, the other names included, is its arguments | Done 2026-11-17 `0d118bdf09` |
| Skills changed while a session runs | Reloaded when a skill's files change; newer versions react only to `.md` files (2.1.144) and do not look into `.trash` (2.1.314) | Codex's app server reloads them when a `.md` file, a folder or a skill root changes; other files, and whatever is in a hidden folder, `__pycache__` or `node_modules` below a root (`.trash`, `.pixi`), do not count. Those folders are still watched, so their directories still take file-watch handles | Done 2026-11-21 |

To hide a Claude Code skill from Codex altogether (one that only makes sense in Claude Code,
or one that duplicates a Codex skill), disable it in `config.toml`:

```toml
[[skills.config]]
name = "<SKILL_NAME>"
enabled = false
```

`/<name>` works with `CCN_CLAUDE_COMPAT=0` too, since such a name was an error before; only the
expansion of the skill's text is turned off, and the user's and the projects' commands are
left out with the skills.

### Hooks

Codex's hook engine takes the same shape as Claude Code's `hooks` key, so the hooks of the
settings files run as they are. What differs:

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| `CLAUDE_CODE_ENTRYPOINT` | Set by what launched it (`cli`, `claude-vscode`, `sdk-ts` and others), `sdk-cli` when nothing did | `cli` in the TUI, `sdk-cli` under `codex exec` (as for `claude -p`), `claude-vscode` for the IDE; other sessions, subagents among them, get none | Done 2026-11-17 `df06fbae01` |
| `session_id` in the input | Claude Code's session id | The Codex thread id | Done 2026-11-17 `80c1279420` |
| Tool names, with the Claude tool set off | Claude Code's tools | Codex's tools under Claude Code's names where they have one: shell commands are `Bash`; `apply_patch` is also matched by `Write` and `Edit`, `spawn_agent` by `Agent` and `Task`, `request_user_input` by `AskUserQuestion` (its input has the same `questions[].question`), `update_plan` by `TodoWrite`. The hook's input keeps Codex's name and Codex's input (`{"command": ...}` for shell commands). `Read`, `Grep`, `Glob`, `WebFetch` and `WebSearch` have no Codex tool and never match | Done 2026-11-17 `df06fbae01` |
| Tool names, with the Claude tool set on | Claude Code's tools | The same names with Claude Code's input and `tool_response` (see "Fully compatible"); a hook in Codex's own `hooks.json` matching `exec_command` or `apply_patch` then does not run for these tools | Done 2026-11-17 `c14ae9210a` |
| `Notification` | Sent for permission prompts (`permission_prompt`), idle prompts (`idle_prompt`, about 60 seconds after a response when the user has not typed since and no background agent is still running), `auth_success`, MCP elicitations (`elicitation_dialog`, `elicitation_complete`, `elicitation_response`; newer versions also `elicitation_url_dialog`), newer versions' `agent_needs_input` (a background session starts waiting on the user) and other notices; awaited before the desktop notification | Sent when Codex asks the user: an approval of a command (`permission_prompt`, message `Codex needs your permission to use Bash`), of a patch (`... to use apply_patch`) or of more permissions (`... to use request_permissions`), and an MCP elicitation (`elicitation_dialog`, or `elicitation_url_dialog` for a URL). A subagent's prompt is `agent_needs_input` instead, with the agent's nickname (or role) as `title`. `idle_prompt` (message `Codex is waiting for your input`) is sent in the TUI and the IDE 60 seconds after a turn that was not interrupted, unless another turn has started or a subagent is still running; typing without sending does not put it off, since Codex sees only what is sent. Not awaited | Done 2026-11-21 |
| `PostToolUseFailure` | Sent when a tool call fails | The same, with `error` set to what went wrong; `is_interrupt` is always `false`, since a cancelled call runs no hook. Codex's own `PostToolUse` hooks still run for a failed shell command | Done 2026-11-17 `df06fbae01` |
| `StopFailure` | Sent when a turn ends on an API error, with `error` one of `rate_limit`, `billing_error`, `authentication_failed`, `invalid_request`, `server_error`, `max_output_tokens`, `unknown` (newer versions have more kinds) and `last_assistant_message` | Sent when a turn ends on an error instead of an answer, with `error` one of `rate_limit`, `billing_error`, `authentication_failed`, `invalid_request`, `server_error`, `unknown`, `error_details` the message Codex shows, and `last_assistant_message`; a model that is not found is `invalid_request` | Done 2026-11-17 `df06fbae01` |
| `if`, when the tool's input has nothing to match the rule's content against (`Edit(*.rs)` for an `apply_patch` call with the Claude tool set off) | The hook does not run: content is matched only for `Bash`, `Read`, `Edit`, `Write`, `Glob` and `Grep`, and for input of their shape | The hook runs and decides for itself, so that a guard does not silently stop firing | Done 2026-11-19 `358f5dc8ca` |
| `type: "prompt"` and `type: "agent"`: which model | The handler's `model`, or a small, fast Claude model; newer versions (2.1.330) take the aliases `haiku` and `sonnet` in a `prompt` hook's `model`, and fall back to the session's model when that one is not available (2.1.300) | The handler's `model` names a Claude model, which Codex cannot run. `[hooks.models]` of `ccn-config.toml` says which Codex model judges in its place: the entry for that very name, or else the longest entry whose name the model's name contains (`haiku` covers `claude-haiku-4-5`), names compared without regard to case. A handler without `model` takes the entry `default`. Where no entry fits, `model` is ignored and the session's own model judges, and so it does, once, when the entry's model cannot be used (its request fails, or it gives no answer). An alias is looked up the same way, so the entries `haiku` and `sonnet` serve it | Done 2026-11-21 |
| `type: "prompt"`: the request | One request, thinking off, the answer held to a JSON schema; no tool runs | One request without tools, the answer held to a JSON schema, at the lowest reasoning effort the model has, which is not always none | Done 2026-11-20 `d7bc2a1b13` |
| `type: "agent"`: the verifier | A loop of its own with the session's tools (none for subagents, plan mode or questions), thinking off, in a permission mode that never asks: a call no rule allows is denied. Its prompt names the transcript file | A child session with read-only permissions that asks for no approvals. Every step of it carries Codex's full tool definitions and starting it takes a moment, so it suits `Stop`, which runs once a turn, better than an event of every tool call. No `prompt` or `agent` hook runs inside it | Done 2026-11-19 `57b49d1405` |
| `type: "agent"`: its limit | 50 turns | 50 tool calls started without an answer, after which it is ended without blocking; a turn that makes several calls at once counts for more here | Done 2026-11-20 `d7bc2a1b13` |
| `type: "http"`: the network path | Through Claude Code's sandbox network proxy when sandboxing is on | Straight from Codex, not through Codex's sandbox network proxy | Done 2026-11-19 `57b49d1405` |
| A `Stop` hook that blocks again and again | Newer versions override the next block once hooks have continued the turn 8 times in a row, counting anew after each tool call (`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`); the reference source has no limit | It may keep one turn from ending 8 times in a row; the next block is not honoured and a warning says so. The number is fixed | Done 2026-11-19 `d3e0b4a47f` |
| Exit code 2 | Means what the event makes of it: it blocks for `PreToolUse`, `UserPromptSubmit`, `Stop` and `SubagentStop`, whatever stderr holds, with `No stderr output` as the reason when it holds nothing; for `PostToolUse` and `PostToolUseFailure` stderr goes to the model, the tool having run; for `SessionStart`, `Notification`, `StopFailure`, `SessionEnd` and the like stderr is only shown | The same for these events, except that a `Stop` or `SubagentStop` hook blocks only when it wrote a reason to stderr | Done 2026-11-21 |
| A block or a denial given with `continue: false` or `stopReason` (`PreToolUse`), or with `interrupt` (`PermissionRequest`) | The call is refused; newer versions (2.1.296) also end the turn | The call is refused and the turn goes on; a warning says what was not honoured | Done 2026-11-21 |
| `type: "prompt"`: a condition that is not met | Ends the turn, with the reason shown. An `agent` hook blocks as exit code 2 does | Both block as a command's exit code 2 does for the event, so the model is told the reason and goes on | Done 2026-11-19 `57b49d1405` |
| `type: "agent"` for `PermissionRequest` | Not run; `prompt` handlers are | Run | Done 2026-11-19 `57b49d1405` |
| `disableAllHooks` | The value after the settings files are merged, a later file's `false` undoing an earlier `true`; hooks of managed settings stay on unless those turn them off | `true` in any of the three files turns every hook off | Done 2026-11-17 `80c1279420` |
| A `matcher` that is a regular expression | JavaScript's syntax | Rust's `regex` syntax: no look-around and no back-references | Done 2026-11-17 `80c1279420` |
| `SessionEnd` | `reason` is `clear`, `resume`, `logout`, `prompt_input_exit` or `other`; the hooks get 1.5 seconds (`CLAUDE_CODE_SESSIONEND_HOOKS_TIMEOUT_MS`) | `reason` is always `other`; the hooks get 1 second, or their `timeout` up to 3 | Done 2026-11-17 `80c1279420` |
| `model` and `turn_id` in the input | `model` for `SessionStart`; no `turn_id` | `model` for most events, and `turn_id`, the id of Codex's turn, for the events of a turn | Done 2026-11-17 `80c1279420` |
| `type: "http"`: `NO_PROXY` | A host it exempts from the proxy is checked for private addresses again | Not read: with a proxy in the environment no host is checked | Done 2026-11-19 `57b49d1405` |
| A command handler under Claude Code's sandbox | Run with the network closed when sandboxing is on, the filesystem open | Run outside Codex's sandbox | Done 2026-11-17 `80c1279420` |
| What a `PreCompact` hook prints | Added to the instructions for the summary, each hook's text cut at 10,000 characters with a marker | The same where Codex writes the summary itself; a compaction the model provider does (remote compaction) has no instructions to add it to | Done 2026-11-21 |
| `CLAUDE_ENV_FILE` | For `SessionStart`, `Setup`, `CwdChanged` and `FileChanged`: a file whose text runs before every later command | For `SessionStart`, Codex having no other of these events; the file's assignments (`NAME=value`, `export NAME=value`, and `declare -x NAME=value` as `export -p` prints them) are added to the environment of the session's later commands (`exec_command`, and `Bash` over it). Values are read as the shell reads them, quotes and `~` included, with `$NAME` and `${NAME}` taken from the command's environment, so `export PATH="$PATH:./node_modules/.bin"` adds to its `PATH`; a line with any other shell code (other commands, command substitution, `${NAME:-...}`) is left out | Done 2026-11-21 |
| `permission_suggestions` and `custom_instructions` in the input | Claude Code's suggested rule updates for `PermissionRequest`; the instructions given with `/compact` for `PreCompact` | Present: always an empty list, and always `null`, since Codex has neither | Done 2026-11-20 `f987126bd2` |
| `type: "http"`: `Content-Length` and `Transfer-Encoding` in `headers` | Newer versions (2.1.326) ignore them: the length comes from the body | Set on the request as written | Done 2026-11-19 `57b49d1405` |

The `prompt` and `agent` paths have not been run against a live model here: what is tested is
how the handlers are read, how a verdict is turned into the hook's result, and the mapping of
models. A hook of these types can run for every tool call, so mapping at least `default` and
`haiku` to a small Codex model in `[hooks.models]` is worth it.

### Claude tool set

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

How each tool is made:

| Claude tool | Replaces | Delegated to Codex | Added by the layer | Status |
| --- | --- | --- | --- | --- |
| `Bash` | `exec_command` | Running the command (one-shot with the timeout; `run_in_background` as a process left running), shell, approval, sandbox, command rules, output truncation, events | Parameters (`command`, `timeout` in ms, default 2 minutes, at most 10; `description`; `run_in_background`; `dangerouslyDisableSandbox`, sent as `sandbox_permissions: "require_escalated"` with the description as justification), the result text: output trimmed, `Exit code N` and the output for a failed command, `(Bash completed with no output)`; the working directory a `cd` leaves, carried over to the next command | Done 2026-11-17 `c14ae9210a`; working directory done 2026-11-20 `897f7f4914` |
| `Read` | `view_image` | Reading through the executor filesystem under the turn's sandbox; images (`png`, `jpg`, `jpeg`, `gif`, `webp`) to `view_image` | Parameters (`file_path`, `offset`, `limit`, `pages`), `N<TAB>line` numbering from 1, the 256 KB limit without `limit`, the 25,000-token limit, refusal of binary files and directories, the empty-file and short-file reminders, the "file does not exist" message; notebooks cell by cell; PDFs page by page, with poppler's `pdftoppm` and `pdftotext` | Done 2026-11-17 `c14ae9210a`; notebooks and PDFs done 2026-11-20 `897f7f4914` |
| `Write` | `apply_patch` | Writing, as a patch: approval, sandbox, patch events, the turn's diff | Parameters (`file_path`, `content`), the rule that an existing file is read first and not changed since, messages | Done 2026-11-17 `c14ae9210a` |
| `Edit` | `apply_patch` | Writing, as a patch, as for `Write` | Parameters (`file_path`, `old_string`, `new_string`, `replace_all`), exact matching (refused when not found, or found more than once without `replace_all`), the read-first rule, messages | Done 2026-11-17 `c14ae9210a` |
| `Glob` | none (offered with `Bash`) | Listing and sorting, as `rg --files --glob <pattern> --sort=modified --no-ignore --hidden` run through `exec_command` (sandbox, approval, command rules, events); the directory check through the executor filesystem | Parameters (`pattern`, `path`), an absolute pattern split into its directory and the rest, the first 100 files and the truncation note, paths inside the working directory shortened, `No files found`, the "directory does not exist" and "not a directory" messages | Done 2026-11-18 `092e61dcc8` |
| `Grep` | none (offered with `Bash`) | Searching, as one `rg` command run through `exec_command` as for `Glob`, files listed newest first by `rg --sortr=modified`; the path check through the executor filesystem | Parameters (`pattern`, `path`, `glob`, `type`, `output_mode`, `-i`, `-n`, `-A`, `-B`, `-C`, `context`, `multiline`, `head_limit`, `offset`), Claude Code's ripgrep arguments (hidden files, VCS directories left out, `--max-columns 500`), `head_limit` (250 by default, 0 for all) and `offset` with the pagination note, paths shortened, the text of each output mode (`Found N files`, `No matches found`, `Found N total occurrences across M files.`), the "path does not exist" message | Done 2026-11-18 `092e61dcc8` |
| `WebSearch` | `web.run` (standalone search) | The search, its backend and settings (`web_search` mode, location, filters), its events and the TUI | Parameters (`query`, at least 2 characters; `allowed_domains`, sent as the search's domain filter; `blocked_domains`, sent as `-site:` operators in the query; not both), the result text with Claude Code's sources reminder | Done 2026-11-18 `5e56c5502a` |
| `WebFetch` | nothing (new) | The request, as a `curl` command through `exec_command`: sandbox, managed network proxy and its host approvals, command rules, approval prompts, events; the answer to `prompt` from the session's model client | Parameters (`url`, `prompt`), URL checks (at most 2000 characters, no credentials, a dotted host), HTTP upgraded to HTTPS, same-site redirects (`www.` added or removed) followed up to 10, other redirects reported in Claude Code's format, the 10 MB and 60-second limits, HTML to Markdown, the 15-minute cache, the secondary model's prompt with Claude Code's quoting rules and preapproved documentation hosts, the result text | Done 2026-11-18 `5e56c5502a` |
| `TodoWrite` | `update_plan` | The list, sent as the plan: the TUI's plan display, plan events, the refusal in Plan mode | Parameters (`todos` of `content`, `status`, `activeForm`), the checks (both forms non-empty), description, result text; the step in progress shows its `activeForm` | Done 2026-11-18 `bf914e9b33`, `a965a5c156` |
| `AskUserQuestion` | `request_user_input` | Asking: the TUI's question panel, the modes it is offered in, the root-thread rule, cancellation, recorded answers | Parameters (1-4 `questions` of `question`, `header`, 2-4 `options` of `label` and `description`, `multiSelect`), the uniqueness checks, description, result text (`"question"="answer"`, with `user notes:`) | Done 2026-11-18 `bf914e9b33` |
| `Agent` | `spawn_agent` (multi-agent v1 or v2, whichever the turn has) | Starting the agent (roles, models, depth and thread limits, sandbox inheritance, TUI events), waiting through Codex's agent status subscription (as `wait_agent` does), the completion notice of a background agent | Parameters (`description`, `prompt`, `subagent_type`, `model`, `run_in_background`), the description listing the agent types with their tools, `general-purpose` as Codex's default role, Claude Code's message for an unknown type, result texts (the agent's final message) | Done 2026-11-18 `bf914e9b33` |
| `Skill` | nothing (new) | The skills Codex lists for the turn, read from this machine | Parameters (`skill`, `args`), the description, the refusals (`Unknown skill: <name>`, `disable-model-invocation`), the expansion as for `/<name> <args>`, the result text | Done 2026-11-20 `f987126bd2` |

`write_stdin` stays, to read the output of a background command and to stop it. Codex's
other agent tools (`wait_agent`, `send_input`, `close_agent` and the rest, or v2's
`send_message`, `followup_task`, `list_agents`) stay beside `Agent`. Every other Codex tool
(the hosted web search, MCP tools and the rest) is offered as before.

What does not behave as Claude Code's tool does:

| Tool | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| `Bash`: description | Claude Code's, with its sandbox settings and attribution lines for commits and pull requests | Claude Code's instructions for commits and pull requests without attribution lines, and a short note on Codex's sandbox in place of Claude Code's sandbox settings | Done 2026-11-17 `c14ae9210a` |
| `Read`: images | Shown | Need a model that takes images and Codex's `view_image` feature | Done 2026-11-17 `c14ae9210a` |
| `Bash`: working directory | `cd` carries over to the next command while it stays inside the project, the working directory and the `additionalDirectories` (outside, the next command starts in the project again, and the result ends with `Shell cwd was reset to <dir>`); a command that fails, a background command and a subagent's commands leave it as it was. Relative paths of the file tools and the default directory of `Glob` and `Grep` follow it | The same, with the session's working directory as the project (`additionalDirectories` is not read). The command writes its `pwd -P` to a file of its own after it succeeds; this is added after approval, so approval prompts, command rules and the TUI show the command as given, and the sandbox may write that one file. Not followed for a command on a remote executor. A command that ends where it started leaves the carried directory as it is, so that of several calls sent at once, one that did not move does not undo another's `cd` (Claude Code fixed the same in 2.1.321). The tool description says when the directory carries over, and the result of a background command that runs `cd`, `pushd`, `popd` or `chdir` ends with Claude Code's `Session cwd remains <dir>` note (2.1.311, 2.1.210). The `cwd` of hook input stays the session's working directory | Done 2026-11-20 `897f7f4914`; parallel calls, description and background note done 2026-11-21 |
| `Read`: PDFs | A PDF of at most 10 pages is sent whole as a document; with `pages` (at most 20), or for a model without PDF support, the pages are rendered as 100 dpi images with poppler's `pdftoppm`; a longer PDF without `pages` is refused | Codex's models take no PDF document, so the pages are always rendered as on Claude Code's second path, each with its text from `pdftotext` beside it; a model that takes no images gets the text alone. Needs poppler-utils (without `pdfinfo` the length is unknown, and a read without `pages` takes the first 20 pages). The hook's `tool_response` is `type: "parts"` with `filePath`, `originalSize` and `count`, without `outputDir` | Done 2026-11-20 `897f7f4914` |
| `MultiEdit` | Gone from current versions | Not offered | Done 2026-11-17 `c14ae9210a` |
| `Glob`, `Grep`: the search program | A ripgrep Claude Code ships | `rg` in the session's environment. Codex's own release packages ship one and put it on the path of every command Codex runs; a Codex built from source has none, so install ripgrep there. Without it they fail with a message saying so | Done 2026-11-18 `092e61dcc8` |
| `Grep`: order of files | Newest first, names in order among files of the same time | Newest first through ripgrep's own sort, which also sorts files that do not match and searches on one thread; files with the same time keep ripgrep's order | Done 2026-11-18 `092e61dcc8` |
| `WebSearch`: results | Anthropic's own search run by the model provider, up to 8 searches a call: links and the model's commentary | Codex's search output wrapped in Claude Code's text. `blocked_domains` is only as good as the engine's `-site:` support. Only plain web search is offered: opening pages goes through `WebFetch` | Done 2026-11-18 `5e56c5502a` |
| `WebFetch`: how the request is made | By Claude Code itself | By a `curl` command (`curl.exe` under PowerShell or `cmd`) on the machine that runs commands, so it needs `Bash` and `curl` there. It goes out under the session's rules: with network in the sandbox, or a managed network proxy (which allows, denies or asks per host), it runs in the sandbox; otherwise it asks to run outside it, as `sandbox_permissions: "require_escalated"`, which Codex approves or refuses by its approval policy (refused under `never`). The approval shows the `curl` command, and command rules for `curl` apply to it. curl writes the body to a file in the temporary directory of that machine (`/tmp` where it names none), which is read back and removed; where the sandbox does not let it write there, the body comes through the command's output, which keeps only its first and last 512 KiB | Done 2026-11-18 `5e56c5502a`; bodies over 1 MiB done 2026-11-21 |
| `WebFetch`: the answer to `prompt` | A separate small model | The session's model at low reasoning effort; its tokens are not counted in the session's usage | Done 2026-11-18 `5e56c5502a` |
| `WebFetch`: cache | 15 minutes, shared by the whole process | 15 minutes, kept per session; a page is dropped when it expires, also when no fetch follows | Done 2026-11-18 `5e56c5502a`; expiry done 2026-11-21 |
| `WebFetch`: deeply nested pages | Converted to Markdown; newer versions (2.1.323) do it faster and with less memory | Converted off the async threads; a page whose elements nest more than 1,000 deep is given as its plain text, a line for each block, since the converter recurses once per level | Done 2026-11-21 |
| `WebFetch`: tables | Each cell kept in its column, empty ones too (newer versions, 2.1.323) | The same for empty cells and for row headers (`<th>` in a body row); `colspan` is not expanded, so the cells after a spanning one move left | Done 2026-11-21 |
| `TodoWrite`: where it is offered | In sessions without a person (`claude -p`, the SDK); an interactive session has `TaskCreate`, `TaskGet`, `TaskList` and `TaskUpdate` in its place; newer versions (2.1.233) offer these tools only to older models (Claude 3.x, Opus 4.0 to 4.7, Sonnet 4.0 to 4.6, Haiku 4.5) unless `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` | Always, also where Codex's own plan tool is off (`[tools.update_plan]` is off by default); a tool policy that removes `update_plan` removes it too; the model is not looked at and `CLAUDE_CODE_ENABLE_TODO_TOOLS` is not read | Done 2026-11-18 `a965a5c156` |
| `Agent`: an agent that has answered | Stays | Under multi-agent v1 it is closed, to free its thread: a foreground agent once its answer is in, or when the call is interrupted, a background one once it ends, its result still added to the conversation. Codex also adds its own completion notice after a foreground agent's answer | Done 2026-11-18 `bf914e9b33`; background and interrupted agents done 2026-11-21 |
| `Bash`: which exit codes are failures | A code other than 0, except code 1 of `grep`, `rg`, `find`, `diff`, `test` and `[`, which is a result | Any code other than 0 | Done 2026-11-17 `c14ae9210a` |
| `Write`, `Edit`: reading the file first | An existing file has to have been read; newer versions count a `cat`, `head` or the like of the one file as a read, and let newer models change an unread file that could be read without asking | Always required, and only a `Read` call counts | Done 2026-11-17 `c14ae9210a` |
| `Read`: the 25,000-token limit | Counted in the model's tokens | Estimated at four bytes a token, so a file dense in tokens passes here and not there | Done 2026-11-17 `c14ae9210a` |
| `Read`: text that is not UTF-8 | UTF-16 with a byte-order mark is read | Read as UTF-8, with a replacement character for what is not | Done 2026-11-17 `c14ae9210a` |
| `Write`, `Edit`: files read before a resume | Known again from the transcript (`--resume`, `--continue`): a file read whole or written by its content then, a file edited by its content on disk at the resume, and from 2.1.212 on a file read with `offset` or `limit` too | Known again from the resumed (or forked) conversation: a file read whole by the lines the read returned, a file written by what was written, a file edited by the edit made again on its content before; a file read only in part, or edited when its whole content was not known, has to be read again | Done 2026-11-21 |
| `Read`: pipes and devices | A pipe is read as a stream, which an interrupt ends; device files that would block or never end (`/dev/stdin`, `/dev/tty`, `/dev/zero`, `/dev/random` and the like) are refused, and other special files such as `/dev/null` are read | Every file that is not a regular file or a directory is refused, pipes and `/dev/null` included | Done 2026-11-21 |
| `Glob`, `Grep`: where they are offered | Newer versions leave them out on macOS and Linux, where the model uses `find` and `grep` through `Bash` | Always offered with `Bash` | Done 2026-11-18 `092e61dcc8` |
| `WebFetch`: time limit | 60 seconds a request; from 2.1.268 on the whole fetch has 5 minutes (`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`) | 60 seconds a request | Done 2026-11-18 `5e56c5502a` |
| `Agent`: the result | The agent's answer, then its id (`agentId: ... (use SendMessage ...)`) and a `<usage>` block; the hook's `tool_response` also has `totalToolUseCount`, `totalDurationMs`, `totalTokens` and `usage`, and `outputFile` for a background agent | The agent's answer; `tool_response` has `status`, `agentId`, `content` and `prompt` | Done 2026-11-18 `bf914e9b33` |
| `Write`, `Edit`: the hook's `tool_response` | Has the change as `structuredPatch` | `structuredPatch` is empty | Done 2026-11-17 `c14ae9210a` |
| `Bash`: the hook's `tool_response` | `stdout`, `stderr`, `interrupted`, and more (`isImage`, `returnCodeInterpretation`, `backgroundTaskId`, `persistedOutputPath` and others) | `stdout`, `stderr`, `interrupted` and some of the others | Done 2026-11-17 `c14ae9210a` |

#### Permission rules

With the tool set on, each Claude tool call is checked against the `permissions.allow`, `ask`
and `deny` rules of `~/.claude/settings.json`, `.claude/settings.json` and
`.claude/settings.local.json` before it runs. The rules are matched as Claude Code matches
them (see "Fully compatible"). What happens once a rule has matched is Codex's, because the
decision is handed to Codex's own approval path:

| Rule | What happens | Status |
| --- | --- | --- |
| `deny` | The tool does not run; the model gets Claude Code's denial (`Permission to use Bash with command ... has been denied.`, `Permission to read ... has been denied.`, ...). | Done 2026-11-18 `6c5d824b9f`, `4e25eb7045` |
| `allow` | A `Bash`, `Write` or `Edit` call Codex would ask about runs without asking. It still runs in the sandbox: the rule settles the approval Codex's orchestrator decides for `exec_command` and `apply_patch`, not the sandbox, and it does not cover a request to leave the sandbox (`dangerouslyDisableSandbox`), a command Codex forbids, or a managed rule that wants a person. An MCP call skips Codex's MCP approval. Tools Codex never asks about (`Read`, ...) run as before. | Done 2026-11-18 `6c5d824b9f`, `4e25eb7045` |
| `ask` | A `Bash`, `Write` or `Edit` call asks even where Codex would not, with the rule as the reason, and no "approve for the session" is remembered for later calls. A tool Codex has no approval for (`Read`, `Glob`, `Grep`, `WebFetch`, MCP tools, ...) asks with Codex's approval prompt, showing the tool and its input; declining gives the model Claude Code's rejection. When the approval policy cannot ask (`never`, or `granular` without rule approvals), the call is denied, as Claude Code denies in a non-interactive session. | Done 2026-11-18 `6c5d824b9f`, `4e25eb7045` |
| none | Codex decides, as for its own tools. | Done 2026-11-18 `6c5d824b9f` |

Where a rule is read as in Claude Code and still decides otherwise:

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| `Agent(<type>)` | Works as a deny rule only, which takes the type away; as an allow or ask rule it does nothing | Honoured as deny, ask and allow | Done 2026-11-18 `4e25eb7045` |
| A command of more than 50 parts | Asks | Matched part by part like any other | Done 2026-11-18 `6c5d824b9f` |
| A command with several `cd`, or `cd` with `git` | Always asks, even with an exact allow rule | No allow rule covers it, so Codex decides | Done 2026-11-18 `6c5d824b9f` |
| Allow rules for a command with a command substitution, a subshell, a group or control flow | The commands inside it are matched too, and the whole is allowed when each of them is; `"$(cat <<'EOF' ... EOF)"` is the text it holds | No prefix or wildcard allow rule covers such a command, which Codex decides unless an exact rule names it; `"$(cat <<'EOF' ... EOF)"`, with the delimiter quoted, is the text it holds, so `Bash(git commit:*)` covers a commit message written that way | Done 2026-11-21 |
| `ask` where nobody can be asked: the text the model gets | Depends on the mode (`dontAsk`, a background agent, `claude -p`) | `Permission to use <tool> has been denied`, with the rule as the reason | Done 2026-11-18 `4e25eb7045` |
| `/path` in a path rule of the project's files | Relative to the session's first working directory | Relative to the directory whose `.claude` holds the file, which is the working directory | Done 2026-11-18 `6c5d824b9f` |
| A project's `allow` rules, in a project Codex does not trust (no `trust_level = "trusted"` for it in `config.toml`) | Applied once the folder is trusted | Left out, since the files come with the repository; its `deny` and `ask` rules are kept | Done 2026-11-20 `2a011b3a43` |
| `Read` and `Edit` rules, and shell commands | The deny rules are also checked against the paths of the commands Claude Code understands (`cat`, `sed`, `rm` and the like) and of output redirections | They govern the file tools only: a `Bash(...)` rule is needed for a command | Done 2026-11-18 `6c5d824b9f` |
| `mcp__<server>` and `mcp__<server>__*` for a server whose name has `__` in it or ends in `_` | Newer versions (2.1.299) reach only the server named: `mcp__a` no longer covers a server `a__b` or `a_`, and an agent's `tools` takes only the named server's tools | The name is split at its first `__`, so `mcp__a` also covers the tools of the servers `a__b` and `a_` | Done 2026-11-18 `6c5d824b9f` |
| Symbolic links in the paths of `Read`, `Write`, `Edit`, `Glob` and `Grep` | The path as given and every link on the way to the file are checked: a deny or ask rule for any of them decides, an allow rule has to cover them all. Newer versions (2.1.268) also apply a deny or ask rule written through a symlinked directory (`Read(//etc/**)` on macOS) to a path given by its real location | The path as given and the path its links lead to in the end (for a file not there yet, its nearest existing directory resolved) are checked the same way, so a deny rule for the target denies the link and an allow rule has to cover both. The links in between are not checked one by one, and a rule written through a symlinked directory is matched only as written | Done 2026-11-21 |

What Claude Code has around the rules and Codex does not apply (`defaultMode`,
`additionalDirectories`, Claude Code's own checks, the managed file) is under "Not yet
implemented" and "Incompatible".

#### Adding a tool

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

### Settings

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| `env` | Written into Claude Code's own environment (the user's file at the start, a project's once the folder is trusted), so that Claude Code itself and everything it starts see it: commands, hooks, MCP servers | Added to the environment of the commands Codex runs, not to Codex's own; a project's files only when Codex trusts the project (`trust_level = "trusted"`); `shell_environment_policy.set` in `config.toml` wins over it | Done 2026-11-20 `2a011b3a43` |
| `permissions`, with the Claude tool set off | Applied to Claude Code's tools | An approximation for commands only: a Bash rule that names a command prefix (`Bash(git status)`, `Bash(git diff*)`, `Bash(gh pr *)`, `Bash(npm run:*)`) becomes a Codex command rule of the same prefix, word by word. `allow` runs without asking (and, as for any Codex allow rule, outside the sandbox), `ask` asks, `deny` forbids; the strictest matching rule wins, as in Claude Code. A word cut short by `*` (`--force*`) matches only that whole word. Only the user's file is read. Rules for other tools, wildcards inside a command, `defaultMode` and `additionalDirectories` are not applied | Done 2026-11-17 `80c1279420` |
| `permissions`, with the Claude tool set on | Applied | All of `allow`, `ask` and `deny` from the three files; see "Permission rules". The approximation above is then not loaded | Done 2026-11-18 `6c5d824b9f` |
| A change to the settings files during a session, for hooks and permission rules | The files are watched; an edit applies to the running session, a file created during the session is loaded (in the project's `.claude` also when the folder is created during the session), and the `ConfigChange` hook runs | Hooks, and the permission rules of the Claude tool set while it is on, are read again once one of the settings files was written, created or removed: before each prompt and each tool call, Codex compares each file's length and modification time with what they were when the hooks or rules were last read. A change takes effect at the next prompt or tool call, not while a call runs. `ConfigChange` is not run; the other keys are under "Not yet implemented" | Done 2026-11-21 |
| `effortLevel` | The default effort level, from the settings file of highest precedence that sets it; a level saved for the model in `modelSettings` comes first, `--effort` and `CLAUDE_CODE_EFFORT_LEVEL` override it, and newer versions apply the user's file's `effortLevel` only to Opus 5, Fable 5.1 and older models | The default of `model_reasoning_effort` when Codex's own configuration (`config.toml`, a profile, `-c`) sets none, for every model: `low`, `medium`, `high` and `xhigh` as Codex's levels of the same name, `max` as `xhigh`, any other value passed over; the later file wins, a project's files only when Codex trusts the project. `modelSettings` and `CLAUDE_CODE_EFFORT_LEVEL` are not read. Read once a session | Done 2026-11-21 |

### Subagents

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| What a subagent is | An agent type of its own | An agent role of the same name: its `description` is the role's description, its body the role's developer instructions, and its `effort` the reasoning effort (`low`, `medium`, `high`, `xhigh`, and `max` as `xhigh`; a number is left out) | Done 2026-11-20 `f987126bd2` |
| A subagent and a Codex role of the same name | Not applicable | Codex's own role wins | Done 2026-11-17 `80c1279420` |
| `tools` and `disallowedTools` | Applied | Applied only with the Claude tool set on; ignored without it. With it, the names are the ones the model sees: Claude Code's names, `mcp__<server>__<tool>` for MCP tools, Codex's own name for a Codex tool with no Claude counterpart; a Codex tool that serves a Claude tool goes with it (`write_stdin` with `Bash`, Codex's other agent tools with `Agent`). Hosted tools (Codex's built-in web search) and the tools code mode and tool search add are not filtered | Done 2026-11-18 `bf914e9b33` |
| A subagent starting agents of its own | Newer versions allow it, 3 layers deep (`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`); the reference source does not | Allowed within Codex's depth limit | Done 2026-11-18 `bf914e9b33` |
| A subagent file with an empty body | Accepted | Ignored | Done 2026-11-17 `80c1279420` |
| A subagent whose `tools` gives it no tool | Newer versions (2.1.208) refuse to start it: the `Agent` call fails with a message naming the entries, as unrecognized, not available to subagents, or matched no tools in this session; an absent `tools`, or one left empty once `disallowedTools` is taken off, is not refused | The same refusal and text, judged against the tools of the agent that calls `Agent`; an entry is matched no tools in this session when it names an MCP server or one of Claude Code's tools, unrecognized otherwise | Done 2026-11-21 |
| Which subagent wins among those of one name | A plugin's, then the user's, then the project's, then one given with `--agents`, then a managed one, the later winning | Codex's own role, then the project's, then the user's, then a plugin's, the earlier winning | Done 2026-11-19 `4ff3147125` |
| A subagent's name | Newer versions reject a name longer than 256 characters (2.1.292), and an agent file whose name has `:` in it, which is kept for the plugin's namespace (2.1.218) | Any name is taken: a file's `:` is kept, and such an agent can take the name of a plugin's agent | Done 2026-11-17 `80c1279420` |

### MCP servers

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| A server of a name Codex configures itself | Not applicable | Left to Codex's configuration | Done 2026-11-17 `80c1279420` |
| Approval of a project's server recorded in the project's entry of `~/.claude.json` | An old place, since moved to the settings files | Still counts | Done 2026-11-17 `80c1279420` |
| A project's server nobody has approved, in a session without a person | Started, unless it is disabled | Never started | Done 2026-11-17 `80c1279420` |
| Names of MCP tools | `mcp__<server>__<tool>`, the server's name with anything but letters, digits, `_` and `-` turned into `_` | Codex's own naming of MCP tools, which permission rules and hook matchers see | Done 2026-11-17 `80c1279420` |
| A project's own settings files approving its `.mcp.json` servers (`enabledMcpjsonServers`, `enableAllProjectMcpServers` in `.claude/settings.json` or `.claude/settings.local.json`) | Count once the folder is trusted; newer versions ignore them in a folder that is not | Count only in a project Codex trusts. Elsewhere only the user's own files approve a server; what the project's files disable is honoured either way | Done 2026-11-20 `2a011b3a43` |
| Credential variables in a remote server's `url` and `headers` | Read as empty, a `:-default` included, so that a declaration cannot send them away; the documentation gives examples, not the full list | Read as empty for the examples Claude Code names (`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `AWS_BEARER_TOKEN_BEDROCK`, `HTTPS_PROXY`, `NPM_TOKEN` and the like) and Codex's own (`OPENAI_API_KEY`, `CODEX_API_KEY`); another credential expands | Done 2026-11-20 `f987126bd2` |
| An MCP tool call's time limit when the server has no `timeout` | `MCP_TOOL_TIMEOUT`, or else about 28 hours; a call that sends nothing for 5 minutes (30 for a stdio server, `CLAUDE_CODE_MCP_TOOL_IDLE_TIMEOUT`) is ended sooner | `MCP_TOOL_TIMEOUT` (see "Fully compatible"), or else Codex's default of 5 minutes | Done 2026-11-21 |
| A remote server's `headersHelper` | A shell command, run on each connection and again after a `401` or `403`, whose JSON object of headers is sent over the static `headers`, within 10 seconds; it sees `CLAUDE_CODE_MCP_SERVER_NAME`, `CLAUDE_CODE_MCP_SERVER_URL` and, for a plugin's server, `CLAUDE_PLUGIN_ROOT`; it runs from the plugin's root, the project's directory or `~/.claude`, by where the server is declared; a project's or plugin's helper runs without the credential variables, and newer versions (2.1.238) run that of `.mcp.json` or of the project's entry in `~/.claude.json` only once the folder is trusted; a helper that fails leaves the static `headers`; `${user_config.*}` in a plugin's helper makes the server misconfigured (2.1.207) | It becomes Codex's `http_headers_helper`, with the same variables, time limit and rerun, its headers sent over the static ones save `Authorization`, where a static one (or a Codex login's token) wins; a plugin's run from the plugin's root, and that of `.mcp.json` or of the project's entry in `~/.claude.json` dropped in a project Codex does not trust. Other helpers run from Codex's working directory, and every helper gets only Codex's minimal environment for MCP servers (`HOME`, `PATH`, `USER`, `LANG`, `TMPDIR` and the like), the user's own servers' included. A helper that fails makes the connection fail. `${user_config.*}` in a plugin's helper leaves the server out with an error. Codex's own `http_headers_helper` in the declaration wins | Done 2026-11-21 |
| A server's `oauth` block | `clientId`, `callbackPort`, `scopes` (one space-separated string, which wins over the discovered scopes) and `authServerMetadataUrl`; the client secret is kept in the keychain, set when the server is added | `clientId` and `callbackPort` become the server's `oauth.client_id` and `oauth.callback_port`, `scopes` its `scopes`; `authServerMetadataUrl` is dropped with a warning in the log (Codex discovers the metadata). Claude Code's stored secret and sign-in are not read: sign in with `codex mcp login <server>` | Done 2026-11-21 |

### Auto memory

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| Where the instructions and the index are shown | The instructions in its system prompt, the index among the `CLAUDE.md` files | Both together, after the project's instructions | Done 2026-11-17 `34b9401100` |
| Tool names in the instructions | Claude Code's | Codex's: files are written with `apply_patch`, the task list is `update_plan`, and `AGENTS.md` is named beside `CLAUDE.md` among the files whose content is not to be saved again | Done 2026-11-17 `34b9401100` |
| Writing a memory without an approval | Always | When the session may write its workspace (`workspace-write`), the directory is added to the sandbox's writable roots; a read-only session stays read-only, and full access needs nothing. Requirements that do not allow the extra root leave the sandbox as it was. Changing the permissions during a session (`/permissions`) drops the extra root | Done 2026-11-17 `34b9401100` |
| A project whose path is longer than 200 characters | The directory's name is cut to 200 characters and a hash | No memory directory is used | Done 2026-11-17 `34b9401100` |
| A project's settings files naming the directory (`autoMemoryDirectory` of `.claude/settings.local.json`, the memory variables in their `env`), in a project Codex does not trust | Taken once the folder is trusted | Not taken: the directory becomes writable, and the files come with the repository. They can still turn the memory off | Done 2026-11-20 `2a011b3a43` |

To keep the memory directory writable whatever the permission mode, add it to `config.toml`
yourself:

```toml
[sandbox_workspace_write]
writable_roots = ["<MEMORY_DIR>"]
```

### Plugins

Codex's plugin loader already reads Claude Code's plugin format (`.claude-plugin/plugin.json`,
`skills/`, `.mcp.json`, `hooks/hooks.json`); what it does not know is which plugins the user
has. Those are taken from Claude Code's own records, and each plugin that is installed and
enabled is handed to Codex's plugin loader as an enabled plugin of the same id, at its install
path: nothing is copied, and Codex's `config.toml` is not written. `[plugins]` policy in Codex's
configuration applies to these plugins as to Codex's own, and `CCN_CLAUDE_COMPAT=0` or turning
Codex's `plugins` feature off leaves them all out.

| Behaviour | Claude Code | Codex | Status |
| --- | --- | --- | --- |
| A plugin Codex configures under the same id, or has installed under the same name from any marketplace | Not applicable | Codex's wins, and the Claude Code one is left out | Done 2026-11-18 `471d20cf39` |
| A plugin that relies on a mechanism only Claude Code has | Used | Disabled whole, with a startup warning; see "Which plugins are used" below | Done 2026-11-18 `471d20cf39` |
| Where `${CLAUDE_PLUGIN_ROOT}` is filled in | Hook, MCP and LSP declarations, the bodies of skills, commands and agents, and the environment of hooks and of MCP and LSP servers | Hook commands, MCP server declarations, agent bodies, and the environment of hooks | Done 2026-11-19 `4ff3147125` |
| Which files give `userConfig` its plain values | `pluginConfigs` of all the settings files, merged | `pluginConfigs` of `~/.claude/settings.json` only | Done 2026-11-19 `4ff3147125` |
| `${CLAUDE_PLUGIN_DATA}` in an MCP server's declaration | Filled in | Left as written; a hook's environment has it | Done 2026-11-19 `4ff3147125` |
| `${user_config.<key>}` in a hook's command written for a shell, a monitor or a `headersHelper` | Newer versions (2.1.207) refuse it as shell injection: a hook passes the value through `args` or `$CLAUDE_PLUGIN_OPTION_<KEY>` | Filled into the command's text | Done 2026-11-19 `4ff3147125` |

#### Which plugins are used

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

## Fully compatible

What behaves as in Claude Code. The right-hand column says what this fork does to make it so;
where a row has a limit, it is in "Differences from Claude Code" above, under the same area.

### Instructions and rules

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| Which files are instructions | `~/.claude/CLAUDE.md`, `~/.claude/rules/**/*.md`, and in a project `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/**/*.md`, `CLAUDE.local.md` | The same files, read in place | Done 2026-11-17 `80c1279420` |
| Order of project files | Every directory from the top of the filesystem down to the working directory | The same order, and in each directory the order of the row above. In the home directory, `.claude` is the user's own configuration and is not read a second time | Done 2026-11-17 `80c1279420` |
| How a file is shown | A section headed `Contents of <path> (<kind>):`, under the line that these instructions override default behaviour | The same heading and the same line | Done 2026-11-17 `80c1279420` |
| Frontmatter and comments | YAML frontmatter and HTML comment blocks are removed (a comment inside a paragraph stays); files that are left empty are skipped | The same | Done 2026-11-17 `80c1279420` |
| `@<path>` includes | Another file is included after the one naming it (`@./x`, `@~/x`, `@/abs/x` or `@x`), up to four files down the chain, not inside code blocks or code spans, and only a file of a text type; in `CLAUDE.md`, `CLAUDE.local.md` and the rules that apply to every file; a project file may include files under the working directory, `~/.claude/CLAUDE.md` and the user's rules any | The same; includes outside the working directory are under "Differences" | Done 2026-11-20 `f987126bd2` |
| Globs of a rule's `paths:` | Gitignore syntax, relative to the working directory for the user's rules and to the directory holding `.claude` for a project's; `{a,b}` is expanded, within a budget in newer versions (2.1.217); a list or a comma-separated string; a malformed pattern breaks nothing (2.1.207) | The same; a pattern that would expand into more than 1,024 globs is matched with its groups as written, and a glob the matcher cannot read (`docs/{draft`, `[z-a]`) is skipped while the rule's other globs still apply | Done 2026-11-21 |
| Which rules wait for a file | Rules with `paths:` (the user's, and those of every directory from the top of the filesystem down to the working directory), and the `CLAUDE.md`, `.claude/CLAUDE.md`, `CLAUDE.local.md` and rules of directories below the working directory, for a file inside the working directory, also one reached through a symlinked path (2.1.198) | The same: `..` in the file's path is resolved, and a file that is then not under the working directory is compared by its real path, symlinks in either path resolved | Done 2026-11-21 |
| Files shown on demand, after the conversation is compacted | Forgotten, and shown again the next time a file they cover is worked on | The same | Done 2026-11-20 `f987126bd2` |
| Files shown on demand, after a resume | Not shown a second time (2.1.287) | The same: a resumed or forked session reads the files already shown back from the headings in its history, which after a compaction holds only those shown since | Done 2026-11-21 |
| Project instructions and `MEMORY.md` after a compaction | Read again, so that edits made earlier in the session are seen (2.1.311) | The same: they are read again at the next request after a compaction and shown anew when they changed; the user's files are read at every request anyway | Done 2026-11-21 |
| A very large file shown on demand | A nested `CLAUDE.md` over the size limit (40,000 characters) loads only its start (2.1.309) | The same, for every file shown on demand, with a line saying it was cut | Done 2026-11-21 |

### Skills and slash commands

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| Skill roots | `~/.claude/skills/`, and `.claude/skills/` in each directory from the working directory up to the repository's root | A user skill root and a repository skill root in each of those directories, read by Codex's own skill loader beside `~/.agents/skills` and `.agents/skills` | Done 2026-11-17 `80c1279420` |
| `description` missing or empty | The first non-empty line of the body stands in, its heading marks removed, cut to 100 characters | The same | Done 2026-11-17 `2471894159` |
| `when_to_use` | Shown with the description | Appended to the description as ` - <text>` | Done 2026-11-17 `2471894159` |
| Custom commands | `~/.claude/commands/<name>.md`, `.claude/commands/<name>.md` of a project and the directories up to the working directory | Loaded in place as skills of that name by the same loader, so they are listed, invoked and expanded as skills are | Done 2026-11-18 `c58a3409c6` |
| A command in a subdirectory | Named after its path, the directories joined with `:` (`commands/git/commit.md` is `/git:commit`); frontmatter `name` is ignored | The same | Done 2026-11-18 `c58a3409c6` |
| A plugin's commands | `<plugin>:<name>`, with the subdirectories in between | The same, as skills (`commands/`, or the directories the manifest's `commands` lists) | Done 2026-11-19 `3134f1b61a` |
| A command's frontmatter | Optional; `description`, or else the first line of the body, is what the model sees; `disable-model-invocation: true` keeps it out of the model's list | The same | Done 2026-11-18 `c58a3409c6` |
| `/<name> <args>` | Invokes the skill or command | In the TUI, for any skill Codex lists (Claude Code's and Codex's own). Once a name is typed after `/`, the slash popup lists the matching skills after the commands; Enter or Tab completes the name. The message is sent as typed, with the skill attached as `$<name>` would attach it | Done 2026-11-17 `0d118bdf09` |
| What the model is given for `/<name> <args>` | The body without its frontmatter, headed `Base directory for this skill: <dir>` when the skill is a directory | The same, when the skill's `SKILL.md` is a file on this machine and Codex itself puts it into the conversation | Done 2026-11-17 `0d118bdf09` |
| `$ARGUMENTS`, `$ARGUMENTS[N]`, `$N` | The arguments as typed; the N-th word, counted from 0 and split as a shell splits words, with quotes. Newer versions (2.1.210) keep `$ARGUMENTS[N]` and `$N` as written when there is no N-th word, so that `$1` in an `awk` program or `$B$2` stays; newer versions (2.1.233) also put each value in once, without reading it again for placeholders | The same | Done 2026-11-21 |
| Named arguments | `$<name>` is the word at the position of `<name>` in the `arguments` frontmatter (a list, or names separated by spaces); empty when there is no such word | The same | Done 2026-11-17 `0d118bdf09` |
| A frontmatter value with `: ` in it (`description: Triggers include: X`) | Put in quotes and the frontmatter read again (2.1.69) | The same, for the skill and for its `arguments` | Done 2026-11-21 |
| A skill, command or subagent file that starts with a UTF-8 byte order mark | Newer versions (2.1.239) read it as if it had none | The same, and a rule file too | Done 2026-11-21 |
| No placeholder in the body | The arguments are appended as `ARGUMENTS: <args>`; in newer versions (2.1.210) also when the only placeholders are `$ARGUMENTS[N]` or `$N` with no N-th word | The same | Done 2026-11-21 |
| `${CLAUDE_SKILL_DIR}` | The skill's directory | The same, also when the skill is mentioned as `$<name>`; such a mention keeps the rest of the text as it is, `$ARGUMENTS` included | Done 2026-11-17 `0d118bdf09` |

### Hooks

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| The `hooks` key | Events, each with groups of a `matcher` and handlers; a handler has `type`, `timeout`, `async`, `statusMessage`, `if`, and what its type needs (`command`; `prompt`, `model`; `url`, `headers`, `allowedEnvVars`) | Read by Codex's hook engine, which takes the same shape, from the same three settings files | Done 2026-11-17 `80c1279420` |
| A malformed entry under `hooks` (a handler without `command`, a `matcher` that is not a string, an event that is not a list) | Left out, the file's other hooks kept; `timeout` takes any positive number of seconds | The same, with a warning; a `timeout` with a fraction is rounded up to whole seconds | Done 2026-11-21 |
| The user's own hooks | Run without a question for each hook | Run without Codex's per-hook trust prompt | Done 2026-11-17 `80c1279420` |
| `CLAUDE_PROJECT_DIR` | The project directory | The working directory | Done 2026-11-17 `df06fbae01` |
| Input on stdin | `cwd`, `hook_event_name`; `tool_name`, `tool_input`, `tool_use_id` and `tool_response` for tool events; `prompt` for `UserPromptSubmit`; `source` (`startup`, `resume`, `clear`, `compact`) for `SessionStart`; `stop_hook_active` and `last_assistant_message` for `Stop`; `agent_id`, `agent_type` and `agent_transcript_path` for `SubagentStop`; `agent_id` and `agent_type` in a subagent's hooks; `message`, `title` and `notification_type` for `Notification` | The same fields | Done 2026-11-17 `80c1279420` |
| JSON output | `hookSpecificOutput.permissionDecision` for `PreToolUse`: `deny` blocks, `allow` runs the call without asking, `ask` asks about it, and `updatedInput` changes its input; `decision: "block"` with `reason` for `PreToolUse`, `Stop`, `SubagentStop`, `UserPromptSubmit` and `PostToolUse`, and the legacy `decision: "approve"` for `PreToolUse`; `additionalContext` for `SessionStart`, `SubagentStart`, `UserPromptSubmit`, `PreToolUse` and `PostToolUse`; `systemMessage`, `continue`, `stopReason` and `suppressOutput` | The same for these; what Codex refuses is under "Not yet implemented". An allowed call still meets the deny and ask rules of the settings files, as in Claude Code | Done 2026-11-20 `f987126bd2` |
| Plain stdout | Context for `SessionStart` and `UserPromptSubmit` | The same | Done 2026-11-17 `80c1279420` |
| Tool events with the Claude tool set on | Claude Code's tool names, input and `tool_response` | `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `WebSearch`, `TodoWrite`, `AskUserQuestion`, `Agent` and `Skill` are the tools' own names: the hook input carries that name and Claude Code's input (`command`, `timeout`, `description`; `file_path`, `old_string`, `new_string`; ...), with the `file_path` of `Read`, `Write` and `Edit` made absolute, as Claude Code gives it, and `PostToolUse` gets Claude Code's `tool_response` (`stdout`, `stderr`, `interrupted` for `Bash`; `bytes`, `code`, `codeText`, `result`, `durationMs`, `url` for `WebFetch`; `query`, `results`, `durationSeconds` for `WebSearch`) | Done 2026-11-17 `c14ae9210a`; absolute `file_path` done 2026-11-21 |
| A shell command that exits with a non-zero code | A failure: `PostToolUseFailure` runs, with `error` set to `Exit code N` and the output, and `PostToolUse` does not | The same for the settings files' hooks. `additionalContext`, or stderr with exit code 2, reaches the model | Done 2026-11-17 `df06fbae01` |
| A tool call cancelled while it runs | Runs no `PostToolUseFailure` hook: the tool's result carries the interruption instead | The same | Done 2026-11-21 |
| `Notification` and `StopFailure` | Run in the background; their output is ignored | The same. The matcher is matched against `notification_type`, or against the kind of error | Done 2026-11-17 `df06fbae01` |
| `if` | A handler with an `if` runs only for the tool calls its permission rule covers (`PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `PermissionRequest`), and never for another event | The same. The rule's tool is compared with the names the call is matched by. Its content is matched for the tools Claude Code matches it for: `Bash(git *)` against each simple command of the command line, in the forms deny rules meet (`sudo git push` and `"git" push` run it), those nested in command substitutions, backticks, subshells, groups and control flow included, so that `echo $(date)` does not run it (a command that cannot be taken apart, such as one with an unclosed quote, runs the hook), `Read`, `Edit` and `Write` against `file_path` as a path rule matches it (`.env` at any depth under the working directory, `src/**` only at its top, `~/.ssh/**` in the home directory, `//etc/**` absolute), `Glob` and `Grep` against `pattern`; content for any other tool never matches | Done 2026-11-19 `358f5dc8ca`; nested commands, words, wrappers and paths done 2026-11-21 |
| `once` in the settings files | Does nothing: it only applies to the hooks of a skill's frontmatter | Does nothing | Done 2026-11-19 `358f5dc8ca` |
| `type: "prompt"` and `type: "agent"`: the verdict | A model reads the handler's `prompt`, with the hook input in place of `$ARGUMENTS` (after the prompt, as `ARGUMENTS: ...`, when it names none), and answers `{"ok": true}` or `{"ok": false, "reason": "..."}`. The prompt may be a condition that must hold ("All unit tests pass") or a rule about what to block or allow ("Block any Bash command that reads `.env` files"). An answer that is not such JSON, an error or a timeout (30 s for `prompt`, 60 s for `agent`, or the handler's `timeout`) does not block | The same; the model is told how to read either kind. What a condition that is not met does is under "Differences" | Done 2026-11-21 |
| `type: "http"` | The hook input is posted as JSON to `url`, and the JSON object the server answers with (or an empty body) is read as a command's stdout is. A response outside 2xx, a failed request, a body that is not a JSON object and a timeout (600 s, or `timeout`) are errors that do not block | The same | Done 2026-11-19 `57b49d1405` |
| `type: "http"`: headers | `headers` values may name environment variables (`$NAME`, `${NAME}`), filled in only for the names in the handler's `allowedEnvVars` and empty otherwise | The same | Done 2026-11-19 `57b49d1405` |
| `type: "http"`: limits | `allowedHttpHookUrls` and `httpHookAllowedEnvVars` of the settings files (lists of several files are joined) narrow which URLs and variables any handler may use; redirects are not followed; without a proxy for the host, a host that resolves to a private or link-local address is refused, loopback excepted; `http` handlers do not run for `SessionStart` | The same | Done 2026-11-19 `57b49d1405` |
| An enabled plugin's hooks | Run without asking | The same for a plugin the user's own `~/.claude/settings.json` enables. They see `CLAUDE_PLUGIN_ROOT` (the install path), `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` and `CLAUDE_CODE_ENTRYPOINT` | Done 2026-11-18 `471d20cf39` |
| `http` handlers of a plugin's hooks file | Run, within the settings files' `allowedHttpHookUrls` and `httpHookAllowedEnvVars` | The same for a Claude Code plugin; a Codex plugin's hooks file keeps Codex's format, which has no `http` handlers | Done 2026-11-21 |
| Events | `PreToolUse`, `PostToolUse` and `PermissionRequest` (matcher: the tool's name), `UserPromptSubmit`, `SessionStart` (matcher: `source`), `SessionEnd`, `Stop`, `SubagentStart` and `SubagentStop` (matcher: the agent's type), `PreCompact` and `PostCompact` (matcher: `manual` or `auto`) | The same events of Codex's own hook engine, run at Codex's moments of the same name | Done 2026-11-17 `80c1279420` |
| A `matcher` of names | Only letters, digits, `_`, `-`, spaces, `,` and `\|`: a list of exact names separated by `\|` or `,`, spaces around them ignored (`Edit, Write`, `code-reviewer`, `mcp__brave-search`); `StopFailure` keeps the narrower set of letters, digits, `_` and `\|` | The same | Done 2026-11-21 |
| A command handler's time limit | 600 seconds, or its `timeout` | The same | Done 2026-11-17 `80c1279420` |
| A command handler that leaves a process in the background (`some-daemon &`) holding its output open | Ends shortly after its own process exits, the background process left running | The same: output is read for 200 ms more, then the hook is done and the process is not killed | Done 2026-11-21 |
| Handlers of one event | Run side by side | The same | Done 2026-11-17 `80c1279420` |
| A block without a reason (`decision: "block"` without `reason`, `permissionDecision: "deny"` without `permissionDecisionReason`) | Blocks, with `Blocked by hook` as the reason | The same | Done 2026-11-20 `f987126bd2` |
| A field a hook's JSON output has and the program does not know (one of a later version, a `hookSpecificOutput` of another event) | Left out; the rest of the output counts | The same | Done 2026-11-20 `f987126bd2` |
| Several `PreToolUse` hooks that decide differently | A block wins over ask, ask over allow | The same | Done 2026-11-20 `f987126bd2` |
| Two handlers that are the same (same command and `if`; same prompt; same URL), in the settings files or in one plugin | Run once, the last of them kept | The same | Done 2026-11-20 `f987126bd2` |
| The shell that runs a command handler | `/bin/sh -c` (Git Bash on Windows) | `/bin/sh -c` for the hooks of Claude Code's settings files and plugins; Codex's own hooks keep the user's shell | Done 2026-11-20 `f987126bd2` |
| What a command handler gets on stdin | The JSON input and a line break | The same | Done 2026-11-20 `f987126bd2` |
| A hook that exits with a code other than 0 and 2 | An error that does not block, shown with the first line of the hook's stderr | The same (`hook exited with code N: <line>`); a `prompt` or `agent` hook whose request fails or times out is shown with its error the same way | Done 2026-11-21 |
| Where a command handler runs once the session's working directory has been removed (a deleted worktree) | In the project directory, or else in the home directory | The same | Done 2026-11-21 |

### Claude tool set

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| Parameters, descriptions and result texts of `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebSearch`, `WebFetch`, `TodoWrite`, `AskUserQuestion`, `Agent` and `Skill` | Claude Code's own | The same, produced by the layer over the Codex tool named in "How each tool is made", except for the parameters and texts the other chapters name | Done 2026-11-18 `c14ae9210a`, `092e61dcc8`, `5e56c5502a`, `bf914e9b33` |
| `Bash`: `timeout` | Milliseconds, 2 minutes by default, at most 10 | The same; what happens at the limit, and the variables that move it, are under "Not yet implemented" | Done 2026-11-17 `c14ae9210a` |
| `Bash`: result | Output trimmed; `Exit code N` and the output for a failed command; `(Bash completed with no output)` | The same | Done 2026-11-17 `c14ae9210a` |
| `Bash`: words starting with `=` under zsh | Newer versions (2.1.328) run commands with zsh's `EQUALS` option off, so `[ a == b ]` and `echo ====` work and `=cmd` is not expanded to a command's path | The same: `setopt NO_EQUALS` is put before the command when zsh runs it, after approval, so approval prompts, command rules and the TUI show the command as given | Done 2026-11-21 |
| `Read`: reading | `N<TAB>line` numbering from 1; the whole file when it is within the limits (256 KB without `limit`, 25,000 tokens); binary files and directories refused; the empty-file and short-file reminders | The same | Done 2026-11-17 `c14ae9210a` |
| `Read`: a second read of the same range of an unchanged file | Read again (newer versions, 2.1.317; the reference source answers `File unchanged since last read`) | The same | Done 2026-11-17 `c14ae9210a` |
| `Read`: notebooks | Each cell as `<cell id="...">` with its type or language, then its outputs: text, images, an error's traceback; outputs of more than 10,000 characters replaced by a `jq` command; the 256 KB and 25,000-token limits on the cells as read | The same; images only for a model that takes them | Done 2026-11-20 `897f7f4914` |
| `Edit`: matching | Exact; refused when `old_string` is not found, or found more than once without `replace_all` | The same | Done 2026-11-17 `c14ae9210a` |
| `Write`, `Edit`: a file changed since it was read | Refused, unless it was read whole (or written) and its content is still the same, whatever its modification time says; newer versions (2.1.295) also refuse a file whose content changed while its modification time did not move. What a `PostToolUse` hook of the call wrote into the file (a formatter) counts as written by the call. The 100 files used last are remembered, and a file forgotten has to be read again | The same; a file read only in part counts as changed when its modification time or size is not what it was | Done 2026-11-21 |
| `Glob`: results | Sorted by modification time, oldest first, and the first 100 files kept, with a truncation note; hidden and ignored files included; paths inside the working directory shortened; `No files found`; newer versions say that the files found are partial when the search hit its time limit (2.1.317) | The same | Done 2026-11-18 `092e61dcc8`; time limit done 2026-11-21 |
| `Grep`: options and output | `output_mode`, `-i`, `-n`, `-A`, `-B`, `-C`, `context`, `multiline`, `glob`, `type`; `head_limit` (250 by default, 0 for all) and `offset` with the pagination note; hidden files searched, VCS directories left out, `--max-columns 500`; `Found N files`, `No files found` (`No matches found` for the other modes), `Found N total occurrences across M files.`; the search stops after 20 seconds, and newer versions say that the results found by then are partial (2.1.317); newer versions sort lines and counts by path so that `offset` pages through the same order (2.1.327), count the totals of the whole search on every page (2.1.208), and say so when `offset` is past the last result instead of reporting no matches (2.1.210) | The same | Done 2026-11-18 `092e61dcc8`; paging and time limit done 2026-11-21 |
| `WebFetch`: checks and limits | A URL of at most 2000 characters without credentials and with a dotted host; HTTP upgraded to HTTPS; same-site redirects (`www.` added or removed) followed up to 10, other redirects reported; 10 MB; HTML to Markdown; 100,000 characters given to the secondary model, with the quoting rules of its prompt; a Markdown page of a preapproved host is returned as it is | The same | Done 2026-11-18 `5e56c5502a` |
| `WebFetch`: the secondary model answers no text | Newer versions (2.1.321) fail the call with an error saying so; before, `No response from model` was returned as the page's content | The call fails, and the error suggests a narrower prompt or `curl` through `Bash` | Done 2026-11-21 |
| `WebSearch`: parameters | `query` of at least 2 characters; `allowed_domains` or `blocked_domains`, not both | The same, with `allowed_domains` sent as the search's domain filter and `blocked_domains` as `-site:` operators | Done 2026-11-18 `5e56c5502a` |
| `TodoWrite`: checks | `content`, `status` and `activeForm` for each item, both forms non-empty | The same; the step in progress shows its `activeForm` | Done 2026-11-18 `bf914e9b33` |
| `AskUserQuestion`: shape | 1-4 questions of `question`, `header` and 2-4 options; unique questions and labels; the answer as `"question"="answer"`, with `user notes:`; not offered to subagents | The same | Done 2026-11-18 `bf914e9b33` |
| `Agent`: types | The description lists each agent type with its tools; an unknown type gets Claude Code's message | The same, with `general-purpose` as Codex's default role | Done 2026-11-18 `bf914e9b33` |
| `Grep`: ripgrep fails without a result (an invalid pattern, a path it cannot read) | From 2.1.208 on, ripgrep's message is returned; before, no match was reported | The model is told ripgrep's message | Done 2026-11-18 `092e61dcc8` |
| `Edit`: files that are not UTF-8 | Refused | The same | Done 2026-11-17 `c14ae9210a` |
| `Edit`: an empty `old_string` | Creates the file, or fills an empty one | The same | Done 2026-11-17 `c14ae9210a` |

### Permission rules (with the Claude tool set on)

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| Sources | `permissions.allow`, `ask` and `deny` of `~/.claude/settings.json`, `.claude/settings.json` and `.claude/settings.local.json` | The same three files | Done 2026-11-18 `6c5d824b9f` |
| Precedence | Deny wins over ask, ask over allow; a rule for a whole tool (`Bash`, `WebSearch`) is looked at first for deny and ask, last for allow | The same | Done 2026-11-18 `6c5d824b9f` |
| A tool denied as a whole (`WebSearch`, `mcp__<server>`, `mcp__<server>__*`, `mcp__<server>__<tool>`) | Not offered to the model at all | The same: the tool is taken out of the tools the model is offered, MCP tools included; a deny rule with content (`Bash(rm *)`) takes nothing away, and `Glob` and `Grep` stay when `Bash` is denied | Done 2026-11-21 |
| `Bash(...)` | An exact command; a legacy prefix `npm run:*` (the word itself, or followed by a space, also after `xargs`); or a pattern with `*` wildcards (`git * main`; `\*` is a literal star; a single trailing ` *` also matches the bare command) | The same | Done 2026-11-18 `6c5d824b9f` |
| Compound commands | Taken apart at `&&`, `\|\|`, `;`, `\|`, `\|&`, `&` and line breaks outside quotes, heredoc bodies left out, a part that is `cd <working directory>` dropped: a deny or ask rule for any part decides the whole, an allow rule allows it only when every part is allowed (or the whole command is an exact rule). Deny and ask rules also meet the commands nested in command and process substitutions, backticks, subshells, groups and the bodies of `if`, `for`, `while` and `case` (`echo "$(git clean -f)"` meets `Bash(git clean *)`) | The same | Done 2026-11-18 `6c5d824b9f`; nested commands done 2026-11-21 |
| What is dropped before matching | Output redirections to plain files, safe variable assignments (`LANG=C`, `NODE_ENV=...`, the list Claude Code keeps) and, at the start, the wrappers `timeout`, `time`, `nice`, `stdbuf`, `nohup`, the builtins `command` (not `command -v`) and `builtin`, and zsh's `noglob`; extra blanks between words count as one space. Deny and ask rules drop any variable assignment, whatever its value (`TZ="$HOME" rm`), read quoted and escaped words as the shell does (`"rm"`, `\rm`), and also look through `!`, `sudo`, `doas`, `env`, `exec`, `watch`, `ionice`, `setsid` and `flock` with their options | The same | Done 2026-11-18 `6c5d824b9f`; words and wrappers done 2026-11-21 |
| Limits of allow rules for commands | A prefix or wildcard allow rule never matches a part that still looks compound | The same | Done 2026-11-18 `6c5d824b9f` |
| `Read(...)` and `Edit(...)` | Gitignore patterns, compared without regard to case. `//path` is absolute, `~/path` is in the home directory, `/path` is relative to the settings file's root (`~/.claude` for the user's file), anything else (`*.env`, `./src/**`) is relative to the working directory, and a pattern without a slash matches at any depth. `dir/**` covers the directory and everything in it: in an allow rule a single directory (`Edit(src/**)`) only at the top (`<cwd>/src`, newer versions, 2.1.214), in a deny or ask rule at any depth; `/**`, `~/**` and `//**` cover everything under their anchor | The same | Done 2026-11-18 `6c5d824b9f`; anchors done 2026-11-21 |
| A `Read(...)` or `Edit(...)` pattern that is no usable glob | Newer versions (2.1.260): a deny or ask rule guards the literal path it spells; an allow rule approves nothing | The same, and the rule is named in the log. Which patterns fail is the pattern library's: here an unclosed `[` is read literally, and an unclosed `{`, a lone `}` or a reversed range (`[z-a]`) fails | Done 2026-11-21 |
| Which tools a path rule governs | `Read` rules govern `Read`, `Glob` and `Grep`, and `Glob` and `Grep` leave out the files a `Read` deny rule covers; `Edit` rules govern `Edit` and `Write` (`Write(...)` rules with a path do nothing); an `Edit` allow rule also allows reading | The same; the files a deny rule covers are left out by ripgrep globs (compared without regard to case), and the search then runs in the directory searched | Done 2026-11-18 `6c5d824b9f`; files left out done 2026-11-21 |
| `WebFetch(domain:<host>)` | The URL's host name, compared without regard to case and with a trailing `.` taken off both the rule and the host name (`example.com.` is `example.com`) | The same; the wildcards of newer versions are under "Not yet implemented" | Done 2026-11-21 |
| `mcp__<server>__<tool>`, `mcp__<server>`, `mcp__<server>__*` | An MCP tool, or every tool of a server | The same; Codex's MCP tools are checked under the same names. A server whose name has `__` in it differs (see "Differences") | Done 2026-11-18 `6c5d824b9f` |
| `Agent(<type>)` as a deny rule, `Task` read as `Agent` | The type is denied | The same decision, with the layer's own denial text | Done 2026-11-18 `4e25eb7045` |
| A denied call | The model gets a denial (`Permission to use Bash with command ... has been denied.`, `Permission to read ... has been denied.`, ...) | The same text | Done 2026-11-18 `6c5d824b9f` |
| `ask` where nobody can be asked | The call fails without a question, in a session without a person | Denied when the approval policy cannot ask (`never`, or `granular` without rule approvals) | Done 2026-11-18 `6c5d824b9f` |
| Files no allow rule covers | `.git`, `.vscode`, `.idea`, `.claude` (except `.claude/worktrees`), shell profiles, `.gitconfig`, `.gitmodules`, `.ripgreprc`, `.mcp.json` and `.claude.json` are asked about whatever the allow rules say, a rule for the whole `Edit` tool included | The same files are left out of every allow rule; who asks is under "Incompatible" | Done 2026-11-20 `2a011b3a43` |

### Settings, subagents and MCP servers

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| `env` of the settings files | The user's file, then the project's and the local one, later files winning | The same order | Done 2026-11-17 `80c1279420` |
| `.claude/settings.local.json` of the repository's root, from a subdirectory | Newer versions (2.1.211) keep the local file at the root of the Git repository (the main checkout's root, from a linked worktree) and read it from any directory below, beside one an earlier version left in the working directory: the root's file wins where both set a key, the permission rules of both apply, and `/path` rules still start at the working directory. The file stays in the working directory alone outside a repository, when the root is the home directory, on Windows, and when the root, its `.git` or its `.claude` is not the user's. `.claude/settings.json` is read from the working directory only | The same, for every part of the settings files this fork reads: hooks, `env`, `effortLevel`, permission rules, enabled plugins and MCP approvals, and auto memory | Done 2026-11-21 |
| Subagent files | `~/.claude/agents/**/*.md` and `.claude/agents/**/*.md`, with `name` and `description` in the frontmatter and the body as the prompt; a project's win over the user's | The same, for `.claude/agents` of the working directory; a project marked untrusted supplies none (see "Incompatible") | Done 2026-11-17 `80c1279420`; untrusted projects done 2026-11-21 |
| Other Markdown files in an agents directory (a README, notes) | Passed over without a warning: a file without a `name` (or without a `description`) is no subagent | The same; such a file is noted in the debug log only | Done 2026-11-21 |
| A subagent's frontmatter that is not valid YAML | A value holding `: ` or another character YAML reads as syntax is put in quotes and the frontmatter read again, so `description: Use this agent when: ...` loads; a bare number or boolean (`name: 3000`) is taken as text (2.1.47) | The same; a value that is valid YAML by itself (a list such as `[a, b]`) is left unquoted | Done 2026-11-21 |
| A subagent's `tools` list, with the Claude tool set on | Every tool a subagent may have when absent or `*`, none when empty; `disallowedTools` taken away, where `Bash(git *)` takes all of `Bash`; `Task` counts as `Agent`; a whole MCP server is named as `mcp__<server>` or `mcp__<server>__*` (every tool whose name starts with `mcp__<server>__`), and `disallowedTools` takes every MCP tool away with `mcp__*` | The same. The list is read from the subagent file each time the agent's tools are built, and applied after MCP and dynamic tools are added | Done 2026-11-18 `bf914e9b33`; whole MCP servers done 2026-11-21 |
| A plugin's agents | `<plugin>:<name>`, with the subdirectories of the agent directory in between; `name` defaults to the file's name and `description` to `Agent from <plugin> plugin`; the body may use `${CLAUDE_PLUGIN_ROOT}` and `${user_config.KEY}` (a sensitive option is named, not shown); a user's or a project's agent of the same name wins | The same, as roles | Done 2026-11-19 `4ff3147125` |
| MCP servers of the user | `mcpServers` of `~/.claude.json` | The same | Done 2026-11-17 `80c1279420` |
| MCP servers of a project | The `.mcp.json` files of the working directory and the directories above it, the root of the filesystem left out, the nearer winning; a server there only once approved by `enabledMcpjsonServers` or `enableAllProjectMcpServers` and not listed in `disabledMcpjsonServers`; then the servers `~/.claude.json` keeps for the project, which win; the servers turned off in `/mcp` (`disabledMcpServers` of the project's entry in `~/.claude.json`) left out. The project's entry is that of the repository's main checkout, also in a linked worktree whose `.git` file has the shape `git worktree add` gives it | The same | Done 2026-11-20 `f987126bd2`; linked worktrees done 2026-11-21 |
| Variables in a server's declaration | `${VAR}` and `${VAR:-default}` are expanded in `command`, `args`, `env`, `url` and `headers`; a variable that is not set and has no default stays as written | The same | Done 2026-11-17 `80c1279420` |
| A remote server's `headers` | Sent with every request to the server | The same: they become the server's `http_headers`, in `~/.claude.json`, `.mcp.json` and a Claude Code plugin's servers | Done 2026-11-21 |
| A server's `timeout` | The time limit of each of its tool calls, in milliseconds; a value below 1000 is ignored | The same: it becomes the server's `tool_timeout_sec`. What applies without it is under "Differences from Claude Code" | Done 2026-11-21 |
| `MCP_TIMEOUT`, `MCP_TOOL_TIMEOUT` | How long a server may take to start, and each tool call of a server without its own `timeout`, in milliseconds (read as `parseInt` reads them: an unset, zero or non-numeric value leaves the default); taken from the environment, which the settings files' `env` adds to | The same for the servers of `~/.claude.json`, of `.mcp.json` and of Claude Code plugins: they become a server's `startup_timeout_sec` and `tool_timeout_sec` where its declaration sets neither, the settings files' `env` (a project's once Codex trusts it) winning over Codex's own environment. Codex's own servers keep their configuration | Done 2026-11-21 |
| How the settings files, `~/.claude.json` and `.mcp.json` are read | A leading UTF-8 byte-order mark is ignored; a `.mcp.json` that is a FIFO or a link to a device is not read (2.1.257) | The same: a file that is not a regular file counts as missing, with a warning in the log, and one over 16 MiB is an error | Done 2026-11-21 |

### Auto memory

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| The directory | `~/.claude/projects/<project>/memory/`, `<project>` being the path of the main checkout of the Git repository (or of the working directory) with every character but letters and digits replaced by `-`; a linked worktree's `.git` file leads to the main checkout only when it has the shape `git worktree add` gives it | The same directory, so a memory saved by one tool is recalled by the other. It is created when the session's configuration is loaded | Done 2026-11-17 `34b9401100`; worktree check done 2026-11-21 |
| Moving it | `autoMemoryDirectory` (absolute, or with `~/`) in `~/.claude/settings.json` or the project's `.claude/settings.local.json`, not the project's shared `.claude/settings.json` (newer versions accept that too, in a trusted folder); `CLAUDE_COWORK_MEMORY_PATH_OVERRIDE` names the directory itself; `CLAUDE_CODE_REMOTE_MEMORY_DIR` stands for `~/.claude` in its path | The same, as the reference source has it | Done 2026-11-17 `34b9401100` |
| The instructions | The four types of memory, one file per memory with `name`, `description` and `type` frontmatter, a one-line pointer to it in `MEMORY.md`, what not to save, when to read memory and how far to trust it; `CLAUDE_COWORK_MEMORY_EXTRA_GUIDELINES` is appended | Claude Code's own text, with the directory filled in | Done 2026-11-17 `34b9401100` |
| The index | `MEMORY.md`, cut to 200 lines and 25,000 characters, with a warning line when cut; in newer versions invisible characters and tags that imitate Claude Code's own markup are neutralized (2.1.284) | The same, counted in bytes; zero-width and bidirectional control characters are removed, and the `<` of a tag of Codex's or Claude Code's own wrappers (`</INSTRUCTIONS>`, `<environment_context>`, `<system-reminder>`) becomes `&lt;` | Done 2026-11-21 |
| Turning it off | `CLAUDE_CODE_DISABLE_AUTO_MEMORY` is `1` or `true` (`0` or `false` turns it on whatever the settings say); `CLAUDE_CODE_SIMPLE` is set; Claude Code on the web without `CLAUDE_CODE_REMOTE_MEMORY_DIR`; the last of the settings files to set `autoMemoryEnabled` sets it to `false` | The same. The variables are taken from the `env` of the settings files first, then from Codex's own environment. When it is off, neither the instructions nor the index are shown and nothing is made writable | Done 2026-11-17 `34b9401100` |

### Plugins

| Feature | Claude Code | Equivalent in Codex | Status |
| --- | --- | --- | --- |
| Which plugins are installed | `~/.claude/plugins/installed_plugins.json`; a plugin is read from the `installPath` of the first installation recorded for it, whatever its scope | The same | Done 2026-11-18 `471d20cf39` |
| Which are enabled | `enabledPlugins` of `~/.claude/settings.json`, the project's `.claude/settings.json` and `.claude/settings.local.json`, merged in that order, a later file's value for a `<plugin>@<marketplace>` id winning; only `true` enables | The same. An enabled id that is not installed is reported at startup | Done 2026-11-18 `471d20cf39` |
| What a plugin brings | Skills as `<plugin>:<skill>`, commands, agents, MCP servers, hooks | The same components, loaded by Codex's plugin loader from the install path | Done 2026-11-19 `471d20cf39`, `c58a3409c6`, `3134f1b61a`, `4ff3147125` |
| `${CLAUDE_PLUGIN_ROOT}` in hook commands, MCP server declarations and agent bodies | The plugin's install path | The same | Done 2026-11-18 `471d20cf39` |
| `userConfig`: where values are kept | The plain ones under `pluginConfigs.<plugin id>.options` of the settings, the ones marked `sensitive` in the secure store | The plain ones from `~/.claude/settings.json`, the sensitive ones from `pluginSecrets.<plugin id>` of `~/.claude/.credentials.json` | Done 2026-11-19 `4ff3147125` |
| `userConfig`: `${user_config.KEY}` | Filled in in MCP server declarations and agent bodies, and in the reference source in hook commands (newer versions refuse it in a hook's shell command and want `CLAUDE_PLUGIN_OPTION_<KEY>` there) | Filled in in all three | Done 2026-11-19 `4ff3147125` |
| `userConfig`: `CLAUDE_PLUGIN_OPTION_<KEY>` | In the environment of the plugin's hooks, the key in upper case with anything but letters, digits and `_` turned into `_` | The same | Done 2026-11-19 `4ff3147125` |
| The manifest's `agents` list | Stands in place of `agents/` | The same, for the directories it lists | Done 2026-11-20 `f987126bd2` |
| A plugin skill whose frontmatter `name` already starts with `<plugin>:` | Newer versions (2.1.246) do not add the prefix again: `name: acme:review` in plugin `acme` is `/acme:review` | The same | Done 2026-11-21 |
| The manifest's `skills` list | Adds to the `skills/` folder, which is still scanned (newer versions, 2.1.136, stopped it hiding `skills/`) | The same | Done 2026-11-21 |
| A `plugin.json` that starts with a UTF-8 byte order mark | Read as if it had none (newer versions, 2.1.246, also when installing) | The same | Done 2026-11-21 |
| A project's settings and the plugins they enable | Count once the folder is trusted | Count in a project Codex trusts; elsewhere they can turn a plugin off and not on | Done 2026-11-20 `f987126bd2` |

## Not yet implemented

What Claude Code has and this fork does not have yet. Nothing here contradicts how Codex
works: each could be added. The last column says what happens today and names the nearest
thing Codex has, where there is one.

### Instructions and rules

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Managed instructions: `CLAUDE.md` and `.claude/rules` under `/etc/claude-code` (`/Library/Application Support/ClaudeCode` on macOS) | Loaded first; cannot be excluded | Not read. Nearest: Codex's own managed configuration | Not planned |
| `claudeMdExcludes` | Globs of instruction files to leave out | Not read: every file found is loaded | Not planned |
| The instruction files of directories added with `--add-dir` (`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`) | Loaded | Not loaded | Not planned |
| A warning for a large instruction file (over 40,000 characters) | Shown at startup; newer versions scale the threshold with the model's context window (2.1.169) | Only the global instructions are measured, against `project_doc_max_bytes` | Not planned |
| `/memory` | Lists the instruction and memory files and opens them; turns auto memory on and off | Absent. Nearest: Codex's `/memories`, which is about Codex's own memories | Not planned |
| `/init` | Writes a `CLAUDE.md` for the project | Codex's `/init` writes `AGENTS.md` | Not planned |
| The `InstructionsLoaded` hook | Runs for each instruction file loaded | Not run (see "Hooks") | Not planned |

### Skills and slash commands

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Shell injection (```` ```! ```` blocks and inline ``!`command` ``) | The commands run, each through the permission check of `Bash`, and their output replaces them; a command that is denied or fails stops the skill | Not run: Codex puts the skill into the conversation before the turn starts, where no command runs under the session's sandbox and approval rules. The text stays as written, so the model sees the commands and can run them itself, with the usual approvals | Not planned |
| `allowed-tools`, `model`, `effort`, `context`, `agent`, `argument-hint`, `shell` in the frontmatter; `disallowed-tools` and `background` (newer versions) | `allowed-tools` allows those tools without asking while the skill's turn runs; `context: fork` runs the skill in a subagent of type `agent`, with `model` and `effort`; `argument-hint` shows while typing; `disallowed-tools` takes tools away while the skill is active (2.1.152); a `context: fork` skill runs in the background unless `background: false` (2.1.218) | Ignored. Nearest: a skill's `agents/openai.yaml`, which holds Codex's own settings for a skill | Not planned |
| `hooks` in a skill's frontmatter, and `once` on them | Registered when the skill is invoked and kept for the rest of the session; `once` takes a hook away after its first success | Not run. Nearest: the hooks of the settings files, which run for the whole session | Not planned |
| `user-invocable: false` | Hides the skill from the slash menu and refuses `/<name>`; the model can still use it | The skill stays in the slash popup and `/<name>` runs it | Not planned |
| `/<name> <args>` outside the terminal interface | Also in `claude -p`, for the skills a user may invoke | A TUI feature. `codex exec` and other clients attach a skill with `$<name>`, and get the expansion only when they send the text together with the skill as a structured skill input, as the TUI does | Not planned |
| A plugin's `commands` entry that names a single file | Read | Not read; directories are | Not planned |
| `paths` in a skill's frontmatter | The skill stays out of the list until a file its globs cover is worked on | Ignored: the skill is always listed | Not planned |
| `.claude/skills` of directories below the working directory | Found when a file there is read or edited; newer versions (2.1.178) call one whose name clashes with another `<dir>:<name>` | Not found. Nearest: the roots from the project root down to the working directory | Not planned |
| Managed skills, and the skills of directories added with `--add-dir` | Loaded | Not loaded | Not planned |
| `@<file>` in a skill's text | The file is attached when the skill is invoked | The text stays as written | Not planned |
| Skills across a compaction | The skills invoked are put back, 5,000 tokens each | Codex's own compaction, which knows nothing of skills | Not planned |
| `skillOverrides`, `disableSkillShellExecution`, `disableBundledSkills` and `CLAUDE_CODE_DISABLE_BUNDLED_SKILLS` (newer versions) | Applied | Not read. Nearest: `[[skills.config]]` in `config.toml` to turn a skill off | Not planned |
| `${CLAUDE_EFFORT}` in a skill's text, and built-in commands through the `Skill` tool | Newer versions fill in the session's effort level (2.1.120), and let the model find and run built-in commands such as `/init` and `/security-review` through `Skill` (2.1.108) | The text stays as written; `Skill` offers only the skills Codex lists | Not planned |

### Hooks

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| The events Codex does not have: `PermissionDenied`, `Setup`, `TeammateIdle`, `TaskCreated`, `TaskCompleted`, `Elicitation`, `ElicitationResult`, `ConfigChange`, `InstructionsLoaded`, `WorktreeCreate`, `WorktreeRemove`, `CwdChanged`, `FileChanged`, and those of newer versions: `MessageDisplay` (2.1.152), `DirectoryAdded` (2.1.219), `PreModelSwitch` and `PostModelSwitch` (2.1.251), `PostToolBatch` (by 2.1.328) | Run | Ignored. Codex has events of its own that Claude Code lacks (`Interrupt`, and two around each request to the model), which the settings files cannot name | Not planned |
| `Notification` of type `auth_success`, `elicitation_complete`, `elicitation_response`, and of newer versions `agent_completed` (2.1.198) | Sent | Never sent. Nearest: Codex's own `notify` program in `config.toml`, which runs when a turn ends | Not planned |
| `Notification`, `PostToolUseFailure`, `StopFailure`: handler types | `command` and `http` for `Notification` and `StopFailure`, any for `PostToolUseFailure` | Command handlers only, and these hooks are not listed among Codex's hooks (`/hooks`) | Not planned |
| The same three events from a plugin's hooks | Run | Do not run; only the settings files' do | Not planned |
| `prompt` and `agent` handlers for `PostToolUseFailure`, `PermissionDenied`, `TaskCreated`, `TaskCompleted` and `TeammateIdle` | Run | Not run: they run for `PreToolUse`, `PostToolUse`, `PermissionRequest`, `UserPromptSubmit`, `Stop` and `SubagentStop` | Not planned |
| The conversation, for a `prompt` handler of `Stop` or `SubagentStop` | Put before the prompt | The hook input only, which has `last_assistant_message`. An `agent` hook reads the transcript file in both | Not planned |
| `transcript_path` in Claude Code's format | One JSON message a line | Codex's rollout file, whose lines are Codex's own records: a hook that parses the transcript finds none of the messages it looks for. Nearest: `last_assistant_message` in the input of `Stop` | Not planned |
| `prompt` and `agent` handlers, and `if`, written into a plugin's manifest rather than a hooks file, or into Codex's own `hooks.json` | Run (the manifest) | Not read there | Not planned |
| Hook output Codex refuses: `continue: false` and `stopReason` from `PreToolUse` and `PermissionRequest`; `updatedInput`, `updatedPermissions` and `interrupt` from `PermissionRequest`; `updatedMCPToolOutput` from `PostToolUse`; `updatedToolOutput` from `PostToolUse`, which newer versions take for every tool (2.1.121) | Honoured | The hook's output is reported as invalid and has no effect, save a block or a denial given with it (see "Differences"). Nearest: exit code 2, and `deny` or `allow` | Not planned |
| `watchPaths` and `initialUserMessage` from `SessionStart`; `additionalContext` from `SubagentStart` and `Notification`; of newer versions, `reloadSkills` (2.1.152) and `sessionTitle` (2.1.152) from `SessionStart`, `sessionTitle` from `UserPromptSubmit` (2.1.94), `terminalSequence` from any hook (2.1.141), `additionalContext` from `Stop` and `SubagentStop` (2.1.163), and a `PreCompact` hook's block, by exit code 2 or `decision: "block"` (2.1.105) | Honoured | Ignored | Not planned |
| `asyncRewake` | The hook runs in the background and its exit code 2 wakes the model | Ignored: the hook runs as an ordinary one. Nearest: `async` | Not planned |
| `shell` on a command handler (`bash`, `powershell`) | Chooses the shell | Ignored: Codex's shell runs it | Not planned |
| Hooks of managed settings, `allowManagedHooksOnly` | Managed hooks run whatever the user's files say; the setting leaves only them | Not read | Not planned |
| `CLAUDE_CODE_SIMPLE` | No hook runs | Read for auto memory only | Not planned |
| `compact_summary` for `PostCompact`, `agent_type` for `SessionStart`; what `permission_suggestions` of `PermissionRequest` suggests; of newer versions, `effort.level` (2.1.133, with `$CLAUDE_EFFORT` in the hook's environment), `duration_ms` for `PostToolUse` and `PostToolUseFailure` (2.1.119), `background_tasks` and `session_crons` for `Stop` and `SubagentStop` (2.1.145), and the staleness and re-cache cost of a resumed session for `SessionStart` (2.1.251) | Present in the input | `compact_summary`, `agent_type` and the fields of newer versions are absent; `permission_suggestions` is there and always empty, Codex having no rule updates to suggest | Not planned |
| Hooks in a subagent's frontmatter; hooks registered by an SDK | Run | Not run | Not planned |
| `{"async": true}` as a command handler's first line of output | The hook goes on in the background from there | Read as the hook's output, and invalid. Nearest: `async` on the handler | Not planned |
| `onFailure` on `command`, `http` and `mcp_tool` handlers | Newer versions: `"block"` makes a handler that fails, times out or cannot start block the action, `"continue"` lets it go ahead (`command` and `http` from 2.1.295, `mcp_tool` from 2.1.328) | Not read: a failing handler never blocks. Nearest: exit code 2 from a handler that runs | Not planned |
| `args` (a command run without a shell) and `continueOnBlock` | Newer versions (2.1.139): `args` starts the program with these arguments, so a path needs no quoting; `continueOnBlock: true` gives Claude a `PostToolUse` hook's reason for blocking and the turn goes on | Not read: a command runs through the shell, and a blocking `PostToolUse` hook ends the call as without it | Not planned |
| `"defer"` as a `PreToolUse` decision, and `CLAUDE_CODE_DISABLE_DEFERRED_TOOL_RESUME` | Newer versions (by 2.1.328): in `claude -p` the turn ends so that a host answers the call later and resumes with `--resume`; the variable keeps the resume from running the deferred call | Absent: a decision is `allow`, `deny` or `ask`. Nearest: `codex exec resume` | Not planned |
| Limits on `prompt` and `agent` handlers | Newer versions: at most 24 at a time in a session, subagents included, and 12 more that waited past their event's timeout (2.1.302); a `timeout` under a second counts as one (2.1.312) | No cap: an event's handlers run together. Only async command hooks are limited, to 8 | Not planned |

### Claude tool set

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Tools beyond the twelve offered | `NotebookEdit`, `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `TaskStop`, `SendMessage`, `EnterPlanMode`, `ExitPlanMode`, `EnterWorktree`, `ExitWorktree`, `ToolSearch`, `LSP`, `ListMcpResourcesTool`, `ReadMcpResourceTool`, `CronCreate`, `CronList`, `CronDelete`, `Monitor`, `PowerShell`, and for agent teams `TeamCreate` and `TeamDelete`, and of newer versions `EndConversation` (2.1.214) and `ListAgents` (2.1.224) | Not offered. Nearest: `update_plan` for the task tools, `write_stdin` for a background command, Codex's own agent tools (`wait_agent`, `send_input`, `close_agent`, or v2's `send_message`, `followup_task`, `list_agents`), Codex's plan mode, its tool search and its MCP resource tools | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): `Monitor`, `CronCreate`, `CronList` and `CronDelete` done 2026-11-20 `aaa300e67a` |
| `Bash`: a background command | Writes to an output file named in the result, which the model reads with `Read` (`TaskOutput` until it was removed in 2.1.277), and is stopped with `TaskStop`; the model is notified when it ends | Read and stopped with `write_stdin`; the model is not notified when it ends | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): the model is told when it ends, done 2026-11-19 `6363eace6b` |
| `Write`: a file that is not UTF-8 | Overwritten, as UTF-8 | Cannot be overwritten, since `apply_patch` takes text | Not planned |
| Long results | Saved to a file past the tool's limit (30,000 characters for `Bash`, 20,000 for `Grep`, up to 50,000 for the others); the model gets the path and the first 2 KB; newer versions cap the saved file at 1 GB (2.1.265), and `bashOutputMaxChars` and `taskOutputMaxChars` (up to 128K characters, 2.1.261) move the limit | Truncated as any tool output in Codex's history; output past Codex's capture of 1 MiB loses its middle before `Glob`, `Grep` or `WebFetch` see it | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): a long command output is kept in a file, done 2026-11-20 `f1627b6dc9` |
| `Glob`, `Grep`: files denied to `Read` | Left out of the results | Not left out; reading them is still denied | Not planned |
| `Read`, `Edit`, `Glob`, `Grep`: a path that does not exist | A similar path is suggested (`Did you mean ...?`) | No suggestion | Not planned |
| `WebFetch`: content that is not text | Decoded as UTF-8 for the secondary model like any other, and also saved to a file the result names | Decoded as UTF-8; nothing is saved | Not planned |
| `WebFetch`: preapproved documentation hosts | Fetched without asking | Never allow a request by themselves. Nearest: a `WebFetch(domain:...)` allow rule, or the managed network proxy's allowed hosts | Not planned |
| `TodoWrite`: clearing and `oldTodos` | A list whose items are all completed is cleared; the hook's `tool_response` has `oldTodos` and `newTodos` | The list is kept; `newTodos` only | Not planned |
| `AskUserQuestion`: `multiSelect` and option `preview`s | Several choices can be ticked; previews are shown | Codex's panel takes one choice a question: the question asks for the other choices in a note. Previews are not offered | Not planned |
| `AskUserQuestion` outside Plan mode | Asks in every permission mode | Only where Codex's `request_user_input` does: in Plan mode, and in Default mode with the `default_mode_request_user_input` feature; elsewhere the model gets Codex's refusal | Not planned |
| `Agent`: `isolation`, `name`, `team_name`, `mode`, and `effort` (newer versions, 2.1.292) | Taken; newer versions ignore `mode` (2.1.212) | Not taken. Nearest: Codex's roles and its own agent tools | Not planned |
| `Agent`: continuing an agent with `SendMessage` | Done | Codex's own agent tools (`send_input`, or v2's `send_message` and `followup_task`) | Not planned |
| `Agent`: a background agent that ends | The model is notified and goes on | Its result is added to the conversation without starting a turn; the model is told to use `wait_agent` to wait for it | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): the session goes on when it ends, done 2026-11-19 `6363eace6b` |
| `Agent(type, ...)` in the tool list of the session's own agent (`--agent`) | Restricts the agent types it may start | No session runs as a subagent, so nothing is restricted | Not planned |
| Auto-review (guardian) of past tool calls | Not applicable | Recognises only Codex's tool names, so calls of the Claude tools are not reviewed as their Codex counterparts are | Not planned |
| `Bash`: a command that reaches its timeout | Newer versions move it to the background | It is stopped and the model is told `Command timed out` | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): moved to the background, done 2026-11-20 `f1627b6dc9` |
| `BASH_DEFAULT_TIMEOUT_MS`, `BASH_MAX_TIMEOUT_MS`, `BASH_MAX_OUTPUT_LENGTH`, `CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS`, `CLAUDE_CODE_GLOB_TIMEOUT_SECONDS`, and of newer versions `bashOutputMaxChars` and `taskOutputMaxChars` (2.1.261), `CLAUDE_CODE_TOOL_MEMORY_LIMIT` and `CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS` (2.1.233), `CLAUDE_CODE_DISABLE_WEB_FETCH` (2.1.285), `CLAUDE_CODE_WEB_SEARCH_REFILLS_PER_HOUR` (2.1.290), and the other variables that move a tool's limits | Read | Not read: the limits are fixed | Not planned |
| `Edit` on a notebook | Refused, in favour of `NotebookEdit` | The notebook's JSON is edited | Not planned |
| `WebFetch`: `offset`, to read on in a long page (newer versions) | Taken | No such parameter | Not planned |
| `WebFetch(domain:...)` rules with `*` (newer versions) | `*.example.com` covers the subdomains | The host is compared as a whole name | Not planned |
| `Agent`: the built-in types `Explore`, `Plan` and the others | Offered beside `general-purpose` | `general-purpose` is Codex's default role; the others are Codex's own roles, if any | Not planned |
| `Read`: `allow_large` | Newer versions (2.1.296): the model reads a text file past the usual limits in one call when the context has room | Not offered: the 25,000-token limit always holds | Not planned |
| `Agent`: how the result reaches the model | Newer versions (2.1.277) put it under a header that marks it as a subagent's output, indented, so that its text cannot pass for the session's own instructions | The agent's final message, without a header | Not planned |
| `Bash`: a diff of the files a command changed (`bashEditDiffEnabled`) | Newer versions (2.1.269) add it to the result | Not read: the result has the command's output only. Nearest: the turn's diff in Codex's interface | Not planned |
| Session caps: `CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS` (20), `CLAUDE_CODE_MAX_SUBAGENTS_PER_SESSION` (200), `CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION` (200) | Newer versions (2.1.212, 2.1.217) refuse a call past the cap | Not read. Nearest: `[agents] max_concurrent_threads_per_session` and `max_depth` in `config.toml`; none for searches | Not planned |
| Code Mode (`CLAUDE_CODE_EXPERIMENTAL_CODE_MODE=1`) | Newer versions (2.1.323, an experiment): the model writes JavaScript in a `REPL` tool that calls the other tools in one step, each call under its own rules and `PreToolUse` hooks | Absent. Nearest: Codex's own code mode (the feature `code_mode`), which calls Codex's tools | Not planned |
| Messages between sessions: `SendMessage` to another session on the machine or the account, `ListAgents`, `@<name>` in the prompt, `crossSessionInbound`, `dialogExpiry` | Newer versions (2.1.224): sessions message each other; a session that runs without permission prompts holds incoming messages for approval | Absent. Nearest: v2's `send_message` and `list_agents`, which reach the agents of one session only | Not planned |

### Permission rules and settings

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Managed settings (`/etc/claude-code/managed-settings.json` and `managed-settings.d/`, the system's device management), which win over every other file; `allowManagedPermissionRulesOnly`, `strictPluginOnlyCustomization`, the keys of newer versions (`claudeMd`, `policyHelper`, `managedSourcesBehavior`, `enforceAvailableModels`, `requiredMinimumVersion`, `requiredMaximumVersion`, `pluginSuggestionMarketplaces`, `prependPlugins`, `appendPlugins`, `allowManagedModsOnly`) and the other keys that only managed settings may set | Applied | Not read. Nearest: Codex's own requirements and managed configuration | Not planned |
| A change to `env`, `effortLevel`, the enabled plugins, the MCP approvals or the auto memory keys of the settings files during a session, and to Bash rules while the Claude tool set is off | Applied when the file changes (`effortLevel` and `model` only at the next session) | Read once a session; hooks and the Claude tool set's permission rules are read again (see "Differences from Claude Code") | Not planned |
| `permissions` for tools other than `Bash`, with the Claude tool set off | Applied | Not applied: only command-prefix rules of the user's file become Codex command rules. Turn the tool set on for the rest | Not planned |
| Settings keys other than the ones this document names | Applied | Not read. Nearest: the key of the same purpose in `config.toml`, where Codex has one | Not planned |
| Rules and settings given on the command line (`--allowedTools`, `--disallowedTools`, `--settings`, `--setting-sources`) | Applied | No such options. Nearest: `-c key=value` and `--profile` for Codex's own settings | Not planned |
| "Don't ask again" for a call | Writes an allow rule into `.claude/settings.local.json` | Codex's own "approve for the session"; nothing is written to Claude Code's files | Not planned |
| Exec wrappers and `find` actions under allow rules (newer versions) | Newer versions (2.1.113): a prefix rule such as `Bash(watch *)` does not approve an exec wrapper (`watch`, `setsid`, `ionice`, `flock`), and `Bash(find *)` does not approve `find -exec`, `-delete` and the other actions that run, delete or write | Such a rule approves them as any other command | Not planned |
| Protected files beyond the reference source's list (`.husky`, `.cargo`, `.devcontainer`, `.npmrc`, `.envrc`, `.zshenv`, `.zlogin`, `.bash_login`, `~/.config/git/` and others, in newer versions) | Asked about | Treated like any other file | Not planned |
| `Tool(param:value)` rules, and a glob as the tool's name in a `deny` rule | Newer versions match a tool's parameter, with `*` as a wildcard (`Agent(model:opus)`, 2.1.178), and deny every tool a glob names (`"*"`, 2.1.166) | Not read: a rule is a tool's name with the content this document lists | Not planned |
| `permissions.denyInsteadOfPrompting`, `--permission-prompts none` | Newer versions (2.1.259, 2.1.319): a call that would wait for a permission prompt is denied and the turn goes on | Not read. Nearest: `approval_policy = "never"`, which refuses whatever it would ask, by policy rather than by rule | Not planned |
| `permissions.blockWritesOutsideWorkingDirectories`, `permissions.blockReadsOutsideWorkingDirectories` | Newer versions: `Write`, `Edit` and `NotebookEdit` refuse paths outside the working directories in every mode (2.1.314); commands that read there ask, in bypass and auto mode too (2.1.257, 2.1.327) | Not read. Nearest: the `workspace-write` sandbox for writes | Not planned |
| Warnings about rules | Newer versions warn at startup about `Write(path)`, `NotebookEdit(path)` and `Glob(path)` rules (2.1.210) and a `Bash` allow rule with a wildcard before the subcommand (`Bash(git * main)`, 2.1.246), and report a rule with text after its closing parenthesis as invalid (2.1.260) | No warning: such a rule is read as written | Not planned |
| `--restricted` and `CLAUDE_CODE_RESTRICTED=1` | Newer versions (2.1.248): the tools that run commands or code and `WebFetch` are taken away, the file tools stay in the working directory, and the user's and the project's settings files are ignored | No such option. Nearest: the `read-only` sandbox and `codex exec --ignore-user-config` | Not planned |

### Subagents

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Frontmatter other than `name`, `description`, `effort`, `tools` and `disallowedTools`: `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `background`, `isolation`, `color`, and of newer versions `omitClaudeMd` (2.1.271) and `autoCompactWindow` (2.1.296) | Used | Left out. Nearest: a Codex role's own configuration file | Not planned |
| `tools` and `disallowedTools` with the Claude tool set off | Applied | Ignored: the names are Claude Code's, which only the tool set gives the tools | Not planned |
| Hosted tools and the tools code mode and tool search add, under `tools` | Not applicable | Not filtered | Not planned |
| A project's subagents in directories above the working directory | Found, up to the repository's root, the nearest winning | Only `.claude/agents` of the working directory is read | Not planned |
| A plugin's `agents` entry that names a single file | Read | Not read; directories are | Not planned |
| `${CLAUDE_PLUGIN_DATA}` in a plugin agent's body | Filled in | Left as written | Not planned |
| A subagent's own memory (`memory: user`, `project` or `local`) | A directory of its own (`~/.claude/agent-memory/<name>/` and the like), shown to that subagent | Not kept | Not planned |
| Subagents given with `--agents`, managed subagents, those of directories added with `--add-dir` | Loaded | Not loaded | Not planned |
| A session that runs as a subagent (`--agent`, the `agent` setting) | The whole session takes the subagent's prompt, tools and model | No such option. Nearest: a Codex profile | Not planned |
| Agent teams (`TeamCreate`, teammates, `team_name`) | An experiment; newer versions (2.1.178) give every session one team, without `TeamCreate` and `TeamDelete`, and start teammates with the `Agent` tool's `name` | Absent. Nearest: Codex's own multi-agent tools | Not planned |
| Forked subagents (`CLAUDE_CODE_FORK_SUBAGENT`) | Newer versions (2.1.117, 2.1.121): a subagent starts from a copy of the parent's conversation; `0` turns it off (2.1.328) | Absent. Nearest: Codex's `/fork`, which forks the session, not a subagent | Not planned |
| `CLAUDE_CODE_SUBAGENT_EFFORT` | Newer versions (2.1.316): the effort of a subagent that names none | Not read: the agent's own `effort`, or else its role's | Not planned |

### MCP servers

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| `sse` and `ws` servers | Started (`sse` is deprecated) | Do not start: Codex speaks stdio and streamable HTTP | Not planned |
| MCP bundles (`.mcpb`) of a plugin | Used | Not loaded; the plugin is still used | Not planned |
| `managed-mcp.json`, `allowedMcpServers`, `deniedMcpServers`, `managedMcpServers` (newer versions, 2.1.259) | Decide which servers may run | Not read | Not planned |
| `--mcp-config`, `--strict-mcp-config` | Servers for one run | No such options. Nearest: `-c mcp_servers.<name>...` | Not planned |
| `MAX_MCP_OUTPUT_TOKENS`, and of newer versions `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` (2.1.274) and `CLAUDE_CODE_MAX_MCP_DESCRIPTION_LENGTH` (2.1.280) | Read | Not read. Nearest: `startup_timeout_sec` and `tool_timeout_sec` of a server in `config.toml` | Not planned |
| A server's prompts as `/mcp__<server>__<prompt>`, its resources as `@<server>:<uri>` | Offered | Whatever Codex's own MCP client offers; these forms are not added | Not planned |
| MCP calls in the background: tasks, calls past 2 minutes, `CLAUDE_CODE_MCP_AUTO_BACKGROUND_MS`, `CLAUDE_CODE_DISABLE_MCP_TASK_BACKGROUND` | Newer versions move a long call (2.1.212) or a server's task (2.1.319) to the background and tell the model its result when it ends | The call is waited for, up to `tool_timeout_sec` | Not planned |
| URL elicitations of newer protocol versions, and `bareElicitationCapability` | Newer versions (2.1.281, 2.1.287): a server may ask for a browser flow, and the call waits until it is done | Codex's own MCP client answers elicitations its own way; the key is not read | Not planned |
| `/mcp reconnect all`, signing in again for a wider OAuth scope, `claude mcp login` and `logout`, `claude mcp add-json <name> -` | Newer versions (2.1.186 to 2.1.327) | Absent for Claude Code's servers. Nearest: `codex mcp login`, `logout` and `add` for Codex's own | Not planned |
| What a server is given: the session's added directories as `roots/list`, `CLAUDE_CODE_SESSION_ID`, `CLAUDECODE=1` and `CLAUDE_PROJECT_DIR` in a stdio server's environment | Newer versions (2.1.139 to 2.1.203) | Not given: the roots are Codex's own, and the variables are set for hooks only | Not planned |
| `alwaysLoad`, `_meta["anthropic/maxResultSizeChars"]`, the reserved name `workspace` | Newer versions: a server's tools stay out of tool search's deferral (2.1.121); a tool's results of up to 500K characters pass whole (2.1.91); a server named `workspace` is skipped (2.1.128) | Not read; no name is reserved | Not planned |

### Auto memory

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Background extraction of memories at the end of a turn | In some builds, behind a switch | Not done: memories are written only when the model writes them during a turn. Nearest: Codex's own memories feature, which keeps its files elsewhere | Not planned |
| Team memory, the daily logs of assistant mode, the search of past transcripts some builds add to the instructions | Present in some builds | Absent | Not planned |
| `CLAUDE_CONFIG_DIR` | Moves the configuration directory, and the memory with it | Not read: Codex always reads `~/.claude` | Not planned |
| The memory directory after the permissions change during a session | Still writable | `/permissions` drops the extra writable root; add the directory to `[sandbox_workspace_write] writable_roots` to keep it | Not planned |

### Plugins

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| A change to the installed or enabled plugins | Shows once the plugins are reloaded | Shows at the start of the next session | Not planned |
| A value for a `userConfig` option that was never set | Asked for in `/plugin`, when the plugin is installed, enabled or configured; the manifest's `default` is used when the user gives nothing, and an option not marked `required` that is left blank reads as empty (2.1.119) | Cannot be asked for. The manifest's `default` is used, and an option not marked `required` reads as empty (done 2026-11-21). A hook whose command names a required option without a value is skipped with a warning, and an MCP server's string that names it is left as written, with a warning in the log. Set the option in Claude Code | Not planned |
| `sensitive` `userConfig` values on macOS | Read from the keychain | Not read: a sensitive option has no value there. On other systems they are read from `~/.claude/.credentials.json` | Not planned |
| `userConfig` values in a plugin's skills and commands | Filled in | Not filled in | Not planned |
| LSP servers of a plugin (`lspServers`, `.lsp.json`) | Started | The plugin is disabled whole (see "Which plugins are used") | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): planned |
| A plugin without a `plugin.json`, whose marketplace entry is its manifest; a marketplace entry that adds components to the `plugin.json` | Loaded | Not read: only `plugin.json` at the install path is | Not planned |
| Managed `enabledPlugins` (`false` there blocks a plugin the user enables), plugins from `--add-dir` and `--plugin-dir`, Claude Code's built-in plugins | Applied | Not read: a plugin the managed settings block is still used | Not planned |
| A path the manifest declares for `hooks` or `mcpServers` | Loaded beside `hooks/hooks.json` or `.mcp.json` | Replaces it. An `mcpServers` list mixing paths and objects is not read | Not planned |
| The plugin's `settings.json` | Its `agent` key is kept, which makes the session run as that agent | Not loaded | Not planned |
| `${CLAUDE_PLUGIN_ROOT}` in a plugin's skills and commands, and in the environment of its MCP servers | Filled in | Not filled in | Not planned |
| A plugin's `dependencies` | A plugin whose dependency is missing is not loaded; newer versions also install a missing one from the marketplaces added (2.1.116), enable them along with the plugin and refuse to disable one another plugin needs (2.1.143), and remove orphans with `claude plugin prune` (2.1.121) | Not checked | Not planned |
| Marketplace policy (`strictKnownMarketplaces`, `blockedMarketplaces`; owner wildcards `"owner/*"`, 2.1.223, and the names `additionalMarketplaces` and `allowedMarketplaces`, 2.1.232, in newer versions) | A plugin of a marketplace that is not allowed does not load | Not read | Not planned |
| The manifest's `commands` as a map of names to files or inline text | Loaded | Only directories are read | Not planned |
| A plugin's `bin/` on the path of commands, its monitors, workflows and themes (newer versions) | Used | Not loaded | Not planned |
| Mods (newer versions, 2.1.287 to 2.1.330): a plugin's JavaScript or TypeScript module, and the built-in mods such as the chart mod easel | Code that runs inside Claude Code through its `$` interface: it steps into prompts, turns, tool calls (`tool.check`), agents and requests, registers tools and commands, and draws panes and bands; managed settings may allow only managed mods (`allowManagedModsOnly`) | Not loaded, without a notice; the rest of the plugin is used. The parts that step into prompts and tool calls could be mapped to this fork's hooks; the interface has no counterpart | Not planned |
| `engines["claude-code"]` in `plugin.json` | Newer versions (2.1.323): a plugin outside the version range it names is not loaded | Not read | Not planned |
| A plugin whose root is its skill: a root `SKILL.md` without `skills/`, `"skills": "./"` or `"."` | Newer versions (2.1.94, 2.1.142, 2.1.221) load it, under its frontmatter's `name` | Not loaded: Codex ignores a `skills` path of `./`, and looks for skills under `skills/` only | Not planned |
| Marketplace sources of newer versions: `archive` (a zip over HTTPS, 2.1.224), `command` (2.1.229), `headersHelper` (2.1.238), GitLab (2.1.232), `skipLfs` (2.1.153); `defaultEnabled: false` (2.1.154); plugins in a `.claude/skills` folder (2.1.157); a folder of plugins as `--plugin-dir` (2.1.265) | Install and load plugins from these | Not read: only the plugins Claude Code installed are used | Not planned |

### Beyond the files this fork reads

Claude Code features that have no part in the configuration this fork reads. Codex often has
a feature of its own for the purpose, configured in its own way.

| Feature | Claude Code | Codex today, and the nearest thing it has | Status |
| --- | --- | --- | --- |
| Output styles (`outputStyle`, `~/.claude/output-styles/`) | Replace parts of the system prompt; newer versions add the built-in style `Concise` (2.1.237) | Not read. Nearest: `personality`, `developer_instructions` and `model_instructions_file` in `config.toml` | Not planned |
| `alwaysThinkingEnabled`, `MAX_THINKING_TOKENS`, `CLAUDE_CODE_DISABLE_ULTRATHINK` (newer versions, 2.1.316) | Set how much the model thinks | Not read. Nearest: `model_reasoning_effort`, which `effortLevel` sets (see "Differences from Claude Code") | Not planned |
| `language` | The language the model answers in | Not read; `ui.language` of `ccn-config.toml` is the language of the interface | Not planned |
| `attribution`, `includeCoAuthoredBy`, `includeGitInstructions` | Decide the attribution lines and the Git instructions in the prompt; newer versions take `"attribution": false` (2.1.281) | Not read. The `Bash` description here carries the commit instructions without attribution lines | Not planned |
| `~/.claude/keybindings.json`, `editorMode`, and of newer versions `keybindingFlavor` (2.1.238), vim's visual modes (2.1.118) and the actions `selection:clear` (2.1.234) and `effortSlider:*` (2.1.284) | Key bindings and vim mode | Not read. Nearest: `[tui.keymap]` and Codex's `/keymap` and `/vim` | Not planned |
| Sessions: transcripts under `~/.claude/projects/`, `--continue`, `--resume`, `--fork-session`, `cleanupPeriodDays`, and of newer versions `claude purge` (2.1.126 as `claude project purge`, renamed in 2.1.288) and `CLAUDE_CODE_PROJECT_DIR_NAME` (2.1.234) | Kept, resumed and cleaned up | A Claude Code session cannot be resumed here, nor the reverse. Nearest: `codex resume` and `codex fork` for Codex's own sessions | Not planned |
| Checkpoints and `/rewind` | Put files and conversation back to an earlier prompt; newer versions also go back to before a `/clear` (2.1.191) | Codex goes back in the conversation only | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): going back to a prompt can put the files back, done 2026-11-20 `39f6a93cc3` |
| `/compact <instructions>`, the automatic compaction settings, and of newer versions idle compaction (`idleCompaction`, `CLAUDE_CODE_IDLE_COMPACT_MIN_TOKENS`, 2.1.299) and `/autocompact` (2.1.288) | Compaction on request with a focus, and by itself | Codex's own `/compact` and `model_auto_compact_token_limit` | Not planned |
| Built-in commands without a Codex counterpart (`/security-review`, `/context`, `/cost`, `/add-dir`, `/doctor` and others; of newer versions `/code-review`, `/cd`, `/reload-skills`, `/subtask`, `/checkup`, `/skill-doctor`, `/doctor prompt-audit`, `/team-onboarding`, `/powerup`, `/scroll-speed`, `/focus`, `/usage`, `/list-agents`) | Present | Codex's own commands (`/review`, `/status`, `/diff` and the rest); a missing one can be written as a skill | Not planned |
| Worktrees (`--worktree`, `.worktreeinclude`, `worktree.*`, with `worktree.baseRef`, 2.1.133, in newer versions) | A session in a worktree of its own | Not read. Nearest: Codex's `/worktree` | Not planned |
| `claude -p` and its options (`--output-format`, `--input-format`, `--json-schema`, `--max-turns`, `--system-prompt`, `--append-system-prompt`; of newer versions `--exclude-dynamic-system-prompt-sections` (2.1.98), `--forward-subagent-text` (2.1.212), `--append-subagent-system-prompt-file` (2.1.261), `--system-prompt-snapshot` (2.1.267)) | A run without a person, with these controls | `codex exec`, with `--json` (events of Codex's own shape) and `--output-schema`; the other options have no counterpart | Not planned |
| Scheduled prompts and monitors (`CronCreate`, `Monitor`, `/loop`; `CLAUDE_CODE_LOOP_KEEPALIVE`, 2.1.319, in newer versions) | The model schedules prompts for itself and watches a command; newer versions keep a recurring task up to 30 days (2.1.330) | Absent | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): `Monitor` and the Cron tools, done 2026-11-20 `aaa300e67a` |
| Channels, LSP servers and the `LSP` tool | Push events into a session; code intelligence | Absent | Not planned; [jjcn](https://github.com/SotA-Harness-Lab/jumbo-jet-cod-claw-ntr): LSP servers and the `LSP` tool planned |
| `CLAUDE_CONFIG_DIR` | Moves `~/.claude` | Not read: `~/.claude` is always used. `CODEX_HOME` moves Codex's own directory | Not planned |
| `fastMode` | A faster output mode of some models | Not read. Nearest: `service_tier` in `config.toml` | Not planned |
| `respectGitignore`, `fileSuggestion`, `defaultShell`, `plansDirectory`, `CLAUDE_CODE_DISABLE_FILE_MENTIONS` (newer versions, 2.1.299) | Tune file completion, the shell and where plans are kept | Not read | Not planned |
| Workflows: the `Workflow` tool, `/workflows`, `.claude/workflows`, `disableWorkflows`, `workflowSizeGuideline`, the `ultracode` keyword, `CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS` | Newer versions (2.1.154 to 2.1.285; the reference source holds an early form behind a build flag): a script Claude writes runs tens to hundreds of subagents in the background | Absent. Nearest: Codex's multi-agent tools and `codex exec` | Not planned |
| Background sessions and the agent view: `claude --bg`, `/bg`, `claude agents`, `claude attach`, `claude logs`, `/stop`, `worktree.bgIsolation` | Newer versions (2.1.139 to 2.1.290; the reference source holds an early form behind a build flag): sessions run detached, are listed in one view, answered there and attached to | Absent. Nearest: `codex resume`, Codex's `/fork`, `/ps` and `/stop` | Not planned |
| Sending at once (`ctrl+enter`) | Newer versions (2.1.275): the queued messages go now, interrupting the turn; running tools move to the background rather than stop | Not offered. Nearest: Codex's queue, with Esc to interrupt | Not planned |
| `--safe-mode`, `CLAUDE_CODE_SAFE_MODE` | Newer versions (2.1.169): a start with `CLAUDE.md`, plugins, skills, hooks and MCP servers off | No such option. Nearest: `CCN_CLAUDE_COMPAT=0`, which leaves out all of Claude Code's configuration | Not planned |
| OpenTelemetry (`CLAUDE_CODE_ENABLE_TELEMETRY`, the `OTEL_*` variables), with what newer versions add (`OTEL_LOG_RAW_API_BODIES`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_METRICS_INCLUDE_REPOSITORY`, `OTEL_RESOURCE_INCLUDE_HOST`, `TRACEPARENT`, new events and attributes; 2.1.98 to 2.1.326) | Metrics, events and traces of Claude Code are exported | Not read. Nearest: `[otel]` in `config.toml`, with Codex's own events | Not planned |
| Retries and streams of model requests: `CLAUDE_CODE_MAX_RETRIES`, `CLAUDE_CODE_RETRY_WATCHDOG` and its variables, `CLAUDE_CODE_OVERLOADED_RETRY_BASE_DELAY_MS` and `MAX_DELAY_MS`, `CLAUDE_CODE_NONSTREAMING_TIMEOUT_RETRIES`, `CLAUDE_ENABLE_STREAM_WATCHDOG`, `CLAUDE_CODE_GZIP_REQUEST_BODIES` | Newer versions (2.1.186 to 2.1.312): unattended retry that waits out overload and usage limits, and the back-off | Not read. Nearest: `request_max_retries`, `stream_max_retries` and `stream_idle_timeout_ms` of a model provider in `config.toml` | Not planned |
| Variables that tune Claude Code's own run, of newer versions: `AI_AGENT` for subprocesses (2.1.120), `CLAUDE_CODE_PERFORCE_MODE` (2.1.98), `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` (2.1.98), `CLAUDE_CODE_SCRIPT_CAPS` (2.1.98), `CLAUDE_CODE_SIMPLE_SYSTEM_PROMPT` (2.1.330), `CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS` (2.1.328), `CLAUDE_CODE_DISABLE_AGENT_PROGRESS_SUMMARIES` and `CLAUDE_CODE_RESUME_HINT_COMMAND` (2.1.323) | Read | Not read | Not planned |
## Incompatible

What cannot be made to behave as in Claude Code, because it contradicts how Codex works at
its core: the models it runs, its sandbox and approval path, its trust in projects, and the
tools the model provider runs. The third column says what stands in the way, the last what
Codex does in its place.

| Feature | Claude Code | What stands in the way | What Codex does in its place | Status |
| --- | --- | --- | --- | --- |
| Claude models: the `model` of a `prompt` or `agent` hook, of a subagent or of a skill, the `Agent` tool's `model` (`sonnet`, `opus`, `haiku`), `inherit`, the `model`, `availableModels` and `fallbackModel` settings, `opusplan`, `[1m]`, `CLAUDE_CODE_SUBAGENT_MODEL`, and of newer versions `CLAUDE_CODE_SUBAGENT_MODEL_FORCE`, `CLAUDE_CODE_WORKFLOW_SUBAGENT_MODEL`, `deniedModels`, `availableModelsMatch`, `enforceAvailableModels`, and the `ANTHROPIC_*` variables | The named model runs | Codex runs the models of its own provider, not Claude's | A hook's model is mapped by `[hooks.models]` of `ccn-config.toml`, or else ignored; a subagent's and a skill's `model` are left out; the `Agent` tool takes a model name Codex knows; the settings are not read | Not planned |
| The permission mode: `defaultMode`, `--permission-mode`, and `permission_mode` in a hook's input (`default`, `acceptEdits`, `plan`, `auto`, `dontAsk`, `bypassPermissions`); `disableBypassPermissionsMode`; a subagent's `permissionMode`; newer versions start an interactive session in auto mode when none is set (2.1.283) and add `CLAUDE_CODE_AUTO_MODE_SERVER` (2.1.281) and `CLAUDE_CODE_DISABLE_DANGEROUS_RM_TIMEOUT` (2.1.281) | One mode decides what is asked | Codex has two settings where Claude Code has one, the approval policy and the sandbox mode, and none of Claude Code's modes is one of their combinations | `defaultMode` is not applied. A hook is told `default`, or `bypassPermissions` when Codex never asks | Not planned |
| An `allow` rule, or `dangerouslyDisableSandbox`, as the last word | The call runs, outside the sandbox where it asks to | The sandbox is not part of the approval: Codex's orchestrator decides what may leave it, and a rule in a file the model can write must not | An allowed call runs without asking, still in the sandbox. Leaving the sandbox goes through Codex's approval, and a command Codex forbids, or a managed rule that wants a person, stays as it is | Not planned |
| `additionalDirectories`, `--add-dir`, `/add-dir` | The model reads there without asking; edits follow the permission mode | Codex reads without asking anyway, and its only way to let edits in is a writable root of the sandbox, which a project's settings could then open by themselves | Not applied. Add a directory to `[sandbox_workspace_write] writable_roots` in `config.toml` yourself | Not planned |
| Claude Code's own checks that are not rules: its read-only command list, its path checks for commands, the classifier and its `autoMode` setting (`allow`, `soft_deny`, `environment`, `hard_deny`, newer versions); the checks newer versions add (an `rm` aimed at the root, home or the working folder, some `cp`, `mv` and `docker` commands, very long arguments); asking about a command its parser does not take apart; asking about `.git`, `.claude`, shell profiles and the like whatever the rules say | Decide what is asked | The decision belongs to Codex's approval path, which has its own judgment of commands and paths; two judges of the same call would contradict each other | Where no rule decides, Codex does, as for its own tools | Not planned |
| A project's hooks, instruction files, subagents, memory and settings, used as soon as the folder is trusted | One trust for the folder, asked once | Codex keeps a trust level for a project and, apart from it, a trust for each of a project's hooks, so that a cloned repository cannot run commands by itself | Project instruction files, subagents and the memory are left out of a project marked untrusted. A project's `env`, its approvals of `.mcp.json` servers, its `allow` rules, the plugins it enables and its naming of the memory directory need the project trusted. A project's hooks (and the hooks of a plugin only a project enables) wait for the hook's trust (`/hooks`, or `--dangerously-bypass-hook-trust`). The user's own hooks run without it | Not planned |
| `WebSearch` with Codex's usual search: its name, `allowed_domains` and `blocked_domains`, hooks matching it | A tool Claude Code runs | The hosted `web_search` tool is run by the model provider inside the model's response: the model calls it in its own format, not as a function Codex could rename, give other parameters or put a hook before | The model keeps Codex's `web_search`. `WebSearch` is offered only with Codex's standalone search (`web.run`); allowed domains can be set for every search with `[tools.web_search] filters.allowed_domains` in `config.toml` | Not planned |
| `ExitPlanMode`, and hooks matching it | A tool call ends plan mode and asks for approval of the plan | Codex's plan mode ends with the plan in the final answer, not with a tool call | Only `Stop` sees the plan | Not planned |
| `Read`, `Grep`, `Glob`, `WebFetch` and `WebSearch` in hook matchers and permission rules, with the Claude tool set off | Tools of their own | Codex's own tool set has no such tools: the model reads and searches with shell commands | Such matchers never match and such rules are not applied; shell commands are matched as `Bash`. Turn the Claude tool set on to have the tools | Not planned |
| `Write` and `Edit` writing the file themselves | `Write` writes the content as given; `Edit` writes the whole file back in the line ending it found | Every write is a patch, so that approval, the sandbox, patch events and the turn's diff stay Codex's; a patch ends every file with a line break and gives new lines the file's first line ending | Untouched lines keep their endings; a file changed since it was read makes the write fail | Not planned |
| `Glob`, `Grep` and `WebFetch` as actions of their own | Tool calls that are not commands | The layer has no way of its own to reach the filesystem or the network: only a command passes through Codex's sandbox, network proxy and command rules | Each is a command of the turn (`rg`, `curl`): it shows as one, is asked for under an approval policy that asks for commands, and needs the program on the machine that runs commands | Not planned |
| A plugin's output styles and channels, and files that configure Claude Code's interface (`statusLine`, `outputStyle`, spinner settings and the rest) | Used | They configure Claude Code's own program, which is not running | A plugin that declares or sets them is disabled whole, unless listed in `[claude.plugins] enable` (see "Which plugins are used") | Not planned |
| A plugin's saved state (`CLAUDE_PLUGIN_DATA`), and managing these plugins from Codex (newer versions add `claude plugin init`, `configure`, `details`, `eval`, `test`, `tag`, `prune`, `restore-set-aside`, `install --marketplace`, `--config` and `--json`) | One data directory; `/plugin` | The plugins stay Claude Code's installations: Codex loads them from their install paths without installing them, and writes nothing into Claude Code's directories for them | `CLAUDE_PLUGIN_DATA` is Codex's own directory (`$CODEX_HOME/plugins/data/...`), so state is not shared; install, enable and disable in Claude Code | Not planned |
| Names of a plugin's MCP servers (`plugin:<plugin>:<server>`) | Prefixed | Codex's plugin loader names a server as its file does, and tool names and permission rules follow from that name | The names their files give them. Of two plugins' servers of the same name, the first plugin's (by id) is used | Not planned |
| The `sandbox.*` settings (`enabled`, `excludedCommands`, `allowUnsandboxedCommands`, `filesystem.*`, `network.*`; of newer versions `filesystem.disabled`, `credentials`, `ripgrep`, `allowAppleEvents`, `allowNestedUserNamespaces`, `network.strictAllowlist`, `network.tlsTerminate`), and permission rules feeding the sandbox | A sandbox that is off unless turned on, built from these settings and from the `Edit`, `Read` and `WebFetch` rules; a sandboxed command needs no approval | Codex's sandbox is always there and is set by Codex's configuration alone; a file a project can write must not widen it | Not read. Set `sandbox_mode`, `[sandbox_workspace_write]` and the managed network proxy in `config.toml` | Not planned |
| Credentials and the provider: `apiKeyHelper`, `awsAuthRefresh`, `forceLoginMethod`, `forceLoginGatewayUrl`, `ANTHROPIC_WORKSPACE_ID`, `CLAUDE_CODE_GATEWAY_HINT_HEADERS`, Bedrock, Vertex and gateway variables | Say how Claude Code reaches Anthropic's models | They belong to a provider Codex does not talk to | Not read. Codex's `model_providers` and its own login | Not planned |
| Tools and connectors that run at Anthropic: scheduled cloud tasks (`RemoteTrigger`), artifacts, claude.ai's MCP connectors, Remote Control and Claude Code on the web; of newer versions the `Artifact` and `SendFeedback` tools, `claude ultrareview`, the Claude apps gateway, and the skills and plugins synced from claude.ai (`syncClaudeAiSkills`, `syncClaudeAiPlugins`) | Work with a claude.ai login | They need Anthropic's services and account | Absent | Not planned |
| Interface settings of newer versions: `tui`, `autoScrollEnabled`, `maxProseWidth`, `timeFormat`, `timeZone`, `spellcheck`, `vimInsertModeRemaps`, `emojiCompletionEnabled`, `wheelScrollAccelerationEnabled`, `footerLinksRegexes`, `prUrlTemplate`, `subagentStatusLine`, `axScreenReader`, themes in `~/.claude/themes/`, `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`, `CLAUDE_CODE_HIDE_CWD`, the status line's new input | Configure Claude Code's terminal interface | They configure Claude Code's own program, which is not running | Not read. Nearest: `[tui]` of `config.toml` (`alternate_screen`, `status_line`, `[tui.keymap]`) | Not planned |
| Anthropic's prompt cache and prices: `promptCacheTtl`, `subagentPromptCacheTtl`, `experimental.cacheTtl` of an agent, `modelPricing`, the cache figures of `/cost` and of the status line | Newer versions (2.1.243 to 2.1.260) | They are about Anthropic's prompt cache and its prices | Codex's own caching and usage display | Not planned |
