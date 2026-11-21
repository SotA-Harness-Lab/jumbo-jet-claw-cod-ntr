// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Text that exec shows on a terminal comes from the model and from the
//! commands it runs. Escape sequences in it other than text styling (window
//! title, clipboard writes, cursor movement, erasing) would act on the user's
//! terminal, so they are removed before the text is printed there.

use std::borrow::Cow;

const ESC: char = '\u{1b}';
const BEL: char = '\u{7}';
/// C1 String Terminator.
const ST: char = '\u{9c}';

/// Returns `text` as it may be printed to a stream: unchanged when the stream
/// is not a terminal (pipes and files get the text as it is), with every
/// control sequence but text styling removed when it is.
pub(crate) fn for_terminal(text: &str, is_terminal: bool) -> Cow<'_, str> {
    if is_terminal {
        keep_only_styling(text)
    } else {
        Cow::Borrowed(text)
    }
}

/// Removes escape sequences and control characters from `text`, keeping SGR
/// sequences (`ESC [ <digits;:> m`), newlines and tabs. A carriage return is
/// kept only as part of a `\r\n` line ending.
pub(crate) fn keep_only_styling(text: &str) -> Cow<'_, str> {
    if !text.chars().any(is_control) {
        return Cow::Borrowed(text);
    }

    let mut out = String::with_capacity(text.len());
    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        match c {
            '\n' | '\t' => out.push(c),
            '\r' => {
                if matches!(chars.peek(), Some((_, '\n'))) {
                    out.push(c);
                }
            }
            ESC => match chars.peek().map(|&(_, next)| next) {
                Some('[') => {
                    chars.next();
                    if let Some(end) = skip_csi(&mut chars)
                        && is_sgr(&text[start..end])
                    {
                        out.push_str(&text[start..end]);
                    }
                }
                Some(']' | 'P' | 'X' | '^' | '_') => {
                    chars.next();
                    skip_control_string(&mut chars);
                }
                Some('\u{20}'..='\u{2f}') => {
                    // nF sequence: intermediates, then one final byte.
                    while matches!(chars.peek(), Some((_, '\u{20}'..='\u{2f}'))) {
                        chars.next();
                    }
                    if matches!(chars.peek(), Some((_, '\u{30}'..='\u{7e}'))) {
                        chars.next();
                    }
                }
                Some('\u{30}'..='\u{7e}') => {
                    chars.next();
                }
                _ => {}
            },
            '\u{9b}' => {
                skip_csi(&mut chars);
            }
            '\u{90}' | '\u{98}' | '\u{9d}' | '\u{9e}' | '\u{9f}' => {
                skip_control_string(&mut chars);
            }
            c if is_control(c) => {}
            c => out.push(c),
        }
    }
    Cow::Owned(out)
}

fn is_control(c: char) -> bool {
    matches!(c, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}') && c != '\n' && c != '\t'
}

/// Consumes the parameters, intermediates and final byte of a CSI sequence
/// whose introducer was already consumed. Returns the byte offset just past
/// the final byte, or `None` when the sequence is cut short; a character that
/// cannot be part of it is left for the caller.
fn skip_csi(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>) -> Option<usize> {
    while let Some(&(index, c)) = chars.peek() {
        match c {
            '\u{20}'..='\u{3f}' => {
                chars.next();
            }
            '\u{40}'..='\u{7e}' => {
                chars.next();
                return Some(index + c.len_utf8());
            }
            _ => return None,
        }
    }
    None
}

fn is_sgr(sequence: &str) -> bool {
    sequence
        .strip_prefix("\u{1b}[")
        .and_then(|rest| rest.strip_suffix('m'))
        .is_some_and(|params| {
            params
                .chars()
                .all(|c| c.is_ascii_digit() || c == ';' || c == ':')
        })
}

/// Consumes an OSC, DCS, SOS, PM or APC string up to and including its
/// terminator (BEL, `ESC \` or ST), or to the end of the text.
fn skip_control_string(chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>) {
    while let Some((_, c)) = chars.next() {
        match c {
            BEL | ST => return,
            ESC => {
                if matches!(chars.peek(), Some((_, '\\'))) {
                    chars.next();
                }
                return;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "terminal_text_tests.rs"]
mod tests;
