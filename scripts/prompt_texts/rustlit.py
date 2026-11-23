# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Finds the string literals of Rust source and gives their values.

Only what tells a string literal from the code around it is read: comments (nested block
comments too), character and byte literals, lifetimes, and the string forms (`"..."`,
`b"..."`, `c"..."` and their raw `r#"..."#` kinds). Doc comments are comments here.
"""

from __future__ import annotations

import re
from dataclasses import dataclass

_IDENT = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
_RAW = re.compile(r'(b|c)?r(#*)"')


@dataclass(frozen=True)
class Literal:
    """A string literal: `start`..`end` is its source text, `value` what it stands for."""

    start: int
    end: int
    value: str
    raw: bool
    prefix: str  # "", "b" or "c"


def _unescape(body: str) -> str:
    out = []
    i = 0
    n = len(body)
    while i < n:
        ch = body[i]
        if ch != "\\":
            out.append(ch)
            i += 1
            continue
        nxt = body[i + 1]
        if nxt == "\n" or (nxt == "\r" and body[i + 2 : i + 3] == "\n"):
            # A line continuation drops the line break and the whitespace after it.
            i += 2
            while i < n and body[i] in " \t\n\r":
                i += 1
            continue
        simple = {"n": "\n", "r": "\r", "t": "\t", "\\": "\\", "0": "\0", "'": "'", '"': '"'}
        if nxt in simple:
            out.append(simple[nxt])
            i += 2
        elif nxt == "x":
            out.append(chr(int(body[i + 2 : i + 4], 16)))
            i += 4
        elif nxt == "u":
            close = body.index("}", i)
            out.append(chr(int(body[i + 3 : close].replace("_", ""), 16)))
            i = close + 1
        else:
            raise ValueError(f"unknown escape \\{nxt}")
    return "".join(out)


def literals(src: str, chars: list[tuple[int, int]] | None = None) -> list[Literal]:
    """Every string literal of `src`, in order. The spans of character and byte literals are
    added to `chars` when it is given."""
    found: list[Literal] = []
    i = 0
    n = len(src)
    while i < n:
        ch = src[i]
        if src.startswith("//", i):
            nl = src.find("\n", i)
            i = n if nl < 0 else nl
            continue
        if src.startswith("/*", i):
            depth = 0
            while i < n:
                if src.startswith("/*", i):
                    depth += 1
                    i += 2
                elif src.startswith("*/", i):
                    depth -= 1
                    i += 2
                    if depth == 0:
                        break
                else:
                    i += 1
            continue
        if ch in "brc" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = _RAW.match(src, i)
            if m:
                hashes = m.group(2)
                body_start = m.end()
                close = src.index('"' + hashes, body_start)
                found.append(
                    Literal(i, close + 1 + len(hashes), src[body_start:close], True, m.group(1) or "")
                )
                i = close + 1 + len(hashes)
                continue
            if ch in "bc" and src[i + 1 : i + 2] == '"':
                end = _string_end(src, i + 2)
                found.append(Literal(i, end, _unescape(src[i + 2 : end - 1]), False, ch))
                i = end
                continue
            if ch == "b" and src[i + 1 : i + 2] == "'":
                end = _char_end(src, i + 1)
                if chars is not None:
                    chars.append((i, end))
                i = end
                continue
        if ch == '"':
            end = _string_end(src, i + 1)
            found.append(Literal(i, end, _unescape(src[i + 1 : end - 1]), False, ""))
            i = end
            continue
        if ch == "'":
            if src[i + 1 : i + 2] == "\\" or src[i + 2 : i + 3] == "'":
                end = _char_end(src, i)
                if chars is not None:
                    chars.append((i, end))
                i = end
                continue
            # A non-ASCII character literal, or a lifetime or label.
            if i + 2 < n and src[i + 1] != "'" and src.find("'", i + 1, i + 6) > 0:
                close = src.find("'", i + 1, i + 6)
                if len(src[i + 1 : close]) == 1:
                    if chars is not None:
                        chars.append((i, close + 1))
                    i = close + 1
                    continue
            m = _IDENT.match(src, i + 1)
            i = m.end() if m else i + 1
            continue
        if ch.isalpha() or ch == "_":
            m = _IDENT.match(src, i)
            i = m.end()
            continue
        i += 1
    return found


def _string_end(src: str, i: int) -> int:
    """The index after the closing quote of a string whose body starts at `i`."""
    while True:
        ch = src[i]
        if ch == "\\":
            i += 2
        elif ch == '"':
            return i + 1
        else:
            i += 1


def _char_end(src: str, i: int) -> int:
    """The index after a character literal whose opening quote is at `i`."""
    j = i + 1
    if src[j] == "\\":
        j += 2
        while src[j] != "'":
            j += 1
        return j + 1
    return j + 2
