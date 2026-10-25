#!/usr/bin/env python3
"""docs/capabilities.md and README.md's MCP sentence against the driver's schema tables: every verb and cause named, every count stated as the tables hold it. Run with `python3 -m unittest discover -s .github/scripts`."""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DRIVER = ROOT / "packages" / "escher-driver" / "src"
STATEMENT = ROOT / "docs" / "capabilities.md"
README = ROOT / "README.md"

NUMBER_WORDS = {
    1: "one",
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
    10: "ten",
    11: "eleven",
    12: "twelve",
    13: "thirteen",
    14: "fourteen",
    15: "fifteen",
    16: "sixteen",
    17: "seventeen",
    18: "eighteen",
    19: "nineteen",
    20: "twenty",
}


def read(path):
    return path.read_text(encoding="utf-8")


def prose(path):
    # A phrase may wrap across lines of the markdown source.
    return " ".join(read(path).split())


def number_word(count):
    if count not in NUMBER_WORDS:
        raise AssertionError(f"no number word for {count}: extend NUMBER_WORDS")
    return NUMBER_WORDS[count]


def verb_names():
    schema = read(DRIVER / "schema.rs")
    body = re.search(r"pub const VERBS: &\[VerbSpec\] = &\[(.*?)\];", schema, re.S)
    if body is None:
        raise AssertionError("VERBS not found in schema.rs")
    return [name.strip().lower() for name in body.group(1).split(",") if name.strip()]


def cause_names():
    refusal = read(DRIVER / "refusal.rs")
    body = re.search(r"pub const fn name\(self\) -> &'static str \{(.*?)\n    \}", refusal, re.S)
    if body is None:
        raise AssertionError("Cause::name not found in refusal.rs")
    return re.findall(r'Cause::\w+ => "([a-z-]+)"', body.group(1))


def table_length(path, pattern):
    found = re.search(pattern, read(path))
    if found is None:
        raise AssertionError(f"{pattern} not found in {path.name}")
    return int(found.group(1))


class CapabilityStatementTest(unittest.TestCase):
    def test_every_verb_is_named_as_a_code_span(self):
        statement = prose(STATEMENT)
        verbs = verb_names()
        self.assertTrue(verbs)
        for verb in verbs:
            self.assertTrue(f"`{verb}`" in statement, verb)

    def test_every_cause_is_named_as_a_code_span(self):
        statement = prose(STATEMENT)
        causes = cause_names()
        self.assertTrue(causes)
        for cause in causes:
            self.assertTrue(f"`{cause}`" in statement, cause)

    def test_the_table_counts_are_stated_in_words(self):
        statement = prose(STATEMENT)
        counts = {
            "verbs": len(verb_names()),
            "refusal causes": len(cause_names()),
            "session errors": table_length(DRIVER / "error.rs", r"SESSION_ERRORS: \[ErrorSpec; (\d+)\]"),
            "node keys": table_length(DRIVER / "schema.rs", r"NODE_FIELDS: \[NodeFieldSpec; (\d+)\]"),
            "exit statuses": table_length(DRIVER / "schema.rs", r"EXIT_STATUSES: \[ExitStatus; (\d+)\]"),
        }
        for noun, count in counts.items():
            self.assertTrue(f"{number_word(count)} {noun}" in statement, f"{number_word(count)} {noun}")

    def test_the_readme_counts_the_mcp_tools(self):
        phrase = f"the {number_word(len(verb_names()))} driver commands as tools"
        self.assertTrue(phrase in prose(README), phrase)


if __name__ == "__main__":
    unittest.main()
