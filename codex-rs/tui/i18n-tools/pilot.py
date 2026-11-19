"""Moves the slash commands' descriptions and the composer placeholders into Fluent messages:
rewrites the Rust literals into tr!("id") and appends the English and Chinese messages."""
import json
import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import ftl

tui = pathlib.Path(__file__).parent.parent
catalog = json.loads(pathlib.Path(sys.argv[1]).read_text())


def kebab(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "-", name).lower()


en, zh, missing = [], [], []


def message(message_id: str, text: str) -> None:
    en.append((message_id, text))
    if text in catalog:
        zh.append((message_id, catalog[text]))
    else:
        missing.append(text)


# Slash command descriptions: `SlashCommand::X => "text",` and the `=> {\n "text"\n }` arms.
path = tui / "src/slash_command.rs"
source = path.read_text()
start = source.index("pub fn description(self) -> &'static str {")
end = source.index("\n    }\n", start)
body = source[start:end]
arm = re.compile(r'SlashCommand::(\w+)((?:\s*\|\s*SlashCommand::\w+)*)\s*=>\s*(\{\s*)?"((?:[^"\\]|\\.)*)"(\s*\})?')


def replace(match: re.Match) -> str:
    message_id = "slash-" + kebab(match.group(1))
    message(message_id, match.group(4))
    return (
        f"SlashCommand::{match.group(1)}{match.group(2)} => "
        f'crate::i18n::tr!("{message_id}")'
    )


new_body = arm.sub(replace, body)
path.write_text(source[:start] + new_body + source[end:])

# Composer placeholders.
path = tui / "src/chatwidget.rs"
source = path.read_text()
for const, message_id in [("PLACEHOLDER", "composer-placeholder"), ("SIDE_PLACEHOLDER", "composer-side-placeholder")]:
    match = re.search(rf'^const {const}: &str = "((?:[^"\\]|\\.)*)";$', source, re.M)
    message(message_id, match.group(1))
    source = source.replace(match.group(0), f"fn {const.lower()}() -> &'static str {{\n    crate::i18n::tr!(\"{message_id}\")\n}}")
path.write_text(source)

ftl.append("en-US", "Slash commands and the composer", en)
ftl.append("zh-CN", "Slash commands and the composer", zh)
print(f"{len(en)} messages, {len(zh)} translated, missing: {missing}")
