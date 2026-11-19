#!/usr/bin/env python3
"""Contract of scripts/target-sweep.py, driven through a cargo shim over a fake build directory.
Run with `python3 -m unittest discover -s .github/scripts`."""

import fcntl
import importlib.util
import json
import os
import stat
import subprocess
import tempfile
import time
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SWEEP = ROOT / "scripts" / "target-sweep.py"

KEPT = "1111111111111111"
KEPT_RUN = "2222222222222222"
KEPT_SCRIPT = "3333333333333333"
STALE = "aaaaaaaaaaaaaaaa"
STALE_BUILD = "bbbbbbbbbbbbbbbb"
LATE = "cccccccccccccccc"

ENDING_KEYS = {
    "swept": {"verb", "outcome", "units_kept", "units_removed", "bytes_before", "bytes_after", "seconds"},
    "planned": {"verb", "outcome", "units_kept", "units_to_remove", "bytes_before", "bytes_after", "seconds"},
    "refused": {"verb", "outcome", "cause"},
    "failed": {"verb", "outcome", "cause"},
}


def write(path, text="x"):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def make_build_directory(root):
    """A target/debug holding one kept library unit, one kept build-script compile and run, one stale unit of each kind
    and the state areas no sweep touches."""
    debug = root / "target" / "debug"
    write(debug / "deps" / f"libengine-{KEPT}.rlib")
    write(debug / "deps" / f"engine-{KEPT}.d")
    write(debug / ".fingerprint" / f"engine-{KEPT}" / "lib-engine")
    write(debug / "build" / f"engine-{KEPT_RUN}" / "out" / "gen.rs")
    write(debug / ".fingerprint" / f"engine-{KEPT_RUN}" / "run-build-script-build-script-build")
    write(debug / "build" / f"engine-{KEPT_SCRIPT}" / "build-script-build")
    write(debug / "build" / f"engine-{KEPT_SCRIPT}" / f"build_script_build-{KEPT_SCRIPT}")
    write(debug / ".fingerprint" / f"engine-{KEPT_SCRIPT}" / "build-script-build-script-build")
    write(debug / "deps" / f"libengine-{STALE}.rlib")
    write(debug / "deps" / f"libengine-{STALE}.rmeta")
    write(debug / "deps" / f"engine-{STALE}.d")
    write(debug / ".fingerprint" / f"engine-{STALE}" / "lib-engine")
    write(debug / "build" / f"engine-{STALE_BUILD}" / "out" / "gen.rs")
    write(debug / ".fingerprint" / f"engine-{STALE_BUILD}" / "run-build-script-build-script-build")
    write(debug / "incremental" / "engine-0kept000000001" / "s-1-a" / "dep-graph.bin")
    write(debug / "incremental" / "engine-0stale00000001" / "s-1-a" / "dep-graph.bin")
    write(debug / ".cargo-lock", "")
    write(debug / "escher-session", "binary")
    write(debug / "examples" / "counter", "example")
    write(root / "target" / "agent-run" / "status.json", "{}")
    write(root / "target" / "cold-agent" / "record" / "check-1.json", "{}")
    write(root / "target" / "tmp" / "session" / "x", "x")
    write(root / "target" / "ci-logs" / "fast.log", "log")
    # Every unit stands a minute older than the keep pass, as a unit built before it does.
    old = time.time() - 60
    stale_incremental = debug / "incremental" / "engine-0stale00000001"
    for path in sorted(debug.rglob("*"), reverse=True):
        os.utime(path, (old, old))
    for path in (stale_incremental, *stale_incremental.rglob("*")):
        os.utime(path, (old - 60, old - 60))
    return debug


def tree(root):
    return {str(p.relative_to(root)): (p.read_bytes() if p.is_file() else None) for p in sorted(root.rglob("*"))}


class SweepCase(unittest.TestCase):
    def shim(self, root, exit_code=0, late_unit=False):
        """A cargo shim naming the kept units in cargo's JSON artifact lines, logging each call."""
        bin_dir = root / "bin"
        bin_dir.mkdir(exist_ok=True)
        debug = root / "target" / "debug"
        artifact = {
            "reason": "compiler-artifact",
            "package_id": "path+file:///work/engine#0.1.0",
            "target": {"name": "engine", "kind": ["lib"]},
            "filenames": [str(debug / "deps" / f"libengine-{KEPT}.rlib")],
            "executable": None,
        }
        # Cargo names a build script's compile unit by its directory: build/{package}-{hash}/build-script-build.
        script_compile = {
            "reason": "compiler-artifact",
            "package_id": "path+file:///work/engine#0.1.0",
            "target": {"name": "build-script-build", "kind": ["custom-build"]},
            "filenames": [str(debug / "build" / f"engine-{KEPT_SCRIPT}" / "build-script-build")],
            "executable": None,
        }
        executed = {
            "reason": "build-script-executed",
            "package_id": "path+file:///work/engine#0.1.0",
            "out_dir": str(debug / "build" / f"engine-{KEPT_RUN}" / "out"),
        }
        late = ""
        if late_unit:
            late = f'mkdir -p "{debug}/.fingerprint/engine-{LATE}"; echo x > "{debug}/deps/libengine-{LATE}.rlib"\n'
        script = (
            "#!/bin/sh\n"
            f'echo "$@" >> "{root}/cargo-calls"\n'
            f"{late}"
            f"echo '{json.dumps(artifact)}'\n"
            f"echo '{json.dumps(script_compile)}'\n"
            f"echo '{json.dumps(executed)}'\n"
            "echo 'Compiling engine' >&2\n"
            f"exit {exit_code}\n"
        )
        shim = bin_dir / "cargo"
        shim.write_text(script, encoding="utf-8")
        shim.chmod(shim.stat().st_mode | stat.S_IXUSR)
        return dict(os.environ, PATH=f"{bin_dir}{os.pathsep}{os.environ['PATH']}")

    def sweep(self, root, env, *argv):
        return subprocess.run(
            ["python3", str(SWEEP), *argv], cwd=root, env=env, capture_output=True, text=True, timeout=60
        )

    def ending(self, proc):
        lines = proc.stdout.splitlines()
        self.assertEqual(len(lines), 1, proc.stdout)
        line = json.loads(lines[0])
        self.assertIsInstance(line, dict)
        self.assertEqual(set(line), ENDING_KEYS[line["outcome"]], line)
        return line

    def calls(self, root):
        path = root / "cargo-calls"
        return path.read_text(encoding="utf-8").splitlines() if path.exists() else []


class UsageTest(SweepCase):
    def test_bad_argv_exits_two_with_nothing_run(self):
        for argv in ([], ["sweep"], ["run", "plan"], ["--help"]):
            with self.subTest(argv=argv), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                make_build_directory(root)
                before = tree(root / "target")
                proc = self.sweep(root, self.shim(root), *argv)
                self.assertEqual(proc.returncode, 2)
                self.assertEqual(proc.stdout, "")
                self.assertIn("usage", proc.stderr)
                self.assertEqual(self.calls(root), [])
                self.assertEqual(tree(root / "target"), before)


class PlanTest(SweepCase):
    def test_plan_counts_the_unkept_units_and_removes_nothing(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            make_build_directory(root)
            before = tree(root / "target")
            proc = self.sweep(root, self.shim(root), "plan")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            line = self.ending(proc)
            self.assertEqual(line["verb"], "plan")
            self.assertEqual(line["outcome"], "planned")
            self.assertEqual(line["units_to_remove"], 3)
            self.assertEqual(line["units_kept"], 4)
            self.assertEqual(line["bytes_before"], line["bytes_after"])
            self.assertEqual(tree(root / "target"), before)


class RunTest(SweepCase):
    def test_run_removes_exactly_the_unkept_units(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            debug = make_build_directory(root)
            before = tree(root / "target")
            proc = self.sweep(root, self.shim(root), "run")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            line = self.ending(proc)
            self.assertEqual((line["verb"], line["outcome"]), ("run", "swept"))
            self.assertEqual(line["units_removed"], 3)
            self.assertEqual(line["units_kept"], 4)
            self.assertLess(line["bytes_after"], line["bytes_before"])
            after = tree(root / "target")
            removed = set(before) - set(after)
            self.assertEqual(set(after) - set(before), set())
            for name in removed:
                self.assertTrue(
                    name.split("/")[1] in ("deps", ".fingerprint", "build", "incremental"), name
                )
                self.assertTrue(STALE in name or STALE_BUILD in name or "0stale" in name, name)
            for name in (f"deps/libengine-{STALE}.rlib", f"deps/libengine-{STALE}.rmeta", f"deps/engine-{STALE}.d",
                         f".fingerprint/engine-{STALE}", f"build/engine-{STALE_BUILD}",
                         f".fingerprint/engine-{STALE_BUILD}", "incremental/engine-0stale00000001"):
                self.assertFalse((debug / name).exists(), name)
            for name in (f"deps/libengine-{KEPT}.rlib", f"deps/engine-{KEPT}.d", f".fingerprint/engine-{KEPT}",
                         f"build/engine-{KEPT_RUN}/out/gen.rs", f"build/engine-{KEPT_SCRIPT}/build-script-build",
                         f".fingerprint/engine-{KEPT_SCRIPT}", "incremental/engine-0kept000000001"):
                self.assertTrue((debug / name).exists(), name)

    def test_state_areas_and_top_level_files_stand_byte_equal(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            make_build_directory(root)
            areas = ("agent-run", "cold-agent", "tmp", "ci-logs")
            before = {a: tree(root / "target" / a) for a in areas}
            binary = (root / "target" / "debug" / "escher-session").read_bytes()
            example = (root / "target" / "debug" / "examples" / "counter").read_bytes()
            proc = self.sweep(root, self.shim(root), "run")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            for area in areas:
                self.assertEqual(tree(root / "target" / area), before[area], area)
            self.assertEqual((root / "target" / "debug" / "escher-session").read_bytes(), binary)
            self.assertEqual((root / "target" / "debug" / "examples" / "counter").read_bytes(), example)

    def test_unit_written_after_the_keep_pass_began_is_kept(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            debug = make_build_directory(root)
            proc = self.sweep(root, self.shim(root, late_unit=True), "run")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.ending(proc)
            self.assertTrue((debug / ".fingerprint" / f"engine-{LATE}").is_dir())
            self.assertTrue((debug / "deps" / f"libengine-{LATE}.rlib").is_file())
            self.assertFalse((debug / "deps" / f"libengine-{STALE}.rlib").exists())

    def test_a_unit_whose_artifacts_live_outside_the_swept_areas_keeps_its_fingerprint(self):
        # Cargo's JSON names an example only by its uplifted path, so its hash reaches no keep set.
        example = "4444444444444444"
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            debug = make_build_directory(root)
            write(debug / ".fingerprint" / f"examples-{example}" / "example-counter")
            write(debug / "examples" / f"counter-{example}", "example")
            old = time.time() - 60
            for path in (debug / ".fingerprint" / f"examples-{example}", debug / "examples" / f"counter-{example}"):
                os.utime(path, (old, old))
            proc = self.sweep(root, self.shim(root), "run")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.assertEqual(self.ending(proc)["units_removed"], 3)
            self.assertTrue((debug / ".fingerprint" / f"examples-{example}" / "example-counter").is_file())
            self.assertTrue((debug / "examples" / f"counter-{example}").is_file())

    def test_every_keep_run_is_locked_and_names_json_artifacts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            make_build_directory(root)
            proc = self.sweep(root, self.shim(root), "plan")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            calls = self.calls(root)
            self.assertEqual(len(calls), 7)
            for call in calls:
                words = call.split()
                self.assertIn("--locked", words)
                self.assertIn("--message-format=json", words)
            self.assertEqual(sum("--no-run" in c.split() for c in calls), 4)


class LockTest(SweepCase):
    def test_run_refuses_while_a_build_holds_the_lock(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            debug = make_build_directory(root)
            before = tree(root / "target")
            with open(debug / ".cargo-lock", "a", encoding="utf-8") as held:
                fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
                proc = self.sweep(root, self.shim(root), "run")
            self.assertEqual(proc.returncode, 3, proc.stderr)
            line = self.ending(proc)
            self.assertEqual((line["outcome"], line["cause"]), ("refused", "build-in-flight"))
            self.assertEqual(tree(root / "target"), before)

    def test_a_second_taker_fails_while_the_sweep_holds_the_lock(self):
        spec = importlib.util.spec_from_file_location("target_sweep", SWEEP)
        sweep = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(sweep)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            debug = make_build_directory(root)
            readings = []
            real_remove = sweep.remove_path

            def remove_while_probing(path):
                with open(debug / ".cargo-lock", "a", encoding="utf-8") as second:
                    try:
                        fcntl.flock(second, fcntl.LOCK_EX | fcntl.LOCK_NB)
                        readings.append("taken")
                    except BlockingIOError:
                        readings.append("held")
                real_remove(path)

            sweep.remove_path = remove_while_probing
            cwd = os.getcwd()
            env = os.environ.copy()
            try:
                os.environ.update(self.shim(root))
                os.chdir(root)
                with open(os.devnull, "w", encoding="utf-8") as sink:
                    code = sweep.main(["run"], out=sink)
            finally:
                os.chdir(cwd)
                os.environ.clear()
                os.environ.update(env)
            self.assertEqual(code, 0)
            self.assertTrue(readings)
            self.assertEqual(set(readings), {"held"})


class EndingTest(SweepCase):
    def test_a_failing_keep_run_exits_one_with_nothing_removed(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            make_build_directory(root)
            before = tree(root / "target")
            proc = self.sweep(root, self.shim(root, exit_code=101), "run")
            self.assertEqual(proc.returncode, 1, proc.stderr)
            line = self.ending(proc)
            self.assertEqual((line["outcome"], line["cause"]), ("failed", "keep-run"))
            self.assertEqual(tree(root / "target"), before)

    def test_no_build_directory_is_refused_before_any_cargo_call(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            proc = self.sweep(root, self.shim(root), "run")
            self.assertEqual(proc.returncode, 3, proc.stderr)
            line = self.ending(proc)
            self.assertEqual((line["outcome"], line["cause"]), ("refused", "no-build-directory"))
            self.assertEqual(self.calls(root), [])
            self.assertFalse((root / "target").exists())

    def test_every_ending_prints_one_line_without_a_path(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            make_build_directory(root)
            proc = self.sweep(root, self.shim(root), "run")
            line = self.ending(proc)
            self.assertNotIn(tmp, proc.stdout)
            self.assertNotIn("engine", proc.stdout)
            for key, value in line.items():
                self.assertIsInstance(value, str if key in ("verb", "outcome", "cause") else (int, float), key)
            self.assertEqual(proc.stdout.strip(), json.dumps(line))


LEG_SCRIPT = ROOT / ".github" / "scripts" / "ci-leg.sh"
SWEEP_LINE = '{"verb": "run", "outcome": "swept", "units_kept": 1, "units_removed": 0}'


class FastLegOwnsTheSweepTest(unittest.TestCase):
    """The local pre-push gate is the sweep's one owning caller: after a green verdict, off CI, never reddening it."""

    def fast(self, root, cargo_exit=0, sweep_exit=0, ci=None):
        bin_dir = root / "bin"
        bin_dir.mkdir()
        calls = root / "calls"
        cargo = bin_dir / "cargo"
        cargo.write_text(f'#!/bin/sh\necho "cargo $*" >> "{calls}"\nexit {cargo_exit}\n', encoding="utf-8")
        python = bin_dir / "python3"
        python.write_text(
            "#!/bin/sh\n"
            'case "$*" in\n'
            f'*scripts/target-sweep.py\\ run*) echo "sweep $*" >> "{calls}"; echo \'{SWEEP_LINE}\'; exit {sweep_exit} ;;\n'
            "esac\n"
            "exit 0\n",
            encoding="utf-8",
        )
        for shim in (cargo, python):
            shim.chmod(shim.stat().st_mode | stat.S_IXUSR)
        env = {k: v for k, v in os.environ.items() if k != "CI"}
        env["PATH"] = f"{bin_dir}{os.pathsep}{os.environ['PATH']}"
        if ci is not None:
            env["CI"] = ci
        proc = subprocess.run(
            ["bash", str(LEG_SCRIPT), "fast"], cwd=root, env=env, capture_output=True, text=True, timeout=60
        )
        lines = calls.read_text(encoding="utf-8").splitlines() if calls.exists() else []
        return proc, lines

    def test_fast_runs_the_sweep_once_after_the_embolden_leg(self):
        with tempfile.TemporaryDirectory() as tmp:
            proc, calls = self.fast(Path(tmp))
            self.assertEqual(proc.returncode, 0, proc.stderr)
            sweeps = [c for c in calls if c.startswith("sweep ")]
            self.assertEqual(len(sweeps), 1)
            self.assertEqual(calls[-1], sweeps[0])
            self.assertIn("blitz-paint/font-embolden", calls[-2])
            self.assertEqual(proc.stdout.splitlines()[-1], SWEEP_LINE)

    def test_a_refused_or_failed_sweep_never_reddens_fast(self):
        for sweep_exit in (3, 1):
            with self.subTest(sweep_exit=sweep_exit), tempfile.TemporaryDirectory() as tmp:
                proc, calls = self.fast(Path(tmp), sweep_exit=sweep_exit)
                self.assertEqual(proc.returncode, 0, proc.stderr)
                self.assertEqual(sum(c.startswith("sweep ") for c in calls), 1)
                self.assertEqual(proc.stdout.splitlines()[-1], SWEEP_LINE)

    def test_fast_under_ci_calls_no_sweep(self):
        with tempfile.TemporaryDirectory() as tmp:
            proc, calls = self.fast(Path(tmp), ci="true")
            self.assertEqual(proc.returncode, 0, proc.stderr)
            self.assertTrue(any("font-embolden" in c for c in calls))
            self.assertFalse(any(c.startswith("sweep ") for c in calls))
            self.assertNotIn('"verb":', proc.stdout)

    def test_a_red_leg_calls_no_sweep(self):
        with tempfile.TemporaryDirectory() as tmp:
            proc, calls = self.fast(Path(tmp), cargo_exit=1)
            self.assertNotEqual(proc.returncode, 0)
            self.assertTrue(calls)
            self.assertFalse(any(c.startswith("sweep ") for c in calls))


if __name__ == "__main__":
    unittest.main()
