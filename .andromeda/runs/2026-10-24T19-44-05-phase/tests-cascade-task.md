# Task — tests · 2026-10-24-a11y-re-read

## The chunk
A re-read of the stand's accessibility checks after 0.2.0's engine changes (inert, disabled read on a form control alone, aria-checked on checkable roles, the select's keyboard and typeahead model, the upstream aria reads, the cherry-picked upstream engine commits), changing no engine behaviour.
It re-asserts `stand_keyboard`'s boot and enabled-state Tab sequences and its Enter and Space activation rows in both layout modes, re-recording any sequence 0.2.0 moved with the change named, and adds a row for a stand control 0.2.0 made keyboard-operable, the flight booker's `select` being the candidate (arrows, Home, End, typeahead).
It re-asserts `stand_contrast`'s control rows, CRUD list rows, text rows and colour-state rows (`TEXT_ROWS`, `STATE_ROWS`) so SC 1.4.3 holds on every non-exempt stand row.
It widens the `a11y` CI leg from its six test targets to the 0.2.0 a11y witnesses (candidates `accessibility_states`, `inert_subtree`, `accessibility_active_descendant_unread`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `select_interaction`, `stand_act_select`), moving the leg's pin in `test_ci_workflows.py` with it; the a11y job keeps its tier and takes no required status check.

## Reached files
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/common/mod.rs
- .github/scripts/ci-leg.sh · merge surface
- .github/scripts/test_ci_workflows.py · merge surface

## Needs
- The `a11y` leg of `ci-leg.sh`: a `cargo test --workspace --locked` run of named `--test` targets, its target list widened, run by ci.yml's Accessibility (a11y) tests job through the leg script, host-reproducible like every linux leg.
- The CI workflow and leg-script invariants pinned by `test_ci_workflows.py`: each leg through `ci-leg.sh`, the `LegScriptTest` tuple of each leg's arguments (the a11y tuple moves with the leg), the a11y job's name, its failure-log upload, and its place among the jobs that `needs` the fast four.
- The CI cache rule: the a11y job restores the test job's `workspace-test` cache with `save-if: false` (a restore-only job), unchanged by a widened target list.
- The failure-log rule: the a11y leg's output in `target/ci-logs/a11y.log`, uploaded only on failure.
- Engine features by runner: the a11y leg builds under the workspace (with `tracing` and `writing-mode` on), so a target added to it reads under different engine features than its `-p blitz-tests` run; a green per-package run is not the leg's green.
- The `stand-repeat` leg and the zero-failure flakiness budget: `stand_keyboard`, `stand_contrast` and every other `stand_*` file, plus the ten named 0.2.0 witnesses (among them `inert_subtree`, `accessibility_states`, `accessibility_active_descendant_unread`, `select_interaction`), run three rounds with no retry, so a row added to a stand check joins that budget.
- The blitz-tests layout: one test target per file under `tests/blitz-tests/tests/`, no `[[test]]` table but the unbuilt `all`, pinned by `BlitzTestsTargetsTest` — a new a11y check would be a new file target, the manifest unchanged.
- The stand checks' shared module `tests/common/mod.rs`: the tables `tab_order(task)`, `ENABLED_TAB_ORDER`, `controls(task)`, `rendered(task)`, `REGION_NAMES` and its helpers (`boot`, `node`, `by_id`, `focused`), stated expectation and never a second role or name mapping, with its stated list and count of reader checks.
- The headless stand boot: `seven_guis::stand::boot` and `stand::options(incremental)` (800×600, scale 1.0, Light, bundled DejaVu Sans, no network), the lean tasks booted fresh per check.
- Both layout modes: every stand pipeline check runs for incremental false and true, incremental and non-incremental layout identical.
- Font-dependent stand checks: no skip path, asserting under the bundled font with system fonts off, so the keyboard and contrast rows measure the same on any runner.
- The harness's input and inspection helpers the rows drive through: `press`, `press_with`, `click`, `type_text`, `focused`, `query`, `attr`, `text_content`, routed through the real event-dispatch pipeline and each pumping after dispatch; Tab sequences and activation read through them.
- The harness's contrast reader: `Harness::text_contrast` and `contrast_ratio`, read-only from computed styles, `required_ratio` 3.0 for large text else 4.5, `None` past opacity below 1, a filter or a background image — the measure every `stand_contrast` row asserts through, with its stated-table unit tests in `contrast.rs`.
- The harness's time: a `VirtualClock` its documents read, advanced only by `tick` and `advance_time`, so a select typeahead row (its buffer reset after 1 s of the document's clock) moves time through the harness, never through a wall-clock sleep.
- The select's rules as tested: `node/select.rs`'s unit tests over stated tables and the blitz-tests checks `select_interaction` and `stand_act_select`, the select's stated limits asserted by no check.
- The engine's keyboard activation and focusability as tested by `stand_keyboard`: Enter and Space on stand buttons, a button click's focus, `disabled` read on a form control alone, and an `inert` subtree skipped by Tab (`inert_subtree` its pin).
- The accessibility-tree checks the leg may take: `accessibility_states`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, and upstream's merged `accessibility_label` and `accessibility_description`, the last two asserting through test-that's `verify_that!` and `TestResult<()>` under the standard harness.
- The shared fixtures module `tests/fixtures/mod.rs` (`fixtures::boot`, `hold`), declared by `select_interaction` and `stand_act_select`, which the widened leg would build under the workspace.
- The agent-run contract's selections: `run stand` picks up `stand_*` files by prefix, a target with no `stand_` prefix reached only by `run {name}` and `run all`, and the stated `run stand` and `run all` totals that an added test moves and that a re-count records.
- The session checks' rules if `stand_act_select` is taken into the leg: `session_common` builders (`hold`, `act`, `type_into`, `press`), no sleep or wall-clock read, a refused call leaving the instance unchanged.
- The tests summary's coverage statements this chunk may move: the a11y checks' list, the statement that no check reads the accessibility states as the platform adapter hands them to an assistive technology (owed, owned by a residuals line), and the select's stated limits asserted by no check.

## Not needed
- The WPT conformance runner, its reftests, attr and testharness runs, scoring, quarantine list and Thai font, and the WPT diff and area-change scripts' tests.
- The browser app's tests (about pages, history, favicon, URL suggestions, persistence) and the blitz-vibey-script DOM, Selection and Preact TodoMVC tests.
- The cold-agent run pipe, its `claude` shim contract tests, verdict and live run.
- Session recording and replay: the kept check, the committed Counter checks under `tests/blitz-tests/checks/`, `record_checks.sh`, `stand_replay` and seven_guis `recording`.
- The MCP server's framing, handshake revisions, result bounds and the `mcp_server` / `mcp_log` process checks.
- The headless screenshot's pixel tests: the shot's PNG equal to the raster, byte-identity across runs and the per-OS PNG difference.
- The telemetry sink's format, scrub and drop checks (`telemetry_*`, `host_log`) and the stand log's line counts by build and level.
- The mutation tier on the dev host (cargo-mutants, nextest `mutants` profile).
- blitz-dom's style-bridge, list-marker, text-transform, SVG and font-face unit tests, and the incremental oracle's paint-tree comparison and randomized stress.
- The snapshot text, parts and diff checks and the driver's command-line flows, which read the stand's screen but not its keyboard or contrast.
