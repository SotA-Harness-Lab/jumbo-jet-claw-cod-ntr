# Prompts

Every text the program gives a model is a file here: the base and developer instructions,
the descriptions of tools, the messages the harness adds to a conversation, and the requests
it makes of models on its own (compaction, reviews, hook judges). The program reads them
when it is built, with `codex_prompt_texts::text!("<path>")` (`codex-rs/prompt-texts`), so
a change here takes effect at the next build, and nothing is read at run time.

## Layout

`<crate>/<module>/<name>.<ext>`: the crate (its directory under `codex-rs/`) and module of
the code that reads the text, and what the text is. A text that was a file already keeps the
place it had under its crate's `templates/`; those of the `codex-rs/prompts` crate are under
`templates/`. `manifest.toml` lists each text with the code that reads it and whether it came
from upstream Codex, from ccn or from jjcn.

## Editing

- A file is built in byte for byte: a final line break or trailing space is part of the text.
  `.editorconfig` keeps editors that read it from adding or removing them.
- A text used as a format string (the code passes it to `format!`) has placeholders such as
  `{name}` or `{}` and writes a literal brace as `{{` or `}}`. A text the code fills with
  `codex_utils_template` has `{{ name }}` placeholders instead. Look at the code that reads
  a file (`manifest.toml`) before adding or removing a placeholder.
- A new text goes here, never into the source: `scripts/prompt_texts/migrate.py` moves one
  written in the source, and `scripts/prompt_texts/check.py` lists those left there.

## Merging ccn

ccn keeps these texts in the Rust source. After `git merge --no-ff ccn/main`, run
`scripts/prompt_texts/merge.py`: it carries ccn's changes to the texts into these files and
resolves the conflicts that only come from a text having moved here.
