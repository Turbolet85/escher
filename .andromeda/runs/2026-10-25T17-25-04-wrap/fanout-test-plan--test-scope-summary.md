proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 8
    section: "## 1. Test Scope Summary"
    change: "CI Python scripts: the ci-scripts count reads `108 tests in eight test files (§4; 104 until 2026-10-25-cold-agent-bar, whose test_cold_agent.py went 33 → 37; 100 in seven until test_capability_statement.py added its 4; …)`, the rest of the line kept."
    sidecar: "test-scope-summary §1 CI Python scripts: ci-scripts 104 → 108 tests, test_cold_agent.py 33 → 37 (2026-10-25-cold-agent-bar)."
    rationale: "Report F9 moves the ci-scripts leg 104 → 108 and test_cold_agent.py 33 → 37, naming test-scope-summary.md:8 stale; Outcome reads `ci-scripts` Ran 108."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:144
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 15
    section: "## 1. Test Scope Summary"
    change: "seven_guis host_log: `tests/host_log.rs` (4 tests, unix only; 3 until 2026-10-25-cold-agent-bar added `a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace` — `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, then `snapshot --part 2`, 0 id and 0 name needles on stderr under both builds, so a paged part read through the host's own sink is measured (examples/seven_guis/tests/host_log.rs:134-277); 1 until 2026-10-19-session-recording …), the rest kept."
    sidecar: "test-scope-summary §1 seven_guis: host_log 3 → 4 tests, the Cells part read through the host's sink measured (2026-10-25-cold-agent-bar)."
    rationale: "Report F13 moves host_log 3 → 4 tests and names test-scope-summary.md:15 stale; Symbols lists the new test; New text lists host_log.rs 134-277."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 14
    section: "## 1. Test Scope Summary"
    change: "seven_guis tests/common/mod.rs: after the log checks' fragment add `and, since 2026-10-25-cold-agent-bar, `cells_needles(part) -> (Vec<String>, String)`, part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds, read by host_log's Cells part read (examples/seven_guis/tests/common/mod.rs:425-454)`."
    sidecar: "test-scope-summary §1 seven_guis: common::cells_needles added (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols names seven_guis tests `common::cells_needles` (common/mod.rs:425-454), the helper of the host_log Cells part read; the line states the module's helpers one by one."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:54
    fact: F13
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 18
    section: "## 1. Test Scope Summary"
    change: "seven_guis cli_commands: `and `cli_commands` starts, reads and stops all six served tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (the four lean tasks until 2026-10-25-cold-agent-bar)`, replacing `all four tasks`."
    sidecar: "test-scope-summary §1 seven_guis: cli_commands starts the six served slugs (2026-10-25-cold-agent-bar)."
    rationale: "Report F1: escher-session serves six tasks (`TASKS: [(&str, Served); 6]`); Outcome Smoke reads `cli_commands starts the six slugs through the binary`."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:435
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 18
    section: "## 1. Test Scope Summary"
    change: "seven_guis library unit tests: after `stand::tests::the_timer_step_takes_the_largest_advance` add `and, since 2026-10-25-cold-agent-bar, `stand::tests::the_stand_options_stay_offline` (`stand::options` keeps `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight` (examples/seven_guis/src/stand.rs:283-291; examples/seven_guis/src/stand.rs:293-306; examples/seven_guis/src/stand.rs:308-329) — `cargo test -p seven_guis --locked --lib stand::` reads 5 passed`."
    sidecar: "test-scope-summary §1 seven_guis: three stand unit tests added, `--lib stand::` 5 passed (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols names the three new stand unit tests; New text lists stand.rs 283-291 · 293-306 · 308-329; the gate `cargo test -p seven_guis --locked --lib stand::` reads 5 passed."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:400
    fact: F17
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 93
    section: "## 1. Test Scope Summary"
    change: "tests/blitz-tests stand: after `the seven_guis headless stand — each lean task mounted in TaskShell offline at the pinned viewport with the bundled font` add `, and since 2026-10-25-cold-agent-bar Cells (`boot_cells`) and the load page (`boot_load`, whose `net_provider` is the in-process `LoadNet`, answering its one sheet after `LOAD_ANSWER_DELAY`, 200 ms, with no `blitz_net`) served beside them, every other boot offline with `net_provider: None``."
    sidecar: "test-scope-summary §1 blitz-tests: the stand boots Cells and the load page, the load page's LoadNet its one network stand-in (2026-10-25-cold-agent-bar)."
    rationale: "Report F17 names test-scope-summary.md:93 stale: the stand is offline but for the load page's in-process stand-in; Symbols describes boot_cells, boot_load and LoadNet."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:185
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 93
    section: "## 1. Test Scope Summary"
    change: "tests/blitz-tests stand: `the 14 controls' roles and names (15 until 2026-10-25-cold-agent-bar replaced the flight booker's two mode buttons with the select `flight-type`)`, replacing `the 15 controls' roles and names`."
    sidecar: "test-scope-summary §1 blitz-tests: the stand's controls 15 → 14 (2026-10-25-cold-agent-bar)."
    rationale: "Report F4: the stand's controls 15 → 14, `controls(task)` rows 18 → 17; stand_accessibility_ids re-pinned (`checked.len()` was 15, Decisions)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:123
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 93
    section: "## 1. Test Scope Summary"
    change: "tests/blitz-tests Tab order: `… CRUD's restated at 2026-10-13-stand-keyboard-harness to six stops, …, and the flight booker's restated at 2026-10-25-cold-agent-bar to `back-btn` → `flight-type` → `flight-start` → `flight-book`, Counter's and Timer's unchanged`, replacing `the three other tasks' unchanged`."
    sidecar: "test-scope-summary §1 blitz-tests: the flight booker's Tab order names the select flight-type (2026-10-25-cold-agent-bar)."
    rationale: "Report F2: the flight booker's mode is the select `flight-type`, Tab `back-btn` → `flight-type` → `flight-start` → `flight-book`; `common` tables hold `flight-type` in `tab_order`."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:114
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "stand_snapshot_state: `since 2026-10-25-cold-agent-bar exactly the chosen flight type's option reading selected and the reading swapping with the choice (`exactly_the_chosen_flight_type_reads_selected`; until then the active flight mode reading pressed, its two `aria-pressed` buttons retired), exactly the selected CRUD row reading selected …, and a date that is no date reading invalid on both date inputs — no other node reading pressed, selected or invalid, the stand stating no `aria-pressed``, replacing the active-flight-mode pressed clause."
    sidecar: "test-scope-summary §1 blitz-tests: stand_snapshot_state reads the chosen flight type selected, the mode buttons' pressed reading retired (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired: the flight booker's two `aria-pressed` mode buttons; F18 names test-scope-summary.md:94 stale; Symbols renames `exactly_the_active_flight_mode_reads_pressed` → `exactly_the_chosen_flight_type_reads_selected`."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "stand_snapshot: add `and, since 2026-10-25-cold-agent-bar, `the_flight_type_select_holds_its_two_options` — the select reads `ComboBox` `Flight type` with value `one-way` and its two `ListBoxOption`s `flight-type-one-way` (selected) and `flight-type-return` (unselected) under it, each at the select's client rect, in both layout modes (tests/blitz-tests/tests/stand_snapshot.rs:118-170)`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_snapshot pins the flight-type select's two options (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols lists the new test `stand_snapshot::the_flight_type_select_holds_its_two_options` and the select's snapshot reading; Deviations: the options are pinned there, not in the tables; New text lists stand_snapshot.rs 118-170."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:47
    fact: F2
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 104
    section: "## 1. Test Scope Summary"
    change: "stand_act_ids: `the 14 controls among them` replacing `the 15 controls among them`, and `14 control rows over `controls(task)` change the screen at boot (17 rows, `back-btn` ×4; …)` replacing `15 control rows … (18 rows, …)`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_act_ids' control count 15 → 14, rows 18 → 17 (2026-10-25-cold-agent-bar)."
    rationale: "Report F4 (controls 15 → 14, `controls(task)` rows 18 → 17, test-scope-summary.md:104 named stale) and F6 (14 controls, 14 changing at boot)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:123
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 104
    section: "## 1. Test Scope Summary"
    change: "stand_act_ids census: `a driver `click` naming it is accepted on 68 and refused on the 9 the check states from the measured stand (70 · 7 until 2026-10-25-cold-agent-bar), each with its cause — the three disabled controls as `disabled`, each task's `task-header-spacer` … as `covered`, and the two `flight-type` options, each at its select's client rect, as `covered``, and `for each of the 11 controls a click is accepted on` replacing `the 12 controls`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_act_ids accepted 70 → 68, refused 7 → 9, the select's options refused covered (2026-10-25-cold-agent-bar)."
    rationale: "Report F6: of 77 snapshot nodes a click is accepted on 68 and refused on 9 (was 70 · 7 — the two options refused `covered`), 14 controls; 11 is the 14 controls less the three disabled the line names."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:133
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 108
    section: "## 1. Test Scope Summary"
    change: "stand_act_states 3: `(a flight-type choice returns the select's value and both options' selected reading — `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`, renamed at 2026-10-25-cold-agent-bar from the flight-mode click returning both mode buttons with their pressed reading; …`, replacing the flight-mode click clause."
    sidecar: "test-scope-summary §1 blitz-tests: stand_act_states' flight test reads the select's choice, the mode buttons retired (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired names the two mode buttons; F2 names test-scope-summary.md:108 stale; Symbols renames `a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` and makes it v020-08's witness."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:61
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 115
    section: "## 1. Test Scope Summary"
    change: "stand_contrast: `(17 rows, one per `controls(task)` row over the four lean tasks at boot, …; pass 13 · fail 0 · exempt-inactive 3 · no-text 1 since 2026-10-25-cold-agent-bar, where `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400) took the place of the `flight-one-way` and `flight-return` rows — 18 rows, pass 14, from 2026-10-14-stand-a11y-assertions until then`, replacing `(18 rows … pass 14 · fail 0 · exempt-inactive 3 · no-text 1 since …`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_contrast control rows 18 → 17, tally 13 · 0 · 3 · 1, flight-type 17.40 (2026-10-25-cold-agent-bar)."
    rationale: "Report F5: control rows 18 → 17 (tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1), the flight-one-way and flight-return rows giving way to flight-type at 17.40; the line holds the 18-row tally."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 116
    section: "## 1. Test Scope Summary"
    change: "stand_contrast STATE_ROWS: `(`STATE_ROWS`, 18 reached states — return-chosen 2 (`flight-type` 17.40, `flight-return-date` 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — 17 pass and 1 exempt, …; 19 with return-chosen 3 until 2026-10-25-cold-agent-bar)`, replacing `19 reached states — return-chosen 3, … — 18 pass and 1 exempt`."
    sidecar: "test-scope-summary §1 blitz-tests: STATE_ROWS 19 → 18, return-chosen 3 → 2, 17 pass · 1 exempt (2026-10-25-cold-agent-bar)."
    rationale: "Report F5: `STATE_ROWS` 19 → 18 (tally 18 · 0 · 1 · 0 → 17 · 0 · 1 · 0), return-chosen rows now `flight-type` 17.40 and `flight-return-date` 17.40; test-scope-summary.md:116 named stale. The painted-background count (17 measured rows) is not restated by the report and is left as it reads."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125
    dependent-of: D-tests-coverage
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 116
    section: "## 1. Test Scope Summary"
    change: "stand_contrast history: `— pass 9 · fail 5 until then, the five former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` then `#ffffff` on `#3a5ce5` at 5.47, `flight-one-way` retired at 2026-10-25-cold-agent-bar —`, so no retired button reads as standing."
    sidecar: "test-scope-summary §1 blitz-tests: flight-one-way no longer named as a standing contrast row (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired: the flight booker's mode buttons `flight-one-way` and `flight-return`; the line says the former fails, `flight-one-way` among them, are `now` white on the accent."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 116
    section: "## 1. Test Scope Summary"
    change: "stand_keyboard count: `since 2026-10-13-stand-keyboard-harness `stand_keyboard` 15 (14 until 2026-10-25-cold-agent-bar added the flight-type select's arrow test; 11 until 2026-10-14-stand-a11y-assertions; …)`, replacing `stand_keyboard` 14 (11 until …`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_keyboard 14 → 15 tests (2026-10-25-cold-agent-bar)."
    rationale: "Report F7: `stand_keyboard`'s tests 14 → 15; Coverage of new surfaces reads `stand_keyboard` 15."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:138
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: "## 1. Test Scope Summary"
    change: "stand_keyboard activation: `Enter and Space on a focused stand button read as its click (16 rows stated and counted — 20 until 2026-10-25-cold-agent-bar retired the flight booker's two mode buttons, `BUTTONS` 10 → 8 — `counter-increment`, `flight-book` and `crud-create` changing)`, replacing the 20-row clause naming `flight-return`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_keyboard's activation table 20 → 16 rows, flight-return gone (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired names `flight-return`; F7: the activation table 20 → 16 rows (`BUTTONS` 10 → 8), test-scope-summary.md:117 named stale."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:138
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: "## 1. Test Scope Summary"
    change: "stand_keyboard enabled states: `the flight booker after the return flight is chosen on `flight-type` by ArrowDown (five stops, `flight-return-date` added; after `flight-return`, six stops, until 2026-10-25-cold-agent-bar) and CRUD after a row is selected (eight stops)`, replacing `the flight booker after `flight-return` (six stops)`."
    sidecar: "test-scope-summary §1 blitz-tests: ENABLED_TAB_ORDER's flight booker state reached by ArrowDown on flight-type, five stops (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired names `flight-return`; F2: enabled, `flight-return-date` added, five stops, reached by Tab and ArrowDown; test-scope-summary.md:117 named stale."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:114
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: "## 1. Test Scope Summary"
    change: "stand_keyboard: add `and, since 2026-10-25-cold-agent-bar, `the_flight_type_select_moves_its_choice_by_its_arrows` — the focused `flight-type` select's arrows move its choice, in both layout modes (tests/blitz-tests/tests/stand_keyboard.rs:249-309)`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_keyboard's flight-type arrow test added (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols lists the new test `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`; New text lists stand_keyboard.rs 249-309, its body `for incremental in [false, true]`."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:65
    fact: F2
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "stand_requirement_sweep acting calls: replace `click flight-return` with the flight booker's choice of the return flight on `flight-type` (a `type` on the select, as `session_common::change` now moves it), the list otherwise kept."
    sidecar: "test-scope-summary §1 blitz-tests: the sweep's flight booker call no longer names the retired flight-return (2026-10-25-cold-agent-bar)."
    rationale: "Report Retired names `flight-return`; F2 names test-scope-summary.md:118 stale; Symbols: `session_common::change` moves the flight booker by `type flight-type return`. The report does not spell the sweep's own call, so the orchestrator should confirm its exact form against the report's stand_requirement_sweep rows."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:52
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 119
    section: "## 1. Test Scope Summary"
    change: "WITNESSES_020: `nine rows v020-01 to v020-09 in order, 49 witnesses (46 until 2026-10-25-cold-agent-bar added v020-07's `stand_load_page::the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-08's `stand_act_states::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` and v020-09's `stand_cells_served::cells_is_walked_in_parts_through_the_driver`, with the subject `Subject::Served`, a page served beside the lean four), … v020-08 one `Unit` (`node/select.rs`) beside its stand witness, …`."
    sidecar: "test-scope-summary §1 blitz-tests: WITNESSES_020 gains three witnesses and Subject::Served (2026-10-25-cold-agent-bar)."
    rationale: "Report F20: `WITNESSES_020` gains three Served / flight rows and the Served subject, test-scope-summary.md:119 named stale; Symbols names the three witnesses; 49 is the stated 46 plus those three."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:56
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 123
    section: "## 1. Test Scope Summary"
    change: "stand_lint at boot: `each lean task at boot reads its measured findings — the flight booker one `overlap` (`flight-type-one-way` × `flight-type-return`, the two options at their select's client rect), the other three 0 (`the_stand_at_boot_reads_its_measured_findings`, renamed at 2026-10-25-cold-agent-bar from `the_stand_at_boot_reads_no_finding` and re-pinned as measured on the founder's «Перезакрепить как измерено (Рекомендую)»; the driver lint's reading of a select's options as overlapping an unresolved defect in `packages/escher-driver/src/lint.rs`, `overlaps`)`, replacing `each lean task at boot reads 0 findings`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_lint at boot reads [0, 1, 0, 0], the select's options overlapping (2026-10-25-cold-agent-bar)."
    rationale: "Report F3 names test-scope-summary.md:123 stale; Spec claims disproved: the lean tasks' lint measured `[0, 1, 0, 0]` in both layout modes; Insufficient fixes names the driver defect; Decisions records the founder's word (inputs#I5)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:120
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: "## 1. Test Scope Summary"
    change: "stand_lint verb test: `which answers what the reader reads — 7 findings on the planted page and each lean task's measured findings at boot, the flight booker's one `overlap` (0 on each until 2026-10-25-cold-agent-bar)`, replacing `0 on each lean task at boot`."
    sidecar: "test-scope-summary §1 blitz-tests: the lint verb test reads the lean tasks' measured boot findings (2026-10-25-cold-agent-bar)."
    rationale: "Report F3: the flight booker reads one `overlap` at boot, the other three 0; a second occurrence of the retired 0-finding claim in the same entry."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:120
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 126
    section: "## 1. Test Scope Summary"
    change: "after the stand_snapshot_parts entry add `; and, since 2026-10-25-cold-agent-bar, `stand_cells_served` 4, a stand check declaring `mod session_common;`, through `Session::run` in both layout modes on Cells held as `cells` through `stand::boot_cells` — the screen walked in parts through the driver, 47,949 bytes in 5 parts (`cells_is_walked_in_parts_through_the_driver`, v020-09's witness), committed cells reading their values and a cell past the viewport reading in the last part, a click right after the scroll landing on the shown overlay scrollbar (the walk to Z25 taking `advance 5000` after the scroll), and the lint at boot reading its measured findings (tests/blitz-tests/tests/stand_cells_served.rs:91-133; tests/blitz-tests/tests/stand_cells_served.rs:135-174; tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:194-209); picked up by `run stand` and the stand-repeat leg by its prefix`."
    sidecar: "test-scope-summary §1 blitz-tests: stand_cells_served 4 added (2026-10-25-cold-agent-bar)."
    rationale: "Report Files lists the new `stand_cells_served.rs`; Symbols names it v020-09's witness; New text lists its four tests; F16 measures Cells at 47,949 bytes in 5 parts; Deviations: `advance 5000` after the scroll; Harness: `stand-repeat` takes the two new files by prefix; Outcome: green in both layout modes (4)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:13
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 126
    section: "## 1. Test Scope Summary"
    change: "after stand_cells_served add `; and `stand_load_page` 2, a stand check through `Session::run` in both layout modes on the load page held as `load` through `stand::boot_load`, whose `LoadNet` answers its sheet after `LOAD_ANSWER_DELAY` (200 ms) — the `load-fetch` click waiting out its load and `load-done` reading `Loaded` (`the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-07's witness), and the page keyed and reading no finding at boot (tests/blitz-tests/tests/stand_load_page.rs:34-68; tests/blitz-tests/tests/stand_load_page.rs:70-84); picked up by `run stand` and the stand-repeat leg by its prefix, declaring no `mod fixtures;``."
    sidecar: "test-scope-summary §1 blitz-tests: stand_load_page 2 added (2026-10-25-cold-agent-bar)."
    rationale: "Report Files lists the new `stand_load_page.rs`; Symbols describes boot_load, LoadNet and the page's ids and names it v020-07's witness; New text lists its two tests; F10: the two new files declare no `mod fixtures;`; Outcome: green in both layout modes (2)."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:30
    fact: F17
note: "Line citations this chunk moved in the file (host_log.rs at lines 15 · 17 · 19, stand.rs at 18 · 19, stand_keyboard.rs at 116-118, stand_lint.rs at 122-124, stand_contrast.rs at 116, stand_act_states.rs and stand_act_ids.rs at 113, stand_requirement_sweep.rs at 118-119) are left to the P2 citation sweep; D-tests-framework and D-tests-obs-harness find no drift in this category."
