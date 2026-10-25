proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 93
    section: "## 1. Test Scope Summary"
    change: "the 15 controls' roles and names" becomes "the 14 controls' roles and names (15 until 2026-10-25-cold-agent-bar, the flight booker's two mode buttons `flight-one-way` · `flight-return` giving way to one select `flight-type`, `Role::ComboBox`)" — `stand_accessibility_ids`' pin of the control count moving with it.
    sidecar: test-scope-summary §1 — the stand's controls 15 → 14, the select `flight-type` in place of the two retired mode buttons (2026-10-25-cold-agent-bar).
    rationale: report Counts F4 (`controls-15`, the stand's controls 15 → 14, `controls(task)` rows 18 → 17, test-scope-summary.md:93 stale) and Retired (the flight booker's two `aria-pressed` mode buttons); Decisions names `stand_accessibility_ids` (`checked.len() == 15`) as a count pin re-pinned this chunk.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:93
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 93
    section: "## 1. Test Scope Summary"
    change: after "each lean task mounted in TaskShell offline at the pinned viewport with the bundled font" add "— and, since 2026-10-25-cold-agent-bar, two served apps beside them, neither a `LeanTask` and so neither holding a keyboard or contrast table row: Cells (`seven_guis::stand::boot_cells(HarnessOptions)`, `Task::Cells` mounted in TaskShell) and a load page (`stand::boot_load(HarnessOptions)`, titled \"Load\", keyed `load-fetch` (a Button, \"Load\") and `load-done` (a Paragraph, \"Loaded\", `display: none` until its sheet lands)), the load page's one load answered by the in-process stand-in `LoadNet` after `LOAD_ANSWER_DELAY` (200 ms), every other boot offline (`net_provider: None`) —".
    sidecar: test-scope-summary §1 — the stand boots Cells and the load page beside the lean four; the load page alone online through `LoadNet` (2026-10-25-cold-agent-bar).
    rationale: task New facts F1; report Symbols / APIs (`boot_cells` stand.rs:131-135, `boot_load` 141-152, the load page's ids) and Counts F17 (`stand-offline`, test-scope-summary.md:93 stale — the stand offline but for the load page's stand-in).
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 94
    section: "## 1. Test Scope Summary"
    change: "since 2026-10-10-snapshot-states exactly the active flight mode reading pressed and the reading swapping with the mode" becomes "exactly the chosen flight type's option under `flight-type` reading selected and the reading swapping with the choice (`exactly_the_chosen_flight_type_reads_selected`; until 2026-10-25-cold-agent-bar, `exactly_the_active_flight_mode_reads_pressed`, the active mode button reading pressed — the stand now states no `aria-pressed`, `accessibility_states` pinning it on a fixture)"; and to the snapshot clause add "the flight booker's `flight-type` reading `ComboBox` \"Flight type\" with value `one-way` and holding its two `ListBoxOption`s, one selected, each at the select's client rect (`stand_snapshot::the_flight_type_select_holds_its_two_options`, since 2026-10-25-cold-agent-bar)".
    sidecar: test-scope-summary §1 — the stand's pressed reading gives way to the flight type select's selected reading; `stand_snapshot` pins the select's two options (2026-10-25-cold-agent-bar).
    rationale: report Counts F18 (`stand-pressed`, test-scope-summary.md:94 stale) and F2; Symbols / APIs renamed `stand_snapshot_state` test and new `stand_snapshot` test; Deviations "The pressed reading leaves the stand … `accessibility_states` still pins `aria-pressed` on a fixture".
    basis: .andromeda/test-plan-detail/test-scope-summary.md:94
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 104
    section: "## 1. Test Scope Summary"
    change: in `stand_act_ids` 2, "the 15 controls among them" becomes "the 14 controls among them"; "accepted on 70 and refused on the 7 the check states" becomes "accepted on 68 and refused on the 9 the check states (70 · 7 until 2026-10-25-cold-agent-bar)"; the causes add "and the flight booker's two options `flight-type-one-way` · `flight-type-return`, each at its select's client rect, as `covered`"; "for each of the 12 controls a click is accepted on" becomes "for each control a click is accepted on"; and "15 control rows over `controls(task)` change the screen at boot (18 rows, `back-btn` ×4" becomes "14 control rows over `controls(task)` change the screen at boot (17 rows, `back-btn` ×4; 15 of 18 until 2026-10-25-cold-agent-bar".
    sidecar: test-scope-summary §1 — `stand_act_ids` 68 accepted · 9 refused of 77, 14 controls, 14 changing at boot (2026-10-25-cold-agent-bar).
    rationale: report Counts F6 (`act-ids-census`: of 77 nodes accepted 68 and refused 9, was 70 · 7 — the two options refused `covered`; 14 controls, was 15, 14 changing at boot) and F4 (test-scope-summary.md:104 stale, `controls(task)` rows 18 → 17); the report states no new count of accepted controls, so the "12" is dropped rather than restated.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:104
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 108
    section: "## 1. Test Scope Summary"
    change: "`stand_act_states` 3 (a flight-mode click returns both mode buttons with their pressed reading;" becomes "`stand_act_states` 3 (a flight-type choice returns the select's value and both options with their selected reading — `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`, a flight-mode click returning both mode buttons' pressed reading until 2026-10-25-cold-agent-bar;".
    sidecar: test-scope-summary §1 — `stand_act_states`' flight test reads the select's value and its options' selected reading (2026-10-25-cold-agent-bar).
    rationale: report Counts F2 (`flight-mode`, test-scope-summary.md:108 stale) and Symbols / APIs (the renamed `stand_act_states` test, v020-08's witness).
    basis: .andromeda/test-plan-detail/test-scope-summary.md:108
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 115
    section: "## 1. Test Scope Summary"
    change: in `stand_contrast` 5, "(18 rows, one per `controls(task)` row over the four lean tasks at boot" becomes "(17 rows, one per `controls(task)` row over the four lean tasks at boot — 18 until 2026-10-25-cold-agent-bar, `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400) in place of the `flight-one-way` accent row and the `flight-return` row" and "pass 14 · fail 0 · exempt-inactive 3 · no-text 1" becomes "pass 13 · fail 0 · exempt-inactive 3 · no-text 1 (14 · 0 · 3 · 1 until that chunk)".
    sidecar: test-scope-summary §1 — `stand_contrast` control rows 18 → 17, tally 13 · 0 · 3 · 1, the select at 17.40 (2026-10-25-cold-agent-bar).
    rationale: report Counts F5 (`contrast-rows`: control rows 18 → 17, tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1; the two retired rows give way to `flight-type` 17.40); this line holds the row count and tally the sweep's :116 site continues; the entity is touched whole.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:115
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 116
    section: "## 1. Test Scope Summary"
    change: after "the five former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` now `#ffffff` on `#3a5ce5` at 5.47" add "(`flight-one-way` retired at 2026-10-25-cold-agent-bar)"; "(`STATE_ROWS`, 19 reached states — return-chosen 3," becomes "(`STATE_ROWS`, 18 reached states — return-chosen 2, `flight-type` 17.40 and `flight-return-date` 17.40 (19 and 3 until 2026-10-25-cold-agent-bar, `flight-return` 5.47 · `flight-one-way` 9.33 · `flight-return-date` 17.40)," and "18 pass and 1 exempt" becomes "17 pass and 1 exempt"; and "`stand_keyboard` 14 (11 until 2026-10-14-stand-a11y-assertions;" becomes "`stand_keyboard` 15 (14 until 2026-10-25-cold-agent-bar, 11 until 2026-10-14-stand-a11y-assertions;".
    sidecar: test-scope-summary §1 — `STATE_ROWS` 19 → 18 (17 pass · 1 exempt), return-chosen rows the select and the return date; `stand_keyboard` 14 → 15 tests (2026-10-25-cold-agent-bar).
    rationale: report Counts F5 (`STATE_ROWS` 19 → 18, tally 18 · 0 · 1 · 0 → 17 · 0 · 1 · 0, the return-chosen rows; test-scope-summary.md:116 stale) and F7 (`stand_keyboard` tests 14 → 15); the line's "all 17 measured rows" of the painted-background check has no restated count in the report and is left for the orchestrator's re-read.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:116
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 117
    section: "## 1. Test Scope Summary"
    change: "(20 rows stated and counted, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing)" becomes "(16 rows stated and counted, `BUTTONS` 8 — 20 and 10 until 2026-10-25-cold-agent-bar, `flight-return` no longer among the buttons — `counter-increment`, `flight-book` and `crud-create` changing)"; "the flight booker after `flight-return` (six stops)" becomes "the flight booker after the return flight is chosen on `flight-type` by ArrowDown (five stops, `back-btn` → `flight-type` → `flight-start` → `flight-return-date` → `flight-book`; six, after `flight-return`, until 2026-10-25-cold-agent-bar)"; and add "the flight type select moving its choice by its arrows (`the_flight_type_select_moves_its_choice_by_its_arrows`, since 2026-10-25-cold-agent-bar);".
    sidecar: test-scope-summary §1 — `stand_keyboard`'s activation table 20 → 16 rows, the flight booker's enabled walk five stops through `flight-type`, its arrows test added (2026-10-25-cold-agent-bar).
    rationale: report Counts F7 (`activation-rows`: 20 → 16 rows, `BUTTONS` 10 → 8; test-scope-summary.md:117 stale) and F2 (`flight-mode`: the Tab order `back-btn` → `flight-type` → `flight-start` → `flight-book`, enabled + `flight-return-date`, five stops reached by Tab and ArrowDown; :117 stale); Symbols / APIs names the new `stand_keyboard` test.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:117
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 118
    section: "## 1. Test Scope Summary"
    change: "one id-addressed acting call (`click counter-increment` · `click flight-return` · `advance 300` · `click crud-person-0`)" becomes "one id-addressed acting call (`click counter-increment` · `type flight-type return` · `advance 300` · `click crud-person-0`; `click flight-return` until 2026-10-25-cold-agent-bar)".
    sidecar: test-scope-summary §1 — the sweep's flight booker step is `type flight-type return` (2026-10-25-cold-agent-bar).
    rationale: report Counts F2 (`flight-mode`, test-scope-summary.md:118 stale) and Symbols / APIs ("blitz-tests `session_common::change` moves the flight booker by `type flight-type return` through the driver"); `flight-return` is retired (Retired).
    basis: .andromeda/test-plan-detail/test-scope-summary.md:118
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 119
    section: "## 1. Test Scope Summary"
    change: after "nine rows v020-01 to v020-09 in order, 46 witnesses" add "until 2026-10-25-cold-agent-bar, which added three — v020-07 `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-08 `stand_act_states.rs::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading` and v020-09 `stand_cells_served.rs::cells_is_walked_in_parts_through_the_driver` — and a `Subject::Served` (a page served beside the lean four), re-pointed v010-17's flight witness and v020-05's lint witness to the renamed tests and carried the two new files in `source`;".
    sidecar: test-scope-summary §1 — `WITNESSES_020` gains three witnesses and `Subject::Served` (2026-10-25-cold-agent-bar).
    rationale: report Counts F20 (`witnesses`: test-scope-summary.md:119 stale) and Symbols / APIs (`stand_requirement_sweep`: `Subject::Served`, three `WITNESSES_020` rows, two re-pointed witnesses, `source` carrying the two new files; its own test count unchanged at 3); the report states no new witness total, so the 46 is dated rather than restated.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:119
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 123
    section: "## 1. Test Scope Summary"
    change: "each lean task at boot reads 0 findings;" becomes "the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` in both layout modes — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the other three 0 (`the_stand_at_boot_reads_its_measured_findings`, re-pinned as measured on the founder's «Перезакрепить как измерено (Рекомендую)», inputs#I5; 0 on each until 2026-10-25-cold-agent-bar);".
    sidecar: test-scope-summary §1 — `stand_lint` at boot reads `[0, 1, 0, 0]`, the flight booker's options overlapping, re-pinned as measured (2026-10-25-cold-agent-bar).
    rationale: report Counts F3 (`lint-at-boot`, test-scope-summary.md:123 stale), Spec claims disproved by measurement (a 0-finding lean stand false once the select stands) and Decisions (inputs#I5).
    basis: .andromeda/test-plan-detail/test-scope-summary.md:123
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: "## 1. Test Scope Summary"
    change: "7 findings on the planted page, 0 on each lean task at boot" becomes "7 findings on the planted page, the measured `[0, 1, 0, 0]` on the lean tasks at boot (0 on each until 2026-10-25-cold-agent-bar)".
    sidecar: test-scope-summary §1 — the `lint` verb's lean-task reading at boot restated with the measured findings (2026-10-25-cold-agent-bar).
    rationale: the same moved fact as the :123 occurrence (report Counts F3, Spec claims disproved — the 0-finding lean stand measured false); this line restates it for `the_lint_verb_answers_what_the_reader_reads`, and the entity is touched whole.
    basis: .andromeda/test-plan-detail/test-scope-summary.md:122
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 123
    section: "## 1. Test Scope Summary"
    change: after the restated lean-task lint reading add "— that `overlap` a driver defect, not a stand defect: the lint reads a select's options as overlapping because the snapshot gives each option its select's client rect, the fix a driver change for every document (`packages/escher-driver/src/lint.rs`, `overlaps`), unresolved, its owner pinned at 2026-10-25-cold-agent-bar's wrap;".
    sidecar: test-scope-summary §1 — the lint's select-option `overlap` recorded as an unresolved driver defect beside `stand_lint` (2026-10-25-cold-agent-bar).
    rationale: task New facts F8; report Insufficient fixes (`stand_lint` re-pinned, the defect in `lint.rs` `overlaps` unresolved, owner at this wrap's P5) and Counts F21.
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 124
    section: "## 1. Test Scope Summary"
    change: after the `stand_load_wait` clause ("… as measured at escher-0.2.0/chunks/2026-10-20-load-wait/report.md;") add "and, since 2026-10-25-cold-agent-bar, `stand_load_page` 2, through `Session::run` in both layout modes on the stand's load page (`stand::boot_load`): a click on `load-fetch` waits out its load, which `LoadNet` answers after 200 ms, and `load-done` shows \"Loaded\" (`the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-07's witness); the page is keyed and reads no lint finding at boot (`the_load_page_is_keyed_and_reads_no_finding_at_boot`); declaring no `mod fixtures;`, picked up by `run stand` and the stand-repeat leg by its prefix;".
    sidecar: test-scope-summary §1 — `stand_load_page` 2 added beside `stand_load_wait` (2026-10-25-cold-agent-bar).
    rationale: task New facts F5; report Files (new `stand_load_page.rs`, 84 lines), New text (tests 34-68 and 70-84, each `for incremental in [false, true]`), Outcome (`stand_cells_served` and `stand_load_page` green in both layout modes, 4 · 2), Harness / gate surface (`stand-repeat` takes the two new files by prefix).
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 126
    section: "## 1. Test Scope Summary"
    change: after the `stand_snapshot_parts` clause add "; and, since 2026-10-25-cold-agent-bar, `stand_cells_served` 4, importing from `session_common`, on the stand's own Cells (`stand::boot_cells`, an over-budget screen of 47,949 bytes in 5 parts) through `Session::run`, each test in both layout modes: Cells walked in parts through the driver (`cells_is_walked_in_parts_through_the_driver`, v020-09's witness); committed cells reading their values and a cell past the viewport reading in the last part; a click right after the scroll landing on the shown overlay scrollbar, so the walk takes `advance 5000` before its click; and the lint at boot reading its measured findings — declaring no `mod fixtures;`, picked up by `run stand` and the stand-repeat leg by its prefix".
    sidecar: test-scope-summary §1 — `stand_cells_served` 4 added: Cells walked in parts through the driver (2026-10-25-cold-agent-bar).
    rationale: task New facts F4; report Files (new `stand_cells_served.rs`, 209 lines), New text (four tests at 91-133 · 135-174 · 176-192 · 194-209, each `for incremental in [false, true]`; `session_common` use block 19-21), Counts F16 (Cells 47,949 bytes in 5 parts), Deviations (`advance 5000` after the scroll), Outcome (4 tests green in both modes).
    fact: F4
note: the entity's stale citations (:113 `stand_act_diff.rs:1-13` · `stand_act_states.rs:79-170`, :122 `stand_lint.rs:415-442`, files whose lines this chunk added before or inside those ranges) are the P2 citation sweep's; stale sites outside this entity — test-scope-summary.md:8 (F9), :15 · :19 (F13 `host_log`) — and F3 · F9 (seven_guis stand unit tests, `common::cells_needles`), which this tests/blitz-tests entity cannot carry, are left to their own pass.
