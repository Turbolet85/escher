proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 12
    section: '## 6. Visual Design Verification › values · visual-contrast'
    change: 'Restate the boot record and STATE_ROWS without the two retired mode buttons: the record reads pass 13 (`back-btn` ×4, `#333333` on `#f0f0f0`, 11.09; `flight-type`, the flight booker''s select, `#1a1a1a` on `#ffffff` at 15px 400, 17.40; `flight-start` 17.40; `crud-filter`, `crud-name` and `crud-surname`, `#000000` on `#ffffff`, 21.00; and the four accent controls `counter-increment`, `flight-book`, `timer-reset` and `crud-create`, each `#ffffff` on `#3a5ce5` at 5.47), fail 0, exempt-inactive 3 (unchanged), no-text 1 (`timer-duration`) — since 2026-10-25-cold-agent-bar, when the select replaced the `aria-pressed` buttons `flight-one-way` (accent, 5.47, the active toggle at boot) and `flight-return` (`#444444` on `#fafafa`, 9.33), whose rows left the record; until 2026-10-14 the then five accent controls read `#ffffff` on `#4a6cf7` at 4.39, failing; the painted pixel matches the measured background within 1 per channel on all 16 measured rows; `STATE_ROWS`, 18 colour states — return-chosen 2 (`flight-type` 17.40, `flight-return-date` 17.40; was 3, with `flight-return` 5.47 and `flight-one-way` 9.33), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — pass 17 and exempt 1 (`flight-book` disabled, 2.26), the lowest pass 5.11 (`flight-start` invalid); TEXT_ROWS 14 unchanged.'
    sidecar: 'visual-design-verification.md:12 — stand_contrast record 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1 and STATE_ROWS 19 → 18 (return-chosen 3 → 2): the retired flight-one-way / flight-return rows give way to flight-type, #1a1a1a on #ffffff, 17.40 (2026-10-25-cold-agent-bar, F5).'
    rationale: 'The report''s Retired bullet names the flight booker''s two aria-pressed mode buttons flight-one-way and flight-return; line 12 still records both as standing rows (and flight-one-way as an accent control). F5 states the moved tallies (control rows 18 → 17, 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1; STATE_ROWS 19 → 18, 18 · 0 · 1 · 0 → 17 · 0 · 1 · 0), the select''s pair 17.40 at 15px 400 and the return-chosen rows; F5 lists visual-design-verification.md:12 stale.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 11
    section: '## 6. Visual Design Verification › values · visual-contrast'
    change: 'End the sentence on the stand check''s row count as measured now: ''— 17 rows (18 until 2026-10-25-cold-agent-bar, when the flight booker''''s two mode buttons gave way to one select, `flight-type`)''; the four-lean-task scope at boot stays.'
    sidecar: 'visual-design-verification.md:11 — stand_contrast''s control rows 18 → 17 (2026-10-25-cold-agent-bar, F5).'
    rationale: 'A second occurrence of the retired buttons'' count in this file: F5 moves stand_contrast''s control rows 18 → 17 (F4: controls 15 → 14, controls(task) rows 18 → 17); F1 keeps line 11''s LeanTask scope no-change, but its trailing ''18 rows'' is F5''s moved fact.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:123
    dependent-of: D-retired
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: '## 6. Visual Design Verification › values · visual-contrast'
    change: 'Keep the user-agent select''s box, border, padding and background (default.css cites unchanged) and replace the clause after it: ''the user-agent pair is not measured — the stand''''s one select, the flight booker''''s `flight-type` (since 2026-10-25-cold-agent-bar), carries author CSS (`.type-select`: 15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10) and reads 17.40 in `stand_contrast`''''s control row and return-chosen state row (examples/seven_guis/src/tasks/flight_booker.rs:160-171 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)''.'
    sidecar: 'visual-design-verification.md:15 — ''the stand holds no select'' retired: flight-type, author-styled, measured 17.40 in stand_contrast (2026-10-25-cold-agent-bar, F2 · F5).'
    rationale: 'The chunk adds an interactive UI element, select#flight-type, whose §6 contrast coverage is measured (Coverage of new surfaces: a11y WCAG/focus/kbd ✓, 17.40); line 15 still says the stand holds no select and stand_contrast''s tables are unmoved — both false now (F2, F5; Symbols / APIs: its CSS at flight_booker.rs:160-171).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:304
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 14
    section: '## 6. Visual Design Verification › values · visual-contrast'
    change: 'Keep the contrast reading and the planted row; restate the boot claim and the leg count: ''The stand at boot reads no `contrast` finding on the four lean tasks in both layout modes, the nodes outside `stand_contrast`''''s rows included; since 2026-10-25-cold-agent-bar its one finding at boot is the flight booker''''s `overlap` of `flight-type-one-way` × `flight-type-return` (the snapshot gives each option its select''''s client rect), the other three tasks 0, `[0, 1, 0, 0]` — re-pinned as measured on the founder''''s «Перезакрепить как измерено (Рекомендую)», the lint''''s reading of a select''''s options unresolved (`stand_lint::the_stand_at_boot_reads_its_measured_findings`)''; and ''the `a11y` leg reads 85 tests over 14 targets (84 until 2026-10-25-cold-agent-bar, `stand_keyboard`''''s select test the 85th), `stand_contrast`''''s 5 among them, unmoved''; add ''as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md'' beside the existing cites.'
    sidecar: 'visual-design-verification.md:14 — the lean stand''s lint at boot reads one overlap (flight booker, the select''s two options), no contrast finding; the a11y leg 84 → 85 tests (2026-10-25-cold-agent-bar, F3 · F8).'
    rationale: 'Another occurrence of the select''s coverage in this file: F3 lists visual-design-verification.md:14 stale for lint-at-boot (the flight booker reads one overlap, the other three 0; Spec claims disproved: a 0-finding lean stand is false), and F8 lists it stale for a11y-84 (14 files, 84 → 85). F1 marks this line for re-read: its scope is stand_lint''s four lean tasks, which stands (served tasks no-change here). inputs#I5 carries the founder''s word.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:120
    dependent-of: D-a11y-surface
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 15
    section: '## 6. Visual Design Verification › values · visual-contrast'
    change: '- Since 2026-10-25-cold-agent-bar the stand serves the load page beside the lean four (`stand::boot_load`, no `LeanTask`), so it holds no `stand_contrast` row: its button `load-fetch` declares the stand accent — white on `#3a5ce5`, hover `#2a4cd3`, active `#1f3fc1`, the pairs this section records at 5.47 · 6.83 · 8.27 — and its message `load-done` `#1a1a1a`; its lint at boot, whose `contrast` reading is `text_contrast`''s, reads no finding in both layout modes (tests/blitz-tests/tests/stand_load_page.rs:70-84 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'visual-design-verification.md — new record: the served load page''s colours (the stand accent) and its lint at boot, 0 findings, outside stand_contrast''s rows (2026-10-25-cold-agent-bar, F1).'
    rationale: 'The chunk adds an interactive element outside the lean four, the load page''s `load-fetch` button (Symbols / APIs: the stand accent #3a5ce5 / white, hover #2a4cd3, active #1f3fc1; Coverage of new surfaces: a11y ✓ for its two elements, keyed, lint 0, no keyboard or contrast row — the page is no LeanTask; tokens #3a5ce5 / #2a4cd3 / #1f3fc1, #1a1a1a, #f5f5f5); this category records the stand''s contrast coverage and its scope, so where the new page''s colours stand belongs here.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:300
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/visual-design-verification.md
    near: 31
    section: '## 6. Visual Design Verification › values · visual-focus-ring'
    change: '- Since 2026-10-25-cold-agent-bar the flight booker''s select `flight-type` drops the outline on `:focus` for a `#3a5ce5` border, as its date inputs do (examples/seven_guis/src/tasks/flight_booker.rs:160-171 — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)'
    sidecar: 'visual-design-verification.md — new focus-ring record: flight-type''s :focus cue, a #3a5ce5 border in place of the outline (2026-10-25-cold-agent-bar, F2).'
    rationale: 'Symbols / APIs: the select''s CSS (flight_booker.rs:160-171), whose `:focus` drops the outline for the #3a5ce5 border as the date inputs do; Coverage of new surfaces names that focus cue for select#flight-type. The focus-ring group records each stand control''s focus cue.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:45
    fact: F2
note: 'Two citations in this file sit in files the chunk edited but are in no citation row of the report — line 31''s examples/seven_guis/src/tasks/flight_booker.rs:183-186 (the file gained 66-86 and 160-171 and lost the mode buttons'' CSS) and line 14''s tests/blitz-tests/tests/stand_lint.rs:174-314 · :415-442 (lines added at 7-9, 277-311, 428-434) — for the P2 citation sweep to re-read; D-a11y-obs-schema fires nothing (no a11y violation or obs log schema moves).'
