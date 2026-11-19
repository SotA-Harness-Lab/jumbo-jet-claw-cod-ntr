# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Rewrites one-line Fluent selectors (`{ $n -> [one] a *[other] b }`) in the given .ftl files
into the multiline form Fluent requires, each variant on its own line."""
import re
import sys


def expand(line: str) -> str:
    out, i = [], 0
    while True:
        start = line.find("{ $", i)
        arrow = line.find(" -> ", start) if start != -1 else -1
        if start == -1 or arrow == -1:
            out.append(line[i:])
            return "".join(out)
        if "}" in line[start:arrow]:
            # A plain placeable before the selector: keep it and look further on.
            close = line.index("}", start) + 1
            out.append(line[i:close])
            i = close
            continue
        # Find the selector's closing brace, skipping nested placeables.
        depth, j = 0, start
        while j < len(line):
            if line[j] == "{":
                depth += 1
            elif line[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        selector = line[start + 2 : arrow].strip()
        body = line[arrow + 4 : j].strip()
        variants = re.split(r"\s+(?=\*?\[[a-z0-9-]+\])", body)
        lines = [f"{{ {selector} ->"]
        for variant in variants:
            indent = "       " if variant.startswith("*") else "        "
            lines.append(indent + variant.strip())
        lines.append("    }")
        out.append(line[i:start] + "\n".join(lines))
        i = j + 1


for path in sys.argv[1:]:
    text = open(path).read()
    new = "\n".join(expand(line) if " -> [" in line else line for line in text.split("\n"))
    if new != text:
        open(path, "w").write(new)
        print("rewrote", path)
