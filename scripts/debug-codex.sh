#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2025 OpenAI
# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0
# Modified by Tianyu (Sky) Lu in 2026: renamed the program


# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/codex/scripts/debug-codex.sh" in VSCode settings to always get the 
# latest codex-rs binary when debugging Codex Extension.


set -euo pipefail

CODEX_RS_DIR=$(realpath "$(dirname "$0")/../codex-rs")
(cd "$CODEX_RS_DIR" && cargo run --quiet --bin jjcn -- "$@")