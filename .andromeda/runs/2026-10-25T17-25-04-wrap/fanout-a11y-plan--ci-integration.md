proposals:
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: in line 10 replace "14 files and 84 tests, over the tier-0 witnesses alone, 9 files and 59 tests" with "14 files and 84 tests, over the tier-0 witnesses alone, 9 files and 59 tests, as measured at escher-0.2.0/chunks/2026-10-24-a11y-re-read/report.md — 14 files and 85 tests since 2026-10-25-cold-agent-bar, `stand_keyboard` 14 → 15 with the flight booker's `select#flight-type`"; the rest of the line unchanged
    sidecar: a11y-plan CI Integration — the a11y leg's test count stated as the 2026-10-24 reading, 85 since 2026-10-25-cold-agent-bar (F8)
    rationale: report Changes F8 — "the a11y leg 14 files, 84 → 85 tests", `a11y-84` stale at a11y ci-integration.md:10; the new `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows` is the new interactive element's keyboard coverage (Coverage of new surfaces, `select#flight-type`, `stand_keyboard` 15); Outcome gate `bash .github/scripts/ci-leg.sh a11y` — green (14 files, 85 passed)
    basis: .andromeda/a11y-plan-detail/ci-integration.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/ci-integration.md
    near: 12
    section: "## 9. CI Integration"
    change: "add after line 12, as its own continuation line (one leading space, as lines 9-12): Since 2026-10-25-cold-agent-bar the flight booker's mode is one `select#flight-type` (`ComboBox` \"Flight type\") in place of the two `aria-pressed` buttons, and the a11y leg runs the restated tables: `tab_order`, `ENABLED_TAB_ORDER` and `controls` hold `flight-type` (controls 15 → 14, `controls(task)` rows 18 → 17), `stand_keyboard` 15 (its activation table 20 → 16 rows, `BUTTONS` 10 → 8, the new `the_flight_type_select_moves_its_choice_by_its_arrows`), `stand_contrast`'s control rows 18 → 17 and `STATE_ROWS` 19 → 18 (the select `#1a1a1a` on `#ffffff`, 17.40); the leg reads 14 files · 85 passed · 0 failed, `stand-repeat: 3 of 3 rounds green`, and the pushed sha's CI#40214713710 green 17/17, with no CI step changed — the two new `stand_*` files (`stand_cells_served`, `stand_load_page`) joining `stand-repeat` by its glob, not the a11y leg — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md."
    sidecar: a11y-plan CI Integration — the a11y leg's 2026-10-25 reading recorded (the flight-type select in the restated tables; 14 files · 85; stand-repeat 3 of 3; CI#40214713710)
    rationale: report Changes F2, F4, F5, F7, F8 and Harness / gate surface ("`stand-repeat` takes the two new `stand_*` files by prefix. No CI step changes."); Outcome (a11y) acceptance "a11y 14 files · 85 · 0; stand-repeat 3 of 3"; Cross-project claims CI#40214713710 green 17/17; line 12's "unedited and nothing re-baselined" is the 2026-10-24 reading (F5 no-change at ci-integration.md:12), so the move is new detail beside it
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:141
    fact: F8
note: a11y-plan.md:196 (the flight booker's `aria-pressed` mode buttons, F2/F18 stale) and a11y-plan.md:64 (18 control rows · 19 STATE_ROWS, F5 stale) sit in the body outside this category; lines 8 and 11 hold dated readings (F8 dated at :11) and need no edit.
