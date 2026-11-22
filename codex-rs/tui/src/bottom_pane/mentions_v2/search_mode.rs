// SPDX-FileCopyrightText: Copyright 2025 OpenAI
// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0
// Modified by Tianyu (Sky) Lu in 2026: made the interface translatable

use super::candidate::MentionType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SearchMode {
    Results,
    FilesystemOnly,
    Tools,
}

impl SearchMode {
    pub(super) fn previous(self) -> Self {
        match self {
            Self::Results => Self::Tools,
            Self::FilesystemOnly => Self::Results,
            Self::Tools => Self::FilesystemOnly,
        }
    }

    pub(super) fn next(self) -> Self {
        match self {
            Self::Results => Self::FilesystemOnly,
            Self::FilesystemOnly => Self::Tools,
            Self::Tools => Self::Results,
        }
    }

    pub(super) fn accepts(self, mention_type: MentionType) -> bool {
        match self {
            Self::Results => true,
            Self::FilesystemOnly => {
                matches!(mention_type, MentionType::File | MentionType::Directory)
            }
            Self::Tools => matches!(mention_type, MentionType::Plugin | MentionType::Skill),
        }
    }

    pub(super) fn label(self) -> &'static str {
        let label = match self {
            Self::Results => "All Results",
            Self::FilesystemOnly => "Filesystem Only",
            Self::Tools => "Plugins",
        };
        crate::i18n::translate(label, /*context*/ None)
    }
}
