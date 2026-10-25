proposals:
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 21
    section: "## 5. Integration Test Strategy"
    change: "Replace from `; `host_log` holds 3 tests and `mcp_log` 2.` to the line's end with: `; `host_log` holds 4 tests and `mcp_log` 2. Since 2026-10-25-cold-agent-bar a paged `part` read through the `escher-session` host's own sink is measured by a standing check — `host_log::a_cells_part_read_through_the_host_logs_no_id_and_no_name_at_trace`: `serve cells` at `RUST_LOG=trace`, a sentinel committed first into a part-2 cell (Cells' part-2 names at boot are empty cells and row numbers), then `snapshot --part 2`, 0 id and 0 name needles on stderr under the `-p seven_guis` and the workspace build, its needles read from the in-process Cells screen by `common::cells_needles` (examples/seven_guis/tests/host_log.rs:134-277; examples/seven_guis/tests/common/mod.rs:425-454) — the measurement its `.andromeda/residuals.md` line owned; the in-process capture reads one too (`stand_act_spans`, §1).`"
    sidecar: "integration-test-strategy.md:21 — the host-sink paged part read restated as measured by host_log's Cells test (host_log 3 → 4 tests); the four-lean-tasks reason retired"
    rationale: "Report F13 (host-sink-part) and F1 (served-tasks) list integration-test-strategy.md:21 stale; Spec claims disproved: «the host boots only the four lean tasks and none pages» is false since `serve cells`; Symbols names the new host_log test and `common::cells_needles`; Deviations: the Cells read commits a sentinel first; inputs#I4 row 1 discharges residuals.md:38 by this measurement."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:157
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 17
    section: "## 5. Integration Test Strategy"
    change: "Replace `its `selected`, `pressed` or `invalid` reading as it reads after the step — a flight-mode click both mode buttons with their pressed reading,` with `its `selected`, `pressed` or `invalid` reading as it reads after the step — a flight-type choice the select `flight-type` with its `value` and both options `flight-type-one-way` · `flight-type-return` with their `selected` reading (`a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`; since 2026-10-25-cold-agent-bar no stand control states `aria-pressed`, so no stand check returns a `pressed` reading — `accessibility_states` pins it on a fixture),`"
    sidecar: "integration-test-strategy.md:17 — stand_act_states' flight-mode click on the two retired aria-pressed mode buttons restated as the flight-type select choice and its options' selected reading"
    rationale: "Report Retired: the flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`), the stand's only `aria-pressed` controls; F2 (flight-mode) lists integration-test-strategy.md:17 stale; Symbols: the renamed test `stand_act_states::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading` → `a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading`; Deviations: `accessibility_states` still pins `aria-pressed` on a fixture; F18."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 18
    section: "## 5. Integration Test Strategy"
    change: "Replace `against the real binary, `start` · `status` · `stop` on four tasks` with `against the real binary, `start` · `status` · `stop` on the six served tasks (the four lean tasks, `cells` and `load`, since 2026-10-25-cold-agent-bar)`"
    sidecar: "integration-test-strategy.md:18 — cli_commands' start · status · stop over the binary restated on the six served tasks (was four)"
    rationale: "Report F1: `escher-session` serves six tasks (`TASKS: [(&str, Served); 6]` — counter · flight-booker · timer · crud · cells · load); Outcome Smoke: `cli_commands` starts the six slugs through the binary; cli_commands.rs is among the changed seven_guis checks. A sweep hit of the four-task claim the F13/F1 primary retires, not listed in the report's served-tasks rows."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:435
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 17
    section: "## 5. Integration Test Strategy"
    change: "Add after line 17, as its own bullet: `- **Session ↔ a served screen over the budget (Cells):** since 2026-10-25-cold-agent-bar `seven_guis::stand::boot_cells` mounts `Task::Cells` in `TaskShell` (Cells stays no `LeanTask`), and `stand_cells_served` (4 tests, both layout modes) holds it through `Session::start(\"cells\", …)` and walks it through the driver: its screen 47,949 bytes, 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, every part read by `snapshot` with `part` (`cells_is_walked_in_parts_through_the_driver`); committed cells read their values and a cell past the viewport reads in the last part; a click at a cell right after `scroll` lands on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the thumb keeps it shown, so the walk takes `advance 5000` past the fade before its click (`a_click_right_after_the_scroll_lands_on_the_shown_scrollbar`, its own test); and the lint at boot reads the measured findings (tests/blitz-tests/tests/stand_cells_served.rs:91-209; examples/seven_guis/src/stand.rs:131-139) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md`"
    sidecar: "integration-test-strategy.md — new Session ↔ Cells entry: stand_cells_served walks the over-budget Cells screen in 5 parts through the driver, the shown-scrollbar click hazard stated"
    rationale: "Report Changes: new file `stand_cells_served.rs` (209 lines), `stand::boot_cells`; Spec claims disproved: Cells 47,949 bytes, 740 lines, 5 parts; Deviations: the walk to Z25 takes `advance 5000` after the scroll and the early click is its own test; F21 new-hazards (the overlay scrollbar's thumb drag); New text lists the four tests at 91-209."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:182
    fact: F16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 17
    section: "## 5. Integration Test Strategy"
    change: "Add after line 17, after the Cells bullet, as its own bullet: `- **Session ↔ a served page holding a load (the load page):** since 2026-10-25-cold-agent-bar `seven_guis::stand::boot_load` serves the load page, titled «Load», in `TaskShell` with its options' `net_provider` the in-process stand-in `LoadNet`, which answers `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms) and does not override `is_noop`, so the harness counts its load; `stand_load_page` (2 tests, both layout modes) holds it through `Session::start(\"load\", …)`: a `click` on `load-fetch` waits out the load as an acting verb does and the screen then shows `load-done` «Loaded» (`the_load_page_click_waits_out_its_load_and_shows_the_message`), and the page reads keyed with no lint finding at boot (`the_load_page_is_keyed_and_reads_no_finding_at_boot`); every other stand boot keeps `net_provider: None` (`the_stand_options_stay_offline`) (tests/blitz-tests/tests/stand_load_page.rs:34-84; examples/seven_guis/src/stand.rs:141-175) — as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md`"
    sidecar: "integration-test-strategy.md — new Session ↔ load page entry: stand_load_page's click waits out LoadNet's 200 ms load and shows the message; every other stand boot offline"
    rationale: "Report Changes: new file `stand_load_page.rs` (84 lines), `stand::boot_load` with `LoadNet` (`impl NetProvider` 167-175, no `is_noop` override, `LOAD_ANSWER_DELAY` 200 ms), the ids `load-fetch` · `load-done`; Coverage of new surfaces: keyed, lint 0; F17 stand-offline; New text lists the two tests at 34-84."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:185
    fact: F17
note: the file's other `host_log.rs:N` cites (:20 `398-411`, :24 `30-492` · `36-40`) and :18's `cli_commands.rs:134-216` sit past this chunk's added ranges (host_log 15-17 · 134-278; cli_commands 37-41 · 137) though the report's citation rows name :23 alone — left to the P2 citation sweep; and :15's two wall-time exceptions are not extended, since `LoadNet`'s 200 ms answer thread lives in stand code and the report states no sleep or clock read in `stand_load_page` itself.
