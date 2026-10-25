# Task — tests · 2026-10-25-cold-agent-bar

## The chunk
Extends the seven_guis stand so the driver's `start` serves every 0.2.0 check: `cells` (the stand's one screen over `SNAPSHOT_TEXT_BUDGET`, so `snapshot`'s `part` is reached), the flight booker's one-way / return choice as a real `select`, and a load in flight on the stand (how it is carried — beside the offline `stand::options` and the fixtures' `HeldNet` — is a P4 fork).
Gives the cold-agent pipe goals over 0.2.0's capabilities end to end (recording and replay, lint, `advance` on the virtual clock, the load wait, the select, the screen in parts) and moves its contract tests with it.
Fires one live cold-agent run on the operator's login, judged by a wrong-call bar the founder sets at P4 from 0.1.0's baseline (0 wrong in 43), the agent-written check replayed on fresh `escher-session mcp` children and possibly the driver's own `--record` kept check replayed by `Check::replay`.
Re-pins the stand claims that move in the tests that hold them: the Tab order and Enter/Space tables, the contrast rows, element ids and actionable keys, the snapshot, diff and act checks, the flight flow script, every stand screen under the budget, and `stand-repeat`'s targets; adds no driver verb, argument, cause, session error, carrier, log field or span and no engine change.

## Reached files
- examples/seven_guis/src/session_host.rs · edits the task list `start` boots
- examples/seven_guis/src/stand.rs · edits the stand boot and `stand::options`
- examples/seven_guis/src/tasks/flight_booker.rs · edits the one-way / return choice
- examples/seven_guis/src/tasks/cells.rs
- examples/seven_guis/tests/flows/flight.sh
- scripts/cold-agent.sh · edits `TASKS` and the stand prompt's goals
- .github/scripts/test_cold_agent.py
- tests/blitz-tests/tests/common/mod.rs · edits `tab_order`, `ENABLED_TAB_ORDER`
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_actionable_keys.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_snapshot_text.rs
- tests/blitz-tests/tests/stand_diff.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_load_wait.rs
- tests/blitz-tests/tests/stand_snapshot_parts.rs
- tests/blitz-tests/tests/fixtures/mod.rs

## Needs
- The headless stand's boot (`seven_guis::stand::boot`, `boot_timer`, the lean tasks in TaskShell) and `stand::options` — 800×600, bundled font, system fonts off, `net_provider: None` — which a served `cells` and a load in flight on the stand change or stretch
- The session host's `start` and host role booting the stand tasks, and the session checks' shared module (`session_common`: `hold`, `slug`, the call builders, `snapshot_part` and `part`, `act`, `Acted`)
- The shared fixtures module and `HeldNet`, the one held network stand-in whose loads the harness counts (no `is_noop` override), and the harness's load counter over a fetching `net_provider`
- The stand checks' shared tables in `tests/common/mod.rs` (`controls`, `rendered`, `tab_order`, `ENABLED_TAB_ORDER`) as stated expectation the flight booker's select moves
- `stand_keyboard`'s Tab sequences and Enter/Space activation over each stand button, and `stand_contrast`'s per-control and state rows, both moving with a new stand control
- Stable element ids and the actionable-key check (`unkeyed_actionable`) over every element a served `cells` and the new select expose
- The snapshot text budget, every stand screen under it, and the paged `part` read on the stand; the integration residual that a paged `part` read through the host's own sink is unmeasured because the host boots only the four lean tasks
- Session host ↔ what it writes: no id, name or typed text on the host's or the `mcp` role's stderr at `trace`, which a served `cells` and select now carry
- The driver's act checks over the stand (click, type on a select, press, advance, the load wait's settle and `LOAD_WAIT`), in both layout modes
- The wall-time rule for session checks: no sleep or wall-clock read except the checks whose subject is wall time (`stand_load_wait`'s reads), which a stand-borne load must keep
- Command-line checks and the POSIX shell flows (`cli_flow`, `flows/flight.sh`) run as processes of the package's `escher-session` binary, `cfg(unix)`-gated, bounded and drained
- MCP checks of the `mcp` role (`mcp_server`, `mcp_log`, `stand_mcp_tools`), the surface the cold agent drives
- The cold-agent run pipe's contract: invocation, exit grammar, `run stand` (`TASKS`, `STAND_PROMPT`, the client flags, the per-run `mcp.json`), the check's judge (`valid_step`, `judge_check`, `MAX_CHECK_BYTES`, `MAX_CHECK_STEPS`, `REPLY_BOUND`, full and ablated replay), the verdict fields (`tasks_met`, `wrong_calls`, `counts_agree`, `isolated`), events, state and its proof
- `test_cold_agent.py`'s cases under the `claude` shim and fake binary — every verdict path, `tasks_met` over the new goals, no scrub-set key or planted marker in events or verdict — run in the fast `ci-scripts` leg
- The live run as proof on the operator host, never in CI: the 2026-10-15 baseline reading (43 calls, 0 wrong, $0.32) and the new run's measured verdict as committed, host-path-masked evidence
- Recorded-check replay and the committed Counter checks (`Check::replay`, `stand_replay`, seven_guis `recording`, `record_checks.sh`), reached if the acceptance's replay means the driver's own kept check
- The agent-run contract's `run stand` selection and its re-counted `run.end` totals, which new or grown `stand_*` files move
- The `stand-repeat` leg and the flakiness budget: the stand checks, its ten named targets and seven_guis' process checks three rounds, zero failures, no retry
- Process-lifecycle checks: a host started and stopped by its check, every wait bounded, a guard that kills and reaps it
- Both layout modes for every stand check (`for incremental in [false, true]`)
- Font-dependent stand checks asserting unconditionally under the bundled DejaVu Sans, which a served `cells` screen reads under
- The local pre-push gate (`ci-leg.sh fast`) and the doc leg as the chunk's done-line

## Not needed
- WPT conformance, its runner's unit tests, scoring, ports and the upstream-guarded WPT workflows — the chunk touches no CSS conformance path
- apps/browser's history, suggestions, favicon and persistence tests
- blitz-vibey-script's DOM, selection and Preact TodoMVC tests and its virtual time
- blitz-dom's style bridge, list-marker, `text-transform`, SVG and font-feature unit tests, and the range and select rule tables in `node/` — no engine change is intended
- The differential incremental oracle, the randomized stale-node stress and the pure-restyle hover paths
- escher-telemetry's format unit tests and the `telemetry_*` integration files — no log field, span or sink change
- Pixel tests' box-shadow expectations and the shot's cross-OS byte rule — no `screenshot` change
- The per-crate coverage floors and the mutation tier — the chunk changes example code, a script and tests, not escher-driver, dioxus-native-dom or blitz-test-harness
- The CI platform matrix's ios and android builds, MSRV, the package-install script and its bound
- `test_capability_statement.py` and the capability statement's counts — no verb, cause, session error, node key or exit is added, and the statement's text is out of scope
- `BlitzTestsTargetsTest`'s manifest pin — no `[[test]]` table changes
- The example fixtures under `examples/assets` and `examples/preact`
