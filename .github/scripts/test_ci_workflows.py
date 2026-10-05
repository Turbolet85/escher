#!/usr/bin/env python3
"""Invariants of the fork's workflows and of ci-leg.sh. Run with `python3 -m unittest discover -s .github/scripts`."""

import os
import re
import stat
import subprocess
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"
LEG_SCRIPT = ROOT / ".github" / "scripts" / "ci-leg.sh"

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
}
NON_COMPILING_JOBS = {"fmt", "ci-scripts"}
LEG_STEP = re.compile(r"^bash \.github/scripts/ci-leg\.sh (\S+)$")


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
        for job_id, job in self.jobs.items():
            if job_id in NON_COMPILING_JOBS:
                continue
            with self.subTest(job=job_id):
                caches = [s for s in job["steps"] if s.get("uses") == "Swatinem/rust-cache@v2"]
                self.assertEqual(len(caches), 1)
                save_if = str(caches[0].get("with", {}).get("save-if", ""))
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
                uploads = [
                    s
                    for s in self.jobs[job_id]["steps"]
                    if str(s.get("uses", "")).startswith("actions/upload-artifact@")
                ]
                self.assertEqual(len(uploads), 1)
                upload = uploads[0]
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


class UpstreamGuardTest(unittest.TestCase):
    def test_f_every_upstream_job_is_repository_guarded(self):
        for name in ("publish-browser.yml", "wpt.yml", "wpt-post-results.yml"):
            for job_id, job in load(name)["jobs"].items():
                with self.subTest(workflow=name, job=job_id):
                    self.assertIn(UPSTREAM_GUARD, str(job.get("if", "")))


class LegScriptTest(unittest.TestCase):
    MARKER = "ci-leg-shim-marker"

    def run_leg(self, cwd, leg, cargo_exit):
        bin_dir = Path(cwd) / "bin"
        bin_dir.mkdir(exist_ok=True)
        shim = bin_dir / "cargo"
        shim.write_text(f"#!/bin/sh\necho {self.MARKER}\nexit {cargo_exit}\n", encoding="utf-8")
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

    def test_unknown_leg_exits_2(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.assertEqual(self.run_leg(tmp, "no-such-leg", 0).returncode, 2)


if __name__ == "__main__":
    unittest.main()
