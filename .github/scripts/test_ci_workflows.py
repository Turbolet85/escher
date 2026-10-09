#!/usr/bin/env python3
"""Invariants of the fork's workflows, of ci-leg.sh and of apt-install.sh. Run with `python3 -m unittest discover -s .github/scripts`."""

import os
import re
import shlex
import signal
import stat
import subprocess
import tempfile
import time
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"
LEG_SCRIPT = ROOT / ".github" / "scripts" / "ci-leg.sh"
INSTALL_SCRIPT = ROOT / ".github" / "scripts" / "apt-install.sh"

UPSTREAM_GUARD = "github.repository == 'DioxusLabs/blitz'"
FAST_JOBS = {"fmt", "clippy", "test-features-default", "ci-scripts"}
LINUX_JOB_LEGS = {
    "fmt": "fmt",
    "clippy": "clippy",
    "test-features-default": "test",
    "ci-scripts": "ci-scripts",
    "build-msrv": "msrv",
    "build-features-default": "build",
    "build-counter": "counter",
    "build-wasm-examples": "wasm",
    "doc": "doc",
    "audit": "audit",
    "a11y": "a11y",
    "coverage": "coverage",
}
# cargo-deny resolves metadata and compiles nothing.
NON_COMPILING_JOBS = {"fmt", "ci-scripts", "audit"}
# Uncached by the overseer's ruling: an instrumented target would evict the other legs' entries
# from the fork's near-full Actions cache.
NO_CACHE_JOBS = {"coverage"}
# Restores the test job's cache under its shared key and never saves a second entry.
RESTORE_ONLY_JOBS = {"a11y"}
CACHE_ACTION = "Swatinem/rust-cache@"
PINNED_USES = re.compile(r"^[^/@\s]+/[^@\s]+@[0-9a-f]{40}$")
LEG_STEP = re.compile(r"^bash \.github/scripts/ci-leg\.sh (\S+)$")
# A package install: the script and package names, no option, optionally behind the PyYAML guard.
INSTALL_STEP = re.compile(
    r"^(?:python3 -c 'import yaml' \|\| )?bash \.github/scripts/apt-install\.sh( [a-z0-9][a-z0-9+.-]+)+$"
)
INSTALL_JOBS = {
    "build-msrv",
    "build-features-default",
    "test-features-default",
    "build-counter",
    "clippy",
    "ci-scripts",
    "doc",
    "a11y",
    "coverage",
}
# An action step cannot be looped: its install is bounded by a step timeout alone.
APT_ACTION_JOBS = ["matrix_test"]


def load(name):
    with open(WORKFLOWS / name, encoding="utf-8") as f:
        return yaml.safe_load(f)


def triggers(workflow):
    # YAML 1.1 reads the bare key `on` as the boolean True.
    return workflow.get(True, workflow.get("on"))


def script_legs():
    proc = subprocess.run(["bash", str(LEG_SCRIPT)], capture_output=True, text=True)
    for line in proc.stderr.splitlines():
        if line.startswith("legs: "):
            return set(line[len("legs: ") :].split())
    raise AssertionError(f"ci-leg.sh printed no leg list: {proc.stderr!r}")


def cache_steps(job):
    return [s for s in job["steps"] if str(s.get("uses", "")).startswith(CACHE_ACTION)]


def uploads_of(job):
    return [s for s in job["steps"] if str(s.get("uses", "")).startswith("actions/upload-artifact@")]


def leg_of(job):
    for step in job.get("steps", []):
        match = LEG_STEP.match(str(step.get("run", "")).strip())
        if match:
            return match.group(1)
    return None


class CiWorkflowTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ci = load("ci.yml")
        cls.jobs = cls.ci["jobs"]
        cls.text = (WORKFLOWS / "ci.yml").read_text(encoding="utf-8")

    def test_a_push_triggers_build_branches(self):
        self.assertIn("build/**", triggers(self.ci)["push"]["branches"])

    def test_b_compiling_jobs_cache_and_save_on_main_and_build(self):
        self.assertTrue(NO_CACHE_JOBS | RESTORE_ONLY_JOBS <= set(self.jobs))
        test_caches = cache_steps(self.jobs["test-features-default"])
        self.assertEqual(len(test_caches), 1)
        shared_key = test_caches[0].get("with", {}).get("shared-key")
        self.assertTrue(shared_key)
        for job_id, job in self.jobs.items():
            if job_id in NON_COMPILING_JOBS:
                continue
            with self.subTest(job=job_id):
                caches = cache_steps(job)
                if job_id in NO_CACHE_JOBS:
                    self.assertEqual(caches, [])
                    continue
                self.assertEqual(len(caches), 1)
                inputs = caches[0].get("with", {})
                save_if = str(inputs.get("save-if", ""))
                if job_id in RESTORE_ONLY_JOBS:
                    self.assertEqual(save_if.lower(), "false")
                    self.assertEqual(inputs.get("shared-key"), shared_key)
                    continue
                self.assertIn("refs/heads/main", save_if)
                self.assertIn("refs/heads/build/", save_if)

    def test_c_slow_jobs_need_every_fast_job(self):
        self.assertTrue(FAST_JOBS <= set(self.jobs))
        for job_id, job in self.jobs.items():
            with self.subTest(job=job_id):
                needs = job.get("needs", [])
                needs = {needs} if isinstance(needs, str) else set(needs)
                if job_id in FAST_JOBS:
                    self.assertEqual(needs, set())
                else:
                    self.assertEqual(needs, FAST_JOBS)

    def test_d_linux_jobs_run_their_leg_through_the_script(self):
        legs = script_legs()
        linux = {j for j, job in self.jobs.items() if job.get("runs-on") == "ubuntu-latest"}
        self.assertEqual(linux, set(LINUX_JOB_LEGS))
        for job_id, leg in LINUX_JOB_LEGS.items():
            with self.subTest(job=job_id):
                self.assertEqual(leg_of(self.jobs[job_id]), leg)
                self.assertIn(leg, legs)

    def test_e_leg_jobs_upload_their_own_log_on_failure(self):
        expected = {job_id: f"target/ci-logs/{leg}.log" for job_id, leg in LINUX_JOB_LEGS.items()}
        expected["matrix_test"] = "target/ci-logs/matrix-${{ matrix.platform.name }}.log"
        for job_id, path in expected.items():
            with self.subTest(job=job_id):
                uploads = uploads_of(self.jobs[job_id])
                # The coverage job also uploads its report, as its own artifact.
                self.assertEqual(len(uploads), 2 if job_id == "coverage" else 1)
                failure_uploads = [u for u in uploads if u.get("if") == "failure()"]
                self.assertEqual(len(failure_uploads), 1)
                upload = failure_uploads[0]
                self.assertEqual(upload.get("if"), "failure()")
                self.assertEqual(upload["with"]["path"], path)
                self.assertTrue(upload["with"]["path"].startswith("target/ci-logs/"))
                self.assertEqual(upload["with"]["retention-days"], 7)

    def test_g_no_secret_reference(self):
        self.assertNotIn("secrets.", self.text)

    def test_h_no_perl_rewrite(self):
        for job_id, job in self.jobs.items():
            for step in job.get("steps", []):
                with self.subTest(job=job_id, step=step.get("name") or step.get("run") or step.get("uses")):
                    self.assertNotIn("perl", str(step.get("run", "")))

    def test_i_matrix_keeps_every_platform_but_linux(self):
        names = {p["name"] for p in self.jobs["matrix_test"]["strategy"]["matrix"]["platform"]}
        self.assertEqual(names, {"windows", "macos", "ios", "android"})

    def test_j_every_action_is_pinned_to_a_commit_sha(self):
        for job_id, job in self.jobs.items():
            for step in job.get("steps", []):
                if "uses" in step:
                    with self.subTest(job=job_id, uses=step["uses"]):
                        self.assertRegex(step["uses"], PINNED_USES)

    def test_k_pinned_rust_toolchain_names_its_toolchain(self):
        # A SHA pin loses the toolchain that the `@stable` branch name used to select.
        for job_id, job in self.jobs.items():
            for step in job.get("steps", []):
                if str(step.get("uses", "")).startswith("dtolnay/rust-toolchain@"):
                    with self.subTest(job=job_id):
                        self.assertTrue(str(step.get("with", {}).get("toolchain", "")))

    def test_l_token_is_read_only_at_the_workflow_level(self):
        self.assertEqual(self.ci.get("permissions"), {"contents": "read"})
        for job_id, job in self.jobs.items():
            with self.subTest(job=job_id):
                self.assertNotIn("permissions", job)

    def test_m_a11y_job_is_named_for_accessibility(self):
        self.assertRegex(self.jobs["a11y"]["name"], re.compile("a11y|accessib", re.IGNORECASE))

    def test_n_coverage_report_is_its_own_artifact(self):
        reports = [u for u in uploads_of(self.jobs["coverage"]) if u.get("if") != "failure()"]
        self.assertEqual(len(reports), 1)
        inputs = reports[0]["with"]
        self.assertEqual(inputs["name"], "coverage-report")
        self.assertEqual(inputs["path"], "target/coverage/")
        self.assertEqual(inputs["retention-days"], 7)

    def test_o_package_installs_are_bounded(self):
        script_jobs = set()
        action_jobs = []
        for job_id, job in self.jobs.items():
            for step in job.get("steps", []):
                run = str(step.get("run", "")).strip()
                with self.subTest(job=job_id, step=step.get("name") or step.get("run") or step.get("uses")):
                    self.assertNotIn("apt-get", run)
                    if "apt-install.sh" in run:
                        script_jobs.add(job_id)
                        self.assertRegex(run, INSTALL_STEP)
                    if "apt" in str(step.get("uses", "")):
                        action_jobs.append(job_id)
                        timeout = step.get("timeout-minutes")
                        self.assertIs(type(timeout), int)
                        self.assertGreater(timeout, 0)
        self.assertEqual(script_jobs, INSTALL_JOBS)
        self.assertEqual(action_jobs, APT_ACTION_JOBS)


class UpstreamGuardTest(unittest.TestCase):
    def test_f_every_upstream_job_is_repository_guarded(self):
        for name in ("publish-browser.yml", "wpt.yml", "wpt-post-results.yml"):
            for job_id, job in load(name)["jobs"].items():
                with self.subTest(workflow=name, job=job_id):
                    self.assertIn(UPSTREAM_GUARD, str(job.get("if", "")))


class LegScriptTest(unittest.TestCase):
    MARKER = "ci-leg-shim-marker"

    def run_leg(self, cwd, leg, cargo_exit, echo_args=False):
        bin_dir = Path(cwd) / "bin"
        bin_dir.mkdir(exist_ok=True)
        shim = bin_dir / "cargo"
        echo = f'echo {self.MARKER} "$@"' if echo_args else f"echo {self.MARKER}"
        shim.write_text(f"#!/bin/sh\n{echo}\nexit {cargo_exit}\n", encoding="utf-8")
        shim.chmod(shim.stat().st_mode | stat.S_IXUSR)
        env = dict(os.environ, PATH=f"{bin_dir}{os.pathsep}{os.environ['PATH']}")
        return subprocess.run(
            ["bash", str(LEG_SCRIPT), leg], cwd=cwd, env=env, capture_output=True, text=True
        )

    def test_failing_leg_propagates_status_and_writes_its_log(self):
        with tempfile.TemporaryDirectory() as tmp:
            proc = self.run_leg(tmp, "fmt", 1)
            self.assertNotEqual(proc.returncode, 0)
            log = Path(tmp) / "target" / "ci-logs" / "fmt.log"
            self.assertIn(self.MARKER, log.read_text(encoding="utf-8"))

    def test_log_is_truncated_at_the_leg_start(self):
        with tempfile.TemporaryDirectory() as tmp:
            log = Path(tmp) / "target" / "ci-logs" / "build.log"
            log.parent.mkdir(parents=True)
            log.write_text("stale-from-a-cache\n", encoding="utf-8")
            proc = self.run_leg(tmp, "build", 0)
            self.assertEqual(proc.returncode, 0)
            self.assertEqual(log.read_text(encoding="utf-8"), f"{self.MARKER}\n")

    def leg_log_words(self, leg):
        with tempfile.TemporaryDirectory() as tmp:
            proc = self.run_leg(tmp, leg, 0, echo_args=True)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            return (Path(tmp) / "target" / "ci-logs" / f"{leg}.log").read_text(encoding="utf-8").split()

    def test_doc_leg_documents_the_whole_workspace(self):
        words = self.leg_log_words("doc")
        for word in ("doc", "--workspace", "--no-deps", "--locked"):
            self.assertIn(word, words)

    def test_gate_legs_dispatch_to_cargo_and_write_their_log(self):
        for leg, expected in (
            ("audit", ("deny", "--locked", "advisories")),
            ("a11y", ("--workspace", "--locked", "accessibility_hidden", "accessibility_roles", "focusability_updates")),
            ("coverage", ("llvm-cov", "--workspace", "--locked", "--lcov")),
        ):
            with self.subTest(leg=leg):
                words = self.leg_log_words(leg)
                self.assertIn(self.MARKER, words)
                for word in expected:
                    self.assertIn(word, words)

    def test_unknown_leg_exits_2(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.assertEqual(self.run_leg(tmp, "no-such-leg", 0).returncode, 2)


class InstallScriptTest(unittest.TestCase):
    STALL_SECONDS = 300
    WAIT_SECONDS = 30

    def run_install(self, cwd, behaviour, *args):
        """Runs the script against a stand-in `sudo` and `apt-get`; returns its exit, stderr and the apt-get calls."""
        bin_dir = Path(cwd) / "bin"
        bin_dir.mkdir()
        calls = Path(cwd) / "calls"
        shims = {
            "sudo": 'exec "$@"',
            "apt-get": f'echo "$*" >> {shlex.quote(str(calls))}\n{behaviour}',
        }
        for name, body in shims.items():
            shim = bin_dir / name
            shim.write_text(f"#!/bin/sh\n{body}\n", encoding="utf-8")
            shim.chmod(shim.stat().st_mode | stat.S_IXUSR)
        env = dict(os.environ, PATH=f"{bin_dir}{os.pathsep}{os.environ['PATH']}")
        proc = subprocess.Popen(
            ["bash", str(INSTALL_SCRIPT), "--pause", "0", *args],
            cwd=cwd,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            start_new_session=True,
        )
        try:
            _, stderr = proc.communicate(timeout=self.WAIT_SECONDS)
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, signal.SIGKILL)
            proc.communicate()
            self.fail(f"the script was still running after {self.WAIT_SECONDS} s")
        recorded = calls.read_text(encoding="utf-8").splitlines() if calls.exists() else []
        return proc.returncode, stderr, recorded

    def test_a_healthy_install_runs_one_update_and_one_install(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, _, calls = self.run_install(tmp, "exit 0", "libfontconfig1-dev", "python3-yaml")
            self.assertEqual(code, 0)
            self.assertEqual(calls, ["update", "install -y libfontconfig1-dev python3-yaml"])

    def test_a_failed_attempt_is_tried_again_from_update(self):
        with tempfile.TemporaryDirectory() as tmp:
            calls_file = shlex.quote(str(Path(tmp) / "calls"))
            fail_first = f'[ "$(wc -l < {calls_file})" -gt 1 ] || exit 100\nexit 0'
            code, stderr, calls = self.run_install(tmp, fail_first, "libfontconfig1-dev")
            self.assertEqual(code, 0)
            self.assertEqual(calls, ["update", "update", "install -y libfontconfig1-dev"])
            self.assertIn("attempt 1 of 3 failed in update (exit 100)", stderr)

    def test_every_attempt_failing_exits_1_after_three_attempts(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, stderr, calls = self.run_install(tmp, "exit 100", "libfontconfig1-dev")
            self.assertEqual(code, 1)
            self.assertEqual(calls, ["update"] * 3)
            self.assertIn("attempt 3 of 3 failed in update (exit 100)", stderr)

    def test_a_stalled_call_ends_at_its_bound_and_is_tried_again(self):
        with tempfile.TemporaryDirectory() as tmp:
            started = time.monotonic()
            code, stderr, calls = self.run_install(
                tmp, f"exec sleep {self.STALL_SECONDS}", "--bound", "1", "--attempts", "2", "libfontconfig1-dev"
            )
            self.assertEqual(code, 1)
            self.assertEqual(calls, ["update", "update"])
            self.assertIn("attempt 2 of 2 failed in update (exit 124)", stderr)
            self.assertLess(time.monotonic() - started, self.WAIT_SECONDS)

    def test_a_bad_call_exits_2_before_anything_runs(self):
        for name, args in (
            ("no package", ()),
            ("options alone", ("--attempts", "2")),
            ("a name outside the form", ("libfontconfig1-dev", "Not_A_Package")),
            ("a shell word as a name", ("libfontconfig1-dev;id",)),
            ("an unknown option", ("--retries", "2", "libfontconfig1-dev")),
            ("a malformed option value", ("--bound", "soon", "libfontconfig1-dev")),
            ("an option with no value", ("libfontconfig1-dev", "--bound")),
            ("no attempt", ("--attempts", "0", "libfontconfig1-dev")),
            ("no bound", ("--bound", "0", "libfontconfig1-dev")),
        ):
            with self.subTest(call=name), tempfile.TemporaryDirectory() as tmp:
                code, stderr, calls = self.run_install(tmp, "exit 0", *args)
                self.assertEqual(code, 2)
                self.assertIn("usage:", stderr)
                self.assertEqual(calls, [])


if __name__ == "__main__":
    unittest.main()
