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
`head` and the like) and the files a patch adds, changes or moves to (Claude Code's editing
tools require the file to have been read first). Each file is shown once per session, as a
developer message headed like the others. The globs follow Claude Code: gitignore syntax,
relative to the working directory for the user's rules and to the directory holding `.claude`
for a project's; `{a,b}` is expanded, and a list or a comma-separated string are both read.
This needs the session's environment to run on this machine.

## Hooks

Codex's hook engine takes the same shape as Claude Code's `hooks` key (events, `matcher`,
`type: "command"`, `timeout`, `async`, `statusMessage`; tool names `Bash`, `Edit`, `Write`),
so the hooks of the settings files run as they are. Differences:

- The user's own hooks run without Codex's per-hook trust prompt, as they do in Claude Code. A
  project's hooks wait for Codex's trust like any other project hook.
- Events Codex does not have (`Notification`, `PostToolUseFailure`, `StopFailure` and others)
  are ignored; handlers of type `http` are skipped with a warning; `prompt` and `agent`
  handlers are skipped as for Codex's own hooks.
- `CLAUDE_PROJECT_DIR` is set to the working directory. `disableAllHooks: true` in any of the
  files turns all of them off.

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

To hide a Claude Code skill from Codex altogether (one that only makes sense in Claude Code,
or one that duplicates a Codex skill), disable it in `config.toml`:

```toml
[[skills.config]]
name = "<SKILL_NAME>"
enabled = false
```

## What is not shared

- Slash commands (`commands/**/*.md`) and plugins.
- Other skill frontmatter (`allowed-tools`, `model`, `context`, `hooks`, `arguments` and the
  like) is ignored; `$ARGUMENTS` and other substitutions in a skill's body are not expanded.
- Managed (enterprise) Claude Code files under `/etc/claude-code`.

## Size

The global instructions are never truncated. When they are larger than `project_doc_max_bytes`
(32 KiB by default), Codex prints a warning at startup; raise that setting to silence it and to
leave room for project instructions.
