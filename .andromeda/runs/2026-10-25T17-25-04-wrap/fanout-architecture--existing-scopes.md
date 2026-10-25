proposals:
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 118
    section: '## Existing Scopes'
    change: 'In the seven_guis unit, replace «native-only module `stand`, the headless stand booting the lean four in TaskShell (examples/seven_guis/src/lib.rs:2-3; examples/seven_guis/src/stand.rs:1-3)» with «native-only module `stand`, the headless stand booting the lean four in TaskShell and, since 2026-10-25-cold-agent-bar, beside them Cells (`boot_cells`) and the load page (`boot_load`, titled "Load", its ids `load-fetch` and `load-done`, its options'' `net_provider` the private in-process stand-in `LoadNet`, which answers its one sheet from a thread of its own after `LOAD_ANSWER_DELAY`, 200 ms, naming no `blitz_net`), neither a `LeanTask`, every other boot `net_provider: None` (examples/seven_guis/src/lib.rs:2-3; examples/seven_guis/src/stand.rs:1-3; examples/seven_guis/src/stand.rs:131-135; examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175)»'
    sidecar: 'existing-scopes: seven_guis'' stand module boots Cells and the load page beside the lean four (`boot_cells`, `boot_load`, the in-process `LoadNet` stand-in), every other boot offline (2026-10-25-cold-agent-bar, F1 · F17)'
    rationale: 'The report''s Symbols / APIs add `seven_guis::stand::boot_cells` (stand.rs:131-135) and `boot_load` with `LOAD_ANSWER_DELAY` and the private `LoadNet` (stand.rs:141-152, 167-175), `stand::options` keeping `net_provider: None` for every other caller; Crates / modules says seven_guis'' `stand` module grows; F1 (`served-tasks`) marks existing-scopes.md:118 stale and the plan''s Expected amendments name it as a Headless-stand site.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:27-38
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 118
    section: '## Existing Scopes'
    change: 'In the same unit, replace «a second binary, `escher-session` — the driver''s command line over the four lean tasks» with «a second binary, `escher-session` — the driver''s command line over six served tasks, `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (its private `TASKS` table over `enum Served { Lean(LeanTask), Cells, Load }`, the usage line''s `apps:` and the MCP `start` tool''s enum following; examples/seven_guis/src/session_host.rs:12-27)», and «the one place a lean task, the stand''s boot and the timer''s tick handle meet a driver `Session`» with «the one place a served task, the stand''s boot and the timer''s tick handle meet a driver `Session`»'
    sidecar: 'existing-scopes: `escher-session` serves six tasks — the lean four, `cells` and `load` (2026-10-25-cold-agent-bar, F1)'
    rationale: 'The report''s Symbols / APIs: `escher-session` (session_host.rs:12-27) holds `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]`; F1 marks existing-scopes.md:118 stale (the four lean tasks → six served tasks).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:49-51
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: '## Existing Scopes'
    change: 'In the blitz-tests unit, after «… a double click selecting the whole value (tests/blitz-tests/tests/password_caret_click.rs:83-121);» insert «since 2026-10-25-cold-agent-bar two served-page checks, each auto-discovered, holding a `Session` and declaring no `mod fixtures;`, run in `stand-repeat` by the `stand_` prefix — `stand_cells_served`, 4 tests in both layout modes: Cells (`boot_cells`) walked in parts through the driver, committed cells reading their values and a cell past the viewport reading in the last part, a click right after the scroll landing on the shown overlay scrollbar, and the lint at boot reading its measured findings (tests/blitz-tests/tests/stand_cells_served.rs:91-133; tests/blitz-tests/tests/stand_cells_served.rs:135-174; tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:194-209) — and `stand_load_page`, 2 tests in both layout modes: the load page''s click waiting out its load and showing `load-done`, and the page keyed and reading no finding at boot (tests/blitz-tests/tests/stand_load_page.rs:34-68; tests/blitz-tests/tests/stand_load_page.rs:70-84);»'
    sidecar: 'existing-scopes: blitz-tests gains `stand_cells_served` (4) and `stand_load_page` (2), the served-page checks (2026-10-25-cold-agent-bar)'
    rationale: 'The report''s Files → new lists `tests/blitz-tests/tests/stand_cells_served.rs` (209 lines) and `stand_load_page.rs` (84 lines); Outcome reads «`stand_cells_served` and `stand_load_page` green in both layout modes: MET (4 · 2)»; F10 says the two new files declare no `mod fixtures;`; Harness / gate surface says `stand-repeat` takes them by prefix; the test heads and ranges are New text, by line (stand_cells_served.rs 91-133, 135-174, 176-192, 194-209; stand_load_page.rs 34-68, 70-84).'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:13-14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 115
    section: '## Existing Scopes'
    change: 'In the session_common sentence, replace «and by `stand_replay` — the readers counted below —» with «and by `stand_replay` and, since 2026-10-25-cold-agent-bar, by `stand_cells_served` and `stand_load_page` — the readers counted below, as measured before them —»'
    sidecar: 'existing-scopes: session_common''s readers gain `stand_cells_served` and `stand_load_page` (2026-10-25-cold-agent-bar)'
    rationale: 'New text, by line shows both new files holding a `Session` through the shared module — stand_cells_served.rs:19-21 «use session_common::{» and :33-37 `Session::start("cells", …)`; stand_load_page.rs:19-23 `Session::start("load", …)` and :25-32 `session.run(&snapshot())`, the `snapshot` builder session_common holds; line 134''s count (29) is dated to 2026-10-20-large-screen-in-parts and stays as measured.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:549
    fact: F1
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: '## Existing Scopes'
    change: 'Replace «`ENABLED_TAB_ORDER` (the flight booker after `flight-return`, CRUD after a row is selected)» with «`ENABLED_TAB_ORDER` (the flight booker with the return flight chosen on its select `flight-type`, CRUD after a row is selected) — since 2026-10-25-cold-agent-bar the tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order`, `ENABLED_TAB_ORDER` and `rendered`, in place of the retired mode buttons `flight-one-way` and `flight-return`»'
    sidecar: 'existing-scopes: the stand tables hold the flight booker''s select `flight-type` in place of the retired `flight-one-way` / `flight-return` buttons (2026-10-25-cold-agent-bar, F2)'
    rationale: 'The report''s Retired bullet names the flight booker''s two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`); Symbols / APIs: blitz-tests `common` tables hold `flight-type` (`Role::ComboBox`) in `controls`, `tab_order`, `ENABLED_TAB_ORDER`, `rendered`; F2 (`flight-mode`) marks existing-scopes.md:106 stale.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91-93
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: '## Existing Scopes'
    change: 'Replace «`stand_keyboard`, 14 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added the last three and put it in the `a11y` CI leg)» with «`stand_keyboard`, 15 tests in both layout modes (14 until 2026-10-25-cold-agent-bar, which added the flight booker''s select `flight-type` moving its choice by its arrows (tests/blitz-tests/tests/stand_keyboard.rs:249-309); 11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg)», and «Enter and Space on each focused stand button read as its click, 20 rows» with «Enter and Space on each focused stand button read as its click, 16 rows (`BUTTONS` 8; 20 rows and 10 buttons until 2026-10-25-cold-agent-bar retired the two mode buttons)»'
    sidecar: 'existing-scopes: `stand_keyboard` 14 → 15 tests, its activation table 20 → 16 rows (2026-10-25-cold-agent-bar, F7)'
    rationale: 'F7 (`activation-rows`): «`stand_keyboard`: the activation table 20 → 16 rows (`BUTTONS` 10 → 8), its tests 14 → 15», existing-scopes.md:106 stale; Symbols / APIs names the new test `stand_keyboard::the_flight_type_select_moves_its_choice_by_its_arrows`, at stand_keyboard.rs 249-309 in New text, by line.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:138-140
    fact: F7
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 107
    section: '## Existing Scopes'
    change: 'Replace «the walk beyond boot — the flight booker after `flight-return` (six stops)» with «the walk beyond boot — the flight booker with the return flight chosen on `flight-type` (five stops, `flight-return-date` joining the boot walk''s four, reached by Tab and ArrowDown; six stops after the `flight-return` button until 2026-10-25-cold-agent-bar retired it)»'
    sidecar: 'existing-scopes: the flight booker''s enabled walk is five stops after the return flight is chosen on `flight-type` (2026-10-25-cold-agent-bar, F2)'
    rationale: 'Retired: the `flight-return` button; F2 (`flight-mode`): «the flight booker''s mode is the select `flight-type` (Tab: `back-btn` → `flight-type` → `flight-start` → `flight-book`; enabled: + `flight-return-date`, five stops, reached by Tab and ArrowDown)», existing-scopes.md:107 stale.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:114-119
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: '## Existing Scopes'
    change: 'Replace «the five former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` now `#ffffff` on `#3a5ce5` at 5.47» with «the five former fails `counter-increment`, `flight-one-way`, `flight-book`, `timer-reset` and `crud-create` — the four that stand now `#ffffff` on `#3a5ce5` at 5.47, `flight-one-way` retired at 2026-10-25-cold-agent-bar with the flight booker''s mode buttons, its place taken by the select `flight-type`, `#1a1a1a` on `#ffffff` at 17.40, 15px 400»'
    sidecar: 'existing-scopes: `stand_contrast`''s `flight-one-way` accent row retired, `flight-type` at 17.40 in its place (2026-10-25-cold-agent-bar, F2 · F5)'
    rationale: 'Retired: `flight-one-way` and its CSS; F5: «the `flight-one-way` accent row (white on `#3a5ce5`, 5.47, 14px 600) and the `flight-return` row (`#444444` on `#fafafa`, 9.33) give way to `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400)»; F2 marks existing-scopes.md:112 stale.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125-130
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: '## Existing Scopes'
    change: 'Replace «the 18 `controls(task)` rows over the four lean tasks at boot each read through `Harness::text_contrast` against a stated record, pass 14 · fail 0 · exempt-inactive 3 · no-text 1» with «the 17 `controls(task)` rows over the four lean tasks at boot each read through `Harness::text_contrast` against a stated record, pass 13 · fail 0 · exempt-inactive 3 · no-text 1 (18 rows, pass 14, until 2026-10-25-cold-agent-bar)», and «(`STATE_ROWS`, 19: 18 pass, 1 exempt)» with «(`STATE_ROWS`, 18: 17 pass, 1 exempt — 19 · 18 until 2026-10-25-cold-agent-bar; the return-chosen rows `flight-type` and `flight-return-date` each 17.40)»'
    sidecar: 'existing-scopes: `stand_contrast` control rows 18 → 17 (13 · 0 · 3 · 1), `STATE_ROWS` 19 → 18 (17 · 0 · 1 · 0) (2026-10-25-cold-agent-bar, F5)'
    rationale: 'F5 (`contrast-rows`): «control rows 18 → 17 (tally 14 · 0 · 3 · 1 → 13 · 0 · 3 · 1), `STATE_ROWS` 19 → 18 (tally 18 · 0 · 1 · 0 → 17 · 0 · 1 · 0)», return-chosen rows `flight-type` 17.40 and `flight-return-date` 17.40; existing-scopes.md:112 stale.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:125-132
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 114
    section: '## Existing Scopes'
    change: 'Replace «`WITNESSES_020`, the 0.2.0 record, nine rows and 46 witnesses» with «`WITNESSES_020`, the 0.2.0 record, twelve rows and 49 witnesses (nine and 46 until 2026-10-25-cold-agent-bar, which added the v020-07 · v020-08 · v020-09 rows, one witness each — `stand_load_page`, `stand_act_states`'' flight-type choice, `stand_cells_served`'' walk in parts — beside a `Subject::Served` for a page served beside the lean four, and re-pointed v010-17''s flight witness and v020-05''s lint witness to the renamed tests)»'
    sidecar: 'existing-scopes: `WITNESSES_020` 9 → 12 rows, 46 → 49 witnesses, the `Served` subject (2026-10-25-cold-agent-bar, F20)'
    rationale: 'F20 (`witnesses`): «`WITNESSES_020` gains three `Served` / flight rows and the `Served` subject», existing-scopes.md:114 stale; Symbols / APIs lists the three rows each with one witness test (v020-07 `stand_load_page.rs::the_load_page_click_waits_out_its_load_and_shows_the_message`, v020-08 `stand_act_states.rs::a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`, v020-09 `stand_cells_served.rs::cells_is_walked_in_parts_through_the_driver`) and the two re-pointed witnesses, which leave the count unmoved — so 46 + 3 = 49.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:56-60
    fact: F20
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 109
    section: '## Existing Scopes'
    change: 'Replace «a step that changes a control''s pressed, selected or invalid reading returning that node as it reads after the step» with «a step that changes a control''s selected or invalid reading returning that node as it reads after the step — since 2026-10-25-cold-agent-bar the flight booker''s choice on its select `flight-type` returning its value and both options'' `selected` reading, the stand stating no `aria-pressed` once its two mode buttons were retired»'
    sidecar: 'existing-scopes: `stand_act_states` reads the flight booker''s select choice (`selected`) in place of the retired mode buttons'' `pressed` reading (2026-10-25-cold-agent-bar, F18)'
    rationale: 'Retired: the flight booker''s two `aria-pressed` mode buttons, «the stand''s only `aria-pressed` controls»; Symbols / APIs renames `stand_act_states::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` → `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`; F18 (`stand-pressed`): «the stand no longer states `aria-pressed` anywhere» — this passage, citing stand_act_states, still describes a stand step changing a pressed reading.'
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:190-193
    dependent-of: D-retired
note: D-arch-decisions reads clean (Dependencies none; LoadNet a hand-written NetProvider over blitz-traits, already a seven_guis native dependency); the citations at lines 106-107, 112 and 114 into stand_keyboard.rs, common/mod.rs, stand_contrast.rs and stand_requirement_sweep.rs sit in files this chunk edited (New text, by line) but no report row dispositions them, so their ranges are left for the P2 citation sweep; line 110's `stand_act_ids.rs:1-16` already matches the report's «its doc now 1-16»; line 134's 21 / 29 / 24 reader counts are dated to 2026-10-20-large-screen-in-parts and stay.
