# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""`prompts/manifest.toml`: where each text of `prompts/` was taken from.

One `[[text]]` table for each place in the source that reads a text:

- `path`: the file under `prompts/`.
- `origin`: `upstream` (Codex's), `ccn` or `jjcn`: who wrote it.
- `source`: the Rust file that reads it, `item` the const, static or fn it is in.
- `moved_from`: for a text that was a file already, where it was.
- `synced`: the hash of the text as the other side last had it, in its own place: the
  literal, or the file at `moved_from`. `merge.py` finds the text by it in the version a merge
  starts from, and sets it anew after each merge that brings a change.
"""

from __future__ import annotations

import json
import tomllib
from dataclasses import dataclass, fields
from pathlib import Path

FILE = Path(__file__).resolve().parents[2] / "prompts" / "manifest.toml"

HEADER = """\
# Where each text of this directory was taken from, kept by scripts/prompt_texts/migrate.py
# and merge.py: see scripts/prompt_texts/manifest.py. Do not edit by hand.
"""


@dataclass
class Entry:
    path: str
    origin: str
    source: str
    item: str | None = None
    moved_from: str | None = None
    synced: str | None = None


def load() -> list[Entry]:
    if not FILE.exists():
        return []
    data = tomllib.loads(FILE.read_text())
    return [Entry(**table) for table in data.get("text", [])]


def save(entries: list[Entry]) -> None:
    out = [HEADER]
    for entry in sorted(entries, key=lambda e: (e.path, e.source, e.item or "")):
        out.append("\n[[text]]\n")
        for field in fields(Entry):
            value = getattr(entry, field.name)
            if value is not None:
                out.append(f"{field.name} = {json.dumps(value, ensure_ascii=False)}\n")
    FILE.write_text("".join(out))
