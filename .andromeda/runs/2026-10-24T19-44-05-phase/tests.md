# tests extract

## Relevance
relevant — the chunk re-reads two stand checks (`stand_keyboard`, `stand_contrast`) and widens a CI leg and its pin, all inside tests' harness, CI and quality-gate sections.

## Constraints
- The `a11y` leg is one of the linux ci.yml jobs run through `bash .github/scripts/ci-leg.sh a11y`, named as literal `--test` words, `--locked`, host-reproducible; widening it means adding files to that `--test` list, never a new runner, job tier or env var (per test-plan §9 CI Integration → Legs; detail `.andromeda/test-plan-detail/ci-integration.md`, values · ci lines 8–10).
- The `a11y` job is restore-only on the test job's `shared-key: workspace-test` cache (`save-if: false`), and `test_ci_workflows.py` pins that and the a11y job's accessibility name; a widened leg keeps both (per test-plan §9 CI Integration → Cache; test-plan §4 Unit Test Strategy → CI workflows and leg script).
- `test_ci_workflows.py`'s `a11y` row pins every a11y-leg test file in order; the pin moves with the leg in the same change, the file order stated (per test-plan §4 Unit Test Strategy → CI workflows and leg script; detail `.andromeda/test-plan-detail/unit-test-strategy.md`, procedure · unit lines 27–30).
- The stand checks hold a zero-failure budget: `stand-repeat` runs the `stand_*.rs` files, ten named non-stand targets and seven_guis' process checks three rounds with no retry, no `--ignored`, no env var — so any row added to `stand_keyboard` or `stand_contrast` must hold three green rounds (per test-plan §10 Quality Gates & Coverage Targets → Flakiness budget; per test-plan §9 CI Integration → Legs).
- Headless-stand checks assert unconditionally under the bundled DejaVu Sans with system fonts off, no skip path — a re-read or new keyboard / contrast row takes no font skip (per test-plan §2 Test Strategy → Font-dependent tests; per test-plan §8 Mocking & Stubbing Discipline → Real dependencies kept).
- The workspace build and the `-p blitz-tests` build compile the same test file against different engine features (`tracing`, `writing-mode` workspace-only), so a green per-package a11y run is not the workspace `test` leg's green — both are read (per test-plan §9 CI Integration → Engine features by runner).
- A blitz-tests check is its own auto-discovered target, one file per behaviour, no `[[test]]` line added (the manifest holds `all` alone, pinned by `BlitzTestsTargetsTest`); files outside the `stand_` prefix (`accessibility_*`, `inert_subtree`, `select_interaction`) are reached by the agent harness through `run {name}` and `run all` only (per test-plan §4 Unit Test Strategy → blitz-tests targets; per test-plan §3 Test Harness Contract → Agent-run contract, `run {selection}`).

## Patterns to follow
- `tests/blitz-tests/tests/common/mod.rs` is the one statement of the stand's shared tables — `tab_order(task)`, `ENABLED_TAB_ORDER`, `controls(task)` — stated expectation, never a second role or name mapping; a moved Tab sequence is re-recorded there, read by `stand_keyboard` (per test-plan §3 Test Harness Contract → Crate-local test helpers, blitz-tests (stand checks)).
- `stand_keyboard` and `stand_contrast` run each test in both layout modes, `stand_keyboard` agreeing the engine's focus, the snapshot's `focused` and the tree's focus at every step, `stand_contrast` reading `TEXT_ROWS` / `STATE_ROWS` through `Harness::text_contrast` with one shared reader whose exempt class stays exactly `enabled == Some(false)` (per test-plan §1 Test Scope Summary → tests/blitz-tests; detail `.andromeda/test-plan-detail/test-scope-summary.md`, procedure · scope-entities lines 91–126).
- Keyboard input goes through the harness's real event pipeline (`press`, `press_with`, the re-exported `Key` / `NamedKey`), each helper pumping after dispatch — a select-key row on the flight booker uses the same helpers (per test-plan §3 Test Harness Contract → blitz-test-harness, Input helpers; per test-plan §5 Integration Test Strategy → Harness ↔ event pipeline).
- Upstream-merged a11y checks (`accessibility_label`, `accessibility_description`) assert through test-that's `verify_that!` / `TestResult<()>` under the standard `#[test]` harness; adding them to a leg adds no framework (per test-plan §3 Test Harness Contract → Rust tests; → Third-party test frameworks).
- A shared fixture page more than one check reads lives in `tests/blitz-tests/tests/fixtures/mod.rs`; a page with one reader stays in its check's file (per test-plan §7 Test Data & Fixtures → tests/blitz-tests/tests/fixtures).

## Anti-patterns to avoid
- Silently re-baselining a stand table (`tab_order`, `ENABLED_TAB_ORDER`, `TEXT_ROWS`, `STATE_ROWS`) to whatever the code reads; the tables are stated expectation (per test-plan §3 Test Harness Contract → Crate-local test helpers, blitz-tests (stand checks)).
- Retrying, `--ignored` or an env-var switch to get a widened leg or the stand repeat green (per test-plan §9 CI Integration → Legs; per test-plan §10 Quality Gates & Coverage Targets → Flakiness budget).
- Reading a per-package green as the workspace green, or an unfinished CI run as green (per test-plan §9 CI Integration → Engine features by runner).

## Contract bindings
- tests §9 `a11y` leg ↔ a11y-plan §9 (the chunk's entry names it as the home of new a11y checks): the leg's `--test` list in `.github/scripts/ci-leg.sh` and its pin in `.github/scripts/test_ci_workflows.py` are tests' own surface; which files witness an a11y requirement is a11y's to say (per test-plan §9 CI Integration → Legs; per test-plan §4 Unit Test Strategy → CI workflows and leg script).
- tests §3 agent-run contract ↔ obs-plan §3: `run stand` / `run {name}` events are harness metadata, not an escher-telemetry sink; the chunk adds no event field (per test-plan §3 Test Harness Contract → Agent-run contract, Events).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh a11y` exits 0 over the widened file list, and `python3 -m unittest discover -s .github/scripts` passes with the `a11y` row naming the same files in the same order (per test-plan §9 CI Integration → Legs; per test-plan §4 Unit Test Strategy → CI workflows and leg script).
- `bash .github/scripts/ci-leg.sh stand-repeat` ends `3 of 3` green with every re-read or added `stand_keyboard` / `stand_contrast` row in it, both layout modes (per test-plan §10 Quality Gates & Coverage Targets → Flakiness budget).
- `bash scripts/agent-run.sh run stand` reads `run.end` failed 0, and each a11y file added to the leg without the `stand_` prefix reads failed 0 under `run {name}` (per test-plan §3 Test Harness Contract → Agent-run contract, `run {selection}`).
- Both the workspace `test` leg and the per-package `cargo test -p blitz-tests --locked` read the re-read checks green (per test-plan §9 CI Integration → Engine features by runner).
