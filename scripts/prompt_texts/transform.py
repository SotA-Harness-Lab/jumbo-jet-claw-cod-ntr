# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Turns a prompt written in Rust source into a `text!` of `prompts/`, and finds it again.

The migration (`migrate.py`) and the merge (`merge.py`) make the same change with the same
code: a literal becomes `codex_prompt_texts::text!("<path>")`, an `include_str!` of a prompt
file likewise, and a format string that captured variables gets them as named arguments,
which a format string from a macro needs.
"""

from __future__ import annotations

import difflib
import hashlib
import re
from dataclasses import dataclass

from rustlit import Literal, literals

TEXT_MACRO = "codex_prompt_texts::text!"

# Macros whose first argument (after the writer, for the `write` kind) is a format string.
FORMAT_MACROS = {
    "format", "format_args", "print", "println", "eprint", "eprintln", "panic", "anyhow",
    "bail", "unreachable", "todo", "unimplemented",
}
WRITER_FORMAT_MACROS = {"write", "writeln"}

_ITEM = re.compile(
    r"\b(?:(?:const|unsafe|async|extern)\s+)*(fn)\s+([A-Za-z_][A-Za-z0-9_]*)"
    r"|\b(?:const|static)\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:"
)
_PLACEHOLDER = re.compile(r"\{\{|\}\}|\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}")


def sha(text: str) -> str:
    return hashlib.sha256(text.encode()).hexdigest()[:16]


def mask(src: str, lits: list[Literal] | None = None) -> str:
    """`src` with comments and the insides of literals blanked out, the same length, so that
    brackets and keywords can be found without reading into text."""
    out = list(src)
    chars: list[tuple[int, int]] = []
    found = literals(src, chars)
    for lit in lits if lits is not None else found:
        for i in range(lit.start + 1, lit.end - 1):
            if out[i] != "\n":
                out[i] = " "
    for start, end in chars:
        for i in range(start + 1, end - 1):
            out[i] = " "
    for m in re.finditer(r"//[^\n]*", "".join(out)):
        for i in range(m.start(), m.end()):
            out[i] = " "
    # Block comments are rare in this code; blank them too.
    text = "".join(out)
    for m in re.finditer(r"/\*.*?\*/", text, re.S):
        for i in range(m.start(), m.end()):
            if out[i] != "\n":
                out[i] = " "
    return "".join(out)


def _matching(masked: str, open_at: int) -> int:
    """The index of the bracket that closes the one at `open_at`."""
    pairs = {"(": ")", "[": "]", "{": "}"}
    stack = []
    for i in range(open_at, len(masked)):
        ch = masked[i]
        if ch in pairs:
            stack.append(pairs[ch])
        elif ch in ")]}":
            if not stack or stack.pop() != ch:
                raise ValueError(f"unbalanced bracket at {i}")
            if not stack:
                return i
    raise ValueError("unclosed bracket")


def item_spans(masked: str) -> list[tuple[str, int, int]]:
    """Every `const`, `static` and `fn` item: its name and the span of its whole text."""
    spans = []
    for m in _ITEM.finditer(masked):
        is_fn = m.group(1) is not None
        name = m.group(2) if is_fn else m.group(3)
        i = m.end()
        # A const or static ends at its `;`; a fn at the brace that closes its body.
        while i < len(masked) and masked[i] not in ";{":
            if masked[i] in "([":
                i = _matching(masked, i)
            i += 1
        if i >= len(masked):
            continue
        end = i + 1 if masked[i] == ";" else _matching(masked, i) + 1
        if masked[i] == "{" and not is_fn:
            # `const X: T = T { .. };`: go on to the `;`.
            j = end
            while j < len(masked) and masked[j] != ";":
                j += 1
            end = j + 1
        spans.append((name, m.start(), end))
    return spans


def enclosing_item(
    masked: str, pos: int, spans: list[tuple[str, int, int]] | None = None
) -> str | None:
    """The name of the innermost item that holds `pos` (`spans`: `item_spans(masked)`)."""
    best = None
    for name, start, end in spans if spans is not None else item_spans(masked):
        if start <= pos < end and (best is None or start >= best[1]):
            best = (name, start)
    return best[0] if best else None


@dataclass(frozen=True)
class Site:
    """Where a prompt is in a version of a source file, and what replaces it."""

    start: int
    end: int
    value: str
    # Named arguments to add at `args_at`, the closing bracket of a format macro's call.
    captures: tuple[str, ...]
    args_at: int | None
    include_path: str | None  # the file an `include_str!` read, for a moved prompt file


def _format_call(src: str, masked: str, lit: Literal) -> tuple[int, set[str]] | None:
    """When `lit` is the format string of a format macro: the closing bracket of the call and
    the names of the arguments it already names."""
    before = masked[: lit.start].rstrip()
    m = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*!\s*\(\s*$", before)
    if m and m.group(1) in FORMAT_MACROS:
        open_at = before.rfind("(")
    else:
        # `write!(f, "...")`: the literal follows the writer.
        if not before.endswith(","):
            return None
        depth = 0
        i = len(before) - 2
        while i >= 0:
            ch = masked[i]
            if ch in ")]}":
                depth += 1
            elif ch in "([{":
                if depth == 0:
                    break
                depth -= 1
            elif ch == "," and depth == 0:
                return None
            i -= 1
        if i < 0 or masked[i] != "(":
            return None
        m = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*!\s*$", masked[:i])
        if not m or m.group(1) not in WRITER_FORMAT_MACROS:
            return None
        open_at = i
    close = _matching(masked, open_at)
    args = masked[lit.end : close]
    named = set(re.findall(r"(?:^|,)\s*([A-Za-z_][A-Za-z0-9_]*)\s*=(?!=)", args))
    return close, named


def captures(fmt: str) -> list[str]:
    """The variables a format string names in its placeholders, in order, once each."""
    names = []
    for m in _PLACEHOLDER.finditer(fmt):
        name = m.group(1)
        if name and name not in names:
            names.append(name)
    return names


def _concat_of(src: str, masked: str, lit: Literal) -> tuple[int, int, str] | None:
    """When `lit` is the first argument of a `concat!` of literals only: its span and value."""
    before = masked[: lit.start]
    m = re.search(r"(?:::)?(?:core::|std::)?concat\s*!\s*\(\s*$", before)
    if not m:
        return None
    close = _matching(masked, before.rfind("("))
    parts = literals(src[m.end() - 1 : close + 1])
    # What is left between the brackets once the literals are blanked: commas and space only.
    rest = masked[m.end() : close]
    for part in parts:
        rest = rest[: part.start - 1] + " " * (part.end - part.start) + rest[part.end - 1 :]
    if re.sub(r"[\s,]", "", rest):
        return None
    return m.start(), close + 1, "".join(part.value for part in parts)


def site_of(src: str, masked: str, lit: Literal) -> Site:
    """What replacing `lit` takes: the literal itself, the `concat!` of literals it starts, or
    the `include_str!` it is read by."""
    concat = _concat_of(src, masked, lit)
    if concat:
        start, end, value = concat
        outer = Literal(start, end, value, False, "")
        call = _format_call(src, masked, outer)
        if call:
            close, named = call
            missing = tuple(name for name in captures(value) if name not in named)
            return Site(start, end, value, missing, close if missing else None, None)
        return Site(start, end, value, (), None, None)
    before = masked[: lit.start]
    m = re.search(r"(?:::)?(?:core|std)?(?:::)?include_str\s*!\s*\(\s*$", before)
    if m:
        close = _matching(masked, before.rfind("("))
        return Site(m.start(), close + 1, lit.value, (), None, lit.value)
    call = _format_call(src, masked, lit)
    if call:
        close, named = call
        missing = tuple(name for name in captures(lit.value) if name not in named)
        return Site(lit.start, lit.end, lit.value, missing, close if missing else None, None)
    return Site(lit.start, lit.end, lit.value, (), None, None)


def apply(src: str, replacements: list[tuple[Site, str]]) -> str:
    """`src` with each site replaced by `text!` of its path, the last first."""
    edits = []
    for site, path in replacements:
        edits.append((site.start, site.end, f'{TEXT_MACRO}("{path}")'))
        if site.args_at is not None:
            named = [f"{name} = {name}" for name in site.captures]
            if src[: site.args_at].rstrip().endswith(","):
                # The call already ends with a comma: add after it, ending with one too.
                args = " " + ", ".join(named) + ","
            else:
                args = "".join(", " + arg for arg in named)
            edits.append((site.args_at, site.args_at, args))
    out = src
    for start, end, new in sorted(edits, key=lambda e: (e[0], e[1]), reverse=True):
        out = out[:start] + new + out[end:]
    return out


def locate(
    src: str,
    item: str | None,
    *,
    value_sha: str | None = None,
    like: str | None = None,
    include_path: str | None = None,
) -> Site | None:
    """The prompt of an entry in one version of its source file: the literal in `item` whose
    value has the hash `value_sha`, or else the one most like `like` (at least 60% alike),
    or the `include_str!` of `include_path`."""
    lits = literals(src)
    masked = mask(src, lits)
    spans = [(start, end) for name, start, end in item_spans(masked) if name == item]
    candidates = [
        lit for lit in lits if not spans or any(start <= lit.start < end for start, end in spans)
    ]
    for lit in candidates:
        site = site_of(src, masked, lit)
        if include_path is not None:
            if site.include_path is not None and (
                site.include_path == include_path
                or site.include_path.endswith("/" + include_path.rsplit("/", 1)[-1])
            ):
                return site
            continue
        if value_sha is not None and sha(site.value) == value_sha:
            return site
    if like is None or include_path is not None:
        return None
    best = None
    for lit in candidates:
        site = site_of(src, masked, lit)
        if site.include_path is not None or abs(len(site.value) - len(like)) > max(len(like), 80):
            continue
        ratio = difflib.SequenceMatcher(None, site.value, like, autojunk=False).ratio()
        if ratio >= 0.6 and (best is None or ratio > best[0]):
            best = (ratio, site)
    return best[1] if best else None
