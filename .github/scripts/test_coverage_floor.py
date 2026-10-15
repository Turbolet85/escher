#!/usr/bin/env python3
"""Tests for coverage_floor.py. Run with `python3 -m unittest discover -s .github/scripts`."""

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parent / "coverage_floor.py"


def record(path, found, hit):
    return f"SF:{path}\nDA:1,1\nLF:{found}\nLH:{hit}\nend_of_record\n"


class CoverageFloorTest(unittest.TestCase):
    def floor(self, lcov_text, *floors):
        """Runs the script in a temp working directory over `lcov_text`; returns exit, stdout lines, stderr."""
        with tempfile.TemporaryDirectory() as tmp:
            lcov = Path(tmp) / "lcov.info"
            lcov.write_text(lcov_text.format(root=tmp), encoding="utf-8")
            proc = subprocess.run(
                [sys.executable, str(SCRIPT), str(lcov), *floors], cwd=tmp, capture_output=True, text=True
            )
            return proc.returncode, proc.stdout.splitlines(), proc.stderr

    LCOV = (
        record("{root}/packages/driver/src/lib.rs", 80, 78)
        + record("{root}/packages/driver/src/cli.rs", 20, 19)
        + record("{root}/packages/dom/src/lib.rs", 200, 160)
        + record("{root}/packages/driver-extra/src/lib.rs", 50, 0)
    )

    def test_a_crate_above_its_floor_passes(self):
        code, lines, _ = self.floor(self.LCOV, "packages/driver=96")
        self.assertEqual(code, 0)
        self.assertEqual(lines[0], "packages/driver 97/100 = 97.00 % (floor 96 %)")
        self.assertEqual(lines[-1], "coverage floor: 1 of 1 crates at or above their floors")

    def test_a_crate_at_its_floor_passes(self):
        code, lines, _ = self.floor(self.LCOV, "packages/driver=97")
        self.assertEqual(code, 0)
        self.assertEqual(lines[-1], "coverage floor: 1 of 1 crates at or above their floors")

    def test_a_crate_under_its_floor_fails(self):
        code, lines, _ = self.floor(self.LCOV, "packages/driver=97.5")
        self.assertEqual(code, 1)
        self.assertEqual(lines[0], "packages/driver 97/100 = 97.00 % (floor 97.5 %)")
        self.assertEqual(lines[-1], "coverage floor: 1 of 1 crates under their floors")

    def test_two_crates_are_read_apart(self):
        # A sibling directory sharing the crate's name as a prefix is not under it.
        code, lines, _ = self.floor(self.LCOV, "packages/driver=97", "packages/dom=81")
        self.assertEqual(code, 1)
        self.assertEqual(
            lines,
            [
                "packages/driver 97/100 = 97.00 % (floor 97 %)",
                "packages/dom 160/200 = 80.00 % (floor 81 %)",
                "coverage floor: 1 of 2 crates under their floors",
            ],
        )

    def test_two_crates_both_holding_pass(self):
        code, lines, _ = self.floor(self.LCOV, "packages/driver=97", "packages/dom=80")
        self.assertEqual(code, 0)
        self.assertEqual(lines[-1], "coverage floor: 2 of 2 crates at or above their floors")

    def test_an_absent_crate_exits_2_naming_it(self):
        code, lines, stderr = self.floor(self.LCOV, "packages/driver=97", "packages/absent=50")
        self.assertEqual(code, 2)
        self.assertEqual(lines, [])
        self.assertIn("packages/absent", stderr)

    def test_a_malformed_call_exits_2_with_usage(self):
        for name, floors in (
            ("no floor", ()),
            ("no percent", ("packages/driver",)),
            ("a word for a percent", ("packages/driver=high",)),
            ("over a hundred", ("packages/driver=101",)),
            ("a crate twice", ("packages/driver=90", "packages/driver=91")),
        ):
            with self.subTest(call=name):
                code, lines, stderr = self.floor(self.LCOV, *floors)
                self.assertEqual(code, 2)
                self.assertEqual(lines, [])
                self.assertIn("usage:", stderr)


if __name__ == "__main__":
    unittest.main()
