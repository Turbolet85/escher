proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 8
    section: "## 2. Test Strategy"
    change: "After the `dom_string` clause (packages/blitz-test-harness/src/inspect.rs:117-131), add: a select's choice moves no class or text `dom_string` shows (measured at 2026-10-25-cold-agent-bar), so `stand_id_persistence` chooses the return flight and books it in one re-render"
    sidecar: "test-strategy#1.1: dom_string shows no select choice, so stand_id_persistence chooses and books in one re-render (2026-10-25-cold-agent-bar)"
    rationale: "The report's Decisions & corrections measured that a select choice moves no class or text Harness::dom_string shows, and its Reverted / negative API facts records that the separate re-render step was folded into the Book step; the task's F10."
    fact: F10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 52
    section: "## 2. Test Strategy"
    change: "`tests/session_common/mod.rs`, read through `mod session_common;` by 31 checks (`stand_cells_served.rs` and `stand_load_page.rs` the latest, since 2026-10-25-cold-agent-bar; 29 with `stand_snapshot_parts.rs`; 27 before `stand_act_select.rs`, …), the older reader chain kept as it stands"
    sidecar: "test-strategy#8.1: session_common has 29 → 31 readers, with stand_cells_served and stand_load_page (2026-10-25-cold-agent-bar)"
    rationale: "The report's new files: stand_cells_served.rs opens with `use session_common::{` (stand_cells_served.rs:19-21) and returns `session_common::Acted` from `step` (62-71), and stand_load_page.rs runs the session's `snapshot()` builder (25-32). The task asks for the reader count (29) to be re-read. Whether either file also declares `mod common;` is not in the report, so the 21 is left as it stands."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "…every other `.rs` file directly under `tests/` stays its own auto-discovered target: 147 `.rs` files, 146 test targets and the unbuilt `all.rs` (145 and 144 before 2026-10-25-cold-agent-bar added `stand_cells_served.rs` and `stand_load_page.rs`, each its own auto-discovered target with no manifest line; 144 and 143 before upstream's `hit_test_dropped_nodes.rs`, …), the rest of the chain kept"
    sidecar: "test-strategy#8.1: blitz-tests 145 → 147 .rs files, 144 → 146 targets (stand_cells_served, stand_load_page)"
    rationale: "The report's Changes list two new files under tests/blitz-tests/tests/ and no change to tests/blitz-tests/Cargo.toml or all.rs. `run stand` went from 48 to 50 files (F10), and stand-repeat takes the two new files by prefix with no CI step changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 29
    section: "## 4. Unit Test Strategy"
    change: "the CI-scripts leg runs 108 tests over eight test files — test_ci_workflows.py's 33, test_wpt_diff_to_pr.py's 7, test_agent_run.py's 14, test_cold_agent.py's 37, test_wpt_area_changes.py's 5, test_blitz_tests_targets.py's 1, test_coverage_floor.py's 7 and test_capability_statement.py's 4 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md (`ci-scripts` `Ran 108`) and green at CI#40214713710 on `55bf3425`; 104, with test_cold_agent.py's 33, as measured at escher-0.2.0/chunks/2026-10-25-capability-statement/evidence/counts.md and green at CI#40184674449 on `e7e71221`; the older readings kept"
    sidecar: "unit-test-strategy#3.1: ci-scripts 104 → 108, test_cold_agent.py 33 → 37 (2026-10-25-cold-agent-bar); the 104 reading dated"
    rationale: "The report's F9 (ci-scripts-104) marks unit-test-strategy.md:29 stale and :30 dated, and its Outcome reads `ci-scripts` Ran 108 and contract tests Ran 37, OK. CI#40214713710 on 55bf34259031 is green 17/17. The other seven files' counts are not moved by the report, and they sum to 108 with 37."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 43
    section: "## 4. Unit Test Strategy"
    change: "**Cold-agent pipe:** test_cold_agent.py, 37 tests (33 until 2026-10-25-cold-agent-bar; 27 until 2026-10-15-cold-agent-test, which retired `ColdAgentStubTest` (6) with the stub) — `ColdAgentPipeTest` (37; 33, and 21 before that) under the `claude` shim and the fake `escher-session` (`FAKE`, which speaks the twelve tools, answering `lint` with an empty findings list, Cells as a paged JSON line of two parts and the load page, under `--record` keeping one check per accepted start, writing `tool_call` span lines and recording its spawns); one case each: a passing six-app run (`test_a_six_app_run_passes_with_no_wrong_call`, renamed from the four-app case); wrong calls counted from span lines and agreeing with the transcript, which now fails `wrong-calls-over-bar` (3 wrong of 32); since 2026-10-25-cold-agent-bar, five wrong calls of a hundred passing and six failing the 5 % bar, a paged answer read as its parts joined, each unused capability failing (four sub-cases) and a start that left no kept check failing (two sub-cases); the rest of the case list kept"
    sidecar: "unit-test-strategy#5.1: test_cold_agent.py 33 → 37: the six-app case, the bar, paged, capability and kept-check cases; the fake pages Cells and keeps checks"
    rationale: "The report's Harness / gate surface: contract tests 33 → 37, the renamed six-app case, the four added cases and their sub-cases, the wrong-call span case now failing wrong-calls-over-bar at 3 of 32, and the fake answering Cells paged and the load page while keeping one check per accepted start under --record. The new cases are at test_cold_agent.py:401-405, :433-441, :443-455 and :457-461 (New text). The file gained 123 lines in 34 ranges, so its older citations (:140-141, :94-168) moved, and the citation sweep re-pins them."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 44
    section: "## 4. Unit Test Strategy"
    change: "the 37 read 34 of 37 failing against the base pipe (`e84dc818`) in a scratch copy, a red-before-green control of 2026-10-25-cold-agent-bar (.github/scripts/test_cold_agent.py:401-405; .github/scripts/test_cold_agent.py:433-441; .github/scripts/test_cold_agent.py:443-455; .github/scripts/test_cold_agent.py:457-461); the 33 read `FAILED (failures=33, errors=3)` against `01297a12`'s pipe, the earlier control"
    sidecar: "unit-test-strategy#5.1: red-before-green 34 of 37 against the base pipe (2026-10-25-cold-agent-bar)"
    rationale: "The report's Decisions & corrections: the new test_cold_agent.py fails 34 of 37 against HEAD's pipe in a scratch copy, HEAD being the base e84dc818 at implement. The line numbers are the new cases' blocks from New text."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 44
    section: "## 4. Unit Test Strategy"
    change: "Add after the control: measured at 2026-10-25-cold-agent-bar, a paged snapshot over MCP answers as a JSON line whose quotes are escaped, so a substring match over the raw answer misses the screen's own text, and `test_a_paged_answer_is_read_as_its_parts_joined` asserts the escaped form in the transcript (.github/scripts/test_cold_agent.py:433-441)"
    sidecar: "unit-test-strategy#5.1: a paged MCP answer escapes its quotes, so the pipe's case asserts the escaped form (2026-10-25-cold-agent-bar)"
    rationale: "The report's Decisions & corrections (Measured) and the task's F11. The test's block is at test_cold_agent.py:433-441 in New text."
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 15
    section: "## 5. Integration Test Strategy"
    change: "After the `stand_load_wait` exception add: since 2026-10-25-cold-agent-bar `stand_load_page` holds no sleep or wall-clock read of its own — its click waits out the stand's `LoadNet`, which answers from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms), through the driver's `LOAD_WAIT` (tests/blitz-tests/tests/stand_load_page.rs:34-68) — and `stand_cells_served` moves time only by `advance 5000`"
    sidecar: "integration-test-strategy#2.1: wall time: stand_load_page waits out LoadNet's 200 ms through the driver, stand_cells_served only advances"
    rationale: "The task's Touches ask for §5 Wall time's exceptions to be re-read. The report's Symbols / APIs give LoadNet's `fetch` answering from a thread of its own after LOAD_ANSWER_DELAY = 200 ms (stand.rs:167-175), its Outcome has the load page's click waiting out its load, and its Deviations note that stand_cells_served takes advance 5000. The sleep is the stand's, not the check's, so the two recorded exceptions stand."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 16
    section: "## 5. Integration Test Strategy"
    change: "After the `stand_act_scroll` clause add: since 2026-10-25-cold-agent-bar, measured on the stand's Cells, a main-button click on a cell inside a scrolled box right after `scroll` lands on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the thumb keeps the scrollbar shown, so an `advance` past the fade (5000 ms) comes before the click (`stand_cells_served::a_click_right_after_the_scroll_lands_on_the_shown_scrollbar`; tests/blitz-tests/tests/stand_cells_served.rs:176-192)"
    sidecar: "integration-test-strategy#2.1: a click right after scroll lands on the shown overlay scrollbar; advance past the fade first (2026-10-25-cold-agent-bar)"
    rationale: "The report's Decisions & corrections measured the hazard (events/pointer.rs, hit_with_scrollbar while scrollbar_opacity > 0), and its Deviations note the advance 5000 and the early click moved to its own test. The task's F7; the test's block is at stand_cells_served.rs:176-192 in New text."
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 17
    section: "## 5. Integration Test Strategy"
    change: "and since 2026-10-10-snapshot-states an acting call returns the node it changed with its `selected`, `pressed` or `invalid` reading as it reads after the step — since 2026-10-25-cold-agent-bar a flight-type choice the select `flight-type` with its `value` and its two options with their `selected` reading (`a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`, renamed from the mode-click test; the stand's two `aria-pressed` mode buttons are retired, so `pressed` is pinned on a fixture by `accessibility_states` alone), a row click the row selected and the row it leaves not selected, a typed start date the field invalid or not — the rest kept"
    sidecar: "integration-test-strategy#2.1: stand_act_states' flight witness moves from the retired aria-pressed mode buttons to the flight-type select's value and selected options"
    rationale: "The report's Retired bullet names the flight booker's two aria-pressed mode buttons (flight-one-way, flight-return). Its Symbols / APIs rename the stand_act_states test, and its Deviations say the pressed reading leaves the stand while accessibility_states still pins aria-pressed on a fixture. F2 (flight-mode) marks integration-test-strategy.md:17 stale."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 18
    section: "## 5. Integration Test Strategy"
    change: "and, against the real binary, `start` · `status` · `stop` on the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`; four until 2026-10-25-cold-agent-bar), a command that finds no session ending as a session error, status `3` (`cli_commands`) — the rest kept"
    sidecar: "integration-test-strategy#2.2: cli_commands starts, reads status and stops the six served tasks (2026-10-25-cold-agent-bar)"
    rationale: "The report's Symbols / APIs: escher-session's TASKS holds six slugs (session_host.rs:12-27), and its Outcome Smoke says `cli_commands` starts the six slugs through the binary. F1 (served-tasks). cli_commands.rs gained lines 3-5, 37-41, 137, 266 and 592-597, so its :134-216 citation moved, and the citation sweep re-pins it."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 21
    section: "## 5. Integration Test Strategy"
    change: "`host_log` holds 4 tests (3 until 2026-10-25-cold-agent-bar) and `mcp_log` 2. Since 2026-10-25-cold-agent-bar a paged `part` read through the `escher-session` host's own sink is measured: `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell, reads `snapshot --part 2` and finds 0 id and 0 name needles under the `-p seven_guis` and the workspace build, its needles from `common::cells_needles(part)` (examples/seven_guis/tests/host_log.rs:134-277; examples/seven_guis/tests/common/mod.rs:425-454). This discharges the `.andromeda/residuals.md` line that owned it. The in-process capture reads one too (`stand_act_spans`, §1)"
    sidecar: "integration-test-strategy#2.3: host_log 3 → 4, and the host-sink paged part read is measured on serve cells: 0 id, 0 name needles under both builds, its residual discharged"
    rationale: "The report's F13 (host-sink-part) marks integration-test-strategy.md:21 stale. Its Spec claims disproved say «the host boots only the four lean tasks and none pages» is false since serve cells, and inputs#I4 row 1 discharges residuals.md:38 by that measurement. Its Deviations say host_log's Cells read commits a sentinel first."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 24
    section: "## 5. Integration Test Strategy"
    change: "After the recording clause add: since 2026-10-25-cold-agent-bar, measured by a probe through the built binary (no standing check), a full recorded Cells session keeps a check of 312,431 bytes and the load page's a check of 1,660 bytes, both under the 1,048,576-byte bound"
    sidecar: "integration-test-strategy#2.3: a recorded Cells session's kept check measures 312,431 bytes and the load page's 1,660, under the 1,048,576 bound (probe)"
    rationale: "The report's Decisions & corrections (Measured: the probe through the built binary, target/tmp/recprobe-1) and the task's F12. The kept check of a recording host is the one this entity's recording clause names."
    fact: F12
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 267
    section: "## 5. Integration Test Strategy"
    change: "a paged `part` read through the host's own sink is measured since 2026-10-25-cold-agent-bar — `host_log` runs `serve cells` at `trace` and reads part 2 with 0 id and 0 name needles under both builds, its residual discharged; the windowed binary is read by an evidence instrument, not a standing check, its typed text unmeasured"
    sidecar: "test-plan §5: the host-sink paged part read is measured (host_log on serve cells), no longer unmeasured"
    rationale: "The report's F13 marks test-plan.md:267 stale, and its Spec claims disproved say the clause «the host boots only the four lean tasks and none pages» is false. This is the broad line the integration-test-strategy#2.3 edit details."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 8
    section: "## 6. E2E Test Strategy"
    change: "`cli_commands` (14): each of the six served tasks — the four lean tasks, `cells` and `load` (four until 2026-10-25-cold-agent-bar) — starts, answers and stops; the rest kept"
    sidecar: "e2e-test-strategy#1.1: cli_commands starts, answers and stops the six served tasks (2026-10-25-cold-agent-bar)"
    rationale: "The report's Symbols / APIs give escher-session's six TASKS, and its Outcome Smoke says cli_commands starts the six slugs through the binary. No new cli_commands test is named, so the 14 stands. cli_commands.rs gained lines 3-5, 37-41, 137, 266 and 592-597, so the entity's cli_commands.rs citations moved, and the citation sweep re-pins them."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 12
    section: "## 6. E2E Test Strategy"
    change: "After the trial's citations add: since 2026-10-25-cold-agent-bar a second real client has driven the role, the cold-agent pipe's live run, the founder's one run of 2026-10-25 (`claude` 2.1.308, the operator's own login, not in CI). It used `mcp --record` over the six served apps: `passed`, 66 tool calls, 0 wrong, `recorded_sessions` 6 with `recording_kept` (test-plan §3 Cold-agent run pipe; escher-0.2.0/chunks/2026-10-25-cold-agent-bar/evidence/live-verdict.json)"
    sidecar: "e2e-test-strategy#1.2: the 2026-10-25 cold-agent live run drove the mcp role over six apps under --record"
    rationale: "The report's Outcome v020-11: live-verdict.json outcome passed, 66 · 66 calls, all six tasks met, recorded_sessions 6 and recording_kept. Its Symbols / APIs say the live server child runs `mcp --record`, and its Smoke says the live run drove the mcp role over all six."
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 290
    section: "## 6. E2E Test Strategy"
    change: "a real client drove it in the operator pass, not in CI — the one trial, and since 2026-10-25-cold-agent-bar the cold-agent live run over the six served apps under `--record`"
    sidecar: "test-plan §6: the MCP role's real-client drives name the 2026-10-25 live run beside the trial"
    rationale: "This is the broad line the e2e-test-strategy#1.2 edit details. The report's Outcome v020-11 and Smoke say the live run drove the mcp role over all six apps."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 19
    section: "## 6. E2E Test Strategy"
    change: "since 2026-10-07-driver-session `host_binary` launches the `escher-session` binary, which boots one task headlessly and serves its lifecycle — one of the four lean tasks, or since 2026-10-25-cold-agent-bar `cells` or `load`, its six served slugs; the rest kept"
    sidecar: "e2e-test-strategy#2.2: the escher-session binary boots one of six served tasks, not one lean task alone"
    rationale: "The report's Symbols / APIs: session_host.rs's `enum Served { Lean(LeanTask), Cells, Load }` and its six-slug TASKS (12-27). Cells and the load page are no LeanTask (F1, served-tasks)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "Add a row after it: | Stand screen over the budget | the stand's own Cells screen, served through `stand::boot_cells` beside the `many_items` fixture: 47,949 bytes and 740 lines, read in 5 parts of `SNAPSHOT_TEXT_BUDGET`, walked in parts by `stand_cells_served` and read by part through the host by `host_log` | (examples/seven_guis/src/stand.rs:131-135; tests/blitz-tests/tests/stand_cells_served.rs:91-133) |"
    sidecar: "test-data-fixtures#3.2: the served Cells screen, 47,949 bytes in 5 parts, is a second over-budget screen beside many_items"
    rationale: "The report's Spec claims disproved say Cells reads 47,949 bytes, 740 lines and 5 parts (stand_cells_served::cells_is_walked_in_parts_through_the_driver), and F16 says the stand-budget claim no longer holds for a served screen. The task's F6. The line numbers are boot_cells (stand.rs:131-135) and the walk's block (stand_cells_served.rs:91-133) in New text."
    fact: F6
note: "No pure-citation proposal is written. The moved `path:N` citations in these entities (test-strategy.md:40's stand.rs:59-61; integration-test-strategy.md:20 · :23 · :24's host_log.rs:398-411 · :42-132 · :30-492 · :36-40; e2e-test-strategy.md:8-10's cli_commands.rs ranges; unit-test-strategy.md:43-44's test_cold_agent.py ranges) are left to the P2 citation sweep, because the report states none of their new numbers."
