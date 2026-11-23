#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Brings ccn's changes to prompts into `prompts/` while ccn is merged in.

    git merge --no-ff ccn/main      # stops with conflicts, or before committing
    scripts/prompt_texts/merge.py   # then commit what is left, after a look at its report

ccn keeps its prompts, and upstream Codex's, in the Rust source; jjcn has them in `prompts/`.
For each text of `prompts/manifest.toml` that ccn or upstream wrote, this finds it in the
version the merge starts from (by the hash the manifest keeps) and in ccn's (the literal most
like it in the same item), and when ccn changed it, merges that change into the file under
`prompts/` with `git merge-file`, as a change of either side to the same text. A Rust file
that the merge left in conflict is merged again with both of the other versions written as
jjcn's is, their prompts replaced by `text!`: what conflicted only because the prompt had
moved then merges cleanly, and the file is added. Prompt files that ccn moved or deleted,
and texts it could not find, are reported to be dealt with by hand. Last, it lists the long
literals that look like prompts in the files ccn changed, for `migrate.py`.

Run it from any directory of the repository, with the merge in progress (`MERGE_HEAD`). It
can be run more than once: a change already merged merges as nothing.
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from check import prompt_candidates  # noqa: E402
from manifest import Entry, load, save  # noqa: E402
from transform import apply, locate, sha  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
PROMPTS = ROOT / "prompts"


def git(*args: str, check: bool = True, input: str | None = None) -> subprocess.CompletedProcess:
    return subprocess.run(
        ["git", "-C", str(ROOT), *args], capture_output=True, text=True, check=check, input=input
    )


def show(ref: str, path: str) -> str | None:
    result = git("show", f"{ref}:{path}", check=False)
    return result.stdout if result.returncode == 0 else None


def merge_file(ours: Path, base: str, theirs: str, labels: tuple[str, str, str]) -> int:
    """Merges into `ours` what changed from `base` to `theirs`; the number of conflicts."""
    with tempfile.TemporaryDirectory() as tmp:
        base_file = Path(tmp) / "base"
        theirs_file = Path(tmp) / "theirs"
        base_file.write_text(base)
        theirs_file.write_text(theirs)
        result = subprocess.run(
            ["git", "merge-file", "-L", labels[0], "-L", labels[1], "-L", labels[2],
             str(ours), str(base_file), str(theirs_file)],
            capture_output=True, text=True,
        )
    if result.returncode < 0 or result.returncode > 127:
        raise RuntimeError(result.stderr)
    return result.returncode


def merge_text_into(
    ours: str, base: str, theirs: str, labels: tuple[str, str, str]
) -> tuple[str, int]:
    """`merge_file` for strings: the merged text, and the number of conflicts."""
    with tempfile.TemporaryDirectory() as tmp:
        ours_file = Path(tmp) / "ours"
        ours_file.write_text(ours)
        conflicts = merge_file(ours_file, base, theirs, labels)
        return ours_file.read_text(), conflicts


def normalized(src: str, entries: list[Entry], values: dict[int, str]) -> str:
    """A version of a source file with the prompts of `entries` replaced by `text!`, each
    found by the value it had in the version the merge starts from (`values`)."""
    replacements = []
    for entry in entries:
        site = (
            locate(src, entry.item, include_path=entry.moved_from)
            if entry.moved_from
            else locate(src, entry.item, value_sha=sha(values[id(entry)]), like=values[id(entry)])
        )
        if site is not None:
            replacements.append((site, entry.path))
    return apply(src, replacements)


def main() -> int:
    if git("rev-parse", "-q", "--verify", "MERGE_HEAD", check=False).returncode != 0:
        print("no merge in progress (MERGE_HEAD)", file=sys.stderr)
        return 2
    theirs_ref = "MERGE_HEAD"
    base_ref = git("merge-base", "HEAD", theirs_ref).stdout.strip()
    labels = ("jjcn", "base", "ccn")
    entries = load()
    problems: list[str] = []
    changed: list[str] = []
    base_values: dict[int, str] = {}
    new_synced: dict[int, str] = {}
    by_source: dict[str, list[Entry]] = defaultdict(list)

    for entry in entries:
        if entry.origin == "jjcn":
            continue
        by_source[entry.source].append(entry)
        if entry.moved_from:
            base_text = show(base_ref, entry.moved_from)
            theirs_text = show(theirs_ref, entry.moved_from)
            # The file as it was where the merge starts; a change git's rename detection
            # already brought into prompts/ merges again as nothing.
            base_text = base_text if base_text is not None else ""
            base_values[id(entry)] = base_text
            if theirs_text is None:
                if base_text:
                    problems.append(
                        f"{entry.moved_from}: ccn moved or deleted it; prompts/{entry.path} is "
                        f"its text here"
                    )
                continue
            if theirs_text != base_text:
                conflicts = merge_file(PROMPTS / entry.path, base_text, theirs_text, labels)
                changed.append(entry.path + (" (CONFLICT)" if conflicts else ""))
                new_synced[id(entry)] = sha(theirs_text)
            back = ROOT / entry.moved_from
            if back.exists():
                git("rm", "-q", "-f", "--", entry.moved_from, check=False)
            continue
        base_src = show(base_ref, entry.source)
        theirs_src = show(theirs_ref, entry.source)
        if base_src is None:
            continue
        base_site = locate(base_src, entry.item, value_sha=entry.synced)
        if base_site is None and theirs_src is not None:
            # A run before this one merged ccn's text already and recorded it as synced.
            theirs_site = locate(theirs_src, entry.item, value_sha=entry.synced)
            if theirs_site is not None:
                base_site = locate(base_src, entry.item, like=theirs_site.value)
                if base_site is not None:
                    base_values[id(entry)] = base_site.value
                    continue
        if base_site is None:
            problems.append(
                f"{entry.source} ({entry.item}): the text of prompts/{entry.path} is not in the "
                f"version the merge starts from; check the manifest's hash"
            )
            continue
        base_values[id(entry)] = base_site.value
        if theirs_src is None:
            problems.append(f"{entry.source}: ccn moved or deleted it (prompts/{entry.path})")
            continue
        theirs_site = locate(
            theirs_src, entry.item, value_sha=entry.synced, like=base_site.value
        )
        if theirs_site is None:
            problems.append(
                f"{entry.source} ({entry.item}): ccn rewrote or removed the text of "
                f"prompts/{entry.path}; compare by hand"
            )
            continue
        if theirs_site.value != base_site.value:
            conflicts = merge_file(PROMPTS / entry.path, base_site.value, theirs_site.value, labels)
            changed.append(entry.path + (" (CONFLICT)" if conflicts else ""))
            new_synced[id(entry)] = sha(theirs_site.value)

    unmerged = set(git("diff", "--name-only", "--diff-filter=U").stdout.split())
    resolved, still = [], []
    for source, source_entries in sorted(by_source.items()):
        if source not in unmerged:
            continue
        ours_src = show("HEAD", source)
        base_src = show(base_ref, source)
        theirs_src = show(theirs_ref, source)
        if ours_src is None or base_src is None or theirs_src is None:
            still.append(source)
            continue
        found = [e for e in source_entries if id(e) in base_values]
        merged, conflicts = merge_text_into(
            ours_src,
            normalized(base_src, found, base_values),
            normalized(theirs_src, found, base_values),
            labels,
        )
        (ROOT / source).write_text(merged)
        if conflicts:
            still.append(source)
        else:
            git("add", "--", source)
            resolved.append(source)

    for entry in entries:
        if id(entry) in new_synced:
            entry.synced = new_synced[id(entry)]
    save(entries)
    git("add", "--", "prompts/manifest.toml")

    files_changed = git("diff", "--name-only", base_ref, theirs_ref, "--", "codex-rs").stdout.split()
    candidates = prompt_candidates([f for f in files_changed if f.endswith(".rs")])

    def section(title: str, lines: list[str]) -> None:
        if lines:
            print(f"{title}:")
            for line in lines:
                print(f"  {line}")

    section("prompts ccn changed, merged into prompts/", changed)
    section("Rust files whose conflicts were only moved prompts, merged and added", resolved)
    section("Rust files still in conflict", still)
    section("to check by hand", problems)
    section("long literals in files ccn changed that may be prompts (migrate.py)", candidates)
    return 1 if still or problems or any("CONFLICT" in c for c in changed) else 0


if __name__ == "__main__":
    sys.exit(main())
