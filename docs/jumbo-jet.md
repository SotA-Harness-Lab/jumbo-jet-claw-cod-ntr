# Jumbo Jet refinements

What this project changes in the harness itself, on top of CodClaw NTR. Everything in
[Claude Code compatibility](./claude-compat.md) holds here too.

## Waking for finished background work

A session that has nothing to do goes on by itself when work it left in the background ends:
the model is told, and a new turn starts without the user typing anything. Before, the model
only learned of it at the user's next message, or by polling.

- **Background commands.** When a command started with `exec_command` (the `Bash` tool with
  `run_in_background` in the Claude tool set) ends after the call that started it has returned,
  the model receives a `<background_command_notification>` with the command, its
  `session_id` and its exit code. Output it has not read yet is still returned by
  `write_stdin` with that id. No notice is sent when a tool call already reported the exit: the
  starting call while it still waited, or a `write_stdin` that found the command gone.
- **Background agents.** The notice Codex already writes when a spawned agent finishes now
  starts a turn in the idle parent as well.
- If a turn is running, the notice goes into that turn and nothing new starts.

A session is only woken where upstream's own rules for it allow: it is the root session (a
subagent is not woken; it reads the notice in its next turn), it is not in Plan mode, and the
user has not just interrupted it. In those cases the notice is recorded and the model reads it
at the next turn.

This is upstream's feature `wake_on_agent_messages`, which upstream keeps off and marks as under
development, turned on and extended to commands. To turn it off:

```toml
# ~/.codex/config.toml
[features]
wake_on_agent_messages = false
```

or set `CCN_BACKGROUND_WAKE=0` in the environment. With it off, sessions behave as upstream's:
no notice for commands, and an agent's notice waits for the next turn.
