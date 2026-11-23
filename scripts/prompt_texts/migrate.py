#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Moves prompts written in the Rust source into `prompts/`.

    migrate.py <PLAN_FILE>

Each line of the plan is `<source file>:<line>[#<n>]<TAB><text path>`: the string literal that
starts on that line of the file (the n-th there, counting from 1, when there are several) goes
to `prompts/<text path>`, and its place in the source becomes `codex_prompt_texts::text!`. A
literal read by `include_str!` is a prompt file already: the file is moved (`git mv`) and the
`include_str!` replaced. A format string that captured variables gets them as named arguments.
The crate gets the dependency, and `prompts/manifest.toml` an entry, which `merge.py` reads.
Blank lines and lines starting with `#` are skipped. Paths are from the repository's root.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from manifest import Entry, load, save  # noqa: E402
from rustlit import literals  # noqa: E402
from transform import apply, enclosing_item, mask, sha, site_of  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PROMPTS = ROOT / "prompts"
DEPENDENCY = "codex-prompt-texts = { workspace = true }"


def git(*args: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True, check=check)


def show(ref: str, path: str) -> str | None:
    result = git("show", f"{ref}:{path}", check=False)
    return result.stdout if result.returncode == 0 else None


def origin_of(path: str, source_text: str) -> str:
    """Where a prompt comes from: upstream Codex, ccn, or jjcn's own."""
    for ref, name in (("ccn/upstream", "upstream"), ("ccn/main", "ccn")):
        text = show(ref, path)
        if text is not None and source_text in text:
            return name
    return "jjcn"


def add_dependency(source: Path) -> None:
    crate = source.parent
    while not (crate / "Cargo.toml").exists():
        crate = crate.parent
    manifest = crate / "Cargo.toml"
    if crate.name == "prompt-texts":
        return
    text = manifest.read_text()
    if "codex-prompt-texts" in text:
        return
    lines = text.split("\n")
    try:
        at = lines.index("[dependencies]")
    except ValueError:
        lines += ["", "[dependencies]", DEPENDENCY]
    else:
        # Keep the section's order where it is sorted.
        end = at + 1
        while end < len(lines) and lines[end].strip() and not lines[end].startswith("["):
            end += 1
        body = lines[at + 1 : end]
        position = end
        for i, line in enumerate(body):
            if line.split("=")[0].strip() > "codex-prompt-texts":
                position = at + 1 + i
                break
        lines.insert(position, DEPENDENCY)
    manifest.write_text("\n".join(lines))


def main() -> int:
    plan_file = Path(sys.argv[1])
    rows: dict[str, list[tuple[int, int, str]]] = {}
    for number, raw in enumerate(plan_file.read_text().splitlines(), 1):
        raw = raw.strip()
        if not raw or raw.startswith("#"):
            continue
        where, text_path = raw.split("\t")
        source, _, line = where.rpartition(":")
        line, _, nth = line.partition("#")
        rows.setdefault(source, []).append((int(line), int(nth or 1), text_path.strip()))

    entries = load()
    failures = 0
    moved_to: dict[str, str] = {}  # files moved in this run, by their old place
    for source, wanted in sorted(rows.items()):
        path = ROOT / source
        src = path.read_text()
        lits = literals(src)
        masked = mask(src, lits)
        replacements = []
        for line, nth, text_path in wanted:
            on_line = [lit for lit in lits if src.count("\n", 0, lit.start) + 1 == line]
            if len(on_line) < nth:
                print(f"{source}:{line}: no string literal #{nth} starts on this line", file=sys.stderr)
                failures += 1
                continue
            lit = on_line[nth - 1]
            if lit.prefix:
                print(f"{source}:{line}: a {lit.prefix}\"\" literal is not text", file=sys.stderr)
                failures += 1
                continue
            site = site_of(src, masked, lit)
            target = PROMPTS / text_path
            target.parent.mkdir(parents=True, exist_ok=True)
            item = enclosing_item(masked, lit.start)
            entry = Entry(path=text_path, origin="", source=source, item=item)
            if site.include_path is not None:
                moved = (path.parent / site.include_path).resolve()
                relative = os.path.relpath(moved, ROOT)
                entry.moved_from = relative
                entry.origin = "upstream" if show("ccn/upstream", relative) is not None else (
                    "ccn" if show("ccn/main", relative) is not None else "jjcn"
                )
                if relative in moved_to:
                    # A second `include_str!` of a file this run moved already.
                    if moved_to[relative] != text_path:
                        print(f"{source}:{line}: {relative} went to {moved_to[relative]}", file=sys.stderr)
                        failures += 1
                        continue
                    entry.synced = sha(target.read_text())
                else:
                    if not moved.exists() or target.exists():
                        print(f"{source}:{line}: cannot move {relative} to {text_path}", file=sys.stderr)
                        failures += 1
                        continue
                    entry.synced = sha(moved.read_text())
                    git("mv", relative, os.path.relpath(target, ROOT))
                    moved_to[relative] = text_path
            else:
                # The same text in a second place may share the file: one entry for each place.
                if target.exists() and target.read_text() != site.value:
                    print(f"{text_path}: exists with other text", file=sys.stderr)
                    failures += 1
                    continue
                target.write_text(site.value)
                entry.origin = origin_of(source, src[lit.start : lit.end])
                entry.synced = sha(site.value)
            replacements.append((site, text_path))
            entries.append(entry)
        if replacements:
            path.write_text(apply(src, replacements))
            add_dependency(path)
    save(entries)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
