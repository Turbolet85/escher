proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 44
    section: '## 7. Screen Reader Support'
    change: 'Restate the item''s opening as: on the stand, since 2026-10-10-snapshot-states, controls state a role or a state by attribute alone, with no element, class, style, key, id or order moving — the CRUD list `role="listbox"` with each person row `role="option"` carrying `aria-selected`, reading `ListBox` / `ListBoxOption` in the tree, and the flight booker''s two date inputs `aria-invalid`; the flight booker''s two mode buttons (`flight-one-way`, `flight-return`) stated `aria-pressed` until 2026-10-25-cold-agent-bar, which removed them and their CSS (`.flight-type-row`, `.type-btn`, `.type-btn-active`) for the native select `flight-type` (next item), so the stand states no `aria-pressed` anywhere and `accessibility_states` alone pins it, on a fixture; in its cites replace examples/seven_guis/src/tasks/flight_booker.rs:69 and :76 by examples/seven_guis/src/tasks/flight_booker.rs:66-86; the rest of the item (the actionable-key check, the `aria-activedescendant` gap, the list''s name) unchanged'
    sidecar: 'screen-reader-support §7 sr-stand-states: the flight booker''s aria-pressed mode buttons retired for the select flight-type; the stand states no aria-pressed'
    rationale: 'The report''s Retired bullet names the two aria-pressed mode buttons and their CSS, the stand''s only aria-pressed controls; F18 (stand-pressed) and F2 (flight-mode) each list screen-reader-support.md:44 stale; the select stands at flight_booker.rs:66-86 (Symbols / APIs).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 25
    section: '## 7. Screen Reader Support'
    change: 'Replace «exactly the active flight mode reads pressed» with «exactly the chosen flight type reads selected — since 2026-10-25-cold-agent-bar the select `flight-type`, its chosen option reading `selected` and the other `unselected` (`exactly_the_chosen_flight_type_reads_selected`, renamed from `exactly_the_active_flight_mode_reads_pressed`), no stand node stating `pressed`, that reading pinned on a fixture by `accessibility_states`»; the rest of the item unchanged'
    sidecar: 'screen-reader-support §7 sr-snapshot: the stand''s pressed witness retired; the chosen flight type reads selected'
    rationale: 'F18 lists screen-reader-support.md:25 stale (the stand no longer states aria-pressed); the report renames stand_snapshot_state::exactly_the_active_flight_mode_reads_pressed to exactly_the_chosen_flight_type_reads_selected and its Deviations say v010-17''s flight witness moves to selected while accessibility_states still pins aria-pressed on a fixture.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:63
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 44
    section: '## 7. Screen Reader Support'
    change: 'Add after line 44, in the sr-stand-states group: - The flight booker''s mode is one native `select#flight-type` (`aria-label` "Flight type") holding `option#flight-type-one-way` (value `one-way`, "one-way flight") and `option#flight-type-return` (value `return`, "return flight"), each `selected` from the flight-type signal, so it states its choice by no ARIA attribute: the tree and the snapshot read `ComboBox` "Flight type", value `one-way` at boot, and two `ListBoxOption`s named by their text, the chosen one `selected` and the other `unselected`, each at the select''s client rect, in both layout modes; ArrowDown on the focused select and the driver''s `type flight-type return` choose the return flight, `value` and both options'' `selected` moving; because both options carry the select''s client rect the driver lint reads them as one `overlap` (`flight-type-one-way` × `flight-type-return`), a driver defect left open, `stand_lint` re-pinned as measured on the founder''s «Перезакрепить как измерено (Рекомендую)» (examples/seven_guis/src/tasks/flight_booker.rs:66-86; tests/blitz-tests/tests/stand_snapshot.rs:118-170; tests/blitz-tests/tests/stand_act_states.rs:95-108; tests/blitz-tests/tests/stand_keyboard.rs:249-309) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md'
    sidecar: 'screen-reader-support §7 sr-stand-states: the stand''s select flight-type recorded — ComboBox "Flight type", two ListBoxOptions by selected, the lint''s measured overlap'
    rationale: 'The report adds select#flight-type (Symbols / APIs: flight_booker.rs:66-86, read as ComboBox "Flight type" with value one-way and two ListBoxOptions selected / unselected at the select''s client rect), its coverage a11y WCAG/focus/kbd ✓ (Coverage of new surfaces), new tests stand_snapshot::the_flight_type_select_holds_its_two_options and stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows, and F3 / F21 the lint''s overlap of the two options.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:41
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 38
    section: '## 7. Screen Reader Support'
    change: 'Append to the actionable-key item, after «…at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md»: ; since 2026-10-25-cold-agent-bar `escher-session` serves six tasks, Cells and a load page beside the lean four, and the check returns nothing on all six — the load page''s two elements reading their keys, `load-fetch` (Button "Load") and `load-done` (Paragraph "Loaded", shown once its sheet lands), in both layout modes (tests/blitz-tests/tests/stand_load_page.rs:70-84), and the flight booker''s select and its two options their `id`s `flight-type`, `flight-type-one-way` and `flight-type-return` — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md'
    sidecar: 'screen-reader-support §7 sr-actionable-keys: unkeyed_actionable empty on the six served tasks; the load page''s two elements and the flight-type select keyed'
    rationale: 'F1 lists screen-reader-support.md:37 stale and :38 for re-read (six served tasks); the Outcome reads `unkeyed_actionable` empty on the six served tasks, MET, Cells and the load page by their own checks; the load page''s ids load-fetch and load-done (Symbols / APIs) and stand_load_page::the_load_page_is_keyed_and_reads_no_finding_at_boot at 70-84 (New text, by line).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:384
    fact: F1
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/screen-reader-support.md
    near: 17
    section: '## 7. Screen Reader Support'
    change: 'Replace «on the stand the 15 interactive controls carry an HTML-AAM role and a non-empty accessible name» with «on the stand the 14 interactive controls (15 until 2026-10-25-cold-agent-bar, whose select `flight-type`, `ComboBox` named "Flight type" by `aria-label`, took the place of the flight booker''s two mode buttons, `stand_accessibility_ids` re-pinned) carry an HTML-AAM role and a non-empty accessible name»; the rest of the item unchanged'
    sidecar: 'screen-reader-support §7 sr-author-id: the stand''s named controls 15 → 14, the select flight-type in place of the two mode buttons'
    rationale: 'F4 moves the stand''s controls 15 → 14; the scope record''s companion stand_accessibility_ids.rs pinned the stand''s 15 controls (`checked.len() == 15`) and went red at the first gate firing, re-pinned (added line 181); the new select is named "Flight type" by aria-label (Symbols / APIs). The sweep''s controls-15 pattern did not reach this line''s wording.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:123
note: 'Citations in this file shifted by the chunk''s edits with no new number in the report — stand_snapshot_state.rs:367-407 · :409-459 · :461-496 (lines 25, 44), tests/blitz-tests/tests/common/mod.rs:17-123 · :125-128 · :132-168 and stand_accessibility_ids.rs:255-308 · :310-352 (line 17), flight_booker.rs:93 · :101 (line 44) — are left to the P2 citation sweep; the load page''s `load-fetch` has no keyboard or contrast row (no LeanTask), a §5 / §6 question outside this category; D-a11y-obs-schema finds no change (no a11y violation or obs log schema moved).'
