#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Checks that the prompts are where they belong.

    check.py                 # report, exit 1 when something is wrong
    check.py --record-rest   # take every literal now listed as a candidate for one reviewed

It reports a file of `prompts/` that no `text!` reads, a `text!` of a file that is not there,
an `include_str!` or `include_bytes!` of a file that is not there (in tests too: one that read
a prompt file before it moved to `prompts/` breaks only when its crate's tests are built), a
manifest entry whose source no longer reads its text, and long literals in the Rust source
(outside tests) that read like prose and are neither in `prompts/` nor reviewed: a prompt
written in place, or one a merge brought in. A literal reviewed and found not to be a prompt
(an SQL statement, a shell script, a message for the user) is kept in `reviewed.tsv`, by file
and hash of its text, and is not listed again until its text changes.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from manifest import load  # noqa: E402
from rustlit import literals  # noqa: E402
from transform import sha  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PROMPTS = ROOT / "prompts"
REVIEWED = Path(__file__).resolve().parent / "reviewed.tsv"
NOT_TEXTS = {"README.md", "manifest.toml", ".editorconfig"}
MIN_LENGTH = 160
_TEXT_REF = re.compile(r'codex_prompt_texts::text!\(\s*"([^"]+)"\s*\)')
# A file read at compile time by its path from the source file, or from the crate's root.
_INCLUDE = re.compile(
    r'include_(?:str|bytes)!\(\s*(?:"([^"]+)"'
    r'|concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]+)"\s*\))'
)


def _is_test_file(path: Path) -> bool:
    parts = path.parts
    return (
        "tests" in parts
        or "benches" in parts
        or path.name.endswith("_tests.rs")
        or path.name in ("tests.rs", "test.rs")
    )


def _looks_like_prose(text: str) -> bool:
    words = re.findall(r"[A-Za-z]{3,}", text)
    return len(text) >= MIN_LENGTH and len(words) * 16 >= len(text)


def reviewed() -> set[tuple[str, str]]:
    if not REVIEWED.exists():
        return set()
    rows = set()
    for line in REVIEWED.read_text().splitlines():
        if line and not line.startswith("#"):
            source, digest = line.split("\t")[:2]
            rows.add((source, digest))
    return rows


def _candidates_in(relative: str) -> list[tuple[str, int, str]]:
    path = ROOT / relative
    if not path.exists() or _is_test_file(path):
        return []
    src = path.read_text()
    cut = src.find("#[cfg(test)]\nmod tests {")
    out = []
    for lit in literals(src):
        if lit.prefix or (cut >= 0 and lit.start >= cut):
            continue
        if _looks_like_prose(lit.value):
            out.append((relative, src.count("\n", 0, lit.start) + 1, lit.value))
    return out


def prompt_candidates(files: list[str]) -> list[str]:
    """Long prose literals in `files` that are not reviewed, as `file:line  start of text`."""
    seen = reviewed()
    out = []
    for relative in files:
        for source, line, value in _candidates_in(relative):
            if (source, sha(value)) not in seen:
                out.append(f"{source}:{line}  {value[:70]!r}")
    return out


def rust_files() -> list[str]:
    return sorted(
        str(p.relative_to(ROOT))
        for p in (ROOT / "codex-rs").rglob("*.rs")
        if "target" not in p.parts and "node_modules" not in p.parts
    )


def main() -> int:
    files = rust_files()
    if "--record-rest" in sys.argv:
        rows = sorted(
            {(source, sha(value)) for f in files for source, _, value in _candidates_in(f)}
            | reviewed()
        )
        REVIEWED.write_text(
            "# Literals reviewed and found not to be prompts: file, hash of the text.\n"
            + "".join(f"{source}\t{digest}\n" for source, digest in rows)
        )
        print(f"{len(rows)} literals recorded as reviewed")
        return 0
    problems = []
    referenced: dict[str, set[str]] = {}
    for relative in files:
        text = (ROOT / relative).read_text()
        for path in _TEXT_REF.findall(text):
            referenced.setdefault(path, set()).add(relative)
            if not (PROMPTS / path).is_file():
                problems.append(f"{relative}: reads prompts/{path}, which is not there")
        for m in _INCLUDE.finditer(text):
            line = text.count("\n", 0, m.start()) + 1
            if m.group(1) is not None:
                target = (ROOT / relative).parent / m.group(1)
            else:
                crate = (ROOT / relative).parent
                while not (crate / "Cargo.toml").exists() and crate != ROOT:
                    crate = crate.parent
                target = crate / m.group(2).lstrip("/")
            if not target.exists():
                problems.append(
                    f"{relative}:{line}: includes {m.group(1) or m.group(2)}, which is not there"
                    " (moved to prompts/? read it with text!)"
                )
    for path in sorted(PROMPTS.rglob("*")):
        relative = str(path.relative_to(PROMPTS))
        if path.is_file() and path.name not in NOT_TEXTS and relative not in referenced:
            problems.append(f"prompts/{relative}: no text! reads it")
    for entry in load():
        if entry.source not in referenced.get(entry.path, set()):
            problems.append(f"manifest: {entry.source} does not read prompts/{entry.path}")
    candidates = prompt_candidates(files)
    for line in problems:
        print(line)
    if candidates:
        print("long literals that may be prompts (migrate.py, or --record-rest once reviewed):")
        for line in candidates:
            print(f"  {line}")
    return 1 if problems or candidates else 0


if __name__ == "__main__":
    sys.exit(main())
