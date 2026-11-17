# Claude Code compatibility

This fork reads the global Claude Code configuration where Claude Code keeps it, so skills and
instructions are maintained in one place and used by both tools. Nothing is copied or generated,
and there is nothing to set up: when the directory below exists, it is used.

The Claude Code configuration directory is `~/.claude`. Setting `CODEX_CLAUDE_COMPAT=0` in the
environment turns the whole mode off.

## What is shared

| Claude Code | In Codex | When a change shows |
| --- | --- | --- |
| `~/.claude/skills/` | A user skill root, beside `~/.agents/skills` | Like any other skill root |
| `~/.claude/CLAUDE.md` | Appended to the global instructions, after `$CODEX_HOME/AGENTS.md` | At the next request |
| `~/.claude/rules/**/*.md` | Appended after `CLAUDE.md`, sorted by path | At the next request |

- Skills are found as Codex finds its own: nested folders up to six levels deep, directory
  symlinks followed, hidden folders (such as `.trash`) skipped. A skill that is reachable from
  two roots through a symlink is listed once.
- Each instruction file becomes a section headed by its path. A rule limited to some files by a
  `paths:` entry in its frontmatter is kept, with a line saying which files it applies to; it is
  not matched against the files a turn touches.
- A short preamble tells the model that these instructions come from Claude Code, that `/<name>`
  means the skill of that name, and that the hooks they mention do not run in Codex.

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

- Hooks, permissions and other entries of Claude Code's `settings.json`. A rule that Claude Code
  enforces with a hook is only text here.
- Subagents, slash commands, plugins and MCP servers. Codex's own `/import` copies those once.
- Project files. To read a repository's `CLAUDE.md`, set
  `project_doc_fallback_filenames = ["CLAUDE.md"]` in `config.toml`.
- Other skill frontmatter (`allowed-tools`, `model`, `context`, `hooks`, `arguments` and the
  like) is ignored; `$ARGUMENTS` and other substitutions in a skill's body are not expanded.

## Size

The global instructions are never truncated. When they are larger than `project_doc_max_bytes`
(32 KiB by default), Codex prints a warning at startup; raise that setting to silence it and to
leave room for project instructions.
