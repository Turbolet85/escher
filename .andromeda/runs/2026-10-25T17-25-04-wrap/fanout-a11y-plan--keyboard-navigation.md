proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 38
    section: '## 5. Keyboard Navigation › keyboard-handling'
    change: 'Replace «SC 2.1.1 on the stand''s buttons is met by measurement: Enter and Space on each focused stand button read as its click (20 rows, `counter-increment`, `flight-return`, `flight-book` and `crud-create` changing the screen)» with «SC 2.1.1 on the lean tasks'' stand buttons is met by measurement: Enter and Space on each focused lean-task button read as its click (16 rows, `BUTTONS` 8, since 2026-10-25-cold-agent-bar, which retired the flight booker''s two `aria-pressed` mode buttons `flight-one-way` · `flight-return` for `select#flight-type` — 20 rows before it; `counter-increment`, `flight-book` and `crud-create` changing the screen)»; the rest of the line stands.'
    sidecar: 'keyboard-navigation.md:38 — the activation table 20 → 16 rows; `flight-return` leaves the screen-changing buttons (retired with the mode buttons); the measured SC 2.1.1 claim scoped to the lean tasks'' buttons.'
    rationale: 'Report **Retired**: the flight booker''s two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) are gone; F7 `stand_keyboard`: the activation table 20 → 16 rows (`BUTTONS` 10 → 8), site keyboard-navigation.md:38 stale; F2 `flight-mode` site keyboard-navigation.md:38 stale. The scope to the lean tasks follows Coverage of new surfaces: the load page''s button has «no keyboard or contrast row — the page is no `LeanTask`».'
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:38
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 98
    section: '## 5. Keyboard Navigation › keyboard-harness'
    change: 'Replace «the flight booker after `flight-return`, six stops: `back-btn`, `flight-one-way`, `flight-return`, `flight-start`, `flight-return-date`, `flight-book`» with «the flight booker after its return flight is chosen on `flight-type` (reached by Tab and ArrowDown), five stops: `back-btn`, `flight-type`, `flight-start`, `flight-return-date`, `flight-book` — since 2026-10-25-cold-agent-bar (its boot order `back-btn` → `flight-type` → `flight-start` → `flight-book`); before it six, through the retired `flight-one-way` · `flight-return` buttons»; the CRUD state and the rest of the line stand.'
    sidecar: 'keyboard-navigation.md:98 — the flight booker''s `ENABLED_TAB_ORDER` row restated: five stops through `flight-type`, reached by Tab and ArrowDown, the two mode buttons retired.'
    rationale: 'Report **Retired** (the two mode buttons) and F2: «the flight booker''s mode is the select `flight-type` (Tab: `back-btn` → `flight-type` → `flight-start` → `flight-book`; enabled: + `flight-return-date`, five stops, reached by Tab and ArrowDown)», site keyboard-navigation.md:98 stale.'
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:98
  - detector: D-retired
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 97
    section: '## 5. Keyboard Navigation › keyboard-harness'
    change: 'Replace «Enter and Space on each focused stand button read as its click (20 rows)» with «Enter and Space on each focused lean-task button read as its click (16 rows since 2026-10-25-cold-agent-bar, `BUTTONS` 8; 20 before the flight booker''s two mode buttons were retired)»; the rest of the line stands.'
    sidecar: 'keyboard-navigation.md:97 — `stand_keyboard`''s activation rows 20 → 16 with the mode buttons'' retirement.'
    rationale: 'F7 `activation-rows`: the activation table 20 → 16 rows (`BUTTONS` 10 → 8), site keyboard-navigation.md:97 stale — the same retirement the line-38 proposal corrects.'
    basis: .andromeda/a11y-plan-detail/keyboard-navigation.md:97
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 75
    section: '## 5. Keyboard Navigation › keyboard-shortcuts'
    change: 'Add after line 75: «- seven_guis flight booker: since 2026-10-25-cold-agent-bar its flight type is one `select#flight-type` (class `type-select`), named "Flight type" by `aria-label`, holding `option#flight-type-one-way` (value `one-way`) and `option#flight-type-return` (value `return`), each `selected:` from the `flight_type` signal; it is the second Tab stop (`back-btn` → `flight-type` → `flight-start` → `flight-book`) and, focused, the engine''s select keys move its choice (Keyboard handling above), its `oninput` writing the signal by a closed match (`"return"` → return, every other value → one-way); `:focus` drops the outline for a `#3a5ce5` border, as the date inputs do; the two `aria-pressed` mode buttons it replaces are retired (examples/seven_guis/src/tasks/flight_booker.rs:66-86; examples/seven_guis/src/tasks/flight_booker.rs:160-171) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»'
    sidecar: 'keyboard-navigation.md — the flight booker''s `select#flight-type` recorded under per-surface keyboard handling: Tab stop, select keys, closed-match `oninput`, border focus cue.'
    rationale: 'Report Symbols / APIs, the flight booker (flight_booker.rs:66-86, CSS 160-171) and F2; Coverage of new surfaces: `select#flight-type` a11y WCAG/focus/kbd✓ (Tab stop, arrows, named "Flight type", focus cue `#3a5ce5` border) — new detail for the keyboard-shortcuts group, one item.'
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 100
    section: '## 5. Keyboard Navigation › keyboard-harness'
    change: 'Add after line 100: «- Since 2026-10-25-cold-agent-bar `stand_keyboard` (15 tests) also moves the flight booker''s `flight-type` choice by its arrows in both layout modes (tests/blitz-tests/tests/stand_keyboard.rs:249-309); the shared tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order` and `ENABLED_TAB_ORDER` — the select alone, not its options, which take no focus — and the options are pinned instead by `stand_snapshot::the_flight_type_select_holds_its_two_options` (tests/blitz-tests/tests/stand_snapshot.rs:118-170) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»'
    sidecar: 'keyboard-navigation.md — `stand_keyboard`''s new select test (14 → 15 tests) and the tables'' select-alone rule recorded under the keyboard harness.'
    rationale: 'Report Symbols / APIs (new test `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`, `common` tables holding `flight-type` `Role::ComboBox`), F7 (its tests 14 → 15) and Deviations («Step 4''s tables hold the select alone … the options are pinned by `stand_snapshot::the_flight_type_select_holds_its_two_options`»); New text, by line: stand_keyboard.rs 249-309, stand_snapshot.rs 118-170.'
    fact: F7
  - detector: D-a11y-surface
    severity: warning
    file: .andromeda/a11y-plan-detail/keyboard-navigation.md
    near: 39
    section: '## 5. Keyboard Navigation › keyboard-handling'
    change: 'Add after line 39: «- Since 2026-10-25-cold-agent-bar the stand also serves two pages beside the lean four, Cells (`boot_cells`) and the load page (`boot_load`), which are no `LeanTask` and so hold no `stand_keyboard` row: the load page''s `load-fetch` button (keyed, lint 0) has no measured Enter or Space activation and no Tab-order row, so the measured SC 2.1.1 reading above covers the lean tasks'' buttons alone; Cells'' own surface is unchanged (examples/seven_guis/src/stand.rs:131-135; examples/seven_guis/src/stand.rs:141-152) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md»'
    sidecar: 'keyboard-navigation.md — the served pages'' keyboard coverage recorded: the load page''s `load-fetch` button carries no keyboard row, the SC 2.1.1 measurement scoped to the lean tasks.'
    rationale: 'D-a11y-surface: the chunk adds an interactive element, the load page''s `load-fetch` Button (Symbols / APIs), and Coverage of new surfaces states «a11y ✓ for the page''s two elements (keyed, lint 0; no keyboard or contrast row — the page is no `LeanTask`)» — §5''s keyboard coverage (an Enter / Space activation row, a Tab-order row) is absent for it, so the file records the gap instead of implying every stand button is measured.'
note: 'The `load-fetch` keyboard gap (D-a11y-surface) has no owner in the report — a route owner is the wrap''s to pin; `stand_keyboard.rs:N` citations at lines 38, 39, 62, 96-98 likely shifted (lines added at 7-15 · 45 · 208 · 214 · 220-221 · 249-310) but the report''s citation rows list none in this file, so none is proposed; D-a11y-obs-schema: no violation (no obs log schema or a11y violation schema moves).'
