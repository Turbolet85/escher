# test-plan · the rows that bind 2026-11-21-dependency-and-ci-debts — picked 45 of 642 (30 by every draw) · rule any · pin 2957

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/native-platform-ci-matrix · Native platform CI matrix — CI tests on windows and macos and only builds on ios and android; linux is tested by the fast test leg, not the matrix.
decision/workspace-tests-in-fast-ci-leg · Workspace tested in the fast CI leg — Every CI run tests the whole workspace with a locked lockfile on ubuntu with default features, as the fast test leg.
decision/wpt-run-on-host-on-fork · WPT run on the host on the fork — The CI wpt job runs css and svg only on the upstream repository; on the fork, Web Platform Tests for css and svg are run on the host.
decision/ci-package-installs-bounded · CI package installs bounded — Every package install of the CI workflow is bounded, a workflow invariant held by the CI scripts' unit tests. · guard: test_ci_workflows.py
decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/bounded-ci-package-installs · Bounded CI package installs — Every package install in the CI workflow is bounded. · guard: test_ci_workflows.py
decision/upstream-buffered-render-test-kept-as-written · Upstream buffered-render test kept as written — Upstream's test of a task polled during render accessing the document stays in the fork as upstream wrote it, and the runtime probe's two spawned-task pins are the fix's discriminating witness on Dioxus 0.7. · why: On Dioxus 0.7 the upstream test is insensitive to the borrow; it guards the Dioxus 0.8 scheduling it was written for. · guard: dioxus_runtime_probe

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/msrv-verified-by-build · MSRV verified by build only — The minimum supported Rust version is verified by build only, not by test.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/capability-claims-name-their-checks · Capability claims name their checks — The capabilities document ends naming the check behind each claim it makes, and states each limit as pinned by its check or as asserted by no check.

## 3-test-harness-contract · ## 3. Test Harness Contract
decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
decision/wpt-suites-run-in-ci · WPT suites run in CI — CI builds the WPT runner in release mode and runs its css and svg suites, producing a JSON WPT report that the wpt cli then scores and diffs.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/ci-actions-pinned-to-sha · CI actions pinned to commit SHAs — Every action every CI workflow uses is pinned to a commit SHA. · guard: test_ci_workflows.py
decision/ci-legs-through-leg-script · CI legs run through the leg script — Each linux CI job runs its leg through `ci-leg.sh`, the leg present in the script's list, with a failure-log upload and no perl rewrite. · guard: CiWorkflowTest in test_ci_workflows.py
decision/ci-no-secrets · CI uses no secrets — The CI workflow references no `secrets.`. · guard: test_ci_workflows.py
decision/ci-read-only-permissions · CI read-only permissions — The CI workflow grants `contents: read` at workflow level and no job adds a grant. · guard: CiWorkflowTest in test_ci_workflows.py

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/actions-pinned-to-commit-sha · CI actions pinned to commit SHAs — Every `uses:` in the CI workflows is pinned to a 40-hex commit SHA, and every Rust toolchain step names its toolchain. · guard: WorkflowPinTest
decision/ci-uses-no-secrets · CI uses no secrets — No CI workflow step reads a `secrets.` value. · guard: CiWorkflowTest
decision/failure-logs-uploaded · Leg logs uploaded on failure — Each leg job uploads its log on failure, kept 7 days. · guard: CiWorkflowTest
decision/fast-jobs-gate-the-rest · Fast jobs gate the rest — The four fast CI jobs need nothing and every other job needs all four. · guard: CiWorkflowTest
decision/bounded-package-installs · Bounded package installs in CI — No CI step runs `apt-get` directly: packages are installed through the install script with package names only, and the one action step that installs packages carries a `timeout-minutes` above 0. · guard: test_o_package_installs_are_bounded
decision/upstream-repository-guard · Upstream-only publishing jobs — Every job of publish-browser, wpt and wpt-post-results runs only in the `DioxusLabs/blitz` repository. · guard: UpstreamGuardTest
contract/ci-leg-exit-codes/2 · CI leg script exit codes · 2 — An unknown leg exits 2.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/build-directory-held-by-sweep · Build directory held by a sweep, resolutions kept — The build directory's growth is held by sweeping superseded unit hashes after a green fast leg, never by merging feature resolutions; the workspace, blitz-tests, embolden and seven_guis resolutions each stay a build of their own. · why: Cargo mints a unit hash per feature resolution, lockfile and toolchain epoch, never per source edit, and collects none it supersedes; each resolution is load-bearing, so merging two was measured only.
decision/cargo-legs-locked · Cargo legs run locked — Every cargo leg of CI runs with --locked. · except: fmt, and examples/wasm_hello, which has no Cargo.lock
decision/ci-cache-save-branches · Cache saved on main and build branches — CI caches are saved only on main and build/* branches.
decision/ci-legs-host-reproducible · Host-reproducible CI legs — Every linux CI job runs one leg of a shared leg script that the dev host also runs, so every linux leg is reproducible on the dev host.
decision/local-baseline · Local baseline for CI comparison — The local baseline is the dev host, the dev profile with line-tables-only debug info, workspace default features, every cargo command locked and no opt-level rewrite; cold is the first run after a clean, warm an immediate re-run. · why: It is the reference the fork's CI is compared against.
decision/local-pre-push-gate · Local pre-push gate — The fast leg runs fmt, clippy, test, ci-scripts and embolden in that order, stopping at the first red, as the local pre-push gate; on a dev host with CI unset, a green fast leg ends with the target sweep, whose refusal or failure never reddens the leg, and a red leg calls none.
decision/shared-test-cache · Shared workspace test cache — The test job saves the cache under shared-key workspace-test, which the a11y and stand-repeat jobs restore without saving.
contract/ci-artifacts/ci-log-job-id · CI logs and artifacts · ci-log-{job id} — The leg's log, uploaded only on failure and kept 7 days.
contract/ci-artifacts/leg-log · CI logs and artifacts · target/ci-logs/{leg}.log — Holds each leg's merged output, truncated at the leg's start.
decision/ci-fast-slow-split · Fast/slow CI split — fmt, clippy, test-features-default and ci-scripts run first with no dependencies, and every other job, the platform matrix included, needs all four.
decision/ci-rust-cache · SHA-pinned Rust cache — Every compiling CI job uses the SHA-pinned Swatinem/rust-cache. · except: coverage
decision/package-installs-bounded-retried · Bounded, retried package installs — Every package install of the CI workflow is bounded and retried: up to 3 attempts under a 120 s bound, each retry opening with a bounded dpkg repair. · why: the bound answers a package-install hang · except: The test matrix's one package install, through a SHA-pinned apt cache action, is bounded by its step timeout and not retried.
decision/per-package-green-not-workspace-green · Per-package green is not the workspace green — A green per-package test run is not the workspace test leg's green. · why: The two runs build the same test file against different engine features, tracing and writing-mode on in the workspace build only.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-shared-test-cache · Shared CI build cache — Every compiling CI job but coverage uses a SHA-pinned Swatinem/rust-cache saved on main and build branches; the a11y and stand-repeat jobs restore the test job's cache without saving.
decision/dependency-audit-advisories-and-licenses · Dependency audit — The audit leg checks advisories and licenses with cargo-deny over the whole locked workspace with all features.
decision/msrv-1-91 · Minimum supported Rust version — CI builds the workspace with Rust 1.91 as the minimum supported version.
decision/embolden-local-witness · Local witness for font-embolden paint — A paint change under the OS-gated font-embolden feature has its local witness in the embolden leg, the last of the fast legs, which has no CI job. · why: macOS exercises the feature in CI, and the leg's command reproduces macOS's numbers on linux. · guard: test_fast_leg_ends_on_the_font_embolden_witness; select_popup_painted
decision/free-disk-on-test-and-stand-repeat · Free disk on the test and stand-repeat jobs — The Stand checks repeated job and the default-features test job each free runner disk with a pinned free-disk action after the toolchain and before the cache; the matrix job holds no free-disk step. · guard: test_free_disk_only_on_test_and_stand_repeat
decision/package-installs-bounded-and-retried · Bounded, retried package installs — Every CI package install is bounded and retried: up to 3 attempts, each apt-get update then install under a 120 s bound with a kill grace, a bounded dpkg repair before each retry. · why: Answers a CI run whose package-install step hung twice. · guard: test_o_package_installs_are_bounded; InstallScriptTest · except: The matrix job's third-party Install apt deps action step carries timeout-minutes: 10 instead.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/complexity-ceiling-host-only · Cognitive-complexity ceiling read on the host — The cognitive-complexity ceiling of 15 is read on the dev host alone, test functions counted; no CI leg runs it, it binds no function, and engine functions over it are left as they are. · why: Ratified for upstream-authored functions: those over the ceiling are kept as upstream wrote them. · except: The engine's click handler is held below the ceiling, upstream's arms included, at the price of a hand re-apply when an upstream sync edits one of its arms.
decision/oversize-test-targets-kept · Oversize integration-test targets kept whole — Escher's integration-test targets over 800 code lines are kept whole, each one file and one target, with no route entry to split them. · why: Ratified; a split into several targets moves the pinned target layout the CI legs read.
decision/upstream-oversize-files-kept · Upstream-authored oversize files kept as upstream's — Upstream-authored files over 800 code lines are kept as upstream wrote them; the fork splits none of them. · why: Ratified; a split would be a standing divergence in the files every upstream sync touches most.
decision/upstream-test-clone-pairs-kept · Upstream's duplicated test pairs kept as upstream's — Upstream's cdp-server and vibey-script duplicated test pairs are kept as upstream wrote them, and the duplication reading is not held against them. · why: Ratified.

# test-plan · its absences, names and overviews, whole

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
> NOT YET MEASURED — the reading gathered no tier justification, critical-path list or coverage triggers for this summary

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
> NOT YET MEASURED — windowed or browser-driven end-to-end runs lay outside every slice read

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
absence/windowed-end-to-end-tests — An automated test that launches a window

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
absence/mocking-library — A mocking library

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
> NOT YET MEASURED — a performance budget: none is set

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
> NO RECORDED INTENT

## 12-test-decisions-log · ## 12. Test Decisions Log
> NO RECORDED INTENT

## session-lifecycle · key of ## 3. Test Harness Contract
overview/session-lifecycle — The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is no verb of the five-command agent-run contract, which itself starts and binds nothing; its run session selection runs the session binary's cli_flow flows as tests, their hosts started by the flows.
