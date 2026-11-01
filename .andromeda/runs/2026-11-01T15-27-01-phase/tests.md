# tests extract

## Relevance
relevant

## Constraints
- The take's own green CI run has its test-bearing job logs re-read and tallied per OS by the plan's recorded method, its ANSI strip included, the a11y job and the stand repeat beside the OS jobs, each moved count attributed by name against the newest recorded reading; the expected shape between OSes is the doc-test lines and the unix-gated binaries (per test-plan §9 Per-OS test tallies; detail: .andromeda/test-plan-detail/ci-integration.md).
- Upstream's new and edited test files stand under the fork's layout, one auto-discovered target per file, never upstream's one `all` target (per test-plan §2 Test directory + naming conventions → Directory pattern). `tests/blitz-tests/Cargo.toml` must still read as `BlitzTestsTargetsTest` pins it, which a take of upstream's manifest would otherwise break; whether each candidate shape touches that manifest is research's question (per test-plan §4 blitz-tests targets).
- A green `-p blitz-tests` run does not stand in for the workspace leg: the two builds differ in engine features, so a sink-capture check counts by target, never in total, and each body of the bounds reader, which `layout/writing_mode.rs`'s predicted conflict touches, holds a check (per test-plan §9 Engine features by runner).
- The stand checks, the named blitz-tests targets and seven_guis' process checks hold a zero-failure budget over three rounds with no retry, and that budget binds after the take; whether an upstream target joins the `--test` list is research's question (per test-plan §10 Flakiness budget).
- The three per-crate line floors hold after the take. The `keyboard_compat.rs` exclusion lifts only when escher takes upstream's retirement of it in `286874cc1`, so that exclusion's fate follows the founder's fork (per test-plan §10 Coverage floors; detail: .andromeda/test-plan-detail/quality-gates-coverage-targets.md).
- The sink re-check is read through the standing log checks: stderr at the plan's stated levels holds no stable id, accessible name or typed text, a target outside the allowlist is dropped whole, and any line count the taken delta moves is re-read and explained (per test-plan §3 Stand log format).
- A CI or leg change this chunk folds, the `font-embolden` local gate or the matrix job's free-disk `if:`, stays inside `test_ci_workflows.py`'s pinned invariants and its leg-under-a-`cargo`-shim tests, and is extended there where a pin names the changed shape (per test-plan §4 CI workflows and leg script). Whether a local gate already builds `blitz-paint/font-embolden` and whether the free-disk step still skips on every row are research's questions (per test-plan §9 Red on `Test (macos)` and §9 The matrix job's free-disk step never fires; detail: .andromeda/test-plan-detail/ci-integration.md).

## Patterns to follow
- Each upstream sync re-counts the agent harness: `bash scripts/agent-run.sh run stand` and `run all`, the `run.end` line recorded, a new upstream file with no `stand_` prefix named as picked up by `run all` with no script change (per test-plan §3 Agent-run contract → Proof).
- The stand checks and the incremental oracle run in both layout modes; the differential oracle compares incremental and non-incremental documents after each step, the read that `fe33b778b`'s narrower restyle invalidation and `8b1a0a3ce`'s layout damage must pass if taken (per test-plan §2 Differential oracle).
- A restyle-only regression check drives `:hover`, not an attribute mutation, because a mutation inserts full damage and masks an under-damaging bug (per test-plan §2 Pure-restyle paths).
- A regression test is named for the bug it guards and documents it, the form for a `node.rs:1775` panic check should the founder's fork take a fix (per test-plan §2 Test directory + naming conventions → Test function naming).
- Upstream's checks may assert through test-that's `verify_that!` under the standard harness; a further third-party test crate has the hand review that bump had, and no new test framework is implied (per test-plan §3 Runners and invocation → Rust tests and Third-party test frameworks).

## Anti-patterns to avoid
- Retrying, passing `--ignored` or setting an env var to green the `stand-repeat` leg (per test-plan §9 Legs).
- Adding a skip path to a headless-stand check, or to a shot whose font context resolves no `sans-serif`, to get past a moved font or layout number (per test-plan §2 Font-dependent tests).
- Grading a criterion on a WPT reading taken on the dev host; that reading is report-only (per test-plan §2 WPT conformance).

## Contract bindings
- tests §3 Stand log format ↔ obs §3 and §6: the sink's allowlist and the log fields the standing captures read; the delta's new `tracing`/`log` targets are judged against both.
- tests §3 Agent-run contract ↔ obs §3: the harness's events are harness metadata, not an escher-telemetry sink, and `scripts/agent-run.sh` stays outside the sink re-check.
- tests §9 Legs (`ci-leg.sh a11y`) ↔ a11y §Bootstrap: the a11y assertions run in the same workflow; the leg's file list and tally are re-read with the per-OS tallies.
- tests §9 Legs (`ci-leg.sh audit`) ↔ security §Dependency Security: the audit runs in the same workflow; a test crate entering the lock answers to both this and tests §3 Third-party test frameworks.

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh fast` (its `ci-scripts` step carrying `BlitzTestsTargetsTest` and `test_ci_workflows.py`), `doc`, `a11y` and `audit` each exit 0 on the dev host, and fork CI is green on the pushed commit with every job settled (per test-plan §9 Legs).
- A per-OS tally of the take's green CI run is recorded with a comparison against the newest recorded reading, every moved result line and passed count explained by name, and none left unattributed (per test-plan §9 Per-OS test tallies).
- `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` passes and a local gate runs a `font-embolden` build; a test in `test_ci_workflows.py` fails if that gate is removed (per test-plan §9 Red on `Test (macos)`).
- The `stand-repeat` leg reads `stand-repeat: 3 of 3 rounds green` and the `coverage` leg ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Flakiness budget and §10 Coverage floors).
