"""Shared helpers for the scripts that move the TUI's English strings into Fluent files.

`escape` turns a string into a Fluent pattern, `entry` formats one message, and `append` adds
messages to i18n/<language>/codex-tui.ftl, keeping ids unique.
"""

import pathlib
import re

I18N = pathlib.Path(__file__).parent.parent / "i18n"


def escape(text: str) -> str:
    """A Fluent pattern that renders exactly `text`, where `text` is the content of a Rust
    string literal (so `\\n`, `\\t`, `\\"` and `\\\\` are still escapes)."""
    unescaped = (
        text.replace("\\\\", "\0").replace("\\n", "\n").replace("\\t", "\t")
        .replace('\\"', '"').replace("\0", "\\")
    )
    out = []
    for index, char in enumerate(unescaped):
        if char in "{}":
            out.append('{"' + char + '"}')
        elif char == "\n":
            out.append('{"\\u000A"}')
        elif char == "\t":
            out.append('{"\\u0009"}')
        elif char == "\\":
            out.append('{"\\\\"}')
        elif char == " " and (index == 0 or index == len(unescaped) - 1):
            out.append('{" "}')
        elif index == 0 and char in "[*.":
            out.append('{"' + char + '"}')
        else:
            out.append(char)
    return "".join(out)


def entry(message_id: str, text: str) -> str:
    return f"{message_id} = {escape(text)}\n"


def existing_ids(language: str) -> set[str]:
    path = I18N / language / "codex-tui.ftl"
    if not path.exists():
        return set()
    return set(re.findall(r"^([a-z][a-z0-9-]*) =", path.read_text(), re.M))


def append(language: str, header: str, messages: list[tuple[str, str]]) -> None:
    path = I18N / language / "codex-tui.ftl"
    path.parent.mkdir(parents=True, exist_ok=True)
    known = existing_ids(language)
    lines = [f"\n## {header}\n\n"] if messages else []
    for message_id, text in messages:
        if message_id in known:
            raise SystemExit(f"duplicate message id {message_id} in {path}")
        known.add(message_id)
        lines.append(entry(message_id, text))
    with path.open("a") as file:
        file.writelines(lines)
