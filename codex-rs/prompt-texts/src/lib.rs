// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! The texts the program gives models (instructions, tool descriptions, the messages it adds
//! to a conversation), kept as files in the repository's `prompts/` directory and built into
//! the program by [`text!`]. `prompts/README.md` says how the directory is laid out.

include!(concat!(env!("OUT_DIR"), "/text_macro.rs"));
