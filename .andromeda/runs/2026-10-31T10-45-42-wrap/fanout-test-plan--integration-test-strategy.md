proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 8
    section: "## 5. Integration Test Strategy"
    change: "In the row's text replace everything from `is an exception to the rule` through `(§2 Differential oracle)` with: `runs each of them for incremental in [false, true] since 2026-10-31-engine-debts, under the rule pipeline tests hold, the exception this row recorded ended`; the row's closing origin gains `, the both-modes run at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`."
    sidecar: "§5 Layout assertions ↔ both layout modes: upstream's both-modes exception ended; pseudo_element_flow_classification's three tests run in both layout modes (2026-10-31-engine-debts)."
    rationale: "The report's Retired bullet names `upstream's both-modes exception`, its three tests now running `for incremental in [false, true]`; its Counts bullet lists this line as changed under `flow-class` and `no-assert-gap`, and its Outcome reads CARRY 2 met with the target in both modes."
    basis: tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 14
    section: "## 5. Integration Test Strategy"
    change: "New item after the row: `- **Script ↔ DOM, a radio's change:** since 2026-10-31-engine-debts dom.rs reads a radio's click firing input then change only where the radio was not checked before it, a click on the checked radio firing input alone, by radio_click_fires_change_only_where_it_was_not_checked; the change is the engine's, so a page's script hears one, the script runtime's own synthesis gone (packages/blitz-vibey-script/tests/dom.rs:372-400) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`"
    sidecar: "§5 Script ↔ DOM: dom.rs's radio change check added; a page's script hears one change, the engine's (2026-10-31-engine-debts)."
    rationale: "The report's F10 moves `change` on a checkbox and a radio into the engine and removes the runtime's synthesis; New text lists the dom.rs test at 372-400 and Counts moves blitz-vibey-script `dom` 30 to 31. A new item keeps the long row under the 2,000-char line limit."
    basis: packages/blitz-vibey-script/tests/dom.rs:373
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: "## 5. Integration Test Strategy"
    change: "New item after the row: `- **Session ↔ held instance, a multiple select:** since 2026-10-31-engine-debts stand_act_select reads through a held session, in both layout modes: a multiple select's name, role and rows, each row with its own bounds; a click on a row accepted and choosing it; a press with Shift spanning the rows; a type choosing its match alone with no stray click; and an option under a closed select's group refused covered (tests/blitz-tests/tests/stand_act_select.rs:262-457) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`"
    sidecar: "§5 Session ↔ held instance: stand_act_select's multiple-select and closed-group readings added (2026-10-31-engine-debts)."
    rationale: "The report's F6 and F8 add driver paths and name `stand_act_select` their check, 5 to 10; New text lists the five tests at 311-457. The Session ↔ held instance entity runs lines 20-25 near its 8 KB limit, so the fact takes an item of its own."
    basis: tests/blitz-tests/tests/stand_act_select.rs:312
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: "## 5. Integration Test Strategy"
    change: "New item after the row: `- **Session ↔ held instance, a zoomed world:** since 2026-10-31-engine-debts a control in a scrolling box under a zoomed world is reached where it shows, in both layout modes, visible_region reading the box through its transforms (stand_act_transformed; tests/blitz-tests/tests/stand_act_transformed.rs:138-176) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`"
    sidecar: "§5 Session ↔ held instance: stand_act_transformed's zoomed-world reading added (2026-10-31-engine-debts)."
    rationale: "The report's F1 reads each clipping box through its transforms with `in_view` and `crop` as callers, and Coverage of new surfaces names `stand_act_transformed`, 2 to 3; New text lists the test at 138-176."
    basis: tests/blitz-tests/tests/stand_act_transformed.rs:139
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: "## 5. Integration Test Strategy"
    change: "New item after the row: `- **Session ↔ held instance, Cells by keyboard:** since 2026-10-31-engine-debts a held session walks to a Cells cell by keyboard and commits a value read in the first part, in both layout modes — press walks and opens the editor, type supplies the characters, press commits, the driver's twelve key names holding no character (stand_cells_served; tests/blitz-tests/tests/stand_cells_served.rs:179-214) — as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md`"
    sidecar: "§5 Session ↔ held instance: stand_cells_served's keyboard walk and commit added (2026-10-31-engine-debts)."
    rationale: "The report's F13 adds the `cells-grid` keys; Coverage of new surfaces names `stand_cells_served`, 4 to 5; Deviations 3 and Spec claims disproved state the press, type, press path; New text lists the test at 179-214."
    basis: tests/blitz-tests/tests/stand_cells_served.rs:180
note: "D-tests-coverage, D-tests-framework and D-tests-obs-harness read no violation here; the report places the five new targets under test-plan §1, §2, §3, §9, §10, and it does not state what stand_load_page's new test reads, so neither has a proposal in this category."
