
## Slash commands and the composer

slash-config = configure Codex preferences
slash-feedback = send logs to maintainers or save a ZIP
slash-new = start a new chat during a conversation
slash-init = create an AGENTS.md file with instructions for Codex
slash-compact = summarize conversation to prevent hitting the context limit
slash-recap = summarize the current conversation now
slash-review = review my current changes and find issues
slash-rename = rename the current thread
slash-resume = resume a saved chat
slash-archive = archive this session
slash-delete = permanently delete this session
slash-clear = clear the terminal and start a new chat
slash-fork = fork the current chat
slash-worktree = start or continue a conversation in a new worktree
slash-quit = exit Codex
slash-copy = copy the last response or part of it
slash-export = export the conversation as markdown
slash-raw = toggle raw scrollback mode for copy-friendly terminal selection
slash-tui = choose the TUI mode for the next launch
slash-diff = show git diff (including untracked files)
slash-mention = mention a file
slash-skills = use skills to improve how Codex performs specific tasks
slash-import = import setup, this project, and recent chats from Claude Code
slash-hooks = view and manage lifecycle hooks
slash-daemon = Manage the local background server
slash-warnings = view retained warnings and diagnostic details
slash-status = show current session configuration and token usage
slash-cd = change the current working directory
slash-pwd = show the current working directory
slash-usage = view account usage or use a usage limit reset
slash-debug-config = show config layers and requirement sources for debugging
slash-title = configure which items appear in the terminal title
slash-statusline = configure which items appear in the status line
slash-theme = choose a syntax highlighting theme
slash-pets = choose or hide the terminal pet
slash-ps = list background terminals
slash-stop = stop all background terminals
slash-memory-drop = DO NOT USE
slash-memory-update = DO NOT USE
slash-model = choose what model and reasoning effort to use
slash-presets = run or save reusable prompts and slash commands
slash-daybreak = turn Daybreak on or off
slash-ide = include current selection, open files, and other context from your IDE
slash-plan = switch to Plan mode
slash-voice = start or stop voice; use /voice settings to choose a voice
slash-goal = set or view the goal for a long-running task
slash-agents = open the agent command center
slash-multi-agents = switch between this session's subagents
slash-side = start a side conversation
slash-permissions = choose what Codex is allowed to do
slash-keymap = remap TUI shortcuts
slash-vim = toggle Vim mode for the composer
slash-elevate-sandbox = set up elevated agent sandbox
slash-experimental = toggle experimental features
slash-auto-review = approve one retry of a recent auto-review denial
slash-memories = configure memory use and generation
slash-mcp = list MCP tools; /mcp verbose or /mcp login <name> [--environment <id>]
slash-apps = manage apps
slash-plugins = browse plugins
slash-logout = log out of Codex
slash-rollout = print the rollout file path
slash-test-approval = test approval request
composer-placeholder = Ask Codex to do anything
composer-side-placeholder = Ask a follow-up question

## bottom_pane/actionable_banner.rs
actionable-banner-press-number = Press a number to choose
actionable-banner-number-esc-type = Press a number to choose · esc to dismiss · type to continue
actionable-banner-esc-type = esc to dismiss · type to continue

## bottom_pane/apply_patch_header.rs
apply-patch-header-thread = Thread:{" "}
apply-patch-header-default-description = Apply proposed file edits
apply-patch-header-description = Description:{" "}
apply-patch-header-destination = Destination:{" "}
apply-patch-header-unavailable = unavailable

## bottom_pane/async_questions/mod.rs
async-questions-type-answer = Type your answer
async-questions-progress = { $current } of { $total }
async-questions-other-write-answer = Other (write an answer)
async-questions-other = Other

## bottom_pane/async_questions/render.rs
async-questions-no-options = No options
async-questions-submit = submit
async-questions-skip = skip
async-questions-prev-question = prev question
async-questions-main-prompt = main prompt
async-questions-next-question = next question
async-questions-queued-messages = queued messages
async-questions-option-position = option { $current }/{ $total }

## bottom_pane/async_questions/state.rs
async-questions-expand-terminal = Expand terminal to read the entire option
async-questions-answer-too-long = Answer too long; shorten it before sending

## bottom_pane/chat_composer/footer_state.rs
footer-state-shortcuts = {" "}shortcuts

## bottom_pane/chat_composer/passive_hints.rs
passive-hints-ide-context = IDE context
passive-hints-to-queue-message = {" "}to queue message
passive-hints-to-queue = {" "}to queue
passive-hints-for-agents = {" "}for agents
passive-hints-for-shortcuts = {" "}for shortcuts

## bottom_pane/chat_composer/warning_notice.rs
warning-notice-count = { $count ->
    [one] 1 warning
   *[other] { $count } warnings
}
warning-notice-to-view = {" "}to view

## bottom_pane/chat_composer/history_search.rs
history-search-prompt = reverse-i-search:{" "}
history-search-searching = {" "}{" "}searching
history-search-accept = {" "}accept
history-search-cancel = {" "}cancel
history-search-no-match = {" "}{" "}no match

## bottom_pane/command_popup.rs
command-popup-unavailable-side = not available in a side conversation
command-popup-no-matches = no matches

## bottom_pane/custom_prompt_view/picker.rs
custom-prompt-picker-normal-mode = {" "}normal mode
custom-prompt-picker-back = {" "}back

## bottom_pane/file_search_popup.rs
file-search-popup-loading = {" "}{" "}loading...
file-search-popup-no-matches = {" "}{" "}no matches

## bottom_pane/experimental_features_view.rs
experimental-features-loading-server = Loading server experiments…
experimental-features-title = Experimental features
experimental-features-intro = Checked features are configured on. Some experimental features take effect only in new tasks or after restarting the Codex server.
experimental-features-saving-hint = Saving… Closing this popup will not cancel the write.
experimental-features-retained-hint = Selections retained. Save to retry, or cancel to close.
experimental-features-read-only = {" "}(read-only)
experimental-features-saving = Saving experimental features…
experimental-features-save-interrupted = Saving was interrupted. Reopen /experimental to check configured values.
experimental-features-discovery-interrupted = Discovery was interrupted
experimental-features-no-server = No server experiments available.
experimental-features-server-unavailable = Server experiments unavailable. Reopen /experimental to retry; restart this Codex client if requests remain unanswered.
experimental-features-none-available = {" "}{" "}No experimental features available for now
experimental-features-toggle = {" "}toggle
experimental-features-save = {" "}save
experimental-features-cancel = {" "}cancel

## bottom_pane/feedback_note_view.rs
feedback-note-scroll-disclosure = {" "}scroll disclosure
feedback-note-cancel = cancel
feedback-note-save-zip = save ZIP
feedback-note-send-to-openai = send to OpenAI
feedback-note-save = save
feedback-note-send = send
feedback-note-uploads-disabled = Feedback uploads are disabled.
feedback-note-employee-notice = Feedback sent to OpenAI may be used to improve our models and products
feedback-note-employee-agree = By submitting feedback, you agree that OpenAI can use your feedback for safety purposes and internal model training, as explained in more detail{" "}
feedback-note-here = here
feedback-note-employee-avoid = {"."} Please avoid sharing tented information under NDA, or privileged, HR-related, or sensitive personal information.
feedback-note-external-notice = Feedback sent to OpenAI can be used to improve ChatGPT.{" "}
feedback-note-learn-more = Learn more
feedback-note-save-zip-also = You can also save feedback to a ZIP file, which is not sent to OpenAI.
feedback-note-save-zip-only = You can save feedback to a ZIP file, which is not sent to OpenAI.
feedback-note-title-bad-result = Tell us more (bad result)
feedback-note-title-good-result = Tell us more (good result)
feedback-note-title-bug = Tell us more (bug)
feedback-note-title-safety-check = Tell us more (safety check)
feedback-note-title-other = Tell us more (other)
feedback-note-placeholder-default = (optional) Write a short description to help us further
feedback-note-placeholder-safety-check = (optional) Share what was refused and why it should have been allowed

## bottom_pane/feedback_view.rs
feedback-view-uploaded = • Feedback uploaded.
feedback-view-recorded-no-logs = • Feedback recorded (no logs).
feedback-view-share-in-slack = { $prefix } You can share this in #codex-feedback:
feedback-view-open-issue = { $prefix } Please open an issue using the following URL:
feedback-view-thanks = { $prefix } Thanks for the feedback!
feedback-view-sentry-id = {" "}{" "}Sentry Feedback ID:{" "}
feedback-view-sentry-url = {" "}{" "}Sentry URL:{" "}
feedback-view-or-mention = {" "}{" "}Or mention your thread ID{" "}
feedback-view-in-existing-issue = {" "}in an existing issue.
feedback-view-thread-id = {" "}{" "}Thread ID:{" "}
feedback-view-how-was-this = How was this?
feedback-view-cat-bug = bug
feedback-view-cat-bug-desc = Crash, error message, hang, or broken UI/behavior
feedback-view-cat-bad-result = bad result
feedback-view-cat-bad-result-desc = Output was off-target, incorrect, incomplete, or unhelpful
feedback-view-cat-good-result = good result
feedback-view-cat-good-result-desc = Helpful, correct, high‑quality, or delightful result worth celebrating
feedback-view-cat-safety-check = safety check
feedback-view-cat-safety-check-desc = Benign usage blocked due to safety checks or refusals
feedback-view-cat-other = other
feedback-view-cat-other-desc = Slowness, feature suggestion, UX feedback, or anything else
feedback-view-disclosure-zip = Saving a ZIP does not submit feedback to OpenAI.{"\u000A"}Files may contain prompts, commands, and tool output:
feedback-view-disclosure = Files may contain prompts, commands, and tool output:
feedback-view-include-logs = Include logs and session history?
feedback-view-client-logs = client-logs.txt (if available)
feedback-view-file-if-available = { $filename } (if available)
feedback-view-connectivity = Connectivity diagnostics
feedback-view-yes = Yes
feedback-view-yes-desc = Include session logs and diagnostics in the report
feedback-view-no = No
feedback-view-no-desc = Keep only your note and report metadata

## bottom_pane/hooks_browser_render.rs
hooks-browser-event-header = Event               Active/Installed
hooks-browser-no-hooks = {" "}{" "}No hooks installed for this event.

## bottom_pane/memories_settings_view.rs
memories-use = Use memories
memories-use-desc = Use memories starting with the next thread
memories-generate = Generate memories
memories-generate-desc = Generate memories from this thread and future threads
memories-reset-all = Reset all memories
memories-reset-all-desc = Clear local memory files and summaries without changing existing threads
memories-learn-more = Learn more:{" "}
memories-title = Memories
memories-intro = Choose how Codex uses and creates memories. Changes are saved to config.toml
memories-reset-title = Reset all memories?
memories-reset-intro = This clears local memory files and rollout summaries for the current Codex home.
memories-go-back = Go back
memories-reset-delete-desc = Delete local memory files and rollout summaries
memories-reset-back-desc = Return to memory settings
memories-none-available = {" "}{" "}No memory settings available
memories-toggle = {" "}toggle
memories-save-select = {" "}save/select
memories-cancel = {" "}cancel

## bottom_pane/mentions_v2/footer.rs
mentions-footer-insert = {" "}insert ·{" "}
mentions-footer-close = {" "}close ·{" "}
mentions-footer-select = {" "}select ·{" "}
mentions-footer-filter = {" "}filter

## bottom_pane/mentions_v2/candidate.rs
mention-candidate-plugin = Plugin
mention-candidate-skill = Skill
mention-candidate-task = Task
mention-candidate-file = File
mention-candidate-dir = Dir

## bottom_pane/mentions_v2/render.rs
mentions-render-title = Mentions
mentions-render-subtitle = Files, directories, plugins, skills and tasks
mentions-render-type-to-search = Type to search mentions

## bottom_pane/mentions_v2/search_catalog.rs
mentions-catalog-plugin = Plugin
mentions-catalog-plugin-with-capabilities = Plugin - { $capabilities }
mentions-catalog-skills = skills
mentions-catalog-mcp-servers = { $count ->
    [one] 1 MCP server
   *[other] { $count } MCP servers
}
mentions-catalog-apps = { $count ->
    [one] 1 app
   *[other] { $count } apps
}

## bottom_pane/mentions_v2/search_mode.rs
mentions-mode-all-results = All Results
mentions-mode-filesystem-only = Filesystem Only
mentions-mode-plugins = Plugins

## bottom_pane/multi_select_picker.rs
multi-select-type-to-search = Type to search
multi-select-no-matches = no matches
multi-select-toggle = {" "}toggle
multi-select-reorder = {" "}reorder
multi-select-save = {" "}save
multi-select-cancel = {" "}cancel

## bottom_pane/pending_input_preview.rs
pending-input-after-tool-call = Messages to be submitted after next tool call
pending-input-press = {" "}(press{" "}
pending-input-to-interrupt = {" "}to interrupt and send immediately)
pending-input-end-of-turn = Messages to be submitted at end of turn
pending-input-queued-follow-ups = Queued follow-up inputs
pending-input-edit-last = {" "}edit last queued message

## bottom_pane/popup_consts.rs
popup-press = Press{" "}
popup-to-confirm-or = {" "}to confirm or{" "}
popup-to-go-back = {" "}to go back
popup-select = {" "}select
popup-back = {" "}back
popup-or = {" "}or{" "}

## bottom_pane/questions.rs
questions-count = { $count ->
    [one] 1 question
   *[other] { $count } questions
}
questions-to-answer = {" "}to answer

## bottom_pane/request_user_input/render.rs
request-input-unanswered = { $count ->
    [one] 1 unanswered question
   *[other] { $count } unanswered questions
}
request-input-no-choices = No choices
request-input-no-options = No options
request-input-option-position = option { $selected }/{ $total }

## bottom_pane/selection_row_layout.rs
selection-row-desc-disabled = { $desc } (disabled: { $reason })
selection-row-disabled = disabled: { $reason }
selection-row-disabled-suffix = {" "}(disabled)

## bottom_pane/shortcut_overlay.rs
shortcut-overlay-commands = Commands
shortcut-overlay-mention-files = Mention files
shortcut-overlay-shell-command = Shell command
shortcut-overlay-new-line = New line
shortcut-overlay-paste-image = Paste image
shortcut-overlay-external-editor = External editor
shortcut-overlay-search-history = Search history
shortcut-overlay-edit-last-message = Edit last message
shortcut-overlay-queue-message = Queue message
shortcut-overlay-send-message = Send message
shortcut-overlay-change-mode = Change mode
shortcut-overlay-less-reasoning = Less reasoning
shortcut-overlay-more-reasoning = More reasoning
shortcut-overlay-voice = Voice
shortcut-overlay-agents = Agents (empty prompt)
shortcut-overlay-subagents = Subagents (empty prompt)
shortcut-overlay-inspect-activity = Inspect activity
shortcut-overlay-interrupt = Interrupt
shortcut-overlay-quit = Quit
shortcut-overlay-open-transcript = Open transcript
shortcut-overlay-find-text = Find text
shortcut-overlay-scroll = Scroll
shortcut-overlay-start-selection = Start selection
shortcut-overlay-top-latest = Top / latest
shortcut-overlay-group-compose = Compose
shortcut-overlay-group-session = Session
shortcut-overlay-group-transcript = Transcript (open first)
shortcut-overlay-keyboard-shortcuts = Keyboard shortcuts
shortcut-overlay-close = {" "}close
shortcut-overlay-customize = {" "}customize
shortcut-overlay-resize = … resize to see all

## bottom_pane/shortcut_hud.rs
shortcut-hud-esc-cancel = Esc cancel
shortcut-hud-more = … more shortcuts

## bottom_pane/list_selection_view.rs
list-selection-current = {" "}(current)
list-selection-header-elided = {"["}… { $count } lines]
list-selection-view-all = {" "}view all
list-selection-search-label = Search ›{" "}
list-selection-no-matches = no matches


## codex-rs/tui/src/bottom_pane/skill_popup.rs

skill-popup-no-matches = no matches
skill-popup-insert = {" "}insert ·{" "}
skill-popup-close = {" "}close

## codex-rs/tui/src/bottom_pane/skills_toggle_view.rs

skills-toggle-view-search-placeholder = Type to search skills
skills-toggle-view-title = Enable/Disable Skills
skills-toggle-view-subtitle = Turn skills on or off. Your changes are saved automatically.
skills-toggle-view-no-matches = no matches
skills-toggle-view-toggle = {" "}toggle
skills-toggle-view-sep = {" "}·{" "}
skills-toggle-view-close = {" "}close

## codex-rs/tui/src/bottom_pane/status_line_setup.rs

status-line-item-model-name = Current model name
status-line-item-model-with-reasoning = Current model name with reasoning level
status-line-item-reasoning = Current reasoning level
status-line-item-profile = Config profile selected with --profile (omitted when unset)
status-line-item-provider = Model provider for the active thread
status-line-item-current-dir = Current working directory
status-line-item-project-root = Project name (omitted when unavailable)
status-line-item-hostname = Current machine hostname (omitted when unavailable)
status-line-item-git-branch = Current Git branch (omitted when unavailable)
status-line-item-pull-request-number = Open pull request number for the current branch (omitted when unavailable)
status-line-item-branch-changes = Committed branch changes against the default branch (omitted when unavailable)
status-line-item-status = Compact session run-state text (Ready, Working, Thinking)
status-line-item-permissions = Active permission profile or sandbox mode
status-line-item-approval-mode = Active command approval mode
status-line-item-context-remaining = Percentage of context window remaining (omitted when unknown)
status-line-item-context-used = Percentage of context window used (omitted when unknown)
status-line-item-five-hour-limit = Remaining usage on the primary usage limit (omitted when unavailable)
status-line-item-weekly-limit = Remaining usage on the secondary usage limit (omitted when unavailable)
status-line-item-codex-version = Codex application version
status-line-item-context-window-size = Total context window size in tokens (omitted when unknown)
status-line-item-used-tokens = Total tokens used in session (omitted when zero)
status-line-item-total-input-tokens = Total input tokens used in session
status-line-item-total-output-tokens = Total output tokens used in session
status-line-item-thread-credits = Estimated current-thread credits (Enterprise workspaces only; omitted when unavailable)
status-line-item-estimated-thread-cost = Estimated current-thread cost in USD (Enterprise workspaces only; omitted when unavailable)
status-line-item-session-id = Current thread identifier (omitted until thread starts)
status-line-item-fast-mode = Effective processing tier for the current model
status-line-item-daybreak = Whether Daybreak is enabled for this thread
status-line-item-raw-output = Whether raw scrollback mode is active
status-line-item-thread-name = Current thread name (omitted when unnamed)
status-line-item-thread-title = Current thread title, or thread identifier when unnamed
status-line-item-workspace-headline = Workspace notification headline (Enterprise workspaces only; omitted when unavailable)
status-line-item-task-progress = Latest task progress from update_plan (omitted until available)
status-line-setup-choose-plugin = Choose a statusline plugin
status-line-setup-use-theme-colors = Use theme colors
status-line-setup-use-theme-colors-desc = Apply colors from the active /theme
status-line-setup-title = Configure Status Line
status-line-setup-subtitle = Select which items to display in the status line.

## codex-rs/tui/src/bottom_pane/status_surface_preview.rs

status-surface-preview-sample-status = Working
status-surface-preview-sample-thread-name = thread name
status-surface-preview-sample-thread-title = thread title
status-surface-preview-sample-permissions = Workspace
status-surface-preview-sample-context-remaining = Context 0% left
status-surface-preview-sample-context-used = Context 0% used
status-surface-preview-sample-five-hour-limit = primary 0%
status-surface-preview-sample-weekly-limit = secondary 0%
status-surface-preview-sample-context-window-size = 0 window
status-surface-preview-sample-used-tokens = 0 used
status-surface-preview-sample-total-input-tokens = 0 in
status-surface-preview-sample-total-output-tokens = 0 out
status-surface-preview-sample-thread-credits = 5.2 credits
status-surface-preview-sample-fast-mode = Fast
status-surface-preview-sample-daybreak = Daybreak off
status-surface-preview-sample-raw-output = raw output
status-surface-preview-sample-workspace-headline = Workspace headline
status-surface-preview-sample-profile = profile: <name>
status-surface-preview-sample-provider = provider: <id>
status-surface-preview-sample-task-progress = Tasks 0/0
status-surface-preview-limit-desc-secondary = Remaining usage on the secondary usage limit (omitted when unavailable)
status-surface-preview-limit-desc-primary = Remaining usage on the primary usage limit (omitted when unavailable)
status-surface-preview-limit-desc-five-hour = Remaining usage on the 5-hour usage limit (omitted when unavailable)
status-surface-preview-limit-desc-daily = Remaining usage on the daily usage limit (omitted when unavailable)
status-surface-preview-limit-desc-weekly = Remaining usage on the weekly usage limit (omitted when unavailable)
status-surface-preview-limit-desc-monthly = Remaining usage on the monthly usage limit (omitted when unavailable)
status-surface-preview-limit-desc-annual = Remaining usage on the annual usage limit (omitted when unavailable)

## codex-rs/tui/src/bottom_pane/title_setup.rs

terminal-title-item-app-name = Codex app name
terminal-title-item-project = Project name (falls back to current directory name)
terminal-title-item-current-dir = Current working directory
terminal-title-item-spinner = Spinner while working, action-required message while blocked
terminal-title-item-status = Compact session run-state text (Ready, Working, Thinking)
terminal-title-item-thread-name = Current thread name (omitted when unnamed)
terminal-title-item-thread = Current thread title, or thread identifier when unnamed
terminal-title-item-git-branch = Current Git branch (omitted when unavailable)
terminal-title-item-context-remaining = Percentage of context window remaining (omitted when unknown)
terminal-title-item-context-used = Percentage of context window used (omitted when unknown)
terminal-title-item-five-hour-limit = Remaining usage on the primary usage limit (omitted when unavailable)
terminal-title-item-weekly-limit = Remaining usage on the secondary usage limit (omitted when unavailable)
terminal-title-item-codex-version = Codex application version
terminal-title-item-used-tokens = Total tokens used in session (omitted when zero)
terminal-title-item-total-input-tokens = Total input tokens used in session
terminal-title-item-total-output-tokens = Total output tokens used in session
terminal-title-item-thread-credits = Estimated current-thread credits (Enterprise workspaces only; omitted when unavailable)
terminal-title-item-estimated-thread-cost = Estimated current-thread cost (Enterprise workspaces only; omitted when unavailable)
terminal-title-item-session-id = Current thread identifier (omitted until thread starts)
terminal-title-item-fast-mode = Effective processing tier for the current model
terminal-title-item-daybreak = Whether Daybreak is enabled for this thread
terminal-title-item-model = Current model name
terminal-title-item-model-with-reasoning = Current model name with reasoning level
terminal-title-item-reasoning = Current reasoning level
terminal-title-item-task-progress = Latest task progress from update_plan (omitted until available)
title-setup-title = Configure Terminal Title
title-setup-subtitle = Select which items to display in the terminal title.

## codex-rs/tui/src/bottom_pane/unified_exec_footer.rs

unified-exec-footer-summary = { $count ->
    [one] 1 background terminal running · /ps to view · /stop to close
   *[other] { $count } background terminals running · /ps to view · /stop to close
}

## codex-rs/tui/src/bottom_pane/user_verification.rs

user-verification-to-confirm = to confirm
user-verification-to-cancel = to cancel
user-verification-verify = Verify and approve
user-verification-cancel = Cancel this request
user-verification-thread = Thread:{" "}
user-verification-server = Server:{" "}
user-verification-waiting = Waiting for verification…
user-verification-cancel-hint = to cancel this request

## codex-rs/tui/src/bottom_pane/warnings_view_render.rs

warnings-view-title-entry = Warnings · { $current } of { $total } · { $source }
warnings-view-title = Warnings
warnings-view-none = No warnings
warnings-view-restart-hint = {"\u000A"}These warnings may reflect settings from when the background server started. If you’ve already fixed them, use /daemon to restart it.
warnings-view-keep-next = keep & next
warnings-view-dismiss-close = dismiss & close
warnings-view-copy = copy
warnings-view-warning = warning
warnings-view-scroll = scroll

## codex-rs/tui/src/app_server_approval_conversions.rs

approval-conversions-server-environment = { $server_name } (environment { $environment_id })

## codex-rs/tui/src/history_cell/approvals.rs

approvals-approved = approved
approvals-codex-to-run = {" "}codex to run{" "}
approvals-this-time = {" "}this time
approvals-this-request = {" "}this request
approvals-codex-network-access-to = {" "}codex network access to{" "}
approvals-codex-to-always-run-commands-that-st = {" "}codex to always run commands that start with{" "}
approvals-every-time-this-session = {" "}every time this session
approvals-persisted = persisted
approvals-codex-network-access-to-2 = {" "}Codex network access to{" "}
approvals-denied = denied
approvals-and-saved-that-rule = {" "}and saved that rule
approvals-did-not-approve = did not approve
approvals-request = Request{" "}
approvals-for-codex-to-run = {" "}for codex to run{" "}
approvals-review = Review{" "}
approvals-timed-out = timed out
approvals-before-codex-could-run = {" "}before codex could run{" "}
approvals-before-this-request-could-be-approve = {" "}before this request could be approved
approvals-before-codex-could-access = {" "}before codex could access{" "}
approvals-canceled = canceled
approvals-the-request-to-run = {" "}the request to run{" "}
approvals-the-request-for-codex-network-access = {" "}the request for codex network access to{" "}
approvals-you = You{" "}
approvals-auto-reviewer = Auto-reviewer{" "}
approvals-for-codex-to-apply = {" "}for codex to apply{" "}
approvals-a-patch-touching = a patch touching{" "}
approvals-files = {" "}files
approvals-for = {" "}for{" "}
approvals-before-codex-could-apply = {" "}before codex could apply{" "}
approvals-before = {" "}before{" "}

## codex-rs/tui/src/app/agent_status_feed.rs

agent-status-feed-heading = Sub-agents running
agent-status-feed-none = {" "} • No sub-agents running.
agent-status-feed-no-activity = No recent activity yet.
agent-status-feed-updated-files = Updated { $count } file(s)
agent-status-feed-tool = Tool { $tool }
agent-status-feed-spawned = Spawned an agent
agent-status-feed-sent-input = Sent input to an agent
agent-status-feed-resumed = Resumed an agent
agent-status-feed-waited = Waited for an agent
agent-status-feed-closed = Closed an agent
agent-status-feed-started = Started
agent-status-feed-contacted = Contacted
agent-status-feed-interrupted = Interrupted
agent-status-feed-completed = Completed
agent-status-feed-action-agent = { $action } { $agent_path }
agent-status-feed-web-search = Web search: { $query }
agent-status-feed-viewed = Viewed { $path }
agent-status-feed-generated-image = Generated an image
agent-status-feed-entered-review = Entered review mode
agent-status-feed-exited-review = Exited review mode
agent-status-feed-compacted = Compacted context

## codex-rs/tui/src/app/statusline_picker.rs

statusline-picker-unavailable = {" "}(Unavailable)
statusline-picker-default = Default status line
statusline-picker-offline = Plugin providers are unavailable while disconnected.
statusline-picker-discovering = Discovering MCP status line providers…
statusline-picker-title = Select a status line provider
statusline-picker-save-failed = Failed to save status line selection: { $error }

## codex-rs/tui/src/chatwidget/goal_status.rs

goal-status-budget-tokens = { $used } / { $budget } tokens
goal-status-tokens = { $used } tokens

## codex-rs/tui/src/chatwidget/status_controls.rs

status-controls-copy-model = Model
status-controls-copy-directory = Directory
status-controls-copy-thread-name = Thread name
status-controls-copy-session-id = Session ID
status-controls-limit-left = { $label } { $remaining }% left
status-controls-effort-default = default

## codex-rs/tui/src/chatwidget/status_state.rs

status-state-working = Working
status-state-reviewing-request = Reviewing approval request
status-state-reviewing-prefix = Reviewing{" "}
status-state-reviewing-requests = Reviewing { $count } approval requests
status-state-more = +{ $remaining } more

## codex-rs/tui/src/chatwidget/status_surface_commands.rs

status-surface-commands-clear-title = Cannot combine --clear with terminal-title items.
status-surface-commands-clear-status = Cannot combine --clear with status-line items.
status-surface-commands-ambiguous = Ambiguous { $surface } item '{ $token }'. Use '{ $five_hour_id }' or '{ $weekly_id }'.
status-surface-commands-unknown = Unknown { $surface } item '{ $token }'. Use { $command } to see available items.

## codex-rs/tui/src/chatwidget/statusline_reader.rs
statusline-reader-initializing = Initializing…
statusline-reader-timed-out = Status line timed out
statusline-reader-unavailable = Status line unavailable
statusline-reader-failed = Status line failed

## codex-rs/tui/src/status/helpers.rs
status-helpers-reasoning = reasoning { $effort }
status-helpers-summaries-off = summaries off
status-helpers-summaries = summaries { $summary }
status-helpers-unknown = <unknown>
status-helpers-none = <none>
status-helpers-time-on-date = { $time } on { $date }

## codex-rs/tui/src/status/remote_connection.rs
remote-connection-cloud = Cloud
remote-connection-unknown = unknown
remote-connection-invalid-websocket-url = <invalid websocket URL>
remote-connection-service-older = A background Codex service is running v{ $server }, older than your Codex CLI v{ $client }.
remote-connection-service-different = A background Codex service is running v{ $server }, different from your Codex CLI v{ $client }.

## codex-rs/tui/src/status/thread_usage.rs
thread-usage-models-label = {"  "}Models
thread-usage-reasoning-label = {"  "}Reasoning
thread-usage-speed-label = {"  "}Speed
thread-usage-billed-tokens-label = {"  "}Billed tokens
thread-usage-label = Thread usage
thread-usage-other = Other
thread-usage-reasoning-none = None
thread-usage-reasoning-minimal = Minimal
thread-usage-reasoning-light = Light
thread-usage-reasoning-medium = Medium
thread-usage-reasoning-high = High
thread-usage-reasoning-extra-high = Extra High
thread-usage-reasoning-max = Max
thread-usage-reasoning-ultra = Ultra
thread-usage-speed-fast = Fast mode
thread-usage-speed-ultrafast = Ultrafast
thread-usage-speed-standard = Standard
thread-usage-credits = { $credits } credits
thread-usage-separator = {" "}·{" "}
thread-usage-tokens-input = { $tokens } input
thread-usage-tokens-cached = ({ $tokens } cached)
thread-usage-tokens-output = { $tokens } output
thread-usage-group-share = { $name } { $percentage }%

## codex-rs/tui/src/status_indicator_widget.rs
status-indicator-working = Working
status-indicator-elapsed-open = ({ $elapsed } •{" "}
status-indicator-to-interrupt = {" "}to interrupt)
status-indicator-separator = {" "}·{" "}

## codex-rs/tui/src/terminal_program_status.rs
terminal-program-status-idle = Idle
terminal-program-status-working = Working
terminal-program-status-waiting = Waiting

## codex-rs/tui/src/chatwidget/status_surfaces.rs
status-surfaces-action-required-shown = {"["} ! ] Action Required
status-surfaces-action-required-hidden = {"["} . ] Action Required
status-surfaces-ignored-invalid-status-line = Ignored invalid status line { $count ->
        [one] item
       *[other] items
    }: { $items }.
status-surfaces-ignored-invalid-terminal-title = Ignored invalid terminal title { $count ->
        [one] item
       *[other] items
    }: { $items }.
status-surfaces-profile = profile: { $name }
status-surfaces-provider = provider: { $provider }
status-surfaces-pull-request = PR #{ $number }
status-surfaces-no-changes = No changes
status-surfaces-tokens-used = { $tokens } used
status-surfaces-context-left = Context { $percent }% left
status-surfaces-context-used = Context { $percent }% used
status-surfaces-context-window = { $tokens } window
status-surfaces-tokens-in = { $tokens } in
status-surfaces-tokens-out = { $tokens } out
status-surfaces-credits = { $credits } credits
status-surfaces-standard = Standard
status-surfaces-daybreak-on = Daybreak on
status-surfaces-daybreak-off = Daybreak off
status-surfaces-raw-output = raw output
status-surfaces-tasks-progress = Tasks { $completed }/{ $total }
status-surfaces-permissions-read-only = Read Only
status-surfaces-permissions-workspace = Workspace
status-surfaces-permissions-full-access = Full Access
status-surfaces-permissions-custom = Custom permissions
status-surfaces-approval-auto-review = Approve for me
status-surfaces-approval-ask = Ask for approval

## codex-rs/tui/src/onboarding/auth/headless_chatgpt_login.rs
headless-login-unexpected-response = Unexpected account/login/start response: { $response }
headless-login-finish-signing-in = Finish signing in via your browser
headless-login-preparing = Preparing device code login
headless-login-step-open-link = {"  "}1. Open this link in your browser and sign in
headless-login-step-enter-code = {"  "}2. Enter this one-time code after you are signed in (expires in 15 minutes)
headless-login-continue-only-if = {"  "}Continue only if you started this login in Codex. If a website or another person gave you this code, cancel.
headless-login-requesting-code = {"  "}Requesting a one-time code...
headless-login-press = {"  "}Press{" "}
headless-login-to-cancel = {" "}to cancel

## codex-rs/tui/src/onboarding/bedrock.rs
bedrock-set-up = Set up Amazon Bedrock
bedrock-govcloud-title = Using Codex with AWS GovCloud
bedrock-govcloud-notice-before = You are using Codex with AWS GovCloud. Please ensure you or your administrator have read the{" "}
bedrock-govcloud-guidance-link = application configuration and security guidance
bedrock-govcloud-notice-after = {" "}before proceeding.
bedrock-acknowledge = > Acknowledge
bedrock-checking-credentials = {"  "}Checking for existing AWS credentials...
bedrock-setting-up = {"  "}Setting up Amazon Bedrock...
bedrock-profile-detected = {"  "}AWS profile detected: { $name }
bedrock-region = {"  "}Region: { $region }
bedrock-choose-profile = {"  "}Choose an AWS profile.
bedrock-env-credentials-detected = {"  "}AWS credentials detected in the server environment.
bedrock-no-credentials = {"  "}No AWS credentials found.
bedrock-choose-auth-method = {"  "}Choose how you authenticate with AWS.
bedrock-enter-profile-name = {"  "}Enter the name of your AWS profile.
bedrock-label-aws-profile = {"  "}AWS profile:{" "}
bedrock-enter-api-key = {"  "}Enter your Amazon Bedrock API key.
bedrock-label-api-key = {"  "}Bedrock API key:{" "}
bedrock-enter-region = {"  "}Enter the AWS Region to use with Amazon Bedrock.
bedrock-label-region = {"  "}AWS Region:{" "}
bedrock-enter-access-keys = {"  "}Enter your AWS access keys.
bedrock-field-access-key-id = AWS access key ID
bedrock-field-secret-access-key = AWS secret access key
bedrock-field-session-token = AWS session token (optional)
bedrock-field-prompt = { $marker } { $label }:{" "}
bedrock-export-credentials = {"  "}Export AWS credentials, then run `codex --no-daemon` in that shell.
bedrock-setup-guide = {"  "}Setup guide:{" "}
bedrock-press = {"  "}Press{" "}
bedrock-to-continue = {" "}to continue
bedrock-to-go-back = {" "}to go back
bedrock-scroll-then = {" "}scroll ·{" "}
bedrock-continue-with = Continue with { $name }
bedrock-use-existing-credentials = Use your existing AWS credentials
bedrock-use-env-api-key = Use the Bedrock API key from the server environment
bedrock-use-env-access-keys = Use AWS access keys from the server environment
bedrock-continue-detected = Continue with detected credentials
bedrock-choose-another-method = Choose another sign-in method
bedrock-other-methods = Other AWS sign-in methods
bedrock-other-methods-description = Use another profile, access keys, environment variables, or a Bedrock API key
bedrock-method-profile = AWS profile
bedrock-method-profile-description = Use AWS SSO or a named profile
bedrock-method-access-keys = AWS access keys
bedrock-method-access-keys-description = Enter an access key ID and secret access key
bedrock-method-env = Environment variables
bedrock-method-env-description = Use AWS credentials exported in your shell
bedrock-method-api-key = Bedrock API key
bedrock-method-api-key-description = Enter a Bedrock API key
bedrock-unable-to-check = Unable to check AWS credentials: { $error }
bedrock-unexpected-response = Unexpected account/login/start response: { $response }
bedrock-unable-to-set-up = Unable to set up Amazon Bedrock: { $error }

## codex-rs/tui/src/onboarding/auth.rs
auth-api-key-disabled = API key login is disabled.
auth-sign-in-another-way-heading = {"  "}Sign in another way
auth-choose-how-to-use = {"  "}Choose how you want to use Codex.
auth-sign-in-paid-plan = Sign in with ChatGPT to use Codex as part of your paid plan
auth-or-connect-api-key = or connect an API key for usage-based billing
auth-chatgpt-login-disabled = ChatGPT login is disabled
auth-usage-included-plans = Usage included with Plus, Pro, Business, and Enterprise plans
auth-device-code-description = Sign in from another device with a one-time code
auth-sign-in-another-way = Sign in another way
auth-use-api-key-or-provider = Use an API key or another provider
auth-use-openai-api-key-description = Use an OpenAI API key
auth-sign-in-chatgpt = Sign in with ChatGPT
auth-sign-in-device-code-option = Sign in with Device Code
auth-use-openai-api-key = Use an OpenAI API key
auth-pay-for-what-you-use = Pay for what you use
auth-use-bedrock = Use Amazon Bedrock
auth-bedrock-description = Connect using your AWS credentials
auth-use-vertex = Use Google Cloud Vertex AI
auth-vertex-description = Connect using your Google Cloud credentials
auth-gateway-description = Choose a saved configuration or create a new one
auth-api-key-disabled-by-workspace = {"  "}API key login is disabled by this workspace. Sign in with ChatGPT to continue.
auth-press = {"  "}Press{" "}
auth-to-continue = {" "}to continue
auth-to-go-back = {" "}to go back
auth-finish-signing-in = Finish signing in via your browser
auth-link-does-not-open = {"  "}If the link doesn't open automatically, press{" "}
auth-to-copy-it = {" "}to copy it:
auth-remote-machine-press = {"  "}On a remote or headless machine? Press{" "}
auth-and-choose = {" "}and choose{" "}
auth-sign-in-device-code = Sign in with Device Code
auth-to-cancel = {" "}to cancel
auth-learn-permissions = {"  "}Learn about permissions and approvals in the{" "}
auth-codex-docs = Codex docs
auth-plan-rate-limits = {"  "}Your plan's rate limits and{" "}
auth-training-data-preferences = training data preferences
auth-apply = {" "}apply
auth-signed-in = ✓ Signed in with your ChatGPT account
auth-in-control = {"  "}You're in control
auth-can-make-mistakes = {"  "}Codex can make mistakes
auth-review-code = {"  "}Review the code it writes and commands it runs
auth-included-with-plan = {"  "}Included with your ChatGPT plan
auth-api-key-configured = ✓ API key configured
auth-usage-based-billing = {"  "}Codex will use usage-based billing with your API key.
auth-use-own-api-key = Use your own OpenAI API key for usage-based billing
auth-paste-key-below = {"  "}Paste or type your API key below.
auth-detected-env-key = {"  "}Detected OPENAI_API_KEY environment variable.
auth-paste-different-key = {"  "}Paste a different key if you prefer to use another account.
auth-paste-key-placeholder = Paste or type your API key
auth-api-key-title = API key
auth-to-save = {" "}to save
auth-api-key-empty = API key cannot be empty
auth-unexpected-response = Unexpected account/login/start response: { $response }
auth-failed-to-save-key = Failed to save API key: { $error }
auth-gateway-finish-signing-in = Finish signing in to your gateway in the browser.
auth-gateway-starting = Starting gateway sign-in…
auth-press-ctrl-c-quit = Press Ctrl+C to quit.
auth-press-enter-retry = Press Enter to retry or Ctrl+C to quit.
auth-custom-provider-configured = ✓ Custom provider configured
auth-vertex-configured = ✓ Google Cloud Vertex AI configured
auth-bedrock-configured = ✓ Amazon Bedrock configured


## onboarding/directory_trust.rs

directory-trust-folder-trust-changed = Folder trust changed. Reopen the destination with its restricted settings.

## onboarding/gateway.rs

gateway-default-provider-locked = The default provider is set by configuration policy or a launch override.
gateway-loading = Loading gateways…
gateway-saving = Saving gateway settings…
gateway-hint-to-go-back = to go back
gateway-hint-to-create = to create
gateway-hint-to-use-as-default = to use as default

## onboarding/gateway_login.rs

gateway-login-selected-gateway-changed = The selected gateway changed during sign-in.

## onboarding/gcp.rs

gcp-title = > Set up Google Cloud Vertex AI
gcp-guidance-project = Enter the Google Cloud project ID to use with Vertex AI.
gcp-label-project-id = {"  "}Project ID:{" "}
gcp-guidance-location = Press Enter to use global, or enter a Vertex AI location.
gcp-label-location = {"  "}Location (optional):{" "}
gcp-guidance-credentials = Enter an absolute file path on the host running Codex, or press Enter to use Application Default Credentials (ADC).
gcp-label-credential-file = {"  "}Credential file (optional):{" "}
gcp-validating = Validating credentials and saving configuration…
gcp-adc-hint = {"  "}To set up ADC with the Google Cloud CLI, run
gcp-footer-set-up = {" "}set up
gcp-footer-continue = {" "}continue
gcp-footer-back = {" "}back
gcp-error-project-required = Enter a Google Cloud project ID.

## onboarding/welcome.rs

welcome-welcome-to = Welcome to{" "}
welcome-coding-agent-tagline = , OpenAI's command-line coding agent

## onboarding/trust_directory.rs

trust-directory-existing-task-notice = This existing task may retain settings and history, including project configuration or hooks loaded while it was trusted. To use restricted settings, start a new task. The folder's trust setting will not change.
trust-directory-restricted-notice = Config, hooks, and exec policies from untrusted folders stay disabled. Trusted project folders can still contribute settings. Skills still load, and tools follow your permission settings. Opening will not change saved trust.
trust-directory-trust-prompt = Trust this folder? Codex can read, edit, and run files here, subject to your permission settings. Folder settings can run code automatically, even without a model request. Continue only if you trust these files. Your trust decision will be saved.
trust-directory-open-existing-task = Open existing task
trust-directory-open-restricted = Open restricted
trust-directory-trust-and-continue = Trust and continue
trust-directory-quit = Quit
trust-directory-back-to-agents = Back to Agent Command Center
trust-directory-keep-current-dir = Keep current directory
trust-directory-hint-continue-sandbox = {" "}continue and create sandbox
trust-directory-hint-continue = {" "}continue
trust-directory-hint-quit = {" "}quit
trust-directory-hint-back = {" "}back
trust-directory-hint-cancel = {" "}cancel
trust-directory-git-subdir-note = Note: You’re in a subdirectory of a Git project. Trusting will apply to the repository root:
trust-directory-folder-access = {"  "}Folder access

## onboarding/onboarding_screen.rs

onboarding-screen-gateway-sign-in = {"  "}Sign in to your gateway
onboarding-screen-gateway-step-open-link = {"  "}1. Open this link in a browser
onboarding-screen-gateway-step-enter-code = {"  "}2. Enter this one-time code when prompted
onboarding-screen-copy-copying = Copying link…
onboarding-screen-copy-busy = Clipboard busy; try again
onboarding-screen-copy-confirmed = Copied link to clipboard
onboarding-screen-copy-unconfirmed = Copy requested; check your clipboard
onboarding-screen-copy-failed = Could not copy link: { $error }
onboarding-screen-trust-failed = Failed to set trust for { $path }: { $error }

## chatwidget/keymap_picker.rs

keymap-picker-invalid-config = Invalid `tui.keymap` configuration: { $err }

## keymap/leader.rs

leader-single-prefix-key = Invalid `tui.keymap.global.leader`: use a single prefix key.

## keymap/vim_search.rs

vim-search-conflicting-bindings = Conflicting `{ $path }` and `{ $other }` bindings

## keymap/chords.rs

chords-cannot-dispatch-inventory = Cannot dispatch `{ $path }`: the keymap action inventory exceeds { $max } internal tokens.
chords-cannot-dispatch-unknown = Cannot dispatch unknown keymap action `{ $path }`.
chords-leader-repeat-reserved = Ambiguous `{ $path }` = `{ $spec }`: repeating the leader is reserved for cancellation.
chords-esc-reserved = Ambiguous `{ $path }` = `{ $spec }`: plain `esc` is reserved for cancelling a pending chord.
chords-backspace-reserved = Invalid `{ $path }` = `{ $spec }`: `backspace` is reserved for editing task input.
chords-esc-cancel = Esc cancel

## keymap_setup/actions.rs

keymap-actions-change-where-to-run = Change Where to Run
keymap-actions-binding-inactive = { $spec } (inactive)
keymap-actions-source-custom = Custom
keymap-actions-source-custom-global = Custom global
keymap-actions-source-default = Default
keymap-actions-ctx-global = Global
keymap-actions-ctx-chat = Chat
keymap-actions-ctx-composer = Composer
keymap-actions-ctx-editor = Editor
keymap-actions-ctx-vim-normal = Vim normal
keymap-actions-ctx-vim-search = Vim search
keymap-actions-ctx-vim-operator = Vim operator
keymap-actions-ctx-vim-text-object = Vim text object
keymap-actions-ctx-pager = Pager
keymap-actions-ctx-list = List
keymap-actions-ctx-agents = Agents
keymap-actions-ctx-approval = Approval
keymap-actions-global-change-host = Change where a new conversation runs.
keymap-actions-global-focus-main-composer = Focus the main conversation composer.
keymap-actions-global-focus-sidebar-tabs = Open the sidebar tab chooser.
keymap-actions-global-focus-sidebar-content = Focus the side conversation, or hide it when already focused.
keymap-actions-global-hide-pane = Hide the focused sidebar pane.
keymap-actions-global-close-sidebar-tab = Close the active sidebar tab.
keymap-actions-global-sidebar-up = Switch to the previous sidebar tab.
keymap-actions-global-sidebar-down = Switch to the next sidebar tab.
keymap-actions-global-focus-pane-left = Focus the parent conversation.
keymap-actions-global-focus-pane-right = Focus the open side conversation.
keymap-actions-global-minimize-pane-width = Minimize the focused pane width.
keymap-actions-global-maximize-pane-width = Maximize the focused pane width.
keymap-actions-global-equalize-pane-widths = Equalize visible pane widths.
keymap-actions-global-resize-panes = Enter keyboard resize mode for visible dividers.
keymap-actions-global-move-divider-left = Move the divider left when the side pane has focus.
keymap-actions-global-move-divider-right = Move the divider right when the side pane has focus.
keymap-actions-global-toggle-pane-zoom = Zoom or restore the focused pane.
keymap-actions-global-open-agents = Open the agent command center.
keymap-actions-global-open-transcript = Open the transcript overlay.
keymap-actions-global-find-transcript = Find text in the full transcript.
keymap-actions-global-focus-activity = Focus transcript activity groups to expand or collapse their details.
keymap-actions-global-open-external-editor = Open the current draft in an external editor.
keymap-actions-global-copy = Copy the last agent response to the clipboard.
keymap-actions-global-clear-terminal = Clear the terminal UI.
keymap-actions-global-toggle-vim-mode = Turn Vim composer mode on or off.
keymap-actions-global-toggle-fast-mode = Turn Fast mode on or off.
keymap-actions-global-toggle-raw-output = Toggle raw scrollback mode.
keymap-actions-global-toggle-side-conversation = Switch between a side conversation and its parent.
keymap-actions-global-leader = Set the prefix used by shared leader shortcuts.
keymap-actions-chat-interrupt-turn = Interrupt the active turn.
keymap-actions-chat-decrease-reasoning-effort = Decrease reasoning effort.
keymap-actions-chat-increase-reasoning-effort = Increase reasoning effort.
keymap-actions-chat-previous-permission-mode = Switch to the previous available permission mode.
keymap-actions-chat-next-permission-mode = Switch to the next available permission mode.
keymap-actions-chat-edit-queued-message = Move forward through questions, then edit the last queued message.
keymap-actions-chat-prompt-stack-back = Move back through questions toward the composer.
keymap-actions-chat-skip-question = Skip the focused question.
keymap-actions-chat-toggle-voice = Start or stop a voice conversation.
keymap-actions-chat-open-presets = Run or save reusable inputs.
keymap-actions-chat-toggle-voice-mute = Mute or unmute the voice microphone.
keymap-actions-composer-submit = Submit the current composer draft.
keymap-actions-composer-queue = Queue the draft while a task is running.
keymap-actions-composer-toggle-shortcuts = Show or hide the composer shortcut overlay.
keymap-actions-composer-history-search-previous = Open history search or move to the previous match.
keymap-actions-composer-history-search-next = Move to the next history search match.
keymap-actions-composer-undo = Undo the last composer edit.
keymap-actions-composer-redo = Redo the last composer edit.
keymap-actions-editor-insert-newline = Insert a newline in the editor.
keymap-actions-editor-move-left = Move the cursor left.
keymap-actions-editor-move-right = Move the cursor right.
keymap-actions-editor-move-up = Move the cursor up.
keymap-actions-editor-move-down = Move the cursor down.
keymap-actions-editor-move-word-left = Move to the beginning of the previous word.
keymap-actions-editor-move-word-right = Move to the end of the next word.
keymap-actions-editor-move-line-start = Move to the beginning of the line.
keymap-actions-editor-move-line-end = Move to the end of the line.
keymap-actions-editor-delete-backward = Delete one grapheme to the left.
keymap-actions-editor-delete-forward = Delete one grapheme to the right.
keymap-actions-editor-delete-backward-word = Delete the previous word.
keymap-actions-editor-delete-forward-word = Delete the next word.
keymap-actions-editor-kill-line-start = Delete from cursor to line start.
keymap-actions-editor-kill-whole-line = Delete the current line.
keymap-actions-editor-kill-line-end = Delete from cursor to line end.
keymap-actions-editor-yank = Paste the kill buffer.
keymap-actions-vim-normal-enter-insert = Enter insert mode at the cursor.
keymap-actions-vim-normal-append-after-cursor = Enter insert mode after the cursor.
keymap-actions-vim-normal-append-line-end = Enter insert mode at end of line.
keymap-actions-vim-normal-insert-line-start = Enter insert mode at the first non-blank character.
keymap-actions-vim-normal-open-line-below = Open a new line below and enter insert mode.
keymap-actions-vim-normal-open-line-above = Open a new line above and enter insert mode.
keymap-actions-vim-normal-enter-replace-mode = Enter replace mode and overwrite characters.
keymap-actions-vim-normal-move-left = Move left in Vim normal mode.
keymap-actions-vim-normal-move-right = Move right in Vim normal mode.
keymap-actions-vim-normal-move-up = Move up or recall older history in Vim normal mode.
keymap-actions-vim-normal-move-down = Move down or recall newer history in Vim normal mode.
keymap-actions-vim-normal-move-word-forward = Move to the start of the next word.
keymap-actions-vim-normal-move-word-backward = Move to the start of the previous word.
keymap-actions-vim-normal-move-word-end = Move to the end of the current or next word.
keymap-actions-vim-normal-move-line-start = Move to the start of the line.
keymap-actions-vim-normal-move-line-end = Move to the end of the line.
keymap-actions-vim-normal-find-forward = Find the next character on the current line.
keymap-actions-vim-normal-find-backward = Find the previous character on the current line.
keymap-actions-vim-normal-till-forward = Stop before the next character on the current line.
keymap-actions-vim-normal-till-backward = Stop after the previous character on the current line.
keymap-actions-vim-normal-jump-top = Jump to the first buffer line.
keymap-actions-vim-normal-jump-bottom = Jump to the last buffer line.
keymap-actions-vim-normal-delete-char = Delete the character under the cursor.
keymap-actions-vim-normal-replace-char = Replace the character under the cursor.
keymap-actions-vim-normal-repeat-last-change = Repeat the last complete edit.
keymap-actions-vim-normal-substitute-char = Delete the character under the cursor and enter insert mode.
keymap-actions-vim-normal-delete-to-line-end = Delete from cursor to end of line.
keymap-actions-vim-normal-change-to-line-end = Change from cursor to end of line and enter insert mode.
keymap-actions-vim-normal-yank-line = Yank the entire line.
keymap-actions-vim-normal-paste-after = Paste after the cursor.
keymap-actions-vim-normal-start-delete-operator = Begin a delete operator and wait for a motion.
keymap-actions-vim-normal-start-yank-operator = Begin a yank operator and wait for a motion.
keymap-actions-vim-normal-start-change-operator = Begin a change operator and wait for a motion or text object.
keymap-actions-vim-normal-undo = Undo the last complete edit.
keymap-actions-vim-normal-redo = Redo the last undone edit.
keymap-actions-vim-normal-cancel-operator = Cancel a pending Vim operator.
keymap-actions-vim-search-forward = Search forward in the active buffer.
keymap-actions-vim-search-backward = Search backward in the active buffer.
keymap-actions-vim-search-next = Repeat the accepted search.
keymap-actions-vim-search-previous = Repeat the search in the opposite direction.
keymap-actions-vim-operator-delete-line = Repeat delete operator to delete the whole line.
keymap-actions-vim-operator-yank-line = Repeat yank operator to yank the whole line.
keymap-actions-vim-operator-motion-left = Operator motion left.
keymap-actions-vim-operator-motion-right = Operator motion right.
keymap-actions-vim-operator-motion-up = Operator motion up.
keymap-actions-vim-operator-motion-down = Operator motion down.
keymap-actions-vim-operator-motion-word-forward = Operator motion to start of next word.
keymap-actions-vim-operator-motion-word-backward = Operator motion to start of previous word.
keymap-actions-vim-operator-motion-word-end = Operator motion to end of word.
keymap-actions-vim-operator-motion-line-start = Operator motion to line start.
keymap-actions-vim-operator-motion-line-end = Operator motion to line end.
keymap-actions-vim-operator-motion-find-forward = Operator motion to the next character on the current line.
keymap-actions-vim-operator-motion-find-backward = Operator motion to the previous character on the current line.
keymap-actions-vim-operator-motion-till-forward = Stop before the next character on the current line.
keymap-actions-vim-operator-motion-till-backward = Stop after the previous character on the current line.
keymap-actions-vim-operator-motion-jump-top = Operator motion to the first buffer line.
keymap-actions-vim-operator-motion-jump-bottom = Operator motion to the last buffer line.
keymap-actions-vim-operator-select-inner-text-object = Select an inner text object.
keymap-actions-vim-operator-select-around-text-object = Select an around text object.
keymap-actions-vim-operator-cancel = Cancel the pending operator.
keymap-actions-vim-text-object-word = Target the current word.
keymap-actions-vim-text-object-big-word = Target the current WORD.
keymap-actions-vim-text-object-parentheses = Target enclosing parentheses.
keymap-actions-vim-text-object-brackets = Target enclosing brackets.
keymap-actions-vim-text-object-braces = Target enclosing braces.
keymap-actions-vim-text-object-double-quote = Target enclosing double quotes.
keymap-actions-vim-text-object-single-quote = Target enclosing single quotes.
keymap-actions-vim-text-object-backtick = Target enclosing backticks.
keymap-actions-vim-text-object-cancel = Cancel the pending text object.
keymap-actions-pager-scroll-up = Scroll up by one row.
keymap-actions-pager-scroll-down = Scroll down by one row.
keymap-actions-pager-page-up = Scroll up by one page.
keymap-actions-pager-page-down = Scroll down by one page.
keymap-actions-pager-half-page-up = Scroll up by half a page.
keymap-actions-pager-half-page-down = Scroll down by half a page.
keymap-actions-pager-jump-top = Jump to the beginning.
keymap-actions-pager-jump-bottom = Jump to the end.
keymap-actions-pager-close = Close the pager overlay.
keymap-actions-pager-close-transcript = Close the transcript overlay.
keymap-actions-pager-find = Find text in a transcript pager.
keymap-actions-pager-mark-markdown = Mark table cells in transcript selection mode.
keymap-actions-list-move-up = Move list selection up.
keymap-actions-list-move-down = Move list selection down.
keymap-actions-list-move-left = Move horizontally left in list pickers.
keymap-actions-list-move-right = Move horizontally right in list pickers.
keymap-actions-list-page-up = Move list selection up by one page.
keymap-actions-list-page-down = Move list selection down by one page.
keymap-actions-list-jump-top = Jump to the first list item.
keymap-actions-list-jump-bottom = Jump to the last list item.
keymap-actions-list-accept = Accept the current list selection.
keymap-actions-list-cancel = Cancel and close selection views.
keymap-actions-agents-resume = Open the session resume picker.
keymap-actions-agents-search = Search the available agent tasks.
keymap-actions-agents-new-task = Open a new session in the selected checkout.
keymap-actions-agents-new-worktree = Open a new session in a worktree from the project default branch.
keymap-actions-agents-fork = Fork the selected conversation and open the new session.
keymap-actions-agents-rename = Rename the selected task.
keymap-actions-agents-stop = Stop the selected running task.
keymap-actions-agents-archive = Archive the selected task and its child agents.
keymap-actions-agents-delete = Permanently delete the selected task and its child agents.
keymap-actions-agents-hide = Hide the selected task until explicitly resumed.
keymap-actions-agents-toggle-grouping = Cycle task grouping by project, directory, status, or model.
keymap-actions-approval-open-fullscreen = Open approval details fullscreen.
keymap-actions-approval-open-thread = Open the approval source thread when available.
keymap-actions-approval-approve = Approve the primary option.
keymap-actions-approval-approve-for-session = Approve for the session when available.
keymap-actions-approval-approve-for-prefix = Approve with an exec-policy prefix when available.
keymap-actions-approval-deny = Choose the explicit deny option when available.
keymap-actions-approval-decline = Decline and provide corrective guidance.
keymap-actions-approval-cancel = Cancel an elicitation request.

## keymap/leader.rs

leader-needs-modifier = Invalid `tui.keymap.global.leader` = `{ $label }`: use ctrl, alt, super, or a non-character key so ordinary text input is not intercepted. Ctrl-alt characters may be AltGr input on Windows.
leader-ctrl-z-reserved = Invalid `tui.keymap.global.leader` = `{ $label }`: ctrl-z is reserved for suspending the terminal on Unix.
leader-key-reserved = Invalid `tui.keymap.global.leader` = `{ $label }`: this key is reserved by `{ $reserved_action }`.

## keymap/chords.rs

chords-invalid-leader-binding = Invalid `{ $path }` = `{ $raw }`. Use `leader` followed by one key, such as `leader c`.
chords-invalid-chord-binding = Invalid `{ $path }` = `{ $raw }`. Use a single key such as `ctrl-a` or a two-stroke chord such as `ctrl-x ctrl-t`.
chords-too-many-leader-combinations = Too many leader shortcut combinations (maximum { $max }). Reduce `tui.keymap.global.leader` alternatives or `leader ...` bindings.
chords-prefix-shadows = Ambiguous `{ $path }` = `{ $spec }`: its prefix shadows `{ $other }`. Unbind or remap the existing shortcut before using it as a chord prefix.
chords-chord-already-assigned = Ambiguous `{ $path }` = `{ $spec }`: the same chord is already assigned to `{ $other }`. Choose a unique chord and retry.
chords-altgr-prefix = Invalid `{ $path }` = `{ $spec }`: a ctrl-alt character prefix may be AltGr text input on Windows. Choose a different chord and retry.
chords-plain-prefix = Invalid `{ $path }` = `{ $spec }`: a chord prefix outside Vim or the command center must use ctrl, alt, super, or a non-character key so ordinary text input is not intercepted.
chords-ctrl-z-reserved = Ambiguous `{ $path }` = `{ $spec }`: `ctrl-z` is reserved for suspending the terminal on Unix. Choose a different chord and retry.
chords-key-reserved = Ambiguous `{ $path }` = `{ $spec }`: the chord uses the key reserved by `{ $reserved_action }`. Choose a different chord and retry.

## keymap_setup/actions.rs

keymap-actions-binding-resolved = { $spec } ({ $resolved })
keymap-actions-unbound = unbound

## codex-rs/tui/src/keymap_setup/capture.rs
capture-remap-shortcut = Remap Shortcut
capture-action = Action:{" "}
capture-current = Current:{" "}
capture-press-new-key = Press the new key now. Esc cancels.
capture-press-first-then-second = Press the first key, then the second. Esc cancels.
capture-first-key-press-second = First key: { $first }. Press the second key. Esc cancels.
capture-press-after-leader = Press the key that follows the configured leader. Esc cancels.
capture-error-prefix = Error:{" "}

## codex-rs/tui/src/keymap_setup/debug.rs
debug-tip-inspect-keys = Tip: Codex can only inspect keys your terminal sends.
debug-still-waiting = Still waiting? If nothing changes when you press a key, your terminal is not sending that key to Codex. Only received keys can be assigned as shortcuts.
debug-keypress-inspector = Keypress Inspector
debug-press-any-key = Press any key to see what Codex receives. Esc is inspected; Ctrl+C closes.
debug-waiting-keypress = Waiting for a keypress...
debug-detected = Detected:{" "}
debug-config-key = Config key:{" "}
debug-unsupported = unsupported - { $error }
debug-raw-event = Raw event:{" "}
debug-assigned-actions = Assigned actions:
debug-none = {"  "}none

## codex-rs/tui/src/keymap_setup/picker.rs
picker-tab-app = App
picker-tab-app-desc = Global and chat-level shortcuts.
picker-tab-composer = Composer
picker-tab-composer-desc = Composer submission and queue shortcuts.
picker-tab-editor = Editor
picker-tab-editor-desc = Inline editor movement and editing shortcuts.
picker-tab-vim = Vim
picker-tab-vim-desc = Vim normal-mode and operator shortcuts.
picker-tab-navigation = Navigation
picker-tab-navigation-desc = Pager and selection-list navigation shortcuts.
picker-tab-agents = Agents
picker-tab-agents-desc = Shared agents dashboard shortcuts.
picker-tab-approval = Approval
picker-tab-approval-desc = Approval prompt shortcuts.
picker-tab-all = All
picker-all-desc = All configurable shortcuts.
picker-all-summary = { $total } actions, { $custom_count } customized, { $unbound_count } unbound.
picker-no-shortcuts = No shortcuts available
picker-no-configurable-shortcuts = No configurable shortcuts are available
picker-tab-common = Common
picker-common-desc = Frequently customized shortcuts.
picker-no-common = No common shortcuts
picker-no-common-actions = No common shortcut actions are available
picker-tab-customized = Customized ({ $custom_count })
picker-customized-desc = Root-level shortcut overrides.
picker-no-customized = No customized shortcuts
picker-no-overrides = No root-level keymap overrides have been configured
picker-tab-unbound = Unbound ({ $unbound_count })
picker-unbound-desc = Actions without an active shortcut.
picker-no-unbound = No unbound shortcuts
picker-all-bound = Every configurable action currently has a shortcut
picker-no-group-shortcuts = No shortcuts in this group
picker-no-group-actions = No configurable actions are available in this group
picker-search-shortcuts = Type to search shortcuts
picker-tab-debug = Debug
picker-debug-desc = Inspect keypresses from your terminal.
picker-debug-summary = See the key Codex detects and any shortcuts assigned to it.
picker-inspect-keypresses = Inspect keypresses
picker-inspect-hint = Press Enter, then any key to inspect it (Ctrl+C exits)
picker-inspect-selected-hint = Open a live inspector that shows the detected key, config key, and matching actions
picker-keymap-title = Keymap
picker-one-action = 1 action.
picker-n-actions = { $count } actions.
picker-hint-group = {" "}group ·{" "}
picker-hint-edit = {" "}edit shortcut ·{" "}
picker-hint-custom = {" "}custom ·{" "}
picker-hint-unbound = {" "}unbound ·{" "}
picker-hint-close = {" "}close
picker-hint-start-inspector = {" "}start inspector ·{" "}

## codex-rs/tui/src/chatwidget.rs
chatwidget-memories-enable-notice = Memories will be enabled in the next session.
chatwidget-feedback-unavailable-cloud = Feedback is unavailable in Cloud threads.
chatwidget-feedback-uploads-disabled = Feedback uploads are disabled, and saving a ZIP requires a local app server.
chatwidget-feature-subagents = Subagents
chatwidget-feature-subagents-lower = subagents
chatwidget-feature-memories = Memories
chatwidget-feature-memories-lower = memories
chatwidget-enable-feature = Enable { $name }?
chatwidget-feature-disabled = { $label } are disabled in this TUI session.
chatwidget-learn-more = Learn more:{" "}
chatwidget-yes-enable = Yes, enable
chatwidget-enable-for-new-threads = Save on the server for new threads without changing this thread
chatwidget-not-now = Not now
chatwidget-keep-disabled = Keep { $name } disabled
chatwidget-code-review-started = >> Code review started: { $hint } <<
chatwidget-code-review-finished = << Code review finished >>
chatwidget-stopping-bg-terminals = Stopping all background terminals.
chatwidget-raw-output-on = Raw output mode on: transcript text is shown for clean terminal selection.
chatwidget-raw-output-off = Raw output mode off: rich transcript rendering restored.
chatwidget-vim-enabled = Vim mode enabled.
chatwidget-vim-disabled = Vim mode disabled.

## codex-rs/tui/src/chatwidget/backend_banners.rs
backend-banners-usage-limit-reached = Usage limit reached
backend-banners-usage-exhausted = Your included usage is exhausted. Choose an option below to continue.
backend-banners-continue-luna-reserve = Continue with Luna Reserve
backend-banners-to-confirm = to confirm
backend-banners-to-continue-working = to continue working

## codex-rs/tui/src/chatwidget/clipboard.rs
clipboard-copy-failed = Copy failed: { $error }

## codex-rs/tui/src/chatwidget/command_lifecycle.rs
command-lifecycle-waiting-bg-terminal = Waiting for background terminal

## codex-rs/tui/src/chatwidget/compaction.rs
compaction-working = Working
compaction-context-compacted = Context compacted
compaction-context-compacted-elapsed = Context compacted · { $elapsed }

## codex-rs/tui/src/chatwidget/connectors.rs
connectors-apps-disabled = Apps are disabled.
connectors-enable-apps-feature = Enable the apps feature to use $ or /apps.
connectors-no-apps = No apps available.
connectors-apps = Apps
connectors-loading-installed-available = Loading installed and available apps...
connectors-loading-apps = Loading apps...
connectors-updates-when-ready = This updates when the full list is ready
connectors-failed-to-load = Failed to load apps.
connectors-directory-unavailable = App directory unavailable
connectors-retry-directory-hint = Retry the failed app directory request, or press Esc to continue
connectors-retry = Retry
connectors-reload-apps = Reload installed and available apps
connectors-use-dollar = Use $ to insert an installed app into your prompt.
connectors-installed-of-total = Installed { $installed } of { $total } available apps.
connectors-status-open-manage = { $status_label } · Press Enter to open the app page to install, manage, or enable/disable this app
connectors-status-open-install = { $status_label } · Press Enter to open the app page to install this app
connectors-status-link-unavailable = { $status_label } · App link unavailable
connectors-manage-in-browser = Manage this app in your browser.
connectors-install-in-browser = Install this app in your browser, then reload Codex.
connectors-search-apps = Type to search apps
connectors-status-description = { $status_label } · { $description }
connectors-status-installed = Installed
connectors-status-installed-disabled = Installed · Disabled
connectors-status-can-install = Can be installed

## codex-rs/tui/src/chatwidget/copy_picker.rs
copy-picker-whole-status = Whole status
copy-picker-whole-response = Whole response
copy-picker-code-block = Code block
copy-picker-language-code = { $language } code
copy-picker-blockquote = Blockquote
copy-picker-copy-to-clipboard = Copy to clipboard

## codex-rs/tui/src/chatwidget/goal_menu.rs
goal-menu-edit-goal = Edit goal
goal-menu-type-objective = Type a goal objective and press Enter
goal-menu-resume-paused-title = Resume paused goal?
goal-menu-goal-objective = Goal: { $objective }
goal-menu-resume-goal = Resume goal
goal-menu-resume-goal-desc = Mark it active and continue when idle
goal-menu-leave-paused = Leave paused
goal-menu-leave-paused-desc = Keep it paused; use /goal resume later
goal-menu-goal = Goal
goal-menu-status = Status:{" "}
goal-menu-objective = Objective:{" "}
goal-menu-time-used = Time used:{" "}
goal-menu-tokens-used = Tokens used:{" "}
goal-menu-token-budget = Token budget:{" "}
goal-menu-commands-active = Commands: /goal edit, /goal pause, /goal clear
goal-menu-commands-paused = Commands: /goal edit, /goal resume, /goal clear
goal-menu-commands-done = Commands: /goal edit, /goal clear
goal-menu-status-active = active
goal-menu-status-paused = paused
goal-menu-status-stalled = stalled
goal-menu-status-usage-limited = usage limited
goal-menu-status-budget-limited = limited by budget
goal-menu-status-complete = complete

## codex-rs/tui/src/chatwidget/hooks.rs
hooks-failed-to-load = Failed to load hooks: { $err }

## codex-rs/tui/src/chatwidget/ide_context.rs
ide-context-off = IDE context is off.
ide-context-usage = Usage: /ide [on|off|status]
ide-context-skipped = IDE context was skipped for this message.
ide-context-on = IDE context is on.
ide-context-future-messages = Future messages will include your current IDE selection and open tabs.
ide-context-connected = Connected to your IDE.
ide-context-enable-failed = IDE context could not be enabled.

## codex-rs/tui/src/chatwidget/image_submission.rs
image-submission-failed-images = Failed to prepare images: { $error }
image-submission-too-large = image attachments exceed the 32 MiB transport limit
image-submission-failed-image = Failed to prepare image: { $path }: { $error }

## codex-rs/tui/src/chatwidget/input_flow.rs
input-flow-preparing-images = Preparing images: { $text }
input-flow-cannot-switch-mode = Cannot switch collaboration mode while a turn is running.

input-flow-working = Working
## codex-rs/tui/src/chatwidget/input_restore.rs
input-restore-interrupted-for-steer = Model interrupted to submit steer instructions.
input-restore-goal-budget-reached = Goal budget reached - the turn was stopped.
input-restore-waiting-startup = Waiting for startup
input-restore-cancel = cancel

## codex-rs/tui/src/chatwidget/input_submission.rs
input-submission-model-unavailable = Thread model is unavailable. Wait for the thread to finish syncing or choose a model before sending input.
input-submission-shell-help-title = Prefix a command with ! to run it locally
input-submission-shell-help-hint = Example: !ls

## codex-rs/tui/src/chatwidget/luna_reserve_model.rs
luna-reserve-settings-unavailable = Luna model settings are unavailable; please try /model again in a moment.
luna-reserve-select-model = Select Model
luna-reserve-other-models-return = Other models return when ordinary usage is available again.

## codex-rs/tui/src/chatwidget/mcp_startup.rs
mcp-startup-multi-header-prefix = Starting MCP servers
mcp-startup-single-header-prefix = Booting MCP server:
mcp-startup-interrupted = MCP startup interrupted. The following servers were not initialized: { $servers }
mcp-startup-incomplete = MCP startup incomplete (failed: { $servers })
mcp-startup-server-error = { $server }: { $error }
mcp-startup-client-failed = MCP client for `{ $server }` failed to start

## codex-rs/tui/src/chatwidget/misalignment_policy.rs
misalignment-policy-title = Chat stopped as a precaution
misalignment-policy-description = We couldn’t confirm the agent was acting safely and following your instructions. To continue working, start or resume another chat.
misalignment-policy-new-chat = New chat
misalignment-policy-resume-chat = Resume another chat
misalignment-policy-review-findings = Review findings
misalignment-policy-command-center = Agent command center
misalignment-policy-paused-title = Chat paused as a precaution
misalignment-policy-paused-description = We couldn’t confirm the agent was interpreting your instructions correctly. Review what we detected before deciding to continue.
misalignment-policy-acknowledge = Acknowledge findings and continue
misalignment-policy-back = Back

## codex-rs/tui/src/chatwidget/model_command.rs
model-command-unexpected-argument = Unexpected argument '{ $unexpected }'. Use '--reasoning=<level>' or '--speed=<tier>'.
model-command-reasoning-unavailable = Reasoning level '{ $effort }' is unavailable for { $model }. Available: { $available }.
model-command-speed-unavailable = Speed '{ $speed }' is unavailable for { $model }. Available: { $available }.
model-command-session-starting = Session is still starting; try /model again in a moment.
model-command-reserve-args-unavailable = Model arguments are unavailable while using Reserve. Use /model to adjust effort.
model-command-model-unavailable = Model '{ $model_name }' is unavailable. Use /model to see available models.

## codex-rs/tui/src/chatwidget/model_popup_state.rs
model-popup-state-list-stale = The model list may be out of date. Restart the app server and TUI to refresh /model.
model-popup-state-saving-provider = Saving the default provider. Try /model after the save finishes.
model-popup-state-load-failed = Could not load models. Try /model again.

## codex-rs/tui/src/chatwidget/model_popups.rs
model-popups-disabled-until-startup = Model selection is disabled until startup completes.
model-popups-being-updated = Models are being updated; please try /model again in a moment.
model-popups-loading-models = Loading models…
model-popups-server-effort = server effort
model-popups-accepted-future-turns = Accepted for future turns: { $model } · { $effort }
model-popups-base-url-warning = Warning: OpenAI base URL is overridden to { $base_url }. Selecting models may not be supported or work properly.
model-popups-all-models = All models
model-popups-choose-specific = Choose a specific model and reasoning level (current: { $current_label })
model-popups-select-model = Select Model
model-popups-pick-quick-mode = Pick a quick auto mode or browse all models.
model-popups-no-additional = No additional models are available right now.
model-popups-select-model-effort = Select Model and Effort
model-popups-no-reasoning = no reasoning
model-popups-n-reasoning = { $label } reasoning
model-popups-selected-reasoning = the selected reasoning
model-popups-always-use-in-plan = Always use { $reasoning_phrase } in Plan mode
model-popups-user-plan-override = user-chosen Plan override ({ $label })
model-popups-builtin-plan-default = built-in Plan default ({ $label })
model-popups-builtin-plan-default-none = built-in Plan default (no reasoning)
model-popups-builtin-plan-default-plain = built-in Plan default
model-popups-all-modes-description = Set the global default reasoning level and Plan mode override, replacing the current { $plan_reasoning_source }
model-popups-choose-where-apply = Choose where to apply { $reasoning_phrase }.
model-popups-apply-plan-override = Apply to Plan mode override
model-popups-apply-global-and-plan = Apply to global default and Plan mode override
model-popups-apply-reasoning-change = Apply reasoning change
model-popups-plus-rate-limit-warning = ⚠ { $effort_label } reasoning effort can quickly consume Plus plan rate limits
model-popups-default-suffix = {" "}(default)
model-popups-and = {" "}and{" "}
model-popups-more-reasoning = More reasoning…
model-popups-advanced-consumes =
    { $count ->
        [one] { $label } consumes usage limits faster
       *[other] { $label } consume usage limits faster
    }
model-popups-select-reasoning-level = Select Reasoning Level for { $model_label }
model-popups-max-description = For difficult problems when quality matters more than speed · higher usage
model-popups-ultra-description = For demanding work using multiple agents · highest usage
model-popups-advanced-reasoning = Advanced Reasoning
model-popups-consumes-faster = ⚠ Consumes usage limits faster
model-popups-effort-none = None
model-popups-effort-minimal = Minimal
model-popups-effort-low = Low
model-popups-effort-medium = Medium
model-popups-effort-high = High
model-popups-effort-xhigh = Extra high
model-popups-effort-max = Max
model-popups-effort-ultra = Ultra
model-popups-effort-persistent = Persistent
model-popups-ultra-concurrency-warning = Ultra reasoning may proactively use multiple agents. This session is configured for { $max_threads } concurrent threads with up to { $max_subagents } subagents which can increase usage quickly. Consider setting features.multi_agent_v2.max_concurrent_threads_per_session below 8.

## codex-rs/tui/src/chatwidget/notifications.rs
notifications-agent-turn-complete = Agent turn complete
notifications-approval-requested = Approval requested: { $subject }
notifications-wants-to-edit = Codex wants to edit { $target }
notifications-n-files = { $count } files
notifications-approval-requested-by = Approval requested by { $server_name }
notifications-plan-mode-prompt = Plan mode prompt: { $title }
notifications-question = Question: { $title }

## codex-rs/tui/src/chatwidget/permission_discovery.rs
permission-discovery-unknown-policy = Unknown approval policy `{ $value }`. Expected `untrusted`, `on-request`, `never`, or a granular policy.
permission-discovery-unknown-reviewer = Unknown permission reviewer `{ $value }`. Expected `user` or `auto_review`.
permission-discovery-usage = Usage: /permissions <profile-id> [--approval-policy=<policy>] [--reviewer=user|auto_review]
permission-discovery-unexpected-argument = Unexpected permission argument `{ $argument }`.
permission-discovery-update-title = Update Model Permissions
permission-discovery-loading-profiles = Loading permission profiles…
permission-discovery-retry = Retry

## codex-rs/tui/src/chatwidget/permission_popups.rs
permission-popups-non-admin-sandbox = { $label } (non-admin sandbox)
permission-popups-non-admin-sandbox-note = The non-admin sandbox protects your files and prevents network access under most circumstances. However, it carries greater risk if prompt injected. To upgrade to the default sandbox, run{" "}
permission-popups-update-title = Update Model Permissions
permission-popups-no-denials = No recent auto-review denials in this thread.
permission-popups-denials-recorded = Denials are recorded after auto-review rejects an action.
permission-popups-thread-unavailable = That thread is no longer available.
permission-popups-action = Action
permission-popups-rationale = Rationale
permission-popups-no-rationale = Auto-review did not include a rationale
permission-popups-denials-title = Auto-review Denials
permission-popups-select-denied = Select a denied action to approve.
permission-popups-denial-unavailable = That auto-review denial is no longer available.
permission-popups-approval-recorded = Approval recorded for one retry of the selected auto-review denial.
permission-popups-approval-context = The model will see the approval context; the retry still goes through auto-review.
permission-popups-permissions-updated = Permissions updated to { $label }
permission-popups-enable-full-access = Enable full access?
permission-popups-recommend-approve-for-me = We strongly recommend selecting "Approve for me" instead, and customizing the reviewer policy for your use case.
permission-popups-recommend-ask = We strongly recommend selecting "Ask for approval" instead.
permission-popups-full-access-warning = When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval.
permission-popups-cyber-risk = Cyber models carry a higher risk of dangerous actions.
permission-popups-cyber-safeguards = {" "}Ensure proper safeguards are in place before granting full access.{" "}
permission-popups-full-access-warning-sp = When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval.{" "}
permission-popups-exercise-caution = Exercise caution when enabling full access. This significantly increases the risk of data loss, leaks, or unexpected behavior.
permission-popups-yes-continue = Yes, continue anyway
permission-popups-apply-full-access = Apply full access for this session
permission-popups-cancel = Cancel
permission-popups-go-back = Go back without enabling full access

## codex-rs/tui/src/chatwidget/permission_shortcuts.rs
permission-shortcuts-none-available = No other permission modes are available.

## codex-rs/tui/src/chatwidget/permissions_menu.rs
permissions-menu-read-only-desc = Read workspace files, with approval required for edits or internet access
permissions-menu-auto-desc = Read and edit workspace files and run commands, with approval required for internet access or edits outside the workspace
permissions-menu-full-access-desc = Use with caution: Codex can edit files outside this workspace and access the internet without approval
permissions-menu-approve-for-me-unavailable = Approve for me is unavailable. Run `/permissions` to see available profiles.
permissions-menu-unknown-profile = Unknown permission profile `{ $profile_id }`. Run `/permissions` to see available profiles.
permissions-menu-profile-unavailable = Permission profile `{ $profile_id }` is unavailable: { $reason }.
permissions-menu-configured-profile = Configured permission profile
permissions-menu-current-profile = Current permission profile
permissions-menu-not-available-on-server = Not available on this server
permissions-menu-title = Update Model Permissions
permissions-menu-no-profiles = No permission profiles returned by the server.

## codex-rs/tui/src/chatwidget/pets.rs
pets-unavailable = Pets unavailable
pets-close = Close
pets-failed-to-load-configured = Failed to load configured pet: { $err }
pets-loading-title = Loading Pet
pets-preparing = Preparing the terminal pet.
pets-loading-selected = Loading selected pet...
pets-failed-to-start-runtime = failed to start pet load runtime: { $err }

## codex-rs/tui/src/chatwidget/plan_implementation.rs
plan-implementation-default-unavailable = Default mode unavailable
plan-implementation-no-approved-plan = No approved plan available
plan-implementation-fresh-thread = Fresh thread with this plan
plan-implementation-fresh-thread-context = Start a fresh thread (current context: { $label })
plan-implementation-title = Implement this plan?
plan-implementation-yes = Yes, implement this plan
plan-implementation-yes-desc = Switch to Default and start coding
plan-implementation-clear-context = Yes, clear context and implement
plan-implementation-no = No, stay in Plan mode
plan-implementation-no-desc = Continue planning with the model

## codex-rs/tui/src/chatwidget/plugin_catalog.rs
plugin-catalog-openai-curated-loading-desc = This updates when OpenAI Curated plugins finish loading
plugin-catalog-remove-marketplace-question = Remove { $name } marketplace?
plugin-catalog-removing-name = Removing { $name }...
plugin-catalog-upgrading-name-marketplace = Upgrading { $name } marketplace...
plugin-catalog-upgrading-marketplaces = Upgrading marketplaces...
plugin-catalog-loading-details-for = Loading details for { $name }...
plugin-catalog-installing-name = Installing { $name }...
plugin-catalog-uninstalling-name = Uninstalling { $name }...
plugin-catalog-add-marketplace = Add marketplace
plugin-catalog-installed-of-available = Installed { $installed } of { $total } available plugins.
plugin-catalog-browse-plugins = Browse plugins from available marketplaces.
plugin-catalog-installed-tab = Installed ({ $installed })
plugin-catalog-showing-installed = Showing { $installed } installed plugins.
plugin-catalog-installed-plugins = Installed plugins.
plugin-catalog-installed-of-curated = Installed { $installed } of { $total } OpenAI Curated plugins.
plugin-catalog-openai-curated-marketplace = OpenAI Curated marketplace.
plugin-catalog-label-installed-successfully = { $label } installed successfully.
plugin-catalog-select-plugins-to-use = Select the plugins you want to use and press Enter to install or view details.
plugin-catalog-label-period = { $label }.
plugin-catalog-installed-of-label = Installed { $installed } of { $total } { $label } plugins.
plugin-catalog-hint-upgrade = { $upgrade } upgrade
plugin-catalog-hint-remove = { $remove } remove
plugin-catalog-hint-tab-toggle = tab toggle
plugin-catalog-hint-tabs = ←/→ tabs
plugin-catalog-hint-enter-details = enter details
plugin-catalog-hint-esc-close = esc close
plugin-catalog-toggle-disable = disable
plugin-catalog-toggle-enable = enable
plugin-catalog-status-tab-to-toggle-enter = { $status }   Tab to { $action }; Enter view details
plugin-catalog-status-tab-to-toggle = { $status }   Tab to { $action }
plugin-catalog-status-press-enter-details = { $status }   Press Enter to view plugin details
plugin-catalog-status-details-unavailable = { $status }   Plugin details are unavailable
plugin-catalog-status-press-enter-install = { $status }   Press Enter to install or view plugin details
plugin-catalog-status-remote-unavailable = { $status }   Remote plugin details are not available yet
plugin-catalog-source-remote = Remote · { $label }
plugin-catalog-version-local = local { $version }
plugin-catalog-version-remote = remote { $version }
plugin-catalog-remote-id = Remote ID { $id }
plugin-catalog-creator-name-account = creator { $name } ({ $account_id })
plugin-catalog-creator-name = creator { $name }
plugin-catalog-creator-account = creator account { $account_id }
plugin-catalog-no-explicit-principals = No explicit principals
plugin-catalog-one-principal = 1 principal: { $name }
plugin-catalog-n-principals = { $count } principals
plugin-catalog-loading-label-plugins-dots = Loading { $label } plugins...
plugin-catalog-label-unavailable = { $label } unavailable
plugin-catalog-loading-label-plugins = Loading { $label } plugins.
plugin-catalog-local-functionality-available = Local plugin functionality is already available.
plugin-catalog-section-loaded = This section loaded successfully.
plugin-catalog-label-unavailable-period = { $label } unavailable.
plugin-catalog-local-functionality-still = Local plugin functionality is still available.
plugin-catalog-details-unavailable-reason = plugin details are unavailable
plugin-catalog-openai-curated = OpenAI Curated
plugin-catalog-workspace = Workspace
plugin-catalog-shared-with-me = Shared with me
plugin-catalog-shared-with-me-link = Shared with me (link)
plugin-catalog-local = Local
plugin-catalog-this-updates-when-workspace-plugins-f = This updates when workspace plugins finish loading
plugin-catalog-no-workspace-plugins-available = No workspace plugins available
plugin-catalog-no-workspace-directory-plugins-are-av = No workspace directory plugins are available
plugin-catalog-this-updates-when-shared-plugins-fini = This updates when shared plugins finish loading
plugin-catalog-no-shared-plugins-available = No shared plugins available
plugin-catalog-no-plugins-have-been-shared-with = No plugins have been shared with you
plugin-catalog-plugins = Plugins
plugin-catalog-loading-available-plugins = Loading available plugins...
plugin-catalog-this-updates-when-the-marketplace-lis = This updates when the marketplace list is ready.
plugin-catalog-loading-plugins = Loading plugins...
plugin-catalog-this-updates-when-the-marketplace-lis-2 = This updates when the marketplace list is ready
plugin-catalog-adding-marketplace = Adding marketplace...
plugin-catalog-this-updates-when-marketplace-install = This updates when marketplace installation completes
plugin-catalog-this-removes-the-configured-marketpla = This removes the configured marketplace from Codex.
plugin-catalog-select = {" "}select
plugin-catalog-close = {" "}close
plugin-catalog-remove-marketplace = Remove marketplace
plugin-catalog-remove-this-marketplace-from-the-avai = Remove this marketplace from the available plugin list
plugin-catalog-back-to-plugins = Back to plugins
plugin-catalog-keep-this-marketplace-installed = Keep this marketplace installed
plugin-catalog-removing-marketplace = Removing marketplace...
plugin-catalog-this-updates-when-marketplace-removal = This updates when marketplace removal completes
plugin-catalog-this-updates-when-marketplace-upgrade = This updates when marketplace upgrade completes
plugin-catalog-loading-plugin-details = Loading plugin details...
plugin-catalog-this-updates-when-plugin-details-load = This updates when plugin details load
plugin-catalog-installing-plugin = Installing plugin...
plugin-catalog-this-updates-when-plugin-installation = This updates when plugin installation completes
plugin-catalog-uninstalling-plugin = Uninstalling plugin...
plugin-catalog-this-updates-when-the-plugin-removal = This updates when the plugin removal completes
plugin-catalog-failed-to-load-plugins = Failed to load plugins.
plugin-catalog-plugin-marketplace-unavailable = Plugin marketplace unavailable
plugin-catalog-marketplace-add-failed = Marketplace add failed
plugin-catalog-failed-to-add-marketplace-from-the = Failed to add marketplace from the provided source
plugin-catalog-try-again = Try again
plugin-catalog-enter-a-marketplace-source = Enter a marketplace source
plugin-catalog-return-to-the-plugin-list = Return to the plugin list
plugin-catalog-failed-to-add-marketplace = Failed to add marketplace.
plugin-catalog-marketplace-removal-failed = Marketplace removal failed
plugin-catalog-failed-to-remove-the-selected-marketp = Failed to remove the selected marketplace
plugin-catalog-review-the-confirmation-prompt-again = Review the confirmation prompt again
plugin-catalog-failed-to-remove-marketplace = Failed to remove marketplace.
plugin-catalog-plugin-detail-unavailable = Plugin detail unavailable
plugin-catalog-failed-to-load-plugin-details = Failed to load plugin details.
plugin-catalog-no-marketplace-plugins-available = No marketplace plugins available
plugin-catalog-no-plugins-are-available-in-the = No plugins are available in the discovered marketplaces
plugin-catalog-all-plugins = All Plugins
plugin-catalog-no-installed-plugins = No installed plugins
plugin-catalog-loading-openai-curated-plugins = Loading OpenAI Curated plugins...
plugin-catalog-openai-curated-unavailable = OpenAI Curated unavailable
plugin-catalog-no-openai-curated-plugins-available = No OpenAI Curated plugins available
plugin-catalog-no-plugins-available-in-this-marketpl = No plugins available in this marketplace
plugin-catalog-type-to-search-plugins = Type to search plugins
plugin-catalog-add-marketplace-2 = Add Marketplace
plugin-catalog-add-a-marketplace-from-a-git = Add a marketplace from a Git repo or local root.
plugin-catalog-enter-a-source-to-make-its = Enter a source to make its plugins available in this menu.
plugin-catalog-enter-owner-repo-a-git-url = Enter owner/repo, a Git URL, or a local marketplace path
plugin-catalog-press-enter-to-enter-a-marketplace = Press Enter to enter a marketplace source
plugin-catalog-data-shared-with-this-app-is = Data shared with this app is subject to the app's{" "}
plugin-catalog-and = {" "}and{" "}
plugin-catalog-terms-of-service = terms of service
plugin-catalog-privacy-policy = privacy policy
plugin-catalog-period-space = {"."}{" "}
plugin-catalog-learn-more = Learn more
plugin-catalog-period = {"."}
plugin-catalog-installed-by-admin = Installed by admin
plugin-catalog-this-plugin-is-installed-by-your = This plugin is installed by your workspace admin
plugin-catalog-uninstall-plugin = Uninstall plugin
plugin-catalog-remove-this-plugin-now = Remove this plugin now
plugin-catalog-this-remote-plugin-did-not-provide = This remote plugin did not provide an uninstall identity
plugin-catalog-install-plugin = Install plugin
plugin-catalog-this-plugin-is-disabled-by-your = This plugin is disabled by your workspace admin
plugin-catalog-this-plugin-is-not-installable-from = This plugin is not installable from this marketplace
plugin-catalog-install-this-plugin-now = Install this plugin now
plugin-catalog-this-plugin-did-not-provide-an = This plugin did not provide an install location
plugin-catalog-skills = Skills
plugin-catalog-hooks = Hooks
plugin-catalog-apps = Apps
plugin-catalog-mcp-servers = MCP Servers
plugin-catalog-disabled-by-admin = Disabled by admin
plugin-catalog-enabled-by-admin = Enabled by Admin
plugin-catalog-installed = Installed
plugin-catalog-disabled = Disabled
plugin-catalog-not-installable = Not installable
plugin-catalog-can-be-installed = Can be installed
plugin-catalog-source = Source
plugin-catalog-auth = Auth
plugin-catalog-version = Version
plugin-catalog-sharing = Sharing
plugin-catalog-auth-on-install = Auth on install
plugin-catalog-auth-on-use = Auth on use
plugin-catalog-listed = Listed
plugin-catalog-workspace-link = Workspace link
plugin-catalog-private = Private
plugin-catalog-admin-assigned = Admin assigned
plugin-catalog-available = Available
plugin-catalog-no-plugin-skills = No plugin skills
plugin-catalog-no-plugin-apps = No plugin apps
plugin-catalog-no-plugin-hooks = No plugin hooks
plugin-catalog-no-plugin-mcp-servers = No plugin MCP servers

## codex-rs/tui/src/chatwidget/plugins.rs
plugins-disabled = Plugins are disabled.
plugins-disabled-hint = Enable the plugins feature to use /plugins.
plugins-add-marketplace = Add marketplace
plugins-add-marketplace-placeholder = owner/repo, git URL, or local marketplace path
plugins-add-marketplace-examples = Examples: owner/repo, git URL, ./marketplace
plugins-installed = Installed { $name } plugin.
plugins-no-auth-required = No additional app authentication is required.
plugins-apps-need-auth = { $count } app(s) still need authentication: { $app_names }
plugins-marketplace-already-added = Marketplace { $name } is already added.
plugins-marketplace-added = Added marketplace { $name }.
plugins-marketplace-root = Marketplace root: { $root }
plugins-marketplace-removed = Removed marketplace { $name }.
plugins-marketplace-config-removed = Removed marketplace config for { $name }.
plugins-no-git-marketplaces = No configured Git marketplaces to upgrade.
plugins-only-git-upgradable = Only configured Git marketplaces can be upgraded.
plugins-marketplace-up-to-date = Marketplace { $name } is already up to date.
plugins-marketplaces-all-up-to-date = Checked { $count } marketplaces; all are already up to date.
plugins-checked = Checked: { $names }
plugins-updated-roots = Updated roots: { $roots }
plugins-config-update-failed = Failed to update plugin config for { $plugin_id }: { $err }
plugins-uninstalled = Uninstalled { $name } plugin.
plugins-bundled-apps-remain = Bundled apps remain installed.
plugins-app-already-installed = Already installed in this session.
plugins-install-required-apps = Install the required Apps in ChatGPT to continue:
plugins-heading = Plugins
plugins-plugin-installed = { $name } plugin installed.
plugins-app-setup-progress = App setup { $current }/{ $total }: { $name }
plugins-manage-on-chatgpt = Manage on ChatGPT
plugins-install-on-chatgpt = Install on ChatGPT
plugins-open-app-management = Open the ChatGPT app management page
plugins-open-app-page = Open the app page in your browser
plugins-apps-link-unavailable = ChatGPT apps link unavailable
plugins-no-install-url = This app did not provide an install/manage URL
plugins-continue = Continue
plugins-app-already-installed-desc = This app is already installed
plugins-advance-next-app = Advance to the next app
plugins-ive-installed-it = I've installed it
plugins-trust-confirmation = Trust your confirmation and continue to the next app
plugins-continue-without-refresh = Continue without waiting for refresh to complete
plugins-skip-app-setup = Skip remaining app setup
plugins-stop-follow-up = Stop this follow-up flow for this plugin
plugins-abandon-app-setup = Abandon remaining required app setup
plugins-skipped-app-setup = Skipped remaining app setup for { $name } plugin.
plugins-may-not-be-usable = The plugin may not be usable until required apps are installed.
plugins-completed-app-setup = Completed app setup flow for { $name } plugin.
plugins-continue-managing = You can now continue managing plugins from /plugins.
plugins-upgraded = Upgraded { $count } { $count ->
        [one] marketplace
       *[other] marketplaces
    }.
plugins-upgrade-failed = Failed to upgrade { $count } { $count ->
        [one] marketplace
       *[other] marketplaces
    }: { $errors }

## codex-rs/tui/src/chatwidget/protocol.rs

protocol-signed-in-to = Signed in to { $name }.
protocol-mcp-sign-in-failed = MCP sign-in failed.
protocol-server-busy-reconnecting = Server is busy, reconnecting { $attempt }
protocol-extra-safety-checks = This request requires additional safety checks, some tool calls might take extra time

## codex-rs/tui/src/chatwidget/protocol_requests.rs

protocol-requests-localize-permission-paths = failed to localize requested filesystem paths: { $err }
protocol-requests-localize-guardian-paths = failed to localize guardian filesystem paths: { $err }

## codex-rs/tui/src/chatwidget/provider_error.rs

provider-error-continue-in-new-chat = {" "}continue in new chat ·{" "}
provider-error-start-fresh = {" "}start fresh ·{" "}
provider-error-other-chats = {" "}other chats
provider-error-chat-read-only = This chat uses { $provider }, but Codex now uses { $current }, so this chat is read-only.
provider-error-not-configured = This chat uses model provider “{ $provider }”, which is no longer configured. Restore this provider’s configuration to continue.
provider-error-policy-conflict = This chat uses model provider “{ $provider }”, but your organization requires “{ $required }”. Contact your administrator or start a new chat using the required provider.

## codex-rs/tui/src/chatwidget/questions.rs

questions-question-requested = Question requested
questions-n-requested = { $count } questions requested

## codex-rs/tui/src/chatwidget/quick_dials.rs

quick-dials-none-saved = No presets saved. Press n to create one.
quick-dials-hud-new = new
quick-dials-hud-from-settings = from settings
quick-dials-hud-edit = edit
quick-dials-hud-delete = delete
quick-dials-hud-close = close
quick-dials-presets = Presets
quick-dials-images-not-copied = Images cannot be copied into a preset. Starting with empty text.
quick-dials-mentions-not-copied = Linked mentions cannot be copied into a preset. Starting with empty text.
quick-dials-model-not-recallable = The current model cannot be recalled with /model.
quick-dials-choose-reasoning = Choose an available reasoning effort with /model before saving.
quick-dials-choose-speed = Choose an available speed with /model before saving.
quick-dials-choose-title-items = Choose available terminal-title items with /title before saving.
quick-dials-choose-status-items = Choose available status-line items with /statusline before saving.
quick-dials-unavailable = Presets are unavailable in this conversation.
quick-dials-after-startup = Presets are available after startup completes.
quick-dials-preset-reason = Preset { $slot }: { $reason }
quick-dials-empty = empty
quick-dials-save-to-slot = Save preset to slot
quick-dials-edit-a-preset = Edit a preset
quick-dials-delete-a-preset = Delete a preset
quick-dials-new-preset = New preset
quick-dials-edit-preset-n = Edit preset { $slot }
quick-dials-review-changes = review changes
quick-dials-choose-slot = choose slot
quick-dials-write-prompt = Write a prompt or slash command
quick-dials-approval-not-recallable = The approval policy cannot be recalled.
quick-dials-profile-not-recallable = The permission profile name cannot be recalled.
quick-dials-select-profile = Select a named permission profile with /permissions first.
quick-dials-current-model = Current model
quick-dials-current-permissions = Current permissions
quick-dials-current-title = Current terminal title
quick-dials-current-status = Current status line
quick-dials-start-from-settings = Start preset from current settings
quick-dials-replace-question = Replace preset { $slot }?
quick-dials-replace-name = Replace preset { $slot }
quick-dials-delete-question = Delete preset { $slot }?
quick-dials-delete-name = Delete preset { $slot }
quick-dials-save-question = Save preset { $slot }?
quick-dials-save-name = Save preset { $slot }

## codex-rs/tui/src/chatwidget/rate_limits.rs

rate-limits-heads-up = Heads up, you have less than { $remaining_percent }% of your { $limit_label } limit left. Run /status for a breakdown.
rate-limits-window-5h = 5h
rate-limits-window-daily = daily
rate-limits-window-weekly = weekly
rate-limits-window-monthly = monthly
rate-limits-window-annual = annual
rate-limits-fallback-secondary = secondary usage
rate-limits-fallback-primary = usage
rate-limits-no-longer-close = You’re no longer close to your usage limit.
rate-limits-dismissing-in = Dismissing in { $seconds ->
    [one] 1 second
   *[other] { $seconds } seconds
}…
rate-limits-dismiss = Dismiss
rate-limits-fewer-credits = Uses fewer credits for upcoming turns
rate-limits-switch-to = Switch to { $switch_model }
rate-limits-keep-model = Keep current model
rate-limits-keep-model-never = Keep current model (never show again)
rate-limits-hide-reminders = Hide future rate limit reminders about switching models
rate-limits-approaching = Approaching rate limits
rate-limits-switch-lower-usage = Switch to { $switch_model } for lower credit usage?
rate-limits-workspace-credit-limit = You've reached your workspace credit limit
rate-limits-workspace-out-of-credits = Your workspace is out of credits. Ask your workspace owner to add more. Notify owner?
rate-limits-usage-limit-reached = Usage limit reached
rate-limits-request-increase = Request a limit increase from your owner to continue using codex. Request increase?
rate-limits-yes = Yes
rate-limits-no-default = No (default)
rate-limits-owner-notified = Workspace owner notified.
rate-limits-owner-already-notified = Workspace owner was already notified recently.
rate-limits-owner-notify-failed = Could not notify your workspace owner. Please try again.
rate-limits-increase-requested = Limit increase requested.
rate-limits-increase-already-requested = A limit increase was already requested recently.
rate-limits-increase-request-failed = Could not request a limit increase. Please try again.

## codex-rs/tui/src/chatwidget/read_only.rs

read-only-external-writer = Close it there and press R to continue here.
read-only-archived = Press U to unarchive this thread and continue here.
read-only-retry = retry
read-only-unarchive = unarchive

## codex-rs/tui/src/chatwidget/realtime/session_lock.rs

session-lock-voice-stopped = Voice stopped because your session locked.
session-lock-monitoring-failed = Voice conversation stopped because session lock monitoring failed.

## codex-rs/tui/src/chatwidget/realtime/recording_controls.rs

recording-controls-voice-failed = Voice conversation failed: { $error }
recording-controls-mic-update-failed = Failed to update microphone: { $error }
recording-controls-start-voice-first = Start voice mode before muting the microphone.
recording-controls-connecting = connecting
recording-controls-muted = muted
recording-controls-heard = heard
recording-controls-speaking = speaking
recording-controls-listening = listening

## codex-rs/tui/src/chatwidget/realtime_settings.rs

realtime-settings-back = Back
realtime-settings-voice-settings = Voice settings
realtime-settings-set-sound-devices = Set sound devices
realtime-settings-choose-voice = Choose a voice
realtime-settings-select-voice = Select voice
realtime-settings-applies-next = Applies to your next voice conversation.
realtime-settings-input-device = Input device
realtime-settings-output-device = Output device
realtime-settings-system-default = System default
realtime-settings-sound-devices = Sound devices
realtime-settings-identical-names = Identical device names; use System default.
realtime-settings-input-channels = Input channels
realtime-settings-all-channels-mixed = All channels (mixed)
realtime-settings-inputs = Inputs { $channels }
realtime-settings-microphone = Microphone: { $name }
realtime-settings-select-channels = Select the input channels to mix. Select at least one.
realtime-settings-input-n = Input { $number }
realtime-settings-select-one-channel = Select at least one input channel.
realtime-settings-voice-set = Voice set to { $voice }. Applies to your next voice conversation.

## codex-rs/tui/src/chatwidget/reasoning_shortcuts.rs

reasoning-shortcuts-lowest = Reasoning is already at the lowest level ({ $label }).
reasoning-shortcuts-highest = Reasoning is already at the highest level ({ $label }).
reasoning-shortcuts-disabled-startup = Reasoning shortcuts are disabled until startup completes.
reasoning-shortcuts-unavailable = Reasoning shortcuts are unavailable for { $model }.
reasoning-shortcuts-all-models = All models → { $model }
reasoning-shortcuts-ultra-available = Ultra is available under /model → { $path } → More reasoning…

## codex-rs/tui/src/chatwidget/reconnect.rs

reconnect-reconnecting = Reconnecting to server…
reconnect-quit = quit
reconnect-message-with-attachments = message with attachments
reconnect-unconfirmed = Couldn't confirm whether “{ $preview }” was sent. It hasn't been resent.
reconnect-failed-header = Reconnect failed — check the endpoint, then relaunch
reconnect-failed-body = Automatic reconnect could not restore this session. Your draft is still editable. Copy it before quitting with Ctrl+C, then reconnect with the same command.

## codex-rs/tui/src/chatwidget/rendering.rs

rendering-open-in-another-app = This conversation is open in another app
rendering-thread-archived = This thread was archived
rendering-key-to-action = {" "}to { $action }
rendering-fork = fork
rendering-command-center = command center
rendering-exit = exit
rendering-transcript = transcript
rendering-forking = Forking conversation…
rendering-continue-new-chat = Continue in a new chat
rendering-configuration-on = Configuration on { $address }
rendering-cloud = Cloud
rendering-cycle = {" "}cycle
rendering-change = {" "}change

## codex-rs/tui/src/chatwidget/reset_credits.rs

reset-credits-expires = Expires { $time } on { $date }
reset-credits-expiration-unavailable = Expiration unavailable
reset-credits-does-not-expire = Does not expire
reset-credits-full-reset = Full reset
reset-credits-reset-limits = Reset your current usage limits

## codex-rs/tui/src/chatwidget/review_popups.rs

review-popups-against-base = Review against a base branch
review-popups-pr-style = (PR Style)
review-popups-uncommitted = Review uncommitted changes
review-popups-a-commit = Review a commit
review-popups-custom-instructions = Custom review instructions
review-popups-select-preset = Select a review preset
review-popups-detached-head = (detached HEAD)
review-popups-select-base-branch = Select a base branch
review-popups-current-branch = Current branch: { $branch }
review-popups-search-branches = Type to search branches
review-popups-select-commit = Select a commit to review
review-popups-search-commits = Type to search commits
review-popups-type-instructions = Type instructions and press Enter

## codex-rs/tui/src/chatwidget/safety_buffering.rs

safety-buffering-header = Giving this request a little extra thought
safety-buffering-message-with-retry = If you'd rather not wait, retry with a faster model. It may be less capable of handling complex requests.
safety-buffering-footer = No action is required. Codex will keep waiting, and this menu will close when the response is ready.
safety-buffering-working = Working
safety-buffering-retry-faster = Retry with a faster model
safety-buffering-dismiss-keep-waiting = Dismiss and keep waiting
safety-buffering-learn-more = Learn more
safety-buffering-stop-retry-title = Stop this attempt and retry?
safety-buffering-stop-retry-body = This will stop the current attempt and retry in a new thread. Any file changes or other actions already taken will remain.
safety-buffering-resend-with-model = Your message will be sent again using { $model }, which may be less capable on complex tasks.
safety-buffering-keep-waiting = Keep waiting
safety-buffering-stop-and-retry = Stop and retry

## codex-rs/tui/src/chatwidget/safety_recourse.rs

safety-recourse-cant-help = I can’t help with this request. Preparing alternative suggestions…
safety-recourse-enable-daybreak = Enable Daybreak
safety-recourse-give-feedback = Give feedback
safety-recourse-choose-rewrite = You can choose a rewrite to start a new chat with your earlier context:

## codex-rs/tui/src/chatwidget/session_model_selection.rs

session-model-selection-apply = {" "}apply ·{" "}
session-model-selection-default = {" "}default ·{" "}
session-model-selection-session = {" "}session
session-model-selection-back = {" "}back

## codex-rs/tui/src/chatwidget/settings_popups.rs

settings-popups-experimental-unavailable = Experimental features are unavailable until startup completes.

## codex-rs/tui/src/chatwidget/service_tiers.rs

service-tiers-open-cloud-first = Open the Cloud conversation before choosing a speed.

## codex-rs/tui/src/chatwidget/session_flow.rs

session-flow-daybreak-on = Daybreak is on for new turns.
session-flow-thread-forked-from = Thread forked from{" "}
session-flow-reverted = Conversation reverted to this point. File changes are unchanged.

## codex-rs/tui/src/chatwidget/settings.rs

settings-daybreak-disable = Disable broader access for cybersecurity work
settings-daybreak-enable = Enable broader access for cybersecurity work
settings-daybreak-learn = Learn about broader access for cybersecurity work
settings-daybreak-manage = Manage broader access for cybersecurity work
settings-image-inputs-unsupported = Model { $model } does not support image inputs. Remove images or switch models.
settings-model-changed = Model changed to { $model } for { $mode } mode.
settings-model-changed-effort = Model changed to { $model } { $effort } for { $mode } mode.
settings-reasoning-default = default

## codex-rs/tui/src/chatwidget/skills.rs
skills-list-skills = List skills
skills-tip-dollar = Tip: press $ to open this list directly
skills-enable-disable = Enable/Disable Skills
skills-enable-or-disable = Enable or disable skills
skills-title = Skills
skills-choose-action = Choose an action
skills-none-available = No skills available.
skills-disabled-count = Disabled { $count ->
    [one] 1 skill
   *[other] { $count } skills
    }
skills-enabled-count = Enabled { $count ->
    [one] 1 skill
   *[other] { $count } skills
    }
skills-enabled-and-disabled = Enabled { $enabled ->
    [one] 1 skill
   *[other] { $enabled } skills
    }, disabled { $disabled ->
    [one] 1 skill
   *[other] { $disabled } skills
    }
skills-read-annotation = { $name } ({ $skill } skill)

## codex-rs/tui/src/chatwidget/slash_command_args.rs
slash-command-args-invalid-quoting = Invalid command arguments. Check the quoting.
slash-command-args-one-of = Specify only one of --{ $name } or --no-{ $name }.
slash-command-args-duplicate = Command setting `{ $name }` was provided more than once.
slash-command-args-requires-value = Command setting `{ $name }` requires a value.
slash-command-args-no-value = Command setting `{ $name }` does not take a value.
slash-command-args-unknown = Unknown command setting `{ $name }`.

## codex-rs/tui/src/chatwidget/startup_submission.rs
startup-submission-waiting = Waiting for startup
startup-submission-cancel = cancel

## codex-rs/tui/src/chatwidget/tui_mode_picker.rs
tui-mode-picker-scrollback = Scrollback
tui-mode-picker-scrollback-desc = Native terminal copy, paste and scrollback
tui-mode-picker-fullscreen = Fullscreen
tui-mode-picker-fullscreen-desc = Scroll within Codex's fullscreen view
tui-mode-picker-title = TUI mode for next launch
tui-mode-picker-footer = Restart to apply. Launch overrides still apply.

## codex-rs/tui/src/chatwidget/transcript_export.rs
transcript-export-title = Export conversation
transcript-export-subtitle = Save the complete conversation as Markdown
transcript-export-copy = Copy to clipboard
transcript-export-copy-desc = Copy the complete Markdown transcript
transcript-export-save = Save to file
transcript-export-save-desc = Choose a Markdown filename
transcript-export-prompt-title = Save conversation
transcript-export-prompt-hint = Type a filename and press Enter

## codex-rs/tui/src/chatwidget/tool_requests.rs
tool-requests-subject-reason = { $subject }: { $reason }
tool-requests-patch-one = apply_patch touching { $file }
tool-requests-patch-many = apply_patch touching { $count } files
tool-requests-network-access = network access to { $target }
tool-requests-mcp-on = MCP { $tool_name } on { $label }
tool-requests-permission-request = permission request
tool-requests-could-action = codex could { $summary }
tool-requests-could-mcp = codex could call MCP tool { $server }.{ $tool_name }
tool-requests-could-access = codex could access { $target }
tool-requests-could-permissions = codex could request permissions
tool-requests-to-action = codex to { $summary }
tool-requests-to-mcp = codex to call MCP tool { $server }.{ $tool_name }
tool-requests-to-access = codex to access { $target }
tool-requests-to-permissions = codex to request permissions
tool-requests-send-input = send terminal input
tool-requests-network-host = network access to { $host }
tool-requests-question-requested = Question requested
tool-requests-questions-requested = { $count } questions requested

## codex-rs/tui/src/chatwidget/turn_runtime.rs
turn-runtime-working = Working
turn-runtime-websocket-timing = WebSocket timing: { $label }
turn-runtime-percent-used = { $used_percent }% used
turn-runtime-tokens-used = { $tokens } used
turn-runtime-high-load = Codex is currently experiencing high load.
turn-runtime-daybreak-title = Turn on Daybreak for your next request?
turn-runtime-daybreak-enable = Enable Daybreak
turn-runtime-daybreak-not-now = Not now
turn-runtime-out-of-credits = You're out of credits. Your workspace is out of credits. Add credits to continue using Codex.
turn-runtime-usage-limit-reached = Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.

## codex-rs/tui/src/chatwidget/usage.rs
usage-n-available = { $count } available
usage-check-availability = Check availability
usage-none-available = None available
usage-title = Usage
usage-subtitle = Account usage and resets.
usage-hint-open = open
usage-hint-choose = choose
usage-hint-confirm = confirm
usage-hint-back = back
usage-view-analytics = View analytics
usage-usage-history = Usage history
usage-redeem-reset = Redeem reset
usage-resets-title = Usage limit resets
usage-checking-resets = Checking your available resets...
usage-loading = Loading...
usage-no-resets = You don't have any usage limit resets available.
usage-load-failed = Couldn't load usage limit resets. Please try again.
usage-cancel = Cancel
usage-resets-available = { $count ->
    [one] { $count } usage limit reset
   *[other] { $count } usage limit resets
    } available.
usage-use-this-reset = Use this reset?
usage-yes-use-reset = Yes, use reset
usage-no-go-back = No, go back
usage-choose-different = Choose a different reset
usage-close = Close
usage-try-again = Try again
usage-resetting = Resetting your usage...
usage-using-reset = Using a reset...
usage-nothing-to-reset = Your usage does not need a reset right now.
usage-reset-gone = That reset is no longer available. Refresh to see your current resets.
usage-none-left = No usage limit resets are available.
usage-reset-failed = Couldn't reset usage. Please try again.
usage-reset-left = Usage reset. You have { $count ->
    [one] { $count } usage limit reset
   *[other] { $count } usage limit resets
    } left.
usage-reset-done = Usage reset.
usage-reset-checking = Usage reset. Checking your remaining resets...
usage-refreshing = Refreshing...
usage-reset-hint = You have { $count ->
    [one] { $count } usage limit reset
   *[other] { $count } usage limit resets
    } available. Run /usage to use one.

## codex-rs/tui/src/chatwidget/usage_notice.rs
usage-notice-summary = ⚠ { $label } limit: { $remaining } left
usage-notice-summary-only = ⚠ { $label } limit: only { $remaining } left
usage-notice-resets-at = resets at { $time }
usage-notice-compact = ⚠ { $label } { $remaining } left

## codex-rs/tui/src/chatwidget/working_directory.rs
working-directory-not-started = The session must start before you can change its working directory.
working-directory-not-idle = Changing directories requires an idle primary session without queued input.

## codex-rs/tui/src/chatwidget/windows_sandbox_prompts.rs
windows-sandbox-setup-intro = Set up the Codex agent sandbox to protect your files and control network access. Learn more <https://developers.openai.com/codex/windows>
windows-sandbox-org-requires = Your organization requires the default Codex agent sandbox to continue. Set it up to protect your files and control network access.
windows-sandbox-learn-more = Learn more <https://developers.openai.com/codex/windows>
windows-sandbox-setup-default = Set up default sandbox (requires Administrator permissions)
windows-sandbox-use-non-admin = Use non-admin sandbox (higher risk if prompt injected)
windows-sandbox-quit = Quit
windows-sandbox-fallback-title = Couldn't set up your sandbox with Administrator permissions
windows-sandbox-fallback-still = You can still use Codex in a non-admin sandbox. It carries greater risk if prompt injected.
windows-sandbox-fallback-org = Your organization requires the default sandbox before Codex can continue.
windows-sandbox-retry-admin = Try setting up admin sandbox again
windows-sandbox-use-codex-non-admin = Use Codex with non-admin sandbox
windows-sandbox-input-disabled = Input disabled until setup completes.
windows-sandbox-setting-up = Setting up sandbox...
windows-sandbox-hang-tight = Hang tight, this may take a few minutes

## codex-rs/tui/src/chatwidget/worktree_picker.rs
worktree-picker-where-new = Where should the new conversation run?
worktree-picker-where-fork = Where should the forked conversation run?
worktree-picker-use-current = Use current Git worktree
worktree-picker-use-current-desc = Keep using the current working directory
worktree-picker-create-new = Create new Git worktree
worktree-picker-create-new-desc = Create a separate checkout of this repository in another directory
worktree-picker-require-local = Managed worktrees require a local Git repository.
worktree-picker-title = Worktrees
worktree-picker-continue = Continue current conversation
worktree-picker-continue-desc = Preserve this conversation in the new checkout
worktree-picker-start-new = Start new conversation
worktree-picker-start-new-desc = Open a fresh conversation in the new checkout
worktree-picker-browse = Browse worktrees
worktree-picker-browse-desc = Resume an owner thread or copy a working directory
worktree-picker-managed-title = Managed worktrees
worktree-picker-loading = Loading worktrees…
worktree-picker-empty = No worktrees in this repository's configured pool
worktree-picker-select = Select a worktree to resume, copy its path, or delete it
worktree-picker-no-thread = No attached thread
worktree-picker-owner-unavailable = Owner thread unavailable
worktree-picker-copy-label = Worktree working directory
worktree-picker-resume-owner = Resume owner thread
worktree-picker-copy-cwd = Copy working directory
worktree-picker-not-utf8 = Path is not valid UTF-8
worktree-picker-delete = Delete worktree
worktree-picker-switch-first = Switch to another checkout before deleting this one
worktree-picker-single = Worktree
worktree-picker-delete-title = Delete this worktree?
worktree-picker-cancel = Cancel
worktree-picker-delete-desc = Keeps thread history; may disrupt other sessions
worktree-picker-just-now = just now
worktree-picker-list-error = Cannot list managed worktrees: { $error }
worktree-picker-status-line = { $status } · { $cwd }
worktree-picker-updated = updated { $ago } · { $cwd }
worktree-picker-archived-updated = Archived · updated { $ago } · { $cwd }
worktree-picker-titled = Worktree: { $title }
worktree-picker-minutes-ago = { $minutes }m ago
worktree-picker-hours-ago = { $hours }h ago
worktree-picker-days-ago = { $days }d ago

## codex-rs/tui/src/chatwidget/slash_dispatch.rs
slash-dispatch-side-starting = Side starting...
slash-dispatch-unavailable-side = '/{ $command }' is unavailable in side conversations. Press Ctrl+C to return to the main thread first.
slash-dispatch-goal-example = Example: /goal improve benchmark coverage
slash-dispatch-raw-usage = Usage: /raw [on|off]
slash-dispatch-usage-login = Sign in with ChatGPT to use /usage.
slash-dispatch-collab-disabled = Collaboration modes are disabled.
slash-dispatch-collab-enable = Enable collaboration modes to use /plan.
slash-dispatch-plan-unavailable = Plan mode unavailable right now.
slash-dispatch-before-start = '/{ $command }' is unavailable before the session starts.
slash-dispatch-task-in-progress = '/{ $command }' is disabled while a task is in progress.
slash-dispatch-daybreak-provider = Daybreak requires the OpenAI provider with either ChatGPT sign-in or an API key with Daybreak support enabled.
slash-dispatch-daybreak-no-account = Daybreak is not available for this account. Apply at https://openai.com/form/enterprise-trusted-access-for-cyber/. Learn more at https://help.openai.com/en/articles/20001326.
slash-dispatch-daybreak-unknown = Daybreak availability could not be determined from the connected host's model catalog.
slash-dispatch-daybreak-thread = Daybreak is unavailable until the thread starts.
slash-dispatch-archive-title = Archive this session?
slash-dispatch-archive-running = This will stop the current turn and archive the session.
slash-dispatch-archive-sure = Are you sure? This will archive the current session
slash-dispatch-archive-no = No, don't archive
slash-dispatch-return-current = Return to the current session
slash-dispatch-archive-yes = Yes, archive
slash-dispatch-archive-now = Archive this session now
slash-dispatch-delete-title = Delete this session?
slash-dispatch-delete-subtitle = Also deletes subagents and saved side chats. Cannot be undone.
slash-dispatch-delete-no = No, keep this session
slash-dispatch-delete-yes = Yes, delete and return to command center
slash-dispatch-delete-now = Permanently delete this session now
slash-dispatch-recap-starting = Session is still starting; try /recap again in a moment.
slash-dispatch-no-auto-preset = Internal error: missing the 'auto' approval preset.
slash-dispatch-diff-not-git = `/diff` — _not inside a git repository_
slash-dispatch-diff-failed = Failed to compute diff: { $error }
slash-dispatch-diff-no-runner = Failed to compute diff: workspace command runner unavailable
slash-dispatch-current-cwd = Current working directory: { $path }
slash-dispatch-rollout-path = Current rollout path: { $path }
slash-dispatch-rollout-unavailable = Rollout path is not available yet.
slash-dispatch-pwd-usage = Usage: /pwd
slash-dispatch-usage-usage = Usage: /usage [daily|weekly|cumulative]
slash-dispatch-voice-usage = Usage: /voice [settings|mute|stop]
slash-dispatch-mcp-login-usage = Usage: /mcp login <name> [--environment <id>]
slash-dispatch-mcp-needs-session = MCP sign-in requires an active session.
slash-dispatch-mcp-usage = Usage: /mcp [verbose | login <name> [--environment <id>]]
slash-dispatch-keymap-invalid = Invalid `tui.keymap` configuration: { $error }
slash-dispatch-keymap-usage = Usage: /keymap [debug]
slash-dispatch-thread-name-empty = Thread name cannot be empty.
slash-dispatch-goal-change-before-start = The session must start before you can change a goal.
slash-dispatch-goal-set-before-start = The session must start before you can set a goal.
slash-dispatch-unrecognized = Unrecognized command '/{ $name }'. Type "/" for a list of supported commands.
slash-dispatch-review-running = '/{ $command }' is unavailable while code review is running.
slash-dispatch-working = Working

## codex-rs/tui/src/history_cell/async_replies.rs
async-replies-re = Re:{" "}

## codex-rs/tui/src/history_cell/computer_activity.rs
computer-activity-using-computer = Using computer
computer-activity-used-computer = Used computer
computer-activity-action-count = {" "}· { $count ->
        [one] 1 action
       *[other] { $count } actions
    }
computer-activity-failed-count = {" "}· { $failures } failed
computer-activity-computer-action = Computer action
computer-activity-failed-error = Failed: { $error }
computer-activity-failed-title-error = Failed: { $title } — { $error }
computer-activity-failed-title = Failed: { $title }
computer-activity-captured-screenshot = Captured screenshot · { $title }
computer-activity-hidden-more = { $hidden } more · { $key }

## codex-rs/tui/src/history_cell/delegated_message.rs
delegated-message-sent-by-codex = Sent by Codex from task { $source_thread_id }

## codex-rs/tui/src/history_cell/dynamic.rs
dynamic-interrupted-before-result = Interrupted before this tool returned a result.
dynamic-image-content = <image content>
dynamic-audio-content = <audio content>
dynamic-interrupted = Interrupted
dynamic-calling = Calling
dynamic-called = Called
dynamic-failed = Failed
dynamic-duration-ms = {" "}· { $duration_ms }ms
dynamic-no-output = (no output)
dynamic-output-unavailable = (output unavailable)
dynamic-earlier-lines-hidden = {" "} … { $skip } earlier lines hidden
dynamic-arguments = Arguments: { $args }

## codex-rs/tui/src/history_cell/exec.rs
exec-waited-for-terminal = Waited for terminal
exec-sent-terminal-input = Sent terminal input
exec-waited-for-background-terminal = Waited for background terminal
exec-interacted-with-background-terminal = Interacted with background terminal
exec-waited-for-background-terminal-command = Waited for background terminal: { $command }
exec-interacted-with-background-terminal-command = Interacted with background terminal: { $command }
exec-background-terminals = Background terminals
exec-no-background-terminals-running = No background terminals running.
exec-and-more-running = {"."}.. and { $remaining } more running

## codex-rs/tui/src/history_cell/hook_cell.rs
hook-cell-running-hooks = Running hooks
hook-cell-running-hook = Running hook
hook-cell-hook-prefix = ↳ Hook ·{" "}
hook-cell-hook-completed = Hook completed
hook-cell-hook-failed = Hook failed
hook-cell-blocked-by-hook = Blocked by hook
hook-cell-hook-stopped = Hook stopped
hook-cell-hook-running = Hook running

## codex-rs/tui/src/history_cell/mcp_result.rs
mcp-result-returned-image = Returned image
mcp-result-audio-content = <audio content>
mcp-result-embedded-resource = embedded resource: { $uri }
mcp-result-unknown-embedded-resource = <unknown embedded resource>
mcp-result-link = link: { $uri }

## codex-rs/tui/src/history_cell/mcp_preview.rs
mcp-preview-environment = {" "}(environment { $environment_id })
mcp-preview-called = Called
mcp-preview-failed = Failed
mcp-preview-calling = Calling
mcp-preview-returned-image = Returned image
mcp-preview-error-prefix = Error:{" "}

## codex-rs/tui/src/history_cell/notices.rs
notices-conversation-recap = Conversation recap
notices-run = Run{" "}
notices-to-update = {" "}to update.
notices-see = See{" "}
notices-for-installation-options = {" "}for installation options.
notices-update-available = Update available!
notices-see-full-release-notes = See full release notes:
notices-see-installation-options-raw = See https://github.com/openai/codex for installation options.
notices-use-daemon = Use /daemon to manage the local background server.
notices-updating-may-interrupt = Updating may interrupt active or queued work.
notices-content-cant-be-shown = This content can't be shown
notices-extra-caution-biological = We take extra caution with requests involving biological research and applications that could pose safety risks.
notices-eligible-researchers = Eligible researchers can apply for Trusted Access.
notices-trusted-access = Trusted Access
notices-learn-more = Learn more
notices-daybreak-apply-body = If you’re doing authorized security work, apply for Daybreak to get broader access.
notices-apply-for-daybreak = Apply for Daybreak
notices-daybreak-off-body = Daybreak is currently off. If you’re doing authorized security work, turn it on and try your request again.
notices-daybreak-astra-body = Daybreak isn’t available for Astra. Some cybersecurity requests may still be limited.
notices-daybreak-enabled-body = Some cybersecurity requests are still limited, even when Daybreak is on.
notices-content-cant-be-shown-curly = This content can’t be shown
notices-extra-care-cyber = We take extra care with some cybersecurity requests.
notices-deprecation = Deprecation
notices-conversation-interrupted = ■ Conversation interrupted - use /feedback if something went wrong
notices-generating-recap = Generating conversation recap
notices-generating-recap-dots = Generating conversation recap...
notices-next-prefix = Next:{" "}
notices-recap-prefix = Recap:{" "}
notices-recap-prefix-short = Recap:
notices-next-action = Next: { $action }

## codex-rs/tui/src/history_cell/plans.rs
plans-proposed-plan = Proposed Plan
plans-empty = (empty)
plans-updated-plan-label = Updated plan
plans-updated-plan = Updated Plan
plans-steps-complete = {" "}· { $completed }/{ $total } complete
plans-no-steps-provided = (no steps provided)

## codex-rs/tui/src/history_cell/request_user_input.rs
request-user-input-questions = Questions
request-user-input-answered-count = {" "}{ $answered }/{ $total } answered
request-user-input-interrupted-suffix = {" "}(interrupted)
request-user-input-unanswered-suffix = {" "}(unanswered)
request-user-input-answer-label = answer:{" "}
request-user-input-note-label = note:{" "}
request-user-input-interrupted-with-unanswered = interrupted with { $unanswered } unanswered
request-user-input-questions-answered = Questions { $answered }/{ $total } answered
request-user-input-interrupted-raw = (interrupted)
request-user-input-unanswered-raw = (unanswered)

## codex-rs/tui/src/history_cell/separators.rs
separators-less-than-one-second = <1s
separators-worked-for = Worked for { $elapsed }
separators-date-format-this-year = %b %-d at{" "}
separators-date-format-other-year = %b %-d, %Y at{" "}
separators-local-tools = Local tools: { $count } { $count ->
        [one] call
       *[other] calls
    } ({ $duration })
separators-inference = Inference: { $count } { $count ->
        [one] call
       *[other] calls
    } ({ $duration })
separators-websocket-events-sent = WebSocket: { $count } events send ({ $duration })
separators-stream-events = { $count ->
        [one] Stream
       *[other] Streams
    }: { $count } { $count ->
        [one] event
       *[other] events
    } ({ $duration })
separators-websocket-events-received = { $count } events received ({ $duration })
separators-responses-api-overhead = Responses API overhead: { $duration }
separators-responses-api-inference = Responses API inference: { $duration }
separators-duration-iapi = { $duration } (iapi)
separators-duration-service = { $duration } (service)
separators-ttft = TTFT: { $parts }
separators-tbt = TBT: { $parts }

## codex-rs/tui/src/history_cell/session.rs
session-tip = Tip: { $tip }
session-get-started = {" "} To get started, describe a task or try one of these commands:
session-help-init = {" "}- create an AGENTS.md file with instructions for Codex
session-help-status = {" "}- show current session configuration
session-help-permissions = {" "}- choose what Codex is allowed to do
session-help-model = {" "}- choose what model and reasoning effort to use
session-help-review = {" "}- review any changes and find issues
session-model-changed = model changed:
session-model-requested = requested: { $requested_model }
session-model-used = used: { $model }
session-permissions-label = {" "} permissions:{" "}
session-yolo-mode = YOLO mode
session-raw-model = model: { $model }{ $reasoning }
session-raw-directory = directory: { $directory }
session-raw-permissions-yolo = permissions: YOLO mode

## codex-rs/tui/src/history_cell/startup_warnings.rs
startup-warnings-startup = Startup
startup-warnings-mcp-source = MCP · { $server }
startup-warnings-mcp-incomplete = MCP startup incomplete: { $server }
startup-warnings-sign-in-required = Sign-in required.

## codex-rs/tui/src/history_cell/warnings.rs
warnings-warning = Warning

## codex-rs/tui/src/exec_cell/compact.rs
compact-exited-status = Exited (status { $status })
compact-running = Running
compact-background-output = Background output
compact-ran = Ran
compact-no-output = (no output)

## codex-rs/tui/src/exec_cell/live_output.rs
live-output-lines-omitted = … +{ $omitted } { $omitted ->
        [one] line
       *[other] lines
    }
live-output-bytes-omitted = {"."}.. { $bytes } bytes omitted ...

## codex-rs/tui/src/exec_cell/render.rs
render-interacted-with = Interacted with `{ $command_display }`, sent `{ $preview }`
render-waited-for = Waited for `{ $command_display }`
render-nonzero-exits = {" "}· { $count } { $count ->
        [one] nonzero exit
       *[other] nonzero exits
    }
render-reading = Reading
render-read-past = Read
render-query-in-path = { $query } in { $path }
render-searching-for = Searching for
render-searched-for = Searched for
render-searching-files-in = Searching files in
render-searched-files-in = Searched files in
render-listing-files-in = Listing files in
render-listed-files-in = Listed files in
render-running = Running
render-ran = Ran
render-exit-code = {" "}(exit { $code })
render-lines-omitted-hint = … +{ $omitted } { $omitted ->
        [one] line
       *[other] lines
    } ({ $hint })
render-lines-omitted = … +{ $omitted } { $omitted ->
        [one] line
       *[other] lines
    }
render-exploring = Exploring
render-explored = Explored
render-title-read = Read
render-title-list = List
render-title-search = Search
render-title-run = Run
render-in = {" "}in{" "}
render-command-exit-code = {" "}(command exit { $code })
render-you-ran = You ran
render-no-output = (no output)

## codex-rs/tui/src/app/agent_center/mod.rs
agent-center-filter-all = All
agent-center-filter-needs-you = Needs you
agent-center-filter-working = Working
agent-center-filter-ready = Ready
agent-center-filter-inactive = Inactive

## codex-rs/tui/src/app/agent_center/hints.rs
hints-navigate = Navigate
hints-up = Up
hints-down = Down
hints-open = Open
hints-page-up = Page up
hints-page-down = Page down
hints-quit = Quit
hints-tasks = Tasks
hints-new = New
hints-new-worktree = New worktree
hints-resume = Resume
hints-fork = Fork
hints-rename-label = Rename
hints-stop = Stop
hints-archive = Archive
hints-hide = Hide
hints-delete = Delete
hints-pin-unpin = Pin/unpin
hints-view = View
hints-filter = Filter
hints-search = Search
hints-group = Group
hints-task-shortcuts = Task shortcuts
hints-help-short = help
hints-back = back
hints-move = move
hints-rename-short = rename
hints-open-short = open
hints-new-short = new

## codex-rs/tui/src/app/agent_center/render.rs
render-search-prefix = Search ›{" "}
render-rename-prefix = Rename ›{" "}
render-grouping-project = Project
render-grouping-directory = Directory
render-grouping-status = Status
render-grouping-model = Model
render-agent-command-center = Agent command center
render-group-label = {" "} Group: { $grouping }  { $group_key }
render-filter-hint = filter
render-creating-worktree = Creating worktree…
render-archiving-task = Archiving task…  You can still browse.
render-deleting-task = Deleting task…  You can still browse.
render-error-loading-tasks = Error loading tasks
render-resize-to-see-all = … resize to see all

## codex-rs/tui/src/app/agent_center/rows.rs
rows-reconnecting = Reconnecting…
rows-no-matching-tasks = No matching tasks
rows-loading-tasks = Loading tasks…
rows-could-not-load-tasks = Could not load tasks
rows-no-tasks-yet = No tasks yet
rows-column-tasks = Tasks
rows-column-status = Status
rows-column-updated = Updated
rows-searching-history = Searching history…
rows-show-more-retry = Show more (retry)
rows-show-more-matches = Show more matches
rows-loading-more = Loading more…
rows-show-more = Show more
rows-pinned = Pinned
rows-count-of-total = { $count } of { $total }
rows-voice-badge = {" "} voice

## codex-rs/tui/src/app/agent_navigation.rs
agent-navigation-picker-subtitle = Select an agent to watch. { $previous } previous, { $next } next.

## codex-rs/tui/src/app/agents_overview.rs
agents-overview-reconnect-failed-stale = Reconnect failed — agent list is stale; relaunch to retry
agents-overview-reconnecting-stale = Reconnecting — agent list is stale
agents-overview-rename-target-gone = The rename target disappeared. Unsubmitted title: { $title }
agents-overview-session-unavailable-refresh = Agent session { $thread_id } is unavailable. Refresh Command Center and try again.
agents-overview-session-unavailable = Agent session { $root_thread_id } is unavailable: { $error }
agents-overview-load-task-settings-failed = Failed to load task settings: { $error }
agents-overview-resume-needs-permissions = Cannot resume task without preserving the selected permissions.
agents-overview-load-conversation-failed = Couldn't load this conversation. Please try again.
agents-overview-attach-failed = Failed to attach to task: { $error }
agents-overview-wait-permissions = Wait for permissions to update before starting a session.
agents-overview-load-project-settings-failed = Failed to load project settings: { $error }
agents-overview-profile-differs = Permission profile has different settings.
agents-overview-load-new-session-settings-failed = Failed to load new session settings: { $error }
agents-overview-stop-task-failed = Failed to stop background task: { $error }

## codex-rs/tui/src/app/agents_overview_actions.rs
agents-overview-actions-pin-failed = Failed to pin task: { $error }
agents-overview-actions-unpin-failed = Failed to unpin task: { $error }
agents-overview-actions-untitled-task = Untitled task
agents-overview-actions-archive-title = Archive “{ $name }”?
agents-overview-actions-archive-task-description = This stops any running work in this task and archives it. Child agents stay unchanged. Its history can be restored from the resume picker.
agents-overview-actions-archive-task = Archive task
agents-overview-actions-archive-children-description = This stops any running work in this task and its child agents, then archives them. Their history can be restored from the resume picker.
agents-overview-actions-archive-children = Archive task and child agents
agents-overview-actions-delete-title = Permanently delete “{ $name }”?
agents-overview-actions-delete-description = Tasks with saved side chats stay intact. Otherwise, deleting this task stops its running work and permanently deletes its history and its child agents’ history.
agents-overview-actions-delete = Permanently delete task and child agents
agents-overview-actions-cancel = Cancel
agents-overview-actions-could-not-archive = Could not archive task
agents-overview-actions-could-not-delete = Could not delete task
agents-overview-actions-work-may-have-stopped = Work may have stopped. Resume the task to continue, or retry the action.
agents-overview-actions-back-to-agents = Back to agents

## codex-rs/tui/src/app/agents_overview_details.rs
agents-overview-details-agent = Agent:{" "}
agents-overview-details-needs-attention = Needs attention
agents-overview-details-open-to-review = Open task to review.
agents-overview-details-waiting-approval = Waiting for approval.
agents-overview-details-waiting-response = Waiting for your response.
agents-overview-details-task-error = Task encountered an error.
agents-overview-details-latest-activity = Latest activity

## codex-rs/tui/src/app/agents_overview_errors.rs
agents-overview-errors-return = Return to command center
agents-overview-errors-unable = Unable to complete action

## codex-rs/tui/src/app/agents_overview_grouping.rs
agents-overview-grouping-unknown = Unknown
agents-overview-grouping-save-failed = Failed to save Command Center grouping: { $err }

## codex-rs/tui/src/app/agents_overview_loading.rs
agents-overview-loading-task = Loading task…

## codex-rs/tui/src/app/agents_overview_new.rs
agents-overview-new-checkout-retained = { $reason } A checkout was retained at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.
agents-overview-new-could-not-start = Could not start the session.
agents-overview-new-could-not-load-settings = Could not load the new session settings.
agents-overview-new-sessions-paused = New sessions are paused while active turns finish or are canceled. The default shutdown wait is up to one minute.
agents-overview-new-will-reconnect = {" "}If the server restarts, Codex will reconnect automatically.
agents-overview-new-server-shutting-down = Codex's background server is shutting down
agents-overview-new-start-failed = Failed to start session: { $error }
agents-overview-new-worktree-not-applied = The server did not apply the worktree directory.
agents-overview-new-could-not-open = Could not open the new session.
agents-overview-new-worktree-support = Managed worktrees require local worktree support.
agents-overview-new-project-untrusted = The source project is not trusted.
agents-overview-new-worktree-task-failed = Worktree creation task failed: { $error }

## codex-rs/tui/src/app/agents_overview_usage.rs
agents-overview-usage-tokens-in = { $count } in
agents-overview-usage-tokens-out = { $count } out
agents-overview-usage-tokens = Tokens:{" "}
agents-overview-usage-credits = { $count } credits
agents-overview-usage-estimated = Est. usage:{" "}

## codex-rs/tui/src/app/agents_overview_view.rs
agents-overview-view-needs-input = Needs input
agents-overview-view-working = Working
agents-overview-view-ready = Ready
agents-overview-view-inactive = Inactive
agents-overview-view-untitled-task = Untitled task
agents-overview-view-error = Error
agents-overview-view-model = Model:{" "}
agents-overview-view-reasoning = Reasoning:{" "}
agents-overview-view-unknown = Unknown
agents-overview-view-task-details = Task details
agents-overview-view-project = Project
agents-overview-view-branch = Branch
agents-overview-view-prompt = Prompt
agents-overview-view-no-prompt = No prompt available.
agents-overview-view-last-message = Last message

## codex-rs/tui/src/app/app_server_events.rs
app-server-events-windows-sandbox-setup-failed = Windows sandbox setup failed: { $error }
app-server-events-unknown-error = unknown error

## codex-rs/tui/src/app/backend_banner_fallback.rs
backend-banner-fallback-switched-to = Automatically switched to
backend-banner-fallback-usage-limits = {" "}due to usage limits.
backend-banner-fallback-switched-back-to = Automatically switched back to
backend-banner-fallback-usage-recovered = {" "}because ordinary usage is available again.

## codex-rs/tui/src/app/backend_selection.rs
backend-selection-connection-unavailable = That connection is unavailable. Choose another and try again.
backend-selection-end-voice-call = End the voice call before switching.
backend-selection-wait-for-turn = Wait for the current turn to finish before switching.
backend-selection-settings-unavailable = Connection settings are unavailable.
backend-selection-prepare-new-failed = Failed to prepare the new conversation: { $error }
backend-selection-read-defaults-failed = Failed to read new session defaults: { $error }
backend-selection-start-new-failed = Failed to start the new conversation: { $error }
backend-selection-open-new-failed = Failed to open the new conversation: { $error }
backend-selection-save-default-failed = Switched, but could not save the default for this project: { $error }
backend-selection-settings-unavailable-reopen = Connection settings are unavailable. Open the session picker again.
backend-selection-conversation-connection-unavailable = The connection for this conversation is unavailable. Open the session picker again.
backend-selection-prepare-failed = Failed to prepare the conversation: { $error }

## codex-rs/tui/src/app/backend_sessions.rs
backend-sessions-cloud = Cloud
backend-sessions-local = Local
backend-sessions-connection = Connection { $number }

## codex-rs/tui/src/app/cloud_environments.rs
cloud-environments-title = Cloud environment
cloud-environments-loading = Loading saved environments…
cloud-environments-use-default = { $error }. You can still use Default.
cloud-environments-default = Default
cloud-environments-default-description = Use the default environment with the current GitHub project, if available
cloud-environments-workspace = Workspace environment
cloud-environments-personal = Personal environment
cloud-environments-search = Search environments

## codex-rs/tui/src/app/cloud_switch_wait.rs
cloud-switch-wait-connecting = Connecting to Cloud…
cloud-switch-wait-did-not-open = Cloud conversation did not open
cloud-switch-wait-draft-kept = draft kept

## codex-rs/tui/src/app/clipboard.rs
clipboard-busy = Clipboard busy; try again
clipboard-paste-image-failed = Failed to paste image: { $err }

## codex-rs/tui/src/app/config_panel.rs
config-panel-could-not-save = Could not save: { $error }
config-panel-rebuild-config-failed = Failed to rebuild config for cwd { $cwd }
config-panel-saved-reload-failed = Saved, but could not reload: { $error }
config-panel-saved-overridden = Saved · overridden by config or system setting
config-panel-saved-new-composers = Saved · applies to new composers
config-panel-test-already-scheduled = Test already scheduled · switch apps
config-panel-test-scheduled = Test scheduled in 5s · switch apps
config-panel-notification-test = Codex notification test
config-panel-test-send-failed = Could not send test · check terminal connection
config-panel-test-sent-ghostty = Test sent · Ghostty hides banners while focused
config-panel-test-sent = Test sent · check terminal/system settings

## codex-rs/tui/src/app/daemon_menu.rs
daemon-menu-running = Running daemon: { $version }
daemon-menu-not-connected = Not connected to the local background server.
daemon-menu-title = Daemon
daemon-menu-remote-unavailable = Manage this server on its host. Local daemon updates are unavailable for remote connections.
daemon-menu-run-cli = Run the Codex CLI to manage the daemon from this menu.
daemon-menu-install-stable = Install latest public stable
daemon-menu-use-this-cli = Use this CLI build
daemon-menu-no-local-package = This CLI has no local package to copy
daemon-menu-restart = Restart background server
daemon-menu-restart-unverified = Codex hasn't verified it manages this server. Restart it where it was started.
daemon-menu-restart-title = Restart background server?
daemon-menu-restart-interrupt = Active or queued work in other chats may be interrupted.
daemon-menu-restart-reconnect = Codex will reconnect automatically.
daemon-menu-cancel = Cancel
daemon-menu-restart-button = Restart
daemon-menu-stable-explanation = Install the latest public stable release. Restore production updates; keep your automatic-update setting.
daemon-menu-this-cli-explanation = Use this CLI package v{ $version } from { $path }. Copy the complete package and pin it against automatic updates.
daemon-menu-update-interrupt = This may restart the daemon and interrupt active or queued work.
daemon-menu-update-exit = Codex exits to update in this terminal. Relaunch it afterward.
daemon-menu-update-title = Update daemon and exit Codex?
daemon-menu-update-and-exit = Update and exit

## codex-rs/tui/src/app/background_requests.rs
background-requests-add-marketplace-failed = Failed to add marketplace: { $err }
background-requests-remove-marketplace-failed = Failed to remove marketplace: { $err }
background-requests-upgrade-marketplace-failed = Failed to upgrade marketplace: { $err }
background-requests-install-plugin-failed = Failed to install plugin: { $err }
background-requests-uninstall-plugin-failed = Failed to uninstall plugin: { $err }
background-requests-update-plugin-config-failed = Failed to update plugin config: { $err }
background-requests-update-hook-config-failed = Failed to update hook config: { $error }
background-requests-trust-hook-failed = Failed to trust hook: { $error }
background-requests-trust-hooks-failed = Failed to trust hooks: { $error }
background-requests-feedback-zip-needs-local = Saving a feedback ZIP requires a local app server.
background-requests-feedback-zip-saved = Feedback ZIP saved to { $path }
background-requests-load-mcp-inventory-failed = Failed to load MCP inventory: { $err }
background-requests-next-step-sign-in-chatgpt-auth = Sign in with ChatGPT auth; API key auth cannot load remote plugin catalogs
background-requests-next-step-sign-in-retry-section = Sign in to ChatGPT, then try loading this section again
background-requests-next-step-admin-enable-plugins = Ask a workspace admin to enable Codex plugins or plugin sharing
background-requests-next-step-switch-workspace = Switch to the matching workspace or ask the sharer for access
background-requests-next-step-check-workspace-access = Check that you are signed in to the correct workspace and still have access
background-requests-next-step-update-codex = Update Codex, then try opening the shared plugin again
background-requests-next-step-try-again-later = Try again later; local plugin functionality is still available
background-requests-next-step-admin-confirm-access = Ask a workspace admin to confirm plugin access
background-requests-next-step-sharer-confirm-access = Ask the sharer or a workspace admin to confirm plugin access
background-requests-enable-plugin-sharing = Enable plugin sharing for this Codex session to load shared plugins

## codex-rs/tui/src/app/config_persistence.rs
config-persistence-task-failed = { $context } task failed
config-persistence-rebuild-config-cwd-failed = Failed to rebuild config for cwd { $cwd }
config-persistence-rebuild-config-profile-failed = Failed to rebuild config for permission profile { $profile_id }
config-persistence-set-profile-failed = Failed to set permission profile `{ $profile_id }`: { $err }
config-persistence-set-approval-policy-failed = Failed to set approval policy
config-persistence-permissions-updated = Permissions updated to { $label }
config-persistence-wait-task-connect = Wait for the task to connect before selecting permissions.
config-persistence-selection-requested = Permission selection requested: { $label }
config-persistence-selection-needs-newer-server = Permission selection requires a newer app server.
config-persistence-select-permissions-failed = Failed to select permissions: { $error }
config-persistence-wait-before-changing = Wait for permissions to update before changing permissions.
config-persistence-wait-before-switching = Wait for permissions to update before switching tasks.
config-persistence-saved-refresh-failed = { $setting } were saved, but Codex could not refresh the effective config: { $err }
config-persistence-carry-approval-policy-failed = Failed to carry forward approval policy override: { $err }
config-persistence-carry-approval-policy-failed-context = Failed to carry forward approval policy override
config-persistence-carry-reviewer-failed = Failed to carry forward approvals reviewer
config-persistence-carry-profile-failed-context = Failed to carry forward permission profile override
config-persistence-carry-profile-failed = Failed to carry forward permission profile override: { $err }
config-persistence-prefixed-error = { $prefix }: { $err }
config-persistence-unsupported-profile = { $prefix }: unsupported active permission profile `{ $id }`
config-persistence-update-feature-failed = Failed to update experimental feature `{ $feature_key }`: { $err }
config-persistence-label-approve-for-me = Approve for me
config-persistence-label-ask-for-approval = Ask for approval
config-persistence-enable-approve-failed-prefix = Failed to enable Approve for me
config-persistence-update-features-failed = Failed to update experimental features: { $error }
config-persistence-features-saved-not-applied = Experimental feature changes were saved but not applied: { $message }
config-persistence-setting-experimental-features = Experimental feature changes
config-persistence-enable-approve-failed = Failed to enable Approve for me: { $err }
config-persistence-save-memory-failed = Failed to save memory settings: { $err }
config-persistence-memory-saved-not-applied = Memory setting changes were saved but not applied: { $message }
config-persistence-setting-memory = Memory setting changes
config-persistence-memory-thread-update-failed = Saved memory settings, but failed to update the current thread: { $err }
config-persistence-reset-memories-failed = Failed to reset memories: { $err }
config-persistence-reset-memories = Reset local memories.
config-persistence-reasoning-default = default
config-persistence-overrides-unsupported-remote = Permission overrides are not supported when resuming a remote task.
config-persistence-refresh-approve-failed = Failed to refresh overridden Approve for me settings: { $err }
config-persistence-refresh-approve-failed-prefix = Failed to refresh overridden Approve for me settings
config-persistence-windows-sandbox-overridden = Windows sandbox setup completed, but its mode was overridden by the effective configuration.
config-persistence-overridden-by-layer = the effective config is overridden by a higher-priority layer
