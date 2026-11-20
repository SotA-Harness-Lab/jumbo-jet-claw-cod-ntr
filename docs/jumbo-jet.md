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

## Continuing a response cut off at the output limit

When a response ends because it reached the output token limit, the turn used to fail with
`Incomplete response returned, reason: max_output_tokens`, although part of the work was done.
Now the model is asked to go on:

- What the response had finished stays: complete tool calls have run and their results are in
  the history, and complete messages are kept.
- Text the model had begun is kept as written, cut off where the limit fell. A tool call or a
  reasoning item that was cut off cannot be replayed and is dropped.
- A short developer message tells the model what happened and asks it to continue with a
  shorter response, without repeating what is already there. A warning tells the user.

This happens up to 3 times in a row; a response that completes resets the count. One more
cut-off response is the error it always was. To change the number, or turn it off with `0`:

```toml
# ~/.codex/ccn-config.toml
[harness]
output_limit_continuations = 3
```

Not covered: with `responses.process_mode = "after_complete_response"`, Codex holds a
response's items until it completes, so a cut-off response leaves nothing to continue from and
the model starts that step over.
