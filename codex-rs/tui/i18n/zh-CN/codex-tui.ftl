# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

## Slash commands and the composer

slash-config = 配置 Codex 偏好设置
slash-feedback = 将日志发送给维护者，或保存为 ZIP 文件
slash-new = 在对话过程中开始新对话
slash-init = 创建包含 Codex 指令的 AGENTS.md 文件
slash-compact = 汇总对话，以免达到上下文限制
slash-recap = 立即汇总当前对话
slash-review = 审查我当前的改动并找出问题
slash-rename = 重命名当前会话
slash-resume = 恢复已保存的对话
slash-archive = 归档此会话
slash-delete = 永久删除此会话
slash-clear = 清空终端并开始新对话
slash-fork = 分叉当前对话
slash-worktree = 在新的 worktree 中开始或继续对话
slash-quit = 退出 Codex
slash-copy = 复制最后一条回复或其中的一部分
slash-export = 将对话导出为 Markdown
slash-raw = 切换原始滚动缓冲模式，便于在终端中复制选中内容
slash-tui = 选择下次启动时使用的 TUI 模式
slash-diff = 显示 git diff（包括未跟踪的文件）
slash-mention = 提及某个文件
slash-skills = 使用 skill 来改进 Codex 执行特定任务的方式
slash-import = 从 Claude Code 导入设置、本项目和最近的对话
slash-hooks = 查看和管理生命周期 hook
slash-daemon = 管理本地后台服务
slash-warnings = 查看保留的警告和诊断详情
slash-status = 显示当前会话配置和 token 用量
slash-cd = 更改当前工作目录
slash-pwd = 显示当前工作目录
slash-usage = 查看账户用量或使用一次用量限额重置
slash-debug-config = 显示用于调试的配置层和要求来源
slash-title = 设置终端标题中显示的项目
slash-statusline = 设置状态栏中显示的项目
slash-theme = 选择语法高亮主题
slash-pets = 选择或隐藏终端宠物
slash-ps = 列出后台终端
slash-stop = 停止所有后台终端
slash-memory-drop = 请勿使用
slash-memory-update = 请勿使用
slash-model = 选择要使用的模型和推理强度
slash-presets = 运行或保存可复用的提示词和斜杠命令
slash-daybreak = 开启或关闭 Daybreak
slash-ide = 包含 IDE 中的当前选区、打开的文件和其他上下文
slash-plan = 切换到计划模式
slash-voice = 开始或停止语音；使用 /voice settings 选择语音
slash-goal = 设置或查看长时间运行任务的目标
slash-agents = 打开代理指挥中心
slash-multi-agents = 在本会话的子代理之间切换
slash-side = 开始一个侧边对话
slash-permissions = 选择 Codex 被允许执行的操作
slash-keymap = 重新映射 TUI 快捷键
slash-vim = 切换输入框的 Vim 模式
slash-elevate-sandbox = 设置提升权限的代理沙箱
slash-experimental = 开启或关闭实验性功能
slash-auto-review = 批准一次对最近一次自动审查拒绝的重试
slash-memories = 配置记忆的使用和生成
slash-mcp = 列出 MCP 工具；/mcp verbose 或 /mcp login <name> [--environment <id>]
slash-apps = 管理应用
slash-plugins = 浏览插件
slash-logout = 退出 Codex 登录
slash-rollout = 打印 rollout 文件路径
slash-test-approval = 测试审批请求
composer-placeholder = 让 Codex 做任何事
composer-side-placeholder = 继续追问

## bottom_pane/actionable_banner.rs
actionable-banner-press-number = 按数字选择
actionable-banner-number-esc-type = 按数字选择 · esc 关闭 · 输入以继续
actionable-banner-esc-type = esc 关闭 · 输入以继续

## bottom_pane/apply_patch_header.rs
apply-patch-header-thread = 会话:{" "}
apply-patch-header-default-description = 应用拟议的文件修改
apply-patch-header-description = 描述:{" "}
apply-patch-header-destination = 目标位置:{" "}
apply-patch-header-unavailable = 不可用

## bottom_pane/async_questions/mod.rs
async-questions-type-answer = 输入你的答案
async-questions-progress = { $current } / { $total }
async-questions-other-write-answer = 其他（填写答案）
async-questions-other = 其他

## bottom_pane/async_questions/render.rs
async-questions-no-options = 没有选项
async-questions-submit = 提交
async-questions-skip = 跳过
async-questions-prev-question = 上一题
async-questions-main-prompt = 主提示
async-questions-next-question = 下一题
async-questions-queued-messages = 排队中的消息
async-questions-option-position = 选项 { $current }/{ $total }

## bottom_pane/async_questions/state.rs
async-questions-expand-terminal = 放大终端以查看完整选项
async-questions-answer-too-long = 答案太长；发送前请缩短

## bottom_pane/chat_composer/footer_state.rs
footer-state-shortcuts = {" "}快捷键

## bottom_pane/chat_composer/passive_hints.rs
passive-hints-ide-context = IDE 上下文
passive-hints-to-queue-message = {" "}加入排队消息
passive-hints-to-queue = {" "}加入队列
passive-hints-for-agents = {" "}查看代理
passive-hints-for-shortcuts = {" "}查看快捷键

## bottom_pane/chat_composer/warning_notice.rs
warning-notice-count = { $count } 条警告
warning-notice-to-view = {" "}查看

## bottom_pane/chat_composer/history_search.rs
history-search-prompt = 反向搜索:{" "}
history-search-searching = {" "}{" "}搜索中
history-search-accept = {" "}接受
history-search-cancel = {" "}取消
history-search-no-match = {" "}{" "}无匹配

## bottom_pane/command_popup.rs
command-popup-unavailable-side = 在侧边会话中不可用
command-popup-no-matches = 没有匹配项

## bottom_pane/custom_prompt_view/picker.rs
custom-prompt-picker-normal-mode = {" "}普通模式
custom-prompt-picker-back = {" "}返回

## bottom_pane/file_search_popup.rs
file-search-popup-loading = {" "}{" "}加载中...
file-search-popup-no-matches = {" "}{" "}没有匹配项

## bottom_pane/experimental_features_view.rs
experimental-features-loading-server = 正在加载服务器实验…
experimental-features-title = 实验性功能
experimental-features-intro = 已勾选的功能即为开启。部分实验性功能仅在新任务中生效，或在重启 Codex 服务器后生效。
experimental-features-saving-hint = 正在保存…关闭此弹窗不会取消写入。
experimental-features-retained-hint = 已保留选择。保存以重试，或取消以关闭。
experimental-features-read-only = {" "}(只读)
experimental-features-saving = 正在保存实验性功能…
experimental-features-save-interrupted = 保存已中断。重新打开 /experimental 以检查已配置的值。
experimental-features-discovery-interrupted = 发现过程已中断
experimental-features-no-server = 没有可用的服务器实验。
experimental-features-server-unavailable = 服务器实验不可用。重新打开 /experimental 重试；若请求仍无回应，请重启此 Codex 客户端。
experimental-features-none-available = {" "}{" "}目前没有可用的实验性功能
experimental-features-toggle = {" "}切换
experimental-features-save = {" "}保存
experimental-features-cancel = {" "}取消

## bottom_pane/feedback_note_view.rs
feedback-note-scroll-disclosure = {" "}滚动查看说明
feedback-note-cancel = 取消
feedback-note-save-zip = 保存 ZIP
feedback-note-send-to-openai = 发送至 OpenAI
feedback-note-save = 保存
feedback-note-send = 发送
feedback-note-uploads-disabled = 已禁用反馈上传。
feedback-note-employee-notice = 发送给 OpenAI 的反馈可用于改进我们的模型和产品
feedback-note-employee-agree = 提交反馈即表示你同意 OpenAI 将反馈用于安全目的和内部模型训练，详情见{" "}
feedback-note-here = 此处
feedback-note-employee-avoid = 。请避免分享受 NDA 约束的机密信息，或享有特权、与人力资源相关的信息，以及敏感的个人信息。
feedback-note-external-notice = 发送给 OpenAI 的反馈可用于改进 ChatGPT。{" "}
feedback-note-learn-more = 了解更多
feedback-note-save-zip-also = 你也可以将反馈保存为 ZIP 文件，该文件不会发送给 OpenAI。
feedback-note-save-zip-only = 你可以将反馈保存为 ZIP 文件，该文件不会发送给 OpenAI。
feedback-note-title-bad-result = 告诉我们更多（结果不佳）
feedback-note-title-good-result = 告诉我们更多（结果良好）
feedback-note-title-bug = 告诉我们更多（缺陷）
feedback-note-title-safety-check = 告诉我们更多（安全检查）
feedback-note-title-other = 告诉我们更多（其他）
feedback-note-placeholder-default = （可选）简单描述一下，帮助我们进一步了解
feedback-note-placeholder-safety-check = （可选）说明被拒绝的内容，以及为何本应允许

## bottom_pane/feedback_view.rs
feedback-view-uploaded = • 反馈已上传。
feedback-view-recorded-no-logs = • 反馈已记录（不含日志）。
feedback-view-share-in-slack = { $prefix } 你可以在 #codex-feedback 中分享此内容：
feedback-view-open-issue = { $prefix } 请使用以下 URL 打开 issue：
feedback-view-thanks = { $prefix } 感谢你的反馈！
feedback-view-sentry-id = {" "}{" "}Sentry 反馈 ID:{" "}
feedback-view-sentry-url =   Sentry URL：
feedback-view-or-mention = {" "}{" "}或提供你的会话 ID{" "}
feedback-view-in-existing-issue = {" "}在已有 issue 中。
feedback-view-thread-id = {" "}{" "}会话 ID:{" "}
feedback-view-how-was-this = 这次体验如何？
feedback-view-cat-bug = bug
feedback-view-cat-bug-desc = 崩溃、错误信息、卡死，或界面/行为异常
feedback-view-cat-bad-result = 结果不佳
feedback-view-cat-bad-result-desc = 输出偏离目标、不正确、不完整或没有帮助
feedback-view-cat-good-result = 结果良好
feedback-view-cat-good-result-desc = 结果有帮助、正确、质量高，或令人惊喜，值得庆祝
feedback-view-cat-safety-check = 安全检查
feedback-view-cat-safety-check-desc = 正常使用因安全检查或拒绝而被阻止
feedback-view-cat-other = 其他
feedback-view-cat-other-desc = 速度慢、功能建议、体验反馈或其他
feedback-view-disclosure-zip = 保存 ZIP 不会向 OpenAI 提交反馈。{"\u000A"}文件可能包含提示词、命令和工具输出:
feedback-view-disclosure = 文件可能包含提示词、命令和工具输出:
feedback-view-include-logs = 是否包含日志和会话历史？
feedback-view-client-logs = client-logs.txt（如果可用）
feedback-view-file-if-available = { $filename }（如果可用）
feedback-view-connectivity = 连接诊断
feedback-view-yes = 是
feedback-view-yes-desc = 在报告中包含会话日志和诊断信息
feedback-view-no = 否
feedback-view-no-desc = 仅保留你的备注和报告元数据

## bottom_pane/hooks_browser_render.rs
hooks-browser-event-header = 事件                Active/Installed
hooks-browser-no-hooks = {" "}{" "}此事件没有安装任何 hook。

## bottom_pane/memories_settings_view.rs
memories-use = 使用记忆
memories-use-desc = 从下一个会话开始使用记忆
memories-generate = 生成记忆
memories-generate-desc = 从当前会话及之后的会话生成记忆
memories-reset-all = 重置所有记忆
memories-reset-all-desc = 清除本地记忆文件和摘要，不改动现有会话
memories-learn-more = 了解更多：{" "}
memories-title = 记忆
memories-intro = 选择 Codex 如何使用和生成记忆。更改会保存到 config.toml
memories-reset-title = 要重置所有记忆吗？
memories-reset-intro = 这将清除当前 Codex 主目录下的本地记忆文件和 rollout 摘要。
memories-go-back = 返回
memories-reset-delete-desc = 删除本地记忆文件和 rollout 摘要
memories-reset-back-desc = 返回记忆设置
memories-none-available = {" "}{" "}没有可用的记忆设置
memories-toggle = {" "}切换
memories-save-select = {" "}保存/选择
memories-cancel = {" "}取消

## bottom_pane/mentions_v2/footer.rs
mentions-footer-insert = {" "}插入 ·{" "}
mentions-footer-close = {" "}关闭 ·{" "}
mentions-footer-select = {" "}选择 ·{" "}
mentions-footer-filter = {" "}筛选

## bottom_pane/mentions_v2/candidate.rs
mention-candidate-plugin = 插件
mention-candidate-skill = skill
mention-candidate-task = 任务
mention-candidate-file = 文件
mention-candidate-dir = 目录

## bottom_pane/mentions_v2/render.rs
mentions-render-title = 提及
mentions-render-subtitle = 文件、目录、插件、skill 和任务
mentions-render-type-to-search = 输入以搜索提及项

## bottom_pane/mentions_v2/search_catalog.rs
mentions-catalog-plugin = 插件
mentions-catalog-plugin-with-capabilities = 插件 - { $capabilities }
mentions-catalog-skills = skill
mentions-catalog-mcp-servers = { $count } 个 MCP 服务器
mentions-catalog-apps = { $count } 个应用

## bottom_pane/mentions_v2/search_mode.rs
mentions-mode-all-results = 全部结果
mentions-mode-filesystem-only = 仅文件系统
mentions-mode-plugins = 插件

## bottom_pane/multi_select_picker.rs
multi-select-type-to-search = 输入以搜索
multi-select-no-matches = 没有匹配项
multi-select-toggle = {" "}切换
multi-select-reorder = {" "}重新排序
multi-select-save = {" "}保存
multi-select-cancel = {" "}取消

## bottom_pane/pending_input_preview.rs
pending-input-after-tool-call = 将在下一次工具调用后提交的消息
pending-input-press = {" "}(按{" "}
pending-input-to-interrupt = {" "}以中断并立即发送)
pending-input-end-of-turn = 将在本轮结束时提交的消息
pending-input-queued-follow-ups = 已排队的后续输入
pending-input-edit-last = {" "}编辑最后一条排队消息

## bottom_pane/popup_consts.rs
popup-press = 按{" "}
popup-to-confirm-or = {" "}确认或{" "}
popup-to-go-back = {" "}返回
popup-select = {" "}选择
popup-back = {" "}返回
popup-or = {" "}或{" "}

## bottom_pane/questions.rs
questions-count = { $count } 个问题
questions-to-answer = {" "}回答

## bottom_pane/request_user_input/render.rs
request-input-unanswered = { $count } 个问题未回答
request-input-no-choices = 没有选项
request-input-no-options = 没有选项
request-input-option-position = 选项 { $selected }/{ $total }

## bottom_pane/selection_row_layout.rs
selection-row-desc-disabled = { $desc } (已禁用: { $reason })
selection-row-disabled = 已禁用: { $reason }
selection-row-disabled-suffix = {" "}(已禁用)

## bottom_pane/shortcut_overlay.rs
shortcut-overlay-commands = 命令
shortcut-overlay-mention-files = 提及文件
shortcut-overlay-shell-command = Shell 命令
shortcut-overlay-new-line = 换行
shortcut-overlay-paste-image = 粘贴图片
shortcut-overlay-external-editor = 外部编辑器
shortcut-overlay-search-history = 搜索历史
shortcut-overlay-edit-last-message = 编辑上一条消息
shortcut-overlay-queue-message = 排队消息
shortcut-overlay-send-message = 发送消息
shortcut-overlay-change-mode = 切换模式
shortcut-overlay-less-reasoning = 降低推理强度
shortcut-overlay-more-reasoning = 提高推理强度
shortcut-overlay-voice = 语音
shortcut-overlay-agents = 代理（提示词为空时）
shortcut-overlay-subagents = 子代理（提示词为空时）
shortcut-overlay-inspect-activity = 查看活动
shortcut-overlay-interrupt = 中断
shortcut-overlay-quit = 退出
shortcut-overlay-open-transcript = 打开对话记录
shortcut-overlay-find-text = 查找文本
shortcut-overlay-scroll = 滚动
shortcut-overlay-start-selection = 开始选择
shortcut-overlay-top-latest = 顶部 / 最新
shortcut-overlay-group-compose = 输入框
shortcut-overlay-group-session = 会话
shortcut-overlay-group-transcript = 对话记录 (需先打开)
shortcut-overlay-keyboard-shortcuts = 键盘快捷键
shortcut-overlay-close = {" "}关闭
shortcut-overlay-customize = {" "}自定义
shortcut-overlay-resize = … 调整窗口大小以查看全部

## bottom_pane/shortcut_hud.rs
shortcut-hud-esc-cancel = Esc 取消
shortcut-hud-more = … 更多快捷键

## bottom_pane/list_selection_view.rs
list-selection-current = {" "}(当前)
list-selection-header-elided = {"["}… { $count } 行]
list-selection-view-all = {" "}查看全部
list-selection-search-label = 搜索 ›{" "}
list-selection-no-matches = 没有匹配项


## codex-rs/tui/src/bottom_pane/skill_popup.rs

skill-popup-no-matches = 没有匹配项
skill-popup-insert = {" "}插入 ·{" "}
skill-popup-close = {" "}关闭

## codex-rs/tui/src/bottom_pane/skills_toggle_view.rs

skills-toggle-view-search-placeholder = 输入以搜索 skill
skills-toggle-view-title = 启用/禁用 skill
skills-toggle-view-subtitle = 开启或关闭 skill。更改会自动保存。
skills-toggle-view-no-matches = 没有匹配项
skills-toggle-view-toggle = {" "}切换
skills-toggle-view-sep = {" "}·{" "}
skills-toggle-view-close = {" "}关闭

## codex-rs/tui/src/bottom_pane/status_line_setup.rs

status-line-item-model-name = 当前模型名称
status-line-item-model-with-reasoning = 当前模型名称及推理强度
status-line-item-reasoning = 当前推理强度
status-line-item-profile = 通过 --profile 选择的配置档案（未设置时省略）
status-line-item-provider = 当前会话的模型提供方
status-line-item-current-dir = 当前工作目录
status-line-item-project-root = 项目名称（不可用时省略）
status-line-item-hostname = 当前机器的主机名（不可用时省略）
status-line-item-git-branch = 当前 Git 分支（不可用时省略）
status-line-item-pull-request-number = 当前分支已打开的 PR 编号（不可用时省略）
status-line-item-branch-changes = 相对默认分支已提交的分支改动（不可用时省略）
status-line-item-status = 紧凑的会话运行状态文本（Ready, Working, Thinking）
status-line-item-permissions = 当前权限配置或沙箱模式
status-line-item-approval-mode = 当前命令审批模式
status-line-item-context-remaining = 上下文窗口剩余百分比（未知时省略）
status-line-item-context-used = 上下文窗口已用百分比（未知时省略）
status-line-item-five-hour-limit = 主用量限制的剩余用量（不可用时省略）
status-line-item-weekly-limit = 次用量限制的剩余用量（不可用时省略）
status-line-item-codex-version = Codex 应用版本
status-line-item-context-window-size = 上下文窗口总大小（token 数，未知时省略）
status-line-item-used-tokens = 本会话已用 token 总数（为零时省略）
status-line-item-total-input-tokens = 本会话已用输入 token 总数
status-line-item-total-output-tokens = 本会话已用输出 token 总数
status-line-item-thread-credits = 当前会话的预估积分（仅限企业版工作区；不可用时省略）
status-line-item-estimated-thread-cost = 当前会话的预估费用（美元，仅限企业版工作区；不可用时省略）
status-line-item-session-id = 当前会话标识符（会话开始前省略）
status-line-item-fast-mode = 当前模型实际生效的处理档位
status-line-item-daybreak = 此会话是否启用 Daybreak
status-line-item-raw-output = 是否启用原始滚动回看模式
status-line-item-thread-name = 当前会话名称（未命名时省略）
status-line-item-thread-title = 当前会话标题；未命名时显示会话标识符
status-line-item-workspace-headline = 工作区通知标题（仅限企业版工作区；不可用时省略）
status-line-item-task-progress = 来自 update_plan 的最新任务进度（可用前省略）
status-line-setup-choose-plugin = 选择状态栏插件
status-line-setup-use-theme-colors = 使用主题颜色
status-line-setup-use-theme-colors-desc = 应用当前 /theme 的颜色
status-line-setup-title = 配置状态栏
status-line-setup-subtitle = 选择要在状态栏中显示的项目。

## codex-rs/tui/src/bottom_pane/status_surface_preview.rs

status-surface-preview-sample-status = 运行中
status-surface-preview-sample-thread-name = 会话名称
status-surface-preview-sample-thread-title = 会话标题
status-surface-preview-sample-permissions = 工作区
status-surface-preview-sample-context-remaining = 上下文剩余 0%
status-surface-preview-sample-context-used = 上下文已用 0%
status-surface-preview-sample-five-hour-limit = 主要 0%
status-surface-preview-sample-weekly-limit = 次要 0%
status-surface-preview-sample-context-window-size = 0 窗口
status-surface-preview-sample-used-tokens = 已用 0
status-surface-preview-sample-total-input-tokens = 输入 0
status-surface-preview-sample-total-output-tokens = 输出 0
status-surface-preview-sample-thread-credits = 5.2 积分
status-surface-preview-sample-fast-mode = 快速
status-surface-preview-sample-daybreak = Daybreak 已关闭
status-surface-preview-sample-raw-output = 原始输出
status-surface-preview-sample-workspace-headline = 工作区标题
status-surface-preview-sample-profile = 配置档案：<name>
status-surface-preview-sample-provider = 提供方：<id>
status-surface-preview-sample-task-progress = 任务 0/0
status-surface-preview-limit-desc-secondary = 次用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-primary = 主用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-five-hour = 5 小时用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-daily = 每日用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-weekly = 每周用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-monthly = 每月用量限制的剩余用量（不可用时省略）
status-surface-preview-limit-desc-annual = 每年用量限制的剩余用量（不可用时省略）

## codex-rs/tui/src/bottom_pane/title_setup.rs

terminal-title-item-app-name = Codex 应用名称
terminal-title-item-project = 项目名称（回退为当前目录名）
terminal-title-item-current-dir = 当前工作目录
terminal-title-item-spinner = 工作时显示加载动画，受阻时显示需要处理的消息
terminal-title-item-status = 紧凑的会话运行状态文本（Ready, Working, Thinking）
terminal-title-item-thread-name = 当前会话名称（未命名时省略）
terminal-title-item-thread = 当前会话标题；未命名时显示会话标识符
terminal-title-item-git-branch = 当前 Git 分支（不可用时省略）
terminal-title-item-context-remaining = 上下文窗口剩余百分比（未知时省略）
terminal-title-item-context-used = 上下文窗口已用百分比（未知时省略）
terminal-title-item-five-hour-limit = 主用量限制的剩余用量（不可用时省略）
terminal-title-item-weekly-limit = 次用量限制的剩余用量（不可用时省略）
terminal-title-item-codex-version = Codex 应用版本
terminal-title-item-used-tokens = 本会话已用 token 总数（为零时省略）
terminal-title-item-total-input-tokens = 本会话已用输入 token 总数
terminal-title-item-total-output-tokens = 本会话已用输出 token 总数
terminal-title-item-thread-credits = 当前会话的预估积分（仅限企业版工作区；不可用时省略）
terminal-title-item-estimated-thread-cost = 当前会话的预估费用（仅限企业版工作区；不可用时省略）
terminal-title-item-session-id = 当前会话标识符（会话开始前省略）
terminal-title-item-fast-mode = 当前模型实际生效的处理档位
terminal-title-item-daybreak = 此会话是否启用 Daybreak
terminal-title-item-model = 当前模型名称
terminal-title-item-model-with-reasoning = 当前模型名称及推理强度
terminal-title-item-reasoning = 当前推理强度
terminal-title-item-task-progress = 来自 update_plan 的最新任务进度（可用前省略）
title-setup-title = 配置终端标题
title-setup-subtitle = 选择要在终端标题中显示的项目。

## codex-rs/tui/src/bottom_pane/unified_exec_footer.rs

unified-exec-footer-summary = { $count } 个后台终端运行中 · /ps 查看 · /stop 关闭

## codex-rs/tui/src/bottom_pane/user_verification.rs

user-verification-to-confirm = 确认
user-verification-to-cancel = 取消
user-verification-verify = 验证并批准
user-verification-cancel = 取消此请求
user-verification-thread = 会话：
user-verification-server = 服务器：
user-verification-waiting = 等待验证…
user-verification-cancel-hint = 取消此请求

## codex-rs/tui/src/bottom_pane/warnings_view_render.rs

warnings-view-title-entry = 警告 · 第 { $current } 条，共 { $total } 条 · { $source }
warnings-view-title = 警告
warnings-view-none = 没有警告
warnings-view-restart-hint = {"\u000A"}这些警告可能反映后台服务启动时的设置。如果你已经修复了它们，请使用 /daemon 重启它。
warnings-view-keep-next = 保留并继续
warnings-view-dismiss-close = 忽略并关闭
warnings-view-copy = 复制
warnings-view-warning = 警告
warnings-view-scroll = 滚动

## codex-rs/tui/src/app_server_approval_conversions.rs

approval-conversions-server-environment = { $server_name }（环境 { $environment_id }）

## codex-rs/tui/src/history_cell/approvals.rs

approvals-approved = 已批准
approvals-codex-to-run = {" "}codex 运行{" "}
approvals-this-time = （仅本次）
approvals-this-request = {" "}此请求
approvals-codex-network-access-to = {" "}codex 访问{" "}
approvals-codex-to-always-run-commands-that-st = {" "}codex 始终运行以如下内容开头的命令{" "}
approvals-every-time-this-session = （本会话内每次）
approvals-persisted = 已保存
approvals-codex-network-access-to-2 = {" "}Codex 访问许可：
approvals-denied = 已拒绝
approvals-and-saved-that-rule = {" "}并保存了该规则
approvals-did-not-approve = 未批准
approvals-request = 请求
approvals-for-codex-to-run = ：codex 运行{" "}
approvals-review = 审查
approvals-timed-out = 已超时
approvals-before-codex-could-run = ，codex 尚未能运行{" "}
approvals-before-this-request-could-be-approve = ，此请求尚未被批准
approvals-before-codex-could-access = ，codex 尚未能访问{" "}
approvals-canceled = 已取消
approvals-the-request-to-run = {" "}运行请求：
approvals-the-request-for-codex-network-access = {" "}codex 访问请求：
approvals-you = 你
approvals-auto-reviewer = 自动审查员
approvals-for-codex-to-apply = ：codex 应用
approvals-a-patch-touching = 补丁，涉及：
approvals-files = {" "}个文件
approvals-for = ：
approvals-before-codex-could-apply = ，codex 尚未能应用
approvals-before = ，涉及：

## codex-rs/tui/src/app/agent_status_feed.rs

agent-status-feed-heading = 正在运行的子代理
agent-status-feed-none = {" "} • 没有正在运行的子代理。
agent-status-feed-no-activity = 暂无最近活动。
agent-status-feed-updated-files = 已更新 { $count } 个文件
agent-status-feed-tool = 工具 { $tool }
agent-status-feed-spawned = 已启动一个代理
agent-status-feed-sent-input = 已向代理发送输入
agent-status-feed-resumed = 已恢复代理
agent-status-feed-waited = 已等待代理
agent-status-feed-closed = 已关闭代理
agent-status-feed-started = 已启动
agent-status-feed-contacted = 已联系
agent-status-feed-interrupted = 已中断
agent-status-feed-completed = 已完成
agent-status-feed-action-agent = { $action } { $agent_path }
agent-status-feed-web-search = 网页搜索：{ $query }
agent-status-feed-viewed = 已查看 { $path }
agent-status-feed-generated-image = 已生成图片
agent-status-feed-entered-review = 已进入审查模式
agent-status-feed-exited-review = 已退出审查模式
agent-status-feed-compacted = 已压缩上下文

## codex-rs/tui/src/app/statusline_picker.rs

statusline-picker-unavailable = {" "}（不可用）
statusline-picker-default = 默认状态栏
statusline-picker-offline = 断开连接时插件提供方不可用。
statusline-picker-discovering = 正在发现 MCP 状态栏提供方…
statusline-picker-title = 选择状态栏提供方
statusline-picker-save-failed = 无法保存状态栏选择：{ $error }

## codex-rs/tui/src/chatwidget/goal_status.rs

goal-status-budget-tokens = { $used } / { $budget } 个 token
goal-status-tokens = { $used } 个 token

## codex-rs/tui/src/chatwidget/status_controls.rs

status-controls-copy-model = 模型
status-controls-copy-directory = 目录
status-controls-copy-thread-name = 会话名称
status-controls-copy-session-id = 会话 ID
status-controls-limit-left = { $label } 剩余 { $remaining }%
status-controls-effort-default = 默认

## codex-rs/tui/src/chatwidget/status_state.rs

status-state-working = 运行中
status-state-reviewing-request = 正在审查审批请求
status-state-reviewing-prefix = 正在审查
status-state-reviewing-requests = 正在审查 { $count } 个审批请求
status-state-more = 另外 { $remaining } 个

## codex-rs/tui/src/chatwidget/status_surface_commands.rs

status-surface-commands-clear-title = 不能将 --clear 与终端标题项目一起使用。
status-surface-commands-clear-status = 不能将 --clear 与状态栏项目一起使用。
status-surface-commands-ambiguous = { $surface } 项目 '{ $token }' 有歧义。请使用 '{ $five_hour_id }' 或 '{ $weekly_id }'。
status-surface-commands-unknown = 未知的 { $surface } 项目 '{ $token }'。使用 { $command } 查看可用项目。

## codex-rs/tui/src/chatwidget/statusline_reader.rs
statusline-reader-initializing = 正在初始化…
statusline-reader-timed-out = 状态栏超时
statusline-reader-unavailable = 状态栏不可用
statusline-reader-failed = 状态栏出错

## codex-rs/tui/src/status/helpers.rs
status-helpers-reasoning = 推理 { $effort }
status-helpers-summaries-off = 摘要已关闭
status-helpers-summaries = 摘要 { $summary }
status-helpers-unknown = <未知>
status-helpers-none = <无>
status-helpers-time-on-date = { $date } { $time }

## codex-rs/tui/src/status/remote_connection.rs
remote-connection-cloud = 云端
remote-connection-unknown = 未知
remote-connection-invalid-websocket-url = <无效的 WebSocket URL>
remote-connection-service-older = 后台 Codex 服务正在运行 v{ $server }，比你的 Codex CLI v{ $client } 旧。
remote-connection-service-different = 后台 Codex 服务正在运行 v{ $server }，与你的 Codex CLI v{ $client } 不同。

## codex-rs/tui/src/status/thread_usage.rs
thread-usage-models-label = {"  "}模型
thread-usage-reasoning-label = {"  "}推理强度
thread-usage-speed-label = {"  "}速度
thread-usage-billed-tokens-label = {"  "}计费 token
thread-usage-label = 会话用量
thread-usage-other = 其他
thread-usage-reasoning-none = 无
thread-usage-reasoning-minimal = 最低
thread-usage-reasoning-light = 轻量
thread-usage-reasoning-medium = 中
thread-usage-reasoning-high = 高
thread-usage-reasoning-extra-high = 超高
thread-usage-reasoning-max = 最大
thread-usage-reasoning-ultra = 超强
thread-usage-speed-fast = 快速模式
thread-usage-speed-ultrafast = 超快
thread-usage-speed-standard = 标准
thread-usage-credits = { $credits } 积分
thread-usage-separator = {" "}·{" "}
thread-usage-tokens-input = 输入 { $tokens }
thread-usage-tokens-cached = （缓存 { $tokens }）
thread-usage-tokens-output = 输出 { $tokens }
thread-usage-group-share = { $name } { $percentage }%

## codex-rs/tui/src/status_indicator_widget.rs
status-indicator-elapsed-open = ({ $elapsed } •{" "}
status-indicator-to-interrupt = {" "}中断)
status-indicator-separator = {" "}·{" "}

## codex-rs/tui/src/terminal_program_status.rs
terminal-program-status-idle = 空闲
terminal-program-status-waiting = 等待中

## codex-rs/tui/src/chatwidget/status_surfaces.rs
status-surfaces-action-required-shown = {"["} ! ] 需要操作
status-surfaces-action-required-hidden = {"["} . ] 需要操作
status-surfaces-ignored-invalid-status-line = 已忽略无效的状态栏项：{ $items }。
status-surfaces-ignored-invalid-terminal-title = 已忽略无效的终端标题项：{ $items }。
status-surfaces-profile = 配置：{ $name }
status-surfaces-provider = 提供方：{ $provider }
status-surfaces-pull-request = PR #{ $number }
status-surfaces-no-changes = 没有改动
status-surfaces-tokens-used = 已用 { $tokens }
status-surfaces-context-left = 上下文剩余 { $percent }%
status-surfaces-context-used = 上下文已用 { $percent }%
status-surfaces-context-window = 窗口 { $tokens }
status-surfaces-tokens-in = 输入 { $tokens }
status-surfaces-tokens-out = 输出 { $tokens }
status-surfaces-credits = { $credits } 积分
status-surfaces-standard = 标准
status-surfaces-daybreak-on = Daybreak 已开启
status-surfaces-daybreak-off = Daybreak 已关闭
status-surfaces-raw-output = 原始输出
status-surfaces-tasks-progress = 任务 { $completed }/{ $total }
status-surfaces-permissions-read-only = 只读
status-surfaces-permissions-workspace = 工作区
status-surfaces-permissions-full-access = 完全访问
status-surfaces-permissions-custom = 自定义权限
status-surfaces-approval-auto-review = 自动审批
status-surfaces-approval-ask = 请求审批

## codex-rs/tui/src/onboarding/auth/headless_chatgpt_login.rs
headless-login-unexpected-response = account/login/start 返回了意外的响应：{ $response }
headless-login-finish-signing-in = 请在浏览器中完成登录
headless-login-preparing = 正在准备设备代码登录
headless-login-step-open-link = {"  "}1. 在浏览器中打开此链接并登录
headless-login-step-enter-code = {"  "}2. 登录后输入此一次性代码（15 分钟内有效）
headless-login-continue-only-if = {"  "}仅当你是在 Codex 中开始此次登录时才继续。如果此代码来自网站或其他人，请取消。
headless-login-requesting-code = {"  "}正在请求一次性代码…
headless-login-press = {"  "}按{" "}
headless-login-to-cancel = {" "}取消

## codex-rs/tui/src/onboarding/bedrock.rs
bedrock-set-up = 设置 Amazon Bedrock
bedrock-govcloud-title = 正在使用 AWS GovCloud
bedrock-govcloud-notice-before = 你正在使用 AWS GovCloud 运行 Codex。请确保你或你的管理员已阅读
bedrock-govcloud-guidance-link = 应用配置与安全指南
bedrock-govcloud-notice-after = ，然后再继续。
bedrock-acknowledge = > 确认
bedrock-checking-credentials = {"  "}正在检查现有的 AWS 凭据…
bedrock-setting-up = {"  "}正在设置 Amazon Bedrock…
bedrock-profile-detected = {"  "}已检测到 AWS profile：{ $name }
bedrock-region = {"  "}区域：{ $region }
bedrock-choose-profile = {"  "}请选择一个 AWS profile。
bedrock-env-credentials-detected = {"  "}服务器环境中检测到 AWS 凭据。
bedrock-no-credentials = {"  "}未找到 AWS 凭据。
bedrock-choose-auth-method = {"  "}请选择你的 AWS 认证方式。
bedrock-enter-profile-name = {"  "}请输入你的 AWS profile 名称。
bedrock-label-aws-profile = {"  "}AWS profile：
bedrock-enter-api-key = {"  "}请输入你的 Amazon Bedrock API key。
bedrock-label-api-key = {"  "}Bedrock API key：
bedrock-enter-region = {"  "}请输入要与 Amazon Bedrock 一起使用的 AWS 区域。
bedrock-label-region = {"  "}AWS 区域：{" "}
bedrock-enter-access-keys = {"  "}请输入你的 AWS 访问密钥。
bedrock-field-access-key-id = AWS 访问密钥 ID
bedrock-field-secret-access-key = AWS 秘密访问密钥
bedrock-field-session-token = AWS 会话 token（可选）
bedrock-field-prompt = { $marker } { $label }：
bedrock-export-credentials = {"  "}先导出 AWS 凭据，然后在该 shell 中运行 `codex --no-daemon`。
bedrock-setup-guide = {"  "}设置指南：{" "}
bedrock-press = {"  "}按{" "}
bedrock-to-continue = {" "}继续
bedrock-to-go-back = {" "}返回
bedrock-scroll-then = {" "}滚动 ·{" "}
bedrock-continue-with = 继续使用 { $name }
bedrock-use-existing-credentials = 使用已有的 AWS 凭据
bedrock-use-env-api-key = 使用服务器环境中的 Bedrock API key
bedrock-use-env-access-keys = 使用服务器环境中的 AWS 访问密钥
bedrock-continue-detected = 使用检测到的凭据继续
bedrock-choose-another-method = 选择其他登录方式
bedrock-other-methods = 其他 AWS 登录方式
bedrock-other-methods-description = 使用其他 profile、访问密钥、环境变量或 Bedrock API key
bedrock-method-profile = AWS profile
bedrock-method-profile-description = 使用 AWS SSO 或命名 profile
bedrock-method-access-keys = AWS 访问密钥
bedrock-method-access-keys-description = 输入访问密钥 ID 和 secret access key
bedrock-method-env = 环境变量
bedrock-method-env-description = 使用在 shell 中导出的 AWS 凭据
bedrock-method-api-key = Bedrock API key
bedrock-method-api-key-description = 输入 Bedrock API key
bedrock-unable-to-check = 无法检查 AWS 凭证：{ $error }
bedrock-unexpected-response = account/login/start 返回了意外的响应：{ $response }
bedrock-unable-to-set-up = 无法设置 Amazon Bedrock：{ $error }

## codex-rs/tui/src/onboarding/auth.rs
auth-api-key-disabled = 已禁用 API key 登录。
auth-sign-in-another-way-heading = {"  "}改用其他方式登录
auth-choose-how-to-use = {"  "}选择你希望如何使用 Codex。
auth-sign-in-paid-plan = 使用付费套餐中的 Codex，请用 ChatGPT 登录
auth-or-connect-api-key = 或连接 API key，按用量计费
auth-chatgpt-login-disabled = 已禁用 ChatGPT 登录
auth-usage-included-plans = Plus、Pro、Business 和 Enterprise 套餐已包含用量
auth-device-code-description = 使用一次性代码从其他设备登录
auth-sign-in-another-way = 改用其他方式登录
auth-use-api-key-or-provider = 使用 API key 或其他提供方
auth-use-openai-api-key-description = 使用 OpenAI API key
auth-sign-in-chatgpt = 使用 ChatGPT 登录
auth-sign-in-device-code-option = 使用设备代码登录
auth-use-openai-api-key = 使用 OpenAI API key
auth-pay-for-what-you-use = 按实际用量付费
auth-use-bedrock = 使用 Amazon Bedrock
auth-bedrock-description = 使用你的 AWS 凭据连接
auth-use-vertex = 使用 Google Cloud Vertex AI
auth-vertex-description = 使用你的 Google Cloud 凭据连接
auth-gateway-description = 选择已保存的配置，或新建配置
auth-api-key-disabled-by-workspace = {"  "}此工作区已禁用 API key 登录。请使用 ChatGPT 登录后继续。
auth-press = {"  "}按{" "}
auth-to-continue = {" "}继续
auth-to-go-back = {" "}返回
auth-finish-signing-in = 请在浏览器中完成登录
auth-link-does-not-open = {"  "}如果链接没有自动打开，请按{" "}
auth-to-copy-it = {" "}复制：
auth-remote-machine-press = {"  "}在远程或无头机器上？请按{" "}
auth-and-choose = {" "}然后选择{" "}
auth-sign-in-device-code = 使用设备代码登录
auth-to-cancel = {" "}取消
auth-learn-permissions = {"  "}了解权限与审批，请参阅{" "}
auth-codex-docs = Codex 文档
auth-plan-rate-limits = {"  "}你的套餐速率限制与
auth-training-data-preferences = 训练数据偏好设置
auth-apply = {" "}同样适用
auth-signed-in = ✓ 已使用你的 ChatGPT 账户登录
auth-in-control = {"  "}由你掌控
auth-can-make-mistakes = {"  "}Codex 可能会出错
auth-review-code = {"  "}请检查它编写的代码和运行的命令
auth-included-with-plan = {"  "}已包含在你的 ChatGPT 套餐中
auth-api-key-configured = ✓ 已配置 API key
auth-usage-based-billing = {"  "}Codex 将使用你的 API key，按用量计费。
auth-use-own-api-key = 使用你自己的 OpenAI API key，按用量计费
auth-paste-key-below = {"  "}请在下方粘贴或输入你的 API key。
auth-detected-env-key = {"  "}检测到 OPENAI_API_KEY 环境变量。
auth-paste-different-key = {"  "}如果你想使用其他账户，请粘贴另一个 key。
auth-paste-key-placeholder = 粘贴或输入你的 API key
auth-api-key-title = API key
auth-to-save = {" "}保存
auth-api-key-empty = API key 不能为空
auth-unexpected-response = account/login/start 返回了意外的响应：{ $response }
auth-failed-to-save-key = 保存 API key失败：{ $error }
auth-gateway-finish-signing-in = 请在浏览器中完成网关登录。
auth-gateway-starting = 正在开始网关登录…
auth-press-ctrl-c-quit = 按 Ctrl+C 退出。
auth-press-enter-retry = 按 Enter 重试，或按 Ctrl+C 退出。
auth-custom-provider-configured = ✓ 已配置自定义提供方
auth-vertex-configured = ✓ 已配置 Google Cloud Vertex AI
auth-bedrock-configured = ✓ 已配置 Amazon Bedrock


## onboarding/directory_trust.rs

directory-trust-folder-trust-changed = 文件夹信任状态已更改。请用其受限设置重新打开目标。

## onboarding/gateway.rs

gateway-default-provider-locked = 默认提供方由配置策略或启动时的覆盖设置决定。
gateway-loading = 正在加载网关…
gateway-saving = 正在保存网关设置…
gateway-hint-to-go-back = 返回
gateway-hint-to-create = 创建
gateway-hint-to-use-as-default = 设为默认

## onboarding/gateway_login.rs

gateway-login-selected-gateway-changed = 登录过程中所选网关已更改。

## onboarding/gcp.rs

gcp-title = > 设置 Google Cloud Vertex AI
gcp-guidance-project = 请输入要与 Vertex AI 一起使用的 Google Cloud 项目 ID。
gcp-label-project-id = {"  "}项目 ID：{" "}
gcp-guidance-location = 按 Enter 使用 global，或输入 Vertex AI 位置。
gcp-label-location = {"  "}位置（可选）：{" "}
gcp-guidance-credentials = 请输入运行 Codex 的主机上的绝对文件路径，或按 Enter 使用应用默认凭据（ADC）。
gcp-label-credential-file = {"  "}凭据文件（可选）：{" "}
gcp-validating = 正在验证凭据并保存配置…
gcp-adc-hint = {"  "}要用 Google Cloud CLI 设置 ADC，请运行
gcp-footer-set-up = {" "}设置
gcp-footer-continue = {" "}继续
gcp-footer-back = {" "}返回
gcp-error-project-required = 请输入 Google Cloud 项目 ID。

## onboarding/welcome.rs

welcome-welcome-to = 欢迎使用{" "}
welcome-coding-agent-tagline = ，OpenAI 的命令行编程代理

## onboarding/trust_directory.rs

trust-directory-existing-task-notice = 此现有任务可能保留设置和历史记录，包括在其受信任时加载的项目配置或 hook。要使用受限设置，请新建任务。该文件夹的信任设置不会更改。
trust-directory-restricted-notice = 来自不受信任文件夹的配置、hook 和 exec 策略保持禁用。受信任的项目文件夹仍可提供设置。skill 仍会加载，工具遵循你的权限设置。打开不会更改已保存的信任状态。
trust-directory-trust-prompt = 信任此文件夹吗？Codex 可以在此读取、编辑和运行文件，但受你的权限设置约束。文件夹设置可以自动运行代码，即使没有模型请求。仅当你信任这些文件时才继续。你的信任决定将被保存。
trust-directory-open-existing-task = 打开已有任务
trust-directory-open-restricted = 以受限模式打开
trust-directory-trust-and-continue = 信任并继续
trust-directory-quit = 退出
trust-directory-back-to-agents = 返回代理指挥中心
trust-directory-keep-current-dir = 保持当前目录
trust-directory-hint-continue-sandbox = {" "}继续并创建沙箱
trust-directory-hint-continue = {" "}继续
trust-directory-hint-quit = {" "}退出
trust-directory-hint-back = {" "}返回
trust-directory-hint-cancel = {" "}取消
trust-directory-git-subdir-note = 注意：你位于 Git 项目的子目录中。信任将应用于仓库根目录：
trust-directory-folder-access = {"  "}文件夹访问

## onboarding/onboarding_screen.rs

onboarding-screen-gateway-sign-in = {"  "}登录你的网关
onboarding-screen-gateway-step-open-link = {"  "}1. 在浏览器中打开此链接
onboarding-screen-gateway-step-enter-code = {"  "}2. 出现提示时输入此一次性代码
onboarding-screen-copy-copying = 正在复制链接…
onboarding-screen-copy-busy = 剪贴板正忙，请重试
onboarding-screen-copy-confirmed = 链接已复制到剪贴板
onboarding-screen-copy-unconfirmed = 已请求复制，请检查剪贴板
onboarding-screen-copy-failed = 无法复制链接：{ $error }
onboarding-screen-trust-failed = 无法设置 { $path } 的信任状态：{ $error }

## chatwidget/keymap_picker.rs

keymap-picker-invalid-config = `tui.keymap` 配置无效：{ $err }

## keymap/leader.rs

leader-single-prefix-key = `tui.keymap.global.leader` 无效：请使用单个前缀键。

## keymap/vim_search.rs

vim-search-conflicting-bindings = `{ $path }` 与 `{ $other }` 的绑定冲突

## keymap/chords.rs

chords-cannot-dispatch-inventory = 无法分发 `{ $path }`：键位动作清单超过了 { $max } 个内部 token。
chords-cannot-dispatch-unknown = 无法分发未知的键位动作 `{ $path }`。
chords-leader-repeat-reserved = `{ $path }` = `{ $spec }` 有歧义：重复前缀键保留用于取消。
chords-esc-reserved = `{ $path }` = `{ $spec }` 有歧义：单独的 `esc` 保留用于取消待完成的组合键。
chords-backspace-reserved = `{ $path }` = `{ $spec }` 无效：`backspace` 保留用于编辑任务输入。
chords-esc-cancel = Esc 取消

## keymap_setup/actions.rs

keymap-actions-change-where-to-run = 更改运行位置
keymap-actions-binding-inactive = { $spec }（未激活）
keymap-actions-source-custom = 自定义
keymap-actions-source-custom-global = 自定义（全局）
keymap-actions-source-default = 默认
keymap-actions-ctx-global = 全局
keymap-actions-ctx-chat = 聊天
keymap-actions-ctx-composer = 输入框
keymap-actions-ctx-editor = 编辑器
keymap-actions-ctx-vim-normal = Vim 普通模式
keymap-actions-ctx-vim-search = Vim 搜索
keymap-actions-ctx-vim-operator = Vim 操作符
keymap-actions-ctx-vim-text-object = Vim 文本对象
keymap-actions-ctx-pager = 分页器
keymap-actions-ctx-list = 列表
keymap-actions-ctx-agents = 代理
keymap-actions-ctx-approval = 审批
keymap-actions-global-change-host = 更改新会话的运行位置。
keymap-actions-global-focus-main-composer = 聚焦主会话输入框。
keymap-actions-global-focus-sidebar-tabs = 打开侧边栏标签页选择器。
keymap-actions-global-focus-sidebar-content = 聚焦侧边会话；若已聚焦则隐藏它。
keymap-actions-global-hide-pane = 隐藏已聚焦的侧边栏面板。
keymap-actions-global-close-sidebar-tab = 关闭当前侧边栏标签页。
keymap-actions-global-sidebar-up = 切换到上一个侧边栏标签页。
keymap-actions-global-sidebar-down = 切换到下一个侧边栏标签页。
keymap-actions-global-focus-pane-left = 聚焦父会话。
keymap-actions-global-focus-pane-right = 聚焦已打开的侧边会话。
keymap-actions-global-minimize-pane-width = 将已聚焦面板的宽度缩到最小。
keymap-actions-global-maximize-pane-width = 将已聚焦面板的宽度扩到最大。
keymap-actions-global-equalize-pane-widths = 均分可见面板的宽度。
keymap-actions-global-resize-panes = 进入键盘调整模式，调整可见的分隔线。
keymap-actions-global-move-divider-left = 侧边面板获得焦点时，将分隔线左移。
keymap-actions-global-move-divider-right = 侧边面板获得焦点时，将分隔线右移。
keymap-actions-global-toggle-pane-zoom = 放大或还原当前聚焦的面板。
keymap-actions-global-open-agents = 打开代理指挥中心。
keymap-actions-global-open-transcript = 打开对话记录浮层。
keymap-actions-global-find-transcript = 在完整对话记录中查找文本。
keymap-actions-global-focus-activity = 聚焦对话记录中的活动分组，以展开或收起其详情。
keymap-actions-global-open-external-editor = 在外部编辑器中打开当前草稿。
keymap-actions-global-copy = 将最近一次代理回复复制到剪贴板。
keymap-actions-global-clear-terminal = 清空终端界面。
keymap-actions-global-toggle-vim-mode = 开启或关闭 Vim 输入框模式。
keymap-actions-global-toggle-fast-mode = 开启或关闭 Fast 模式。
keymap-actions-global-toggle-raw-output = 切换原始回滚模式。
keymap-actions-global-toggle-side-conversation = 在侧边对话与其父会话之间切换。
keymap-actions-global-leader = 设置共享前导键快捷键所用的前缀。
keymap-actions-chat-interrupt-turn = 中断当前轮次。
keymap-actions-chat-decrease-reasoning-effort = 降低推理强度。
keymap-actions-chat-increase-reasoning-effort = 提高推理强度。
keymap-actions-chat-previous-permission-mode = 切换到上一个可用的权限模式。
keymap-actions-chat-next-permission-mode = 切换到下一个可用的权限模式。
keymap-actions-chat-edit-queued-message = 依次前进浏览问题，然后编辑最后一条排队中的消息。
keymap-actions-chat-prompt-stack-back = 依次后退浏览问题，回到输入框。
keymap-actions-chat-skip-question = 跳过当前聚焦的问题。
keymap-actions-chat-toggle-voice = 开始或结束语音对话。
keymap-actions-chat-open-presets = 运行或保存可复用的输入。
keymap-actions-chat-toggle-voice-mute = 开启或关闭语音麦克风静音。
keymap-actions-composer-submit = 提交输入框中的当前草稿。
keymap-actions-composer-queue = 任务运行时将草稿加入队列。
keymap-actions-composer-toggle-shortcuts = 显示或隐藏输入框快捷键浮层。
keymap-actions-composer-history-search-previous = 打开历史搜索，或移动到上一个匹配项。
keymap-actions-composer-history-search-next = 移动到下一个历史搜索匹配项。
keymap-actions-composer-undo = 撤销输入框中的上一次编辑。
keymap-actions-composer-redo = 重做输入框中的上一次撤销的编辑。
keymap-actions-editor-insert-newline = 在编辑器中插入换行。
keymap-actions-editor-move-left = 将光标左移。
keymap-actions-editor-move-right = 将光标右移。
keymap-actions-editor-move-up = 将光标上移。
keymap-actions-editor-move-down = 将光标下移。
keymap-actions-editor-move-word-left = 移动到上一个单词的开头。
keymap-actions-editor-move-word-right = 移动到下一个单词的末尾。
keymap-actions-editor-move-line-start = 移动到行首。
keymap-actions-editor-move-line-end = 移动到行尾。
keymap-actions-editor-delete-backward = 向左删除一个字素。
keymap-actions-editor-delete-forward = 向右删除一个字素。
keymap-actions-editor-delete-backward-word = 删除前一个单词。
keymap-actions-editor-delete-forward-word = 删除后一个单词。
keymap-actions-editor-kill-line-start = 从光标处删除到行首。
keymap-actions-editor-kill-whole-line = 删除当前行。
keymap-actions-editor-kill-line-end = 从光标处删除到行尾。
keymap-actions-editor-yank = 粘贴剪切缓冲区的内容。
keymap-actions-vim-normal-enter-insert = 在光标处进入插入模式。
keymap-actions-vim-normal-append-after-cursor = 在光标后进入插入模式。
keymap-actions-vim-normal-append-line-end = 在行尾进入插入模式。
keymap-actions-vim-normal-insert-line-start = 在第一个非空白字符处进入插入模式。
keymap-actions-vim-normal-open-line-below = 在下方新建一行并进入插入模式。
keymap-actions-vim-normal-open-line-above = 在上方新建一行并进入插入模式。
keymap-actions-vim-normal-enter-replace-mode = 进入替换模式，逐个覆盖字符。
keymap-actions-vim-normal-move-left = 在 Vim 普通模式下左移。
keymap-actions-vim-normal-move-right = 在 Vim 普通模式下右移。
keymap-actions-vim-normal-move-up = 在 Vim 普通模式下上移，或调出更早的历史。
keymap-actions-vim-normal-move-down = 在 Vim 普通模式下下移，或调出更新的历史。
keymap-actions-vim-normal-move-word-forward = 移动到下一个单词的开头。
keymap-actions-vim-normal-move-word-backward = 移动到上一个单词的开头。
keymap-actions-vim-normal-move-word-end = 移动到当前或下一个单词的末尾。
keymap-actions-vim-normal-move-line-start = 移动到行的开头。
keymap-actions-vim-normal-move-line-end = 移动到行尾。
keymap-actions-vim-normal-find-forward = 在当前行中查找下一个字符。
keymap-actions-vim-normal-find-backward = 在当前行中查找上一个字符。
keymap-actions-vim-normal-till-forward = 在当前行中停在下一个字符之前。
keymap-actions-vim-normal-till-backward = 在当前行中停在上一个字符之后。
keymap-actions-vim-normal-jump-top = 跳到缓冲区第一行。
keymap-actions-vim-normal-jump-bottom = 跳到缓冲区最后一行。
keymap-actions-vim-normal-delete-char = 删除光标下的字符。
keymap-actions-vim-normal-replace-char = 替换光标下的字符。
keymap-actions-vim-normal-repeat-last-change = 重复上一次完整的编辑。
keymap-actions-vim-normal-substitute-char = 删除光标下的字符并进入插入模式。
keymap-actions-vim-normal-delete-to-line-end = 从光标处删除到行尾。
keymap-actions-vim-normal-change-to-line-end = 从光标处修改到行尾并进入插入模式。
keymap-actions-vim-normal-yank-line = 复制整行。
keymap-actions-vim-normal-paste-after = 在光标后粘贴。
keymap-actions-vim-normal-start-delete-operator = 开始删除操作符，并等待一个移动或动作。
keymap-actions-vim-normal-start-yank-operator = 开始复制操作符，并等待一个移动或动作。
keymap-actions-vim-normal-start-change-operator = 开始修改操作符，并等待一个移动或文本对象。
keymap-actions-vim-normal-undo = 撤销上一次完整的编辑。
keymap-actions-vim-normal-redo = 重做上一次撤销的编辑。
keymap-actions-vim-normal-cancel-operator = 取消待执行的 Vim 操作符。
keymap-actions-vim-search-forward = 在当前缓冲区中向前搜索。
keymap-actions-vim-search-backward = 在当前缓冲区中向后搜索。
keymap-actions-vim-search-next = 重复上一次已确认的搜索。
keymap-actions-vim-search-previous = 沿相反方向重复搜索。
keymap-actions-vim-operator-delete-line = 重复删除操作符，删除整行。
keymap-actions-vim-operator-yank-line = 重复复制操作符，复制整行。
keymap-actions-vim-operator-motion-left = 操作符移动：左。
keymap-actions-vim-operator-motion-right = 操作符移动：右。
keymap-actions-vim-operator-motion-up = 操作符移动：上。
keymap-actions-vim-operator-motion-down = 操作符移动：下。
keymap-actions-vim-operator-motion-word-forward = 操作符移动：到下一个单词的开头。
keymap-actions-vim-operator-motion-word-backward = 操作符移动：到上一个单词的开头。
keymap-actions-vim-operator-motion-word-end = 操作符移动：到单词末尾。
keymap-actions-vim-operator-motion-line-start = 操作符移动到行首。
keymap-actions-vim-operator-motion-line-end = 操作符移动到行尾。
keymap-actions-vim-operator-motion-find-forward = 操作符移动到当前行的下一个字符。
keymap-actions-vim-operator-motion-find-backward = 操作符移动到当前行的上一个字符。
keymap-actions-vim-operator-motion-till-forward = 在当前行中停在下一个字符之前。
keymap-actions-vim-operator-motion-till-backward = 在当前行中停在上一个字符之后。
keymap-actions-vim-operator-motion-jump-top = 操作符移动到缓冲区第一行。
keymap-actions-vim-operator-motion-jump-bottom = 操作符移动到缓冲区最后一行。
keymap-actions-vim-operator-select-inner-text-object = 选择内部文本对象。
keymap-actions-vim-operator-select-around-text-object = 选择外部文本对象。
keymap-actions-vim-operator-cancel = 取消待处理的操作符。
keymap-actions-vim-text-object-word = 选中当前单词。
keymap-actions-vim-text-object-big-word = 选中当前 WORD。
keymap-actions-vim-text-object-parentheses = 选中外围的圆括号。
keymap-actions-vim-text-object-brackets = 选中外围的方括号。
keymap-actions-vim-text-object-braces = 选中外围的花括号。
keymap-actions-vim-text-object-double-quote = 选中外围的双引号。
keymap-actions-vim-text-object-single-quote = 选中外围的单引号。
keymap-actions-vim-text-object-backtick = 选中外围的反引号。
keymap-actions-vim-text-object-cancel = 取消待处理的文本对象。
keymap-actions-pager-scroll-up = 向上滚动一行。
keymap-actions-pager-scroll-down = 向下滚动一行。
keymap-actions-pager-page-up = 向上翻一页。
keymap-actions-pager-page-down = 向下翻一页。
keymap-actions-pager-half-page-up = 向上滚动半页。
keymap-actions-pager-half-page-down = 向下滚动半页。
keymap-actions-pager-jump-top = 跳到开头。
keymap-actions-pager-jump-bottom = 跳到末尾。
keymap-actions-pager-close = 关闭分页器浮层。
keymap-actions-pager-close-transcript = 关闭对话记录浮层。
keymap-actions-pager-find = 在对话记录分页器中查找文本。
keymap-actions-pager-mark-markdown = 在对话记录选择模式下标记表格单元格。
keymap-actions-list-move-up = 列表选择上移。
keymap-actions-list-move-down = 列表选择下移。
keymap-actions-list-move-left = 在列表选择器中向左移动。
keymap-actions-list-move-right = 在列表选择器中向右移动。
keymap-actions-list-page-up = 列表选择上移一页。
keymap-actions-list-page-down = 列表选择下移一页。
keymap-actions-list-jump-top = 跳到列表第一项。
keymap-actions-list-jump-bottom = 跳到列表最后一项。
keymap-actions-list-accept = 确认当前列表选择。
keymap-actions-list-cancel = 取消并关闭选择视图。
keymap-actions-agents-resume = 打开会话恢复选择器。
keymap-actions-agents-search = 搜索可用的代理任务。
keymap-actions-agents-new-task = 在所选检出位置打开新会话。
keymap-actions-agents-new-worktree = 从项目默认分支在 worktree 中打开新会话。
keymap-actions-agents-fork = 分叉所选对话并打开新会话。
keymap-actions-agents-rename = 重命名所选任务。
keymap-actions-agents-stop = 停止所选正在运行的任务。
keymap-actions-agents-archive = 归档所选任务及其子代理。
keymap-actions-agents-delete = 永久删除所选任务及其子代理。
keymap-actions-agents-hide = 隐藏所选任务，直到明确恢复。
keymap-actions-agents-toggle-grouping = 按项目、目录、状态或模型循环切换任务分组。
keymap-actions-approval-open-fullscreen = 全屏打开审批详情。
keymap-actions-approval-open-thread = 可用时打开审批来源会话。
keymap-actions-approval-approve = 批准主要选项。
keymap-actions-approval-approve-for-session = 可用时批准本会话。
keymap-actions-approval-approve-for-prefix = 可用时使用 exec 策略前缀批准。
keymap-actions-approval-deny = 可用时选择明确的拒绝选项。
keymap-actions-approval-decline = 拒绝并提供纠正指导。
keymap-actions-approval-cancel = 取消一个 elicitation 请求。

## keymap/leader.rs

leader-needs-modifier = 无效的 `tui.keymap.global.leader` = `{ $label }`：请使用 ctrl、alt、super 或非字符键，以免拦截普通文本输入。在 Windows 上，Ctrl-Alt 字符可能是 AltGr 输入。
leader-ctrl-z-reserved = 无效的 `tui.keymap.global.leader` = `{ $label }`：在 Unix 上 ctrl-z 保留用于挂起终端。
leader-key-reserved = 无效的 `tui.keymap.global.leader` = `{ $label }`：此键已被 `{ $reserved_action }` 保留。

## keymap/chords.rs

chords-invalid-leader-binding = 无效的 `{ $path }` = `{ $raw }`。请使用 `leader` 加一个键，例如 `leader c`。
chords-invalid-chord-binding = 无效的 `{ $path }` = `{ $raw }`。请使用单个键（如 `ctrl-a`）或两键组合（如 `ctrl-x ctrl-t`）。
chords-too-many-leader-combinations = leader 快捷键组合过多（最多 { $max } 个）。请减少 `tui.keymap.global.leader` 的备选项或 `leader ...` 绑定。
chords-prefix-shadows = 有歧义的 `{ $path }` = `{ $spec }`：其前缀会遮蔽 `{ $other }`。请先解除绑定或重新映射现有快捷键，再将其用作组合键前缀。
chords-chord-already-assigned = 有歧义的 `{ $path }` = `{ $spec }`：相同的组合键已分配给 `{ $other }`。请选择唯一的组合键后重试。
chords-altgr-prefix = 无效的 `{ $path }` = `{ $spec }`：在 Windows 上，ctrl-alt 字符前缀可能是 AltGr 文本输入。请选择其他组合键后重试。
chords-plain-prefix = 无效的 `{ $path }` = `{ $spec }`：Vim 或指挥中心之外的组合键前缀必须使用 ctrl、alt、super 或非字符键，以免拦截普通文本输入。
chords-ctrl-z-reserved = 有歧义的 `{ $path }` = `{ $spec }`：在 Unix 上 `ctrl-z` 保留用于挂起终端。请选择其他组合键后重试。
chords-key-reserved = 有歧义的 `{ $path }` = `{ $spec }`：该组合键使用了 `{ $reserved_action }` 保留的键。请选择其他组合键后重试。

## keymap_setup/actions.rs

keymap-actions-binding-resolved = { $spec }（{ $resolved }）
keymap-actions-unbound = 未绑定

## codex-rs/tui/src/keymap_setup/capture.rs
capture-remap-shortcut = 重新映射快捷键
capture-action = 操作：
capture-current = 当前：
capture-press-new-key = 现在按下新的按键。按 Esc 取消。
capture-press-first-then-second = 先按第一个键，再按第二个键。按 Esc 取消。
capture-first-key-press-second = 第一个键：{ $first }。请按第二个键。按 Esc 取消。
capture-press-after-leader = 请按配置的前导键之后的按键。按 Esc 取消。
capture-error-prefix = 错误：

## codex-rs/tui/src/keymap_setup/debug.rs
debug-tip-inspect-keys = 提示：Codex 只能检查终端发送过来的按键。
debug-still-waiting = 仍在等待？如果按下按键后没有变化，说明终端没有把该按键发送给 Codex。只有能收到的按键才能设为快捷键。
debug-keypress-inspector = 按键检查器
debug-press-any-key = 按下任意键，查看 Codex 收到的内容。Esc 会被检查，Ctrl+C 关闭。
debug-waiting-keypress = 等待按键...
debug-detected = 检测到：
debug-config-key = 配置键：
debug-unsupported = 不支持 - { $error }
debug-raw-event = 原始事件：
debug-assigned-actions = 已分配的操作：
debug-none = {"  "}无

## codex-rs/tui/src/keymap_setup/picker.rs
picker-tab-app = 应用
picker-tab-app-desc = 全局和聊天级别的快捷键。
picker-tab-composer = 输入框
picker-tab-composer-desc = 输入框提交和队列的快捷键。
picker-tab-editor = 编辑器
picker-tab-editor-desc = 输入框内的光标移动和编辑快捷键。
picker-tab-vim = Vim
picker-tab-vim-desc = Vim 普通模式和操作符快捷键。
picker-tab-navigation = 导航
picker-tab-navigation-desc = 分页器和选择列表的导航快捷键。
picker-tab-agents = 代理
picker-tab-agents-desc = 共享代理仪表盘的快捷键。
picker-tab-approval = 审批
picker-tab-approval-desc = 审批提示的快捷键。
picker-tab-all = 全部
picker-all-desc = 所有可配置的快捷键。
picker-all-summary = 共 { $total } 个操作，{ $custom_count } 个已自定义，{ $unbound_count } 个未绑定。
picker-no-shortcuts = 没有可用的快捷键
picker-no-configurable-shortcuts = 没有可配置的快捷键
picker-tab-common = 常用
picker-common-desc = 经常自定义的快捷键。
picker-no-common = 没有常用快捷键
picker-no-common-actions = 没有可用的常用快捷键操作
picker-tab-customized = 已自定义（{ $custom_count }）
picker-customized-desc = 根级快捷键覆盖。
picker-no-customized = 没有自定义快捷键
picker-no-overrides = 尚未配置任何根级键位覆盖
picker-tab-unbound = 未绑定（{ $unbound_count }）
picker-unbound-desc = 没有生效快捷键的操作。
picker-no-unbound = 没有未绑定的快捷键
picker-all-bound = 目前每个可配置操作都有快捷键
picker-no-group-shortcuts = 此分组中没有快捷键
picker-no-group-actions = 此分组中没有可配置的操作
picker-search-shortcuts = 输入以搜索快捷键
picker-tab-debug = 调试
picker-debug-desc = 检查终端发出的按键。
picker-debug-summary = 查看 Codex 检测到的按键以及分配给它的快捷键。
picker-inspect-keypresses = 检查按键
picker-inspect-hint = 先按 Enter，再按任意键进行检查（Ctrl+C 退出）
picker-inspect-selected-hint = 打开实时检查器，显示检测到的按键、配置键和匹配的操作
picker-keymap-title = 键位
picker-one-action = 1 个操作。
picker-n-actions = { $count } 个操作。
picker-hint-group = {" "}分组 ·{" "}
picker-hint-edit = {" "}编辑快捷键 ·{" "}
picker-hint-custom = {" "}自定义 ·{" "}
picker-hint-unbound = {" "}未绑定 ·{" "}
picker-hint-close = {" "}关闭
picker-hint-start-inspector = {" "}开始检查 ·{" "}

## codex-rs/tui/src/chatwidget.rs
chatwidget-memories-enable-notice = 记忆将在下一次会话中启用。
chatwidget-feedback-unavailable-cloud = 云端会话不支持反馈。
chatwidget-feedback-uploads-disabled = 反馈上传已禁用，保存 ZIP 需要本地 app server。
chatwidget-feature-subagents = 子代理
chatwidget-feature-subagents-lower = 子代理
chatwidget-feature-memories = 记忆
chatwidget-feature-memories-lower = 记忆
chatwidget-enable-feature = 启用{ $name }？
chatwidget-feature-disabled = { $label }在本次 TUI 会话中已禁用。
chatwidget-learn-more = 了解更多：{" "}
chatwidget-yes-enable = 是，启用
chatwidget-enable-for-new-threads = 在服务器上为新会话保存此设置，不改变当前会话
chatwidget-not-now = 暂不
chatwidget-keep-disabled = 保持{ $name }禁用
chatwidget-code-review-started = >> 代码审查已开始：{ $hint } <<
chatwidget-code-review-finished = << 代码审查已完成 >>
chatwidget-stopping-bg-terminals = 正在停止所有后台终端。
chatwidget-raw-output-on = 原始输出模式已开启：对话记录以纯文本显示，便于在终端中选取。
chatwidget-raw-output-off = 原始输出模式已关闭：已恢复富文本对话记录渲染。
chatwidget-vim-enabled = Vim 模式已启用。
chatwidget-vim-disabled = Vim 模式已禁用。

## codex-rs/tui/src/chatwidget/backend_banners.rs
backend-banners-usage-limit-reached = 已达用量上限
backend-banners-usage-exhausted = 套餐内的用量已用完。请在下方选择一个选项以继续。
backend-banners-continue-luna-reserve = 继续使用 Luna Reserve
backend-banners-to-confirm = 确认
backend-banners-to-continue-working = 继续工作

## codex-rs/tui/src/chatwidget/clipboard.rs
clipboard-copy-failed = 复制失败: { $error }

## codex-rs/tui/src/chatwidget/command_lifecycle.rs
command-lifecycle-waiting-bg-terminal = 正在等待后台终端

## codex-rs/tui/src/chatwidget/compaction.rs
compaction-context-compacted = 上下文已压缩
compaction-context-compacted-elapsed = 上下文已压缩 · { $elapsed }

## codex-rs/tui/src/chatwidget/connectors.rs
connectors-apps-disabled = 应用已禁用。
connectors-enable-apps-feature = 启用 apps 功能以使用 $ 或 /apps。
connectors-no-apps = 没有可用的应用。
connectors-apps = 应用
connectors-loading-installed-available = 正在加载已安装和可用的应用...
connectors-loading-apps = 正在加载应用...
connectors-updates-when-ready = 完整列表准备好后将更新
connectors-failed-to-load = 加载应用失败。
connectors-directory-unavailable = 应用目录不可用
connectors-retry-directory-hint = 重试失败的应用目录请求，或按 Esc 继续
connectors-retry = 重试
connectors-reload-apps = 重新加载已安装和可用的应用
connectors-use-dollar = 使用 $ 将已安装的应用插入提示词。
connectors-installed-of-total = 已安装 { $installed }，共 { $total } 个可用应用。
connectors-status-open-manage = { $status_label } · 按 Enter 打开应用页面，以安装、管理或启用/禁用此应用
connectors-status-open-install = { $status_label } · 按 Enter 打开应用页面以安装此应用
connectors-status-link-unavailable = { $status_label } · 应用链接不可用
connectors-manage-in-browser = 请在浏览器中管理此应用。
connectors-install-in-browser = 请在浏览器中安装此应用，然后重新加载 Codex。
connectors-search-apps = 输入以搜索应用
connectors-status-description = { $status_label } · { $description }
connectors-status-installed = 已安装
connectors-status-installed-disabled = 已安装 · 已禁用
connectors-status-can-install = 可安装

## codex-rs/tui/src/chatwidget/copy_picker.rs
copy-picker-whole-status = 完整状态
copy-picker-whole-response = 完整回复
copy-picker-code-block = 代码块
copy-picker-language-code = { $language } 代码
copy-picker-blockquote = 引用块
copy-picker-copy-to-clipboard = 复制到剪贴板

## codex-rs/tui/src/chatwidget/goal_menu.rs
goal-menu-edit-goal = 编辑目标
goal-menu-type-objective = 输入目标描述后按 Enter
goal-menu-resume-paused-title = 恢复已暂停的目标？
goal-menu-goal-objective = 目标: { $objective }
goal-menu-resume-goal = 恢复目标
goal-menu-resume-goal-desc = 标记为活动状态，并在空闲时继续
goal-menu-leave-paused = 保持暂停
goal-menu-leave-paused-desc = 保持暂停；稍后使用 /goal resume
goal-menu-goal = 目标
goal-menu-status = 状态:{" "}
goal-menu-objective = 目标:{" "}
goal-menu-time-used = 已用时间:{" "}
goal-menu-tokens-used = 已用 Token:{" "}
goal-menu-token-budget = Token 预算:{" "}
goal-menu-commands-active = 命令: /goal edit，/goal pause，/goal clear
goal-menu-commands-paused = 命令: /goal edit，/goal resume，/goal clear
goal-menu-commands-done = 命令: /goal edit，/goal clear
goal-menu-status-active = 进行中
goal-menu-status-paused = 已暂停
goal-menu-status-stalled = 已停滞
goal-menu-status-usage-limited = 用量受限
goal-menu-status-budget-limited = 受预算限制
goal-menu-status-complete = 已完成

## codex-rs/tui/src/chatwidget/hooks.rs
hooks-failed-to-load = 加载 hooks 失败: { $err }

## codex-rs/tui/src/chatwidget/ide_context.rs
ide-context-off = IDE 上下文已关闭。
ide-context-usage = 用法: /ide [on|off|status]
ide-context-skipped = 本条消息已跳过 IDE 上下文。
ide-context-on = IDE 上下文已开启。
ide-context-future-messages = 后续消息将包含你当前的 IDE 选区和打开的标签页。
ide-context-connected = 已连接到你的 IDE。
ide-context-enable-failed = 无法开启 IDE 上下文。

## codex-rs/tui/src/chatwidget/image_submission.rs
image-submission-failed-images = 准备图片失败: { $error }
image-submission-too-large = 图片附件超过了 32 MiB 的传输上限
image-submission-failed-image = 准备图片失败: { $path }: { $error }

## codex-rs/tui/src/chatwidget/input_flow.rs
input-flow-preparing-images = 正在准备图片: { $text }
input-flow-cannot-switch-mode = 轮次运行期间无法切换协作模式。

## codex-rs/tui/src/chatwidget/input_restore.rs
input-restore-interrupted-for-steer = 模型已中断，以提交引导指令。
input-restore-goal-budget-reached = 已达到目标预算，本轮已停止。
input-restore-waiting-startup = 等待启动
input-restore-cancel = 取消

## codex-rs/tui/src/chatwidget/input_submission.rs
input-submission-model-unavailable = 会话模型不可用。请等待会话同步完成，或先选择一个模型再发送输入。
input-submission-shell-help-title = 在命令前加 ! 可在本地运行它
input-submission-shell-help-hint = 示例：!ls

## codex-rs/tui/src/chatwidget/luna_reserve_model.rs
luna-reserve-settings-unavailable = Luna 模型设置不可用；请稍后再试 /model。
luna-reserve-select-model = 选择模型
luna-reserve-other-models-return = 其他模型会在普通用量重新可用后恢复。

## codex-rs/tui/src/chatwidget/mcp_startup.rs
mcp-startup-multi-header-prefix = 正在启动 MCP 服务器
mcp-startup-single-header-prefix = 正在启动 MCP 服务器：
mcp-startup-interrupted = MCP 启动已中断。以下服务器未初始化：{ $servers }
mcp-startup-incomplete = MCP 启动未完成（失败：{ $servers }）
mcp-startup-server-error = { $server }: { $error }
mcp-startup-client-failed = MCP 客户端 `{ $server }` 启动失败

## codex-rs/tui/src/chatwidget/misalignment_policy.rs
misalignment-policy-title = 为谨慎起见，对话已停止
misalignment-policy-description = 我们无法确认代理是否在安全地工作并遵循你的指令。要继续工作，请开始或恢复另一个对话。
misalignment-policy-new-chat = 新对话
misalignment-policy-resume-chat = 恢复其他对话
misalignment-policy-review-findings = 查看审查结果
misalignment-policy-command-center = 代理指挥中心
misalignment-policy-paused-title = 出于预防，对话已暂停
misalignment-policy-paused-description = 我们无法确认代理是否正确理解了你的指令。继续前请先查看检测到的内容。
misalignment-policy-acknowledge = 确认检测结果并继续
misalignment-policy-back = 返回

## codex-rs/tui/src/chatwidget/model_command.rs
model-command-unexpected-argument = 意外参数 '{ $unexpected }'。请使用 '--reasoning=<level>' 或 '--speed=<tier>'。
model-command-reasoning-unavailable = 推理级别 '{ $effort }' 对 { $model } 不可用。可用项：{ $available }。
model-command-speed-unavailable = 速度 '{ $speed }' 对 { $model } 不可用。可用项：{ $available }。
model-command-session-starting = 会话仍在启动；请稍后再试 /model。
model-command-reserve-args-unavailable = 使用 Reserve 时无法设置模型参数。请使用 /model 调整推理强度。
model-command-model-unavailable = 模型 '{ $model_name }' 不可用。使用 /model 查看可用模型。

## codex-rs/tui/src/chatwidget/model_popup_state.rs
model-popup-state-list-stale = 模型列表可能已过时。请重启应用服务器和 TUI 以刷新 /model。
model-popup-state-saving-provider = 正在保存默认提供方。保存完成后再试 /model。
model-popup-state-load-failed = 无法加载模型。请再次尝试 /model。

## codex-rs/tui/src/chatwidget/model_popups.rs
model-popups-disabled-until-startup = 启动完成前无法选择模型。
model-popups-being-updated = 模型列表正在更新；请稍后再试 /model。
model-popups-loading-models = 正在加载模型…
model-popups-server-effort = 服务器推理强度
model-popups-accepted-future-turns = 已为后续轮次设置：{ $model } · { $effort }
model-popups-base-url-warning = 警告：OpenAI 基础 URL 已被覆盖为 { $base_url }。选择模型可能不受支持或无法正常工作。
model-popups-all-models = 所有模型
model-popups-choose-specific = 选择具体的模型和推理级别（当前：{ $current_label }）
model-popups-select-model = 选择模型
model-popups-pick-quick-mode = 选择快速自动模式，或浏览所有模型。
model-popups-no-additional = 目前没有其他可用模型。
model-popups-select-model-effort = 选择模型和推理强度
model-popups-no-reasoning = 无推理
model-popups-n-reasoning = { $label } 推理
model-popups-selected-reasoning = 所选推理
model-popups-always-use-in-plan = 计划模式下始终使用 { $reasoning_phrase }
model-popups-user-plan-override = 用户选定的计划覆盖设置（{ $label }）
model-popups-builtin-plan-default = 内置计划默认设置（{ $label }）
model-popups-builtin-plan-default-none = 内置计划默认设置（无推理）
model-popups-builtin-plan-default-plain = 内置计划默认设置
model-popups-all-modes-description = 设置全局默认推理级别和计划模式覆盖设置，替换当前的{ $plan_reasoning_source }
model-popups-choose-where-apply = 选择应用{ $reasoning_phrase }的位置。
model-popups-apply-plan-override = 应用到计划模式覆盖设置
model-popups-apply-global-and-plan = 应用到全局默认设置和计划模式覆盖设置
model-popups-apply-reasoning-change = 应用推理强度变更
model-popups-plus-rate-limit-warning = ⚠ { $effort_label } 推理强度会很快耗尽 Plus 套餐的速率限制
model-popups-default-suffix = {" "}（默认）
model-popups-and = 和
model-popups-more-reasoning = 更多推理…
model-popups-advanced-consumes = { $label }会更快消耗用量限额
model-popups-select-reasoning-level = 为 { $model_label } 选择推理级别
model-popups-max-description = 适用于质量比速度更重要的困难问题 · 用量较高
model-popups-ultra-description = 适用于使用多个代理的高要求工作 · 用量最高
model-popups-advanced-reasoning = 高级推理
model-popups-consumes-faster = ⚠ 更快耗尽用量限制
model-popups-effort-none = 无
model-popups-effort-minimal = 最低
model-popups-effort-low = 低
model-popups-effort-medium = 中
model-popups-effort-high = 高
model-popups-effort-xhigh = 超高
model-popups-effort-max = 最大
model-popups-effort-ultra = 超强
model-popups-effort-persistent = 持久
model-popups-ultra-concurrency-warning = Ultra 推理可能会主动使用多个代理。本会话配置了 { $max_threads } 个并发线程，最多 { $max_subagents } 个子代理，这可能导致用量快速增加。建议将 features.multi_agent_v2.max_concurrent_threads_per_session 设为 8 以下。

## codex-rs/tui/src/chatwidget/notifications.rs
notifications-agent-turn-complete = 代理轮次已完成
notifications-approval-requested = 请求审批：{ $subject }
notifications-wants-to-edit = Codex 想要编辑 { $target }
notifications-n-files = { $count } 个文件
notifications-approval-requested-by = { $server_name } 请求审批
notifications-plan-mode-prompt = 计划模式提示：{ $title }
notifications-question = 问题：{ $title }

## codex-rs/tui/src/chatwidget/permission_discovery.rs
permission-discovery-unknown-policy = 未知的审批策略 `{ $value }`。应为 `untrusted`、`on-request`、`never` 或细粒度策略。
permission-discovery-unknown-reviewer = 未知的权限审查者 `{ $value }`。应为 `user` 或 `auto_review`。
permission-discovery-usage = 用法：/permissions <profile-id> [--approval-policy=<policy>] [--reviewer=user|auto_review]
permission-discovery-unexpected-argument = 意外的权限参数 `{ $argument }`。
permission-discovery-update-title = 更新模型权限
permission-discovery-loading-profiles = 正在加载权限配置…
permission-discovery-retry = 重试

## codex-rs/tui/src/chatwidget/permission_popups.rs
permission-popups-non-admin-sandbox = { $label }（非管理员沙箱）
permission-popups-non-admin-sandbox-note = 非管理员沙箱在多数情况下可保护你的文件并阻止网络访问。但如果受到提示注入，风险更大。要升级到默认沙箱，请运行{" "}
permission-popups-update-title = 更新模型权限
permission-popups-no-denials = 本对话中最近没有自动审查拒绝记录。
permission-popups-denials-recorded = 自动审查拒绝某个操作后，会记录在这里。
permission-popups-thread-unavailable = 该对话已不可用。
permission-popups-action = 操作
permission-popups-rationale = 理由
permission-popups-no-rationale = 自动审查未提供理由
permission-popups-denials-title = 自动审查拒绝记录
permission-popups-select-denied = 选择一个被拒绝的操作以批准。
permission-popups-denial-unavailable = 该自动审查拒绝记录已不可用。
permission-popups-approval-recorded = 已记录对所选自动审查拒绝项的一次重试批准。
permission-popups-approval-context = 模型会看到批准上下文；重试仍会经过自动审查。
permission-popups-permissions-updated = 权限已更新为 { $label }
permission-popups-enable-full-access = 启用完全访问？
permission-popups-recommend-approve-for-me = 我们强烈建议改选 "为我批准"，并根据你的使用场景自定义审查策略。
permission-popups-recommend-ask = 我们强烈建议改选 "请求审批"。
permission-popups-full-access-warning = Codex 以完全访问权限运行时，可以在无需你批准的情况下编辑电脑上的任何文件，并运行带网络的命令。
permission-popups-cyber-risk = Cyber 模型的危险操作风险更高。
permission-popups-cyber-safeguards = {" "}授予完全访问权限前，请确保已设置适当的防护措施。{" "}
permission-popups-full-access-warning-sp = Codex 以完全访问权限运行时，可以在无需你批准的情况下编辑电脑上的任何文件，并运行带网络的命令。{" "}
permission-popups-exercise-caution = 开启完全访问时请保持谨慎。这会显著增加数据丢失、泄露或意外行为的风险。
permission-popups-yes-continue = 仍然继续
permission-popups-apply-full-access = 本次会话应用完全访问
permission-popups-cancel = 取消
permission-popups-go-back = 返回，不开启完全访问

## codex-rs/tui/src/chatwidget/permission_shortcuts.rs
permission-shortcuts-none-available = 没有其他可用的权限模式。

## codex-rs/tui/src/chatwidget/permissions_menu.rs
permissions-menu-read-only-desc = 读取工作区文件；编辑或访问互联网需要审批
permissions-menu-auto-desc = 读取和编辑工作区文件并运行命令；访问互联网或编辑工作区外的文件需要审批
permissions-menu-full-access-desc = 请谨慎使用：Codex 可以编辑此工作区之外的文件并访问互联网，且无需审批
permissions-menu-approve-for-me-unavailable = “为我批准”不可用。运行 `/permissions` 查看可用配置。
permissions-menu-unknown-profile = 未知的权限配置 `{ $profile_id }`。运行 `/permissions` 查看可用配置。
permissions-menu-profile-unavailable = 权限配置 `{ $profile_id }` 不可用：{ $reason }。
permissions-menu-configured-profile = 已配置的权限配置
permissions-menu-current-profile = 当前权限配置
permissions-menu-not-available-on-server = 此服务器不可用
permissions-menu-title = 更新模型权限
permissions-menu-no-profiles = 服务器未返回任何权限配置。

## codex-rs/tui/src/chatwidget/pets.rs
pets-unavailable = 宠物不可用
pets-close = 关闭
pets-failed-to-load-configured = 无法加载已配置的宠物：{ $err }
pets-loading-title = 正在加载宠物
pets-preparing = 正在准备终端宠物。
pets-loading-selected = 正在加载所选宠物...
pets-failed-to-start-runtime = 无法启动宠物加载运行时：{ $err }

## codex-rs/tui/src/chatwidget/plan_implementation.rs
plan-implementation-default-unavailable = 默认模式不可用
plan-implementation-no-approved-plan = 没有已批准的计划
plan-implementation-fresh-thread = 使用此计划开启新会话
plan-implementation-fresh-thread-context = 开启新会话（当前上下文：{ $label }）
plan-implementation-title = 实施此计划？
plan-implementation-yes = 是，实施此计划
plan-implementation-yes-desc = 切换到默认模式并开始编码
plan-implementation-clear-context = 是，清空上下文并实施
plan-implementation-no = 否，留在计划模式
plan-implementation-no-desc = 继续与模型规划

## codex-rs/tui/src/chatwidget/plugin_catalog.rs
plugin-catalog-openai-curated-loading-desc = OpenAI Curated 插件加载完成后将更新此处
plugin-catalog-remove-marketplace-question = 移除 { $name } 插件市场？
plugin-catalog-removing-name = 正在移除 { $name }...
plugin-catalog-upgrading-name-marketplace = 正在升级 { $name } 插件市场...
plugin-catalog-upgrading-marketplaces = 正在升级插件市场...
plugin-catalog-loading-details-for = 正在加载 { $name } 的详情...
plugin-catalog-installing-name = 正在安装 { $name }...
plugin-catalog-uninstalling-name = 正在卸载 { $name }...
plugin-catalog-add-marketplace = 添加插件市场
plugin-catalog-installed-of-available = 已安装 { $installed } / { $total } 个可用插件。
plugin-catalog-browse-plugins = 浏览可用插件市场中的插件。
plugin-catalog-installed-tab = 已安装（{ $installed }）
plugin-catalog-showing-installed = 显示 { $installed } 个已安装的插件。
plugin-catalog-installed-plugins = 已安装的插件。
plugin-catalog-installed-of-curated = 已安装 { $installed } / { $total } 个 OpenAI Curated 插件。
plugin-catalog-openai-curated-marketplace = OpenAI Curated 插件市场。
plugin-catalog-label-installed-successfully = { $label } 安装成功。
plugin-catalog-select-plugins-to-use = 选择要使用的插件，然后按 Enter 安装或查看详情。
plugin-catalog-label-period = { $label }。
plugin-catalog-installed-of-label = 已安装 { $installed } / { $total } 个 { $label } 插件。
plugin-catalog-hint-upgrade = { $upgrade } 升级
plugin-catalog-hint-remove = { $remove } 移除
plugin-catalog-hint-tab-toggle = Tab 切换
plugin-catalog-hint-tabs = ←/→ 切换标签页
plugin-catalog-hint-enter-details = Enter 查看详情
plugin-catalog-hint-esc-close = Esc 关闭
plugin-catalog-toggle-disable = 停用
plugin-catalog-toggle-enable = 启用
plugin-catalog-status-tab-to-toggle-enter = { $status }   Tab { $action }；Enter 查看详情
plugin-catalog-status-tab-to-toggle = { $status }   Tab { $action }
plugin-catalog-status-press-enter-details = { $status }   按 Enter 查看插件详情
plugin-catalog-status-details-unavailable = { $status }   插件详情不可用
plugin-catalog-status-press-enter-install = { $status }   按 Enter 安装或查看插件详情
plugin-catalog-status-remote-unavailable = { $status }   远程插件详情暂不可用
plugin-catalog-source-remote = 远程 · { $label }
plugin-catalog-version-local = 本地 { $version }
plugin-catalog-version-remote = 远程 { $version }
plugin-catalog-remote-id = 远程 ID { $id }
plugin-catalog-creator-name-account = 创建者 { $name }（{ $account_id }）
plugin-catalog-creator-name = 创建者 { $name }
plugin-catalog-creator-account = 创建者账户 { $account_id }
plugin-catalog-no-explicit-principals = 没有明确的授权对象
plugin-catalog-one-principal = 1 个授权对象：{ $name }
plugin-catalog-n-principals = { $count } 个授权对象
plugin-catalog-loading-label-plugins-dots = 正在加载 { $label } 插件...
plugin-catalog-label-unavailable = { $label } 不可用
plugin-catalog-loading-label-plugins = 正在加载 { $label } 插件。
plugin-catalog-local-functionality-available = 本地插件功能已可用。
plugin-catalog-section-loaded = 此部分已加载成功。
plugin-catalog-label-unavailable-period = { $label } 不可用。
plugin-catalog-local-functionality-still = 本地插件功能仍可用。
plugin-catalog-details-unavailable-reason = 插件详情不可用
plugin-catalog-openai-curated = OpenAI 精选
plugin-catalog-workspace = 工作区
plugin-catalog-shared-with-me = 与我共享
plugin-catalog-shared-with-me-link = 与我共享（链接）
plugin-catalog-local = 本地
plugin-catalog-this-updates-when-workspace-plugins-f = 工作区插件加载完成后将更新此处
plugin-catalog-no-workspace-plugins-available = 没有可用的工作区插件
plugin-catalog-no-workspace-directory-plugins-are-av = 没有可用的工作区目录插件
plugin-catalog-this-updates-when-shared-plugins-fini = 共享插件加载完成后将更新此处
plugin-catalog-no-shared-plugins-available = 没有可用的共享插件
plugin-catalog-no-plugins-have-been-shared-with = 没有人与你共享插件
plugin-catalog-plugins = 插件
plugin-catalog-loading-available-plugins = 正在加载可用插件...
plugin-catalog-this-updates-when-the-marketplace-lis = 插件市场列表就绪后将更新此处。
plugin-catalog-loading-plugins = 正在加载插件...
plugin-catalog-this-updates-when-the-marketplace-lis-2 = 插件市场列表就绪后将更新此处
plugin-catalog-adding-marketplace = 正在添加插件市场...
plugin-catalog-this-updates-when-marketplace-install = 插件市场安装完成后将更新此处
plugin-catalog-this-removes-the-configured-marketpla = 这将从 Codex 中移除已配置的插件市场。
plugin-catalog-select = {" "}选择
plugin-catalog-close = {" "}关闭
plugin-catalog-remove-marketplace = 移除插件市场
plugin-catalog-remove-this-marketplace-from-the-avai = 从可用插件列表中移除此插件市场
plugin-catalog-back-to-plugins = 返回插件
plugin-catalog-keep-this-marketplace-installed = 保留此插件市场的安装
plugin-catalog-removing-marketplace = 正在移除插件市场...
plugin-catalog-this-updates-when-marketplace-removal = 插件市场移除完成后将更新此处
plugin-catalog-this-updates-when-marketplace-upgrade = 插件市场升级完成后将更新此处
plugin-catalog-loading-plugin-details = 正在加载插件详情...
plugin-catalog-this-updates-when-plugin-details-load = 插件详情加载完成后将更新此处
plugin-catalog-installing-plugin = 正在安装插件...
plugin-catalog-this-updates-when-plugin-installation = 插件安装完成后将更新此处
plugin-catalog-uninstalling-plugin = 正在卸载插件...
plugin-catalog-this-updates-when-the-plugin-removal = 插件移除完成后将更新此处
plugin-catalog-failed-to-load-plugins = 加载插件失败。
plugin-catalog-plugin-marketplace-unavailable = 插件市场不可用
plugin-catalog-marketplace-add-failed = 添加插件市场失败
plugin-catalog-failed-to-add-marketplace-from-the = 无法从所提供的来源添加插件市场
plugin-catalog-try-again = 重试
plugin-catalog-enter-a-marketplace-source = 请输入插件市场来源
plugin-catalog-return-to-the-plugin-list = 返回插件列表
plugin-catalog-failed-to-add-marketplace = 添加插件市场失败。
plugin-catalog-marketplace-removal-failed = 移除插件市场失败
plugin-catalog-failed-to-remove-the-selected-marketp = 无法移除所选插件市场
plugin-catalog-review-the-confirmation-prompt-again = 请重新查看确认提示
plugin-catalog-failed-to-remove-marketplace = 移除插件市场失败。
plugin-catalog-plugin-detail-unavailable = 插件详情不可用
plugin-catalog-failed-to-load-plugin-details = 加载插件详情失败。
plugin-catalog-no-marketplace-plugins-available = 没有可用的插件市场插件
plugin-catalog-no-plugins-are-available-in-the = 已发现的插件市场中没有可用插件
plugin-catalog-all-plugins = 所有插件
plugin-catalog-no-installed-plugins = 没有已安装的插件
plugin-catalog-loading-openai-curated-plugins = 正在加载 OpenAI Curated 插件...
plugin-catalog-openai-curated-unavailable = OpenAI Curated 不可用
plugin-catalog-no-openai-curated-plugins-available = 没有可用的 OpenAI Curated 插件
plugin-catalog-no-plugins-available-in-this-marketpl = 此插件市场中没有可用插件
plugin-catalog-type-to-search-plugins = 输入以搜索插件
plugin-catalog-add-marketplace-2 = 添加插件市场
plugin-catalog-add-a-marketplace-from-a-git = 从 Git 仓库或本地根目录添加插件市场。
plugin-catalog-enter-a-source-to-make-its = 输入来源，使其插件可在此菜单中使用。
plugin-catalog-enter-owner-repo-a-git-url = 输入 owner/repo、Git URL 或本地插件市场路径
plugin-catalog-press-enter-to-enter-a-marketplace = 按 Enter 输入插件市场来源
plugin-catalog-data-shared-with-this-app-is = 与此应用共享的数据受该应用的
plugin-catalog-and = 和
plugin-catalog-terms-of-service = 服务条款
plugin-catalog-privacy-policy = 隐私政策
plugin-catalog-period-space = 。
plugin-catalog-learn-more = 了解更多
plugin-catalog-period = 。
plugin-catalog-installed-by-admin = 由管理员安装
plugin-catalog-this-plugin-is-installed-by-your = 此插件由工作区管理员安装
plugin-catalog-uninstall-plugin = 卸载插件
plugin-catalog-remove-this-plugin-now = 立即移除此插件
plugin-catalog-this-remote-plugin-did-not-provide = 此远程插件未提供卸载标识
plugin-catalog-install-plugin = 安装插件
plugin-catalog-this-plugin-is-disabled-by-your = 此插件已被工作区管理员禁用
plugin-catalog-this-plugin-is-not-installable-from = 此插件不能从该插件市场安装
plugin-catalog-install-this-plugin-now = 立即安装此插件
plugin-catalog-this-plugin-did-not-provide-an = 此插件未提供安装位置
plugin-catalog-skills = skill
plugin-catalog-hooks = hook
plugin-catalog-apps = 应用
plugin-catalog-mcp-servers = MCP 服务器
plugin-catalog-disabled-by-admin = 已被管理员禁用
plugin-catalog-enabled-by-admin = 管理员已启用
plugin-catalog-installed = 已安装
plugin-catalog-disabled = 已禁用
plugin-catalog-not-installable = 不可安装
plugin-catalog-can-be-installed = 可安装
plugin-catalog-source = 来源
plugin-catalog-auth = 认证
plugin-catalog-version = 版本
plugin-catalog-sharing = 共享
plugin-catalog-auth-on-install = 安装时认证
plugin-catalog-auth-on-use = 使用时认证
plugin-catalog-listed = 已列出
plugin-catalog-workspace-link = 工作区链接
plugin-catalog-private = 私有
plugin-catalog-admin-assigned = 管理员已分配
plugin-catalog-available = 可用
plugin-catalog-no-plugin-skills = 没有插件 skill
plugin-catalog-no-plugin-apps = 没有插件应用
plugin-catalog-no-plugin-hooks = 没有插件 hook
plugin-catalog-no-plugin-mcp-servers = 没有插件 MCP 服务器

## codex-rs/tui/src/chatwidget/plugins.rs
plugins-disabled = 插件已禁用。
plugins-disabled-hint = 请启用 plugins 功能以使用 /plugins。
plugins-add-marketplace = 添加插件市场
plugins-add-marketplace-placeholder = owner/repo、git URL 或本地插件市场路径
plugins-add-marketplace-examples = 示例：owner/repo、git URL、./marketplace
plugins-installed = 已安装 { $name } 插件。
plugins-no-auth-required = 无需额外的应用认证。
plugins-apps-need-auth = { $count } 个应用仍需认证：{ $app_names }
plugins-marketplace-already-added = 插件市场 { $name } 已添加。
plugins-marketplace-added = 已添加插件市场 { $name }。
plugins-marketplace-root = 插件市场根目录：{ $root }
plugins-marketplace-removed = 已移除插件市场 { $name }。
plugins-marketplace-config-removed = 已移除 { $name } 的插件市场配置。
plugins-no-git-marketplaces = 没有可升级的已配置 Git 插件市场。
plugins-only-git-upgradable = 只能升级已配置的 Git 插件市场。
plugins-marketplace-up-to-date = 插件市场 { $name } 已是最新。
plugins-marketplaces-all-up-to-date = 已检查 { $count } 个插件市场；均已是最新。
plugins-checked = 已检查：{ $names }
plugins-updated-roots = 已更新的根目录：{ $roots }
plugins-config-update-failed = 无法更新 { $plugin_id } 的插件配置：{ $err }
plugins-uninstalled = 已卸载 { $name } 插件。
plugins-bundled-apps-remain = 内置应用仍保持安装状态。
plugins-app-already-installed = 本会话中已安装。
plugins-install-required-apps = 请在 ChatGPT 中安装所需应用以继续：
plugins-heading = 插件
plugins-plugin-installed = 已安装 { $name } 个插件。
plugins-app-setup-progress = 应用设置 { $current }/{ $total }: { $name }
plugins-manage-on-chatgpt = 在 ChatGPT 中管理
plugins-install-on-chatgpt = 在 ChatGPT 中安装
plugins-open-app-management = 打开 ChatGPT 应用管理页面
plugins-open-app-page = 在浏览器中打开应用页面
plugins-apps-link-unavailable = ChatGPT 应用链接不可用
plugins-no-install-url = 该应用未提供安装或管理链接
plugins-continue = 继续
plugins-app-already-installed-desc = 该应用已安装
plugins-advance-next-app = 进入下一个应用
plugins-ive-installed-it = 我已安装
plugins-trust-confirmation = 信任你的确认，继续下一个应用
plugins-continue-without-refresh = 不等待刷新完成，直接继续
plugins-skip-app-setup = 跳过剩余的应用设置
plugins-stop-follow-up = 停止该插件的后续设置流程
plugins-abandon-app-setup = 放弃剩余的必需应用设置
plugins-skipped-app-setup = 已跳过 { $name } 插件剩余的应用设置。
plugins-may-not-be-usable = 在安装必需的应用之前，该插件可能无法使用。
plugins-completed-app-setup = 已为 { $name } 插件完成应用设置流程。
plugins-continue-managing = 现在可以继续在 /plugins 中管理插件。
plugins-upgraded = 已升级 { $count } 个插件市场。
plugins-upgrade-failed = 升级 { $count } 个插件市场失败：{ $errors }

## codex-rs/tui/src/chatwidget/protocol.rs

protocol-signed-in-to = 已登录 { $name }。
protocol-mcp-sign-in-failed = MCP 登录失败。
protocol-server-busy-reconnecting = 服务器繁忙，正在重新连接 { $attempt }
protocol-extra-safety-checks = 此请求需要额外的安全检查，部分工具调用可能耗时更长

## codex-rs/tui/src/chatwidget/protocol_requests.rs

protocol-requests-localize-permission-paths = 无法本地化请求的文件系统路径: { $err }
protocol-requests-localize-guardian-paths = 无法将 guardian 文件系统路径本地化：{ $err }

## codex-rs/tui/src/chatwidget/provider_error.rs

provider-error-continue-in-new-chat = {" "}在新对话中继续 ·{" "}
provider-error-start-fresh = {" "}重新开始 ·{" "}
provider-error-other-chats = {" "}其他对话
provider-error-chat-read-only = 此对话使用 { $provider }，但 Codex 现在使用 { $current }，因此此对话为只读。
provider-error-not-configured = 此对话使用的模型提供方“{ $provider }”已不再配置。请恢复该提供方的配置后继续。
provider-error-policy-conflict = 此对话使用模型提供方“{ $provider }”，但你的组织要求使用“{ $required }”。请联系管理员，或使用所需的提供方新建对话。

## codex-rs/tui/src/chatwidget/questions.rs

questions-question-requested = 已请求提问
questions-n-requested = 已请求 { $count } 个问题

## codex-rs/tui/src/chatwidget/quick_dials.rs

quick-dials-none-saved = 尚未保存预设。按 n 新建一个。
quick-dials-hud-new = 新建
quick-dials-hud-from-settings = 来自设置
quick-dials-hud-edit = 编辑
quick-dials-hud-delete = 删除
quick-dials-hud-close = 关闭
quick-dials-presets = 预设
quick-dials-images-not-copied = 图片无法复制到预设中，将从空文本开始。
quick-dials-mentions-not-copied = 关联的提及无法复制到预设中，将从空文本开始。
quick-dials-model-not-recallable = 无法通过 /model 调用当前模型。
quick-dials-choose-reasoning = 保存前，请先用 /model 选择一个可用的推理强度。
quick-dials-choose-speed = 保存前，请先用 /model 选择一个可用的速度。
quick-dials-choose-title-items = 保存前，请先用 /title 选择可用的终端标题项。
quick-dials-choose-status-items = 保存前，请先用 /statusline 选择可用的状态栏项。
quick-dials-unavailable = 此对话中无法使用预设。
quick-dials-after-startup = 启动完成后可使用预设。
quick-dials-preset-reason = 预设 { $slot }：{ $reason }
quick-dials-empty = 空
quick-dials-save-to-slot = 将预设保存到槽位
quick-dials-edit-a-preset = 编辑预设
quick-dials-delete-a-preset = 删除预设
quick-dials-new-preset = 新建预设
quick-dials-edit-preset-n = 编辑预设 { $slot }
quick-dials-review-changes = 审查更改
quick-dials-choose-slot = 选择槽位
quick-dials-write-prompt = 输入提示词或斜杠命令
quick-dials-approval-not-recallable = 无法调用审批策略。
quick-dials-profile-not-recallable = 无法调用权限配置名称。
quick-dials-select-profile = 请先用 /permissions 选择一个已命名的权限配置。
quick-dials-current-model = 当前模型
quick-dials-current-permissions = 当前权限
quick-dials-current-title = 当前终端标题
quick-dials-current-status = 当前状态栏
quick-dials-start-from-settings = 根据当前设置创建预设
quick-dials-replace-question = 替换预设 { $slot }？
quick-dials-replace-name = 替换预设 { $slot }
quick-dials-delete-question = 删除预设 { $slot }？
quick-dials-delete-name = 删除预设 { $slot }
quick-dials-save-question = 保存预设 { $slot }？
quick-dials-save-name = 保存预设 { $slot }

## codex-rs/tui/src/chatwidget/rate_limits.rs

rate-limits-heads-up = 注意，你的 { $limit_label } 限额剩余不足 { $remaining_percent }%。运行 /status 查看明细。
rate-limits-window-5h = 5 小时
rate-limits-window-daily = 每日
rate-limits-window-weekly = 每周
rate-limits-window-monthly = 每月
rate-limits-window-annual = 每年
rate-limits-fallback-secondary = 次要用量
rate-limits-fallback-primary = 用量
rate-limits-no-longer-close = 你已不再接近用量上限。
rate-limits-dismissing-in = 将在 { $seconds } 秒后关闭…
rate-limits-dismiss = 关闭
rate-limits-fewer-credits = 后续轮次消耗的额度更少
rate-limits-switch-to = 切换到 { $switch_model }
rate-limits-keep-model = 保持当前模型
rate-limits-keep-model-never = 保持当前模型（不再显示）
rate-limits-hide-reminders = 不再显示有关切换模型的速率限制提醒
rate-limits-approaching = 即将达到速率限制
rate-limits-switch-lower-usage = 切换到 { $switch_model } 以降低额度消耗？
rate-limits-workspace-credit-limit = 你已达到工作区额度上限
rate-limits-workspace-out-of-credits = 工作区额度已用完。请让工作区所有者增加额度。是否通知所有者？
rate-limits-usage-limit-reached = 已达用量上限
rate-limits-request-increase = 请向所有者申请提高限额，以继续使用 codex。是否申请提高？
rate-limits-yes = 是
rate-limits-no-default = 否（默认）
rate-limits-owner-notified = 已通知工作区所有者。
rate-limits-owner-already-notified = 近期已通知过工作区所有者。
rate-limits-owner-notify-failed = 无法通知工作区所有者，请重试。
rate-limits-increase-requested = 已申请提高限额。
rate-limits-increase-already-requested = 近期已申请过提高限额。
rate-limits-increase-request-failed = 无法申请提高限额，请重试。

## codex-rs/tui/src/chatwidget/read_only.rs

read-only-external-writer = 请在那里关闭它，然后按 R 在此继续。
read-only-archived = 按 U 取消归档此会话，并在此继续。
read-only-retry = 重试
read-only-unarchive = 取消归档

## codex-rs/tui/src/chatwidget/realtime/session_lock.rs

session-lock-voice-stopped = 会话已锁定，语音已停止。
session-lock-monitoring-failed = 会话锁定监测失败，语音对话已停止。

## codex-rs/tui/src/chatwidget/realtime/recording_controls.rs

recording-controls-voice-failed = 语音对话失败：{ $error }
recording-controls-mic-update-failed = 无法更新麦克风：{ $error }
recording-controls-start-voice-first = 请先开始语音模式，再静音麦克风。
recording-controls-connecting = 正在连接
recording-controls-muted = 已静音
recording-controls-heard = 已听到
recording-controls-speaking = 正在说话
recording-controls-listening = 正在聆听

## codex-rs/tui/src/chatwidget/realtime_settings.rs

realtime-settings-back = 返回
realtime-settings-voice-settings = 语音设置
realtime-settings-set-sound-devices = 设置音频设备
realtime-settings-choose-voice = 选择语音
realtime-settings-select-voice = 选择语音
realtime-settings-applies-next = 将应用于下一次语音对话。
realtime-settings-input-device = 输入设备
realtime-settings-output-device = 输出设备
realtime-settings-system-default = 系统默认
realtime-settings-sound-devices = 音频设备
realtime-settings-identical-names = 设备名称相同，请使用系统默认。
realtime-settings-input-channels = 输入声道
realtime-settings-all-channels-mixed = 所有声道（混合）
realtime-settings-inputs = 输入 { $channels }
realtime-settings-microphone = 麦克风：{ $name }
realtime-settings-select-channels = 选择要混合的输入声道，至少选择一个。
realtime-settings-input-n = 输入 { $number }
realtime-settings-select-one-channel = 请至少选择一个输入声道。
realtime-settings-voice-set = 语音已设为 { $voice }，将应用于下一次语音对话。

## codex-rs/tui/src/chatwidget/reasoning_shortcuts.rs

reasoning-shortcuts-lowest = 推理强度已是最低级别（{ $label }）。
reasoning-shortcuts-highest = 推理强度已是最高级别（{ $label }）。
reasoning-shortcuts-disabled-startup = 启动完成前，推理快捷操作已禁用。
reasoning-shortcuts-unavailable = { $model } 不支持推理快捷键。
reasoning-shortcuts-all-models = 所有模型 → { $model }
reasoning-shortcuts-ultra-available = Ultra 位于 /model → { $path } → 更多推理… 下

## codex-rs/tui/src/chatwidget/reconnect.rs

reconnect-reconnecting = 正在重新连接服务器…
reconnect-quit = 退出
reconnect-message-with-attachments = 带附件的消息
reconnect-unconfirmed = 无法确认“{ $preview }”是否已发送，尚未重新发送。
reconnect-failed-header = 重新连接失败，请检查端点，然后重新启动
reconnect-failed-body = 自动重新连接无法恢复此会话。草稿仍可编辑。请在用 Ctrl+C 退出前复制草稿，然后使用相同命令重新连接。

## codex-rs/tui/src/chatwidget/rendering.rs

rendering-open-in-another-app = 此对话已在另一个应用中打开
rendering-thread-archived = 此会话已归档
rendering-key-to-action = {" "}{ $action }
rendering-fork = 分叉
rendering-command-center = 指挥中心
rendering-exit = 退出
rendering-transcript = 对话记录
rendering-forking = 正在分叉对话…
rendering-continue-new-chat = 在新对话中继续
rendering-configuration-on = { $address } 上的配置
rendering-cloud = 云端
rendering-cycle = {" "}切换
rendering-change = {" "}更改

## codex-rs/tui/src/chatwidget/reset_credits.rs

reset-credits-expires = { $time } 于 { $date } 过期
reset-credits-expiration-unavailable = 无法获取过期时间
reset-credits-does-not-expire = 不会过期
reset-credits-full-reset = 完全重置
reset-credits-reset-limits = 重置当前用量限额

## codex-rs/tui/src/chatwidget/review_popups.rs

review-popups-against-base = 与基础分支比较审查
review-popups-pr-style = （PR 风格）
review-popups-uncommitted = 审查未提交的更改
review-popups-a-commit = 审查一个提交
review-popups-custom-instructions = 自定义审查说明
review-popups-select-preset = 选择审查预设
review-popups-detached-head = （分离的 HEAD）
review-popups-select-base-branch = 选择基础分支
review-popups-current-branch = 当前分支：{ $branch }
review-popups-search-branches = 输入以搜索分支
review-popups-select-commit = 选择要审查的提交
review-popups-search-commits = 输入以搜索提交
review-popups-type-instructions = 输入说明后按 Enter

## codex-rs/tui/src/chatwidget/safety_buffering.rs

safety-buffering-header = 正在为此请求多花一点思考
safety-buffering-message-with-retry = 如果不想等待，可以换用更快的模型重试。它处理复杂请求的能力可能较弱。
safety-buffering-footer = 无需操作。Codex 会继续等待，回复就绪后此菜单会关闭。
safety-buffering-retry-faster = 换用更快的模型重试
safety-buffering-dismiss-keep-waiting = 关闭并继续等待
safety-buffering-learn-more = 了解更多
safety-buffering-stop-retry-title = 停止此次尝试并重试？
safety-buffering-stop-retry-body = 这将停止当前尝试，并在新会话中重试。已做的文件更改或其他操作将保留。
safety-buffering-resend-with-model = 你的消息将使用 { $model } 重新发送，它在复杂任务上的能力可能较弱。
safety-buffering-keep-waiting = 继续等待
safety-buffering-stop-and-retry = 停止并重试

## codex-rs/tui/src/chatwidget/safety_recourse.rs

safety-recourse-cant-help = 我无法协助此请求。正在准备替代建议…
safety-recourse-enable-daybreak = 启用 Daybreak
safety-recourse-give-feedback = 提交反馈
safety-recourse-choose-rewrite = 你可以选择一种改写方式，带着之前的上下文开始新对话：

## codex-rs/tui/src/chatwidget/session_model_selection.rs

session-model-selection-apply = {" "}应用 ·{" "}
session-model-selection-default = {" "}默认 ·{" "}
session-model-selection-session = {" "}会话
session-model-selection-back = {" "}返回

## codex-rs/tui/src/chatwidget/settings_popups.rs

settings-popups-experimental-unavailable = 启动完成前无法使用实验性功能。

## codex-rs/tui/src/chatwidget/service_tiers.rs

service-tiers-open-cloud-first = 选择速度前，请先打开云端对话。

## codex-rs/tui/src/chatwidget/session_flow.rs

session-flow-daybreak-on = Daybreak 已对新轮次开启。
session-flow-thread-forked-from = 会话分叉自{" "}
session-flow-reverted = 对话已回退到此处。文件更改保持不变。

## codex-rs/tui/src/chatwidget/settings.rs

settings-daybreak-disable = 停用网络安全工作的更广泛访问权限
settings-daybreak-enable = 启用网络安全工作的更广泛访问权限
settings-daybreak-learn = 了解网络安全工作的更广泛访问权限
settings-daybreak-manage = 管理网络安全工作的更广泛访问权限
settings-image-inputs-unsupported = 模型 { $model } 不支持图片输入。请移除图片或切换模型。
settings-model-changed = 模型已切换为 { $model }（{ $mode }模式）。
settings-model-changed-effort = 模型已切换为 { $model } { $effort }（{ $mode }模式）。
settings-reasoning-default = 默认

## codex-rs/tui/src/chatwidget/skills.rs
skills-list-skills = 列出 skill
skills-tip-dollar = 提示：按 $ 可直接打开此列表
skills-enable-disable = 启用/禁用 skill
skills-enable-or-disable = 启用或禁用 skill
skills-title = skill
skills-choose-action = 选择操作
skills-none-available = 没有可用的 skill。
skills-disabled-count = 已禁用 { $count } 个 skill
skills-enabled-count = 已启用 { $count } 个 skill
skills-enabled-and-disabled = 已启用 { $enabled } 个 skill，已禁用 { $disabled } 个 skill
skills-read-annotation = { $name }（{ $skill } skill）

## codex-rs/tui/src/chatwidget/slash_command_args.rs
slash-command-args-invalid-quoting = 命令参数无效，请检查引号。
slash-command-args-one-of = 只能指定 --{ $name } 或 --no-{ $name } 其中之一。
slash-command-args-duplicate = 命令设置 `{ $name }` 被提供了不止一次。
slash-command-args-requires-value = 命令设置 `{ $name }` 需要一个值。
slash-command-args-no-value = 命令设置 `{ $name }` 不接受值。
slash-command-args-unknown = 未知的命令设置 `{ $name }`。

## codex-rs/tui/src/chatwidget/startup_submission.rs
startup-submission-waiting = 等待启动
startup-submission-cancel = 取消

## codex-rs/tui/src/chatwidget/tui_mode_picker.rs
tui-mode-picker-scrollback = 滚动回看
tui-mode-picker-scrollback-desc = 原生终端的复制、粘贴与回滚
tui-mode-picker-fullscreen = 全屏
tui-mode-picker-fullscreen-desc = 在 Codex 全屏视图中滚动
tui-mode-picker-title = 下次启动的 TUI 模式
tui-mode-picker-footer = 重启后生效。启动时的覆盖设置仍然有效。

## codex-rs/tui/src/chatwidget/transcript_export.rs
transcript-export-title = 导出对话
transcript-export-subtitle = 将完整对话保存为 Markdown
transcript-export-copy = 复制到剪贴板
transcript-export-copy-desc = 复制完整的 Markdown 对话记录
transcript-export-save = 保存到文件
transcript-export-save-desc = 选择 Markdown 文件名
transcript-export-prompt-title = 保存对话
transcript-export-prompt-hint = 输入文件名后按 Enter

## codex-rs/tui/src/chatwidget/tool_requests.rs
tool-requests-subject-reason = { $subject }：{ $reason }
tool-requests-patch-one = apply_patch 涉及 { $file }
tool-requests-patch-many = apply_patch 涉及 { $count } 个文件
tool-requests-network-access = 访问网络：{ $target }
tool-requests-mcp-on = MCP { $tool_name }，位于 { $label }
tool-requests-permission-request = 权限请求
tool-requests-could-action = codex 可以{ $summary }
tool-requests-could-mcp = codex 可以调用 MCP 工具 { $server }.{ $tool_name }
tool-requests-could-access = codex 可以访问 { $target }
tool-requests-could-permissions = codex 可以请求权限
tool-requests-to-action = 让 codex { $summary }
tool-requests-to-mcp = 让 codex 调用 MCP 工具 { $server }.{ $tool_name }
tool-requests-to-access = 让 codex 访问 { $target }
tool-requests-to-permissions = 让 codex 请求权限
tool-requests-send-input = 发送终端输入
tool-requests-network-host = 访问网络 { $host }
tool-requests-question-requested = 已请求提问
tool-requests-questions-requested = 已请求 { $count } 个问题

## codex-rs/tui/src/chatwidget/turn_runtime.rs
turn-runtime-websocket-timing = WebSocket 耗时：{ $label }
turn-runtime-percent-used = 已使用 { $used_percent }%
turn-runtime-tokens-used = 已用 { $tokens }
turn-runtime-high-load = Codex 当前负载较高。
turn-runtime-daybreak-title = 为下一次请求开启 Daybreak？
turn-runtime-daybreak-enable = 启用 Daybreak
turn-runtime-daybreak-not-now = 暂不
turn-runtime-out-of-credits = 额度已用完。你的工作区额度已用完。请添加额度以继续使用 Codex。
turn-runtime-usage-limit-reached = 已达用量上限。你已达到用量上限。请提高上限以继续使用 codex。

## codex-rs/tui/src/chatwidget/usage.rs
usage-n-available = { $count } 次可用
usage-check-availability = 检查可用情况
usage-none-available = 无可用次数
usage-title = 用量
usage-subtitle = 账户用量与重置。
usage-hint-open = 打开
usage-hint-choose = 选择
usage-hint-confirm = 确认
usage-hint-back = 返回
usage-view-analytics = 查看分析
usage-usage-history = 用量历史
usage-redeem-reset = 兑换重置
usage-resets-title = 用量上限重置
usage-checking-resets = 正在检查可用的重置次数...
usage-loading = 加载中...
usage-no-resets = 你目前没有可用的用量上限重置次数。
usage-load-failed = 无法加载用量上限重置次数，请重试。
usage-cancel = 取消
usage-resets-available = { $count } 次用量上限重置可用。
usage-use-this-reset = 使用此次重置？
usage-yes-use-reset = 是，使用此次重置
usage-no-go-back = 否，返回
usage-choose-different = 选择其他重置
usage-close = 关闭
usage-try-again = 重试
usage-resetting = 正在重置用量...
usage-using-reset = 正在使用一次重置...
usage-nothing-to-reset = 你的用量目前无需重置。
usage-reset-gone = 该次重置已不可用。请刷新以查看当前的重置次数。
usage-none-left = 目前没有可用的用量上限重置次数。
usage-reset-failed = 无法重置用量，请重试。
usage-reset-left = 用量已重置。剩余 { $count } 次用量上限重置。
usage-reset-done = 用量已重置。
usage-reset-checking = 用量已重置。正在检查剩余的重置次数...
usage-refreshing = 刷新中...
usage-reset-hint = 你有 { $count } 次用量上限重置可用。运行 /usage 即可使用。

## codex-rs/tui/src/chatwidget/usage_notice.rs
usage-notice-summary = ⚠ { $label } 上限：{ $remaining } 可用
usage-notice-summary-only = ⚠ { $label } 上限：仅 { $remaining } 可用
usage-notice-resets-at = 重置于 { $time }
usage-notice-compact = ⚠ { $label } 剩余 { $remaining }

## codex-rs/tui/src/chatwidget/working_directory.rs
working-directory-not-started = 会话启动后才能更改其工作目录。
working-directory-not-idle = 切换目录要求主会话空闲且没有排队的输入。

## codex-rs/tui/src/chatwidget/windows_sandbox_prompts.rs
windows-sandbox-setup-intro = 设置 Codex 代理沙箱，以保护你的文件并控制网络访问。了解详情 <https://developers.openai.com/codex/windows>
windows-sandbox-org-requires = 你的组织要求使用默认的 Codex 代理沙箱才能继续。请进行设置，以保护你的文件并控制网络访问。
windows-sandbox-learn-more = 了解详情 <https://developers.openai.com/codex/windows>
windows-sandbox-setup-default = 设置默认沙箱（需要管理员权限）
windows-sandbox-use-non-admin = 使用非管理员沙箱（若遭受提示词注入，风险更高）
windows-sandbox-quit = 退出
windows-sandbox-fallback-title = 无法使用管理员权限设置沙箱
windows-sandbox-fallback-still = 你仍可以使用非管理员沙箱运行 Codex。若遭受提示词注入，风险更高。
windows-sandbox-fallback-org = 你的组织要求先设置默认沙箱，Codex 才能继续。
windows-sandbox-retry-admin = 重新尝试设置管理员沙箱
windows-sandbox-use-codex-non-admin = 使用非管理员沙箱运行 Codex
windows-sandbox-input-disabled = 设置完成前无法输入。
windows-sandbox-setting-up = 正在设置沙箱...
windows-sandbox-hang-tight = 请稍候，这可能需要几分钟

## codex-rs/tui/src/chatwidget/worktree_picker.rs
worktree-picker-where-new = 新对话应在哪里运行？
worktree-picker-where-fork = 分叉后的对话应在哪里运行？
worktree-picker-use-current = 使用当前 Git worktree
worktree-picker-use-current-desc = 继续使用当前工作目录
worktree-picker-create-new = 创建新的 Git worktree
worktree-picker-create-new-desc = 在另一个目录中创建此仓库的独立检出
worktree-picker-require-local = 托管的 worktree 需要本地 Git 仓库。
worktree-picker-title = Worktree
worktree-picker-continue = 继续当前对话
worktree-picker-continue-desc = 在新的检出中保留此对话
worktree-picker-start-new = 开始新对话
worktree-picker-start-new-desc = 在新的检出中打开全新对话
worktree-picker-browse = 浏览 worktree
worktree-picker-browse-desc = 恢复所属会话，或复制工作目录
worktree-picker-managed-title = 托管的 worktree
worktree-picker-loading = 正在加载 worktree…
worktree-picker-empty = 此仓库配置的池中没有 worktree
worktree-picker-select = 选择要恢复、复制路径或删除的 worktree
worktree-picker-no-thread = 未关联会话
worktree-picker-owner-unavailable = 所属会话不可用
worktree-picker-copy-label = Worktree 工作目录
worktree-picker-resume-owner = 恢复所属会话
worktree-picker-copy-cwd = 复制工作目录
worktree-picker-not-utf8 = 路径不是有效的 UTF-8
worktree-picker-delete = 删除 worktree
worktree-picker-switch-first = 删除此 worktree 前请先切换到其他检出
worktree-picker-single = Worktree
worktree-picker-delete-title = 删除此 worktree？
worktree-picker-cancel = 取消
worktree-picker-delete-desc = 保留会话历史；可能影响其他会话
worktree-picker-just-now = 刚刚
worktree-picker-list-error = 无法列出托管的 worktree：{ $error }
worktree-picker-status-line = { $status } · { $cwd }
worktree-picker-updated = 更新于 { $ago } · { $cwd }
worktree-picker-archived-updated = 已归档 · 更新于 { $ago } · { $cwd }
worktree-picker-titled = Worktree：{ $title }
worktree-picker-minutes-ago = { $minutes }分钟前
worktree-picker-hours-ago = { $hours }小时前
worktree-picker-days-ago = { $days }天前

## codex-rs/tui/src/chatwidget/slash_dispatch.rs
slash-dispatch-side-starting = 侧边对话启动中…
slash-dispatch-unavailable-side = 侧边对话中 '/{ $command }' 不可用。请先按 Ctrl+C 返回主会话。
slash-dispatch-goal-example = 示例：/goal 提升基准测试覆盖率
slash-dispatch-raw-usage = 用法：/raw [on|off]
slash-dispatch-usage-login = 请登录 ChatGPT 以使用 /usage。
slash-dispatch-collab-disabled = 协作模式已禁用。
slash-dispatch-collab-enable = 请启用协作模式以使用 /plan。
slash-dispatch-plan-unavailable = 计划模式当前不可用。
slash-dispatch-before-start = '/{ $command }' 在会话启动前不可用。
slash-dispatch-task-in-progress = 任务进行中时，'/{ $command }' 已禁用。
slash-dispatch-daybreak-provider = Daybreak 需要 OpenAI 提供方，并使用 ChatGPT 登录，或使用已启用 Daybreak 支持的 API 密钥。
slash-dispatch-daybreak-no-account = 此账户无法使用 Daybreak。请在 https://openai.com/form/enterprise-trusted-access-for-cyber/ 申请。了解详情请访问 https://help.openai.com/en/articles/20001326。
slash-dispatch-daybreak-unknown = 无法从所连主机的模型目录确定 Daybreak 是否可用。
slash-dispatch-daybreak-thread = 会话开始前，Daybreak 不可用。
slash-dispatch-archive-title = 归档此会话？
slash-dispatch-archive-running = 这将停止当前轮次并归档此会话。
slash-dispatch-archive-sure = 确定吗？这将归档当前会话
slash-dispatch-archive-no = 否，不归档
slash-dispatch-return-current = 返回当前会话
slash-dispatch-archive-yes = 是，归档
slash-dispatch-archive-now = 立即归档此会话
slash-dispatch-delete-title = 删除此会话？
slash-dispatch-delete-subtitle = 同时删除子代理和已保存的侧边对话。此操作无法撤销。
slash-dispatch-delete-no = 否，保留此会话
slash-dispatch-delete-yes = 是，删除并返回指挥中心
slash-dispatch-delete-now = 立即永久删除此会话
slash-dispatch-recap-starting = 会话仍在启动，请稍后再试 /recap。
slash-dispatch-no-auto-preset = 内部错误：缺少 'auto' 审批预设。
slash-dispatch-diff-not-git = `/diff` — _不在 git 仓库内_
slash-dispatch-diff-failed = 无法计算 diff：{ $error }
slash-dispatch-diff-no-runner = 无法计算 diff：工作区命令运行器不可用
slash-dispatch-current-cwd = 当前工作目录：{ $path }
slash-dispatch-rollout-path = 当前 rollout 路径：{ $path }
slash-dispatch-rollout-unavailable = rollout 路径尚不可用。
slash-dispatch-pwd-usage = 用法：/pwd
slash-dispatch-usage-usage = 用法：/usage [daily|weekly|cumulative]
slash-dispatch-voice-usage = 用法：/voice [settings|mute|stop]
slash-dispatch-mcp-login-usage = 用法：/mcp login <name> [--environment <id>]
slash-dispatch-mcp-needs-session = MCP 登录需要活动会话。
slash-dispatch-mcp-usage = 用法：/mcp [verbose | login <name> [--environment <id>]]
slash-dispatch-keymap-invalid = `tui.keymap` 配置无效：{ $error }
slash-dispatch-keymap-usage = 用法：/keymap [debug]
slash-dispatch-thread-name-empty = 会话名称不能为空。
slash-dispatch-goal-change-before-start = 会话启动后才能更改目标。
slash-dispatch-goal-set-before-start = 会话启动后才能设置目标。
slash-dispatch-unrecognized = 无法识别的命令 '/{ $name }'。输入 "/" 查看支持的命令列表。
slash-dispatch-review-running = 代码审查进行中时，'/{ $command }' 不可用。

## codex-rs/tui/src/history_cell/async_replies.rs
async-replies-re = 回复：

## codex-rs/tui/src/history_cell/computer_activity.rs
computer-activity-using-computer = 正在使用电脑
computer-activity-used-computer = 已使用电脑
computer-activity-action-count = {" "}· { $count } 个操作
computer-activity-failed-count = {" "}· { $failures } 个失败
computer-activity-computer-action = 电脑操作
computer-activity-failed-error = 失败：{ $error }
computer-activity-failed-title-error = 失败：{ $title } — { $error }
computer-activity-failed-title = 失败：{ $title }
computer-activity-captured-screenshot = 已截取屏幕截图 · { $title }
computer-activity-hidden-more = 另有 { $hidden } 项 · { $key }

## codex-rs/tui/src/history_cell/delegated_message.rs
delegated-message-sent-by-codex = 由 Codex 从任务 { $source_thread_id } 发送

## codex-rs/tui/src/history_cell/dynamic.rs
dynamic-interrupted-before-result = 在此工具返回结果前已中断。
dynamic-image-content = <图像内容>
dynamic-audio-content = <音频内容>
dynamic-interrupted = 已中断
dynamic-calling = 调用中
dynamic-called = 已调用
dynamic-failed = 失败
dynamic-duration-ms = {" "}· { $duration_ms }ms
dynamic-no-output = （无输出）
dynamic-output-unavailable = （输出不可用）
dynamic-earlier-lines-hidden = {" "} … 已隐藏前面 { $skip } 行
dynamic-arguments = 参数：{ $args }

## codex-rs/tui/src/history_cell/exec.rs
exec-waited-for-terminal = 已等待终端
exec-sent-terminal-input = 已发送终端输入
exec-waited-for-background-terminal = 已等待后台终端
exec-interacted-with-background-terminal = 已与后台终端交互
exec-waited-for-background-terminal-command = 已等待后台终端：{ $command }
exec-interacted-with-background-terminal-command = 已与后台终端交互：{ $command }
exec-background-terminals = 后台终端
exec-no-background-terminals-running = 没有正在运行的后台终端。
exec-and-more-running = {"."}.. 另有 { $remaining } 个正在运行

## codex-rs/tui/src/history_cell/hook_cell.rs
hook-cell-running-hooks = 正在运行 hooks
hook-cell-running-hook = 正在运行 hook
hook-cell-hook-prefix = ↳ Hook ·{" "}
hook-cell-hook-completed = Hook 已完成
hook-cell-hook-failed = Hook 失败
hook-cell-blocked-by-hook = 已被 hook 阻止
hook-cell-hook-stopped = Hook 已停止
hook-cell-hook-running = Hook 运行中

## codex-rs/tui/src/history_cell/mcp_result.rs
mcp-result-returned-image = 返回的图片
mcp-result-audio-content = <音频内容>
mcp-result-embedded-resource = 嵌入资源：{ $uri }
mcp-result-unknown-embedded-resource = <未知的嵌入资源>
mcp-result-link = 链接：{ $uri }

## codex-rs/tui/src/history_cell/mcp_preview.rs
mcp-preview-environment = {" "}（环境 { $environment_id }）
mcp-preview-called = 已调用
mcp-preview-failed = 失败
mcp-preview-calling = 调用中
mcp-preview-returned-image = 返回的图片
mcp-preview-error-prefix = 错误：

## codex-rs/tui/src/history_cell/notices.rs
notices-conversation-recap = 会话回顾
notices-run = 运行{" "}
notices-to-update = {" "}以更新。
notices-see = 查看{" "}
notices-for-installation-options = {" "}了解安装选项。
notices-update-available = 有可用更新！
notices-see-full-release-notes = 完整发行说明：
notices-see-installation-options-raw = 安装选项请参阅 https://github.com/lutianyu2001/cod-claw-ntr。
notices-use-daemon = 使用 /daemon 管理本地后台服务。
notices-updating-may-interrupt = 更新可能会中断正在进行或排队中的工作。
notices-content-cant-be-shown = 无法显示此内容
notices-extra-caution-biological = 对于涉及生物研究及可能带来安全风险的应用的请求，我们会格外谨慎。
notices-eligible-researchers = 符合条件的研究人员可申请受信任访问。
notices-trusted-access = 受信任访问
notices-learn-more = 了解更多
notices-daybreak-apply-body = 如果你正在进行经授权的安全工作，请申请 Daybreak 以获得更广泛的访问权限。
notices-apply-for-daybreak = 申请 Daybreak
notices-daybreak-off-body = Daybreak 当前已关闭。如果你正在进行经授权的安全工作，请开启它并重新尝试你的请求。
notices-daybreak-astra-body = Daybreak 不适用于 Astra。部分网络安全请求仍可能受到限制。
notices-daybreak-enabled-body = 即使开启了 Daybreak，部分网络安全请求仍会受到限制。
notices-content-cant-be-shown-curly = 无法显示此内容
notices-extra-care-cyber = 对部分网络安全请求，我们会格外谨慎。
notices-deprecation = 弃用
notices-conversation-interrupted = ■ 会话已中断 - 如果出现问题，请使用 /feedback
notices-generating-recap = 正在生成会话回顾
notices-generating-recap-dots = 正在生成会话回顾...
notices-next-prefix = 下一步：
notices-recap-prefix = 回顾：
notices-recap-prefix-short = 回顾：
notices-next-action = 下一步：{ $action }

## codex-rs/tui/src/history_cell/plans.rs
plans-proposed-plan = 建议计划
plans-empty = （空）
plans-updated-plan-label = 已更新计划
plans-updated-plan = 已更新计划
plans-steps-complete = {" "}· 已完成 { $completed }/{ $total }
plans-no-steps-provided = （未提供步骤）

## codex-rs/tui/src/history_cell/request_user_input.rs
request-user-input-questions = 问题
request-user-input-answered-count = {" "}已回答 { $answered }/{ $total }
request-user-input-interrupted-suffix = {" "}（已中断）
request-user-input-unanswered-suffix = {" "}（未回答）
request-user-input-answer-label = 答案：
request-user-input-note-label = 备注：
request-user-input-interrupted-with-unanswered = { $unanswered } 个未回答，已中断
request-user-input-questions-answered = 问题 { $answered }/{ $total } 已回答
request-user-input-interrupted-raw = （已中断）
request-user-input-unanswered-raw = （未回答）

## codex-rs/tui/src/history_cell/separators.rs
separators-less-than-one-second = <1s
separators-worked-for = 已工作 { $elapsed }
separators-date-format-this-year = %-m月%-d日{" "}
separators-date-format-other-year = %Y年%-m月%-d日{" "}
separators-local-tools = 本地工具：{ $count } 次调用（{ $duration }）
separators-inference = 推理：{ $count } 次调用（{ $duration }）
separators-websocket-events-sent = WebSocket：发送 { $count } 个事件（{ $duration }）
separators-stream-events = 流：{ $count } 个事件（{ $duration }）
separators-websocket-events-received = 已接收 { $count } 个事件（{ $duration }）
separators-responses-api-overhead = Responses API 开销：{ $duration }
separators-responses-api-inference = Responses API 推理：{ $duration }
separators-duration-iapi = { $duration }（iapi）
separators-duration-service = { $duration }（service）
separators-ttft = TTFT：{ $parts }
separators-tbt = TBT：{ $parts }

## codex-rs/tui/src/history_cell/session.rs
session-tip = 提示：{ $tip }
session-get-started = {" "} 开始使用前，请描述一个任务，或尝试以下命令之一：
session-help-init = {" "}- 创建包含 Codex 说明的 AGENTS.md 文件
session-help-status = {" "}- 查看当前会话配置
session-help-permissions = {" "}- 选择 Codex 允许执行的操作
session-help-model = {" "}- 选择模型和推理强度
session-help-review = {" "}- 审查所有更改并查找问题
session-model-changed = 模型已更改：
session-model-requested = 请求的模型：{ $requested_model }
session-model-used = 实际使用：{ $model }
session-permissions-label = {" "} 权限：
session-yolo-mode = YOLO 模式
session-raw-model = 模型：{ $model }{ $reasoning }
session-raw-directory = 目录：{ $directory }
session-raw-permissions-yolo = 权限：YOLO 模式

## codex-rs/tui/src/history_cell/startup_warnings.rs
startup-warnings-startup = 启动
startup-warnings-mcp-source = MCP · { $server }
startup-warnings-mcp-incomplete = MCP 启动未完成：{ $server }
startup-warnings-sign-in-required = 需要登录。

## codex-rs/tui/src/history_cell/warnings.rs
warnings-warning = 警告

## codex-rs/tui/src/exec_cell/compact.rs
compact-exited-status = 已退出（状态 { $status }）
compact-running = 运行中
compact-background-output = 后台输出
compact-ran = 已运行
compact-no-output = （无输出）

## codex-rs/tui/src/exec_cell/live_output.rs
live-output-lines-omitted = … 另有 { $omitted } 行
live-output-bytes-omitted = {"."}.. 已省略 { $bytes } 字节 ...

## codex-rs/tui/src/exec_cell/render.rs
render-interacted-with = 已与 `{ $command_display }` 交互，发送了 `{ $preview }`
render-waited-for = 已等待 `{ $command_display }`
render-nonzero-exits = {" "}· { $count } 个非零退出
render-reading = 读取中
render-read-past = 读取
render-query-in-path = { $query }，位于 { $path }
render-searching-for = 正在搜索
render-searched-for = 已搜索
render-searching-files-in = 正在搜索文件，位于
render-searched-files-in = 已搜索文件，位于
render-listing-files-in = 正在列出文件，位于
render-listed-files-in = 已列出文件，位于
render-running = 运行中
render-ran = 已运行
render-exit-code = {" "}（退出码 { $code }）
render-lines-omitted-hint = … 另有 { $omitted } 行（{ $hint }）
render-lines-omitted = … 另有 { $omitted } 行
render-exploring = 正在探索
render-explored = 已探索
render-title-read = 读取
render-title-list = 列出
render-title-search = 搜索
render-title-run = 运行
render-in = {" "}位于{" "}
render-command-exit-code = {" "}（命令退出码 { $code }）
render-you-ran = 你执行了
render-no-output = （无输出）

## codex-rs/tui/src/app/agent_center/mod.rs
agent-center-filter-all = 全部
agent-center-filter-needs-you = 需要你处理
agent-center-filter-working = 运行中
agent-center-filter-ready = 就绪
agent-center-filter-inactive = 未激活

## codex-rs/tui/src/app/agent_center/hints.rs
hints-navigate = 导航
hints-up = 向上
hints-down = 向下
hints-open = 打开
hints-page-up = 向上翻页
hints-page-down = 向下翻页
hints-quit = 退出
hints-tasks = 任务
hints-new = 新建
hints-new-worktree = 新建 worktree
hints-resume = 恢复
hints-fork = 分叉
hints-rename-label = 重命名
hints-stop = 停止
hints-archive = 归档
hints-hide = 隐藏
hints-delete = 删除
hints-pin-unpin = 置顶/取消置顶
hints-view = 查看
hints-filter = 筛选
hints-search = 搜索
hints-group = 分组
hints-task-shortcuts = 任务快捷键
hints-help-short = 帮助
hints-back = 返回
hints-move = 移动
hints-rename-short = 重命名
hints-open-short = 打开
hints-new-short = 新建

## codex-rs/tui/src/app/agent_center/render.rs
render-search-prefix = 搜索 ›{" "}
render-rename-prefix = 重命名 ›{" "}
render-grouping-project = 项目
render-grouping-directory = 目录
render-grouping-status = 状态
render-grouping-model = 模型
render-agent-command-center = 代理指挥中心
render-group-label = {" "} 分组：{ $grouping }  { $group_key }
render-filter-hint = 筛选
render-creating-worktree = 正在创建 worktree…
render-archiving-task = 正在归档任务…  你仍可以浏览。
render-deleting-task = 正在删除任务…  你仍可以浏览。
render-error-loading-tasks = 加载任务时出错
render-resize-to-see-all = … 调整窗口大小以查看全部

## codex-rs/tui/src/app/agent_center/rows.rs
rows-reconnecting = 正在重新连接…
rows-no-matching-tasks = 没有匹配的任务
rows-loading-tasks = 正在加载任务…
rows-could-not-load-tasks = 无法加载任务
rows-no-tasks-yet = 还没有任务
rows-column-tasks = 任务
rows-column-status = 状态
rows-column-updated = 已更新
rows-searching-history = 正在搜索历史…
rows-show-more-retry = 显示更多（重试）
rows-show-more-matches = 显示更多匹配项
rows-loading-more = 正在加载更多…
rows-show-more = 显示更多
rows-pinned = 已置顶
rows-count-of-total = { $count } / { $total }
rows-voice-badge = {" "} 语音

## codex-rs/tui/src/app/agent_navigation.rs
agent-navigation-picker-subtitle = 选择要查看的代理。前一个 { $previous }，后一个 { $next }。

## codex-rs/tui/src/app/agents_overview.rs
agents-overview-reconnect-failed-stale = 重新连接失败，代理列表已过期；请重新启动以重试
agents-overview-reconnecting-stale = 正在重新连接，代理列表已过期
agents-overview-rename-target-gone = 重命名的目标已消失。未提交的标题：{ $title }
agents-overview-session-unavailable-refresh = 代理会话 { $thread_id } 不可用。请刷新指挥中心后重试。
agents-overview-session-unavailable = 代理会话 { $root_thread_id } 不可用: { $error }
agents-overview-load-task-settings-failed = 无法加载任务设置: { $error }
agents-overview-resume-needs-permissions = 无法在不保留所选权限的情况下恢复任务。
agents-overview-load-conversation-failed = 无法加载此对话，请重试。
agents-overview-attach-failed = 无法连接到任务: { $error }
agents-overview-wait-permissions = 请等待权限更新完成后再开始会话。
agents-overview-load-project-settings-failed = 无法加载项目设置: { $error }
agents-overview-profile-differs = 权限配置文件的设置不同。
agents-overview-load-new-session-settings-failed = 无法加载新会话设置: { $error }
agents-overview-stop-task-failed = 无法停止后台任务: { $error }

## codex-rs/tui/src/app/agents_overview_actions.rs
agents-overview-actions-pin-failed = 无法固定任务：{ $error }
agents-overview-actions-unpin-failed = 无法取消固定任务：{ $error }
agents-overview-actions-untitled-task = 未命名任务
agents-overview-actions-archive-title = 归档“{ $name }”？
agents-overview-actions-archive-task-description = 这将停止此任务中正在运行的工作并将其归档。子代理保持不变。可以从恢复选择器中恢复其历史记录。
agents-overview-actions-archive-task = 归档任务
agents-overview-actions-archive-children-description = 这将停止此任务及其子代理中正在运行的工作，然后将它们归档。可以从恢复选择器中恢复它们的历史记录。
agents-overview-actions-archive-children = 归档任务和子代理
agents-overview-actions-delete-title = 永久删除“{ $name }”？
agents-overview-actions-delete-description = 有保存的侧边对话的任务会保持不变。否则，删除此任务会停止其正在运行的工作，并永久删除其历史记录和子代理的历史记录。
agents-overview-actions-delete = 永久删除任务和子代理
agents-overview-actions-cancel = 取消
agents-overview-actions-could-not-archive = 无法归档任务
agents-overview-actions-could-not-delete = 无法删除任务
agents-overview-actions-work-may-have-stopped = 工作可能已停止。请恢复任务以继续，或重试该操作。
agents-overview-actions-back-to-agents = 返回代理列表

## codex-rs/tui/src/app/agents_overview_details.rs
agents-overview-details-agent = 代理：
agents-overview-details-needs-attention = 需要关注
agents-overview-details-open-to-review = 打开任务进行审查。
agents-overview-details-waiting-approval = 等待审批。
agents-overview-details-waiting-response = 等待你的回复。
agents-overview-details-task-error = 任务遇到错误。
agents-overview-details-latest-activity = 最近活动

## codex-rs/tui/src/app/agents_overview_errors.rs
agents-overview-errors-return = 返回指挥中心
agents-overview-errors-unable = 无法完成操作

## codex-rs/tui/src/app/agents_overview_grouping.rs
agents-overview-grouping-unknown = 未知
agents-overview-grouping-save-failed = 无法保存指挥中心分组: { $err }

## codex-rs/tui/src/app/agents_overview_loading.rs
agents-overview-loading-task = 正在加载任务…

## codex-rs/tui/src/app/agents_overview_new.rs
agents-overview-new-checkout-retained = { $reason } 已保留一个检出副本，位于 { $path }；如果不再需要，请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。
agents-overview-new-could-not-start = 无法启动会话。
agents-overview-new-could-not-load-settings = 无法加载新会话设置。
agents-overview-new-sessions-paused = 正在进行的轮次完成或被取消之前，新会话会暂停。默认的关闭等待时间最长为一分钟。
agents-overview-new-will-reconnect = 服务器重启后，Codex 会自动重新连接。
agents-overview-new-server-shutting-down = Codex 的后台服务正在关闭
agents-overview-new-start-failed = 无法启动会话: { $error }
agents-overview-new-worktree-not-applied = 服务器未应用 worktree 目录。
agents-overview-new-could-not-open = 无法打开新会话。
agents-overview-new-worktree-support = 受管 worktree 需要本地 worktree 支持。
agents-overview-new-project-untrusted = 源项目不受信任。
agents-overview-new-worktree-task-failed = worktree 创建任务失败: { $error }

## codex-rs/tui/src/app/agents_overview_usage.rs
agents-overview-usage-tokens-in = { $count } 输入
agents-overview-usage-tokens-out = { $count } 输出
agents-overview-usage-tokens = Token：
agents-overview-usage-credits = { $count } 积分
agents-overview-usage-estimated = 预估用量：

## codex-rs/tui/src/app/agents_overview_view.rs
agents-overview-view-needs-input = 需要输入
agents-overview-view-working = 运行中
agents-overview-view-ready = 就绪
agents-overview-view-inactive = 未激活
agents-overview-view-untitled-task = 未命名任务
agents-overview-view-error = 错误
agents-overview-view-model = 模型：
agents-overview-view-reasoning = 推理强度：
agents-overview-view-unknown = 未知
agents-overview-view-task-details = 任务详情
agents-overview-view-project = 项目
agents-overview-view-branch = 分支
agents-overview-view-prompt = 提示词
agents-overview-view-no-prompt = 没有可用的提示词。
agents-overview-view-last-message = 最后一条消息

## codex-rs/tui/src/app/app_server_events.rs
app-server-events-windows-sandbox-setup-failed = Windows 沙箱设置失败：{ $error }
app-server-events-unknown-error = 未知错误

## codex-rs/tui/src/app/backend_banner_fallback.rs
backend-banner-fallback-switched-to = 已自动切换到
backend-banner-fallback-usage-limits = {" "}因用量限制。
backend-banner-fallback-switched-back-to = 已自动切换回
backend-banner-fallback-usage-recovered = {" "}因普通用量已再次可用。

## codex-rs/tui/src/app/backend_selection.rs
backend-selection-connection-unavailable = 该连接不可用。请选择其他连接后重试。
backend-selection-end-voice-call = 切换前请先结束语音通话。
backend-selection-wait-for-turn = 请等待当前轮次完成后再切换。
backend-selection-settings-unavailable = 连接设置不可用。
backend-selection-prepare-new-failed = 无法准备新对话: { $error }
backend-selection-read-defaults-failed = 无法读取新会话的默认设置: { $error }
backend-selection-start-new-failed = 无法启动新对话: { $error }
backend-selection-open-new-failed = 无法打开新对话: { $error }
backend-selection-save-default-failed = 已切换，但无法保存此项目的默认设置: { $error }
backend-selection-settings-unavailable-reopen = 连接设置不可用。请重新打开会话选择器。
backend-selection-conversation-connection-unavailable = 此对话的连接不可用。请重新打开会话选择器。
backend-selection-prepare-failed = 无法准备对话: { $error }

## codex-rs/tui/src/app/backend_sessions.rs
backend-sessions-cloud = 云端
backend-sessions-local = 本地
backend-sessions-connection = 连接 { $number }

## codex-rs/tui/src/app/cloud_environments.rs
cloud-environments-title = 云端环境
cloud-environments-loading = 正在加载已保存的环境…
cloud-environments-use-default = { $error }。你仍可以使用默认环境。
cloud-environments-default = 默认
cloud-environments-default-description = 如果可用，使用当前 GitHub 项目的默认环境
cloud-environments-workspace = 工作区环境
cloud-environments-personal = 个人环境
cloud-environments-search = 搜索环境

## codex-rs/tui/src/app/cloud_switch_wait.rs
cloud-switch-wait-connecting = 正在连接云端…
cloud-switch-wait-did-not-open = 云端对话未能打开
cloud-switch-wait-draft-kept = 已保留草稿

## codex-rs/tui/src/app/clipboard.rs
clipboard-busy = 剪贴板正忙，请重试
clipboard-paste-image-failed = 无法粘贴图片: { $err }

## codex-rs/tui/src/app/config_panel.rs
config-panel-could-not-save = 无法保存: { $error }
config-panel-rebuild-config-failed = 无法为 cwd { $cwd } 重建配置
config-panel-saved-reload-failed = 已保存，但无法重新加载: { $error }
config-panel-saved-overridden = 已保存 · 被配置或系统设置覆盖
config-panel-saved-new-composers = 已保存 · 对新输入框生效
config-panel-test-already-scheduled = 测试已安排 · 切换应用
config-panel-test-scheduled = 测试将在 5 秒后进行 · 切换应用
config-panel-notification-test = Codex 通知测试
config-panel-test-send-failed = 无法发送测试 · 请检查终端连接
config-panel-test-sent-ghostty = 测试已发送 · Ghostty 在获得焦点时会隐藏横幅
config-panel-test-sent = 测试已发送 · 请检查终端或系统设置

## codex-rs/tui/src/app/daemon_menu.rs
daemon-menu-running = 正在运行的后台服务：{ $version }
daemon-menu-not-connected = 未连接到本地后台服务。
daemon-menu-title = 后台服务
daemon-menu-remote-unavailable = 请在其所在主机上管理此服务器。远程连接无法更新本地后台服务。
daemon-menu-run-cli = 运行 Codex CLI，以便从此菜单管理后台服务。
daemon-menu-use-this-cli = 使用此 CLI 构建版本
daemon-menu-no-local-package = 此 CLI 没有可复制的本地包
daemon-menu-restart = 重启后台服务
daemon-menu-restart-unverified = Codex 尚未确认它管理此服务器。请在启动它的地方重启。
daemon-menu-restart-title = 重启后台服务？
daemon-menu-restart-interrupt = 其他对话中正在进行或排队中的工作可能会被中断。
daemon-menu-restart-reconnect = Codex 会自动重新连接。
daemon-menu-cancel = 取消
daemon-menu-restart-button = 重启
daemon-menu-this-cli-explanation = 使用来自 { $path } 的此 CLI 包 v{ $version }。复制完整的包，并将其固定，避免自动更新。
daemon-menu-update-interrupt = 此操作可能会重启后台服务，并中断正在进行或排队中的工作。
daemon-menu-update-exit = Codex 将退出以在此终端中更新，之后请重新启动。
daemon-menu-update-title = 更新后台服务并退出 Codex？
daemon-menu-update-and-exit = 更新并退出

## codex-rs/tui/src/app/background_requests.rs
background-requests-add-marketplace-failed = 无法添加插件市场: { $err }
background-requests-remove-marketplace-failed = 无法移除插件市场: { $err }
background-requests-upgrade-marketplace-failed = 无法升级插件市场: { $err }
background-requests-install-plugin-failed = 无法安装插件: { $err }
background-requests-uninstall-plugin-failed = 无法卸载插件: { $err }
background-requests-update-plugin-config-failed = 无法更新插件配置: { $err }
background-requests-update-hook-config-failed = 无法更新 hook 配置: { $error }
background-requests-trust-hook-failed = 无法信任 hook: { $error }
background-requests-trust-hooks-failed = 无法信任 hooks: { $error }
background-requests-feedback-zip-needs-local = 保存反馈 ZIP 需要本地应用服务器。
background-requests-feedback-zip-saved = 反馈 ZIP 已保存到 { $path }
background-requests-load-mcp-inventory-failed = 无法加载 MCP 清单: { $err }
background-requests-next-step-sign-in-chatgpt-auth = 请使用 ChatGPT 账号登录；API 密钥认证无法加载远程插件目录
background-requests-next-step-sign-in-retry-section = 请登录 ChatGPT，然后重新尝试加载此部分
background-requests-next-step-admin-enable-plugins = 请工作区管理员启用 Codex 插件或插件共享
background-requests-next-step-switch-workspace = 切换到对应的工作区，或向分享者申请访问权限
background-requests-next-step-check-workspace-access = 请检查是否已登录正确的工作区，并且仍有访问权限
background-requests-next-step-update-codex = 请更新 Codex，然后重新尝试打开共享插件
background-requests-next-step-try-again-later = 请稍后再试；本地插件功能仍可使用
background-requests-next-step-admin-confirm-access = 请工作区管理员确认插件访问权限
background-requests-next-step-sharer-confirm-access = 请分享者或工作区管理员确认插件访问权限
background-requests-enable-plugin-sharing = 为此 Codex 会话启用插件共享，以加载共享插件

## codex-rs/tui/src/app/config_persistence.rs
config-persistence-task-failed = { $context } 任务失败
config-persistence-rebuild-config-cwd-failed = 无法为 cwd { $cwd } 重建配置
config-persistence-rebuild-config-profile-failed = 无法为权限配置 { $profile_id } 重建配置
config-persistence-set-profile-failed = 无法设置权限配置 `{ $profile_id }`：{ $err }
config-persistence-set-approval-policy-failed = 无法设置审批策略
config-persistence-permissions-updated = 权限已更新为 { $label }
config-persistence-wait-task-connect = 请等任务连接完成后再选择权限。
config-persistence-selection-requested = 已请求权限选择：{ $label }
config-persistence-selection-needs-newer-server = 权限选择需要更新版本的应用服务器。
config-persistence-select-permissions-failed = 无法选择权限：{ $error }
config-persistence-wait-before-changing = 请等权限更新完成后再更改权限。
config-persistence-wait-before-switching = 请等权限更新完成后再切换任务。
config-persistence-saved-refresh-failed = { $setting }已保存，但 Codex 无法刷新生效的配置：{ $err }
config-persistence-carry-approval-policy-failed = 无法沿用审批策略覆盖：{ $err }
config-persistence-carry-approval-policy-failed-context = 无法沿用审批策略覆盖
config-persistence-carry-reviewer-failed = 无法沿用审批审查者
config-persistence-carry-profile-failed-context = 无法沿用权限配置覆盖
config-persistence-carry-profile-failed = 无法沿用权限配置覆盖：{ $err }
config-persistence-prefixed-error = { $prefix }：{ $err }
config-persistence-unsupported-profile = { $prefix }：不支持的活动权限配置 `{ $id }`
config-persistence-update-feature-failed = 无法更新实验性功能 `{ $feature_key }`：{ $err }
config-persistence-label-approve-for-me = 自动审批
config-persistence-label-ask-for-approval = 请求审批
config-persistence-enable-approve-failed-prefix = 无法启用“替我批准”
config-persistence-update-features-failed = 无法更新实验性功能：{ $error }
config-persistence-features-saved-not-applied = 实验性功能的更改已保存，但未生效：{ $message }
config-persistence-setting-experimental-features = 实验性功能的更改
config-persistence-enable-approve-failed = 无法启用“替我批准”：{ $err }
config-persistence-save-memory-failed = 无法保存记忆设置：{ $err }
config-persistence-memory-saved-not-applied = 记忆设置的更改已保存，但未生效：{ $message }
config-persistence-setting-memory = 记忆设置的更改
config-persistence-memory-thread-update-failed = 记忆设置已保存，但无法更新当前会话：{ $err }
config-persistence-reset-memories-failed = 无法重置记忆：{ $err }
config-persistence-reset-memories = 已重置本地记忆。
config-persistence-reasoning-default = 默认
config-persistence-overrides-unsupported-remote = 恢复远程任务时不支持权限覆盖。
config-persistence-refresh-approve-failed = 无法刷新被覆盖的“替我批准”设置：{ $err }
config-persistence-refresh-approve-failed-prefix = 无法刷新被覆盖的“替我批准”设置
config-persistence-windows-sandbox-overridden = Windows 沙箱设置已完成，但其模式被生效的配置覆盖。
config-persistence-overridden-by-layer = 生效的配置被更高优先级的层覆盖

## app/provider/rpc.rs

provider-rpc-override-reason = 更高优先级的设置会覆盖此选择。
provider-rpc-update-server = 请更新已连接的 app-server 以浏览模型提供方。
provider-rpc-load-failed = 无法加载模型提供方：{ $err }

## app/resume_config.rs

resume-config-cwd-needs-cd = `tui.resume_cwd = "current"` 在使用远程工作区时需要 `--cd`
resume-config-cwd-failed = 为恢复确定工作目录失败：{ $err }
resume-config-rebuild-failed = 为恢复重新构建配置失败：{ $err }
resume-config-check-folder-failed = 无法检查恢复的文件夹：{ $error }
resume-config-check-trust-failed = 无法检查文件夹信任：{ $error }
resume-config-reload-trust-failed = 重新加载受信任文件夹设置失败：{ $error }
resume-config-load-hooks-failed = 无法加载文件夹 hooks：{ $error }
resume-config-review-hooks-failed = 无法审查文件夹 hooks：{ $error }

## daybreak.rs

daybreak-requires-openai = Daybreak 需要 OpenAI 提供方，并且使用 ChatGPT 登录或启用了 Daybreak 支持的 API 密钥。请先将其关闭再继续。
daybreak-unconfirmed = 已连接的服务器无法确认模型 { $model } 支持 Daybreak。请使用 /daybreak 将其关闭，或选择兼容的模型和服务器。

## device_login.rs

device-login-gateway-failed = 网关登录失败
device-login-lost-result = 丢失了网关登录结果。请重启 Codex 以检查连接。

## external_agent_config_migration/flow.rs

import-flow-source-error = { $source }：{ $error }
import-flow-check-failed = 无法检查可导入的设置：{ $errors }
import-flow-started = 导入已开始。
import-flow-keep-working = {" "}完成前你可以继续工作。
import-flow-applies-new-chats = 导入的设置将应用于新对话。
import-flow-importing = 正在导入：
import-flow-more = ，另有 { $count } 项
import-flow-failed-count = { $count } 项失败
import-flow-finished = 导入完成：
import-flow-imported-count = 已导入 { $count } 项
import-flow-results-by-type = 按类型统计：
import-flow-run-again = 请再次运行 /import 检查其他项目。
import-flow-remaining = 还剩 { $count } 个项目。完成后请再次运行 /import 进行查看。
import-flow-source-gone = 所选导入来源已不可用。
import-flow-failed = 导入失败：{ $err }

## goal_files.rs

goal-files-empty = 目标不能为空。
goal-files-reference-too-long = 目标文件引用过长：{ $actual_chars } 个字符。上限：{ $limit } 个字符。

## lib.rs

lib-workload-identity-remote-host = workload identity 必须在远程 app-server 主机上配置
lib-config-selects-other-provider = 当前配置选用的是 { $current }，而不是 { $selected_id }。
lib-no-saved-session = 未找到 ID 为 { $id_str } 的已保存会话。请运行不带 ID 的 `codex { $action }`，从现有会话中选择。
lib-cloud-connection-failed = 无法添加云端连接。{ $error }
lib-restore-terminal-failed = 无法还原终端。请运行 `reset` 或重启终端以恢复：{ $err }
lib-config-load-error = 加载配置出错：{ $err }
lib-config-toml-error-detail = 加载 config.toml 出错：{"\u000A"}{ $detail }
lib-config-toml-error = 加载 config.toml 出错：{ $err }

## startup_draft_input.rs

startup-draft-waiting = 等待启动
startup-draft-cancel = 取消

## startup_recovery.rs

startup-recovery-unsent-draft = {"\u000A"}未发送的草稿：{"\u000A"}{ $text }{"\u000A"}

## theme_picker.rs

theme-picker-fallback-subtitle = 上下移动以实时预览主题
theme-picker-custom-subtitle = 可以把自定义的 .tmTheme 文件添加到 { $path } 目录中。
theme-picker-custom-name = { $name }（自定义）
theme-picker-title = 选择语法主题
theme-picker-search = 输入以筛选主题…

## pager_overlay/transcript.rs

pager-transcript-latest = { $key } 最新
pager-transcript-select = { $key } 选择
pager-transcript-selection = 所选内容
pager-transcript-open-link-failed = 无法打开链接：{ $error }
pager-transcript-close = 关闭
pager-transcript-find = 查找
pager-transcript-browse-prompts = 浏览提示词
pager-transcript-edit-next = 编辑下一条
pager-transcript-edit-message = 编辑消息

## thread_transcript.rs

thread-transcript-empty = 没有可用的对话内容

## app/clipboard.rs

clipboard-last-message = 最后一条消息

## app/resize_reflow.rs

resize-reflow-earlier-messages = 更早的消息可用，按 { $key } 查看完整对话记录

## transcript_view/turn_details.rs

turn-details-ran = 已运行 { $ran } 个命令
turn-details-explored = 探索文件
turn-details-explored-ran = 探索文件 · 已运行 { $ran } 个命令
turn-details-tool-calls = { $count } 个工具调用 · { $label }
turn-details-previous-messages = { $count } 条之前的消息

## app/event_dispatch.rs

event-dispatch-open-this-task-before-using = 使用该操作前请先打开此任务。
event-dispatch-windows-sandbox-setup-requires-loca = Windows 沙箱设置需要本地连接和执行器。
event-dispatch-a-worktree-is-already-being = 正在创建 worktree。
event-dispatch-changing-directories-requires-an-id = 切换目录要求主会话空闲且没有排队的输入。
event-dispatch-changing-directories-is-not-support = 远程工作区或远程执行环境不支持切换目录。
event-dispatch-export-failed = 导出失败：{ $error }
event-dispatch-nothing-to-copy = 没有可复制的内容
event-dispatch-no-saved-chat-found-matching = 未找到与 '{ $id_or_name }' 匹配的已保存对话。
event-dispatch-failed-to-archive-thread = 归档会话 { $thread_id } 失败：{ $error }
event-dispatch-wait-for-permissions-to-update = 分叉前请等待权限更新完成。
event-dispatch-failed-to-name-the-forked = 为分叉出的会话命名失败：{ $err }
event-dispatch-fork-created-you-can-continue = 已创建分叉。你可以在此继续。
event-dispatch-failed-to-attach-to-forked = 连接到分叉出的 app-server 会话失败：{ $err }
event-dispatch-failed-to-fork-current-session = 通过 app-server 分叉当前会话失败：{ $err }
event-dispatch-a-thread-must-contain-at = 会话至少包含一个轮次才能分叉。
event-dispatch-this-remote-prompt-contains-local = 这条远程提示包含无法安全还原的本地图片路径或命令语法。请重新输入消息并重新附加图片。
event-dispatch-wait-for-permissions-to-update-2 = 编辑此提示前请等待权限更新完成。
event-dispatch-quit-and-stop-running-sessions = 退出并停止正在运行的会话？
event-dispatch-quitting-this-tui-will-stop = 退出此 TUI 将停止其正在运行的会话。
event-dispatch-stay-in-codex = 留在 Codex
event-dispatch-stop-sessions-and-quit = 停止会话并退出
event-dispatch-failed-to-interrupt-task = 无法中断任务: { $error }
event-dispatch-failed-to-pause-task-goal = 无法暂停任务目标: { $error }
event-dispatch-logout-failed = 退出登录失败: { $err }
event-dispatch-failed-to-start-turn = 无法开始轮次: { $err }
event-dispatch-voice-conversation-failed = 语音对话失败: { $err }
event-dispatch-no-changes-detected = 未检测到更改。
event-dispatch-mcp-sign-in-is-starting = MCP 登录正在开始。请等待其完成后再试。
event-dispatch-voice-conversation-failed-2 = 语音对话失败: { $error }
event-dispatch-cloud-model-catalog-account-changed = 云端模型目录的账号已更改。
event-dispatch-failed-to-save-default-model = 无法保存默认模型: { $error }
event-dispatch-model-changed-to-for-this = 此对话的模型已切换为 { $model } { $effort }
event-dispatch-sandbox-ready = 沙箱已就绪
event-dispatch-codex-can-now-safely-edit = Codex 现在可以在你的电脑上安全地编辑文件和执行命令
event-dispatch-daybreak-has-been-turned-off = 由于你当前的账号和服务器无法使用 { $model } 的 Daybreak，新轮次已关闭 Daybreak。此会话中先前的网络安全上下文可能导致拒绝。你可以切换回拥有 Daybreak 权限的模型，并使用 /daybreak 重新开启。
event-dispatch-cyber-models-default-to-approve = 出于安全考虑，网络安全模型默认使用 "为我批准"。
event-dispatch-the-selected-cloud-model-does = 所选云端模型不支持该速度。
event-dispatch-selected-speed-for-the-next = 已为此对话的下一个新轮次选择 { $label } 速度
event-dispatch-the-cloud-account-changed-reopen = 云端账号已更改。选择速度前请重新打开对话。
event-dispatch-service-tier-set-to = 服务层级已设为 { $service_tier }
event-dispatch-service-tier-cleared = 服务层级已清除
event-dispatch-failed-to-set-permission-profile = 无法设置权限配置文件: { $err }
event-dispatch-failed-to-save-rate-limit = 无法保存速率限制提醒偏好: { $err }
event-dispatch-failed-to-save-plan-mode = 无法保存计划模式推理强度: { $err }
event-dispatch-failed-to-save-model-migration = 无法保存模型迁移提示偏好: { $err }
event-dispatch-close-the-side-conversation-before = 分叉另一个任务前，请先关闭侧边对话。
event-dispatch-failed-to-rename-task = 无法重命名任务: { $error }
event-dispatch-could-not-open-file = 无法打开文件: { $error }
event-dispatch-failed-to-update-skill-config = 无法更新 { $path_display } 的 skill 配置: { $err }
event-dispatch-failed-to-update-app-config = 无法更新 { $id } 的应用配置: { $err }
event-dispatch-environment = 环境：
event-dispatch-reason = 原因：
event-dispatch-permission-rule = 权限规则：
event-dispatch-server = 服务器：
event-dispatch-failed-to-save-status-line = 保存状态栏设置失败：{ $error }
event-dispatch-failed-to-save-terminal-title = 保存终端标题项失败：{ $err }
event-dispatch-failed-to-open-setting-because = 无法打开设置，因为清除终端输入失败：{ $err }
event-dispatch-failed-to-save-theme = 保存主题失败：{ $err }
event-dispatch-wait-for-the-current-task = 运行 /recap 前请等待当前任务完成。
event-dispatch-failed-to-save-shortcut = 保存快捷键失败：{ $err }
event-dispatch-failed-to-refresh-shortcuts = 刷新快捷键失败：{ $err }
event-dispatch-removed-custom-shortcut-for = 已移除 `{ $context }.{ $action }` 的自定义快捷键。
event-dispatch-failed-to-remove-shortcut = 移除快捷键失败：{ $err }
event-dispatch-a-thread-must-start-before = 会话必须先启动才能归档。
event-dispatch-archiving = 正在归档…
event-dispatch-failed-to-archive-current-thread = 归档当前会话失败：{ $error }
event-dispatch-a-thread-must-start-before-2 = 会话必须先启动才能删除。
event-dispatch-failed-to-delete-current-thread = 删除当前会话失败：{ $err }
event-dispatch-worktree-removed = 已移除 { $path } 处的 worktree。会话历史已保留。
event-dispatch-worktree-remove-failed = 无法移除 { $path } 处的 worktree：{ $error }
event-dispatch-not-directory = 不是目录：{ $path }
event-dispatch-cannot-access-dir = 无法访问目录 { $path }：{ $error }
event-dispatch-save-reviewer-failed = 无法保存审批审核者：{ $error }

## bottom_pane/request_user_input/mod.rs

request-input-add-notes = 添加备注
request-input-answer-placeholder = 输入你的回答（可选）
request-input-select-option = 选择一个选项以添加备注
request-input-none-of-above = 以上都不是
request-input-other-desc = 可在备注中补充详情（tab）
request-input-go-back-desc = 返回第一个未回答的问题
request-input-go-back = 返回
request-input-proceed = 继续提交
request-input-submit-unanswered = 有 { $count } 个问题未回答，仍要提交
request-input-auto-resolves = { $time } 后自动处理
request-input-question-progress = 问题 { $idx }/{ $total }
request-input-question-progress-unanswered = 问题 { $idx }/{ $total }（{ $unanswered } 个未回答）
request-input-no-questions = 没有问题
request-input-to-add-notes = 添加备注
request-input-or = {" "}或{" "}
request-input-to-clear-notes = {" "}清除备注
request-input-to-submit-answer = 提交回答
request-input-to-submit-all = 全部提交
request-input-to-navigate = 切换问题
request-input-change-question = 切换问题
request-input-to-interrupt = 中断

## bottom_pane/mod.rs

bottom-pane-answer-the-questions-to-continue = 请回答问题以继续。
bottom-pane-install-this-app-in-your = 请在浏览器中安装此应用，然后返回这里。
bottom-pane-enable-this-app-to-use = 请启用此应用，以便用于当前请求。
bottom-pane-respond-to-the-tool-suggestion = 请回复工具建议以继续。
bottom-pane-respond-to-the-mcp-server = 请回复 MCP 服务器的请求以继续。

## tui/src/app/daybreak.rs
daybreak-unavailable-for-account = 无法确认此账号是否可使用 Daybreak。
daybreak-save-thread-failed = 无法为此会话保存 Daybreak：{ $error }
daybreak-read-thread-failed = 无法读取会话以保存 Daybreak：{ $error }
daybreak-default-preference-setting = 默认 Daybreak 偏好
daybreak-read-default-failed = 无法读取默认 Daybreak 偏好：{ $error }
daybreak-save-default-failed = 无法保存默认 Daybreak 偏好：{ $error }
daybreak-on-applies-to-new-turns = Daybreak 已开启。对新轮次生效。
daybreak-off-applies-to-new-turns = Daybreak 已关闭。对新轮次生效。

## tui/src/app/quick_dials.rs
quick-dials-defined-outside-config = 预设 { $slot } 在用户配置之外定义。请改为编辑其源配置。
quick-dials-update-failed = 更新预设 { $slot } 失败：{ $error }
quick-dials-saved = 已保存预设 { $slot }。
quick-dials-deleted = 已删除预设 { $slot }。

## tui/src/app/transcript_export.rs
transcript-export-no-active = 没有可导出的活动对话。
transcript-export-saved = 对话已保存到 { $path }
transcript-export-load-failed = 无法加载对话：{ $error }
transcript-export-load-history-failed = 无法加载对话记录：{ $error }
transcript-export-no-content = 没有可导出的对话内容。
transcript-export-no-home-dir = 无法确定主目录
transcript-export-create-failed = 无法创建 { $path }：{ $error }
transcript-export-write-failed = 无法写入 { $path }：{ $error }

## tui/src/app/history_ui.rs
history-ui-open-browser-failed = 无法为 { $url } 打开浏览器：{ $error }

## tui/src/app/working_directory.rs
working-directory-idle-required = 切换目录要求主会话空闲且没有排队的输入。
working-directory-remote-unsupported = 远程工作区或远程执行环境不支持切换目录。
working-directory-task-not-replaceable = 此任务无法安全替换。
working-directory-wait-permissions = 请等待权限更新完成后再切换目录。
working-directory-named-profile-unsupported = 使用命名配置文件时不支持切换目录。
working-directory-unconfirmed-profile-unsupported = 使用未确认的命名配置文件时不支持切换目录。
working-directory-mcp-loading = MCP 清单仍在加载。
working-directory-agent-running = 无法更改：另一个代理正在运行。
working-directory-history-not-saved = 对话历史不会被保存。
working-directory-profile-differs = 权限配置文件的设置不同。
working-directory-profile-not-preserved = /cd 无法保留权限配置文件。
working-directory-agent-status-failed = 无法检查代理状态。
working-directory-task-running = 无法切换：任务正在运行。
working-directory-not-applied = 未应用所请求的目录或权限。
working-directory-offline-checkout = 会话离线或被策略警告阻止时，无法继续进入新的 worktree。已在 { $path } 创建一个未使用的检出副本；请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。
working-directory-source-changed-checkout = 创建 worktree 期间源对话发生了变化。已在 { $path } 创建一个未使用的检出副本；请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。
working-directory-fork-instructions-differ = 开发者指令不同，无法分叉到此 worktree。请改为开始新对话。已在 { $path } 创建一个未使用的检出副本；请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。
working-directory-retained-checkout = 无法在托管的 worktree 中启动会话。检出副本保留在 { $path }；如果不再需要，请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。
working-directory-load-failed = 无法加载 { $cwd }：{ $error }
working-directory-read-defaults-failed = 无法读取默认设置：{ $error }
working-directory-task-status-failed = 无法检查任务状态：{ $error }
working-directory-change-failed = 切换失败：{ $error }
working-directory-register-ownership-failed = 无法登记托管 worktree 的归属：{ $error }
working-directory-name-session-failed = 无法为 worktree 会话命名：{ $error }
working-directory-change-dirs-failed = 无法切换目录：{ $error }
working-directory-restore-failed = 无法恢复会话：{ $error }
working-directory-changed = 工作目录已切换为：{ $path }

## tui/src/clipboard_copy/tmux.rs
clipboard-tmux-unavailable = 系统 PATH 中找不到 tmux
clipboard-tmux-resolve-path-failed = 无法解析系统 PATH：{ $error }
clipboard-tmux-no-pane = tmux 剪贴板转发不可用：没有当前面板
clipboard-tmux-spawn-failed = 启动 tmux 失败：{ $error }
clipboard-tmux-open-stdin-failed = 打开 tmux 的 stdin 失败
clipboard-tmux-write-failed = 写入 tmux 失败：{ $error }
clipboard-tmux-wait-failed = 等待 tmux 失败：{ $error }
clipboard-tmux-exit-status = tmux 退出，状态码 { $status }
clipboard-tmux-failed = tmux 失败：{ $stderr }
clipboard-tmux-disabled = tmux 剪贴板转发已禁用
clipboard-tmux-no-client = tmux 剪贴板转发不可用：没有已附加的客户端
clipboard-tmux-missing-ms = tmux 剪贴板转发不可用：缺少 Ms 能力
clipboard-tmux-output-not-utf8 = tmux 输出不是 UTF-8：{ $error }

## tui/src/clipboard_paste.rs
clipboard-paste-unavailable = 剪贴板不可用：{ $error }
clipboard-paste-no-image = 剪贴板中没有图片：{ $error }
clipboard-paste-encode-failed = 无法编码图片：{ $error }
clipboard-paste-io-error = I/O 错误：{ $error }
clipboard-paste-android-unsupported = Android 上不支持粘贴剪贴板图片

## tui/src/model_migration.rs
model-migration-try-new = 尝试新模型
model-migration-use-existing = 继续使用现有模型
model-migration-upgrade-heading = Codex 刚刚升级。现推出 { $name }。
model-migration-recommended = 推荐使用 { $name }，性能与可靠性更好。
model-migration-recommend-switch = 建议从 { $current } 切换到 { $target }。
model-migration-learn-more = { $description } 了解 { $name } 的更多信息，请访问{" "}
model-migration-keep-existing = 如果你更喜欢，可以继续使用 { $current }。
model-migration-hint-confirm = {" "}确认 ·{" "}
model-migration-hint-continue = {" "}继续 ·{" "}
model-migration-hint-quit = {" "}退出

## tui/src/oss_selection.rs
oss-selection-lmstudio-desc = 本地 LM Studio 服务器（默认端口 1234）
oss-selection-ollama-desc = 本地 Ollama 服务器（Responses API，默认端口 11434）
oss-selection-title = 选择开源提供方
oss-selection-subtitle = 为本次会话选择本地 AI 服务器。
oss-selection-running = ● 运行中
oss-selection-not-running = ○ 未运行
oss-selection-unknown = ? 未知
oss-selection-hint-choose = {" "}选择 ·{" "}
oss-selection-hint-select = {" "}选择 ·{" "}
oss-selection-hint-lmstudio = {" "}LM Studio ·{" "}
oss-selection-hint-exit = {" "}退出

## tui/src/startup_draft_layout.rs
startup-draft-resuming = {"  "}正在恢复会话…
startup-draft-forking = {"  "}正在分叉会话…

## tui/src/token_usage.rs
token-usage-summary = token 用量：总计={ $total } 输入={ $input }{ $cached } 输出={ $output }{ $reasoning }
token-usage-cached = {" "}（+ { $count } 个已缓存）
token-usage-reasoning = {" "}（推理 { $count }）

## tui/src/app_backtrack.rs
app-backtrack-no-previous-message = 没有可编辑的上一条消息。
app-backtrack-edit-failed = 无法编辑所选提示词：{ $error }

## tui/src/app_backtrack.rs (continued)
app-backtrack-steer = 所选提示词是一条引导消息，无法单独编辑
app-backtrack-in-progress = 所选提示词所属的轮次仍在进行中
app-backtrack-mismatch = 所选对话记录中的提示词与已保存的会话不再一致
app-backtrack-not-found = 在已保存的会话中找不到所选提示词
app-backtrack-boundary-missing = 已加载历史的边界已不存在


## tui/src/app/feedback_upload.rs
feedback-upload-failed-to-upload = 上传反馈失败
feedback-upload-failed-to-save-zip = 保存反馈 ZIP 失败
feedback-upload-no-codex-home = app-server 未报告其 Codex 主目录
feedback-upload-create-dir-failed = 无法在 { $directory } 创建反馈目录
feedback-upload-update-server = 请更新 Codex 服务器以保存反馈 ZIP

## tui/src/app/model_defaults.rs
model-defaults-setting-default-model = 默认模型和推理强度
model-defaults-model-changed = 模型已更改为 { $model }
model-defaults-failed-to-save = 无法保存默认模型: { $error }
model-defaults-open-cloud-first = 选择模型前请先打开 Cloud 对话。
model-defaults-choose-supported-cloud = 请以当前速度选择受支持的 Cloud 模型和强度。
model-defaults-speed-fast = 快速
model-defaults-speed-standard = 标准
model-defaults-speed-unsupported = { $model } 不支持 { $tier } 速度。已清除待生效的速度更改。
model-defaults-cloud-account-changed = Cloud 账号已变化。选择模型前请重新打开对话。
model-defaults-cloud-no-model-change = 此云端对话不支持在 TUI 中更改模型。
model-defaults-needs-settings-update = 选择模型需要支持 thread/settings/update 的 app-server。
model-defaults-saved-but-overridden = 已保存 { $setting }，但更高优先级的配置层覆盖了已保存的值。
model-defaults-for-this-session-only = {" "}（仅限本次会话）

## tui/src/app/new_session.rs
new-session-only-cloud-cancel = 只能取消 Cloud 切换

## tui/src/app/resume_preparation.rs
resume-preparation-still-archiving = 此任务仍在归档中。
resume-preparation-failed-load-history = 加载对话历史失败：{ $read_err }
resume-preparation-failed-resume = 从 { $path_display } 恢复会话失败：{ $err }

## tui/src/app/recap.rs
recap-failure = 无法生成回顾。请再试一次。
recap-cloud-unavailable = Cloud 对话不支持对话回顾。
recap-in-progress = 正在生成回顾。
recap-empty-history = 没有可回顾的对话历史。

## tui/src/app_server_session/side_chat_history.rs
side-chat-history-outside-attachment = 该会话有一个位于 TUI 侧边栏之外的附件。删除会话前请先移除它。

## tui/src/bottom_pane/hooks_browser_view.rs
hooks-browser-view-hooks = Hooks
hooks-browser-view-lifecycle-hooks = 来自配置和已启用插件的生命周期 hook。
hooks-browser-view-event-hooks = { $event } hook
hooks-browser-view-turn-hooks-on-off = 开启或关闭 hook。你的更改会自动保存。
hooks-browser-view-event = 事件
hooks-browser-view-installed = 已安装
hooks-browser-view-active = 已启用
hooks-browser-view-review = 待审查
hooks-browser-view-description = 说明
hooks-browser-view-issues = 问题
hooks-browser-view-unnamed-hook = 未命名 hook
hooks-browser-view-modified-suffix = {" "}· 已修改
hooks-browser-view-new-suffix = {" "}· 新增
hooks-browser-view-no-hooks = 此事件未安装 hook。
hooks-browser-view-matcher = 匹配器
hooks-browser-view-source = 来源
hooks-browser-view-command = 命令
hooks-browser-view-mode = 模式
hooks-browser-view-async = 异步
hooks-browser-view-sync = 同步
hooks-browser-view-mcp-server = MCP 服务器
hooks-browser-view-mcp-tool = MCP 工具
hooks-browser-view-handler = 处理器
hooks-browser-view-prompt = 提示词
hooks-browser-view-agent = 代理
hooks-browser-view-timeout = 超时
hooks-browser-view-unlimited = 不限
hooks-browser-view-limit-tokens = 上限：约 { $limit } 个 token
hooks-browser-view-context = 上下文
hooks-browser-view-trust = 信任
hooks-browser-view-trust-all = {" "}全部信任 ·{" "}
hooks-browser-view-review-hint = {" "}审查 ·{" "}
hooks-browser-view-details = {" "}详情 ·{" "}
hooks-browser-view-close = {" "}关闭
hooks-browser-view-managed-always-on = 受管（始终开启） ·{" "}
hooks-browser-view-trust-hint = {" "}信任 ·{" "}
hooks-browser-view-toggle = {" "}切换 ·{" "}
hooks-browser-view-back = {" "}返回
hooks-browser-view-trust-managed = 受管
hooks-browser-view-trust-trusted = 已信任
hooks-browser-view-trust-untrusted = 新 hook - 需要审查
hooks-browser-view-trust-modified = 自上次信任后已修改 - 需要审查
hooks-browser-view-desc-before-tool = 工具执行之前
hooks-browser-view-desc-permission-requested = 请求权限时
hooks-browser-view-desc-after-tool = 工具执行之后
hooks-browser-view-desc-before-compaction = 上下文压缩之前
hooks-browser-view-desc-after-compaction = 上下文压缩之后
hooks-browser-view-desc-before-model-request = 模型请求之前
hooks-browser-view-desc-after-model-request = 模型请求之后（包括失败）
hooks-browser-view-desc-session-start = 新会话开始时
hooks-browser-view-desc-session-end = 会话结束之前
hooks-browser-view-desc-prompt-submit = 用户提交提示词时
hooks-browser-view-desc-subagent-start = 创建子代理时
hooks-browser-view-desc-subagent-stop = 子代理结束其轮次之前
hooks-browser-view-desc-stop = Codex 结束其轮次之前
hooks-browser-view-desc-interrupt = 被中断的轮次终止之前
hooks-browser-view-environment = 环境 - { $id }
hooks-browser-view-plugin-id = 插件 - { $plugin_id }
hooks-browser-view-plugin = 插件
hooks-browser-view-src-admin-config = 管理员配置
hooks-browser-view-src-user-config = 用户配置
hooks-browser-view-src-project-config = 项目配置
hooks-browser-view-src-session-flags = 会话参数
hooks-browser-view-src-cloud-managed-config = 云端托管配置
hooks-browser-view-src-unknown-source = 未知来源
hooks-browser-view-needs-review = 有 { $count } 个 hook 需要审查后才能运行。

## tui/src/bottom_pane/textarea.rs
textarea-vim-normal = Vim：普通
textarea-vim-insert = Vim：插入
textarea-vim-replace = Vim：替换

## tui/src/clipboard_copy.rs
clipboard-copy-copied-to-clipboard = 已将{ $label }复制到剪贴板
clipboard-copy-unconfirmed = 复制未确认；/export 可保存对话
clipboard-copy-copying = 正在复制{ $label }…
clipboard-copy-already-in-progress = 复制已在进行中
clipboard-copy-nothing-to-copy = 没有可复制的内容：所选内容为空

## tui/src/debug_config.rs
debug-config-session-runtime = 会话运行时：
debug-config-layer-stack = 配置层堆栈（优先级从低到高）：
debug-config-none-indented = {"  "}<无>
debug-config-disabled = 已禁用
debug-config-enabled = 已启用
debug-config-reason = {"     "}原因：{ $reason }
debug-config-requirements = 要求：
debug-config-none-bullet = {"     "}- <无>
debug-config-mdm-value = MDM 值
debug-config-enterprise-value = 企业托管配置值
debug-config-layer-value = 配置层的值
debug-config-requirement-line = {"  "}- { $name }：{ $value }（来源：{ $source }）
debug-config-unspecified = <未指定>
debug-config-empty = <空>
debug-config-unset = <未设置>

## tui/src/diff_render.rs
diff-render-added = 已添加
diff-render-deleted = 已删除
diff-render-edited = 已编辑
diff-render-file-count = {" "}{ $count } 个文件{" "}

## tui/src/external_agent_config_migration/render.rs
external-agent-migration-bring-over = 从其他编码代理导入受支持的配置。
external-agent-migration-may-add-files = Codex 可能会向当前项目文件夹添加文件。
external-agent-migration-not-changed = 你现有的配置不会被更改。
external-agent-migration-choose-items = 选择要导入的项目。
external-agent-migration-import-setup = 导入配置
external-agent-migration-choose-what = 选择要导入的内容
external-agent-migration-selected-of = 已选择 { $selected } / { $total } 项。
external-agent-migration-use = 使用{" "}
external-agent-migration-to-move = {" "}移动，
external-agent-migration-to-select = {" "}选择，
external-agent-migration-to-customize = {" "}自定义
external-agent-migration-press = 按{" "}
external-agent-migration-to-continue = {" "}继续，
external-agent-migration-to-go-back = {" "}返回
external-agent-migration-to-toggle = {" "}切换，

## tui/src/onboarding/gateway.rs
gateway-use-custom-provider = 使用自定义提供方或网关

## tui/src/pager_overlay.rs
pager-overlay-to-scroll = 滚动
pager-overlay-to-page = 翻页
pager-overlay-to-jump = 跳转
pager-overlay-close = 关闭

## tui/src/pets/ambient.rs
ambient-running = 运行中
ambient-needs-input = 需要输入
ambient-ready = 就绪
ambient-blocked = 已阻塞
ambient-thinking = 思考中

## tui/src/pets/image_protocol.rs
image-protocol-tmux = 此 tmux 面板中无法使用宠物。请全屏运行 Codex 并设置 allow-passthrough all，或在 tmux 之外运行 Codex。
image-protocol-zellij = Zellij 中已禁用宠物。Zellij 中的终端图像无法可靠地保持在各自的面板内。请在 Zellij 之外运行 Codex 以使用宠物。
image-protocol-iterm2-too-old = 宠物需要 iTerm2 3.6 或更高版本。请升级 iTerm2 以使用终端宠物。
image-protocol-terminal = 此终端无法使用宠物。终端宠物需要图像支持，而当前终端环境未提供受支持的图像协议。请尝试使用支持 Kitty graphics 或 Sixel 的终端，或在 tmux 之外运行 Codex。

## tui/src/transcript_view/composer_gap.rs
composer-gap-copied-chars-host = 已将 { $characters } 个字符复制到主机剪贴板
composer-gap-copied-chars = 已复制 { $characters } 个字符
composer-gap-copied = 已复制
composer-gap-copying-chars = 正在复制 { $characters } 个字符…
composer-gap-copying = 正在复制…
composer-gap-already-in-progress = 复制已在进行中
composer-gap-in-progress = 复制进行中
composer-gap-busy = 复制繁忙
composer-gap-sent-to-terminal = 已发送到终端 · 粘贴以验证
composer-gap-sent-verify = 已发送 · 请粘贴验证
composer-gap-unconfirmed = 复制未确认
composer-gap-failed-try-again = 复制失败 · 请重试
composer-gap-failed = 复制失败

## tui/src/ui_consts.rs
ui-consts-to-view-transcript = { $key } 查看对话记录


## tui/src/additional_dirs.rs

additional-dirs-ignored-warning = 已忽略 --add-dir（{ $joined_paths }），因为当前生效的权限不允许额外的可写根目录。请切换到 workspace-write 或 danger-full-access 以允许它们。

## tui/src/app/provider/editor.rs

editor-edit-provider = 编辑 { $name }
editor-example-team-gateway = 团队网关
editor-save-focused = › 保存
editor-save = {" "}{" "}保存
editor-hint-env-var-name = 请在 app-server 上输入环境变量名。
editor-hint-auth-managed = 身份验证由 app-server 管理。
editor-required-note = {"*"} 必填
editor-complete-suggestion = {" "}补全 { $suggestion }
editor-key-move = {" "}移动 ·{" "}
editor-key-next-save = {" "}下一项/保存 ·{" "}
editor-key-normal-mode = {" "}普通模式
editor-key-back = {" "}返回

## tui/src/app/session_picker.rs

session-picker-open-failed = 打开会话选择器失败：{ $err }
session-picker-unable-to-resume = 无法恢复会话
session-picker-return-to-center = 返回指挥中心

## tui/src/app/side_focus.rs

side-focus-chat-still-closing = 侧边会话仍在关闭。请稍后再试。

## tui/src/app/startup_prompts.rs

startup-prompts-skills-skipped = 已跳过加载 { $error_count } 个 skill，因为其 SKILL.md 文件无效。
startup-prompts-project-config-disabled = 在项目被信任之前，以下文件夹中的项目本地配置、hook 和 exec 策略均被禁用，但 skill 仍会加载。{"\u000A"}
startup-prompts-no-longer-offered = 在 Codex 中已不再提供
startup-prompts-no-longer-available = 已不再可用
startup-prompts-model-migration = { $current_name } { $availability }{"\u000A"}{"\u000A"}Codex 现在改用 { $target_name } 代替 { $current_name }。请切换到 { $target_name } 以继续。{"\u000A"}

## tui/src/app/thread_attachment.rs

thread-attachment-not-yet-available = 代理会话 { $thread_id } 尚不能用于回放或实时挂接。

## tui/src/app/thread_routing.rs

thread-routing-main-default = 主会话 [默认]
thread-routing-agent = 代理
thread-routing-already-viewing = 正在查看 { $label }。
thread-routing-no-active-thread = 没有可用的活动会话。
thread-routing-read-only = 此对话为只读或不可用，未发送任何操作。
thread-routing-not-available-yet = 会话 { $thread_id } 在 TUI 中暂不可用。
thread-routing-interrupt-failed = 无法中断轮次：{ $error }
thread-routing-rename-failed = 无法重命名会话：{ $error }
thread-routing-shell-unavailable-cloud = 云端对话中不可使用以 ! 开头的 shell 命令。
thread-routing-resolve-request-failed = 无法解析会话 { $thread_id } 的 app-server 请求：{ $err }
thread-routing-agent-closed-switched = 代理会话 { $closed_thread_id } 已关闭，已切回主会话。
thread-routing-agent-closed-switch-failed = 代理会话 { $closed_thread_id } 已关闭，切回主会话 { $primary_thread_id } 失败。

## tui/src/bottom_pane/voice_strip.rs

voice-strip-voice-prefix = 语音{" "}
voice-strip-voice = 语音
voice-strip-unmute = 取消静音
voice-strip-mute = 静音{" "}{" "}
voice-strip-mic = {" "}{" "}麦克风{" "}
voice-strip-codex = {" "}{" "}Codex{" "}

## tui/src/local_settings.rs

local-settings-invalid-audio = 机器音频设置无效：{ $error }

## tui/src/markdown_render/mermaid.rs

mermaid-unsupported = 此 Mermaid 图表使用了终端渲染器不支持的功能。
mermaid-too-wide = 此 Mermaid 图表放不进当前终端宽度。
mermaid-limit = 此 Mermaid 图表超出了终端渲染器的尺寸限制。

## tui/src/pets/picker.rs

picker-select-pet = 选择宠物
picker-choose-pet = 选择要在终端中唤醒的宠物。
picker-filter-pets-placeholder = 输入以筛选宠物...
picker-disable-pets = 禁用终端宠物

## tui/src/sidebar_chooser.rs

sidebar-chooser-resize-hint-short = ←/→ 调整大小 · Enter/Esc 完成
sidebar-chooser-hint-retry = 重试
sidebar-chooser-hint-back = 返回
sidebar-chooser-hint-scroll = 滚动
sidebar-chooser-hint-open = 打开
sidebar-chooser-hint-cancel-opening = 取消打开
sidebar-chooser-hint-path = 路径
sidebar-chooser-hint-move = 移动
sidebar-chooser-hint-hide-pane = 隐藏面板
sidebar-chooser-side-n = 侧边 { $number }
sidebar-chooser-side-chat = 侧边聊天

## tui/src/startup_draft.rs

startup-draft-did-not-open = 新对话未能打开
startup-draft-kept = 已保留草稿
startup-draft-loading = 加载中

## tui/src/status/card.rs

card-label-model = 模型
card-label-directory = 目录
card-label-permissions = 权限
card-label-model-provider = 模型提供方
card-label-account = 账户
card-label-chatgpt-workspace = ChatGPT 工作区
card-label-thread-name = 会话名称
card-label-session = 会话
card-label-forked-from = 分叉自
card-label-collaboration-mode = 协作模式
card-label-token-usage = Token 用量
card-label-context-window = 上下文窗口
card-label-limits = 限额
card-label-warning = 警告
card-label-server = 服务器
card-limits-not-available = 此账户不可用
card-limits-stale-run-status = 限额可能已过期，请稍后再次运行 /status。
card-limits-stale-new-turn = 限额可能已过期，开始新一轮对话即可刷新。
card-limits-refresh-requested = 已请求刷新；请稍后再次运行 /status。
card-limits-no-data-yet = 暂无数据
card-token-total = {" "}合计{" "}
card-token-input = {" "}输入
card-token-plus = {" "}+{" "}
card-token-output = {" "}输出
card-context-used-of = {" "}已用 /{" "}
card-context-percent-left = 剩余 { $percent }%
card-resets-at = （重置于 { $resets_at }）
card-permissions-read-only-network = 只读（允许联网）
card-permissions-read-only = 只读
card-permissions-workspace = 工作区{ $suffix }（{ $approval }）
card-permissions-workspace-network = 工作区（允许联网）{ $suffix }（{ $approval }）
card-permissions-full-access = 完全访问
card-permissions-no-sandbox = 无沙箱（{ $approval }）
card-permissions-profile = 配置 { $id }（{ $sandbox }，{ $approval }）
card-permissions-read-only-approval = 只读（{ $approval }）
card-permissions-custom = 自定义（{ $sandbox }，{ $approval }）
card-approval-approve-for-me = 自动审批
card-approval-ask = 请求审批
card-account-api-key = 已配置 API key（运行 codex login 以使用 ChatGPT）
card-local-background-server = 本地后台服务
card-visit = 访问{" "}
card-for-up-to-date = {" "}获取最新的
card-rate-limits-info = 速率限制和额度信息

## tui/src/transcript_view/layout.rs

layout-show-less = − 收起
layout-hide-details = − 隐藏详情
layout-show-details = + 显示详情
layout-more-lines = + { $count } 行
layout-to-expand-hint = {" "}（{ $key } 展开）

## tui/src/thread_transcript/tools.rs

tools-in-progress-at-last-update = 上次更新时仍在进行
tools-completed = 已完成
tools-failed = 失败
tools-declined = 已拒绝
tools-terminal-interaction = 终端交互 · { $status }
tools-exit-code = {" "}· 退出码 { $code }
tools-completed-result-unavailable = 已完成 · 结果不可用
tools-failed-result-unavailable = 失败 · 结果不可用
tools-mcp-tool = MCP 工具：{ $server }/{ $tool } · { $status }
tools-interrupted = 已中断
tools-agent-tool = 代理工具：{ $tool } · { $status }
tools-declined-command = 已拒绝 · { $command }

## tui/src/update_prompt.rs

update-prompt-update-available = 有可用更新
update-prompt-release-notes = 发布说明：{" "}
update-prompt-update-now = 立即更新（运行 `{ $update_command }`）
update-prompt-skip = 跳过
update-prompt-skip-until-next = 跳过直到下一个版本
update-prompt-continue = {" "}继续 ·{" "}
update-prompt-skip-hint = {" "}跳过

## app/experimental_features.rs
app-experimental-features-label-subagents = 子代理
app-experimental-features-label-memories = 记忆
app-experimental-features-saved-new-threads = { $label } 设置已保存到服务器，供新会话使用。此会话未变。项目或任务设置可能会覆盖它。
app-experimental-features-saved-overridden = { $label } 设置已保存，但被覆盖：{ $detail }
app-experimental-features-save-failed = 保存 { $label } 设置失败：{ $detail }
app-experimental-features-save-in-progress = 实验性功能的保存仍在进行中。请等它完成后重试。

## app/managed_worktree_creation.rs
managed-worktree-daemon-cannot-check = 本地 Codex 服务无法检查后台终端。请运行 `ccn app-server daemon update --from-cli`，然后重启 Codex。
managed-worktree-background-blocks-cd = 活动的后台终端会阻止 /cd。
managed-worktree-untrusted-source = 无法从明确不受信任的源创建 worktree。
managed-worktree-local-only = 托管 worktree 仅支持本地会话。
managed-worktree-needs-idle = 创建 worktree 要求主会话空闲且没有排队的输入。
managed-worktree-already-creating = 正在创建 worktree。
managed-worktree-mcp-loading = MCP 清单仍在加载。
managed-worktree-agent-running = 无法更改：另一个代理正在运行。
managed-worktree-history-not-saved = 对话历史不会被保存。
managed-worktree-source-changed = 源会话已变化或不可用，无法继续到新的 worktree。
managed-worktree-source-config-changed = 源配置已变化，无法继续到新的 worktree。
managed-worktree-not-trusted = 新的 worktree 不受信任；请在其中运行 Codex。
managed-worktree-task-failed = worktree 创建任务失败: { $error }
managed-worktree-load-config-failed = 无法加载新 worktree 的配置：{ $error }
managed-worktree-retained-checkout = { $reason } 检出目录保留在 { $path }；如果不再需要，请在源仓库中用 `git worktree remove <checkout-path>` 将其删除。

## app/pets.rs
app-pets-disable-failed = 禁用宠物失败：{ $err }
app-pets-save-failed = 保存宠物选择失败：{ $err }
app-pets-load-failed = 加载宠物失败：{ $err }

## app/provider/write.rs
provider-write-already-saving = 提供方更改正在保存。
provider-write-saving = 正在保存模型提供方…
provider-write-offline = 保存提供方失败：请重新连接 app-server 后重试。
provider-write-saved-config = 已保存提供方配置。
provider-write-update-server = 请更新已连接的 app-server 以更改默认模型提供方。
provider-write-save-failed = 保存提供方失败：{ $error }
provider-write-confirm-failed = 无法确认 { $name } 是否已保存：{ $error }
provider-write-now-use = 新对话现在使用 { $name }。
provider-write-saved-name = 已保存 { $name }。{ $reason }

## app/startup_warnings.rs
startup-warnings-nfs-migration = NFS 上的部分 Codex SQLite 文件权限不是 0600。迁移会保留这些权限，并需要你确认。运行 `codex migrate-nfs-db` 之前，请在所有使用这些数据库的主机上停止 Codex。

## app/user_turn_submission.rs
user-turn-submission-label = 提交
user-turn-submission-label-cloud = 云端提交
user-turn-submission-unconfirmed = 无法确认{ $label }：{ $error }

## app/realtime_settings.rs
realtime-settings-save-failed = 保存音频设置失败：{ $error }
realtime-settings-input-overridden = 输入设备已保存，但输入通道被另一配置层覆盖。请先更新该覆盖设置，再开始语音。
realtime-settings-saved = 音频设置已保存，将应用于你的下一次语音对话。
realtime-settings-overridden = 音频设置已保存，但被另一配置层覆盖。
realtime-settings-read-failed = 音频设置已保存，但无法读取生效的设置：{ $error }
realtime-settings-list-devices-failed = 无法列出音频设备。请检查音频设备和语音包。
realtime-settings-list-voices-failed = 列出语音失败：{ $error }
realtime-settings-read-voice-failed = 读取语音设置失败：{ $error }
realtime-settings-voice-overridden-no-read = 已保存的语音被覆盖，但此服务器无法读取生效的设置
realtime-settings-voice-not-applied = 语音偏好已保存但未生效：{ $detail }
realtime-settings-voice-read-failed = 语音偏好已保存，但无法读取生效的设置：{ $error }
realtime-settings-voice-save-failed = 保存语音失败：{ $error }

## bottom_pane/pending_thread_approvals.rs
pending-thread-approvals-needed = { $thread } 中需要审批
pending-thread-approvals-review = {" "}查看请求

## chatwidget/permissions_menu.rs
permissions-menu-missing-preset-read-only = 内部错误：缺少 'read-only' 审批预设。
permissions-menu-missing-preset-auto = 内部错误：缺少 'auto' 审批预设。
permissions-menu-missing-preset-full-access = 内部错误：缺少 'full-access' 审批预设。

## experimental_features.rs
experimental-features-err-request-failed = 实验性功能请求失败
experimental-features-err-page-exceeds = 实验性功能页面超出请求的上限
experimental-features-err-cursor-repeated = 实验性功能分页重复了 cursor
experimental-features-err-exceeded-pages = 实验性功能发现超过 10 页
experimental-features-err-discovery-timed-out = 实验性功能发现超时
experimental-features-err-discovery-interrupted-short = 功能发现被中断
experimental-features-err-not-advertised = 服务器未声明实验性功能 `{ $name }`
experimental-features-err-save-timed-out = 保存实验性功能超时；写入可能仍会完成。请重新打开 /experimental 进行检查。
experimental-features-err-save-failed = 无法保存实验性功能。请重新打开 /experimental，检查已配置的值后再重试。
experimental-features-err-readback-interrupted = 功能已保存，但回读被中断
experimental-features-err-refresh-failed = 功能已保存，但无法刷新已配置的值：{ $error }
experimental-features-err-values-differ = 更改已保存，但已配置的值与你的选择不同。优先级更高的设置可能会覆盖它们。

## external_agent_config_migration/model.rs
external-agent-migration-group-tools = 工具与设置
external-agent-migration-group-tools-desc = 设置、指令、集成、代理、命令和 skill
external-agent-migration-group-current-project = 当前项目
external-agent-migration-group-projects = 项目（{ $project_count }）
external-agent-migration-group-projects-desc = 将 Codex 文件添加到现有项目文件旁
external-agent-migration-group-sessions = 聊天会话（{ $session_count }）
external-agent-migration-group-sessions-desc = 最近 30 天的聊天
external-agent-migration-type-instructions = 指令
external-agent-migration-type-settings = 设置
external-agent-migration-type-skills = skill
external-agent-migration-type-plugins = 插件
external-agent-migration-type-mcp-servers = MCP 服务器
external-agent-migration-type-agents = 代理
external-agent-migration-type-hooks = hook
external-agent-migration-type-commands = 斜杠命令
external-agent-migration-type-memory = 记忆
external-agent-migration-item-sessions = 最近的聊天会话
external-agent-migration-type-sessions = 聊天会话
external-agent-migration-summary-entry = { $label } { $count }
external-agent-migration-count-skill = { $count } 个 skill
external-agent-migration-count-mcp-server = { $count } 个 MCP 服务器
external-agent-migration-count-agent = { $count } 个代理
external-agent-migration-count-hook = { $count } 个 hook
external-agent-migration-count-command = { $count } 个斜杠命令
external-agent-migration-count-session = { $count } 个对话会话
external-agent-migration-count-memory = { $count } 条记忆
external-agent-migration-count-with-names = { $summary }：{ $names }

## goal_display.rs
goal-display-status-active = 进行中
goal-display-status-paused = 已暂停
goal-display-status-stalled = 已停滞
goal-display-status-usage-limited = 用量受限
goal-display-status-budget-limited = 受预算限制
goal-display-status-complete = 已完成
goal-display-objective = 目标：{ $objective }
goal-display-time = 时间：{ $time }。
goal-display-tokens = Token：{ $used }/{ $budget }。

## transcript_view/search_footer.rs
search-footer-find-prefix = 查找：
search-footer-retry = { $previous } 重试
search-footer-history-unavailable = 历史记录不可用 · { $retry_hint }
search-footer-next-hint = enter 接受 · { $previous } 更早 · { $next } 更新
search-footer-exhausted-hint = 没有更多匹配 · { $next_hint }
search-footer-reading-nav = ctrl+p 更早 · ctrl+n 更新
search-footer-ctrl-p-older = ctrl+p 更早
search-footer-enter-accept = enter 接受
search-footer-type-to-find = 输入以查找
search-footer-searching = 正在搜索…
search-footer-loading = 加载中…
search-footer-searching-earlier = 正在搜索更早的历史记录…
search-footer-no-more-reading = 没有更多匹配 · ctrl+p 更早 · ctrl+n 更新
search-footer-no-matches = 没有匹配项
search-footer-query-limited = {" "}· 查询限制为 4 KiB
search-footer-reading-status = 查找 · { $status } · esc 返回最新
search-footer-reading-compact = 查找 · { $compact } · esc 返回最新
search-footer-esc-latest = esc 返回最新
search-footer-full-transcript = { $status } · 完整对话记录 · esc 取消{ $limit }
search-footer-status-cancel = { $status } · esc 取消{ $limit }
search-footer-compact-cancel = { $compact } · esc 取消
search-footer-esc-cancel = esc 取消
search-footer-compact-esc = { $compact } · esc

## sidebar_image.rs
sidebar-image-unsupported-format = 不支持的图像格式
sidebar-image-tabs = 标签页
sidebar-image-close = 关闭
sidebar-image-preview-unavailable = 此终端中无法预览图像

## pets/preview.rs
pets-preview-loading = 正在加载预览...
pets-preview-disabled = 终端宠物已禁用
pets-preview-no-pet = 不会显示宠物。
pets-preview-unavailable = 预览不可用

## resume_picker.rs
resume-picker-different-backend = 此会话属于另一个后端。请重新打开会话选择器。
resume-picker-thread = 会话 { $id }
resume-picker-title-resume = 恢复之前的会话
resume-picker-title-fork = 分叉之前的会话
resume-picker-action-resume = 恢复
resume-picker-action-fork = 分叉
resume-picker-sort-created = 创建时间
resume-picker-sort-updated = 更新时间
resume-picker-no-transcript = 此会话没有可用的对话记录
resume-picker-metadata-failed-path = 无法从 { $path } 读取会话元数据
resume-picker-metadata-failed = 无法从所选会话读取会话元数据
resume-picker-backend-load-failed = 无法加载一个后端。请重新打开选择器重试。
resume-picker-load-more-failed = 无法加载更多会话
resume-picker-preview-load-failed = 无法加载对话记录预览
resume-picker-save-view-failed = 保存视图模式失败：{ $err }
resume-picker-no-message-yet = （暂无消息）
resume-picker-type-to-search = 输入以搜索
resume-picker-search = 搜索：{ $query }
resume-picker-status-compact = 状态：
resume-picker-status = 状态：
resume-picker-active = 进行中
resume-picker-archived = 已归档
resume-picker-sort-compact = 排序：
resume-picker-sort = 排序：
resume-picker-filter-compact = 筛选：
resume-picker-filter = 筛选：
resume-picker-filter-cwd = 目录
resume-picker-filter-repo = 仓库
resume-picker-filter-all = 全部
resume-picker-hint-loading = 加载中
resume-picker-hint-transcript = 对话记录
resume-picker-hint-quit = 退出
resume-picker-hint-restore = 还原
resume-picker-hint-start-new = 新建
resume-picker-hint-new = 新建
resume-picker-hint-exit = 退出
resume-picker-hint-clear-search = 清除搜索
resume-picker-hint-clear = 清除
resume-picker-hint-dense-view = 紧凑视图
resume-picker-hint-comfortable-view = 舒适视图
resume-picker-hint-dense = 紧凑
resume-picker-hint-comfy = 舒适
resume-picker-hint-archive = 归档
resume-picker-hint-focus-sort-filter = 聚焦排序/筛选
resume-picker-hint-focus = 聚焦
resume-picker-hint-change-option = 切换选项
resume-picker-hint-option = 选项
resume-picker-hint-preview = 预览
resume-picker-hint-expand = 展开
resume-picker-hint-exp = 展开
resume-picker-hint-browse = 浏览
resume-picker-loading-transcript = 正在加载对话记录…
resume-picker-more-above = ↑ 更多
resume-picker-loading-more = ↓ 正在加载更多
resume-picker-more-below = ↓ 更多
resume-picker-loading-older = 正在加载更早的会话…
resume-picker-no-branch = 无分支
resume-picker-no-cwd = 无目录
resume-picker-loading-recent-transcript = 正在加载最近的对话记录...
resume-picker-label-session = 会话：
resume-picker-label-created = 创建：
resume-picker-label-updated = 更新：
resume-picker-label-directory = 目录：
resume-picker-label-branch = 分支：
resume-picker-label-conversation = 对话：
resume-picker-no-preview = 没有可用的对话记录预览
resume-picker-now = 刚刚
resume-picker-ago-seconds-short = { $seconds } 秒前
resume-picker-ago-minutes-short = { $minutes } 分钟前
resume-picker-ago-hours-short = { $hours } 小时前
resume-picker-ago-days-short = { $days } 天前
resume-picker-ago-second = { $count } 秒前
resume-picker-ago-minute = { $count } 分钟前
resume-picker-ago-hour = { $count } 小时前
resume-picker-ago-day = { $count } 天前
resume-picker-searching = 正在搜索…
resume-picker-scan-capped = 搜索仅扫描了前 { $count } 个会话；可能还有更多
resume-picker-no-results = 没有符合搜索条件的结果
resume-picker-loading-sessions = 正在加载会话…
resume-picker-no-sessions = 还没有会话
resume-picker-time-separator = { $relative } · { $timestamp }

## startup_hooks_review.rs
startup-hooks-review-trust-failed = 信任 hook 失败：{ $error }
startup-hooks-review-one-changed = 有 1 个 hook 是新增或已更改的。
startup-hooks-review-many-changed = 有 { $count } 个 hook 是新增或已更改的。
startup-hooks-review-title = hook 需要审查
startup-hooks-review-warning = 信任 hook 后，它们可以在沙箱之外运行。
startup-hooks-review-trusting = 正在信任 hook...
startup-hooks-review-confirm = 确认
startup-hooks-review-skip = 跳过
startup-hooks-review-review = 审查 hook
startup-hooks-review-trust-all = 全部信任并继续
startup-hooks-review-continue-without = 不信任并继续（hook 不会运行）

## app/input.rs

input-no-editor = 无法打开外部编辑器：请在启动 Codex 前设置 $VISUAL 或 $EDITOR。
input-open-editor-failed = 打开编辑器失败：{ $err }
input-redraw-failed = 重绘对话记录失败：{ $err }
input-unarchive-failed = 取消归档会话失败：{ $error }
input-task-running-title = 任务仍在运行
input-task-running-subtitle = 选择如何处理当前任务。
input-cancel-task = 取消任务
input-cancel-task-desc = 停止当前任务并留在 Codex
input-run-background = 在后台运行
input-run-background-desc = 退出 Codex，任务继续运行
input-exit = 退出
input-exit-desc = 停止当前任务并退出 Codex
input-switch-side-failed = 切换侧边对话失败：{ $err }
input-clear-ui-failed = 清除终端界面失败：{ $err }

## app/platform_actions.rs

platform-sandbox-mixed = 本地和远程执行器同时配置时，无法设置 Windows 沙箱。
platform-sandbox-config-unreadable = 无法从 app-server 读取 Windows 沙箱的配置和要求。
platform-sandbox-still-running = Windows 沙箱设置仍在进行。请重启 Codex 后重试。
platform-sandbox-option-disallowed = 该 Windows 沙箱选项被要求禁止。
platform-sandbox-timed-out = Windows 沙箱设置请求超时。正在等待完成；如果长时间未完成，请重启 Codex。
platform-sandbox-response-lost = Windows 沙箱设置的响应丢失。正在等待完成或重新连接。
platform-sandbox-update-server = 请更新本地 app-server 以设置 Windows 沙箱。
platform-sandbox-failed = Windows 沙箱设置失败。
platform-sandbox-not-started = Windows 沙箱设置未能启动。

## app/reconnect.rs

reconnect-reopen-cloud = 请重新打开 Codex 以重新连接 Cloud 后端。
reconnect-in-process = 进程内会话没有可恢复的连接
reconnect-initial-thread-unknown = 初始会话可能已创建，但未收到其 ID。未进行重试。你的提示可以编辑；重新启动前请检查你的任务。
reconnect-stage-connecting = 正在连接服务器
reconnect-stage-loading-settings = 正在加载会话设置
reconnect-stage-resuming = 正在恢复对话
reconnect-timed-out = 自动重连在{ $stage }时超时。
reconnect-agent-list-stale = 正在重新连接，代理列表已过期
reconnect-conversation-unavailable = 此对话不可用。其缓存的对话记录和草稿仍保留在此；输入已暂停。请打开代理选择器或返回父会话以继续。
reconnect-sandbox-interrupted = Windows 沙箱设置被中断。使用代理模式前请重启 Codex。

## app/right_click_paste.rs

right-click-paste-inline = 内联模式下不可用；请使用终端的粘贴命令
right-click-paste-ssh = 通过 SSH 不可用；请使用终端的粘贴命令
right-click-paste-vscode = VS Code 负责粘贴；请使用其终端的粘贴命令
right-click-paste-wsl = Auto 无法识别此 WSL 终端；请选择 On 或使用终端的粘贴命令
right-click-paste-android = Android 上不可用

## app/provider/editor/form.rs

form-read-only = 此提供方配置为只读。
form-enter-name = 请输入提供方名称。
form-enter-base-url = 请输入基础 URL。
form-enter-base-url-for-key = 请输入基础 URL 以使用此 API key 变量。
form-enter-env-var = 请输入环境变量名，例如 TEAM_API_KEY。
form-unrecognized-bedrock = 无法识别的 Bedrock 提供方。

## app/server_version_notice.rs

server-version-notice-summary = 服务 v{ $server } { $comparison } Codex CLI v{ $client_version }{ $guidance }

## app/safety_buffering.rs

safety-buffering-wait-permissions = 分叉前请等待权限更新完成。
safety-buffering-retry-unavailable = 无法用更快的模型重试：原始轮次不可用。
safety-buffering-retry-failed = 无法用更快的模型重试：{ $err }
safety-buffering-input-too-long = 无法安全重试输入超出有界历史页的轮次。

## app/safety_recourse.rs

safety-recourse-new-chat-failed = 无法为此重写开始新对话。{ $text }

## app/session_lifecycle.rs

session-lifecycle-no-agents = 暂无可用代理。
session-lifecycle-needs-approval = { $name }（需要审批）
session-lifecycle-subagents = 子代理
session-lifecycle-side-closing = 侧边会话仍在关闭。请稍后再试。
session-lifecycle-thread-unavailable = 代理会话 { $thread_id } 已不可用。
session-lifecycle-attach-failed = 连接代理会话 { $thread_id } 失败：{ $err }
session-lifecycle-finish-sandbox = 切换会话前请先完成 Windows 沙箱设置。
session-lifecycle-already-active = 代理会话 { $thread_id } 已处于活动状态。
session-lifecycle-resume-live-failed = 代理会话 { $thread_id } 无法实时恢复。正在回放已保存的对话记录。
session-lifecycle-thread-closed = 代理会话 { $thread_id } 已关闭。正在回放已保存的对话记录。

## app/sidebar_close.rs

sidebar-close-button-close = {" "}关闭{" "}
sidebar-close-button-archive = {" "}归档{" "}
sidebar-close-button-cancel = {" "}取消{" "}
sidebar-close-resize = 调整大小以查看 · Esc 取消
sidebar-close-hint-choose = 选择
sidebar-close-hint-confirm = 确认
sidebar-close-hint-cancel = 取消
sidebar-close-loss-stop-turn = 停止正在运行的轮次
sidebar-close-loss-discard-input = 丢弃未发送的输入
sidebar-close-loss-temp-history = 丢失这个临时聊天的记录
sidebar-close-loss-remove-side = 从侧边聊天中移除
sidebar-close-side-chat-n = 侧边聊天 { $index }
sidebar-close-verb-close = 关闭
sidebar-close-verb-archive = 归档
sidebar-close-and = 、
sidebar-close-confirm = 这将{ $losses }。{ $verb }“{ $title }”？
sidebar-close-archive-failed = 无法归档侧边对话 { $target }：{ $err }
sidebar-close-interrupt-failed = 无法中断侧边对话 { $target }：{ $err }
sidebar-close-close-failed = 无法关闭侧边对话 { $target }：{ $err }

## resume_picker/archive.rs

archive-no-thread-id = 所选会话没有会话 ID。
archive-use-archive-cmd = 使用 /archive 归档当前会话并退出。
archive-session-failed = 无法归档会话：{ $error }
archive-restore-failed = 无法恢复已归档的会话：{ $error }

## resume_picker/sources.rs

sources-no-backends = 没有可用的会话后端。
sources-backend-unavailable = 会话的后端不可用。
sources-backend-disconnected = 会话后端已断开连接。

## worktree_browser.rs

worktree-browser-untitled = 未命名对话

## session_queue_commands.rs

queue-no-daemon = --no-daemon 不能与 codex queue 一起使用。排队必须找到共享服务器，以免通过单独的服务器写入。
queue-embedded-conflict = 无法通过嵌入式 app 服务器排队，因为本地 app-server 后台服务正在运行；请移除配置覆盖项，或使用 --remote
queue-resume-warning = 警告：消息已排队，但无法恢复会话 { $thread_id }：{ $err }
queue-queued = 已将消息 { $id } 排队到会话 { $thread_id }。
queue-no-active-session = 没有找到匹配 '{ $target }' 的活动会话。
queue-start-daemon = 启动本地后台服务，或用 --remote 指定服务器

## multi_agents.rs

multi-agents-main-default = 主会话 [默认]
multi-agents-agent-cap = 代理
multi-agents-agent-lower = 代理
multi-agents-resume-failed = 代理恢复失败
multi-agents-resume-agent-failed = 恢复代理失败
multi-agents-started = 已启动{" "}
multi-agents-interacted = 已交互{" "}
multi-agents-interrupted-prefix = 已中断{" "}
multi-agents-completed-prefix = 已完成{" "}
multi-agents-spawned = 已创建
multi-agents-spawn-failed = 代理创建失败
multi-agents-sent-input-to = 已发送输入至
multi-agents-waiting-for = 等待
multi-agents-waiting-for-agents = 等待代理
multi-agents-waiting-for-n = 等待 { $count } 个代理
multi-agents-finished-waiting = 等待结束
multi-agents-closed = 已关闭
multi-agents-resuming = 正在恢复
multi-agents-resumed = 已恢复
multi-agents-none-completed = 尚无代理完成
multi-agents-pending-init = 等待初始化
multi-agents-running = 运行中
multi-agents-interrupted = 已中断
multi-agents-completed = 已完成
multi-agents-agent-errored = 代理出错
multi-agents-shutdown = 已停止
multi-agents-not-found = 未找到
multi-agents-error = 错误

## transcript_view/footer.rs

transcript-footer-loading = 加载中…
transcript-footer-loading-sep = 正在加载… ·{" "}
transcript-footer-retry = 重试：
transcript-footer-new-activity = 有新活动
transcript-footer-new-dot = 新内容 ·{" "}
transcript-footer-loading-earlier = 正在加载更早的消息…
transcript-footer-new-activity-after = {" "}· 有新活动
transcript-footer-retry-after = {" "}· 重试：
transcript-footer-retry-history = 重试历史：
transcript-footer-earlier-available = 还有更早的消息。 {" "}
transcript-footer-new-activity-sep = 有新活动 ·{" "}

## app.rs

app-external-editor-hint = 保存并关闭外部编辑器以继续。


## src/app/side_pane.rs
side-pane-save-width-failed = 无法保存侧边栏宽度：{ $err }

## src/app/sidebar_document.rs
sidebar-document-connection-changed = 连接已变化，请重新打开此文件再试。
sidebar-document-limit-bytes = 源码预览上限为 1 MiB
sidebar-document-limit-lines = 源码预览上限为 { $max } 行
sidebar-document-opening = 正在打开…
sidebar-document-open-failed = 无法打开文件：{ $error }
sidebar-document-esc-close = Esc 关闭
sidebar-document-hint-read = 阅读
sidebar-document-hint-find = 查找
sidebar-document-hint-link = 链接
sidebar-document-hint-select = 选择
sidebar-document-hint-clear-highlight = 清除高亮
sidebar-document-hint-close = 关闭

## src/app_backtrack/prompt_navigation.rs
prompt-navigation-hint-details = 详情
prompt-navigation-hint-scroll = 滚动
prompt-navigation-hint-prompts = 提示词
prompt-navigation-hint-rewind = 回退
prompt-navigation-hint-back = 返回
prompt-navigation-browsing-transcript = 正在浏览对话记录
prompt-navigation-browsing = 浏览中

## src/app_server_session/fs.rs
fs-preview-needs-workspace = 文件预览需要 app-server 主机上的工作区
fs-not-utf8-text = 不是 UTF-8 文本文件
fs-not-text = 不是文本文件
fs-preview-windows-unavailable = Windows 主机上无法预览文件
fs-workspace-changed = 工作区已变更。请重新打开文件再试。
fs-preview-needs-host-path = 文件预览需要 app-server 主机上的路径

## src/app_server_session/rollout_history.rs
rollout-history-limited-notice = 最多显示最近 100 条提示词和最终回复。中间消息和工具活动不可用。

## src/app_server_session/user_message_submission.rs
user-message-cloud-account-changed = Cloud 账户已变更。发送前请重新打开会话。
user-message-failed-to-start-turn = 启动轮次失败

## src/app/background_requests.rs
background-requests-section-openai-curated = OpenAI 精选
background-requests-section-workspace = 工作区
background-requests-section-shared-with-me = 与我共享

## src/chatwidget/streaming.rs

## src/daemon_recovery.rs
daemon-recovery-running-without = 未使用共享后台服务运行：{ $reason }。
daemon-recovery-incompatible-settings = 后台服务的功能设置不兼容
daemon-recovery-cannot-use = 无法使用后台服务
daemon-recovery-settings-persist = 这些设置会保留下来，并可能停用其他客户端的功能。重启可能会中断正在进行或排队中的工作。
daemon-recovery-restart-will-use = 重启后将使用以下共享功能设置：
daemon-recovery-and-more = {" "}{" "}以及另外 { $count } 项
daemon-recovery-run-without = 本次不使用后台服务运行
daemon-recovery-restart-recommended = 使用这些设置重启（推荐）
daemon-recovery-restart = 使用这些设置重启
daemon-recovery-not-managed = 此服务器不由 Codex 管理。
daemon-recovery-cannot-resolve = 重启无法解决此兼容性检查。
daemon-recovery-cancel = 取消

## src/history_cell/patches.rs
patches-edited-file = 已编辑 { $name }
patches-edited-files = 已编辑多个文件
patches-failed-to-apply = 应用补丁失败
patches-error-details-unavailable = （无法获取错误详情）
patches-viewed-image = 已查看图片 
patches-image-generation-failed = 图片生成失败
patches-generated-image-heading = 已生成图片：
patches-saved-to = 保存到：
patches-generated-image = 已生成图片

## src/permission_discovery.rs
permission-discovery-not-available = 此服务器不可用
permission-discovery-disabled-by-requirements = 被要求配置禁用
permission-discovery-cloud-server-selected = Cloud 会话使用由服务器选定的权限。
permission-discovery-duplicate-profiles = 服务器返回了重复的权限配置。
permission-discovery-pagination-limit = 权限发现超出了分页上限。请再次尝试 /permissions。
permission-discovery-timed-out = 权限发现超时。请再次尝试 /permissions。
permission-discovery-unsupported = 此服务器不支持权限发现。请升级 Codex 服务器以使用此菜单。
permission-discovery-load-failed = 加载权限失败：{ $error }

## src/bottom_pane/selection_popup_common.rs
selection-popup-disabled-suffix = {" "}(已禁用)

## src/public_widgets/composer_input.rs
composer-input-placeholder = 撰写新任务

## src/unarchive_prompt.rs
unarchive-prompt-title = 此对话已归档
unarchive-prompt-unarchive-and-resume = 取消归档并恢复
unarchive-prompt-unarchive-and-fork = 取消归档并分叉
unarchive-prompt-cancel = 取消
unarchive-prompt-hint-continue =  继续 · 
unarchive-prompt-hint-cancel =  取消

## src/session_archive_commands.rs
session-archive-archived-named = 已归档会话 { $name }（{ $id }）。
session-archive-archived = 已归档会话 { $id }。
session-archive-deleted-named = 已删除会话 { $name }（{ $id }）。
session-archive-deleted = 已删除会话 { $id }。
session-archive-unarchived-named = 已取消归档会话 { $name }（{ $id }）。
session-archive-unarchived = 已取消归档会话 { $id }。
session-archive-delete-cancelled = 已取消删除。
session-archive-not-found-active-or-archived = 未找到与 '{ $target }' 匹配的活动或已归档会话。
session-archive-not-found-active = 未找到与 '{ $target }' 匹配的活动会话。
session-archive-not-found-archived = 未找到与 '{ $target }' 匹配的已归档会话。
session-archive-confirm-needs-terminal = 没有交互式终端，无法确认删除会话；请使用 --force 和会话 UUID 重新运行
session-archive-confirm-delete-named = 永久删除会话 '{ $name }'（{ $id }）？
session-archive-confirm-delete = 永久删除会话 { $id }？
session-archive-cannot-undo = 此操作无法撤销。子代理会话也将被删除。
session-archive-continue-prompt = 继续？[y/N]: 

## src/app_server_session.rs
app-server-session-no-models = 没有可用的模型。请显式设置 `model`，或检查模型目录配置。
app-server-session-cloud-permissions = Cloud 会话目前使用由服务器选定的权限。
app-server-session-side-context-not-saved = 侧边会话的上下文未保存
app-server-session-side-prepare-failed = 准备侧边会话 { $child } 失败：{ $message }
app-server-session-cloud-fork-unsupported = TUI 暂不支持分叉 Cloud 会话。
app-server-session-title-saving = 正在保存自动标题。请稍后再重命名会话。

## src/app/provider/fields.rs
provider-field-name = 名称
provider-field-base-url = 基础 URL
provider-field-api-key-variable = API key 变量
provider-field-external-auth = 外部身份验证
provider-field-aws-profile = AWS 配置文件
provider-field-aws-region = AWS 区域
provider-field-project-id = 项目 ID
provider-field-location = 位置
provider-field-credential-file = 凭据文件（服务器）


## tui/src/app/exit_summary.rs

exit-summary-failed-archive-thread = 归档会话 { $thread_id } 失败：{ $error }
exit-summary-archive-not-confirmed = 未能确认线程 { $thread_id } 已归档。
exit-summary-use-stop-shortcut = 使用已配置的停止快捷键
exit-summary-press-key = 按 { $key }
exit-summary-disconnected-running-continues = 已与此任务断开连接。正在运行的工作将继续。
exit-summary-disconnected-may-be-running = 已与此任务断开连接。工作可能仍在运行。
exit-summary-disconnected-turn-stopped = 已与此任务断开连接。当前轮次已停止。
exit-summary-to-reconnect = 要重新连接，请运行：
exit-summary-stop-current-turn = 停止当前轮次：运行 { $command }，选择此任务，然后{ $hint }。
exit-summary-to-continue = 要继续此会话，请运行：
exit-summary-or-run-select = 或运行 { $command } 并选择 { $name }。
exit-summary-session-id = 会话 ID：{ $thread_id }

## tui/src/app/exit_summary.rs


## tui/src/app/side.rs

side-cannot-be-renamed = 侧边对话不能重命名。
side-main-thread-unavailable = 主会话准备就绪前无法使用 '/side'。
side-no-started-conversation = 当前对话开始前无法使用 '/side'。请先发送一条消息，然后再试 /side。
side-already-open = 侧边对话已打开。按 ctrl + c 返回后再开始另一个。
side-main-needs-input = 主会话需要输入
side-parent-needs-input = 父会话需要输入
side-main-needs-approval = 主会话需要审批
side-parent-needs-approval = 父会话需要审批
side-main-failed = 主会话失败
side-parent-failed = 父会话失败
side-main-interrupted = 主会话已中断
side-parent-interrupted = 父会话已中断
side-main-closed = 主会话已关闭
side-parent-closed = 父会话已关闭
side-main-finished = 主会话已完成
side-parent-finished = 父会话已完成
side-from-main-thread = 来自主会话
side-from-parent-thread = 来自父会话（{ $parent_label }）
side-wait-permissions = 分叉前请等待权限更新完成。
side-failed-close-still-open = 无法关闭侧边对话 { $thread_id }；它仍处于打开状态：{ $err }
side-failed-prepare = 无法准备侧边对话 { $child_thread_id }：{ $err }
side-failed-switch-into-err = 无法切换到侧边对话 { $child_thread_id }：{ $err }
side-failed-switch-into = 无法切换到侧边对话 { $child_thread_id }。
side-failed-start = 无法启动侧边对话：{ $err }

## tui/src/app/sidebar_files.rs

sidebar-files-requires-server-workspace = 需要位于服务器主机上的工作区
sidebar-files-preview-unavailable-windows = Windows 上无法预览

## tui/src/app/startup.rs

startup-no-startup-model = 初始 app-server 连接未提供启动模型。
startup-permission-overrides-resuming = 恢复远程任务时不支持权限覆盖。
startup-permission-overrides-forking = 分叉远程任务时不支持权限覆盖。
startup-invalid-keymap = `tui.keymap` 配置无效：{ $err }{"\u000A"}请修复配置后重试。{"\u000A"}支持的操作和示例请参阅 Codex 键位文档。
startup-default-environment = 默认
startup-could-not-restart-server = 无法重启后台服务：{ $err }
startup-reconnect-failed-stale = 重新连接失败，代理列表已过期；请重新启动以重试
startup-reconnect-timed-out = 自动重连超时：等待服务器完成关闭。

## tui/src/app/thread_settings.rs

thread-settings-failed-update = 无法更新会话设置：{ $err }

## tui/src/app/user_verification_errors.rs

user-verification-could-not-complete-local = 无法通过本地 Codex 程序完成用户验证。
user-verification-could-not-verify-request = 本地 Codex 程序无法验证此请求。
user-verification-no-credential = 本地 Codex 程序中没有可用的用户验证凭据。
user-verification-biometric-unavailable = 此设备当前不支持生物识别验证。
user-verification-unavailable-for-request = 此请求不支持用户验证。
user-verification-cancelled = 用户验证已取消。
user-verification-interrupted = 用户验证已中断。
user-verification-biometric-failed = 生物识别验证未成功，请求已取消。
user-verification-timed-out = 用户验证超时，请求已取消。
user-verification-could-not-complete = 本地 Codex 程序无法完成用户验证。
side-suffix-for-side = {" "}进入侧边对话
side-suffix-to-switch = {" "}切换
side-suffix-to-archive = {" "}归档
side-suffix-to-close = {" "}关闭
side-label-prefix = 侧边{" "}

## tui/src/bottom_pane/footer.rs

footer-plan-mode = 计划模式
footer-cycle-suffix = {" "}({ $key } 切换)
footer-cycle-open = {" "}(
footer-cycle-close = {" "}切换)
footer-for-agents = {" "}查看代理
footer-for-shortcuts = {" "}查看快捷键
footer-to-queue-message = {" "}加入排队消息
footer-to-queue = {" "}加入队列
footer-goal-pursuing-usage = 正在推进目标（{ $usage }）
footer-goal-pursuing = 正在推进目标
footer-goal-paused = 目标已暂停（/goal resume）
footer-goal-stalled = 目标已停滞（/goal resume）
footer-goal-usage-limited = 目标触及用量限制（/goal resume）
footer-goal-unmet-usage = 目标未达成（{ $usage }）
footer-goal-abandoned = 目标已放弃
footer-goal-achieved-usage = 目标已达成（{ $usage }）
footer-goal-achieved = 目标已达成
footer-ide-context = IDE 上下文
footer-reverse-i-search = 反向搜索:{" "}
footer-again-to-quit = {" "}再按一次退出
footer-again-to-edit-previous = {" "}再按一次编辑上一条消息
footer-to-edit-previous = {" "}编辑上一条消息
footer-percent-context-left = 剩余上下文 { $percent }%
footer-tokens-used = 已用 { $used_fmt }
footer-full-context-left = 剩余上下文 100%

## tui/src/auto_review_denials.rs

auto-review-send-input = 向终端 { $process_id } 发送输入：{ $stdin }
auto-review-apply-patch-one = apply_patch 修改 { $file }
auto-review-apply-patch-many = apply_patch 修改 { $count } 个文件
auto-review-network-access = 访问网络：{ $target }
auto-review-mcp-on = { $label } 上的 MCP { $tool_name }
auto-review-permission-request-reason = 权限请求：{ $reason }
auto-review-permission-request = 权限请求

## tui/src/chatwidget/realtime.rs

realtime-still-stopping = 语音对话仍在停止中。
realtime-unavailable-side = 侧边对话中无法使用语音模式。请先返回主会话。
realtime-unsupported-platform = 语音功能需要 macOS、基于 MSVC 的 Windows 构建或基于 glibc 的 Linux 构建。
realtime-start-conversation-first = 请先开始对话，再使用语音模式。
realtime-failed-start = 无法启动语音模式：{ $message }
realtime-conversation-failed = 语音对话失败：{ $message }
realtime-failed-restore-mute = 无法恢复麦克风静音：{ $error }
realtime-connection-timed-out = 语音连接超时。清理后将重试一次。
realtime-failed-connect = 无法连接语音模式：{ $error }
realtime-conversation-started = 语音对话已开始。
realtime-started-hint = 使用 /voice mute 静音，或使用 /voice 停止。
realtime-failed-deliver = 无法送达语音回复。
realtime-closed-during-startup = 语音连接在启动期间关闭。将重试一次。
realtime-conversation-ended = 语音对话已结束：{ $reason }

## tui/src/history_cell/search.rs

history-search-in-url = { $url } 中的 '{ $pattern }'
history-search-opened-page = 已打开页面
history-search-opened = 已打开
history-search-opening-page = 正在打开页面
history-search-opening = 正在打开
history-search-searched-for = 已搜索
history-search-searched-page = 已搜索页面
history-search-searching-for = 正在搜索
history-search-searching-page = 正在搜索页面
history-search-browsing = 正在浏览网页
history-search-searched-web = 已搜索网页
history-search-searching-web = 正在搜索网页
history-search-for = ：

## tui/src/session_start.rs

session-start-failed-unarchive = 无法取消归档会话 { $thread_id }
session-start-failed-resume = 无法从 { $target_label } 恢复会话：{ $err }
session-start-failed-fork = 无法从 { $target_label } 分叉会话：{ $err }

## tui/src/tool_output.rs

tool-output-omitted-lines = +{ $omitted } 行（{ $hint }）

## tui/src/external_agent_config_migration/mod.rs

external-migration-import-selected = 导入所选项
external-migration-customize-selection = 自定义选择
external-migration-cancel = 取消
external-migration-review-selection = 检查所选项
external-migration-current-project = 当前项目：
external-migration-home = 主目录
external-migration-importing-none = 正在导入：无
external-migration-importing-summary = 正在导入：{ $count_summary }
external-migration-plus-more = 另有 { $hidden_plugin_count } 个
external-migration-plus-more-marketplaces = 另有 { $hidden_marketplace_count } 个插件市场
external-migration-plugin-counts = （{ $marketplaces } 个插件市场，{ $plugins } 个插件）

## tui/src/keymap_setup.rs

keymap-setup-configure-this-shortcut = 配置此快捷键。
keymap-setup-no-custom-override = 没有可移除的自定义根覆盖
keymap-setup-custom-root-override = 自定义根覆盖
keymap-setup-default-keymap = 默认键位
keymap-setup-edit-shortcut = 编辑快捷键
keymap-setup-current = 当前{" "}
keymap-setup-config = 配置{" "}
keymap-setup-set-key = 设置按键
keymap-setup-capture-for-unbound = 为此未绑定的操作捕获一个按键
keymap-setup-replace-binding = 替换绑定
keymap-setup-capture-replacement = 为 `{ $current_binding }` 捕获替换按键
keymap-setup-add-alternate-binding = 添加备用绑定
keymap-setup-keep-and-add-key = 保留 `{ $current_binding }` 并添加另一个按键
keymap-setup-replace-one-binding = 替换其中一个绑定...
keymap-setup-choose-existing-to-replace = 选择要替换的现有绑定
keymap-setup-replace-all-bindings = 替换所有绑定
keymap-setup-replace-with-one-key = 将 `{ $current_binding }` 替换为一个按键
keymap-setup-use-leader = 使用前导键...
keymap-setup-capture-after-leader = 捕获紧随已配置前导键之后的按键
keymap-setup-leader-unbound = 前导键未绑定
keymap-setup-configure-leader = 配置前导键...
keymap-setup-set-leader-first = 请先设置前导键，再分配此快捷键
keymap-setup-set-key-chord = 设置组合键序列
keymap-setup-capture-two-keys = 为此操作捕获两个连续按键
keymap-setup-replace-with-chord = 替换为组合键序列
keymap-setup-replace-with-two-stroke = 将 `{ $current_binding }` 替换为两键组合序列
keymap-setup-add-alternate-chord = 添加备用组合键序列
keymap-setup-keep-and-add-chord = 保留 `{ $current_binding }` 并添加两键组合序列
keymap-setup-remove-custom-binding = 移除自定义绑定
keymap-setup-restore-default-binding = 恢复默认键位绑定
keymap-setup-back-to-shortcuts = 返回快捷键列表
keymap-setup-return-to-list = 返回快捷键列表
keymap-setup-changes-write-root = 更改会写入根级{" "}
keymap-setup-override-suffix = {" "}覆盖。
keymap-setup-replace-binding-title = 替换绑定
keymap-setup-choose-binding-to-replace = 选择要替换的绑定。
keymap-setup-replace-with-another-key = 将 `{ $binding }` 替换为另一个按键
keymap-setup-binding-key-chord = { $binding }（组合键序列）
keymap-setup-binding-replace-two-stroke = 将 `{ $binding }` 替换为两键组合序列
keymap-setup-shortcut-conflict = 快捷键冲突
keymap-setup-cannot-use = { $context }.{ $action } 不能使用 `{ $key }`。
keymap-setup-pick-another-key = 换一个按键
keymap-setup-return-to-capture = 返回此操作的按键捕获
keymap-setup-cancel = 取消
keymap-setup-leave-unchanged = 保持键位不变
keymap-setup-no-change = 无更改：`{ $context }.{ $action }` 已在使用 `{ $key }`。
keymap-setup-no-longer-uses = `{ $context }.{ $action }` 不再使用 `{ $old_key }`。请重新打开 /keymap 并再次选择绑定。
keymap-setup-remapped = 已将 `{ $context }.{ $action }` 重新映射为 `{ $key }`。
keymap-setup-added = 已将 `{ $key }` 添加到 `{ $context }.{ $action }`。
keymap-setup-replaced = 已将 `{ $context }.{ $action }` 的 `{ $old_key }` 替换为 `{ $key }`。
keymap-setup-unknown-action = 未知的键位操作 `{ $context }.{ $action }`。请重新打开 /keymap 并选择操作。
keymap-setup-only-modifiers = `tui.keymap` 中只能存储 ctrl、alt、shift 和 super 修饰键。
keymap-setup-only-function-keys = `tui.keymap` 中只能存储 F1 到 F{ $max } 的功能键。
keymap-setup-only-ascii = `tui.keymap` 中只能存储可打印的 ASCII 按键。
keymap-setup-key-unsupported = `tui.keymap` 不支持该按键。

## tui/src/worktree_startup.rs

worktree-startup-untrusted-source = `--worktree` 无法从明确不受信任的来源创建检出
worktree-startup-local-only = `--worktree` 仅支持本地会话
worktree-startup-session-not-found = 未找到会话：{ $id_or_name }
worktree-startup-recovery = 启动未能完成将会话绑定到此 worktree：{ $root }{"\u000A"}检出已保留。请检查它，并确认没有会话正在使用它。{"\u000A"}要删除它，请在源仓库中运行 `git worktree remove <checkout-path>`，{"\u000A"}并将 <checkout-path> 替换为上面的路径。请勿使用 --force。


## app/history_pagination.rs

history-pagination-no-previous-message = 没有可编辑的上一条消息。

## app/misalignment_policy.rs

misalignment-policy-continuation-request = 继续请求（引用）
misalignment-policy-what-we-detected = 我们检测到的内容
misalignment-policy-cannot-continue = 无法继续此对话。请在再试前查看其最新状态。

## app/user_verification.rs

user-verification-unavailable-remote = 远程工作区不支持用户验证。

## app_server_connection.rs

app-server-connection-reopen-cloud = 请重新打开 Codex 以重新连接 Cloud 后端。

## app/thread_goal_actions.rs

thread-goal-actions-ephemeral = 目标需要已保存的会话，而当前会话是临时的。{"\u000A"}运行 `codex` 开始一个可保存的会话，或用 `codex resume` / `/resume` 重新打开一个。
thread-goal-actions-no-goal-set = 当前未设置目标。
thread-goal-actions-goal-status = 目标 { $status }
thread-goal-actions-cleared = 目标已清除
thread-goal-actions-no-goal-to-clear = 没有可清除的目标
thread-goal-actions-no-goal-this-thread = 此会话当前没有目标。
thread-goal-actions-replace-current = 替换当前目标
thread-goal-actions-replace-desc = 设置新目标并立即开始
thread-goal-actions-cancel = 取消
thread-goal-actions-keep-current = 保留当前目标
thread-goal-actions-replace-title = 替换目标？
thread-goal-actions-new-objective = 新目标：{ $objective }
thread-goal-actions-create-first = 编辑目标前请先创建目标。
thread-goal-actions-failed-read = 读取会话目标失败：{ $err }
thread-goal-actions-failed-replace = 替换会话目标失败：{ $err }
thread-goal-actions-failed-set = 设置会话目标失败：{ $err }
thread-goal-actions-failed-update = 更新会话目标失败：{ $err }
thread-goal-actions-failed-clear = 清除会话目标失败：{ $err }

## app/workspace_hud.rs

workspace-hud-panes = 面板
workspace-hud-move = 移动
workspace-hud-this-pane = 当前面板
workspace-hud-command-center = 命令中心
workspace-hud-main-composer = 主输入框
workspace-hud-open-in-sidebar = 在侧边栏中打开
workspace-hud-hide-sidebar = 隐藏侧边栏
workspace-hud-sidebar-content = 侧边栏内容
workspace-hud-resize-panes = 调整面板大小
workspace-hud-minimum-width = 最小宽度
workspace-hud-maximum-width = 最大宽度
workspace-hud-equal-widths = 等宽
workspace-hud-divider-left = 左移分隔条
workspace-hud-divider-right = 右移分隔条
workspace-hud-restore-layout = 恢复布局
workspace-hud-zoom-pane = 放大面板
workspace-hud-close-tab = 关闭标签页
workspace-hud-previous-tab = 上一个标签页
workspace-hud-next-tab = 下一个标签页
workspace-hud-hide-pane = 隐藏面板
workspace-hud-pane-left = 向左移动面板
workspace-hud-pane-right = 向右移动面板
workspace-hud-choose-environment = 选择环境
workspace-hud-presets = 预设 ›
workspace-hud-find-text = 查找文本
workspace-hud-select-transcript = 选择对话记录
workspace-hud-mark-table-cell = 标记表格单元格
workspace-hud-composer = 输入框
workspace-hud-sidebar = 侧边栏
workspace-hud-conversation = 对话
workspace-hud-leader-title = { $prefix } · { $origin }


## bottom_pane/mentions_v2/popup.rs

mentions-v2-popup-loading = 加载中...
mentions-v2-popup-no-matches = 没有匹配项


## chatwidget/interaction.rs

interaction-review-steer-unavailable = /review 期间不支持引导消息。现在按 Ctrl+C 可取消审查。
interaction-ctrl-l-disabled = 任务进行期间 Ctrl+L 不可用。
interaction-no-agent-response = 没有可复制的代理回复
interaction-copy-failed = 复制失败: { $error }
interaction-rename-thread = 重命名会话
interaction-name-thread = 命名会话
interaction-type-a-name = 输入名称并按 Enter
interaction-thread-name-empty = 会话名称不能为空。
interaction-generating-title = 正在生成标题建议…
interaction-suggested-from-conversation = 根据此对话生成的建议

## clipboard_copy/worker.rs

clipboard-worker-selection-ended = 选择在送达剪贴板前已结束
clipboard-worker-nothing-to-copy = 没有可复制的内容：所选内容为空
clipboard-worker-stopped = 剪贴板工作进程已停止
clipboard-worker-no-read-pending = 没有待处理的剪贴板读取
clipboard-worker-read-timed-out = 读取剪贴板超时
clipboard-worker-start-failed = 无法启动剪贴板工作进程：{ $error }

## clipboard_paste/text.rs

clipboard-text-windows-reader-unavailable = Windows 剪贴板读取器不可用
clipboard-text-reader-start-failed = 无法启动剪贴板读取器
clipboard-text-unavailable-over-ssh = 通过 SSH 无法使用剪贴板文本
clipboard-text-windows-reader-busy = Windows 剪贴板读取器忙碌或不可用
clipboard-text-read-timed-out = 读取剪贴板超时
clipboard-text-unavailable = 剪贴板文本不可用
clipboard-text-too-large = 剪贴板文本超出消息大小限制
clipboard-text-windows-reader-start-failed = 无法启动 Windows 剪贴板读取器
clipboard-text-reader-no-input = 剪贴板读取器没有输入
clipboard-text-read-start-failed = 无法开始读取剪贴板
clipboard-text-reader-no-output = 剪贴板读取器没有输出
clipboard-text-read-failed = 无法读取剪贴板文本
clipboard-text-not-utf8 = 剪贴板文本不是 UTF-8 编码

## config_panel/render.rs

config-render-animations = 动画
config-render-animations-desc = 为欢迎界面、工作指示器和过渡效果添加动画
config-render-composer-stars = 输入框星标
config-render-composer-stars-desc = 为新的 Astra 任务短暂显示星标
config-render-effects = 视觉效果
config-render-effects-subtitle = 配置视觉效果
config-render-stars-need-animations = 开启动画以显示 Astra 任务星标
config-render-mouse-scroll-speed = 鼠标滚动速度
config-render-mouse-scroll-speed-desc = 为全屏对话记录选择鼠标滚轮速度
config-render-speed-default = 1×（默认）
config-render-unfocused = 未聚焦时
config-render-always = 始终
config-render-notify-when = 通知时机
config-render-notify-when-desc = 仅在你离开或终端也处于聚焦状态时通知
config-render-auto-default = 自动（默认）
config-render-off = 关闭
config-render-on = 开
config-render-copy-on-select = 选中即复制
config-render-copy-on-select-desc = 松开鼠标时复制对话记录、页脚和输入框中的选中内容；自动则沿用终端的默认设置
config-render-right-click-paste = 右键粘贴
config-render-dense = 紧凑
config-render-comfortable = 宽松
config-render-session-picker-layout = 会话选择器布局
config-render-session-picker-layout-desc = 恢复和分叉会话时，选择更紧凑的行或更宽松的列表
config-render-ask = 询问
config-render-current-directory = 当前目录
config-render-session-directory = 会话目录
config-render-resume-directory = 恢复时的目录
config-render-resume-directory-desc = 恢复会话时选择工作目录；本次会话中 --cd 优先
config-render-requires-cd-remote = 远程会话需要 --cd
config-render-changes-save-automatically = 更改会自动保存
config-render-configuration = 配置
config-render-hint-toggle = 切换
config-render-hint-back = 返回
config-render-hint-group = 分组
config-render-hint-search = 搜索
config-render-hint-open = 打开
config-render-hint-test = 测试
config-render-hint-clear-search-close = 清除搜索/关闭
config-render-hint-clear-search = 清除搜索
config-render-search-placeholder = 输入以搜索设置

## history_cell/mcp.rs

history-mcp-auth-unknown = 未知
history-mcp-auth-unsupported = 不支持
history-mcp-auth-not-logged-in = 未登录
history-mcp-auth-bearer-token = Bearer token
history-mcp-called = 已调用
history-mcp-calling = 调用中
history-mcp-failed-verb = 失败
history-mcp-returned-image = 返回的图片
history-mcp-see-the = {" "}   请参阅{" "}
history-mcp-mcp-docs = MCP 文档
history-mcp-to-configure-them = {" "}了解如何配置。
history-mcp-mcp-tools = MCP 工具
history-mcp-no-servers = {" "} • 未配置 MCP 服务器。
history-mcp-no-tools = {" "} • 没有可用的 MCP 工具。
history-mcp-status-connected = 已连接
history-mcp-status-starting = 启动中
history-mcp-status-auth-required = 需要认证
history-mcp-status-failed = 失败
history-mcp-status-not-started = 未启动
history-mcp-status-disabled = 已禁用
history-mcp-status-cancelled = 已取消
history-mcp-status-unknown = 未知
history-mcp-label-auth = {" "}   • 认证：
history-mcp-tools-none = {" "}   • 工具：（无）
history-mcp-label-tools = {" "}   • 工具：
history-mcp-resources-none = {" "}   • 资源：（无）
history-mcp-label-resources = {" "}   • 资源：
history-mcp-templates-none = {" "}   • 资源模板：（无）
history-mcp-label-templates = {" "}   • 资源模板：
history-mcp-use-verbose = {" "} 使用 /mcp verbose 查看工具和资源。
history-mcp-loading-inventory = 正在加载 MCP 清单
history-mcp-loading-inventory-raw = 正在加载 MCP 清单...
history-mcp-error = 错误：{ $error }
history-mcp-verb-name = { $verb } { $name }
history-mcp-name-with-environment = { $name }（环境 { $environment_id }）
history-mcp-environment-suffix = {" "}（环境 { $environment_id }）
history-mcp-tool-count = （{ $count } 个工具）

## thread_transcript/other_items.rs

other-items-patch-declined = 补丁应用已被拒绝
other-items-patch-in-progress = 补丁应用进行中
other-items-status-unavailable = 状态不可用
other-items-context-compacted = 上下文已压缩
other-items-hook-prompt = hook 提示：{" "}
other-items-image-generation = 图像生成 · { $status }
other-items-review-started = >> 代码审查已开始：{ $review } <<
other-items-review-finished = << 代码审查已结束：{ $review } >>

## text_formatting.rs

text-formatting-and-two = { $first }和{ $second }
text-formatting-and-last = { $list }和{ $last }

## tooltips.rs

tooltips-tip-prefix = **提示：** { $tip }

## session_resume.rs

session-resume-parent-hint = {"\u000A"}请改为恢复其父会话：codex resume { $parent_id }
session-resume-guardian = 无法恢复 Guardian 审查会话。{ $parent_hint }
session-resume-cwd-unknown = 无法确定所选会话记录的工作目录

## tui/src/app/provider.rs
provider-back = 返回
provider-loading-title = 正在加载模型提供方…
provider-reconnect-to-browse = 请重新连接 app-server 以浏览模型提供方。
provider-list-title = 模型提供方
provider-details-title = 模型提供方详情
provider-edit-configuration = 编辑配置
provider-no-active-chat = 没有活动对话
provider-server-default = 服务器默认
provider-default-for-new-chats = 新对话的默认值
provider-client-launch-selects = 此客户端的启动设置为新对话选择 { $id }。
provider-server-fixes-default = 服务器配置固定了默认提供方。
provider-use-new-to-start = 使用 /new 以所选提供方开始对话。
provider-list-subtitle = 当前对话：{ $current_label } · { $default_label }：{ $default_id }
provider-hint-details = {" "}详情
provider-hint-set-default = {" "}设为默认
provider-hint-back = {" "}返回
provider-field-id = 提供方 ID
provider-field-type = 类型
provider-type-builtin-openai = 内置 OpenAI 提供方
provider-absent-not-reported = 未报告
provider-absent-managed-externally = 由外部管理
provider-absent-none = 无
provider-absent-not-set = 未设置

## tui/src/app/tui_mode_picker.rs
tui-mode-picker-saved = 已保存 TUI 模式：{ $mode }。重启 Codex 后生效；启动参数的覆盖设置仍然有效。
tui-mode-picker-save-failed = 无法保存 TUI 模式：{ $error }



## tui/src/bottom_pane/approval_overlay.rs
approval-overlay-send-input-existing = 是否向现有终端发送输入？
approval-overlay-send-input-terminal = 是否向终端 { $process_id } 发送输入？
approval-overlay-run-command = 是否运行以下命令？
approval-overlay-network-access = 是否批准访问网络地址 "{ $host }"？
approval-overlay-grant-permissions = 是否授予这些权限？
approval-overlay-make-edits = 是否进行以下修改？
approval-overlay-needs-approval = { $server } 需要你的批准。
approval-overlay-granted-none = 你没有授予额外权限
approval-overlay-granted-strict = 你已授予额外权限（严格自动审查）
approval-overlay-granted-session = 你已为本次会话授予额外权限
approval-overlay-granted = 你已授予额外权限
approval-overlay-hint-confirm = 确认
approval-overlay-hint-cancel = 取消
approval-overlay-hint-or = {" "}或{" "}
approval-overlay-hint-press = 按{" "}
approval-overlay-hint-open-thread = {" "}打开会话
approval-overlay-label-thread = 会话:{" "}
approval-overlay-label-environment = 环境：
approval-overlay-label-permission-rule = 权限规则：
approval-overlay-label-reason = 原因：
approval-overlay-label-input = 输入：
approval-overlay-label-server = 服务器：
approval-overlay-yes-once = 是，仅此一次
approval-overlay-yes-proceed = 是，继续
approval-overlay-yes-prefix = 是，且不再询问以 `{ $rendered_prefix }` 开头的命令
approval-overlay-yes-host-conversation = 是，且在本次对话中允许此主机
approval-overlay-yes-permissions-session = 是，且在本次会话中允许这些权限
approval-overlay-yes-command-session = 是，且在本次会话中不再询问此命令
approval-overlay-yes-host-future = 是，且以后允许此主机
approval-overlay-no-host-future = 否，且以后阻止此主机
approval-overlay-no-continue-without-running = 否，不运行并继续
approval-overlay-no-tell-codex = 否，并告诉 Codex 该如何调整
approval-overlay-yes-files-session = 是，且不再询问这些文件
approval-overlay-yes-grant-turn = 是，在本轮授予这些权限
approval-overlay-yes-grant-strict = 是，在本轮授予并启用严格自动审查
approval-overlay-yes-grant-session = 是，在本次会话中授予这些权限
approval-overlay-no-without-permissions = 否，不授予权限并继续
approval-overlay-yes-provide-info = 是，提供所请求的信息
approval-overlay-no-continue-without-it = 否，不提供并继续
approval-overlay-cancel-request = 取消此请求
approval-overlay-rule-network = 网络
approval-overlay-rule-read = 读取 { $reads }
approval-overlay-rule-write = 写入 { $writes }
approval-overlay-rule-deny-read = 禁止读取 { $denied_reads }
approval-overlay-rule-glob = glob `{ $pattern }`

## tui/src/bottom_pane/chat_composer.rs
chat-composer-message-too-large = 消息超过最大长度 { $max } 个字符（已提供 { $actual_chars } 个）。
chat-composer-viewing-subagent = 正在查看子代理：已禁用直接输入
chat-composer-unrecognized-command = 无法识别的命令 '/{ $name }'。输入 "/" 查看支持的命令列表。
chat-composer-message-too-long-limit = 消息过长；上限为 { $max } 个字符
chat-composer-command-disabled-during-task = 任务进行中已禁用 '/{ $command }'。
chat-composer-shell-mode = Shell 模式
chat-composer-capability-skills = skills
chat-composer-plugin-description = 插件
chat-composer-plugin-description-capabilities = 插件 · { $capabilities }
chat-composer-shutting-down = 正在关闭…
chat-composer-input-disabled = 输入已禁用。
chat-composer-capability-mcp-servers = { $count } 个 MCP 服务器
chat-composer-capability-apps = { $count } 个应用

## tui/src/bottom_pane/mcp_server_elicitation.rs
mcp-elicitation-answer-placeholder = 输入你的答案
mcp-elicitation-answer-placeholder-optional = 输入你的答案（可选）
mcp-elicitation-run-tool-continue = 运行该工具并继续
mcp-elicitation-allow-request-continue = 允许此请求并继续
mcp-elicitation-allow = 允许
mcp-elicitation-run-tool-session = 运行该工具，并在本次会话中记住此选择
mcp-elicitation-allow-request-session = 允许此请求，并在本次会话中记住此选择
mcp-elicitation-allow-session = 本次会话中允许
mcp-elicitation-run-tool-always = 运行该工具，并在今后的工具调用中记住此选择
mcp-elicitation-allow-request-always = 允许此请求，并在今后的请求中记住此选择
mcp-elicitation-always-allow = 始终允许
mcp-elicitation-cancel = 取消
mcp-elicitation-cancel-tool-call = 取消此次工具调用
mcp-elicitation-deny = 拒绝
mcp-elicitation-decline-continue = 拒绝此请求并继续
mcp-elicitation-cancel-request = 取消此请求
mcp-elicitation-true = 是
mcp-elicitation-false = 否
mcp-elicitation-server-message = 服务器：{ $server }{"\u000A"}{"\u000A"}{ $message }
mcp-elicitation-tip-submit = 提交
mcp-elicitation-tip-submit-all = 提交全部
mcp-elicitation-tip-submit-answer = 提交答案
mcp-elicitation-tip-navigate-fields = 切换字段
mcp-elicitation-tip-change-field = 切换字段
mcp-elicitation-tip-cancel = 取消
mcp-elicitation-answer-required = 提交前请先回答必填字段。
mcp-elicitation-no-options = 没有选项
mcp-elicitation-option-position = 选项 { $selected }/{ $total }
mcp-elicitation-field-position = 字段 { $idx }/{ $total }
mcp-elicitation-field-position-unanswered = { $base }（{ $unanswered } 个必填项未回答）
mcp-elicitation-no-fields = 没有字段


## tui/src/bottom_pane/app_link_view.rs
app-link-sign-in-instructions = 在浏览器中登录此应用，然后返回此处。
app-link-action-required = 需要操作
app-link-server = 服务器：{ $server }
app-link-complete-action-instructions = 在浏览器中完成所要求的操作，然后返回此处。
app-link-open-sign-in-url = 打开登录链接
app-link-back = 返回
app-link-already-signed-in = 我已登录
app-link-open-link = 打开链接
app-link-finished = 我已完成
app-link-manage-on-chatgpt = 在 ChatGPT 中管理
app-link-disable-app = 停用应用
app-link-enable-app = 启用应用
app-link-install-on-chatgpt = 在 ChatGPT 中安装
app-link-already-installed = 我已安装
app-link-use-dollar-insert = 使用 $ 将此应用插入提示词。
app-link-new-apps-delay = 新安装的应用可能需要几分钟才会出现在 /apps 中。
app-link-after-installed-insert = 安装后，使用 $ 将此应用插入提示词。
app-link-finish-app-sign-in = 完成应用登录
app-link-finish-authentication = 完成认证
app-link-finish-in-browser = 在浏览器中完成
app-link-finish-app-setup = 完成应用设置
app-link-chatgpt-sign-in-browser = 请在刚打开的浏览器窗口中登录 ChatGPT 上的应用。
app-link-complete-auth-browser = 请在刚打开的浏览器窗口中完成认证。
app-link-return-select-signed-in = 然后返回此处并选择 "我已登录"。
app-link-complete-action-browser = 请在刚打开的浏览器窗口中完成所要求的操作。
app-link-return-select-finished = 然后返回此处并选择 "我已完成"。
app-link-complete-setup-browser = 请在刚打开的浏览器窗口中完成 ChatGPT 上的应用设置。
app-link-return-select-installed = 如有需要请先在那里登录，然后返回此处并选择 "我已安装"。
app-link-sign-in-url-label = 登录链接：
app-link-link-label = 链接：
app-link-setup-url-label = 设置链接：
app-link-hint-move = {" "}移动
app-link-hint-select = {" "}选择
app-link-hint-close = {" "}关闭
app-link-no-actions = 没有可用操作

## tui/src/cwd_prompt.rs
cwd-prompt-verb-resume = 恢复
cwd-prompt-verb-fork = 派生
cwd-prompt-past-resumed = 已恢复
cwd-prompt-past-forked = 已派生
cwd-prompt-save-failed = 保存工作目录偏好失败。
cwd-prompt-title = 工作目录 · { $action_verb }
cwd-prompt-explanation = 会话 = { $action_past }会话中记录的最新工作目录{"\u000A"}当前 = 你当前的工作目录
cwd-prompt-use-session-dir = 使用会话目录（{ $session_cwd }）
cwd-prompt-use-current-dir = 使用当前目录（{ $current_cwd }）
cwd-prompt-always-session-dir = 始终使用会话目录
cwd-prompt-always-current-dir = 始终使用当前目录
cwd-prompt-always-current-dir-path = 始终使用当前目录（{ $path }）
cwd-prompt-hint-continue = {" "}继续 ·{" "}
cwd-prompt-hint-use-session = {" "}使用会话目录 ·{" "}
cwd-prompt-hint-quit = {" "}退出

## tui/src/external_agent_config_migration/source.rs
external-agent-source-title = 选择导入来源
external-agent-source-subtitle = 选择要导入其配置的应用。
external-agent-source-hint-continue = {" "}继续 ·{" "}
external-agent-source-hint-cancel = {" "}取消

## tui/src/daemon_startup.rs
daemon-startup-state-enabled = 启用
daemon-startup-state-disabled = 禁用
daemon-startup-requires-feature = 此会话要求 { $name } 处于{ $state }状态
daemon-startup-running-without-server = 未使用共享后台服务运行：{ $reason }。

## tui/src/status/rate_limits.rs
rate-limits-bucket-limit = { $bucket } 限额
rate-limits-bucket-window-limit = { $bucket } { $window }限额
rate-limits-window-limit = { $window }限额
rate-limits-monthly-credit-limit = 每月额度上限
rate-limits-credits-used = 已使用 { $used } / { $limit } 额度
rate-limits-percent-left = 剩余 { $percent }%
rate-limits-credits = 额度
rate-limits-unlimited = 无限制
rate-limits-available = 可用
rate-limits-credits-balance = { $balance } 额度

## tui/src/transcript_view/follow_control.rs
follow-control-new-back-esc = {" "}有新活动 · ↓ 回到底部 · esc{" "}
follow-control-new-bottom = {" "}有新活动 · ↓ 底部{" "}
follow-control-new-short = {" "}新 · ↓ 底部{" "}
follow-control-bottom = {" "}↓ 底部{" "}
follow-control-arrow = {" "}↓{" "}
follow-control-back-esc = {" "}↓ 回到底部 · esc{" "}
follow-control-back = {" "}↓ 回到底部{" "}
follow-control-bottom-esc = {" "}↓ 底部 · esc{" "}

## tui/src/transcript_view/disclosure.rs
disclosure-collapse = 折叠
disclosure-expand = 展开
disclosure-previous = 上一个
disclosure-next = 下一个
disclosure-back = 返回


## tui/src/app/owned_transcript.rs
owned-transcript-starting-side-chat = 正在启动旁路对话…
owned-transcript-nav-enter-esc-latest = enter/esc 回到最新
owned-transcript-nav-esc-latest = esc 回到最新
owned-transcript-selecting-text = 正在选择文本 · esc 取消

## Display constants shared between files
const-resize-hint = 调整大小 · ←/h →/l · Shift+←/→ 10 列 · Enter/Esc 完成
const-external-agent-config-import-in-progress-message = 上一次外部代理导入仍在进行，请等它完成后再导入。
const-side-edit-previous-unavailable-message = 侧边对话中无法编辑之前的提示词。
const-trusted-access-for-cyber-verification-warning = 你的对话被标记了多项潜在网络安全风险。由于额外的安全检查已开启，响应可能会更慢。如需获得安全相关工作的授权，请加入 Trusted Access for Cyber 计划：https://chatgpt.com/cyber
const-tui-stub-message = TUI 中暂不可用。
const-parent-owned-input-message = 此子代理由其父级控制，已禁用直接输入。
const-ask-for-approval-label = 请求审批
const-approve-for-me-label = 自动审批
const-auto-review-description = 仅在检测到可能不安全的操作时请求审批
const-create-custom-provider = 创建自定义提供方或网关
const-unconfirmed-create = 无法确认保存结果。再次添加前请检查提供方列表。
const-setup-connection-lost = Codex 在设置过程中断开了连接。请重启 Codex 后继续。
const-goal-usage = 用法：/goal [<objective>|clear|edit|pause|resume]
const-compaction-header = 正在压缩上下文
const-compaction-details = 正在腾出空间以继续。
const-unanswered-confirm-title = 仍有未回答的问题，确定提交吗？
const-plan-implementation-title = 实施此计划？

## Permissions presets (codex-utils-approval-presets)
permissions-preset-read-only = 只读
permissions-preset-default = 默认
permissions-preset-full-access = 完全访问

## Collaboration modes (codex-protocol's ModeKind)
collaboration-mode-default = 默认
collaboration-mode-plan = 计划
