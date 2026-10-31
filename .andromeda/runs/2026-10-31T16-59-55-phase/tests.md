# tests extract

## Relevance
relevant

## Constraints
- Commands: test-plan §3 Agent-run contract binds the checks to `bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}`. `select_interaction`, with no `stand_` prefix, is reached by `run select_interaction` and `run all`, and `stand_*` files by `run stand`; an empty run is never a pass (per test-plan §3 Agent-run contract).
- test-plan §1 blitz-dom covered behaviours records the select's stated limits as pinned by `select_interaction` and `stand_act_select`: no popup painted, Enter and Space opening nothing on a single select, a closed option refused `covered`, `size` unread, a group's label unpainted (detail: `.andromeda/test-plan-detail/test-scope-summary.md`). Each limit this chunk ends requires its pin restated to the new behaviour, not dropped; which ones end is the founder's forks and P3's reading (per test-plan §1 blitz-dom covered behaviours).
- A check that drives a held session requires both layout modes, and a refused call must leave the instance unchanged and read as a text holding none of the bytes the call supplied. This binds the option acted on through the driver and the restated `covered` (per test-plan §5 Session ↔ held instance).
- The opened list's paint and hit testing require readings under the incremental and the non-incremental document, since the oracle compares layout and paint tree after each step. Whether an open list is reachable by the oracle's fixtures is research's question (per test-plan §2 Differential oracle).
- New select rules in `node/select.rs` require table unit tests beside the existing ones. Driver refusal and schema text is pinned by escher-driver's unit tests, so the restated `covered` text moves those pins with it (per test-plan §4 blitz-dom (node); per test-plan §4 escher-driver).
- Every new select check that is not `stand_*` needs a literal `--test` word to reach `stand-repeat`, whose named list is pinned by `test_stand_repeat_leg_runs_three_rounds`. The stand checks hold a zero-failure budget over three rounds with no retry (per test-plan §9 Legs; per test-plan §10 Flakiness budget).
- A green per-package run is not the workspace leg's green, since `tracing` and `writing-mode` are on in the workspace build only; the local pre-push gate is `bash .github/scripts/ci-leg.sh fast` (per test-plan §9 Engine features by runner; per test-plan §9 Local pre-push gate).

## Patterns to follow
- Drive the list with the harness input helpers, each pumping after dispatch: `click_at`, `press`, `type_text`, `hit`, `hit_node`. `dispatch_recorded` bypasses Dioxus forwarding, so it cannot witness a page's `input` and `change` (per test-plan §3 blitz-test-harness; per test-plan §5 Harness ↔ event pipeline).
- A fixture page with more than one reader lives in `tests/blitz-tests/tests/fixtures/mod.rs`, booted through `fixtures::boot` or `fixtures::hold`; a page with one reader stays in its check's file (per test-plan §3 blitz-tests (fixtures)).
- Session checks read through `tests/session_common/mod.rs`: `act`, `click`, `press`, `refused_unchanged`, `screenshot_of`, `lint`, `linted` (per test-plan §3 blitz-tests (session checks)).
- Stand expectation stays in the tables of `tests/common/mod.rs`, which name `flight-type` as `Role::ComboBox` and hold `tab_order` and `ENABLED_TAB_ORDER`, stated expectation and never a second mapping. Any new row in the stand's flight booker moves these tables (per test-plan §3 blitz-tests (stand checks)).
- Painted output is read through `Harness::screenshot` or `screenshot_rect` and sampled pixels, each resolving at the harness's own time (per test-plan §3 blitz-test-harness; per test-plan §2 Pixel tests).

## Anti-patterns to avoid
- No sleep and no wall-clock read in a session check whose subject is not wall time. The list's open, close and any motion are read on the harness's `VirtualClock` (per test-plan §5 Wall time in the session checks).
- No shot in a check compared byte for byte across OSes, and no `screenshot` step in a committed check, since a shot's PNG bytes differ by OS (per test-plan §2 Pixel tests).
- No skip path for fonts in a stand check: the list's text is asserted under the bundled DejaVu Sans with system fonts off (per test-plan §2 Font-dependent tests).

## Contract bindings
- tests ↔ a11y: `select_interaction`, `stand_act_select`, `stand_keyboard`, `stand_contrast` and `stand_accessibility_ids` are files of the `a11y` leg's fixed list, in the same workflow as the other legs (test-plan §9 Legs), so the list's SC 2.1.1 · 2.4.3 readings land there. A new witness file needs that list changed, which `test_ci_workflows.py` pins (test-plan §4 CI workflows and leg script).
- tests ↔ a11y and design-system: the list's engine colours fall under SC 1.4.3 and are read by `text_contrast`, which reads computed styles only (test-plan §3 blitz-test-harness); §1's chosen-row pair in `default.css` is the measured precedent. Whether `text_contrast` can read a painted overlay row is research's question.
- tests ↔ obs: `run` events are harness metadata, not an escher-telemetry sink (test-plan §3 Agent-run contract); a call run through `Session::run` leaves one `escher_driver` line of fixed words and counts, so an option's name or value in the list stays out of it (test-plan §5 The driver's log line; test-plan §3 Stand log format, binding obs-plan §3 and §6).
- tests ↔ driver surface and docs: `test_capability_statement.py` pins `docs/capabilities.md` to the driver schema's tables and counts (test-plan §4 Capability statement). A restated `covered` keeps the counts only if the chunk adds no verb, cause or node key, as the scope infers; any fork adding one moves the pin.
- tests ↔ arch: no test launches a window (test-plan §6 Observed absent), so the scope's inferred windowed list holds no standing check. Whether the headless reading stands for `dioxus-native`'s path is research's question.

## Acceptance criteria contributions
- `bash scripts/agent-run.sh run select_interaction` and `run stand` each exit 0 with `run.end` outcome `passed`, holding checks that open the list, choose by pointer and by key, and close it with no choice, the last changing no value and firing no `input` or `change` (per test-plan §3 Agent-run contract).
- An option of the opened list is acted on through `Session::run` in both layout modes, its choice read in the next snapshot, and a refused call on an option leaves the snapshot and focus readings equal before and after (per test-plan §5 Session ↔ held instance).
- `bash .github/scripts/ci-leg.sh stand-repeat` reads `stand-repeat: 3 of 3 rounds green` with every new select target inside it, no retry (per test-plan §10 Flakiness budget).
- The `coverage` leg ends `coverage floor: 3 of 3 crates at or above their floors`, `packages/escher-driver` at 97 %, `packages/dioxus-native-dom` at 83 %, `packages/blitz-test-harness` at 90 % (per test-plan §10 Coverage floors).
