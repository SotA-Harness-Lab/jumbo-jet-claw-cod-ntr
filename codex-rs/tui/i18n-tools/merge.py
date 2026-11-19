# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Merges the staged message files (staged/NN.<language>.ftl) into i18n/<language>/codex-tui.ftl.

Checks before writing anything: each staged file parses as Fluent, no id is defined twice (in
the staged files or against the messages already merged), and every English message has a
Chinese one using only its variables (a
Chinese message may leave out a variable, such as the count of an English plural). Run it with the Fluent parser available:

    uv run --with fluent.syntax python codex-rs/tui/i18n-tools/merge.py

On success the merged staged files are deleted; on any problem nothing is changed.
"""

import pathlib
import sys

from fluent.syntax import FluentParser
from fluent.syntax import ast
from fluent.syntax.visitor import Visitor

TOOLS = pathlib.Path(__file__).parent
I18N = TOOLS.parent / "i18n"
STAGED = TOOLS / "staged"
LANGUAGES = ("en-US", "zh-CN")


def messages(text: str, origin: str, problems: list[str]) -> dict[str, set[str]]:
    """Message id -> the variables it uses, with parse errors added to `problems`."""
    resource = FluentParser(with_spans=False).parse(text)
    found: dict[str, set[str]] = {}
    for entry in resource.body:
        if isinstance(entry, ast.Junk):
            problems.append(f"{origin}: does not parse: {entry.content.strip()[:120]!r}")
        elif isinstance(entry, ast.Message):
            if entry.id.name in found:
                problems.append(f"{origin}: {entry.id.name} defined twice")
            variables: set[str] = set()

            class Collect(Visitor):
                def visit_VariableReference(self, node):  # noqa: N802 (visitor naming)
                    variables.add(node.id.name)

            Collect().visit(entry)
            found[entry.id.name] = variables
    return found


def main() -> int:
    problems: list[str] = []
    merged = {
        language: messages((I18N / language / "codex-tui.ftl").read_text(), language, problems)
        for language in LANGUAGES
    }
    groups = sorted({path.name.split(".")[0] for path in STAGED.glob("*.ftl")})
    staged_text = {}
    for group in groups:
        group_messages = {}
        for language in LANGUAGES:
            path = STAGED / f"{group}.{language}.ftl"
            if not path.exists():
                problems.append(f"{path.name}: missing")
                continue
            text = path.read_text()
            staged_text[(group, language)] = text
            group_messages[language] = messages(text, path.name, problems)
            for message_id in group_messages[language]:
                if message_id in merged[language]:
                    problems.append(f"{path.name}: {message_id} is already defined")
            merged[language].update(group_messages[language])
        english = group_messages.get("en-US", {})
        chinese = group_messages.get("zh-CN", {})
        for message_id, variables in english.items():
            if message_id not in chinese:
                problems.append(f"{group}: {message_id} has no Chinese message")
            elif not chinese[message_id] <= variables:
                problems.append(
                    f"{group}: {message_id} variables differ: {sorted(variables)} vs "
                    f"{sorted(chinese[message_id])}"
                )
        for message_id in chinese.keys() - english.keys():
            problems.append(f"{group}: {message_id} has no English message")
    if problems:
        print("\n".join(problems))
        print(f"{len(problems)} problems; nothing merged")
        return 1
    for (group, language), text in sorted(staged_text.items()):
        with (I18N / language / "codex-tui.ftl").open("a") as file:
            file.write("\n" + text.rstrip("\n") + "\n")
    for path in STAGED.glob("*.ftl"):
        path.unlink()
    print(f"merged {len(groups)} groups; {len(merged['en-US'])} messages in total")
    return 0


if __name__ == "__main__":
    sys.exit(main())
