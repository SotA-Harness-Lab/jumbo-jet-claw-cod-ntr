# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

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
status-indicator-elapsed-open = ({ $elapsed } •{" "}
status-indicator-to-interrupt = {" "}to interrupt)
status-indicator-separator = {" "}·{" "}

## codex-rs/tui/src/terminal_program_status.rs
terminal-program-status-idle = Idle
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

## app/provider/rpc.rs

provider-rpc-override-reason = A higher-priority setting overrides this choice.
provider-rpc-update-server = Update the connected app server to browse model providers.
provider-rpc-load-failed = Could not load model providers: { $err }

## app/resume_config.rs

resume-config-cwd-needs-cd = `tui.resume_cwd = "current"` requires `--cd` when using a remote workspace
resume-config-cwd-failed = Failed to determine working directory for resume: { $err }
resume-config-rebuild-failed = Failed to rebuild configuration for resume: { $err }
resume-config-check-folder-failed = Unable to check resumed folder: { $error }
resume-config-check-trust-failed = Unable to check folder trust: { $error }
resume-config-reload-trust-failed = Failed to reload trusted folder settings: { $error }
resume-config-load-hooks-failed = Unable to load folder hooks: { $error }
resume-config-review-hooks-failed = Unable to review folder hooks: { $error }

## daybreak.rs

daybreak-requires-openai = Daybreak requires the OpenAI provider with either ChatGPT sign-in or an API key with Daybreak support enabled. Turn it off to continue.
daybreak-unconfirmed = Daybreak support for model { $model } could not be confirmed by the connected server. Use /daybreak to turn it off, or choose a compatible model and server.

## device_login.rs

device-login-gateway-failed = Gateway sign-in failed
device-login-lost-result = Lost the gateway sign-in result. Restart Codex to check the connection.

## external_agent_config_migration/flow.rs

import-flow-source-error = { $source }: { $error }
import-flow-check-failed = Could not check for importable setup: { $errors }
import-flow-started = Import started.
import-flow-keep-working = {" "}You can keep working while it finishes.
import-flow-applies-new-chats = Imported setup will apply to new chats.
import-flow-importing = Importing:
import-flow-more = , +{ $count } more
import-flow-failed-count = { $count } failed
import-flow-finished = Import finished:{" "}
import-flow-imported-count = { $count } imported
import-flow-results-by-type = Results by type:
import-flow-run-again = Run /import again to check for additional items.
import-flow-remaining = 
    { $count ->
        [one] 1 additional item remains. After it finishes, run /import again to review it.
       *[other] { $count } additional items remain. After it finishes, run /import again to review them.
    }
import-flow-source-gone = Selected import source is no longer available.
import-flow-failed = Import failed: { $err }

## goal_files.rs

goal-files-empty = Goal objective must not be empty.
goal-files-reference-too-long = Goal objective file reference is too long: { $actual_chars } characters. Limit: { $limit } characters.

## lib.rs

lib-workload-identity-remote-host = workload identity must be configured on the remote app-server host
lib-config-selects-other-provider = The current configuration selects { $current } instead of { $selected_id }.
lib-no-saved-session = No saved session found with ID { $id_str }. Run `codex { $action }` without an ID to choose from existing sessions.
lib-cloud-connection-failed = Could not add the Cloud connection. { $error }
lib-restore-terminal-failed = failed to restore terminal. Run `reset` or restart your terminal to recover: { $err }
lib-config-load-error = Error loading configuration: { $err }
lib-config-toml-error-detail = Error loading config.toml:{"\u000A"}{ $detail }
lib-config-toml-error = Error loading config.toml: { $err }

## startup_draft_input.rs

startup-draft-waiting = Waiting for startup
startup-draft-cancel = cancel

## startup_recovery.rs

startup-recovery-unsent-draft = {"\u000A"}Unsent draft:{"\u000A"}{ $text }{"\u000A"}

## theme_picker.rs

theme-picker-fallback-subtitle = Move up/down to live preview themes
theme-picker-custom-subtitle = Custom .tmTheme files can be added to the { $path } directory.
theme-picker-custom-name = { $name } (custom)
theme-picker-title = Select Syntax Theme
theme-picker-search = Type to filter themes...

## pager_overlay/transcript.rs

pager-transcript-latest = { $key } latest
pager-transcript-select = { $key } select
pager-transcript-selection = selection
pager-transcript-open-link-failed = Could not open link: { $error }
pager-transcript-close = close
pager-transcript-find = find
pager-transcript-browse-prompts = browse prompts
pager-transcript-edit-next = to edit next
pager-transcript-edit-message = to edit message

## thread_transcript.rs

thread-transcript-empty = No transcript content available

## app/clipboard.rs

clipboard-last-message = last message

## app/resize_reflow.rs

resize-reflow-earlier-messages = Earlier messages are available — press { $key } to view the full transcript

## transcript_view/turn_details.rs

turn-details-ran = 
    { $ran ->
        [one] Ran { $ran } command
       *[other] Ran { $ran } commands
    }
turn-details-explored = Explored files
turn-details-explored-ran = 
    { $ran ->
        [one] Explored files · ran { $ran } command
       *[other] Explored files · ran { $ran } commands
    }
turn-details-tool-calls = { $count } tool calls · { $label }
turn-details-previous-messages = 
    { $count ->
        [one] 1 previous message
       *[other] { $count } previous messages
    }

## app/event_dispatch.rs

event-dispatch-open-this-task-before-using = Open this task before using that action.
event-dispatch-windows-sandbox-setup-requires-loca = Windows sandbox setup requires local connections and executors.
event-dispatch-a-worktree-is-already-being = A worktree is already being created.
event-dispatch-changing-directories-requires-an-id = Changing directories requires an idle primary session without queued input.
event-dispatch-changing-directories-is-not-support = Changing directories is not supported for remote workspaces or remote execution environments.
event-dispatch-export-failed = Export failed: { $error }
event-dispatch-nothing-to-copy = Nothing to copy
event-dispatch-no-saved-chat-found-matching = No saved chat found matching '{ $id_or_name }'.
event-dispatch-failed-to-archive-thread = Failed to archive thread { $thread_id }: { $error }
event-dispatch-wait-for-permissions-to-update = Wait for permissions to update before forking.
event-dispatch-failed-to-name-the-forked = Failed to name the forked session: { $err }
event-dispatch-fork-created-you-can-continue = Fork created. You can continue here.
event-dispatch-failed-to-attach-to-forked = Failed to attach to forked app-server thread: { $err }
event-dispatch-failed-to-fork-current-session = Failed to fork current session through the app server: { $err }
event-dispatch-a-thread-must-contain-at = A thread must contain at least one turn before it can be forked.
event-dispatch-this-remote-prompt-contains-local = This remote prompt contains local image paths or command syntax that cannot be restored safely. Write a new message and reattach any images.
event-dispatch-wait-for-permissions-to-update-2 = Wait for permissions to update before editing this prompt.
event-dispatch-quit-and-stop-running-sessions = Quit and stop running sessions?
event-dispatch-quitting-this-tui-will-stop = Quitting this TUI will stop its running sessions.
event-dispatch-stay-in-codex = Stay in Codex
event-dispatch-stop-sessions-and-quit = Stop sessions and quit
event-dispatch-failed-to-interrupt-task = Failed to interrupt task: { $error }
event-dispatch-failed-to-pause-task-goal = Failed to pause task goal: { $error }
event-dispatch-logout-failed = Logout failed: { $err }
event-dispatch-failed-to-start-turn = Failed to start turn: { $err }
event-dispatch-voice-conversation-failed = Voice conversation failed: { $err }
event-dispatch-no-changes-detected = No changes detected.
event-dispatch-mcp-sign-in-is-starting = MCP sign-in is starting. Wait for it to finish before trying again.
event-dispatch-voice-conversation-failed-2 = Voice conversation failed: { $error }
event-dispatch-cloud-model-catalog-account-changed = Cloud model catalog account changed.
event-dispatch-failed-to-save-default-model = Failed to save default model: { $error }
event-dispatch-model-changed-to-for-this = Model changed to { $model } { $effort } for this conversation
event-dispatch-sandbox-ready = Sandbox ready
event-dispatch-codex-can-now-safely-edit = Codex can now safely edit files and execute commands in your computer
event-dispatch-daybreak-has-been-turned-off = Daybreak has been turned off for new turns because it is not available for { $model } with your current account and server. Prior cybersecurity context in this thread may cause a refusal. You can switch back to a model where you have Daybreak access and use /daybreak to re-enable it.
event-dispatch-cyber-models-default-to-approve = Cyber models default to "Approve for me" for safety reasons.
event-dispatch-the-selected-cloud-model-does = The selected Cloud model does not support that speed.
event-dispatch-selected-speed-for-the-next = Selected { $label } speed for the next new turn in this conversation
event-dispatch-the-cloud-account-changed-reopen = The Cloud account changed. Reopen the conversation before choosing a speed.
event-dispatch-service-tier-set-to = Service tier set to { $service_tier }
event-dispatch-service-tier-cleared = Service tier cleared
event-dispatch-failed-to-set-permission-profile = Failed to set permission profile: { $err }
event-dispatch-failed-to-save-rate-limit = Failed to save rate limit reminder preference: { $err }
event-dispatch-failed-to-save-plan-mode = Failed to save Plan mode reasoning effort: { $err }
event-dispatch-failed-to-save-model-migration = Failed to save model migration prompt preference: { $err }
event-dispatch-close-the-side-conversation-before = Close the side conversation before forking another task.
event-dispatch-failed-to-rename-task = Failed to rename task: { $error }
event-dispatch-could-not-open-file = Could not open file: { $error }
event-dispatch-failed-to-update-skill-config = Failed to update skill config for { $path_display }: { $err }
event-dispatch-failed-to-update-app-config = Failed to update app config for { $id }: { $err }
event-dispatch-environment = Environment:{" "}
event-dispatch-reason = Reason:{" "}
event-dispatch-permission-rule = Permission rule:{" "}
event-dispatch-server = Server:{" "}
event-dispatch-failed-to-save-status-line = Failed to save status line settings: { $error }
event-dispatch-failed-to-save-terminal-title = Failed to save terminal title items: { $err }
event-dispatch-failed-to-open-setting-because = Failed to open setting because terminal input could not be cleared: { $err }
event-dispatch-failed-to-save-theme = Failed to save theme: { $err }
event-dispatch-wait-for-the-current-task = Wait for the current task to finish before running /recap.
event-dispatch-failed-to-save-shortcut = Failed to save shortcut: { $err }
event-dispatch-failed-to-refresh-shortcuts = Failed to refresh shortcuts: { $err }
event-dispatch-removed-custom-shortcut-for = Removed custom shortcut for `{ $context }.{ $action }`.
event-dispatch-failed-to-remove-shortcut = Failed to remove shortcut: { $err }
event-dispatch-a-thread-must-start-before = A thread must start before it can be archived.
event-dispatch-archiving = Archiving…
event-dispatch-failed-to-archive-current-thread = Failed to archive current thread: { $error }
event-dispatch-a-thread-must-start-before-2 = A thread must start before it can be deleted.
event-dispatch-failed-to-delete-current-thread = Failed to delete current thread: { $err }
event-dispatch-worktree-removed = Removed worktree at { $path }. Thread history was kept.
event-dispatch-worktree-remove-failed = Could not remove worktree at { $path }: { $error }
event-dispatch-not-directory = Not a directory: { $path }
event-dispatch-cannot-access-dir = Cannot access directory { $path }: { $error }
event-dispatch-save-reviewer-failed = Failed to save approvals reviewer: { $error }

## bottom_pane/request_user_input/mod.rs

request-input-add-notes = Add notes
request-input-answer-placeholder = Type your answer (optional)
request-input-select-option = Select an option to add notes
request-input-none-of-above = None of the above
request-input-other-desc = Optionally, add details in notes (tab)
request-input-go-back-desc = Return to the first unanswered question
request-input-go-back = Go back
request-input-proceed = Proceed
request-input-submit-unanswered = 
    { $count ->
        [one] Submit with 1 unanswered question
       *[other] Submit with { $count } unanswered questions
    }
request-input-auto-resolves = auto-resolves in { $time }
request-input-question-progress = Question { $idx }/{ $total }
request-input-question-progress-unanswered = Question { $idx }/{ $total } ({ $unanswered } unanswered)
request-input-no-questions = No questions
request-input-to-add-notes = to add notes
request-input-or = {" "}or{" "}
request-input-to-clear-notes = {" "}to clear notes
request-input-to-submit-answer = to submit answer
request-input-to-submit-all = to submit all
request-input-to-navigate = to navigate questions
request-input-change-question = change question
request-input-to-interrupt = to interrupt

## bottom_pane/mod.rs

bottom-pane-answer-the-questions-to-continue = Answer the questions to continue.
bottom-pane-install-this-app-in-your = Install this app in your browser, then return here.
bottom-pane-enable-this-app-to-use = Enable this app to use it for the current request.
bottom-pane-respond-to-the-tool-suggestion = Respond to the tool suggestion to continue.
bottom-pane-respond-to-the-mcp-server = Respond to the MCP server request to continue.

## tui/src/app/daybreak.rs
daybreak-unavailable-for-account = Daybreak availability could not be confirmed for this account.
daybreak-save-thread-failed = Failed to save Daybreak for this thread: { $error }
daybreak-read-thread-failed = Failed to read the thread to save Daybreak: { $error }
daybreak-default-preference-setting = default Daybreak preference
daybreak-read-default-failed = Failed to read the default Daybreak preference: { $error }
daybreak-save-default-failed = Failed to save the default Daybreak preference: { $error }
daybreak-on-applies-to-new-turns = Daybreak on. Applies to new turns.
daybreak-off-applies-to-new-turns = Daybreak off. Applies to new turns.

## tui/src/app/quick_dials.rs
quick-dials-defined-outside-config = Preset { $slot } is defined outside your user config. Edit its source configuration instead.
quick-dials-update-failed = Failed to update preset { $slot }: { $error }
quick-dials-saved = Saved preset { $slot }.
quick-dials-deleted = Deleted preset { $slot }.

## tui/src/app/transcript_export.rs
transcript-export-no-active = No active conversation to export.
transcript-export-saved = Saved conversation to { $path }
transcript-export-load-failed = could not load conversation: { $error }
transcript-export-load-history-failed = could not load conversation history: { $error }
transcript-export-no-content = No conversation content to export.
transcript-export-no-home-dir = could not determine the home directory
transcript-export-create-failed = could not create { $path }: { $error }
transcript-export-write-failed = could not write { $path }: { $error }

## tui/src/app/history_ui.rs
history-ui-open-browser-failed = Failed to open browser for { $url }: { $error }

## tui/src/app/working_directory.rs
working-directory-idle-required = Changing directories requires an idle primary session without queued input.
working-directory-remote-unsupported = Changing directories is not supported for remote workspaces or remote execution environments.
working-directory-task-not-replaceable = This task cannot be safely replaced.
working-directory-wait-permissions = Wait for permissions to update before changing directories.
working-directory-named-profile-unsupported = Changing directories with a named profile is not supported.
working-directory-unconfirmed-profile-unsupported = Changing directories with an unconfirmed named profile is not supported.
working-directory-mcp-loading = MCP inventory is still loading.
working-directory-agent-running = Cannot change: another agent is running.
working-directory-history-not-saved = Conversation history is not saved.
working-directory-profile-differs = Permission profile has different settings.
working-directory-profile-not-preserved = Permission profile cannot be preserved by /cd.
working-directory-agent-status-failed = Cannot check agent status.
working-directory-task-running = Cannot change: a task is running.
working-directory-not-applied = Requested directory or permissions not applied.
working-directory-offline-checkout = Cannot continue into the new worktree while the session is offline or blocked by a policy warning. An unused checkout was created at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository.
working-directory-source-changed-checkout = The source conversation changed while creating the worktree. An unused checkout was created at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository.
working-directory-fork-instructions-differ = Cannot fork into this worktree because developer instructions differ. Start a new conversation instead. An unused checkout was created at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository.
working-directory-retained-checkout = Could not start a session in the managed worktree. A checkout was retained at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.
working-directory-load-failed = Cannot load { $cwd }: { $error }
working-directory-read-defaults-failed = Cannot read defaults: { $error }
working-directory-task-status-failed = Cannot check task status: { $error }
working-directory-change-failed = Failed to change: { $error }
working-directory-register-ownership-failed = Cannot register managed worktree ownership: { $error }
working-directory-name-session-failed = Failed to name the worktree session: { $error }
working-directory-change-dirs-failed = Cannot change directories: { $error }
working-directory-restore-failed = Could not restore session: { $error }
working-directory-changed = Working directory changed to: { $path }

## tui/src/clipboard_copy/tmux.rs
clipboard-tmux-unavailable = tmux is unavailable in the system PATH
clipboard-tmux-resolve-path-failed = failed to resolve system PATH: { $error }
clipboard-tmux-no-pane = tmux clipboard forwarding is unavailable: no current pane
clipboard-tmux-spawn-failed = failed to spawn tmux: { $error }
clipboard-tmux-open-stdin-failed = failed to open tmux stdin
clipboard-tmux-write-failed = failed to write to tmux: { $error }
clipboard-tmux-wait-failed = failed to wait for tmux: { $error }
clipboard-tmux-exit-status = tmux exited with status { $status }
clipboard-tmux-failed = tmux failed: { $stderr }
clipboard-tmux-disabled = tmux clipboard forwarding is disabled
clipboard-tmux-no-client = tmux clipboard forwarding is unavailable: no attached client
clipboard-tmux-missing-ms = tmux clipboard forwarding is unavailable: missing Ms capability
clipboard-tmux-output-not-utf8 = tmux output was not UTF-8: { $error }

## tui/src/clipboard_paste.rs
clipboard-paste-unavailable = clipboard unavailable: { $error }
clipboard-paste-no-image = no image on clipboard: { $error }
clipboard-paste-encode-failed = could not encode image: { $error }
clipboard-paste-io-error = io error: { $error }
clipboard-paste-android-unsupported = clipboard image paste is unsupported on Android

## tui/src/model_migration.rs
model-migration-try-new = Try new model
model-migration-use-existing = Use existing model
model-migration-upgrade-heading = Codex just got an upgrade. Introducing { $name }.
model-migration-recommended = { $name } is recommended for better performance and reliability.
model-migration-recommend-switch = We recommend switching from { $current } to { $target }.
model-migration-learn-more = { $description } Learn more about { $name } at{" "}
model-migration-keep-existing = You can continue using { $current } if you prefer.
model-migration-hint-confirm = {" "}confirm ·{" "}
model-migration-hint-continue = {" "}continue ·{" "}
model-migration-hint-quit = {" "}quit

## tui/src/oss_selection.rs
oss-selection-lmstudio-desc = Local LM Studio server (default port 1234)
oss-selection-ollama-desc = Local Ollama server (Responses API, default port 11434)
oss-selection-title = Select an open-source provider
oss-selection-subtitle = Choose a local AI server for this session.
oss-selection-running = ● Running
oss-selection-not-running = ○ Not running
oss-selection-unknown = ? Unknown
oss-selection-hint-choose = {" "}choose ·{" "}
oss-selection-hint-select = {" "}select ·{" "}
oss-selection-hint-lmstudio = {" "}LM Studio ·{" "}
oss-selection-hint-exit = {" "}exit

## tui/src/startup_draft_layout.rs
startup-draft-resuming = {"  "}Resuming session…
startup-draft-forking = {"  "}Forking session…

## tui/src/token_usage.rs
token-usage-summary = Token usage: total={ $total } input={ $input }{ $cached } output={ $output }{ $reasoning }
token-usage-cached = {" "}(+ { $count } cached)
token-usage-reasoning = {" "}(reasoning { $count })

## tui/src/app_backtrack.rs
app-backtrack-no-previous-message = No previous message to edit.
app-backtrack-edit-failed = Failed to edit the selected prompt: { $error }

## tui/src/app_backtrack.rs (continued)
app-backtrack-steer = the selected prompt is a steer and cannot be edited independently
app-backtrack-in-progress = the selected prompt belongs to a turn that is still in progress
app-backtrack-mismatch = the selected transcript prompt no longer matches the persisted thread
app-backtrack-not-found = the selected prompt was not found in the persisted thread
app-backtrack-boundary-missing = the loaded history boundary no longer exists


## tui/src/app/feedback_upload.rs
feedback-upload-failed-to-upload = Failed to upload feedback
feedback-upload-failed-to-save-zip = Failed to save feedback ZIP
feedback-upload-no-codex-home = The app server did not report its Codex home directory
feedback-upload-create-dir-failed = Could not create the feedback directory at { $directory }
feedback-upload-update-server = Update the Codex server to save a feedback ZIP

## tui/src/app/model_defaults.rs
model-defaults-setting-default-model = default model and reasoning effort
model-defaults-model-changed = Model changed to { $model }
model-defaults-failed-to-save = Failed to save default model: { $error }
model-defaults-open-cloud-first = Open the Cloud conversation before choosing a model.
model-defaults-choose-supported-cloud = Choose a supported Cloud model and effort at the current speed.
model-defaults-speed-fast = fast
model-defaults-speed-standard = standard
model-defaults-speed-unsupported = { $model } does not support { $tier } speed. Cleared the pending speed change.
model-defaults-cloud-account-changed = The Cloud account changed. Reopen the conversation before choosing a model.
model-defaults-cloud-no-model-change = This cloud conversation does not support changing its model from the TUI.
model-defaults-needs-settings-update = Model selection requires an app server with thread/settings/update support.
model-defaults-saved-but-overridden = Saved { $setting }, but a higher-priority configuration layer overrides the saved value.
model-defaults-for-this-session-only = {" "}for this session only

## tui/src/app/new_session.rs
new-session-only-cloud-cancel = Only a Cloud switch can be canceled

## tui/src/app/resume_preparation.rs
resume-preparation-still-archiving = This task is still being archived.
resume-preparation-failed-load-history = Failed to load chat history: { $read_err }
resume-preparation-failed-resume = Failed to resume session from { $path_display }: { $err }

## tui/src/app/recap.rs
recap-failure = Could not generate a recap. Please try again.
recap-cloud-unavailable = Conversation recaps are not available for cloud conversations.
recap-in-progress = A recap is already being generated.
recap-empty-history = There is no conversation history to recap.

## tui/src/app_server_session/side_chat_history.rs
side-chat-history-outside-attachment = The conversation has an attachment outside the TUI sidebar. Remove it before deleting the conversation.

## tui/src/bottom_pane/hooks_browser_view.rs
hooks-browser-view-hooks = Hooks
hooks-browser-view-lifecycle-hooks = Lifecycle hooks from config and enabled plugins.
hooks-browser-view-event-hooks = { $event } hooks
hooks-browser-view-turn-hooks-on-off = Turn hooks on or off. Your changes are saved automatically.
hooks-browser-view-event = Event
hooks-browser-view-installed = Installed
hooks-browser-view-active = Active
hooks-browser-view-review = Review
hooks-browser-view-description = Description
hooks-browser-view-issues = Issues
hooks-browser-view-unnamed-hook = Unnamed hook
hooks-browser-view-modified-suffix = {" "}· modified
hooks-browser-view-new-suffix = {" "}· new
hooks-browser-view-no-hooks = No hooks installed for this event.
hooks-browser-view-matcher = Matcher
hooks-browser-view-source = Source
hooks-browser-view-command = Command
hooks-browser-view-mode = Mode
hooks-browser-view-async = Async
hooks-browser-view-sync = Sync
hooks-browser-view-mcp-server = MCP Server
hooks-browser-view-mcp-tool = MCP Tool
hooks-browser-view-handler = Handler
hooks-browser-view-prompt = Prompt
hooks-browser-view-agent = Agent
hooks-browser-view-timeout = Timeout
hooks-browser-view-unlimited = unlimited
hooks-browser-view-limit-tokens = limit: { $limit } approximate tokens
hooks-browser-view-context = Context
hooks-browser-view-trust = Trust
hooks-browser-view-trust-all = {" "}trust all ·{" "}
hooks-browser-view-review-hint = {" "}review ·{" "}
hooks-browser-view-details = {" "}details ·{" "}
hooks-browser-view-close = {" "}close
hooks-browser-view-managed-always-on = Managed (always on) ·{" "}
hooks-browser-view-trust-hint = {" "}trust ·{" "}
hooks-browser-view-toggle = {" "}toggle ·{" "}
hooks-browser-view-back = {" "}back
hooks-browser-view-trust-managed = Managed
hooks-browser-view-trust-trusted = Trusted
hooks-browser-view-trust-untrusted = New hook - review required
hooks-browser-view-trust-modified = Modified since last trusted - review required
hooks-browser-view-desc-before-tool = Before a tool executes
hooks-browser-view-desc-permission-requested = When permission is requested
hooks-browser-view-desc-after-tool = After a tool executes
hooks-browser-view-desc-before-compaction = Before context compaction
hooks-browser-view-desc-after-compaction = After context compaction
hooks-browser-view-desc-before-model-request = Before a model request
hooks-browser-view-desc-after-model-request = After a model request, including failures
hooks-browser-view-desc-session-start = When a new session starts
hooks-browser-view-desc-session-end = Right before a session ends
hooks-browser-view-desc-prompt-submit = When the user submits a prompt
hooks-browser-view-desc-subagent-start = When a subagent is created
hooks-browser-view-desc-subagent-stop = Right before a subagent ends its turn
hooks-browser-view-desc-stop = Right before Codex ends its turn
hooks-browser-view-desc-interrupt = Right before an interrupted turn is aborted
hooks-browser-view-environment = Environment - { $id }
hooks-browser-view-plugin-id = Plugin - { $plugin_id }
hooks-browser-view-plugin = Plugin
hooks-browser-view-src-admin-config = Admin config
hooks-browser-view-src-user-config = User config
hooks-browser-view-src-project-config = Project config
hooks-browser-view-src-session-flags = Session flags
hooks-browser-view-src-cloud-managed-config = Cloud-managed config
hooks-browser-view-src-unknown-source = Unknown source
hooks-browser-view-needs-review =
    { $count ->
        [one] 1 hook needs review before it can run.
       *[other] { $count } hooks need review before they can run.
    }

## tui/src/bottom_pane/textarea.rs
textarea-vim-normal = Vim: Normal
textarea-vim-insert = Vim: Insert
textarea-vim-replace = Vim: Replace

## tui/src/clipboard_copy.rs
clipboard-copy-copied-to-clipboard = Copied { $label } to clipboard
clipboard-copy-unconfirmed = Copy unconfirmed; /export saves chat
clipboard-copy-copying = Copying { $label }…
clipboard-copy-already-in-progress = Copy already in progress
clipboard-copy-nothing-to-copy = nothing to copy: the selected content is empty

## tui/src/debug_config.rs
debug-config-session-runtime = Session runtime:
debug-config-layer-stack = Config layer stack (lowest precedence first):
debug-config-none-indented = {"  "}<none>
debug-config-disabled = disabled
debug-config-enabled = enabled
debug-config-reason = {"     "}reason: { $reason }
debug-config-requirements = Requirements:
debug-config-none-bullet = {"     "}- <none>
debug-config-mdm-value = MDM value
debug-config-enterprise-value = Enterprise-managed config value
debug-config-layer-value = Layer value
debug-config-requirement-line = {"  "}- { $name }: { $value } (source: { $source })
debug-config-unspecified = <unspecified>
debug-config-empty = <empty>
debug-config-unset = <unset>

## tui/src/diff_render.rs
diff-render-added = Added
diff-render-deleted = Deleted
diff-render-edited = Edited
diff-render-file-count =
    { $count ->
        [one] {" "}{ $count } file{" "}
       *[other] {" "}{ $count } files{" "}
    }

## tui/src/external_agent_config_migration/render.rs
external-agent-migration-bring-over = Bring over supported setup from another coding agent.
external-agent-migration-may-add-files = Codex may add files to your current project folder.
external-agent-migration-not-changed = Your existing setup will not be changed.
external-agent-migration-choose-items = Choose items to import.
external-agent-migration-import-setup = Import setup
external-agent-migration-choose-what = Choose what to import
external-agent-migration-selected-of =
    { $total ->
        [one] Selected { $selected } of { $total } item.
       *[other] Selected { $selected } of { $total } items.
    }
external-agent-migration-use = Use{" "}
external-agent-migration-to-move = {" "}to move,{" "}
external-agent-migration-to-select = {" "}to select,{" "}
external-agent-migration-to-customize = {" "}to customize
external-agent-migration-press = Press{" "}
external-agent-migration-to-continue = {" "}to continue,{" "}
external-agent-migration-to-go-back = {" "}to go back
external-agent-migration-to-toggle = {" "}to toggle,{" "}

## tui/src/onboarding/gateway.rs
gateway-use-custom-provider = Use a custom provider or gateway

## tui/src/pager_overlay.rs
pager-overlay-to-scroll = to scroll
pager-overlay-to-page = to page
pager-overlay-to-jump = to jump
pager-overlay-close = close

## tui/src/pets/ambient.rs
ambient-running = Running
ambient-needs-input = Needs input
ambient-ready = Ready
ambient-blocked = Blocked
ambient-thinking = Thinking

## tui/src/pets/image_protocol.rs
image-protocol-tmux = Pets are unavailable in this tmux pane. Use fullscreen Codex with allow-passthrough all, or run Codex outside tmux.
image-protocol-zellij = Pets are disabled in Zellij. Terminal images don’t stay reliably pane-local in Zellij. Run Codex outside Zellij to use pets.
image-protocol-iterm2-too-old = Pets require iTerm2 3.6 or newer. Upgrade iTerm2 to use terminal pets.
image-protocol-terminal = Pets aren’t available in this terminal. Terminal pets need image support, and this terminal environment doesn’t expose a supported image protocol. Try a terminal with Kitty graphics or Sixel support, or run Codex outside tmux.

## tui/src/transcript_view/composer_gap.rs
composer-gap-copied-chars-host = Copied { $characters } chars to host clipboard
composer-gap-copied-chars = Copied { $characters } chars
composer-gap-copied = Copied
composer-gap-copying-chars = Copying { $characters } chars…
composer-gap-copying = Copying…
composer-gap-already-in-progress = Copy already in progress
composer-gap-in-progress = Copy in progress
composer-gap-busy = Copy busy
composer-gap-sent-to-terminal = Copy sent to terminal · paste to verify
composer-gap-sent-verify = Copy sent · verify paste
composer-gap-unconfirmed = Copy unconfirmed
composer-gap-failed-try-again = Copy failed · try again
composer-gap-failed = Copy failed

## tui/src/ui_consts.rs
ui-consts-to-view-transcript = { $key } to view transcript


## tui/src/additional_dirs.rs

additional-dirs-ignored-warning = Ignoring --add-dir ({ $joined_paths }) because the effective permissions do not allow additional writable roots. Switch to workspace-write or danger-full-access to allow them.

## tui/src/app/provider/editor.rs

editor-edit-provider = Edit { $name }
editor-example-team-gateway = Team gateway
editor-save-focused = › Save
editor-save = {" "}{" "}Save
editor-hint-env-var-name = Enter the environment variable name on the app server.
editor-hint-auth-managed = Authentication is managed on the app server.
editor-required-note = {"*"} Required
editor-complete-suggestion = {" "}complete { $suggestion }
editor-key-move = {" "}move ·{" "}
editor-key-next-save = {" "}next/save ·{" "}
editor-key-normal-mode = {" "}normal mode
editor-key-back = {" "}back

## tui/src/app/session_picker.rs

session-picker-open-failed = Failed to open session picker: { $err }
session-picker-unable-to-resume = Unable to resume session
session-picker-return-to-center = Return to command center

## tui/src/app/side_focus.rs

side-focus-chat-still-closing = Side chat is still closing. Try again in a moment.

## tui/src/app/startup_prompts.rs

startup-prompts-skills-skipped = Skipped loading { $error_count } skill(s) due to invalid SKILL.md files.
startup-prompts-project-config-disabled = Project-local config, hooks, and exec policies are disabled in the following folders until the project is trusted, but skills still load.{"\u000A"}
startup-prompts-no-longer-offered = no longer offered in Codex
startup-prompts-no-longer-available = no longer available
startup-prompts-model-migration = { $current_name } is { $availability }{"\u000A"}{"\u000A"}Codex now uses { $target_name } in place of { $current_name }. Switch to { $target_name } to continue.{"\u000A"}

## tui/src/app/thread_attachment.rs

thread-attachment-not-yet-available = Agent thread { $thread_id } is not yet available for replay or live attach.

## tui/src/app/thread_routing.rs

thread-routing-main-default = Main [default]
thread-routing-agent = Agent
thread-routing-already-viewing = Already viewing { $label }.
thread-routing-no-active-thread = No active thread is available.
thread-routing-read-only = This conversation is read-only or unavailable; no operation was sent.
thread-routing-not-available-yet = Not available in TUI yet for thread { $thread_id }.
thread-routing-interrupt-failed = Failed to interrupt turn: { $error }
thread-routing-rename-failed = Failed to rename thread: { $error }
thread-routing-shell-unavailable-cloud = Shell commands starting with ! are unavailable in Cloud conversations.
thread-routing-resolve-request-failed = Failed to resolve app-server request for thread { $thread_id }: { $err }
thread-routing-agent-closed-switched = Agent thread { $closed_thread_id } closed. Switched back to main thread.
thread-routing-agent-closed-switch-failed = Agent thread { $closed_thread_id } closed. Failed to switch back to main thread { $primary_thread_id }.

## tui/src/bottom_pane/voice_strip.rs

voice-strip-voice-prefix = voice{" "}
voice-strip-voice = voice
voice-strip-unmute = unmute
voice-strip-mute = mute{" "}{" "}
voice-strip-mic = {" "}{" "}mic{" "}
voice-strip-codex = {" "}{" "}codex{" "}

## tui/src/local_settings.rs

local-settings-invalid-audio = Invalid machine audio settings: { $error }

## tui/src/markdown_render/mermaid.rs

mermaid-unsupported = This Mermaid diagram uses features the terminal renderer doesn't support.
mermaid-too-wide = This Mermaid diagram doesn't fit the current terminal width.
mermaid-limit = This Mermaid diagram exceeds the terminal renderer's size limits.

## tui/src/pets/picker.rs

picker-select-pet = Select Pet
picker-choose-pet = Choose a pet to wake in the terminal.
picker-filter-pets-placeholder = Type to filter pets...
picker-disable-pets = Disable terminal pets

## tui/src/sidebar_chooser.rs

sidebar-chooser-resize-hint-short = ←/→ resize · Enter/Esc done
sidebar-chooser-hint-retry = retry
sidebar-chooser-hint-back = back
sidebar-chooser-hint-scroll = scroll
sidebar-chooser-hint-open = open
sidebar-chooser-hint-cancel-opening = cancel opening
sidebar-chooser-hint-path = path
sidebar-chooser-hint-move = move
sidebar-chooser-hint-hide-pane = hide pane
sidebar-chooser-side-n = Side { $number }
sidebar-chooser-side-chat = Side chat

## tui/src/startup_draft.rs

startup-draft-did-not-open = New conversation did not open
startup-draft-kept = draft kept
startup-draft-loading = loading

## tui/src/status/card.rs

card-label-model = Model
card-label-directory = Directory
card-label-permissions = Permissions
card-label-model-provider = Model provider
card-label-account = Account
card-label-chatgpt-workspace = ChatGPT workspace
card-label-thread-name = Thread name
card-label-session = Session
card-label-forked-from = Forked from
card-label-collaboration-mode = Collaboration mode
card-label-token-usage = Token usage
card-label-context-window = Context window
card-label-limits = Limits
card-label-warning = Warning
card-label-server = Server
card-limits-not-available = not available for this account
card-limits-stale-run-status = limits may be stale - run /status again shortly.
card-limits-stale-new-turn = limits may be stale - start new turn to refresh.
card-limits-refresh-requested = refresh requested; run /status again shortly.
card-limits-no-data-yet = data not available yet
card-token-total = {" "}total{" "}
card-token-input = {" "}input
card-token-plus = {" "}+{" "}
card-token-output = {" "}output
card-context-used-of = {" "}used /{" "}
card-context-percent-left = { $percent }% left
card-resets-at = (resets { $resets_at })
card-permissions-read-only-network = Read Only with network access
card-permissions-read-only = Read Only
card-permissions-workspace = Workspace{ $suffix } ({ $approval })
card-permissions-workspace-network = Workspace with network access{ $suffix } ({ $approval })
card-permissions-full-access = Full Access
card-permissions-no-sandbox = No Sandbox ({ $approval })
card-permissions-profile = Profile { $id } ({ $sandbox }, { $approval })
card-permissions-read-only-approval = Read Only ({ $approval })
card-permissions-custom = Custom ({ $sandbox }, { $approval })
card-approval-approve-for-me = Approve for me
card-approval-ask = Ask for approval
card-account-api-key = API key configured (run codex login to use ChatGPT)
card-local-background-server = Local background server
card-visit = Visit{" "}
card-for-up-to-date = {" "}for up-to-date
card-rate-limits-info = information on rate limits and credits

## tui/src/transcript_view/layout.rs

layout-show-less = − Show less
layout-hide-details = − Hide details
layout-show-details = + Show details
layout-more-lines = 
    { $count ->
        [one] + { $count } line
       *[other] + { $count } lines
    }
layout-to-expand-hint = {" "}({ $key } to expand)

## tui/src/thread_transcript/tools.rs

tools-in-progress-at-last-update = In progress at last update
tools-completed = Completed
tools-failed = Failed
tools-declined = Declined
tools-terminal-interaction = Terminal interaction · { $status }
tools-exit-code = {" "}· exit { $code }
tools-completed-result-unavailable = Completed · result unavailable
tools-failed-result-unavailable = Failed · result unavailable
tools-mcp-tool = mcp tool: { $server }/{ $tool } · { $status }
tools-interrupted = Interrupted
tools-agent-tool = agent tool: { $tool } · { $status }
tools-declined-command = Declined · { $command }

## tui/src/update_prompt.rs

update-prompt-update-available = Update available
update-prompt-release-notes = Release notes:{" "}
update-prompt-update-now = Update now (runs `{ $update_command }`)
update-prompt-skip = Skip
update-prompt-skip-until-next = Skip until next version
update-prompt-continue = {" "}continue ·{" "}
update-prompt-skip-hint = {" "}skip

## app/experimental_features.rs
app-experimental-features-label-subagents = Subagents
app-experimental-features-label-memories = Memories
app-experimental-features-saved-new-threads = { $label } setting saved on the server for new threads. This thread is unchanged. Project or task settings may override it.
app-experimental-features-saved-overridden = { $label } setting was saved but is overridden: { $detail }
app-experimental-features-save-failed = Failed to save { $label } setting: { $detail }
app-experimental-features-save-in-progress = An experimental feature save is still in progress. Retry after it finishes.

## app/managed_worktree_creation.rs
managed-worktree-daemon-cannot-check = The local Codex service cannot check background terminals. Run `codex app-server daemon update`, then restart Codex.
managed-worktree-background-blocks-cd = Active background terminals block /cd.
managed-worktree-untrusted-source = Cannot create a worktree from an explicitly untrusted source.
managed-worktree-local-only = Managed worktrees are only supported for local sessions.
managed-worktree-needs-idle = Creating a worktree requires an idle primary session without queued input.
managed-worktree-already-creating = A worktree is already being created.
managed-worktree-mcp-loading = MCP inventory is still loading.
managed-worktree-agent-running = Cannot change: another agent is running.
managed-worktree-history-not-saved = Conversation history is not saved.
managed-worktree-source-changed = Cannot continue into the new worktree while the source session changed or is unavailable.
managed-worktree-source-config-changed = Cannot continue into the new worktree because source configuration changed.
managed-worktree-not-trusted = The new worktree is not trusted; run Codex there.
managed-worktree-task-failed = Worktree creation task failed: { $error }
managed-worktree-load-config-failed = Cannot load the new worktree configuration: { $error }
managed-worktree-retained-checkout = { $reason } A checkout was retained at { $path }; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.

## app/pets.rs
app-pets-disable-failed = Failed to disable pets: { $err }
app-pets-save-failed = Failed to save pet selection: { $err }
app-pets-load-failed = Failed to load pet: { $err }

## app/provider/write.rs
provider-write-already-saving = A provider change is already saving.
provider-write-saving = Saving model provider…
provider-write-offline = Could not save provider: Reconnect to the app server and try again.
provider-write-saved-config = Saved provider configuration.
provider-write-update-server = Update the connected app server to change the default model provider.
provider-write-save-failed = Could not save provider: { $error }
provider-write-confirm-failed = Could not confirm whether { $name } was saved: { $error }
provider-write-now-use = New chats now use { $name }.
provider-write-saved-name = Saved { $name }. { $reason }

## app/startup_warnings.rs
startup-warnings-nfs-migration = Some Codex SQLite files on NFS have permissions other than 0600. Migration preserves these permissions and requires your confirmation. Before running `codex migrate-nfs-db`, stop Codex on every host that uses these databases.

## app/user_turn_submission.rs
user-turn-submission-label = Submission
user-turn-submission-label-cloud = Cloud submission
user-turn-submission-unconfirmed = { $label } could not be confirmed: { $error }

## app/realtime_settings.rs
realtime-settings-save-failed = Failed to save audio setting: { $error }
realtime-settings-input-overridden = Input device saved, but the input channel is overridden by another configuration layer. Update that override before starting voice.
realtime-settings-saved = Audio setting saved. Applies to your next voice conversation.
realtime-settings-overridden = Audio setting was saved but is overridden by another configuration layer.
realtime-settings-read-failed = Audio setting was saved, but effective settings could not be read: { $error }
realtime-settings-list-devices-failed = Could not list audio devices. Check your audio device and voice package.
realtime-settings-list-voices-failed = Failed to list voices: { $error }
realtime-settings-read-voice-failed = Failed to read voice settings: { $error }
realtime-settings-voice-overridden-no-read = the saved voice is overridden, but this server cannot read effective settings
realtime-settings-voice-not-applied = Voice preference was saved but not applied: { $detail }
realtime-settings-voice-read-failed = Voice preference was saved, but effective settings could not be read: { $error }
realtime-settings-voice-save-failed = Failed to save voice: { $error }

## bottom_pane/pending_thread_approvals.rs
pending-thread-approvals-needed = Approval needed in { $thread }
pending-thread-approvals-review = {" "}to review requests

## chatwidget/permissions_menu.rs
permissions-menu-missing-preset-read-only = Internal error: missing the 'read-only' approval preset.
permissions-menu-missing-preset-auto = Internal error: missing the 'auto' approval preset.
permissions-menu-missing-preset-full-access = Internal error: missing the 'full-access' approval preset.

## experimental_features.rs
experimental-features-err-request-failed = Experimental feature request failed
experimental-features-err-page-exceeds = Experimental feature page exceeds requested limit
experimental-features-err-cursor-repeated = Experimental feature pagination repeated a cursor
experimental-features-err-exceeded-pages = Experimental feature discovery exceeded 10 pages
experimental-features-err-discovery-timed-out = Experimental feature discovery timed out
experimental-features-err-discovery-interrupted-short = Feature discovery was interrupted
experimental-features-err-not-advertised = The server did not advertise experimental feature `{ $name }`
experimental-features-err-save-timed-out = Saving experimental features timed out; the write may still finish. Reopen /experimental to check.
experimental-features-err-save-failed = Failed to save experimental features. Reopen /experimental to check configured values before retrying.
experimental-features-err-readback-interrupted = Features were saved, but readback was interrupted
experimental-features-err-refresh-failed = Features were saved, but configured values could not be refreshed: { $error }
experimental-features-err-values-differ = Changes were saved, but the configured values differ from your selections. A higher-priority setting may override them.

## external_agent_config_migration/model.rs
external-agent-migration-group-tools = Tools & setup
external-agent-migration-group-tools-desc = Settings, instructions, integrations, agents, commands, and skills
external-agent-migration-group-current-project = Current project
external-agent-migration-group-projects = Projects ({ $project_count })
external-agent-migration-group-projects-desc = Add Codex files alongside your existing project files
external-agent-migration-group-sessions = Chat sessions ({ $session_count })
external-agent-migration-group-sessions-desc = Last 30 days of chats
external-agent-migration-type-instructions = Instructions
external-agent-migration-type-settings = Settings
external-agent-migration-type-skills = Skills
external-agent-migration-type-plugins = Plugins
external-agent-migration-type-mcp-servers = MCP servers
external-agent-migration-type-agents = Agents
external-agent-migration-type-hooks = Hooks
external-agent-migration-type-commands = Slash commands
external-agent-migration-type-memory = Memory
external-agent-migration-item-sessions = Recent chat sessions
external-agent-migration-type-sessions = Chat sessions
external-agent-migration-summary-entry = { $label } { $count }
external-agent-migration-count-skill =
    { $count ->
        [one] 1 skill
       *[other] { $count } skills
    }
external-agent-migration-count-mcp-server =
    { $count ->
        [one] 1 MCP server
       *[other] { $count } MCP servers
    }
external-agent-migration-count-agent =
    { $count ->
        [one] 1 agent
       *[other] { $count } agents
    }
external-agent-migration-count-hook =
    { $count ->
        [one] 1 hook
       *[other] { $count } hooks
    }
external-agent-migration-count-command =
    { $count ->
        [one] 1 slash command
       *[other] { $count } slash commands
    }
external-agent-migration-count-session =
    { $count ->
        [one] 1 chat session
       *[other] { $count } chat sessions
    }
external-agent-migration-count-memory =
    { $count ->
        [one] 1 memory
       *[other] { $count } memories
    }
external-agent-migration-count-with-names = { $summary }: { $names }

## goal_display.rs
goal-display-status-active = active
goal-display-status-paused = paused
goal-display-status-stalled = stalled
goal-display-status-usage-limited = usage limited
goal-display-status-budget-limited = limited by budget
goal-display-status-complete = complete
goal-display-objective = Objective: { $objective }
goal-display-time = Time: { $time }.
goal-display-tokens = Tokens: { $used }/{ $budget }.

## transcript_view/search_footer.rs
search-footer-find-prefix = Find:{" "}
search-footer-retry = { $previous } retry
search-footer-history-unavailable = History unavailable · { $retry_hint }
search-footer-next-hint = enter accept · { $previous } older · { $next } newer
search-footer-exhausted-hint = No more matches · { $next_hint }
search-footer-reading-nav = ctrl+p older · ctrl+n newer
search-footer-ctrl-p-older = ctrl+p older
search-footer-enter-accept = enter accept
search-footer-type-to-find = Type to find
search-footer-searching = Searching…
search-footer-loading = Loading…
search-footer-searching-earlier = Searching earlier history…
search-footer-no-more-reading = No more matches · ctrl+p older · ctrl+n newer
search-footer-no-matches = No matches
search-footer-query-limited = {" "}· query limited to 4 KiB
search-footer-reading-status = Find · { $status } · esc latest
search-footer-reading-compact = Find · { $compact } · esc latest
search-footer-esc-latest = esc latest
search-footer-full-transcript = { $status } · full transcript · esc cancel{ $limit }
search-footer-status-cancel = { $status } · esc cancel{ $limit }
search-footer-compact-cancel = { $compact } · esc cancel
search-footer-esc-cancel = esc cancel
search-footer-compact-esc = { $compact } · esc

## sidebar_image.rs
sidebar-image-unsupported-format = Unsupported image format
sidebar-image-tabs = tabs
sidebar-image-close = close
sidebar-image-preview-unavailable = Image preview unavailable in this terminal

## pets/preview.rs
pets-preview-loading = Loading preview...
pets-preview-disabled = Terminal pets disabled
pets-preview-no-pet = No pet will be shown.
pets-preview-unavailable = Preview unavailable

## resume_picker.rs
resume-picker-different-backend = This session belongs to a different backend. Open the session picker again.
resume-picker-thread = thread { $id }
resume-picker-title-resume = Resume a previous session
resume-picker-title-fork = Fork a previous session
resume-picker-action-resume = resume
resume-picker-action-fork = fork
resume-picker-sort-created = Created
resume-picker-sort-updated = Updated
resume-picker-no-transcript = No transcript available for this session
resume-picker-metadata-failed-path = Failed to read session metadata from { $path }
resume-picker-metadata-failed = Failed to read session metadata from selected session
resume-picker-backend-load-failed = Could not load one backend. Reopen the picker to retry.
resume-picker-load-more-failed = Could not load more sessions
resume-picker-preview-load-failed = Could not load transcript preview
resume-picker-save-view-failed = Failed to save view mode: { $err }
resume-picker-no-message-yet = (no message yet)
resume-picker-type-to-search = Type to search
resume-picker-search = Search: { $query }
resume-picker-status-compact = Status:
resume-picker-status = Status:{" "}
resume-picker-active = Active
resume-picker-archived = Archived
resume-picker-sort-compact = Sort:
resume-picker-sort = Sort:{" "}
resume-picker-filter-compact = Filter:
resume-picker-filter = Filter:{" "}
resume-picker-filter-cwd = Cwd
resume-picker-filter-repo = Repo
resume-picker-filter-all = All
resume-picker-hint-loading = loading
resume-picker-hint-transcript = transcript
resume-picker-hint-quit = quit
resume-picker-hint-restore = restore
resume-picker-hint-start-new = start new
resume-picker-hint-new = new
resume-picker-hint-exit = exit
resume-picker-hint-clear-search = clear search
resume-picker-hint-clear = clear
resume-picker-hint-dense-view = dense view
resume-picker-hint-comfortable-view = comfortable view
resume-picker-hint-dense = dense
resume-picker-hint-comfy = comfy
resume-picker-hint-archive = archive
resume-picker-hint-focus-sort-filter = focus sort/filter
resume-picker-hint-focus = focus
resume-picker-hint-change-option = change option
resume-picker-hint-option = option
resume-picker-hint-preview = preview
resume-picker-hint-expand = expand
resume-picker-hint-exp = exp
resume-picker-hint-browse = browse
resume-picker-loading-transcript = Loading transcript…
resume-picker-more-above = ↑ more
resume-picker-loading-more = ↓ loading more
resume-picker-more-below = ↓ more
resume-picker-loading-older = Loading older sessions…
resume-picker-no-branch = no branch
resume-picker-no-cwd = no cwd
resume-picker-loading-recent-transcript = Loading recent transcript...
resume-picker-label-session = Session:
resume-picker-label-created = Created:
resume-picker-label-updated = Updated:
resume-picker-label-directory = Directory:
resume-picker-label-branch = Branch:
resume-picker-label-conversation = Conversation:
resume-picker-no-preview = No transcript preview available
resume-picker-now = now
resume-picker-ago-seconds-short = { $seconds }s ago
resume-picker-ago-minutes-short = { $minutes }m ago
resume-picker-ago-hours-short = { $hours }h ago
resume-picker-ago-days-short = { $days }d ago
resume-picker-ago-second =
    { $count ->
        [one] 1 second ago
       *[other] { $count } seconds ago
    }
resume-picker-ago-minute =
    { $count ->
        [one] 1 minute ago
       *[other] { $count } minutes ago
    }
resume-picker-ago-hour =
    { $count ->
        [one] 1 hour ago
       *[other] { $count } hours ago
    }
resume-picker-ago-day =
    { $count ->
        [one] 1 day ago
       *[other] { $count } days ago
    }
resume-picker-searching = Searching…
resume-picker-scan-capped = Search scanned first { $count } sessions; more may exist
resume-picker-no-results = No results for your search
resume-picker-loading-sessions = Loading sessions…
resume-picker-no-sessions = No sessions yet
resume-picker-time-separator = { $relative } · { $timestamp }

## startup_hooks_review.rs
startup-hooks-review-trust-failed = Failed to trust hooks: { $error }
startup-hooks-review-one-changed = 1 hook is new or changed.
startup-hooks-review-many-changed = { $count } hooks are new or changed.
startup-hooks-review-title = Hooks need review
startup-hooks-review-warning = Hooks can run outside the sandbox after you trust them.
startup-hooks-review-trusting = Trusting hooks...
startup-hooks-review-confirm = confirm
startup-hooks-review-skip = skip
startup-hooks-review-review = Review hooks
startup-hooks-review-trust-all = Trust all and continue
startup-hooks-review-continue-without = Continue without trusting (hooks won't run)

## app/input.rs

input-no-editor = Cannot open external editor: set $VISUAL or $EDITOR before starting Codex.
input-open-editor-failed = Failed to open editor: { $err }
input-redraw-failed = Failed to redraw transcript: { $err }
input-unarchive-failed = Failed to unarchive thread: { $error }
input-task-running-title = Task is still running
input-task-running-subtitle = Choose what happens to the current task.
input-cancel-task = Cancel task
input-cancel-task-desc = Stop the current task and stay in Codex
input-run-background = Run in background
input-run-background-desc = Exit Codex and leave the task running
input-exit = Exit
input-exit-desc = Stop the current task and exit Codex
input-switch-side-failed = Failed to switch side conversation: { $err }
input-clear-ui-failed = Failed to clear terminal UI: { $err }

## app/platform_actions.rs

platform-sandbox-mixed = Windows sandbox setup is unavailable when local and remote executors are configured together.
platform-sandbox-config-unreadable = Could not read Windows sandbox configuration and requirements from the app server.
platform-sandbox-still-running = Windows sandbox setup is still running. Restart Codex to retry.
platform-sandbox-option-disallowed = That Windows sandbox option is disallowed by requirements.
platform-sandbox-timed-out = Windows sandbox setup request timed out. Waiting for completion; restart Codex if it does not finish.
platform-sandbox-response-lost = Windows sandbox setup response was lost. Waiting for completion or reconnection.
platform-sandbox-update-server = Update the local app server to set up the Windows sandbox.
platform-sandbox-failed = Windows sandbox setup failed.
platform-sandbox-not-started = Windows sandbox setup did not start.

## app/reconnect.rs

reconnect-reopen-cloud = Reopen Codex to reconnect the cloud backend.
reconnect-in-process = in-process sessions have no connection to restore
reconnect-initial-thread-unknown = The initial thread may have been created, but its ID was not received. Nothing was retried. Your prompt is editable; inspect your tasks before relaunching.
reconnect-stage-connecting = connecting to the server
reconnect-stage-loading-settings = loading session settings
reconnect-stage-resuming = resuming the conversation
reconnect-timed-out = Automatic reconnect timed out while { $stage }.
reconnect-agent-list-stale = Reconnecting — agent list is stale
reconnect-conversation-unavailable = This conversation is unavailable. Its cached transcript and draft remain here; input is paused. Open the agent picker or return to the parent to continue.
reconnect-sandbox-interrupted = Windows sandbox setup was interrupted. Restart Codex before using Agent mode.

## app/right_click_paste.rs

right-click-paste-inline = Unavailable in inline mode; use your terminal's paste command
right-click-paste-ssh = Unavailable over SSH; use your terminal's paste command
right-click-paste-vscode = VS Code handles pasting; use its terminal paste command
right-click-paste-wsl = Auto cannot identify this WSL terminal; use On or your terminal's paste command
right-click-paste-android = Unavailable on Android

## app/provider/editor/form.rs

form-read-only = This provider configuration is read-only.
form-enter-name = Enter a provider name.
form-enter-base-url = Enter a base URL.
form-enter-base-url-for-key = Enter a base URL to use this API key variable.
form-enter-env-var = Enter an environment variable name such as TEAM_API_KEY.
form-unrecognized-bedrock = Unrecognized Bedrock provider.

## app/server_version_notice.rs

server-version-notice-summary = Service v{ $server } { $comparison } Codex CLI v{ $client_version }{ $guidance }

## app/safety_buffering.rs

safety-buffering-wait-permissions = Wait for permissions to update before forking.
safety-buffering-retry-unavailable = Failed to retry with a faster model: original turn is unavailable.
safety-buffering-retry-failed = Failed to retry with a faster model: { $err }
safety-buffering-input-too-long = Cannot safely retry a turn whose input exceeds the bounded history page.

## app/safety_recourse.rs

safety-recourse-new-chat-failed = Couldn’t start a new chat for this rewrite. { $text }

## app/session_lifecycle.rs

session-lifecycle-no-agents = No agents available yet.
session-lifecycle-needs-approval = { $name } (needs approval)
session-lifecycle-subagents = Subagents
session-lifecycle-side-closing = Side chat is still closing. Try again in a moment.
session-lifecycle-thread-unavailable = Agent thread { $thread_id } is no longer available.
session-lifecycle-attach-failed = Failed to attach to agent thread { $thread_id }: { $err }
session-lifecycle-finish-sandbox = Finish Windows sandbox setup before switching threads.
session-lifecycle-already-active = Agent thread { $thread_id } is already active.
session-lifecycle-resume-live-failed = Agent thread { $thread_id } could not be resumed live. Replaying saved transcript.
session-lifecycle-thread-closed = Agent thread { $thread_id } is closed. Replaying saved transcript.

## app/sidebar_close.rs

sidebar-close-button-close = {" "}Close{" "}
sidebar-close-button-archive = {" "}Archive{" "}
sidebar-close-button-cancel = {" "}Cancel{" "}
sidebar-close-resize = Resize to review · Esc to cancel
sidebar-close-hint-choose = choose
sidebar-close-hint-confirm = confirm
sidebar-close-hint-cancel = cancel
sidebar-close-loss-stop-turn = stop the running turn
sidebar-close-loss-discard-input = discard unsent input
sidebar-close-loss-temp-history = lose this temporary chat’s history
sidebar-close-loss-remove-side = remove it from Side chats
sidebar-close-side-chat-n = Side chat { $index }
sidebar-close-verb-close = Close
sidebar-close-verb-archive = Archive
sidebar-close-and = {" "}and{" "}
sidebar-close-confirm = This will { $losses }. { $verb } “{ $title }”?
sidebar-close-archive-failed = Failed to archive side conversation { $target }: { $err }
sidebar-close-interrupt-failed = Failed to interrupt side conversation { $target }: { $err }
sidebar-close-close-failed = Failed to close side conversation { $target }: { $err }

## resume_picker/archive.rs

archive-no-thread-id = Selected session does not have a thread ID.
archive-use-archive-cmd = Use /archive to archive the current session and exit.
archive-session-failed = Failed to archive session: { $error }
archive-restore-failed = Failed to restore archived session: { $error }

## resume_picker/sources.rs

sources-no-backends = No session backends are available.
sources-backend-unavailable = The session's backend is unavailable.
sources-backend-disconnected = The session backend disconnected.

## worktree_browser.rs

worktree-browser-untitled = Untitled conversation

## session_queue_commands.rs

queue-no-daemon = --no-daemon cannot be used with codex queue. Queuing must discover the shared server to avoid writing through a separate server.
queue-embedded-conflict = cannot queue through an embedded app server while a local app-server daemon is running; remove configuration overrides or use --remote
queue-resume-warning = Warning: message was queued, but could not resume thread { $thread_id }: { $err }
queue-queued = Queued message { $id } for thread { $thread_id }.
queue-no-active-session = No active session found matching '{ $target }'.
queue-start-daemon = start a local daemon or specify a server with --remote

## multi_agents.rs

multi-agents-main-default = Main [default]
multi-agents-agent-cap = Agent
multi-agents-agent-lower = agent
multi-agents-resume-failed = Agent resume failed
multi-agents-resume-agent-failed = Failed to resume agent
multi-agents-started = Started{" "}
multi-agents-interacted = Interacted with{" "}
multi-agents-interrupted-prefix = Interrupted{" "}
multi-agents-completed-prefix = Completed{" "}
multi-agents-spawned = Spawned
multi-agents-spawn-failed = Agent spawn failed
multi-agents-sent-input-to = Sent input to
multi-agents-waiting-for = Waiting for
multi-agents-waiting-for-agents = Waiting for agents
multi-agents-waiting-for-n = 
    { $count ->
        [one] Waiting for { $count } agent
       *[other] Waiting for { $count } agents
    }
multi-agents-finished-waiting = Finished waiting
multi-agents-closed = Closed
multi-agents-resuming = Resuming
multi-agents-resumed = Resumed
multi-agents-none-completed = No agents completed yet
multi-agents-pending-init = Pending init
multi-agents-running = Running
multi-agents-interrupted = Interrupted
multi-agents-completed = Completed
multi-agents-agent-errored = Agent errored
multi-agents-shutdown = Shutdown
multi-agents-not-found = Not found
multi-agents-error = Error

## transcript_view/footer.rs

transcript-footer-loading = Loading…
transcript-footer-loading-sep = Loading… ·{" "}
transcript-footer-retry = Retry:{" "}
transcript-footer-new-activity = New activity
transcript-footer-new-dot = New ·{" "}
transcript-footer-loading-earlier = Loading earlier messages…
transcript-footer-new-activity-after = {" "}· New activity
transcript-footer-retry-after = {" "}· Retry:{" "}
transcript-footer-retry-history = Retry history:{" "}
transcript-footer-earlier-available = Earlier messages available. {" "}
transcript-footer-new-activity-sep = New activity ·{" "}

## app.rs

app-external-editor-hint = Save and close external editor to continue.


## src/app/side_pane.rs
side-pane-save-width-failed = Could not save sidebar width: { $err }

## src/app/sidebar_document.rs
sidebar-document-connection-changed = Connection changed. Reopen this file to try again.
sidebar-document-limit-bytes = Source preview limit is 1 MiB
sidebar-document-limit-lines = Source preview limit is { $max } lines
sidebar-document-opening = Opening…
sidebar-document-open-failed = Could not open file: { $error }
sidebar-document-esc-close = Esc close
sidebar-document-hint-read = read
sidebar-document-hint-find = find
sidebar-document-hint-link = link
sidebar-document-hint-select = select
sidebar-document-hint-clear-highlight = clear highlight
sidebar-document-hint-close = close

## src/app_backtrack/prompt_navigation.rs
prompt-navigation-hint-details = details
prompt-navigation-hint-scroll = scroll
prompt-navigation-hint-prompts = prompts
prompt-navigation-hint-rewind = rewind
prompt-navigation-hint-back = back
prompt-navigation-browsing-transcript = Browsing transcript
prompt-navigation-browsing = Browsing

## src/app_server_session/fs.rs
fs-preview-needs-workspace = File previews require a workspace on the app-server host
fs-not-utf8-text = Not a UTF-8 text file
fs-not-text = Not a text file
fs-preview-windows-unavailable = File previews on Windows hosts are unavailable
fs-workspace-changed = Workspace changed. Reopen the file to try again.
fs-preview-needs-host-path = File previews require a path on the app-server host

## src/app_server_session/rollout_history.rs
rollout-history-limited-notice = Showing up to 100 recent prompts and final replies. Intermediate messages and tool activity are unavailable.

## src/app_server_session/user_message_submission.rs
user-message-cloud-account-changed = The Cloud account changed. Reopen the conversation before sending.
user-message-failed-to-start-turn = Failed to start turn

## src/app/background_requests.rs
background-requests-section-openai-curated = OpenAI Curated
background-requests-section-workspace = Workspace
background-requests-section-shared-with-me = Shared with me

## src/chatwidget/streaming.rs

## src/daemon_recovery.rs
daemon-recovery-running-without = Running without the shared background server: { $reason }.
daemon-recovery-incompatible-settings = Background server has incompatible feature settings
daemon-recovery-cannot-use = Cannot use the background server
daemon-recovery-settings-persist = These settings persist and can disable functionality for other clients. Restart may interrupt active or queued work.
daemon-recovery-restart-will-use = Restart will use these shared feature settings:
daemon-recovery-and-more = {" "}{" "}and { $count } more
daemon-recovery-run-without = Run without daemon this time
daemon-recovery-restart-recommended = Restart with these settings (recommended)
daemon-recovery-restart = Restart with these settings
daemon-recovery-not-managed = This server is not managed by Codex.
daemon-recovery-cannot-resolve = Restart cannot resolve this compatibility check.
daemon-recovery-cancel = Cancel

## src/history_cell/patches.rs
patches-edited-file = Edited { $name }
patches-edited-files = Edited files
patches-failed-to-apply = Failed to apply patch
patches-error-details-unavailable = (error details unavailable)
patches-viewed-image = Viewed image{" "}
patches-image-generation-failed = Image generation failed
patches-generated-image-heading = Generated Image:
patches-saved-to = Saved to:{" "}
patches-generated-image = Generated image

## src/permission_discovery.rs
permission-discovery-not-available = Not available on this server
permission-discovery-disabled-by-requirements = Disabled by requirements
permission-discovery-cloud-server-selected = Cloud conversations use server-selected permissions.
permission-discovery-duplicate-profiles = The server returned duplicate permission profiles.
permission-discovery-pagination-limit = Permission discovery exceeded its pagination limit. Try /permissions again.
permission-discovery-timed-out = Permission discovery timed out. Try /permissions again.
permission-discovery-unsupported = This server does not support permission discovery. Upgrade the Codex server to use this menu.
permission-discovery-load-failed = Failed to load permissions: { $error }

## src/bottom_pane/selection_popup_common.rs
selection-popup-disabled-suffix = {" "}(disabled)

## src/public_widgets/composer_input.rs
composer-input-placeholder = Compose new task

## src/unarchive_prompt.rs
unarchive-prompt-title = This conversation is archived
unarchive-prompt-unarchive-and-resume = Unarchive and resume
unarchive-prompt-unarchive-and-fork = Unarchive and fork
unarchive-prompt-cancel = Cancel
unarchive-prompt-hint-continue = {" "}continue ·{" "}
unarchive-prompt-hint-cancel = {" "}cancel

## src/session_archive_commands.rs
session-archive-archived-named = Archived session { $name } ({ $id }).
session-archive-archived = Archived session { $id }.
session-archive-deleted-named = Deleted session { $name } ({ $id }).
session-archive-deleted = Deleted session { $id }.
session-archive-unarchived-named = Unarchived session { $name } ({ $id }).
session-archive-unarchived = Unarchived session { $id }.
session-archive-delete-cancelled = Delete cancelled.
session-archive-not-found-active-or-archived = No active or archived session found matching '{ $target }'.
session-archive-not-found-active = No active session found matching '{ $target }'.
session-archive-not-found-archived = No archived session found matching '{ $target }'.
session-archive-confirm-needs-terminal = cannot confirm session deletion without an interactive terminal; rerun with --force and a session UUID
session-archive-confirm-delete-named = Permanently delete session '{ $name }' ({ $id })?
session-archive-confirm-delete = Permanently delete session { $id }?
session-archive-cannot-undo = This cannot be undone. Subagent threads will also be deleted.
session-archive-continue-prompt = Continue? [y/N]:{" "}

## src/app_server_session.rs
app-server-session-no-models = No models are available. Set `model` explicitly or check your model catalog configuration.
app-server-session-cloud-permissions = Cloud conversations currently use server-selected permissions.
app-server-session-side-context-not-saved = Side conversation context was not saved
app-server-session-side-prepare-failed = Failed to prepare side conversation { $child }: { $message }
app-server-session-cloud-fork-unsupported = Cloud conversations cannot be forked in the TUI yet.
app-server-session-title-saving = An automatic title is still being saved. Try renaming the thread again.

## src/app/provider/fields.rs
provider-field-name = Name
provider-field-base-url = Base URL
provider-field-api-key-variable = API key variable
provider-field-external-auth = External authentication
provider-field-aws-profile = AWS profile
provider-field-aws-region = AWS region
provider-field-project-id = Project ID
provider-field-location = Location
provider-field-credential-file = Credential file (server)


## tui/src/app/exit_summary.rs

exit-summary-failed-archive-thread = Failed to archive thread { $thread_id }: { $error }
exit-summary-archive-not-confirmed = Archive of thread { $thread_id } was not confirmed.
exit-summary-use-stop-shortcut = use the configured stop shortcut
exit-summary-press-key = press { $key }
exit-summary-disconnected-running-continues = Disconnected from this task. Any running work continues.
exit-summary-disconnected-may-be-running = Disconnected from this task. Work may still be running.
exit-summary-disconnected-turn-stopped = Disconnected from this task. The current turn was stopped.
exit-summary-to-reconnect = To reconnect, run:
exit-summary-stop-current-turn = Stop the current turn: run { $command }, select this task, and { $hint }.
exit-summary-to-continue = To continue this session, run:
exit-summary-or-run-select = Or run { $command } and select { $name }.
exit-summary-session-id = Session ID: { $thread_id }

## tui/src/app/exit_summary.rs


## tui/src/app/side.rs

side-cannot-be-renamed = Side conversations cannot be renamed.
side-main-thread-unavailable = '/side' is unavailable until the main thread is ready.
side-no-started-conversation = '/side' is unavailable until the current conversation has started. Send a message first, then try /side again.
side-already-open = A side conversation is already open. Press ctrl + c to return before starting another.
side-main-needs-input = main needs input
side-parent-needs-input = parent needs input
side-main-needs-approval = main needs approval
side-parent-needs-approval = parent needs approval
side-main-failed = main failed
side-parent-failed = parent failed
side-main-interrupted = main interrupted
side-parent-interrupted = parent interrupted
side-main-closed = main closed
side-parent-closed = parent closed
side-main-finished = main finished
side-parent-finished = parent finished
side-from-main-thread = from main thread
side-from-parent-thread = from parent thread ({ $parent_label })
side-wait-permissions = Wait for permissions to update before forking.
side-failed-close-still-open = Failed to close side conversation { $thread_id }; it is still open: { $err }
side-failed-prepare = Failed to prepare side conversation { $child_thread_id }: { $err }
side-failed-switch-into-err = Failed to switch into side conversation { $child_thread_id }: { $err }
side-failed-switch-into = Failed to switch into side conversation { $child_thread_id }.
side-failed-start = Failed to start side conversation: { $err }

## tui/src/app/sidebar_files.rs

sidebar-files-requires-server-workspace = Requires workspace on server host
sidebar-files-preview-unavailable-windows = Preview unavailable on Windows

## tui/src/app/startup.rs

startup-no-startup-model = The initial app-server connection did not provide a startup model.
startup-permission-overrides-resuming = Permission overrides are not supported when resuming a remote task.
startup-permission-overrides-forking = Permission overrides are not supported when forking a remote task.
startup-invalid-keymap = Invalid `tui.keymap` configuration: { $err }{"\u000A"}Fix the config and retry.{"\u000A"}See the Codex keymap documentation for supported actions and examples.
startup-default-environment = Default
startup-could-not-restart-server = Could not restart the background server: { $err }
startup-reconnect-failed-stale = Reconnect failed — agent list is stale; relaunch to retry
startup-reconnect-timed-out = Automatic reconnect timed out waiting for the server to finish shutting down.

## tui/src/app/thread_settings.rs

thread-settings-failed-update = Failed to update thread settings: { $err }

## tui/src/app/user_verification_errors.rs

user-verification-could-not-complete-local = Could not complete user verification with the local Codex binary.
user-verification-could-not-verify-request = The local Codex binary could not verify this request.
user-verification-no-credential = No user-verification credential is available in the local Codex binary.
user-verification-biometric-unavailable = Biometric verification is currently unavailable on this device.
user-verification-unavailable-for-request = User verification is unavailable for this request.
user-verification-cancelled = User verification was cancelled.
user-verification-interrupted = User verification was interrupted.
user-verification-biometric-failed = Biometric verification did not succeed. The request was cancelled.
user-verification-timed-out = User verification timed out. The request was cancelled.
user-verification-could-not-complete = The local Codex binary could not complete user verification.
side-suffix-for-side = {" "}for side
side-suffix-to-switch = {" "}to switch
side-suffix-to-archive = {" "}to archive
side-suffix-to-close = {" "}to close
side-label-prefix = Side{" "}

## tui/src/bottom_pane/footer.rs

footer-plan-mode = Plan mode
footer-cycle-suffix = {" "}({ $key } to cycle)
footer-cycle-open = {" "}(
footer-cycle-close = {" "}to cycle)
footer-for-agents = {" "}for agents
footer-for-shortcuts = {" "}for shortcuts
footer-to-queue-message = {" "}to queue message
footer-to-queue = {" "}to queue
footer-goal-pursuing-usage = Pursuing goal ({ $usage })
footer-goal-pursuing = Pursuing goal
footer-goal-paused = Goal paused (/goal resume)
footer-goal-stalled = Goal stalled (/goal resume)
footer-goal-usage-limited = Goal hit usage limits (/goal resume)
footer-goal-unmet-usage = Goal unmet ({ $usage })
footer-goal-abandoned = Goal abandoned
footer-goal-achieved-usage = Goal achieved ({ $usage })
footer-goal-achieved = Goal achieved
footer-ide-context = IDE context
footer-reverse-i-search = reverse-i-search:{" "}
footer-again-to-quit = {" "}again to quit
footer-again-to-edit-previous = {" "}again to edit previous message
footer-to-edit-previous = {" "}to edit previous message
footer-percent-context-left = { $percent }% context left
footer-tokens-used = { $used_fmt } used
footer-full-context-left = 100% context left

## tui/src/auto_review_denials.rs

auto-review-send-input = send input to terminal { $process_id }: { $stdin }
auto-review-apply-patch-one = apply_patch touching { $file }
auto-review-apply-patch-many = apply_patch touching { $count } files
auto-review-network-access = network access to { $target }
auto-review-mcp-on = MCP { $tool_name } on { $label }
auto-review-permission-request-reason = permission request: { $reason }
auto-review-permission-request = permission request

## tui/src/chatwidget/realtime.rs

realtime-still-stopping = Voice conversation is still stopping.
realtime-unavailable-side = Voice mode is unavailable in side conversations. Return to the main thread first.
realtime-unsupported-platform = Voice requires macOS, an MSVC-based Windows build, or a glibc-based Linux build.
realtime-start-conversation-first = Start a conversation before using voice mode.
realtime-failed-start = Failed to start voice mode: { $message }
realtime-conversation-failed = Voice conversation failed: { $message }
realtime-failed-restore-mute = Failed to restore microphone mute: { $error }
realtime-connection-timed-out = Voice connection timed out. Retrying once after cleanup.
realtime-failed-connect = Failed to connect voice mode: { $error }
realtime-conversation-started = Voice conversation started.
realtime-started-hint = Use /voice mute to mute or /voice to stop.
realtime-failed-deliver = Failed to deliver the voice response.
realtime-closed-during-startup = Voice connection closed during startup. Retrying once.
realtime-conversation-ended = Voice conversation ended: { $reason }

## tui/src/history_cell/search.rs

history-search-in-url = '{ $pattern }' in { $url }
history-search-opened-page = Opened page
history-search-opened = Opened
history-search-opening-page = Opening page
history-search-opening = Opening
history-search-searched-for = Searched for
history-search-searched-page = Searched page
history-search-searching-for = Searching for
history-search-searching-page = Searching page
history-search-browsing = Browsing the web
history-search-searched-web = Searched the web
history-search-searching-web = Searching the web
history-search-for = {" "}for{" "}

## tui/src/session_start.rs

session-start-failed-unarchive = Failed to unarchive session { $thread_id }
session-start-failed-resume = Failed to resume session from { $target_label }: { $err }
session-start-failed-fork = Failed to fork session from { $target_label }: { $err }

## tui/src/tool_output.rs

tool-output-omitted-lines =
    { $count ->
        [one] +{ $omitted } line ({ $hint })
       *[other] +{ $omitted } lines ({ $hint })
    }

## tui/src/external_agent_config_migration/mod.rs

external-migration-import-selected = Import selected
external-migration-customize-selection = Customize selection
external-migration-cancel = Cancel
external-migration-review-selection = Review selection
external-migration-current-project = Current project:{" "}
external-migration-home = Home
external-migration-importing-none = Importing: none
external-migration-importing-summary = Importing: { $count_summary }
external-migration-plus-more = +{ $hidden_plugin_count } more
external-migration-plus-more-marketplaces = +{ $hidden_marketplace_count } more marketplaces
external-migration-plugin-counts = 
    ({ $marketplaces ->
        [one] { $marketplaces } marketplace
       *[other] { $marketplaces } marketplaces
    }, { $plugins ->
        [one] { $plugins } plugin
       *[other] { $plugins } plugins
    })

## tui/src/keymap_setup.rs

keymap-setup-configure-this-shortcut = Configure this shortcut.
keymap-setup-no-custom-override = No custom root override to remove
keymap-setup-custom-root-override = Custom root override
keymap-setup-default-keymap = Default keymap
keymap-setup-edit-shortcut = Edit Shortcut
keymap-setup-current = Current{" "}
keymap-setup-config = Config{" "}
keymap-setup-set-key = Set key
keymap-setup-capture-for-unbound = Capture a key for this unbound action
keymap-setup-replace-binding = Replace binding
keymap-setup-capture-replacement = Capture a replacement key for `{ $current_binding }`
keymap-setup-add-alternate-binding = Add alternate binding
keymap-setup-keep-and-add-key = Keep `{ $current_binding }` and add another key
keymap-setup-replace-one-binding = Replace one binding...
keymap-setup-choose-existing-to-replace = Choose which existing binding to replace
keymap-setup-replace-all-bindings = Replace all bindings
keymap-setup-replace-with-one-key = Replace `{ $current_binding }` with one key
keymap-setup-use-leader = Use leader...
keymap-setup-capture-after-leader = Capture the key that follows the configured leader
keymap-setup-leader-unbound = The leader is unbound
keymap-setup-configure-leader = Configure leader...
keymap-setup-set-leader-first = Set a leader before assigning this shortcut
keymap-setup-set-key-chord = Set key chord
keymap-setup-capture-two-keys = Capture two consecutive keys for this action
keymap-setup-replace-with-chord = Replace with key chord
keymap-setup-replace-with-two-stroke = Replace `{ $current_binding }` with a two-stroke key chord
keymap-setup-add-alternate-chord = Add alternate key chord
keymap-setup-keep-and-add-chord = Keep `{ $current_binding }` and add a two-stroke key chord
keymap-setup-remove-custom-binding = Remove custom binding
keymap-setup-restore-default-binding = Restore the default keymap binding
keymap-setup-back-to-shortcuts = Back to shortcuts
keymap-setup-return-to-list = Return to the shortcut list
keymap-setup-changes-write-root = Changes write the root{" "}
keymap-setup-override-suffix = {" "}override.
keymap-setup-replace-binding-title = Replace Binding
keymap-setup-choose-binding-to-replace = Choose the binding to replace.
keymap-setup-replace-with-another-key = Replace `{ $binding }` with another key
keymap-setup-binding-key-chord = { $binding } (key chord)
keymap-setup-binding-replace-two-stroke = Replace `{ $binding }` with a two-stroke key chord
keymap-setup-shortcut-conflict = Shortcut Conflict
keymap-setup-cannot-use = { $context }.{ $action } cannot use `{ $key }`.
keymap-setup-pick-another-key = Pick another key
keymap-setup-return-to-capture = Return to key capture for this action
keymap-setup-cancel = Cancel
keymap-setup-leave-unchanged = Leave keymap unchanged
keymap-setup-no-change = No change: `{ $context }.{ $action }` already uses `{ $key }`.
keymap-setup-no-longer-uses = `{ $context }.{ $action }` no longer uses `{ $old_key }`. Reopen /keymap and choose a binding again.
keymap-setup-remapped = Remapped `{ $context }.{ $action }` to `{ $key }`.
keymap-setup-added = Added `{ $key }` to `{ $context }.{ $action }`.
keymap-setup-replaced = Replaced `{ $old_key }` with `{ $key }` for `{ $context }.{ $action }`.
keymap-setup-unknown-action = Unknown keymap action `{ $context }.{ $action }`. Reopen /keymap and choose an action.
keymap-setup-only-modifiers = Only ctrl, alt, shift, and super modifiers can be stored in `tui.keymap`.
keymap-setup-only-function-keys = Only function keys F1 through F{ $max } can be stored in `tui.keymap`.
keymap-setup-only-ascii = Only printable ASCII keys can be stored in `tui.keymap`.
keymap-setup-key-unsupported = That key is not supported by `tui.keymap`.

## tui/src/worktree_startup.rs

worktree-startup-untrusted-source = `--worktree` cannot create a checkout from an explicitly untrusted source
worktree-startup-local-only = `--worktree` is only supported for local sessions
worktree-startup-session-not-found = Session not found: { $id_or_name }
worktree-startup-recovery = Startup did not finish binding a thread to this worktree: { $root }{"\u000A"}The checkout was kept. Inspect it and confirm no session is using it.{"\u000A"}To remove it, run `git worktree remove <checkout-path>` from the source repository,{"\u000A"}replacing <checkout-path> with the path above. Do not use --force.


## app/history_pagination.rs

history-pagination-no-previous-message = No previous message to edit.

## app/misalignment_policy.rs

misalignment-policy-continuation-request = Continuation request (quoted)
misalignment-policy-what-we-detected = What we detected
misalignment-policy-cannot-continue = Couldn’t continue this chat. Review its latest status before trying again.

## app/user_verification.rs

user-verification-unavailable-remote = User verification is unavailable for remote workspaces.

## app_server_connection.rs

app-server-connection-reopen-cloud = Reopen Codex to reconnect the cloud backend.

## app/thread_goal_actions.rs

thread-goal-actions-ephemeral = Goals need a saved session. This session is temporary.{"\u000A"}Run `codex` to start a saved session, or `codex resume` / `/resume` to reopen one.
thread-goal-actions-no-goal-set = No goal is currently set.
thread-goal-actions-goal-status = Goal { $status }
thread-goal-actions-cleared = Goal cleared
thread-goal-actions-no-goal-to-clear = No goal to clear
thread-goal-actions-no-goal-this-thread = This thread does not currently have a goal.
thread-goal-actions-replace-current = Replace current goal
thread-goal-actions-replace-desc = Set the new objective and start it now
thread-goal-actions-cancel = Cancel
thread-goal-actions-keep-current = Keep the current goal
thread-goal-actions-replace-title = Replace goal?
thread-goal-actions-new-objective = New objective: { $objective }
thread-goal-actions-create-first = Create a goal before editing it.
thread-goal-actions-failed-read = Failed to read thread goal: { $err }
thread-goal-actions-failed-replace = Failed to replace thread goal: { $err }
thread-goal-actions-failed-set = Failed to set thread goal: { $err }
thread-goal-actions-failed-update = Failed to update thread goal: { $err }
thread-goal-actions-failed-clear = Failed to clear thread goal: { $err }

## app/workspace_hud.rs

workspace-hud-panes = Panes
workspace-hud-move = Move
workspace-hud-this-pane = This pane
workspace-hud-command-center = Command Center
workspace-hud-main-composer = Main composer
workspace-hud-open-in-sidebar = Open in sidebar
workspace-hud-hide-sidebar = Hide sidebar
workspace-hud-sidebar-content = Sidebar content
workspace-hud-resize-panes = Resize panes
workspace-hud-minimum-width = Minimum width
workspace-hud-maximum-width = Maximum width
workspace-hud-equal-widths = Equal widths
workspace-hud-divider-left = Divider left
workspace-hud-divider-right = Divider right
workspace-hud-restore-layout = Restore layout
workspace-hud-zoom-pane = Zoom pane
workspace-hud-close-tab = Close tab
workspace-hud-previous-tab = Previous tab
workspace-hud-next-tab = Next tab
workspace-hud-hide-pane = Hide pane
workspace-hud-pane-left = Pane left
workspace-hud-pane-right = Pane right
workspace-hud-choose-environment = Choose environment
workspace-hud-presets = Presets ›
workspace-hud-find-text = Find text
workspace-hud-select-transcript = Select transcript
workspace-hud-mark-table-cell = Mark table cell
workspace-hud-composer = Composer
workspace-hud-sidebar = Sidebar
workspace-hud-conversation = Conversation
workspace-hud-leader-title = { $prefix } · { $origin }


## bottom_pane/mentions_v2/popup.rs

mentions-v2-popup-loading = loading...
mentions-v2-popup-no-matches = no matches


## chatwidget/interaction.rs

interaction-review-steer-unavailable = Steer messages aren't supported during /review. Press Ctrl+C now to cancel the review.
interaction-ctrl-l-disabled = Ctrl+L is disabled while a task is in progress.
interaction-no-agent-response = No agent response to copy
interaction-copy-failed = Copy failed: { $error }
interaction-rename-thread = Rename thread
interaction-name-thread = Name thread
interaction-type-a-name = Type a name and press Enter
interaction-thread-name-empty = Thread name cannot be empty.
interaction-generating-title = Generating a title suggestion…
interaction-suggested-from-conversation = Suggested from this conversation

## clipboard_copy/worker.rs

clipboard-worker-selection-ended = selection ended before clipboard delivery
clipboard-worker-nothing-to-copy = nothing to copy: the selected content is empty
clipboard-worker-stopped = clipboard worker stopped
clipboard-worker-no-read-pending = no clipboard read pending
clipboard-worker-read-timed-out = clipboard read timed out
clipboard-worker-start-failed = could not start clipboard worker: { $error }

## clipboard_paste/text.rs

clipboard-text-windows-reader-unavailable = Windows clipboard reader is unavailable
clipboard-text-reader-start-failed = could not start clipboard reader
clipboard-text-unavailable-over-ssh = clipboard text is unavailable over SSH
clipboard-text-windows-reader-busy = Windows clipboard reader is busy or unavailable
clipboard-text-read-timed-out = clipboard read timed out
clipboard-text-unavailable = clipboard text is unavailable
clipboard-text-too-large = clipboard text exceeds the message size limit
clipboard-text-windows-reader-start-failed = could not start Windows clipboard reader
clipboard-text-reader-no-input = clipboard reader has no input
clipboard-text-read-start-failed = could not start clipboard read
clipboard-text-reader-no-output = clipboard reader has no output
clipboard-text-read-failed = could not read clipboard text
clipboard-text-not-utf8 = clipboard text is not UTF-8

## config_panel/render.rs

config-render-animations = Animations
config-render-animations-desc = Animate the welcome screen, working indicator, and transitions
config-render-composer-stars = Composer stars
config-render-composer-stars-desc = Show stars briefly for new Astra tasks
config-render-effects = Effects
config-render-effects-subtitle = Configure visual effects
config-render-stars-need-animations = Turn on Animations to show Astra task stars
config-render-mouse-scroll-speed = Mouse scroll speed
config-render-mouse-scroll-speed-desc = Choose a mouse-wheel speed for the fullscreen transcript
config-render-speed-default = 1× (default)
config-render-unfocused = Unfocused
config-render-always = Always
config-render-notify-when = Notify when
config-render-notify-when-desc = Notify only while you are away or while this terminal is focused too
config-render-auto-default = Auto (default)
config-render-off = Off
config-render-on = On
config-render-copy-on-select = Copy on select
config-render-copy-on-select-desc = Copy transcript, footer, and composer selections on release; Auto follows terminal defaults
config-render-right-click-paste = Right-click paste
config-render-dense = Dense
config-render-comfortable = Comfortable
config-render-session-picker-layout = Session picker layout
config-render-session-picker-layout-desc = Choose tighter rows or a roomier list when resuming and forking conversations
config-render-ask = Ask
config-render-current-directory = Current directory
config-render-session-directory = Session directory
config-render-resume-directory = Resume directory
config-render-resume-directory-desc = Choose the working directory when resuming, with --cd taking precedence for this session
config-render-requires-cd-remote = Requires --cd for remote sessions
config-render-changes-save-automatically = Changes save automatically
config-render-configuration = Configuration
config-render-hint-toggle = toggle
config-render-hint-back = back
config-render-hint-group = group
config-render-hint-search = search
config-render-hint-open = open
config-render-hint-test = test
config-render-hint-clear-search-close = clear search/close
config-render-hint-clear-search = clear search
config-render-search-placeholder = Type to search settings

## history_cell/mcp.rs

history-mcp-auth-unknown = Unknown
history-mcp-auth-unsupported = Unsupported
history-mcp-auth-not-logged-in = Not logged in
history-mcp-auth-bearer-token = Bearer token
history-mcp-called = Called
history-mcp-calling = Calling
history-mcp-failed-verb = Failed
history-mcp-returned-image = Returned image
history-mcp-see-the = {" "}   See the{" "}
history-mcp-mcp-docs = MCP docs
history-mcp-to-configure-them = {" "}to configure them.
history-mcp-mcp-tools = MCP Tools
history-mcp-no-servers = {" "} • No MCP servers configured.
history-mcp-no-tools = {" "} • No MCP tools available.
history-mcp-status-connected = connected
history-mcp-status-starting = starting
history-mcp-status-auth-required = authentication required
history-mcp-status-failed = failed
history-mcp-status-not-started = not started
history-mcp-status-disabled = disabled
history-mcp-status-cancelled = cancelled
history-mcp-status-unknown = unknown
history-mcp-label-auth = {" "}   • Auth:{" "}
history-mcp-tools-none = {" "}   • Tools: (none)
history-mcp-label-tools = {" "}   • Tools:{" "}
history-mcp-resources-none = {" "}   • Resources: (none)
history-mcp-label-resources = {" "}   • Resources:{" "}
history-mcp-templates-none = {" "}   • Resource templates: (none)
history-mcp-label-templates = {" "}   • Resource templates:{" "}
history-mcp-use-verbose = {" "} Use /mcp verbose for tools and resources.
history-mcp-loading-inventory = Loading MCP inventory
history-mcp-loading-inventory-raw = Loading MCP inventory...
history-mcp-error = Error: { $error }
history-mcp-verb-name = { $verb } { $name }
history-mcp-name-with-environment = { $name } (environment { $environment_id })
history-mcp-environment-suffix = {" "}(environment { $environment_id })
history-mcp-tool-count =
    { $count ->
        [one] {" "}(1 tool)
       *[other] {" "}({ $count } tools)
    }

## thread_transcript/other_items.rs

other-items-patch-declined = Patch application declined
other-items-patch-in-progress = Patch application in progress
other-items-status-unavailable = status unavailable
other-items-context-compacted = Context compacted
other-items-hook-prompt = hook prompt:{" "}
other-items-image-generation = Image generation · { $status }
other-items-review-started = >> Code review started: { $review } <<
other-items-review-finished = << Code review finished: { $review } >>

## text_formatting.rs

text-formatting-and-two = { $first } and { $second }
text-formatting-and-last = { $list } and { $last }

## tooltips.rs

tooltips-tip-prefix = **Tip:** { $tip }

## session_resume.rs

session-resume-parent-hint = {"\u000A"}Resume its parent session instead: codex resume { $parent_id }
session-resume-guardian = Cannot resume a Guardian review thread.{ $parent_hint }
session-resume-cwd-unknown = failed to determine the working directory recorded for the selected session

## tui/src/app/provider.rs
provider-back = Back
provider-loading-title = Loading model providers…
provider-reconnect-to-browse = Reconnect to the app server to browse model providers.
provider-list-title = Model providers
provider-details-title = Model provider details
provider-edit-configuration = Edit configuration
provider-no-active-chat = No active chat
provider-server-default = Server default
provider-default-for-new-chats = Default for new chats
provider-client-launch-selects = This client's launch settings select { $id } for new chats.
provider-server-fixes-default = The server configuration fixes the default provider.
provider-use-new-to-start = Use /new to start a chat with the selected provider.
provider-list-subtitle = Current chat: { $current_label } · { $default_label }: { $default_id }
provider-hint-details = {" "}details
provider-hint-set-default = {" "}set default
provider-hint-back = {" "}back
provider-field-id = Provider ID
provider-field-type = Type
provider-type-builtin-openai = Built-in OpenAI provider
provider-absent-not-reported = Not reported
provider-absent-managed-externally = Managed externally
provider-absent-none = None
provider-absent-not-set = Not set

## tui/src/app/tui_mode_picker.rs
tui-mode-picker-saved = Saved TUI mode: { $mode }. Restart Codex to apply; launch overrides still apply.
tui-mode-picker-save-failed = Failed to save TUI mode: { $error }



## tui/src/bottom_pane/approval_overlay.rs
approval-overlay-send-input-existing = Would you like to send input to the existing terminal?
approval-overlay-send-input-terminal = Would you like to send input to terminal { $process_id }?
approval-overlay-run-command = Would you like to run the following command?
approval-overlay-network-access = Do you want to approve network access to "{ $host }"?
approval-overlay-grant-permissions = Would you like to grant these permissions?
approval-overlay-make-edits = Would you like to make the following edits?
approval-overlay-needs-approval = { $server } needs your approval.
approval-overlay-granted-none = You did not grant additional permissions
approval-overlay-granted-strict = You granted additional permissions with strict auto review
approval-overlay-granted-session = You granted additional permissions for this session
approval-overlay-granted = You granted additional permissions
approval-overlay-hint-confirm = to confirm
approval-overlay-hint-cancel = to cancel
approval-overlay-hint-or = {" "}or{" "}
approval-overlay-hint-press = Press{" "}
approval-overlay-hint-open-thread = {" "}to open thread
approval-overlay-label-thread = Thread:{" "}
approval-overlay-label-environment = Environment:{" "}
approval-overlay-label-permission-rule = Permission rule:{" "}
approval-overlay-label-reason = Reason:{" "}
approval-overlay-label-input = Input:{" "}
approval-overlay-label-server = Server:{" "}
approval-overlay-yes-once = Yes, just this once
approval-overlay-yes-proceed = Yes, proceed
approval-overlay-yes-prefix = Yes, and don't ask again for commands that start with `{ $rendered_prefix }`
approval-overlay-yes-host-conversation = Yes, and allow this host for this conversation
approval-overlay-yes-permissions-session = Yes, and allow these permissions for this session
approval-overlay-yes-command-session = Yes, and don't ask again for this command in this session
approval-overlay-yes-host-future = Yes, and allow this host in the future
approval-overlay-no-host-future = No, and block this host in the future
approval-overlay-no-continue-without-running = No, continue without running it
approval-overlay-no-tell-codex = No, and tell Codex what to do differently
approval-overlay-yes-files-session = Yes, and don't ask again for these files
approval-overlay-yes-grant-turn = Yes, grant these permissions for this turn
approval-overlay-yes-grant-strict = Yes, grant for this turn with strict auto review
approval-overlay-yes-grant-session = Yes, grant these permissions for this session
approval-overlay-no-without-permissions = No, continue without permissions
approval-overlay-yes-provide-info = Yes, provide the requested info
approval-overlay-no-continue-without-it = No, but continue without it
approval-overlay-cancel-request = Cancel this request
approval-overlay-rule-network = network
approval-overlay-rule-read = read { $reads }
approval-overlay-rule-write = write { $writes }
approval-overlay-rule-deny-read = deny read { $denied_reads }
approval-overlay-rule-glob = glob `{ $pattern }`

## tui/src/bottom_pane/chat_composer.rs
chat-composer-message-too-large = Message exceeds the maximum length of { $max } characters ({ $actual_chars } provided).
chat-composer-viewing-subagent = Viewing sub-agent — direct input is disabled
chat-composer-unrecognized-command = Unrecognized command '/{ $name }'. Type "/" for a list of supported commands.
chat-composer-message-too-long-limit = Message too long; limit { $max } characters
chat-composer-command-disabled-during-task = '/{ $command }' is disabled while a task is in progress.
chat-composer-shell-mode = Shell mode
chat-composer-capability-skills = skills
chat-composer-plugin-description = Plugin
chat-composer-plugin-description-capabilities = Plugin · { $capabilities }
chat-composer-shutting-down = Shutting down...
chat-composer-input-disabled = Input disabled.
chat-composer-capability-mcp-servers =
    { $count ->
        [one] 1 MCP server
       *[other] { $count } MCP servers
    }
chat-composer-capability-apps =
    { $count ->
        [one] 1 app
       *[other] { $count } apps
    }

## tui/src/bottom_pane/mcp_server_elicitation.rs
mcp-elicitation-answer-placeholder = Type your answer
mcp-elicitation-answer-placeholder-optional = Type your answer (optional)
mcp-elicitation-run-tool-continue = Run the tool and continue
mcp-elicitation-allow-request-continue = Allow this request and continue
mcp-elicitation-allow = Allow
mcp-elicitation-run-tool-session = Run the tool and remember this choice for this session
mcp-elicitation-allow-request-session = Allow this request and remember this choice for this session
mcp-elicitation-allow-session = Allow for this session
mcp-elicitation-run-tool-always = Run the tool and remember this choice for future tool calls
mcp-elicitation-allow-request-always = Allow this request and remember this choice for future requests
mcp-elicitation-always-allow = Always allow
mcp-elicitation-cancel = Cancel
mcp-elicitation-cancel-tool-call = Cancel this tool call
mcp-elicitation-deny = Deny
mcp-elicitation-decline-continue = Decline this request and continue
mcp-elicitation-cancel-request = Cancel this request
mcp-elicitation-true = True
mcp-elicitation-false = False
mcp-elicitation-server-message = Server: { $server }{"\u000A"}{"\u000A"}{ $message }
mcp-elicitation-tip-submit = to submit
mcp-elicitation-tip-submit-all = to submit all
mcp-elicitation-tip-submit-answer = to submit answer
mcp-elicitation-tip-navigate-fields = to navigate fields
mcp-elicitation-tip-change-field = change field
mcp-elicitation-tip-cancel = to cancel
mcp-elicitation-answer-required = Answer required fields before submitting.
mcp-elicitation-no-options = No options
mcp-elicitation-option-position = option { $selected }/{ $total }
mcp-elicitation-field-position = Field { $idx }/{ $total }
mcp-elicitation-field-position-unanswered = { $base } ({ $unanswered } required unanswered)
mcp-elicitation-no-fields = No fields


## tui/src/bottom_pane/app_link_view.rs
app-link-sign-in-instructions = Sign in to this app in your browser, then return here.
app-link-action-required = Action required
app-link-server = Server: { $server }
app-link-complete-action-instructions = Complete the requested action in your browser, then return here.
app-link-open-sign-in-url = Open sign-in URL
app-link-back = Back
app-link-already-signed-in = I already signed in
app-link-open-link = Open link
app-link-finished = I finished
app-link-manage-on-chatgpt = Manage on ChatGPT
app-link-disable-app = Disable app
app-link-enable-app = Enable app
app-link-install-on-chatgpt = Install on ChatGPT
app-link-already-installed = I already Installed it
app-link-use-dollar-insert = Use $ to insert this app into the prompt.
app-link-new-apps-delay = Newly installed apps can take a few minutes to appear in /apps.
app-link-after-installed-insert = After installed, use $ to insert this app into the prompt.
app-link-finish-app-sign-in = Finish App Sign In
app-link-finish-authentication = Finish Authentication
app-link-finish-in-browser = Finish in Browser
app-link-finish-app-setup = Finish App Setup
app-link-chatgpt-sign-in-browser = Sign in to the app on ChatGPT in the browser window that just opened.
app-link-complete-auth-browser = Complete authentication in the browser window that just opened.
app-link-return-select-signed-in = Then return here and select "I already signed in".
app-link-complete-action-browser = Complete the requested action in the browser window that just opened.
app-link-return-select-finished = Then return here and select "I finished".
app-link-complete-setup-browser = Complete app setup on ChatGPT in the browser window that just opened.
app-link-return-select-installed = Sign in there if needed, then return here and select "I already Installed it".
app-link-sign-in-url-label = Sign-in URL:
app-link-link-label = Link:
app-link-setup-url-label = Setup URL:
app-link-hint-move = {" "}move
app-link-hint-select = {" "}select
app-link-hint-close = {" "}close
app-link-no-actions = No actions

## tui/src/cwd_prompt.rs
cwd-prompt-verb-resume = resume
cwd-prompt-verb-fork = fork
cwd-prompt-past-resumed = resumed
cwd-prompt-past-forked = forked
cwd-prompt-save-failed = Failed to save working directory preference.
cwd-prompt-title = Working directory · { $action_verb }
cwd-prompt-explanation = Session = latest cwd recorded in the { $action_past } session{"\u000A"}Current = your current working directory
cwd-prompt-use-session-dir = Use session directory ({ $session_cwd })
cwd-prompt-use-current-dir = Use current directory ({ $current_cwd })
cwd-prompt-always-session-dir = Always use session directory
cwd-prompt-always-current-dir = Always use current directory
cwd-prompt-always-current-dir-path = Always use current directory ({ $path })
cwd-prompt-hint-continue = {" "}continue ·{" "}
cwd-prompt-hint-use-session = {" "}use session ·{" "}
cwd-prompt-hint-quit = {" "}quit

## tui/src/external_agent_config_migration/source.rs
external-agent-source-title = Choose an import source
external-agent-source-subtitle = Select the app whose setup you want to import.
external-agent-source-hint-continue = {" "}continue ·{" "}
external-agent-source-hint-cancel = {" "}cancel

## tui/src/daemon_startup.rs
daemon-startup-state-enabled = enabled
daemon-startup-state-disabled = disabled
daemon-startup-requires-feature = This session requires { $name } to be { $state }
daemon-startup-running-without-server = Running without the shared background server: { $reason }.

## tui/src/status/rate_limits.rs
rate-limits-bucket-limit = { $bucket } limit
rate-limits-bucket-window-limit = { $bucket } { $window } limit
rate-limits-window-limit = { $window } limit
rate-limits-monthly-credit-limit = Monthly credit limit
rate-limits-credits-used = { $used } of { $limit } credits used
rate-limits-percent-left = { $percent }% left
rate-limits-credits = Credits
rate-limits-unlimited = Unlimited
rate-limits-available = Available
rate-limits-credits-balance = { $balance } credits

## tui/src/transcript_view/follow_control.rs
follow-control-new-back-esc = {" "}New activity · ↓ Back to bottom · esc{" "}
follow-control-new-bottom = {" "}New activity · ↓ Bottom{" "}
follow-control-new-short = {" "}New · ↓ Bottom{" "}
follow-control-bottom = {" "}↓ Bottom{" "}
follow-control-arrow = {" "}↓{" "}
follow-control-back-esc = {" "}↓ Back to bottom · esc{" "}
follow-control-back = {" "}↓ Back to bottom{" "}
follow-control-bottom-esc = {" "}↓ Bottom · esc{" "}

## tui/src/transcript_view/disclosure.rs
disclosure-collapse = collapse
disclosure-expand = expand
disclosure-previous = previous
disclosure-next = next
disclosure-back = back


## tui/src/app/owned_transcript.rs
owned-transcript-starting-side-chat = Starting side chat…
owned-transcript-nav-enter-esc-latest = enter/esc latest
owned-transcript-nav-esc-latest = esc latest
owned-transcript-selecting-text = Selecting text · esc cancel

## Display constants shared between files
const-resize-hint = Resize · ←/h →/l · Shift+←/→ 10 cols · Enter/Esc done
const-external-agent-config-import-in-progress-message = A previous external agent import is still running. Wait for it to finish before importing again.
const-side-edit-previous-unavailable-message = Editing previous prompts is unavailable in side conversations.
const-trusted-access-for-cyber-verification-warning = Your conversations have multiple flags for possible cybersecurity risk. Responses may take longer because extra safety checks are on. To get authorized for security work, join the Trusted Access for Cyber program: https://chatgpt.com/cyber
const-tui-stub-message = Not available in TUI yet.
const-parent-owned-input-message = This sub-agent is controlled by its parent. Direct input is disabled.
const-ask-for-approval-label = Ask for approval
const-approve-for-me-label = Approve for me
const-auto-review-description = Only ask for actions detected as potentially unsafe
const-create-custom-provider = Create a custom provider or gateway
const-unconfirmed-create = Could not confirm the save. Check the provider list before adding it again.
const-setup-connection-lost = Codex lost its connection during setup. Restart Codex to continue.
const-goal-usage = Usage: /goal [<objective>|clear|edit|pause|resume]
const-compaction-header = Compacting context
const-compaction-details = Making room to continue.
const-unanswered-confirm-title = Submit with unanswered questions?
const-plan-implementation-title = Implement this plan?

## Permissions presets (codex-utils-approval-presets)
permissions-preset-read-only = Read Only
permissions-preset-default = Default
permissions-preset-full-access = Full Access

## Collaboration modes (codex-protocol's ModeKind)
collaboration-mode-default = Default
collaboration-mode-plan = Plan
