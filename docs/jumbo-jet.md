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

## Commands that outlast their timeout

With the Claude tool set on, a `Bash` command that was not started with `run_in_background`
used to be stopped at its timeout (2 minutes, or the `timeout` the model gave, at most 10),
and whatever it had built or downloaded by then was lost. Now it keeps running:

- The call returns at the timeout with the command's id, the output it has written so far,
  and how to read the rest (`write_stdin` with that id).
- The command is then a background command like any other: the model is told when it ends
  (see "Waking for finished background work"), and it is stopped with the session.
- If the turn is interrupted while the call still waits, the command is stopped, as before.
- `sleep` as the first word is still stopped at the timeout, as in Claude Code: there is
  nothing to go on with while a command only waits.

This needs Codex's unified exec, which keeps processes running; without it, and with
`background_on_timeout = false`, commands are stopped at their timeout as before, and the
model is told `Command timed out after ...` (it used to read `Exit code 124`).

Codex's own `exec_command` never stopped a command at a time limit and is unchanged.

## Long command output kept in a file

When a command's output is longer than the model is shown, the model sees its head and its
tail and nothing of the middle, and used to run the command again to see more. Now the output
is written to a file and the last line of what the model reads names it:

```
[Full output saved to: /tmp/ccn-tool-output/<session>/<chunk>.txt - read or search that file
for the part you need instead of running the command again]
```

- This holds for `exec_command`, `write_stdin` and the Claude tool set's `Bash`.
- The files are in the system's temporary directory, in a directory of the session that only
  the user can read. A session's directory is removed once nothing was written to it for a
  week, the next time any session keeps an output.
- Codex itself collects at most 1 MiB of a command's output, the first and the last half of
  that. For a larger output the file holds those two parts, and the line says so.
- Nothing is written for a command that runs on another machine (a remote environment): the
  file would not be where the model's next command runs.

Both are on by default and are turned off in `ccn-config.toml`:

```toml
[harness]
background_on_timeout = false
spill_long_output = false
```

## Monitor and scheduled prompts

Four tools let the model be called back in place of polling. They are offered whether or not
the Claude tool set is on, under the names and with the parameters Claude Code's tools have.

### `Monitor`

`Monitor` starts a long-running command and tells the model what it writes. Parameters:
`command`, `description` (a few words naming what is watched), `timeout_ms` (default 300000,
between 1000 and 1800000) and, an addition of this project, `pattern`, a regular expression
that only lines to deliver match.

- The command is started as a background command is: same approvals, same sandbox, with the
  Claude tool set on the `Bash` permission rules first. It keeps its `session_id`, so
  `write_stdin` still reads all its output and Ctrl-C stops it.
- Lines arrive in `<monitor_event>` messages: those written within 200 ms of each other in
  one, and at most one event every 2 seconds. An event carries up to 40 lines of up to 2000
  bytes; more are counted, not sent. An event starts a turn in an idle session, by the rules
  of "Waking for finished background work".
- The monitor stops its command, with one last event that says why, at the timeout and after
  100 events. When the command ends by itself, its exit is reported as any background
  command's.
- It needs Codex's unified exec.

Not covered: lines the command writes in the instant between the end of the starting call
and the monitor taking over are in `write_stdin`'s output but in no event, and a monitor that
cannot keep up with a flood of output loses chunks, which an event's `omitted_lines` counts.
Claude Code's `ws` parameter (a WebSocket to watch) is not offered.

### `CronCreate`, `CronList`, `CronDelete`

`CronCreate` schedules a prompt: `cron` is a 5-field expression in local time (minute, hour,
day of month, month, day of week; `*`, numbers, ranges, steps and lists; Sunday is 0 or 7;
day of month and day of week together match either), `prompt` the text to deliver, and
`recurring` (default true) whether it is delivered at every match or once.

- A due job is delivered as a `<scheduled_prompt>` message and starts a turn, while no turn is
  running; one that comes due during a turn waits for it to end. Where a session may not be
  woken (Plan mode, a subagent, just interrupted), the prompt is recorded and read at the next
  turn.
- Jobs live in the session and are gone when it ends: Claude Code's `durable` jobs, kept in a
  file, are not offered, and neither is its jitter.
- A recurring job runs one last time once it is 7 days old and is then removed. At most 50
  jobs. A schedule without a time in the next year is refused.
- `CronList` lists the jobs with their next time; `CronDelete` cancels one by its id.

All four are on by default and are turned off together:

```toml
[harness]
background_tools = false
```

## Rewinding puts the files back

Going back to an earlier prompt (Esc, then Esc to pick one, Enter) used to change the
conversation only: the files stayed as the turns taken away had left them. Now the files can
go back too.

- Whenever an edit is applied (`apply_patch`, also when typed in the shell, and the Claude
  tool set's `Write` and `Edit`), what it replaced is kept: for each file, its text before
  the turn's first edit of it, or that the file did not exist. One checkpoint a turn, in
  `$CODEX_HOME/file-history/<thread>/<turn>.json`, readable by the user alone.
- On a rewind, if the turns taken away changed files that still differ from what they held,
  a list asks what to put back: **Restore code and conversation**, **Restore conversation**
  (what a rewind always did), **Restore code** or **Never mind**. It names the files.
  Without such files the rewind goes on as before, unasked.
- Putting back writes each file as it was before the earliest of those turns, and removes
  files they created.

What it does not cover, and the list says so: changes made by shell commands or by hand. Also
not covered: files that are not text or are larger than 10 MiB, and edits in a remote
environment (the checkpoints are on this machine); a turn with such an edit is marked, and the
list says the files may not end up exactly as they were. Checkpoints are not removed when a
session ends.

To turn the checkpoints off:

```toml
[harness]
file_checkpoints = false
```
