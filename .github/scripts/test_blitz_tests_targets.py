#!/usr/bin/env python3
"""Target shape of tests/blitz-tests/Cargo.toml: one test target per file, upstream's `all` unbuilt. Run with `python3 -m unittest discover -s .github/scripts`."""

import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tests" / "blitz-tests" / "Cargo.toml"


def load_manifest():
    with open(MANIFEST, "rb") as f:
        return tomllib.load(f)


class BlitzTestsTargetsTest(unittest.TestCase):
    def test_every_file_is_a_target_and_all_is_unbuilt(self):
        manifest = load_manifest()
        # Upstream (#1123) sets `autotests = false` and builds tests/all.rs alone. The fork keeps
        # auto-discovery: the a11y leg and agent-run.sh select a file by `--test {name}`.
        self.assertIsNot(manifest["package"].get("autotests"), False)
        self.assertEqual(
            manifest.get("test"),
            [{"name": "all", "path": "tests/all.rs", "test": False}],
        )


if __name__ == "__main__":
    unittest.main()
