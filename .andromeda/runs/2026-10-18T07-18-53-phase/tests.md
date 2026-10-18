# tests extract

## Relevance
relevant — the chunk moves a time read under the harness's `screenshot`, `settle` and the driver's `advance`, each pinned by stand checks, the repeat-shot check and the stand-repeat budget.

## Constraints
- Every new pipeline check runs in both layout modes and the incremental/non-incremental oracle stays green: the differential oracle compares `final_layout()`, `layout_children` and the paint tree after each step (per test-plan §2 Differential oracle).
- A check that drives a held session calls no sleep and reads no wall clock, and in the Timer flow no pass runs but the driver's (per test-plan §5 Session ↔ held instance). `settle` reads no clock but the harness's and advances no time (per test-plan §3 Pump semantics). Whether the engine already reads only that clock on these paths is research's question.
- `Harness::screenshot` / `screenshot_rect` resolve at the harness's own time, with no tick and no pump (per test-plan §3 Core). The scope's lean that `Harness::time` becomes the one clock has to keep a shot a non-advancing read.
- The `advance` contract: `with_time(step)` hands the session its caller's time step, and a session without one refuses `advance` as `time-unavailable` (per test-plan §3 session-start). Today the Time row says `advance` "leaves the harness's animation clock where it was" (per test-plan §8 Hand-written fakes and stubs, Time (seven_guis timer) row). Both statements move if `advance` moves the one clock, and whether non-Timer tasks then answer `advance` is the P4 founder fork.
- A per-package build of blitz-test-harness paints no overlay scrollbars, because it takes blitz-paint with `default-features = false`. A green per-package run is also not the workspace leg's green. The repeat-shot proof has to run in a build that paints the thumb, or it passes vacuously (per test-plan §9 Engine features by runner).
- The stand checks hold a zero-failure budget: `stand-repeat` runs three rounds with no retry, so a time-dependent flake in a new check fails the leg (per test-plan §10 Flakiness budget).
- Coverage floors hold per crate: escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 % (per test-plan §10 Coverage floors).

## Patterns to follow
- The byte-identical frame check is `stand_screenshot_repeat`. It re-runs the test binary's own `#[ignore]` child and compares 12 shots across the child runs and the parent, in both layout modes under both builds. PNGs are decoded through blitz-tests' `png` dev-dependency (per test-plan §2 Pixel tests).
- The fade unit test is `opacity_holds_through_the_fade_delay_then_fades_out` in `packages/blitz-dom/src/node/scrollbar.rs`, which asserts opacity at fixed durations. Extend it at fixed virtual instants rather than adding wall-time waits (per test-plan §4 What unit tests cover, blitz-dom (node)).
- `advance` checks with no sleep:
  - `stand_act_timer`: advancing 300 / 50 / 250 reads `advanced_ms` 300 / 0 / 200, and `time-unavailable` is checked on a session with no step (per test-plan §1 Coverage scope, tests/blitz-tests).
  - `host_timer`: a hosted `advance` through the binary (per test-plan §6 Command line).
- Session checks share `tests/session_common/mod.rs`: `hold(task, incremental)`, the call builders, the `screenshot` builders and the `shot` reader. A new session check reuses them rather than its own copies (per test-plan §3 Crate-local test helpers, blitz-tests (session checks)).
- A new `tests/blitz-tests/tests/stand_*.rs` file is its own auto-discovered target. `agent-run.sh run stand` and the `stand-repeat` leg pick it up by prefix (per test-plan §3 `run`; per test-plan §9 Legs).

## Anti-patterns to avoid
- No sleep and no wall-clock read in the stand and session checks (per test-plan §5 Session ↔ held instance). The matrix acceptance asks for "wall time passing" between two shots, so how a check makes wall time pass without breaking this rule is a P4 technical fork. One option is a minimal fixture outside the session checks.
- Proving "repeat shots identical" in a build that never paints overlay scrollbars, the per-package harness build included (per test-plan §9 Engine features by runner).
- No retry, `--ignored` or env var to absorb a timing flake in `stand-repeat` (per test-plan §10 Flakiness budget).

## Contract bindings
- tests ↔ obs:
  - Each call through `Session::run` leaves exactly one `escher_driver` `command` line of fixed words and counts. A sink capture counts lines by target, never in total.
  - `host_log` / `mcp_log` keep their 0-needle reads. An `advance` that now moves the engine's clock must add no log line that carries a time a screen reads.
  - Per test-plan §5 Session ↔ held instance and §9 Engine features by runner.
- tests ↔ driver schema (arch):
  - The `screenshot` help's fade-limit sentence is the schema's fixed `'static` text (per test-plan §9 Engine features by runner).
  - Removing or rewriting it moves whatever pins the help and `describe()` text: escher-driver's `schema_tests.rs` / `describe.rs` unit tests, and `cli_commands`' `help` equal to `describe()`.
  - Which assertions hold that sentence verbatim is research's question.
- tests ↔ a11y: `stand_keyboard` and `stand_contrast` run in the `a11y` and `stand-repeat` legs, and they re-prove that a clock change moves no focus order, activation or contrast row (per test-plan §9 Legs).

## Acceptance criteria contributions
- The repeat-shot proof:
  - Two `screenshot`s of the same state, taken right after a scroll with wall time passing between them and no `advance`, read byte-identical PNGs in a build that paints overlay scrollbars, in both layout modes.
  - The same check shows that build paints the thumb: a shot after an `advance` past the fade differs from one inside it.
  - Per test-plan §2 Pixel tests and §9 Engine features by runner.
- `stand_act_timer` and the Time row are updated, not dropped: the check's "the harness clock reading the same before and after every call" assertion is restated to the one-clock rule P4 rules. `advanced_ms` 300 / 0 / 200 and `time-unavailable` (or its P4 replacement) still pass in both layout modes (per test-plan §8 Hand-written fakes and stubs, Time (seven_guis timer) row; per test-plan §3 session-start).
- `bash .github/scripts/ci-leg.sh stand-repeat` ends `stand-repeat: 3 of 3 rounds green` with the new check counted in it (per test-plan §10 Flakiness budget).
- The `coverage` leg ends `coverage floor: 3 of 3 crates at or above their floors` (per test-plan §10 Coverage floors).
