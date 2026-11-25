#!/usr/bin/env python3
"""Invariants of the fork's workflows, of ci-leg.sh and of apt-install.sh. Run with `python3 -m unittest discover -s .github/scripts`."""

import os
import re
import shlex
import signal
import stat
import subprocess
import sys
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
    "stand-repeat": "stand-repeat",
}
# cargo-deny resolves metadata and compiles nothing.
NON_COMPILING_JOBS = {"fmt", "ci-scripts", "audit"}
# Uncached by the overseer's ruling: an instrumented target would evict the other legs' entries
# from the fork's near-full Actions cache.
NO_CACHE_JOBS = {"coverage"}
# Restores the test job's cache under its shared key and never saves a second entry.
RESTORE_ONLY_JOBS = {"a11y", "stand-repeat"}
CACHE_ACTION = "Swatinem/rust-cache@"
PINNED_USES = re.compile(r"^[^/@\s]+/[^@\s]+@[0-9a-f]{40}$")
USES_LINE = re.compile(r"^\s*(?:-\s*)?uses:")
PINNED_USES_LINE = re.compile(r"^\s*(?:-\s*)?uses:\s*[^/@\s]+/[^@\s]+@[0-9a-f]{40} # \S+\s*$")
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
    "stand-repeat",
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

    def test_free_disk_only_on_test_and_stand_repeat(self):
        def free_disk_steps(job):
            return [step for step in job["steps"] if "free-disk-space" in str(step.get("uses", ""))]

        self.assertEqual(free_disk_steps(self.jobs["matrix_test"]), [])
        self.assertEqual(len(free_disk_steps(self.jobs["stand-repeat"])), 1)
        self.assertEqual(len(free_disk_steps(self.jobs["test-features-default"])), 1)

    def test_test_job_reads_its_disk(self):
        steps = self.jobs["test-features-default"]["steps"]
        runs = [str(step.get("run", "")).strip() for step in steps]
        leg = runs.index("bash .github/scripts/ci-leg.sh test")
        before = [i for i, run in enumerate(runs[:leg]) if run == "df -h"]
        after = [leg + 1 + i for i, run in enumerate(runs[leg + 1 :]) if run == "df -h"]
        self.assertEqual(len(before), 1)
        self.assertEqual(len(after), 1)
        self.assertEqual(steps[after[0]].get("if"), "always()")

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

    def test_p_matrix_builds_skia_from_the_verified_archive(self):
        steps = self.jobs["matrix_test"]["steps"]
        names = [step.get("name") for step in steps]
        verify, test, witness = (
            names.index("Verified skia archive"),
            names.index("test"),
            names.index("Skia built from the verified archive"),
        )
        self.assertLess(names.index("Setup"), verify)
        self.assertEqual((verify + 1, test + 1), (test, witness))
        for step in (steps[verify], steps[witness]):
            with self.subTest(step=step["name"]):
                # No `if:`, so it stands on every matrix row; a plain shell step, no action.
                self.assertNotIn("if", step)
                self.assertNotIn("uses", step)
                self.assertEqual(step.get("shell"), "bash")
        run = steps[verify]["run"]
        for word in (".github/scripts/skia_archive.py ${{ matrix.platform.target }} target/skia-archives",
                     "SKIA_BINARIES_URL=", '"$GITHUB_ENV"', "'python'", "'python3'"):
            self.assertIn(word, run)
        witness_run = steps[witness]["run"]
        # Read by content across skia-bindings' whole build tree, its layout and output file varying by Cargo
        # release; each match printed, and the tree printed with sizes when none read the archive.
        for word in ("/usr/bin/find target/${{ matrix.platform.target }} -path '*skia-bindings*/out' -prune -o "
                     "-type f -path '*skia-bindings*' -size -1024k -print",
                     "grep -I -m1 'FROM: file://'", 'echo "skia-archive: build output $f:$line"',
                     "grep -I -q 'DOWNLOAD AND INSTALL SUCCEEDED'", 'if [ "$found" = 1 ]; then exit 0; fi',
                     "-prune -exec ls -ld {} + \\) -o \\( -path '*skia-bindings*' -exec ls -ld {} + \\) >&2",
                     "exit 1"):
            self.assertIn(word, witness_run)
        self.assertNotIn("-name output", witness_run)
        self.assertNotIn("/debug/build/skia-bindings-*/output", witness_run)
        # In a crates.io build the flag sends skia-bindings to a full source build.
        self.assertNotIn("FORCE_SKIA_BINARIES_DOWNLOAD", self.text)


class WorkflowPinTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.files = sorted(p for p in WORKFLOWS.iterdir() if p.suffix in (".yml", ".yaml"))

    def test_the_pin_check_reads_every_workflow(self):
        self.assertTrue({"ci.yml", "wpt.yml", "publish-browser.yml", "wpt-post-results.yml"} <= {p.name for p in self.files})

    def test_j_every_action_is_pinned_to_a_commit_sha(self):
        for path in self.files:
            for job_id, job in load(path.name)["jobs"].items():
                uses = [job["uses"]] if "uses" in job else []
                uses += [step["uses"] for step in job.get("steps", []) if "uses" in step]
                for ref in uses:
                    with self.subTest(workflow=path.name, job=job_id, uses=ref):
                        self.assertRegex(ref, PINNED_USES)

    def test_every_pin_keeps_its_ref_as_a_trailing_comment(self):
        # YAML drops comments, so the live `uses:` lines are read as text; a commented-out step is not live.
        for path in self.files:
            for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
                if USES_LINE.match(line):
                    with self.subTest(workflow=path.name, line=number):
                        self.assertRegex(line, PINNED_USES_LINE)

    def test_k_pinned_rust_toolchain_names_its_toolchain(self):
        # A SHA pin loses the toolchain that the `@stable` branch name used to select.
        for path in self.files:
            for job_id, job in load(path.name)["jobs"].items():
                for step in job.get("steps", []):
                    if str(step.get("uses", "")).startswith("dtolnay/rust-toolchain@"):
                        with self.subTest(workflow=path.name, job=job_id):
                            self.assertTrue(str(step.get("with", {}).get("toolchain", "")))


class UpstreamGuardTest(unittest.TestCase):
    def test_f_every_upstream_job_is_repository_guarded(self):
        for name in ("publish-browser.yml", "wpt.yml", "wpt-post-results.yml"):
            for job_id, job in load(name)["jobs"].items():
                with self.subTest(workflow=name, job=job_id):
                    self.assertIn(UPSTREAM_GUARD, str(job.get("if", "")))


class LegScriptTest(unittest.TestCase):
    MARKER = "ci-leg-shim-marker"

    SKIA_LEGS = ("clippy", "test", "build", "msrv", "doc", "coverage", "a11y")
    SKIA_SHIM_URL = "file:///skia-shim/"

    def run_leg(self, cwd, leg, cargo_exit, echo_args=False, skia_exit=0, env_extra=None):
        bin_dir = Path(cwd) / "bin"
        bin_dir.mkdir(exist_ok=True)
        shim = bin_dir / "cargo"
        # With echo_args the shim also prints the skia variables cargo runs under.
        skia_env = '"SKIA_BINARIES_URL=${SKIA_BINARIES_URL-unset}" "FORCE=${FORCE_SKIA_BINARIES_DOWNLOAD-unset}"'
        echo = f'echo {self.MARKER} "$@" {skia_env}' if echo_args else f"echo {self.MARKER}"
        shim.write_text(f"#!/bin/sh\n{echo}\nexit {cargo_exit}\n", encoding="utf-8")
        shim.chmod(shim.stat().st_mode | stat.S_IXUSR)
        python_shim = bin_dir / "python3"
        if not python_shim.exists():
            # skia_archive.py answers its target's stand-in URL; every other script runs on this interpreter.
            answer = f'echo "{self.SKIA_SHIM_URL}$2"; ' if skia_exit == 0 else ""
            python_shim.write_text(
                f'#!/bin/sh\ncase "$1" in *skia_archive.py) {answer}exit {skia_exit} ;; esac\n'
                f'exec {shlex.quote(sys.executable)} "$@"\n',
                encoding="utf-8",
            )
            python_shim.chmod(python_shim.stat().st_mode | stat.S_IXUSR)
        env = dict(os.environ, PATH=f"{bin_dir}{os.pathsep}{os.environ['PATH']}")
        env.pop("SKIA_BINARIES_URL", None)
        env.pop("FORCE_SKIA_BINARIES_DOWNLOAD", None)
        env.update(env_extra or {})
        return subprocess.run(
            ["bash", str(LEG_SCRIPT), leg], cwd=cwd, env=env, capture_output=True, text=True
        )

    def leg_log(self, leg, skia_exit=0, env_extra=None):
        """Runs the leg with cargo exiting 0 and echoing its arguments; returns its exit and its log's lines."""
        with tempfile.TemporaryDirectory() as tmp:
            proc = self.run_leg(tmp, leg, 0, echo_args=True, skia_exit=skia_exit, env_extra=env_extra)
            log = (Path(tmp) / "target" / "ci-logs" / f"{leg}.log").read_text(encoding="utf-8")
            return proc.returncode, log.splitlines()

    def cargo_lines(self, lines):
        return [line for line in lines if line.startswith(self.MARKER)]

    def test_skia_legs_build_from_the_verified_archive(self):
        for leg in self.SKIA_LEGS:
            with self.subTest(leg=leg):
                _, lines = self.leg_log(leg)
                cargo = self.cargo_lines(lines)
                self.assertTrue(cargo)
                for line in cargo:
                    url = re.search(r"SKIA_BINARIES_URL=(\S+)", line).group(1)
                    self.assertTrue(url.startswith(self.SKIA_SHIM_URL), line)
                    self.assertGreater(len(url), len(self.SKIA_SHIM_URL), line)
                    # The flag would send a crates.io build to skia-bindings' source build.
                    self.assertIn("FORCE=unset", line.split())

    def test_a_callers_skia_url_is_overwritten_by_the_verified_one(self):
        _, lines = self.leg_log("build", env_extra={"SKIA_BINARIES_URL": "https://example.com/{key}"})
        for line in self.cargo_lines(lines):
            self.assertNotIn("example.com", line)
            self.assertIn(f"SKIA_BINARIES_URL={self.SKIA_SHIM_URL}", line)

    def test_legs_that_build_no_skia_set_no_skia_variable(self):
        for leg in ("fmt", "audit", "stand-repeat"):
            with self.subTest(leg=leg):
                code, lines = self.leg_log(leg)
                self.assertEqual(code, 0)
                cargo = self.cargo_lines(lines)
                self.assertTrue(cargo)
                for line in cargo:
                    self.assertIn("SKIA_BINARIES_URL=unset", line.split())
                    self.assertIn("FORCE=unset", line.split())

    def test_a_refused_archive_ends_the_leg_before_cargo(self):
        code, lines = self.leg_log("build", skia_exit=1)
        self.assertNotEqual(code, 0)
        self.assertEqual(self.cargo_lines(lines), [])

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

    def leg_log_after_red(self, leg):
        """Runs the leg with every cargo call exiting 1; returns its exit and its log's words."""
        with tempfile.TemporaryDirectory() as tmp:
            proc = self.run_leg(tmp, leg, 1, echo_args=True)
            log = (Path(tmp) / "target" / "ci-logs" / f"{leg}.log").read_text(encoding="utf-8")
            return proc.returncode, log.split()

    def test_doc_leg_documents_the_whole_workspace(self):
        words = self.leg_log_words("doc")
        for word in ("doc", "--workspace", "--no-deps", "--locked"):
            self.assertIn(word, words)

    def test_gate_legs_dispatch_to_cargo_and_write_their_log(self):
        for leg, expected in (
            ("audit", ("deny", "--locked", "--workspace", "--all-features", "advisories", "licenses")),
            ("a11y", ("--workspace", "--locked", "accessibility_hidden", "accessibility_roles", "focusability_updates",
                      "stand_keyboard", "stand_contrast", "stand_accessibility_ids", "accessibility_states",
                      "accessibility_label", "accessibility_description", "accessibility_merged_states",
                      "accessibility_active_descendant_unread", "inert_subtree", "select_interaction",
                      "stand_act_select", "key_press_after_container_lost_its_box", "focus_fixup_unshown", "select_popup",
                      "select_popup_painted", "stand_act_popup", "virtual_nodes_as_elements")),
        ):
            with self.subTest(leg=leg):
                words = self.leg_log_words(leg)
                self.assertIn(self.MARKER, words)
                for word in expected:
                    self.assertIn(word, words)

    def coverage_leg(self, tmp, driver_hit):
        """Runs the coverage leg over an lcov seeded in `tmp` (the shim writes none), with the dioxus-native-dom
        and blitz-test-harness crates exactly at their floors and escher-driver at `driver_hit` lines of 100,
        so the run reaches the floor script and its three committed floors decide the leg's exit."""
        lcov = Path(tmp) / "target" / "coverage" / "lcov.info"
        lcov.parent.mkdir(parents=True)
        lcov.write_text(
            "".join(
                f"SF:{tmp}/packages/{crate}/src/lib.rs\nLF:100\nLH:{hit}\nend_of_record\n"
                for crate, hit in (("escher-driver", driver_hit), ("dioxus-native-dom", 83), ("blitz-test-harness", 90))
            ),
            encoding="utf-8",
        )
        proc = self.run_leg(tmp, "coverage", 0, echo_args=True)
        return proc.returncode, (Path(tmp) / "target" / "ci-logs" / "coverage.log").read_text(encoding="utf-8")

    def test_coverage_leg_reaches_the_floor_on_each_crate(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, log = self.coverage_leg(tmp, 97)
            self.assertEqual(code, 0, log)
            for word in ("llvm-cov", "--workspace", "--locked", "--ignore-filename-regex", "--lcov"):
                self.assertIn(word, log.split())
            self.assertIn("packages/escher-driver 97/100 = 97.00 % (floor 97 %)", log)
            self.assertIn("packages/dioxus-native-dom 83/100 = 83.00 % (floor 83 %)", log)
            self.assertIn("packages/blitz-test-harness 90/100 = 90.00 % (floor 90 %)", log)
            self.assertEqual(log.splitlines()[-1], "coverage floor: 3 of 3 crates at or above their floors")

    def test_coverage_leg_fails_under_a_floor(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, log = self.coverage_leg(tmp, 96)
            self.assertEqual(code, 1, log)
            self.assertEqual(log.splitlines()[-1], "coverage floor: 1 of 3 crates under their floors")

    def test_clippy_leg_lints_the_workspace_then_upstream_blitz_dom_line(self):
        words = self.leg_log_words("clippy")
        self.assertEqual(words.count(self.MARKER), 2)
        for word in ("clippy", "--workspace", "--locked", "-p", "blitz-dom", "--no-default-features",
                     "svg,woff,accessibility,system-fonts,file-input,custom-widget", "--all-targets", "-D", "warnings"):
            self.assertIn(word, words)
        code, words = self.leg_log_after_red("clippy")
        self.assertNotEqual(code, 0)
        self.assertEqual(words.count(self.MARKER), 1)
        self.assertNotIn("blitz-dom", words)

    def test_fast_leg_ends_on_the_font_embolden_witness(self):
        with tempfile.TemporaryDirectory() as tmp:
            # `fast` also runs the ci-scripts leg, which would discover nothing from this directory.
            bin_dir = Path(tmp) / "bin"
            bin_dir.mkdir()
            python_shim = bin_dir / "python3"
            python_shim.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            python_shim.chmod(python_shim.stat().st_mode | stat.S_IXUSR)
            proc = self.run_leg(tmp, "fast", 0, echo_args=True)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            words = (Path(tmp) / "target" / "ci-logs" / "embolden.log").read_text(encoding="utf-8").split()
        for word in ("test", "-p", "blitz-tests", "--locked", "--no-fail-fast", "--features",
                     "blitz-paint/font-embolden", "--test", "select_popup_painted"):
            self.assertIn(word, words)
        self.assertEqual(words.count(self.MARKER), 1)

    def test_fast_leg_sweeps_under_the_verified_archive(self):
        with tempfile.TemporaryDirectory() as tmp:
            # Every python3 call but these two exits 0, so the ci-scripts leg discovers nothing from here.
            bin_dir = Path(tmp) / "bin"
            bin_dir.mkdir()
            seen = Path(tmp) / "sweep-env"
            python_shim = bin_dir / "python3"
            python_shim.write_text(
                "#!/bin/sh\ncase \"$1\" in\n"
                f'  *skia_archive.py) echo "{self.SKIA_SHIM_URL}$2" ;;\n'
                f'  *target-sweep.py) echo "SKIA_BINARIES_URL=${{SKIA_BINARIES_URL-unset}}" > {shlex.quote(str(seen))} ;;\n'
                "esac\nexit 0\n",
                encoding="utf-8",
            )
            python_shim.chmod(python_shim.stat().st_mode | stat.S_IXUSR)
            proc = self.run_leg(tmp, "fast", 0, env_extra={"CI": ""})
            self.assertEqual(proc.returncode, 0, proc.stderr)
            line = seen.read_text(encoding="utf-8").strip()
            self.assertTrue(line.startswith(f"SKIA_BINARIES_URL={self.SKIA_SHIM_URL}"), line)
            self.assertGreater(len(line), len(f"SKIA_BINARIES_URL={self.SKIA_SHIM_URL}"), line)

    def test_stand_repeat_leg_runs_three_rounds(self):
        with tempfile.TemporaryDirectory() as tmp:
            proc = self.run_leg(tmp, "stand-repeat", 0, echo_args=True)
            self.assertEqual(proc.returncode, 0, proc.stderr)
            log = (Path(tmp) / "target" / "ci-logs" / "stand-repeat.log").read_text(encoding="utf-8")
        words = log.split()
        self.assertEqual(words.count(self.MARKER), 6)
        for word in ("blitz-tests", "seven_guis", "--locked", "stand_settle", "cli_flow", "mcp_server",
                     "recording", "stand_session_recording", "stand_replay", "inert_subtree",
                     "accessibility_states", "scrolled_box_absolute_position", "hit_clipped_at_scrolling_box",
                     "password_caret_click", "transformed_client_rect", "accessibility_active_descendant_unread",
                     "virtual_clock_engine", "select_interaction", "text_extent", "visible_region_transformed",
                     "hit_non_invertible_transform", "open_state", "checkable_change_event",
                     "text_range_no_change_event", "select_popup", "select_popup_painted"):
            self.assertIn(word, words)
        self.assertNotIn("--ignored", words)
        self.assertEqual(log.splitlines()[-1], "stand-repeat: 3 of 3 rounds green")

    def test_stand_repeat_leg_ends_at_its_first_red(self):
        code, words = self.leg_log_after_red("stand-repeat")
        self.assertNotEqual(code, 0)
        self.assertEqual(words.count(self.MARKER), 1)
        self.assertNotIn("green", words)

    def test_unknown_leg_exits_2(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.assertEqual(self.run_leg(tmp, "no-such-leg", 0).returncode, 2)


class InstallScriptTest(unittest.TestCase):
    STALL_SECONDS = 300
    WAIT_SECONDS = 30

    def run_install(self, cwd, behaviour, *args, repair="exit 0"):
        """Runs the script against a stand-in `sudo`, `apt-get` and `dpkg`; returns its exit, stderr and the
        apt-get and dpkg calls in order, a dpkg call prefixed `dpkg`."""
        bin_dir = Path(cwd) / "bin"
        bin_dir.mkdir()
        calls = Path(cwd) / "calls"
        shims = {
            "sudo": 'exec "$@"',
            "apt-get": f'echo "$*" >> {shlex.quote(str(calls))}\n{behaviour}',
            "dpkg": f'echo "dpkg $*" >> {shlex.quote(str(calls))}\n{repair}',
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

    def test_a_healthy_install_runs_no_repair(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, _, calls = self.run_install(tmp, "exit 0", "libfontconfig1-dev")
            self.assertEqual(code, 0)
            self.assertEqual([c for c in calls if c.startswith("dpkg")], [])

    def test_a_failed_attempt_is_tried_again_from_update(self):
        with tempfile.TemporaryDirectory() as tmp:
            calls_file = shlex.quote(str(Path(tmp) / "calls"))
            fail_first = f'[ "$(wc -l < {calls_file})" -gt 1 ] || exit 100\nexit 0'
            code, stderr, calls = self.run_install(tmp, fail_first, "libfontconfig1-dev")
            self.assertEqual(code, 0)
            self.assertEqual(calls, ["update", "dpkg --configure -a", "update", "install -y libfontconfig1-dev"])
            self.assertIn("attempt 1 of 3 failed in update (exit 100)", stderr)

    def test_an_install_failed_by_its_bound_is_repaired_before_the_next_attempt(self):
        with tempfile.TemporaryDirectory() as tmp:
            calls_file = shlex.quote(str(Path(tmp) / "calls"))
            fail_first_install = f'[ "$1" = install ] && [ "$(wc -l < {calls_file})" -eq 2 ] && exit 124\nexit 0'
            code, stderr, calls = self.run_install(tmp, fail_first_install, "libfontconfig1-dev")
            self.assertEqual(code, 0)
            self.assertEqual(
                calls,
                ["update", "install -y libfontconfig1-dev", "dpkg --configure -a", "update",
                 "install -y libfontconfig1-dev"],
            )
            self.assertIn("attempt 1 of 3 failed in install (exit 124)", stderr)

    def test_a_failed_repair_fails_its_attempt_in_phase_repair(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, stderr, calls = self.run_install(tmp, "exit 100", "--attempts", "2", "libfontconfig1-dev",
                                                   repair="exit 1")
            self.assertEqual(code, 1)
            self.assertEqual(calls, ["update", "dpkg --configure -a"])
            self.assertIn("attempt 2 of 2 failed in repair (exit 1)", stderr)

    def test_every_attempt_failing_exits_1_after_three_attempts(self):
        with tempfile.TemporaryDirectory() as tmp:
            code, stderr, calls = self.run_install(tmp, "exit 100", "libfontconfig1-dev")
            self.assertEqual(code, 1)
            self.assertEqual(calls, ["update", "dpkg --configure -a", "update", "dpkg --configure -a", "update"])
            self.assertIn("attempt 3 of 3 failed in update (exit 100)", stderr)

    def test_a_stalled_call_ends_at_its_bound_and_is_tried_again(self):
        with tempfile.TemporaryDirectory() as tmp:
            started = time.monotonic()
            code, stderr, calls = self.run_install(
                tmp, f"exec sleep {self.STALL_SECONDS}", "--bound", "1", "--attempts", "2", "libfontconfig1-dev"
            )
            self.assertEqual(code, 1)
            self.assertEqual(calls, ["update", "dpkg --configure -a", "update"])
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
