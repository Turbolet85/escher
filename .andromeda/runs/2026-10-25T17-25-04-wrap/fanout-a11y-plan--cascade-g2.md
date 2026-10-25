proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 11
    section: "## 7. Screen Reader Support"
    change: "Add after the entity, in its shape: - Since 2026-10-25-cold-agent-bar a lean stand task holds a single-choice select: the flight booker's `select#flight-type`, named `Flight type` by `aria-label`, reads `ComboBox` with value `one-way` and its two options `flight-type-one-way` and `flight-type-return` as `ListBoxOption` children, named `one-way flight` and `return flight`, each at the select's client rect; `stand_snapshot::the_flight_type_select_holds_its_two_options` and `stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected` pin the two options with exactly the chosen one reading selected, in both layout modes (examples/seven_guis/src/tasks/flight_booker.rs:66-86; tests/blitz-tests/tests/stand_snapshot.rs:118-170; tests/blitz-tests/tests/stand_snapshot_state.rs:121-130) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "screen-reader-support: the stand's flight-type select and its two named options, exactly the chosen one selected, recorded beside the select tree entity (F2)"
    rationale: "Task New facts F2; report Symbols / APIs (flight_booker.rs:66-86: the select, its options, `ComboBox \"Flight type\"` with two `ListBoxOption`s at the select's client rect) and New tests `stand_snapshot::the_flight_type_select_holds_its_two_options`, the rename to `exactly_the_chosen_flight_type_reads_selected`; New text stand_snapshot.rs 118-170, stand_snapshot_state.rs 121-130."
    basis: examples/seven_guis/src/tasks/flight_booker.rs:66-86
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 17
    section: "## 7. Screen Reader Support"
    change: "Append to the entity: since 2026-10-25-cold-agent-bar the stand holds 14 such controls, not 15 — the flight booker's two mode buttons retired for one `select#flight-type`, role `ComboBox`, named `Flight type` by `aria-label`, which the shared module carries in `controls`, `tab_order`, `ENABLED_TAB_ORDER` and `rendered` as `Role::ComboBox`, and `stand_accessibility_ids` re-pinned to the 14 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "screen-reader-support: the stand's interactive controls 15 → 14, the flight-type select in the shared tables in place of the two mode buttons"
    rationale: "Report F4 (the stand's controls 15 → 14, `controls(task)` rows 18 → 17); Symbols / APIs (`common` tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order`, `ENABLED_TAB_ORDER`, `rendered`); scope record companion `stand_accessibility_ids.rs` pinning the stand's controls, re-pinned (New text line 181)."
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 25
    section: "## 7. Screen Reader Support"
    change: "Replace «exactly the active flight mode reads pressed» with «exactly the selected row reads selected, a date that is no date reads invalid; since 2026-10-25-cold-agent-bar the stand states no `aria-pressed` — the flight booker's mode is the `flight-type` select, exactly its chosen option reading selected (`stand_snapshot_state::exactly_the_chosen_flight_type_reads_selected`, renamed from `exactly_the_active_flight_mode_reads_pressed`; tests/blitz-tests/tests/stand_snapshot_state.rs:121-130), and `pressed` is pinned on a fixture alone, by `accessibility_states`», keeping the 2026-10-10 citation's dated tag"
    sidecar: "screen-reader-support: the snapshot state reading on the stand — the flight mode's pressed reading gives way to the flight-type select's selected option"
    rationale: "Report Retired (the two `aria-pressed` mode buttons, the stand's only ones), F18 (`stand-pressed` stale at screen-reader-support.md:25), Renamed tests, Deviations («The pressed reading leaves the stand … `accessibility_states` still pins `aria-pressed` on a fixture»)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 37
    section: "## 7. Screen Reader Support"
    change: "Append to the entity: since 2026-10-25-cold-agent-bar `escher-session` serves six tasks — the lean four, Cells and the load page — and the check returns nothing on all six: on the lean four through the stand checks, on Cells as above, and on the load page by `stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot`, in both layout modes (tests/blitz-tests/tests/stand_load_page.rs:70-84); the flight booker's select and its two options read their author keys `flight-type`, `flight-type-one-way` and `flight-type-return` — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "screen-reader-support: the actionable-key check read empty on the six served tasks, the load page beside the lean four and Cells"
    rationale: "Report F1 (`served-tasks` stale at screen-reader-support.md:37), Outcome a11y criterion («`unkeyed_actionable` empty on the six served tasks: MET … Cells and the load page by their own checks»), Coverage of new surfaces (the load page keyed, lint 0); New text stand_load_page.rs 70-84."
    basis: tests/blitz-tests/tests/stand_load_page.rs:70-84
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 38
    section: "## 7. Screen Reader Support"
    change: "Add after the entity, in its shape: - The load page, served beside the lean four and no `LeanTask`, holds two elements: `load-fetch`, a Button named `Load` (white on the stand accent `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`), and `load-done`, a Paragraph `Loaded` at `display: none` until its sheet lands, so out of the tree and the snapshot until then; `stand_load_page` pins them in both layout modes — a click waiting out the 200 ms load and the message then read, and the page keyed with no finding at boot; it carries no Tab-order, keyboard-activation or contrast row (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:185-200; examples/seven_guis/src/stand.rs:239-243; tests/blitz-tests/tests/stand_load_page.rs:34-68; tests/blitz-tests/tests/stand_load_page.rs:70-84) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "screen-reader-support: the load page's two elements, the Load button and the Loaded paragraph out of the tree until its sheet lands (F6)"
    rationale: "Task New facts F6; report Symbols / APIs (`boot_load` 141-152, `LoadPage` 185-200, `.load-done` 239-243, the ids `load-fetch` and `load-done`, no CSS selecting by id) and Coverage of new surfaces (a11y ✓ for the page's two elements, no keyboard or contrast row); New text stand_load_page.rs 34-68 · 70-84."
    basis: tests/blitz-tests/tests/stand_load_page.rs:34-68
    fact: F6
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 44
    section: "## 7. Screen Reader Support"
    change: "Append to the entity: since 2026-10-25-cold-agent-bar the flight booker's two `aria-pressed` mode buttons `flight-one-way` and `flight-return` are retired with their CSS — its mode is one `select#flight-type`, named `Flight type` by `aria-label`, whose options `flight-type-one-way` and `flight-type-return` state the choice by `selected` — so the stand states no `aria-pressed` and two kinds of control state by attribute remain, the CRUD list's `aria-selected` and the date inputs' `aria-invalid`; `aria-pressed` stays pinned on a fixture by `accessibility_states`, and v010-17's flight witness moved to `stand_act_states::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`; the citations flight_booker.rs:69 and :76 went with the buttons, and the flight_booker.rs and stand_snapshot_state.rs citations moved with the change (examples/seven_guis/src/tasks/flight_booker.rs:66-86) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "screen-reader-support: the flight booker's aria-pressed mode buttons retired; the stand states no aria-pressed, its mode a select stating its choice by selected"
    rationale: "Report Retired bullet (`flight-one-way`, `flight-return`, `.flight-type-row` · `.type-btn` · `.type-btn-active`, the stand's only `aria-pressed` controls); F18 (`stand-pressed` stale at screen-reader-support.md:44), F2 (`flight-mode` stale at :44); Deviations (the v010-17 witness moved; `accessibility_states` still pins `aria-pressed`)."
    basis: examples/seven_guis/src/tasks/flight_booker.rs:66-86
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan.md
    near: 196
    section: "## 7. Screen Reader Support"
    change: "Replace the line with: - On the stand two kinds of control state a role or a state by attribute alone — the CRUD list `listbox` with `option` rows carrying `aria-selected`, and the flight booker's date inputs `aria-invalid`; the stand states no `aria-pressed`, the flight booker's mode a `select` whose options state its choice by `selected`; one gap stands open: the rows are not focusable and the tree's focus is the list, never the option, the missing `aria-activedescendant` kept unread (§5 Per-surface keyboard shortcuts)."
    sidecar: "a11y-plan §7: the stand's states by attribute drop the retired aria-pressed mode buttons"
    rationale: "Report Retired bullet; F2 (`flight-mode` stale at a11y-plan.md:196) and F18 (`stand-pressed` stale at a11y-plan.md:196); Expected amendments (a11y-plan §7, its state reading in place of `aria-pressed`)."
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 9
    section: "## 8. Cognitive Accessibility"
    change: "Append to the entity: since 2026-10-25-cold-agent-bar the flight booker's sequence in `tab_order` is restated to four stops, `back-btn`, `flight-type`, `flight-start`, `flight-book`, and its enabled order in `ENABLED_TAB_ORDER` adds `flight-return-date`, five stops, reached by Tab and ArrowDown — the two mode buttons retired for the `flight-type` select — the error-recovery cue itself (the `invalid` class, `aria-invalid`, the disabled Book) unchanged; the flight_booker.rs, common/mod.rs, stand_accessibility_ids.rs and stand_snapshot.rs citations moved with the change and are re-cited from the files — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "cognitive-accessibility: the flight booker's Tab sequence restated to the flight-type select; the error cue unchanged"
    rationale: "Report F2 (Tab: `back-btn` → `flight-type` → `flight-start` → `flight-book`; enabled + `flight-return-date`, five stops); the entity's «the three other tasks' unchanged» is no longer so for the flight booker; Files list (flight_booker.rs, common/mod.rs, stand_accessibility_ids.rs, stand_snapshot.rs changed — New text adds lines above or inside each cited range). The task's Untouched list keeps the date inputs' `aria-invalid`, the class and the disabled Book."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "Append to the line: since 2026-10-25-cold-agent-bar the leg reads 85 tests over the same fourteen files, its target list and pin unchanged — `stand_keyboard` 14 → 15, `the_flight_type_select_moves_its_choice_by_its_arrows` added — green locally (`ci-leg.sh a11y`: 14 files, 85 passed) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "a11y ci-integration: the a11y leg 84 → 85 tests over the same fourteen files, stand_keyboard 14 → 15"
    rationale: "Report F8 (the a11y leg 14 files, 84 → 85 tests; `a11y-84` stale at a11y ci-integration.md:10, dated at :11), F7 (`stand_keyboard` tests 14 → 15), Gates (`bash .github/scripts/ci-leg.sh a11y` — green, 14 files, 85 passed), Harness / gate surface (no CI step changes)."
  - detector: D-cascade
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "Append to the line: since 2026-10-25-cold-agent-bar those tables are re-pinned to the flight booker's `flight-type` select — `tab_order`, `ENABLED_TAB_ORDER` and `controls` (14 controls, 17 rows) carrying it as `Role::ComboBox`, `stand_keyboard`'s activation table 20 → 16 rows, `stand_contrast`'s control rows 18 → 17 and `STATE_ROWS` 19 → 18, `TEXT_ROWS` 14 unchanged — a stand change, no engine crate moving, green in the leg and in three `stand-repeat` rounds — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md"
    sidecar: "a11y ci-integration: the stand's keyboard, contrast and control tables re-pinned to the flight-type select, recorded after the 0.2.0 re-read reading"
    rationale: "Report F4, F5, F7 (the tables' rows moved), Changes (re-pinned stand checks `stand_keyboard`, `stand_contrast`, `stand_accessibility_ids`; no `packages` file in the diff, entry 11), Outcome (a11y leg and `stand-repeat` 3 of 3 green); the line's «unedited and nothing re-baselined» is the 2026-10-24 re-read's reading and stands as dated."
note: "D-a11y-surface: the load page's `load-fetch` Button is a new interactive element with no Tab-order, keyboard-activation or contrast row (report Coverage of new surfaces: «no keyboard or contrast row — the page is no `LeanTask`») — §5 and §6 are group 1's categories; and the moved line citations into common/mod.rs, stand_accessibility_ids.rs, stand_snapshot.rs, stand_snapshot_state.rs and flight_booker.rs in screen-reader-support.md:17-18 · :24-31 · :44 have no new number the report states, so they need a citation-sweep re-read."
