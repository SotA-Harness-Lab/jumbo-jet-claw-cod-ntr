// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: marked the remaining Windows-only code; removed the desktop app integration; made the interface translatable

use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Presets,
    Daybreak,
    Ide,
    Permissions,
    Keymap,
    Vim,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    Experimental,
    #[strum(to_string = "approve")]
    AutoReview,
    Memories,
    Skills,
    Import,
    Hooks,
    Review,
    Rename,
    New,
    Archive,
    Delete,
    Resume,
    Fork,
    Worktree,
    Init,
    Compact,
    Config,
    Recap,
    Plan,
    Voice,
    Goal,
    Agents,
    Side,
    Btw,
    Copy,
    Export,
    Raw,
    Tui,
    Diff,
    Mention,
    Status,
    Daemon,
    Warnings,
    Cd,
    #[strum(to_string = "pwd", serialize = "cwd")]
    Pwd,
    Usage,
    DebugConfig,
    Title,
    Statusline,
    Theme,
    #[strum(to_string = "pets", serialize = "pet")]
    Pets,
    Mcp,
    Apps,
    Plugins,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    #[strum(to_string = "stop", serialize = "clean")]
    Stop,
    Clear,
    TestApproval,
    #[strum(serialize = "subagents")]
    MultiAgents,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Config => crate::i18n::tr!("slash-config"),
            SlashCommand::Feedback => crate::i18n::tr!("slash-feedback"),
            SlashCommand::New => crate::i18n::tr!("slash-new"),
            SlashCommand::Init => crate::i18n::tr!("slash-init"),
            SlashCommand::Compact => crate::i18n::tr!("slash-compact"),
            SlashCommand::Recap => crate::i18n::tr!("slash-recap"),
            SlashCommand::Review => crate::i18n::tr!("slash-review"),
            SlashCommand::Rename => crate::i18n::tr!("slash-rename"),
            SlashCommand::Resume => crate::i18n::tr!("slash-resume"),
            SlashCommand::Archive => crate::i18n::tr!("slash-archive"),
            SlashCommand::Delete => crate::i18n::tr!("slash-delete"),
            SlashCommand::Clear => crate::i18n::tr!("slash-clear"),
            SlashCommand::Fork => crate::i18n::tr!("slash-fork"),
            SlashCommand::Worktree => crate::i18n::tr!("slash-worktree"),
            SlashCommand::Quit | SlashCommand::Exit => crate::i18n::tr!("slash-quit"),
            SlashCommand::Copy => crate::i18n::tr!("slash-copy"),
            SlashCommand::Export => crate::i18n::tr!("slash-export"),
            SlashCommand::Raw => crate::i18n::tr!("slash-raw"),
            SlashCommand::Tui => crate::i18n::tr!("slash-tui"),
            SlashCommand::Diff => crate::i18n::tr!("slash-diff"),
            SlashCommand::Mention => crate::i18n::tr!("slash-mention"),
            SlashCommand::Skills => crate::i18n::tr!("slash-skills"),
            SlashCommand::Import => crate::i18n::tr!("slash-import"),
            SlashCommand::Hooks => crate::i18n::tr!("slash-hooks"),
            SlashCommand::Daemon => crate::i18n::tr!("slash-daemon"),
            SlashCommand::Warnings => crate::i18n::tr!("slash-warnings"),
            SlashCommand::Status => crate::i18n::tr!("slash-status"),
            SlashCommand::Cd => crate::i18n::tr!("slash-cd"),
            SlashCommand::Pwd => crate::i18n::tr!("slash-pwd"),
            SlashCommand::Usage => crate::i18n::tr!("slash-usage"),
            SlashCommand::DebugConfig => crate::i18n::tr!("slash-debug-config"),
            SlashCommand::Title => crate::i18n::tr!("slash-title"),
            SlashCommand::Statusline => crate::i18n::tr!("slash-statusline"),
            SlashCommand::Theme => crate::i18n::tr!("slash-theme"),
            SlashCommand::Pets => crate::i18n::tr!("slash-pets"),
            SlashCommand::Ps => crate::i18n::tr!("slash-ps"),
            SlashCommand::Stop => crate::i18n::tr!("slash-stop"),
            SlashCommand::MemoryDrop => crate::i18n::tr!("slash-memory-drop"),
            SlashCommand::MemoryUpdate => crate::i18n::tr!("slash-memory-update"),
            SlashCommand::Model => crate::i18n::tr!("slash-model"),
            SlashCommand::Presets => crate::i18n::tr!("slash-presets"),
            SlashCommand::Daybreak => crate::i18n::tr!("slash-daybreak"),
            SlashCommand::Ide => crate::i18n::tr!("slash-ide"),
            SlashCommand::Plan => crate::i18n::tr!("slash-plan"),
            SlashCommand::Voice => crate::i18n::tr!("slash-voice"),
            SlashCommand::Goal => crate::i18n::tr!("slash-goal"),
            SlashCommand::Agents => crate::i18n::tr!("slash-agents"),
            SlashCommand::MultiAgents => crate::i18n::tr!("slash-multi-agents"),
            SlashCommand::Side | SlashCommand::Btw => crate::i18n::tr!("slash-side"),
            SlashCommand::Permissions => crate::i18n::tr!("slash-permissions"),
            SlashCommand::Keymap => crate::i18n::tr!("slash-keymap"),
            SlashCommand::Vim => crate::i18n::tr!("slash-vim"),
            SlashCommand::ElevateSandbox => crate::i18n::tr!("slash-elevate-sandbox"),
            SlashCommand::Experimental => crate::i18n::tr!("slash-experimental"),
            SlashCommand::AutoReview => crate::i18n::tr!("slash-auto-review"),
            SlashCommand::Memories => crate::i18n::tr!("slash-memories"),
            SlashCommand::Mcp => crate::i18n::tr!("slash-mcp"),
            SlashCommand::Apps => crate::i18n::tr!("slash-apps"),
            SlashCommand::Plugins => crate::i18n::tr!("slash-plugins"),
            SlashCommand::Logout => crate::i18n::tr!("slash-logout"),
            SlashCommand::Rollout => crate::i18n::tr!("slash-rollout"),
            SlashCommand::TestApproval => crate::i18n::tr!("slash-test-approval"),
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Model
                | SlashCommand::Permissions
                | SlashCommand::Title
                | SlashCommand::Statusline
                | SlashCommand::Rename
                | SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Fork
                | SlashCommand::Plan
                | SlashCommand::Goal
                | SlashCommand::Voice
                | SlashCommand::Ide
                | SlashCommand::Keymap
                | SlashCommand::Mcp
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Cd
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Pets
                | SlashCommand::Side
                | SlashCommand::Btw
                | SlashCommand::Resume
        )
    }

    /// Whether this command remains available inside an active side conversation.
    pub fn available_in_side_conversation(self) -> bool {
        matches!(
            self,
            SlashCommand::Copy
                | SlashCommand::Agents
                | SlashCommand::Export
                | SlashCommand::Raw
                | SlashCommand::Diff
                | SlashCommand::Mention
                | SlashCommand::Status
                | SlashCommand::Daemon
                | SlashCommand::Warnings
                | SlashCommand::Pwd
                | SlashCommand::Usage
                | SlashCommand::Ide
                | SlashCommand::Rename
        )
    }

    /// Whether dispatch needs thread state to validate this command before consuming its draft.
    /// The composer must defer busy-state rejection and draft clearing for these commands.
    pub(crate) fn requires_dispatch_validation(self) -> bool {
        matches!(self, SlashCommand::Fork | SlashCommand::Review)
    }

    /// Commands that do not require a writable current thread. The server must still be connected.
    pub(crate) fn available_when_thread_unavailable(self) -> bool {
        matches!(
            self,
            SlashCommand::New
                | SlashCommand::Clear
                | SlashCommand::Resume
                | SlashCommand::Agents
                | SlashCommand::MultiAgents
                | SlashCommand::Quit
                | SlashCommand::Exit
                | SlashCommand::Status
                | SlashCommand::Warnings
                | SlashCommand::DebugConfig
                | SlashCommand::Pwd
                | SlashCommand::Rollout
                | SlashCommand::Copy
                | SlashCommand::Raw
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Delete
            | SlashCommand::Fork
            | SlashCommand::Worktree
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Recap
            | SlashCommand::Export
            | SlashCommand::Keymap
            | SlashCommand::Tui
            | SlashCommand::Vim
            | SlashCommand::ElevateSandbox
            | SlashCommand::Experimental
            | SlashCommand::Memories
            | SlashCommand::Import
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Cd
            | SlashCommand::Clear
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Archive
            | SlashCommand::Resume
            | SlashCommand::Model
            | SlashCommand::Presets
            | SlashCommand::Daybreak
            | SlashCommand::Permissions
            | SlashCommand::Copy
            | SlashCommand::Raw
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Hooks
            | SlashCommand::Status
            | SlashCommand::Daemon
            | SlashCommand::Warnings
            | SlashCommand::Pwd
            | SlashCommand::Usage
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Stop
            | SlashCommand::Goal
            | SlashCommand::Voice
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Plugins
            | SlashCommand::Title
            | SlashCommand::Statusline
            | SlashCommand::AutoReview
            | SlashCommand::Feedback
            | SlashCommand::Ide
            | SlashCommand::Quit
            | SlashCommand::Exit
            | SlashCommand::Side
            | SlashCommand::Btw => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Agents | SlashCommand::MultiAgents => true,
            SlashCommand::Config | SlashCommand::Theme | SlashCommand::Pets => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::Copy => !cfg!(target_os = "android"),
            SlashCommand::Voice => true,
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    use super::SlashCommand;

    #[test]
    fn stop_command_is_canonical_name() {
        assert_eq!(SlashCommand::Stop.command(), "stop");
    }

    #[test]
    fn clean_alias_parses_to_stop_command() {
        assert_eq!(SlashCommand::from_str("clean"), Ok(SlashCommand::Stop));
    }

    #[test]
    fn pet_alias_parses_to_pets_command() {
        assert_eq!(SlashCommand::Pets.command(), "pets");
        assert_eq!(SlashCommand::from_str("pet"), Ok(SlashCommand::Pets));
    }

    #[test]
    fn certain_commands_are_available_during_task() {
        assert!(SlashCommand::Goal.available_during_task());
        assert!(SlashCommand::Ide.available_during_task());
        assert!(SlashCommand::Title.available_during_task());
        assert!(SlashCommand::Statusline.available_during_task());
        assert!(SlashCommand::Raw.available_during_task());
        assert!(SlashCommand::Raw.available_in_side_conversation());
        assert!(SlashCommand::Raw.supports_inline_args());
    }

    #[test]
    fn auto_review_command_is_approve() {
        assert_eq!(SlashCommand::AutoReview.command(), "approve");
        assert_eq!(
            SlashCommand::from_str("approve"),
            Ok(SlashCommand::AutoReview)
        );
    }
}
