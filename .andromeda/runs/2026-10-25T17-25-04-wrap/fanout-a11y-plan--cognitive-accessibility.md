proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 9
    section: '## 8. Cognitive Accessibility'
    change: 'In the tab_order clause, after "the three other tasks'' unchanged" add: "; since 2026-10-25-cold-agent-bar the flight booker''s restated to four stops, `back-btn`, `flight-type`, `flight-start`, `flight-book` — the select `flight-type` (`ComboBox` "Flight type") in place of the two retired mode buttons `flight-one-way` · `flight-return` — and, with the return flight chosen, `ENABLED_TAB_ORDER` adds `flight-return-date`, five stops, reached by Tab and ArrowDown"'
    sidecar: 'cognitive-accessibility.md:9 — the shared tab_order clause names the flight booker''s sequence restated around the select `flight-type` (2026-10-25-cold-agent-bar, F2), no longer read as unchanged since 2026-10-13'
    rationale: 'The report''s Changes (F2, report.md:114-119; Symbols / APIs, report.md:41-53) restates the flight booker''s Tab order in the shared `tab_order` / `ENABLED_TAB_ORDER` tables around the new select, and its Retired bullet (report.md:91-93) drops the two mode buttons; this line still reads the flight booker''s sequence as unchanged since 2026-10-13-stand-keyboard-harness. The `flight-mode` sweep did not reach it because the line names no old id.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:114
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/cognitive-accessibility.md
    near: 18
    section: '## 8. Cognitive Accessibility'
    change: 'Add after line 18 (cognitive-orientation): "- Since 2026-10-25-cold-agent-bar the stand''s served load page (`seven_guis::stand::boot_load`, `escher-session start load`) shows its status in text: the paragraph `load-done` reads "Loaded", `display: none` until the page''s sheet lands after the Button "Load" (`load-fetch`) is clicked, its in-process stand-in answering after `LOAD_ANSWER_DELAY` (200 ms); the page is keyed and reads no lint finding at boot, no keyboard or contrast row (it is no `LeanTask`) (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:185-200; tests/blitz-tests/tests/stand_load_page.rs:34-84)"'
    sidecar: 'cognitive-accessibility.md — cognitive-orientation gains the served load page''s text status cue `load-done` "Loaded" (2026-10-25-cold-agent-bar)'
    rationale: 'The report''s Symbols / APIs (report.md:30-40) adds the load page with a "Loaded" message shown once its load lands, and its Coverage of new surfaces (report.md:300-303) records a11y for its two elements (keyed, lint 0, no keyboard or contrast row) — a status cue new to the stand that the orientation group does not hold; line numbers are the report''s New text, by line (stand.rs 141-152, 185-200; stand_load_page.rs 34-68, 70-84).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:39
note: 'cognitive-accessibility.md:9 cites flight_booker.rs:90-110 · :93 · :101 · :194-198, stand_snapshot_state.rs:461-496, stand_snapshot.rs:262-315 and tests/common/mod.rs:47-63 in files this chunk edited above those lines (report New text, by line: flight_booker.rs 66-86 · 160-171, stand_snapshot_state.rs 121-131 …, stand_snapshot.rs 118-170, common/mod.rs 25 · 52), yet the report''s citation sweep lists no site in this file — for the P2 citation sweep to re-read; D-a11y-obs-schema not fired (no a11y violation or obs log schema moves, report.md:87-89).'
