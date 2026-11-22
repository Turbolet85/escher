#!/usr/bin/env python3
"""Checks on skia_archive.py and the committed archive record. Run with `python3 -m unittest discover .github/scripts`."""

import contextlib
import hashlib
import io
import re
import tempfile
import unittest
import urllib.request
from pathlib import Path

import skia_archive
from test_cargo_lock import CARGO_LOCK, parse_lock

TARGET = "x86_64-unknown-linux-gnu"
KEY = "0000-test-key"
BODY = b"archive bytes"
RECORDED_TARGETS = {
    "x86_64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
    "x86_64-pc-windows-msvc",
    "aarch64-linux-android",
}


class Opener:
    """Fails its first `failures` calls, then reads the URL through urllib; counts every call."""

    def __init__(self, failures=0):
        self.failures = failures
        self.calls = 0

    def __call__(self, url, timeout):
        self.calls += 1
        if self.calls <= self.failures:
            raise OSError("unreachable")
        return urllib.request.urlopen(url, timeout=timeout)


class Fixture:
    """A source directory serving one archive by `file://`, an output directory, and a record naming both."""

    def __init__(self, tmp, body=BODY, recorded=BODY):
        self.source = Path(tmp) / "source"
        self.source.mkdir()
        (self.source / f"skia-binaries-{KEY}.tar.gz").write_bytes(body)
        self.out = Path(tmp) / "out"
        self.record_path = Path(tmp) / "record.toml"
        self.sha = hashlib.sha256(recorded).hexdigest()
        url = (self.source / "skia-binaries-{key}.tar.gz").resolve().as_posix()
        self.record_path.write_text(
            f'tag = "1.0.0"\nurl = "file://{url}"\n\n[targets."{TARGET}"]\nkey = "{KEY}"\nsha256 = "{self.sha}"\n',
            encoding="utf-8",
        )
        self.record = skia_archive.load_record(self.record_path)
        self.archive = self.out / f"skia-binaries-{KEY}.tar.gz"

    def run(self, argv=None, opener=None):
        """Runs main; returns its exit, stdout and stderr."""
        stdout, stderr = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            code = skia_archive.main(
                argv if argv is not None else [TARGET, str(self.out)],
                record_path=self.record_path,
                opener=opener or Opener(),
                pause=0,
            )
        return code, stdout.getvalue(), stderr.getvalue()


class VerifiedArchiveTest(unittest.TestCase):
    def test_a_matching_local_copy_is_reused_with_no_fetch(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            fixture.out.mkdir()
            fixture.archive.write_bytes(BODY)
            opener = Opener()
            path = skia_archive.verified_archive(fixture.record, TARGET, fixture.out, opener, pause=0)
            self.assertEqual(path, fixture.archive)
            self.assertEqual(opener.calls, 0)

    def test_a_fetched_archive_whose_sum_matches_is_put_in_place(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            opener = Opener()
            path = skia_archive.verified_archive(fixture.record, TARGET, fixture.out, opener, pause=0)
            self.assertEqual(path, fixture.archive)
            self.assertEqual(fixture.archive.read_bytes(), BODY)
            self.assertEqual(opener.calls, 1)
            self.assertEqual(sorted(p.name for p in fixture.out.iterdir()), [fixture.archive.name])

    def test_a_local_copy_whose_sum_differs_is_fetched_again(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            fixture.out.mkdir()
            fixture.archive.write_bytes(b"stale")
            opener = Opener()
            path = skia_archive.verified_archive(fixture.record, TARGET, fixture.out, opener, pause=0)
            self.assertEqual(path, fixture.archive)
            self.assertEqual(fixture.archive.read_bytes(), BODY)
            self.assertEqual(opener.calls, 1)

    def test_a_source_failing_twice_is_fetched_on_attempt_3(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            opener = Opener(failures=2)
            code, out, err = fixture.run(opener=opener)
            self.assertEqual(code, 0, err)
            self.assertEqual(opener.calls, 3)
            self.assertEqual(
                err.splitlines(),
                ["skia-archive: attempt 1 of 3 failed in fetch", "skia-archive: attempt 2 of 3 failed in fetch"],
            )
            self.assertEqual(fixture.archive.read_bytes(), BODY)


class MainTest(unittest.TestCase):
    def test_success_prints_exactly_one_url_line_and_nothing_on_stderr(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            code, out, err = fixture.run()
            self.assertEqual(code, 0)
            self.assertEqual(err, "")
            expected = "file://" + (fixture.out.resolve() / "skia-binaries-{key}.tar.gz").as_posix()
            self.assertEqual(out, expected + "\n")
            # skia-bindings fills {key}; the recorded key then names the verified copy.
            self.assertEqual(
                out.strip().replace("{key}", KEY)[len("file://") :], fixture.archive.resolve().as_posix()
            )

    def test_a_mismatched_archive_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp, body=b"tampered bytes")
            code, out, err = fixture.run()
            self.assertEqual(code, 1)
            self.assertEqual(out, "")
            self.assertFalse(fixture.archive.exists())
            self.assertEqual(list(fixture.out.iterdir()), [])
            self.assertEqual(
                err.splitlines(),
                [f"skia-archive: attempt {n} of 3 failed in verify" for n in (1, 2, 3)]
                + ["skia-archive: no verified archive"],
            )

    def test_a_source_failing_three_times_exits_1_after_three_attempts(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            opener = Opener(failures=3)
            code, out, err = fixture.run(opener=opener)
            self.assertEqual(code, 1)
            self.assertEqual(out, "")
            self.assertEqual(opener.calls, 3)
            self.assertEqual(
                err.splitlines(),
                [f"skia-archive: attempt {n} of 3 failed in fetch" for n in (1, 2, 3)]
                + ["skia-archive: no verified archive"],
            )
            self.assertEqual(list(fixture.out.iterdir()), [])

    def test_an_unrecorded_target_exits_2_before_any_fetch(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            opener = Opener()
            code, out, err = fixture.run(["riscv64gc-unknown-linux-gnu", str(fixture.out)], opener)
            self.assertEqual(code, 2)
            self.assertEqual((out, err), ("", "skia-archive: no recorded archive for this target\n"))
            self.assertEqual(opener.calls, 0)
            self.assertFalse(fixture.out.exists())

    def test_a_bad_call_exits_2(self):
        with tempfile.TemporaryDirectory() as tmp:
            fixture = Fixture(tmp)
            code, out, err = fixture.run([TARGET])
            self.assertEqual((code, out, err), (2, "", "skia-archive: usage\n"))


class CommittedRecordTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.record = skia_archive.load_record(skia_archive.RECORD)

    def test_the_record_holds_the_five_built_targets(self):
        self.assertEqual(len(self.record["targets"]), 5)
        self.assertEqual(set(self.record["targets"]), RECORDED_TARGETS)

    def test_every_sum_is_64_hex_digits(self):
        for target, entry in self.record["targets"].items():
            with self.subTest(target=target):
                self.assertRegex(entry["sha256"], re.compile(r"^[0-9a-f]{64}$"))

    def test_the_tag_is_the_locked_skia_bindings_version(self):
        versions = [v for (name, v) in parse_lock(CARGO_LOCK.read_text(encoding="utf-8")) if name == "skia-bindings"]
        self.assertEqual(versions, [self.record["tag"]])

    def test_the_url_is_the_release_template(self):
        self.assertEqual(
            self.record["url"],
            "https://github.com/rust-skia/skia-binaries/releases/download/{tag}/skia-binaries-{key}.tar.gz",
        )


if __name__ == "__main__":
    unittest.main()
