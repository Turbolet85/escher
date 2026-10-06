#!/usr/bin/env python3
"""Contract of scripts/agent-run.sh, driven through a cargo shim. Run with `python3 -m unittest discover -s .github/scripts`."""

import json
import os
import stat
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
AGENT_RUN = ROOT / "scripts" / "agent-run.sh"

SCRUB_KEYS = {"url", "href", "src", "html", "text", "value", "attrs", "path", "request", "error"}
PANIC_TEXT = "agent-run-panic-marker"

PASSING_STAND = """\
   Compiling blitz-tests v0.3.0-beta.2 (/work/tests/blitz-tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
     Running tests/stand_boot.rs (target/debug/deps/stand_boot-008997856a834811)

running 2 tests
test each_lean_task_mounts_inside_task_shell ... ok
test two_fresh_boots_lay_out_identically ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s

     Running tests/stand_timer.rs (target/debug/deps/stand_timer-a2da96447557fd5b)

running 1 test
test reset_zeroes_elapsed ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.12s

"""

FAILING_STAND = f"""\
     Running tests/stand_counter.rs (target/debug/deps/stand_counter-08a8b2bbe851b57c)

running 2 tests
test counter_starts_at_zero ... ok
test counter_click_increments_the_display ... FAILED

failures:

---- counter_click_increments_the_display stdout ----

thread 'counter_click_increments_the_display' panicked at tests/blitz-tests/tests/stand_counter.rs:12:5:
{PANIC_TEXT}: left 0, right 1
test {PANIC_TEXT}_printed_by_the_test ... ok

failures:
    counter_click_increments_the_display

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

error: test failed, to rerun pass `--test stand_counter`
"""

EMPTY_RUN = """\
     Running tests/stand_boot.rs (target/debug/deps/stand_boot-008997856a834811)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

"""

DOC_TEST_NAME = "src/lib.rs - harness::Harness (line 12)"
DOC_TESTS = f"""\
     Running unittests lib.rs (target/debug/deps/blitz_tests-1a2b3c4d5e6f7a8b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests blitz_tests

running 1 test
test {DOC_TEST_NAME} ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

"""


class AgentRunTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.root = Path(self._tmp.name)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.calls_log = self.root / "cargo-calls"
        self.state = self.root / "target" / "agent-run"
        self.shim(0)

    def tearDown(self):
        self._tmp.cleanup()

    def shim(self, exit_code, output=""):
        out = self.root / "cargo-output"
        out.write_text(output, encoding="utf-8")
        shim = self.bin / "cargo"
        shim.write_text(
            f'#!/bin/sh\nprintf "%s\\n" "$*" >> "{self.calls_log}"\ncat "{out}"\nexit {exit_code}\n',
            encoding="utf-8",
        )
        shim.chmod(shim.stat().st_mode | stat.S_IXUSR)

    def add_test_files(self, *names):
        tests_dir = self.root / "tests" / "blitz-tests" / "tests"
        tests_dir.mkdir(parents=True, exist_ok=True)
        for name in names:
            (tests_dir / f"{name}.rs").write_text("#[test]\nfn t() {}\n", encoding="utf-8")

    def agent_run(self, *args):
        env = dict(os.environ, PATH=f"{self.bin}{os.pathsep}{os.environ['PATH']}")
        return subprocess.run(
            ["bash", str(AGENT_RUN), *args], cwd=self.root, env=env, capture_output=True, text=True
        )

    def cargo_calls(self):
        if not self.calls_log.exists():
            return []
        return [line.split() for line in self.calls_log.read_text(encoding="utf-8").splitlines()]

    def json_lines(self, text):
        return [json.loads(line) for line in text.splitlines()]

    def boot(self):
        self.shim(0)
        proc = self.agent_run("boot")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        return proc

    def run_with(self, selection, cargo_exit, output):
        self.shim(cargo_exit, output)
        return self.agent_run("run", selection)

    def test_status_before_boot_exits_3(self):
        proc = self.agent_run("status")
        self.assertEqual(proc.returncode, 3)
        (event,) = self.json_lines(proc.stdout)
        self.assertEqual(event["event"], "status")
        self.assertFalse(event["booted"])
        self.assertEqual(event["run"]["state"], "none")

    def test_missing_or_unknown_verb_exits_2(self):
        self.assertEqual(self.agent_run().returncode, 2)
        self.assertEqual(self.agent_run("no-such-verb").returncode, 2)

    def test_boot_builds_every_test_binary_and_is_ready(self):
        proc = self.boot()
        (event,) = self.json_lines(proc.stdout)
        self.assertEqual((event["event"], event["outcome"], event["cargo_exit"]), ("boot", "ready", 0))
        self.assertEqual(self.cargo_calls(), [["test", "-p", "blitz-tests", "--locked", "--no-run"]])
        status = self.agent_run("status")
        self.assertEqual(status.returncode, 0)
        self.assertTrue(self.json_lines(status.stdout)[0]["booted"])

    def test_failed_boot_exits_1_and_status_is_not_ready(self):
        self.shim(101)
        proc = self.agent_run("boot")
        self.assertEqual(proc.returncode, 1)
        (event,) = self.json_lines(proc.stdout)
        self.assertEqual((event["outcome"], event["cargo_exit"]), ("failed", 101))
        self.assertEqual(self.agent_run("status").returncode, 3)

    def test_run_before_boot_exits_3(self):
        self.add_test_files("stand_boot")
        self.assertEqual(self.agent_run("run", "stand").returncode, 3)
        self.assertEqual(self.cargo_calls(), [])

    def test_run_with_an_unknown_selection_exits_2(self):
        self.add_test_files("stand_boot", "Bad_Name")
        self.boot()
        for args in (("run",), ("run", "no_such_file"), ("run", "Bad_Name"), ("run", "../stand_boot")):
            with self.subTest(args=args):
                self.assertEqual(self.agent_run(*args).returncode, 2)
        self.assertEqual(len(self.cargo_calls()), 1)

    def test_run_stand_reports_each_test_and_the_counts(self):
        self.add_test_files("stand_boot", "stand_timer", "dioxus_falsy_disabled")
        self.boot()
        proc = self.run_with("stand", 0, PASSING_STAND)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        events = self.json_lines(proc.stdout)
        self.assertEqual(events[0]["event"], "run.start")
        self.assertEqual(events[0]["files"], ["stand_boot", "stand_timer"])
        self.assertEqual(
            [(e["file"], e["test"], e["outcome"]) for e in events if e["event"] == "test"],
            [
                ("stand_boot", "each_lean_task_mounts_inside_task_shell", "ok"),
                ("stand_boot", "two_fresh_boots_lay_out_identically", "ok"),
                ("stand_timer", "reset_zeroes_elapsed", "ignored"),
            ],
        )
        end = events[-1]
        self.assertEqual(
            (end["event"], end["passed"], end["failed"], end["ignored"], end["cargo_exit"], end["outcome"]),
            ("run.end", 2, 0, 1, 0, "passed"),
        )
        self.assertEqual(
            self.cargo_calls()[-1],
            ["test", "-p", "blitz-tests", "--locked", "--test", "stand_boot", "--test", "stand_timer"],
        )
        run = self.json_lines(self.agent_run("status").stdout)[0]["run"]
        self.assertEqual((run["selection"], run["state"], run["passed"]), ("stand", "passed", 2))

    def test_failing_run_exits_1_and_carries_no_captured_output(self):
        self.add_test_files("stand_counter")
        self.boot()
        proc = self.run_with("stand", 101, FAILING_STAND)
        self.assertEqual(proc.returncode, 1)
        end = self.json_lines(proc.stdout)[-1]
        self.assertEqual((end["passed"], end["failed"], end["outcome"]), (1, 1, "failed"))
        self.assertNotIn(PANIC_TEXT, proc.stdout)
        self.assertNotIn(PANIC_TEXT, (self.state / "events.jsonl").read_text(encoding="utf-8"))
        self.assertEqual(self.json_lines(self.agent_run("status").stdout)[0]["run"]["state"], "failed")

    def test_an_empty_run_is_never_a_pass(self):
        self.add_test_files("stand_boot")
        self.boot()
        proc = self.run_with("stand", 0, EMPTY_RUN)
        self.assertEqual(proc.returncode, 1)
        end = self.json_lines(proc.stdout)[-1]
        self.assertEqual((end["passed"], end["cargo_exit"], end["outcome"]), (0, 0, "failed"))

    def test_a_doc_test_name_yields_one_json_event(self):
        self.boot()
        proc = self.run_with("all", 0, DOC_TESTS)
        self.assertEqual(proc.returncode, 0, proc.stderr)
        tests = [e for e in self.json_lines(proc.stdout) if e["event"] == "test"]
        self.assertEqual(tests, [{"event": "test", "file": "doc:blitz_tests", "test": DOC_TEST_NAME, "outcome": "ok"}])
        self.assertEqual(self.cargo_calls()[-1], ["test", "-p", "blitz-tests", "--locked"])

    def test_logs_before_boot_exits_3(self):
        proc = self.agent_run("logs")
        self.assertEqual(proc.returncode, 3)
        self.assertEqual(proc.stdout, "")

    def test_logs_prints_exactly_the_appended_events(self):
        self.add_test_files("stand_boot", "stand_timer")
        boot = self.boot()
        run = self.run_with("stand", 0, PASSING_STAND)
        logs = self.agent_run("logs")
        self.assertEqual(logs.returncode, 0)
        self.assertEqual(logs.stdout, boot.stdout + run.stdout)
        for event in self.json_lines(logs.stdout):
            self.assertFalse(SCRUB_KEYS & set(event), event)

    def test_cleanup_is_idempotent_and_leaves_no_state(self):
        self.boot()
        for _ in range(2):
            proc = self.agent_run("cleanup")
            self.assertEqual(proc.returncode, 0)
            self.assertEqual(self.json_lines(proc.stdout), [{"event": "cleanup", "outcome": "done"}])
        self.assertFalse(self.state.exists())
        self.assertEqual(self.agent_run("status").returncode, 3)

    def test_run_log_holds_the_raw_cargo_output(self):
        self.add_test_files("stand_counter")
        self.boot()
        self.run_with("stand", 101, FAILING_STAND)
        self.assertEqual((self.state / "run.log").read_text(encoding="utf-8"), FAILING_STAND)


if __name__ == "__main__":
    unittest.main()
