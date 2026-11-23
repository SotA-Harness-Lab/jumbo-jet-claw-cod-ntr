# SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
# SPDX-License-Identifier: Apache-2.0

"""Tests of the prompt tools: `python3 -m unittest discover -s scripts/prompt_texts`."""

from __future__ import annotations

import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from rustlit import literals  # noqa: E402
from transform import apply, captures, locate, mask, site_of  # noqa: E402


class LiteralsTest(unittest.TestCase):
    def test_values_and_what_is_not_a_string(self):
        src = textwrap.dedent(
            r'''
            // "not a string"
            /* "nor /* this */ one" */
            fn f<'a>(x: &'a str) -> char { let c = '"'; let d = b'"'; let e = '\''; '{' }
            const A: &str = "tab\tquote\" \u{4e2d}\
                continued";
            const B: &str = r#"raw "quoted""#;
            const C: &[u8] = b"bytes";
            '''
        )
        values = [(lit.value, lit.prefix) for lit in literals(src)]
        self.assertEqual(
            values,
            [('tab\tquote" 中continued', ""), ('raw "quoted"', ""), ("bytes", "b")],
        )


class TransformTest(unittest.TestCase):
    def test_a_format_string_gets_the_variables_it_captured(self):
        src = 'fn f(x: u8) -> String { let who = 1; format!("{who} {x:?} {{y}} {0}", 3) }\n'
        lits = literals(src)
        site = site_of(src, mask(src, lits), lits[0])
        self.assertEqual(captures(site.value), ["who", "x"])
        self.assertEqual(
            apply(src, [(site, "a/b.md")]),
            'fn f(x: u8) -> String { let who = 1; '
            'format!(codex_prompt_texts::text!("a/b.md"), 3, who = who, x = x) }\n',
        )

    def test_an_include_str_is_replaced_whole(self):
        src = 'const P: &str = include_str!("../templates/p.md");\n'
        lits = literals(src)
        site = site_of(src, mask(src, lits), lits[0])
        self.assertEqual(site.include_path, "../templates/p.md")
        self.assertEqual(
            apply(src, [(site, "x/p.md")]), 'const P: &str = codex_prompt_texts::text!("x/p.md");\n'
        )

    def test_a_changed_literal_is_found_in_its_item(self):
        src = 'const A: &str = "other";\nfn g() { h("You are a helpful agent. Be brief.") }\n'
        site = locate(src, "g", like="You are a helpful agent. Be short.")
        self.assertEqual(site.value, "You are a helpful agent. Be brief.")
        self.assertIsNone(locate(src, "g", like="Something else entirely, not alike."))


def run(cwd: Path, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(args, cwd=cwd, capture_output=True, text=True, check=check)


PROMPT_BASE = (
    "You are reviewing a change. Read every file it touches.\n"
    "Report each bug with its file and line.\n"
    "Say nothing about style."
)


class MergeTest(unittest.TestCase):
    """jjcn moved a prompt to prompts/ and edited it; ccn changed it in place, and the code
    beside it. The merge conflicts in the Rust file; merge.py resolves it and merges ccn's
    change to the prompt with jjcn's."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp())
        self.repo = self.tmp / "repo"
        repo = self.repo
        (repo / "scripts").mkdir(parents=True)
        shutil.copytree(HERE, repo / "scripts" / "prompt_texts",
                        ignore=shutil.ignore_patterns("__pycache__", "reviewed.tsv"))
        (repo / "prompts").mkdir()
        src = repo / "codex-rs" / "x" / "src"
        src.mkdir(parents=True)
        (repo / "codex-rs" / "x" / "Cargo.toml").write_text('[package]\nname = "x"\n\n[dependencies]\n')
        self.source = src / "lib.rs"
        self.source.write_text(self.rust(PROMPT_BASE, "let limit = 3;"))
        run(repo, "git", "init", "-q", "-b", "ccn")
        run(repo, "git", "config", "user.email", "t@example.com")
        run(repo, "git", "config", "user.name", "t")
        run(repo, "git", "config", "commit.gpgsign", "false")
        run(repo, "git", "add", "-A")
        run(repo, "git", "commit", "-q", "-m", "base")
        run(repo, "git", "update-ref", "refs/remotes/ccn/main", "ccn")
        run(repo, "git", "switch", "-q", "-c", "jjcn")

    def tearDown(self):
        shutil.rmtree(self.tmp)

    @staticmethod
    def rust(prompt: str, line: str) -> str:
        literal = prompt.replace("\n", "\\n")
        return (
            "pub fn review() -> String {\n"
            f"    {line}\n"
            f'    format!("{literal} At most {{limit}} findings.")\n'
            "}\n"
        )

    def test_ccn_s_change_to_a_moved_prompt_is_merged_into_prompts(self):
        repo = self.repo
        plan = self.tmp / "plan"
        plan.write_text("codex-rs/x/src/lib.rs:3\tx/review/prompt.md\n")
        run(repo, sys.executable, "scripts/prompt_texts/migrate.py", str(plan))
        text = repo / "prompts" / "x" / "review" / "prompt.md"
        self.assertEqual(text.read_text(), PROMPT_BASE + " At most {limit} findings.")
        self.assertIn(
            'format!(codex_prompt_texts::text!("x/review/prompt.md"), limit = limit)',
            self.source.read_text(),
        )
        self.assertIn("codex-prompt-texts", (repo / "codex-rs/x/Cargo.toml").read_text())
        # jjcn edits the first line of the prompt.
        text.write_text(text.read_text().replace("Read every file", "Read each file"))
        run(repo, "git", "add", "-A")
        run(repo, "git", "commit", "-q", "-m", "jjcn")

        run(repo, "git", "switch", "-q", "ccn")
        changed = PROMPT_BASE.replace("Say nothing about style.", "Say nothing about naming.")
        self.source.write_text(self.rust(changed, "let limit = 5;"))
        run(repo, "git", "commit", "-q", "-am", "ccn")
        run(repo, "git", "switch", "-q", "jjcn")

        merge = run(repo, "git", "merge", "--no-ff", "ccn", check=False)
        self.assertNotEqual(merge.returncode, 0, "the Rust file should conflict")
        result = run(repo, sys.executable, "scripts/prompt_texts/merge.py", check=False)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(
            text.read_text(),
            PROMPT_BASE.replace("Read every file", "Read each file").replace(
                "about style", "about naming"
            )
            + " At most {limit} findings.",
        )
        merged = self.source.read_text()
        self.assertIn("let limit = 5;", merged)
        self.assertIn('codex_prompt_texts::text!("x/review/prompt.md")', merged)
        self.assertNotIn("<<<<<<<", merged)
        self.assertEqual(run(repo, "git", "diff", "--name-only", "--diff-filter=U").stdout, "")
        # A second run finds nothing more to do.
        again = run(repo, sys.executable, "scripts/prompt_texts/merge.py", check=False)
        self.assertEqual(again.returncode, 0, again.stdout)
        self.assertNotIn("prompts ccn changed", again.stdout)


if __name__ == "__main__":
    unittest.main()
