# Task — test-plan · 2026-10-25-cold-agent-bar

## The session
The headless stand now serves six apps: the four lean tasks plus Cells (`stand::boot_cells`) and a new load page (`stand::boot_load`, with an in-process network stand-in `LoadNet` that answers after 200 ms). `escher-session` serves all six slugs. Neither new app is a `LeanTask`.
The flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) were retired and replaced by one select, `flight-type` (ComboBox "Flight type", two options). The stand now states no `aria-pressed` anywhere.
The stand checks were re-pinned to the new control set: controls 15 → 14, the keyboard activation table 20 → 16 rows, contrast rows 18 → 17, act-ids 70·7 → 68·9. The lint at boot now reads `[0, 1, 0, 0]`: the flight booker's two options overlap, because the driver's lint reads both at the select's client rect (a defect left unresolved).
Two new checks were added: `stand_cells_served` (4 tests) and `stand_load_page` (2 tests). `host_log` gained a fourth test, which measures a paged part read through the host's own sink (0 id and 0 name needles, both builds). That measurement discharges the residual line that owned it.
The cold-agent pipe was extended. `STAND_PROMPT` names six apps. The verdict gains seven fields and three reasons, and `wrong_calls` now decides the outcome against the founder's 5 % bar. The live server child runs `mcp --record` into an owner-only `target/cold-agent/record/`. `tasks_met` and the replay decode paged answers. `test_cold_agent.py` grows 33 → 37 cases.
The founder's one live run of 2026-10-25 passed: 6 tasks, 66 calls, 0 wrong, $0.8244, `claude` 2.1.308. Gate counts moved: `run stand` 50 files · 205 passed · 7 ignored, `ci-scripts` 108, the a11y leg 85, the workspace 208 lines · 1117 passed.

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

## Touches
- §3 Cold-agent pipe, Invocation: "the four lean apps in one client session" is now six apps (counter, flight booker, timer, CRUD, cells, the load page). The `cold-agent.sh:N` citations moved, because the file grew by 127 lines in 25 ranges (Invocation, Exit grammar, run stand, the server and judge, Events and State bullets, and their detail sites).
- §3 Cold-agent pipe, `run stand`: `STAND_PROMPT` now asks for six apps and their goals. These are the flight booker's return booking chosen on its flight type control plus one screenshot; Cells `A0` 5, `B1` `=A0*2` reading 10, `Z25` `=B1+1` reading 11 plus one visual lint; the load page's resource loaded and its message shown. The prompt still names no stand id. The live server child now runs `mcp --record` into `target/cold-agent/record` (made `mkdir -m 700`), and replays run without `--record`.
- §3 Cold-agent pipe, the server and judge: `span_counts` also returns the accepted `tool_call` lines for the `COUNTED` verbs (`lint`, `screenshot`, `advance`). The replay's expect match also reads a paged answer's decoded text (`screen_text`). There are new functions `paged`, `screen_text`, `part_reads`, `started_in_order` and `recording`.
- §3 Cold-agent pipe, Verdict: the new fields are `wrong_call_bar_pct` (5), `lint_calls`, `screenshot_calls`, `advance_calls`, `part_reads`, `recorded_sessions` and `recording_kept`. The new reasons are `capability-unused`, `recording-not-kept` and `wrong-calls-over-bar`, ordered after `task-unmet` and before the check's reasons. `wrong_calls` now decides the outcome (`wrong_calls × 100 > 5 × tool_calls` fails), so the line "recorded, never deciding the outcome" is stale. `tasks_met` needs all six apps, reads accepted snapshot answers only, resets an app on its accepted `start`, and joins the latest answer of each part of a paged screen.
- §3 Cold-agent pipe, State: `target/cold-agent/record/` is added (owner-only), and `cleanup` removes it with the rest of the state.
- §3 Cold-agent pipe, Proof: `test_cold_agent.py` 33 → 37. The 2026-10-25 live run is new: passed, isolated (twelve `mcp__escher__*` tools), `counts_agree` 66 · 66, `tasks_met` all six, `lint_calls` 1 · `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2, `recorded_sessions` 6 with `recording_kept`, `check_passed` and `check_can_fail` (59 steps, 16 expects), 0 wrong of 66, $0.8244, `claude` 2.1.308. Calls by verb: advance 1 · click 14 · lint 1 · press 13 · screenshot 1 · scroll 1 · snapshot 13 · start 6 · stop 6 · type 10. The clause "the live run is not re-fired" is stale. The 2026-10-15 run (4 tasks, 43 calls, $0.32) becomes the earlier reading.
- §4 Cold-agent pipe unit tests: the four-app case was renamed `test_a_six_app_run_passes_with_no_wrong_call`. Four cases were added: `test_five_wrong_calls_of_a_hundred_pass_and_six_fail_the_bar`, `test_a_paged_answer_is_read_as_its_parts_joined`, `test_each_unused_capability_fails` (four sub-cases) and `test_a_start_that_left_no_kept_check_fails` (two sub-cases). The wrong-call span case now fails `wrong-calls-over-bar` (3 wrong of 32). The fake server answers Cells as a paged JSON line of two parts and answers the load page, and under `--record` it keeps one check per accepted start. Red-before-green: 34 of 37 fail against the old pipe. Sites are in unit-test-strategy.md:43 · :44 (the `test_cold_agent.py:67-128` citation moved).
- The `ci-scripts` leg is 104 → 108 tests (test-plan.md:187 · :217, test-scope-summary.md:8, unit-test-strategy.md:29 · :43). unit-test-strategy.md:30's "27 → 33" becomes a dated reading.
- §3 Agent-run Proof re-count chain and the test-data bootstrap line: `run stand` is 48 → 50 files, passed 197 → 205, ignored 7. The two new `stand_*` files are picked up by the prefix with no script change. `run all` was not re-counted. Sites: test-plan.md:170-176 (dated), :189's "197 · 0 · 7 over 48 files", ci-integration.md:8, quality-gates-coverage-targets.md:22 ("48 files").
- §3 Stand construction (test-plan.md:130): beside `boot` and `boot_timer`, the stand now boots Cells through `boot_cells(HarnessOptions)` and the load page through `boot_load(HarnessOptions)`. The `stand.rs:65-118` citation moved because lines were added at 1-9, 12 and 17.
- §7 `stand::options` (test-plan.md:336): it still gives `net_provider: None` to every caller but `boot_load`, which replaces it with `LoadNet` through `..options`. The `stand.rs:65-76` and `stand.rs:59-61` citations moved; the latter is also cited in §8 Real dependencies and §9 Fonts.
- §3 `tests/common/mod.rs` tables (test-plan.md:146, test-scope-summary.md:93 · :104): `controls(task)` goes from 18 rows naming 15 controls to 17 rows naming 14. `flight-type` (`Role::ComboBox`) replaces the two mode buttons in `controls`, `tab_order`, `ENABLED_TAB_ORDER` and `rendered`. The flight booker's enabled order is reached by choosing return: `back-btn` → `flight-type` → `flight-start` → `flight-return-date` → `flight-book`, five stops where the stated version had six, after `flight-return`. The select's two options are not in the tables.
- §3 `session_common` (test-plan.md:148-150): `change` now moves the flight booker by `type flight-type return` through the driver rather than by a click. `stand_cells_served` imports from `session_common` (its use block) and `stand_load_page` uses the snapshot builder, so re-read the reader count (29).
- The flight booker's mode, wherever test-plan names it (test-scope-summary.md:108 · :116 · :117 · :118, integration-test-strategy.md:17): `flight-one-way` and `flight-return` are gone, and the select `flight-type` with options `flight-type-one-way` and `flight-type-return` stands in their place.
- `stand_keyboard` (test-scope-summary.md:117): the activation table is 20 → 16 rows and `BUTTONS` 10 → 8. Tests are 14 → 15, with `the_flight_type_select_moves_its_choice_by_its_arrows` added.
- `stand_contrast` (test-scope-summary.md:116): control rows 18 → 17 (tally 13 · 0 · 3 · 1) and `STATE_ROWS` 19 → 18 (tally 17 · 0 · 1 · 0). The `flight-one-way` accent row and the `flight-return` row give way to `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400). The return-chosen rows are `flight-type` 17.40 and `flight-return-date` 17.40.
- `stand_act_ids` (test-scope-summary.md:113): of 77 snapshot nodes, a click is accepted on 68 and refused on 9 (was 70 · 7; the two options are refused `covered`). 14 controls, all 14 changing at boot. The `stand_act_ids.rs:1-15` citation is now 1-16, and `:60` moved.
- `stand_lint` at boot (test-scope-summary.md:123): the "0 findings per lean task" claim is stale. It reads `[0, 1, 0, 0]`, the flight booker's one `overlap` (`flight-type-one-way` × `flight-type-return`), in both layout modes. The test was renamed `the_stand_at_boot_reads_its_measured_findings`.
- `aria-pressed` on the stand (test-scope-summary.md:94): the stand states no `aria-pressed`, and the select states its choice by `selected`. Renamed tests: `stand_act_states::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` and `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`. A new test, `stand_snapshot::the_flight_type_select_holds_its_two_options`, pins the options.
- `stand_requirement_sweep` (test-scope-summary.md:119): `WITNESSES_020` gains three rows, v020-07 (`stand_load_page`), v020-08 (`stand_act_states`) and v020-09 (`stand_cells_served`), plus a `Subject::Served`. The v010-17 and v020-05 witnesses were re-pointed to the renamed tests, and the `source` table carries the two new files. Its own test count is unchanged at 3.
- §5 Session host ↔ what it writes (test-plan.md:267, integration-test-strategy.md:21, test-scope-summary.md:15): "a paged part read through the host's own sink is unmeasured — the host boots only the four lean tasks and none pages" is now false. `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` runs `serve cells` at `RUST_LOG=trace` and commits a sentinel into a part-2 cell, then reads `snapshot --part 2` with 0 id and 0 name needles under both builds. The residual line that owned the read is discharged. `host_log` has 3 → 4 tests.
- `host_log.rs:N` citations moved (the body shifted by the lines added at 134-278): test-plan.md:155's `host_log.rs:398-411`, :156's `host_log.rs:30-492`, test-scope-summary.md:19 and integration-test-strategy.md:23.
- `escher-session` serves six tasks, so every test-plan statement of what the host or `start` boots now names six slugs (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`). `cli_commands` starts the six slugs through the binary.
- §8 Network (test-plan.md:352, mocking-stubbing-discipline.md:14 · :15, the stand-in table) and the stand-offline claims (test-plan.md:336): the stand is offline except for the load page's in-process stand-in `LoadNet`. That is now a third hand-written network stand-in, beside `HeldNet` and `stand_load_wait`'s `LateNet`.
- §5 Wall time in the session checks (test-plan.md:261): re-read the list of exceptions. `LoadNet` answers from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), and `stand_load_page`'s click waits that load out. `stand_cells_served` moves time only by `advance 5000`.
- §9 tallies (ci-integration.md:8 · :42 · :63 · :65): the a11y leg is 14 files, 84 → 85 tests. The workspace tally goes from 206 lines · 1105 passed to 208 · 1117, 0 failed, 12 ignored. The earlier readings there become dated.
- §10 coverage readings (quality-gates-coverage-targets.md:16): dioxus-native-dom 2741/3202 = 85.60 % (was 85.57 %), blitz-test-harness 952/1008 = 94.44 % (was 94.15 %), escher-driver 97.99 % unchanged. The floors are unchanged.
- §9 / §10 `stand-repeat`: the leg takes the two new `stand_*` files by prefix (3 of 3 rounds green), with no CI step changed. CI#40214713710 is green, 17/17.

## New facts
- F1 · test-scope-summary · `seven_guis::stand::boot_cells(HarnessOptions)` mounts `Task::Cells` in `TaskShell`. `boot_load(HarnessOptions)` mounts a load page titled "Load" with two keyed ids: `load-fetch` (a Button, "Load") and `load-done` (a Paragraph, "Loaded", `display: none` until its sheet lands). Neither app is a `LeanTask`, so neither has a keyboard or contrast table row.
- F2 · mocking-stubbing-discipline · `LoadNet`, private to `stand.rs`, is the load page's network stand-in. Each `fetch` answers the fixed sheet `LOADED_CSS` for `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms). It does not override `is_noop`, so the harness counts its load. It uses no `blitz_net`, print, env read, socket or listener.
- F3 · test-scope-summary · Three new stand unit tests in seven_guis: `the_stand_options_stay_offline` (every boot but `boot_load` keeps `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight`. `stand::` now holds 5 tests.
- F4 · test-scope-summary · `stand_cells_served` has 4 tests, each in both layout modes. They walk Cells in parts through the driver, read committed cells and a cell past the viewport in the last part, pin that a click right after the scroll lands on the shown scrollbar, and read the lint at boot as measured.
- F5 · test-scope-summary · `stand_load_page` has 2 tests, each in both layout modes. In the first, the click waits out the load and the message shows. The second checks that the page is keyed and reads no lint finding at boot.
- F6 · test-data-fixtures · The stand's own Cells screen is now an over-budget screen beside the `many_items` fixture: 47,949 bytes and 740 lines, read in 5 parts of `SNAPSHOT_TEXT_BUDGET`.
- F7 · integration-test-strategy · A main-button click on a cell inside a scrolled box right after `scroll` lands on the shown overlay scrollbar. The scrollbar takes it as a thumb drag and dispatches nothing. A pointer left on the thumb keeps the scrollbar shown, so an `advance` past the fade (5000 ms) is needed before the click.
- F8 · test-scope-summary · The driver's lint reads a select's options as overlapping, because the snapshot gives each option its select's client rect. The defect sits in `packages/escher-driver/src/lint.rs` (`overlaps`) and is unresolved, its owner to be pinned at P5. On the founder's word, `stand_lint` is re-pinned to the measured finding.
- F9 · test-scope-summary · seven_guis' shared test module gains `common::cells_needles(part)`. It returns the author keys of that part of the in-process Cells screen and the middle column-A cell the part holds.
- F10 · test-strategy · Measured: a select choice moves no class or text that `Harness::dom_string` shows, so `stand_id_persistence` chooses the return flight and books it in one re-render.
- F11 · unit-test-strategy · Measured: a paged snapshot over MCP answers as a JSON line with escaped quotes, so a substring match over the raw answer misses the screen's text. `test_a_paged_answer_is_read_as_its_parts_joined` asserts the escaped form in the transcript.
- F12 · integration-test-strategy · Measured by a probe through the built binary: a full recorded Cells session keeps a check of 312,431 bytes and the load page's a check of 1,660 bytes, both under the 1,048,576-byte bound.

## Untouched
- The agent-run contract itself: `scripts/agent-run.sh`'s verbs, exit grammar, events, state and contract tests (`test_agent_run.py`) are unchanged. Only its Proof re-count moves.
- `tests/blitz-tests/tests/fixtures/mod.rs`, its pages and `HeldNet`'s own description: the two new files declare no `mod fixtures;`, so the 22 `stand_*` readers and 24 declarers stand.
- `stand_diff`: unchanged. Book still moves eight nodes, seven by bounds alone.
- The committed Counter checks under `tests/blitz-tests/checks/`, `stand_replay` and the recorded-check replay strategy: the Counter is untouched.
- Every engine and driver crate under `packages/` and their unit tests: the diff over `packages`, `Cargo.toml` and `Cargo.lock` is 0. No driver verb, argument, cause, session error, span field or log line moved.
- The `Harness` API (construction, pump, settle, screenshot, contrast, input and inspection helpers) and `HarnessOptions`' fields: `boot_load` only fills the existing `net_provider`.
- The stand log format, its two record classes and the `command` / `tool_call` span fields: no log line or field was added.
- `accessibility_states` and the dioxus-native-dom boolean-model limits: the engine's `aria-pressed` read is unchanged and still pinned on a fixture.
- The coverage floor values (97 % / 83 % / 90 %), `coverage_floor.py`, the mutation tier and the CI workflow's jobs, cache, fast/slow split and package installs.
- WPT, the browser app, blitz-vibey-script, blitz-html, blitz-net and blitz-paint test inventories.
