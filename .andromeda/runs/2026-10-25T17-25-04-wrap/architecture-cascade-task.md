# Task — architecture · 2026-10-25-cold-agent-bar

## The session
The stand was extended past the four lean tasks: `seven_guis::stand` gained `boot_cells` (Cells in TaskShell, still no `LeanTask`) and `boot_load` (a load page whose one stylesheet fetch is answered by an in-process stand-in, `LoadNet`, after 200 ms), and `escher-session` now serves six tasks — counter, flight-booker, timer, crud, cells, load.
The flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) and their CSS were retired for one `select#flight-type` (`ComboBox` "Flight type", options `flight-type-one-way` · `flight-type-return`), so the stand states no `aria-pressed` anywhere; the stand checks were re-pinned to the new control set (controls 15 → 14, contrast and keyboard tables, act-ids census, lint at boot).
The cold-agent pipe (`scripts/cold-agent.sh`) now names six apps, records the live child's session under `--record`, decodes paged answers, and decides on the founder's 5 % wrong-call bar with seven new verdict fields and three new reasons; its contract tests grew 33 → 37.
Two new stand checks landed (`stand_cells_served`, `stand_load_page`), `host_log` measured a paged Cells part read through the host's own sink (0 id and 0 name needles under both builds), and the founder's one live run passed: 6 tasks, 66 calls, 0 wrong, $0.8244, Claude Code 2.1.308.
Measured along the way: the driver's lint reads a select's two options as overlapping (one `overlap` on the flight booker at boot, re-pinned as measured, the driver defect left open), a served Cells screen is 47,949 bytes in 5 parts, and a click right after a `scroll` lands on the shown overlay scrollbar. No engine crate, driver schema, dependency or CI step changed.

## Reached files
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_load_page.rs
- examples/seven_guis/src/tasks/flight_booker.rs
- examples/seven_guis/src/stand.rs
- examples/seven_guis/src/session_host.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/common/mod.rs
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/session_common/mod.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_act_states.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- tests/blitz-tests/tests/stand_act_disabled.rs
- tests/blitz-tests/tests/stand_act_ids.rs
- tests/blitz-tests/tests/stand_act_diff.rs
- tests/blitz-tests/tests/stand_element_ids.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_snapshot.rs
- tests/blitz-tests/tests/stand_session_recording.rs
- tests/blitz-tests/tests/stand_requirement_sweep.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_accessibility_ids.rs
- scripts/cold-agent.sh
- .github/scripts/test_cold_agent.py
- escher-0.2.0/verification-matrix.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/scope-record.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/operator-pass.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-run.md
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-check.json
- escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-transcript.jsonl

## Touches
- Headless stand contract (architecture body §Standard Contracts → Headless stand, and its detail standard-contracts.md:120): the stand no longer boots only the four lean tasks offline — it also boots Cells and the load page, and every boot is offline but `boot_load`, whose options carry the in-process `LoadNet` stand-in; still no `blitz_net`, no telemetry, no env read.
- CLIs contract (architecture body §Standard Contracts → CLIs, standard-contracts.md:233): `escher-session` is the driver's command line over six served tasks, not the four lean ones; its usage line's apps and the MCP `start` tool's enum follow the table.
- standard-contracts.md:218 and :234, existing-scopes.md:118: the served-task count (four → six) and `session_host.rs` line citations moved by the new `Served` table.
- The flight booker's mode control in existing-scopes.md:106 · :107 · :112: the two pressed mode buttons are gone, replaced by `select#flight-type`; the Tab order is `back-btn` → `flight-type` → `flight-start` → `flight-book`, plus `flight-return-date` when enabled (five stops).
- standard-contracts.md:211: any claim that the stand's flight booker states which control is pressed by `aria-pressed` is now false — the select states its choice by `selected`; the engine's `aria-pressed` read itself is unchanged and still pinned on a fixture.
- standard-contracts.md:163 (and the `stand_act_ids` citations at existing-scopes.md:110, standard-contracts.md:151): the act-ids census moved — of 77 snapshot nodes a click is accepted on 68 and refused on 9 (was 70 · 7), 14 controls (was 15); the file's doc now spans 1-16.
- The stand's control count 15 → 14 (`controls(task)` rows 18 → 17) wherever architecture counts the stand's controls.
- The stand's contrast tables wherever cited (existing-scopes.md:112): control rows 18 → 17, `STATE_ROWS` 19 → 18, the `flight-type` pair `#1a1a1a` on `#ffffff` at 17.40 replacing the `flight-one-way` accent row and the `flight-return` row.
- The `stand_keyboard` activation table 20 → 16 rows (`BUTTONS` 10 → 8), its tests 14 → 15 (existing-scopes.md:106).
- The lint at boot: a claim that every lean task reads 0 findings is false — the flight booker reads one `overlap` (`flight-type-one-way` × `flight-type-return`), the other three 0; `stand_lint` re-pinned as measured on the founder's word.
- `stand_requirement_sweep`'s witness table (existing-scopes.md:114): `WITNESSES_020` gains three rows (v020-07 load page, v020-08 flight type, v020-09 Cells walk) and a `Served` subject; v010-17 and v020-05 witnesses re-pointed to renamed tests.
- The blitz-tests target inventory: two new `stand_*` files (one test target per file), `agent-run.sh run stand` 48 → 50 files, passed 197 → 205; the a11y leg 84 → 85 tests over 14 files; the workspace tally 1105 → 1117 passed over 208 lines.
- standard-contracts.md:261 (CI contracts, the pipe): the `ci-scripts` leg 104 → 108 tests, `test_cold_agent.py` 33 → 37 cases; the pipe's verdict now decides on `wrong_calls` by the 5 % bar.
- The cold-agent pipe's contract wherever architecture states it (CI contracts; occupied-resources.md:39 · :40): six apps and their goals, the live server child run with `mcp --record` into an owner-only record directory, replays without `--record`, seven verdict fields and three reasons added.
- `scripts/cold-agent.sh` line citations moved (the file grew by 127 lines in 25 ranges): occupied-resources.md:24, stack-and-technologies.md:73.
- Outbound hosts and the live run (occupied-resources.md:17 · :40): the founder's one live run of 2026-10-25 carried the stand's new screens — Cells' contents, the select's options and value, a lint's findings, a shot — to the model provider on his word, within his one-run budget; 6 tasks, 66 calls, 0 wrong, $0.8244, `claude` 2.1.308, beside 0.1.0's 4 tasks and 43 calls.
- `host_log` 3 → 4 tests and its line citations moved (occupied-resources.md:44): a paged part read through the host's own sink is now measured — `serve cells` at `RUST_LOG=trace`, `snapshot --part 2`, 0 id and 0 name needles under both builds; any claim that the host boots only the four lean tasks and none pages is false.
- `session_host.rs` line citations moved: cross-cutting-patterns.md:14, occupied-resources.md:38 · :44.
- The hand-written network stand-ins: beside `HeldNet` (fixtures) and `stand_load_wait`'s `LateNet`, the stand's `LoadNet` is a third, living in the stand module rather than under tests.
- Coverage readings where architecture cites them: dioxus-native-dom 85.60 %, blitz-test-harness 94.44 %, escher-driver 97.99 % unchanged.
- The snapshot budget reading: every lean stand screen stays under `SNAPSHOT_TEXT_BUDGET`, but a served screen does not — a claim of every stand screen under the budget must now say the lean four.
- Retired: the flight booker's `flight-one-way` · `flight-return` ids and their CSS classes `.flight-type-row` · `.type-btn` · `.type-btn-active`, the stand's only `aria-pressed` controls; any architecture text naming them is stale.
- Renamed tests: `stand_act_states`' flight test, `stand_snapshot_state`'s pressed test and `stand_lint`'s boot test carry new names; any citation of the old names is stale.

## New facts
- F1 · standard-contracts · `seven_guis::stand::boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts the Cells task in TaskShell through `cells_root`; Cells stays no `LeanTask`.
- F2 · standard-contracts · `seven_guis::stand::boot_load(HarnessOptions)` mounts a load page titled "Load" (ids `load-fetch`, a Button "Load", and `load-done`, a Paragraph "Loaded" hidden until its sheet lands), replacing only `net_provider` with `LoadNet`; `stand::options` keeps `net_provider: None` for every other caller.
- F3 · standard-contracts · `LoadNet` answers every fetch with the fixed `LOADED_CSS` for the fixed `http://stand.test/load.css` after `pub const LOAD_ANSWER_DELAY` (200 ms), with no `is_noop` override so the harness counts its load, and no `blitz_net`, print, env read, socket or listener.
- F4 · occupied-resources · the stand's `LoadNet` answers each fetch from a thread of its own, a thread a load-page boot spawns per fetch.
- F5 · standard-contracts · `escher-session` holds a private `Served` table of six slugs (`Lean`, `Cells`, `Load`) from which its usage line's apps and the MCP `start` tool's enum are written as program-fixed text; the Timer still carries `with_time(timer_step)`.
- F6 · occupied-resources · the cold-agent state area holds `target/cold-agent/record/`, made owner-only, where the live server child keeps its kept checks under `--record`; `cleanup` removes it with the state, and no kept check from it is committed.
- F7 · standard-contracts · the driver's lint reads a select's options as overlapping, because the snapshot gives each option its select's client rect; the defect is unresolved and its fix is a driver change for every document in the lint's overlap reading.
- F8 · established-decisions · right after a `scroll`, a main-button pointerdown at a point in the shown overlay scrollbar's band is taken as a thumb drag and dispatches nothing; on the virtual clock the bar stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown.
- F9 · existing-scopes · a served Cells screen reads 47,949 bytes, 740 lines, in 5 parts of `SNAPSHOT_TEXT_BUDGET`, walked by part through the driver in `stand_cells_served`.
- F10 · standard-contracts · a select choice moves no class or text that `Harness::dom_string` shows.
- F11 · standard-contracts · a paged snapshot answer crosses MCP as a JSON line whose quotes are escaped, so a substring match over the raw answer misses the screen's own text; the cold-agent pipe decodes paged answers before matching.
- F12 · standard-contracts · a full recorded Cells session keeps a check of 312,431 bytes and the load page's 1,660, both under the 1,048,576-byte recording bound.
- F13 · existing-scopes · the seven_guis process tests share `common::cells_needles(part)`, returning a part's author keys of the in-process Cells screen and the middle column-A cell it holds.

## Untouched
- The driver's command and refusal schema: twelve verbs, eight causes, fifteen session errors, the argument kinds and the self-description are unchanged — no verb, argument, cause, session error or result field moved.
- The `v3` wire, the Unix-domain socket listener, its state directory and idle expiry are unchanged; no new listener, port or carrier was added.
- The `command` and `tool_call` spans and their fields, and the telemetry sink's target allowlist and scrub sets, are unchanged; no log line or span field was added.
- The session recording format, `Check::from_bytes`, `Check::replay` and the two committed Counter checks are unchanged.
- No engine crate under `packages/` changed: the select model, range input, keyboard activation, the document's clock, hit testing, scrolling and painting all stand as before.
- The accessibility builder's `aria-pressed` · `aria-checked` · `aria-selected` · `aria-invalid` reads are unchanged; only the stand stopped stating `aria-pressed`.
- Dependency pinning, the root manifest, the lockfile and the cargo-deny configuration are unchanged; no crate was added or removed.
- CI workflows and the leg runner's legs are unchanged; the stand-repeat leg takes the two new stand checks by their prefix with no step change.
- The browser app, rdme, the WPT runner and the Dioxus bridge's element-id tiers and snapshot model are unchanged.
- Project intent's upstream sync and the inherited defaults are unchanged.
