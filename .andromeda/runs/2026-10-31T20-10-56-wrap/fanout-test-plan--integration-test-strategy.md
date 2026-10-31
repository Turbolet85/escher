proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 30
    section: "## 5. Integration Test Strategy"
    change: "after the clause of the two readings, ending at `stand_act_obstructed`'s sixth test and its citations, and before its closing semicolon, add: «; since 2026-10-31-select-popup `covered`'s fixed meaning also gains a clause that is neither reading, «an option of a closed select lies under it: click the select first, then the option», the cause's name, remedy and order unchanged — an option of a closed select stays refused `covered` with nothing changed (`stand_act_popup`'s `a_closed_option_stays_refused_covered_and_changes_nothing`) and a button under an open list reads `covered` (`a_button_under_the_list_is_refused_covered`), each in both layout modes»"
    sidecar: "integration-test-strategy §5: `covered`'s fixed meaning gains the closed-option clause, read by `stand_act_popup`."
    rationale: "Report Counts `covered-text` lists integration-test-strategy:30 as changed; Symbols `refusal.rs` states the clause, name, remedy and order unchanged; New text lists both tests of `stand_act_popup`."
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:30
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 31
    section: "## 5. Integration Test Strategy"
    change: "«(the fifteen `stand_act_*` files, §1)» reads «(the sixteen `stand_act_*` files, §1)»; nothing else on the line moves"
    sidecar: "integration-test-strategy §5: `stand_act_*` files fifteen to sixteen, `stand_act_popup`."
    rationale: "Report Counts `act-files`, `stand_act_*` files fifteen to sixteen, lists integration-test-strategy:31 as changed; `type-focus` keeps the line's `type` reading as no-change."
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 32
    section: "## 5. Integration Test Strategy"
    change: "new record after this line: «- **Session host ↔ session socket, a select's list:** since 2026-10-31-select-popup `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in the state directory `cc-popup` under `CARGO_TARGET_TMPDIR`, opens the list through a held host, reads its shot equal to the in-process one and chooses a row by a click (examples/seven_guis/tests/cli_commands.rs:419-464) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `cli_commands`' hosted list and shot."
    rationale: "Report Coverage, the list's pixels on the screenshot carriers: `cli_commands` hosted shot equal to in-process; Symbols names `cc-popup` (`cli_commands.rs:442`); New text lists the test at 419-464."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 35
    section: "## 5. Integration Test Strategy"
    change: "«`host_log` holds 4 tests and `mcp_log` 2» reads «`host_log` holds 5 tests and `mcp_log` 3»; `host_log`'s fourth test stays the fourth, the new one standing after it"
    sidecar: "integration-test-strategy §5: `host_log` 4 to 5 tests, `mcp_log` 2 to 3."
    rationale: "Report Coverage, select list: process (`cli_commands`, `host_log`, `mcp_log` +1 each); New text lists one new test in each file, `host_log`'s at 282-393. No Counts row carries this wording, so the pre-pass did not reach it."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 38
    section: "## 5. Integration Test Strategy"
    change: "new record after this line: «- **Session host ↔ what it writes, a select's list:** since 2026-10-31-select-popup `host_log`'s `cfg(unix)` `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in the state directory `hl-popup` under `CARGO_TARGET_TMPDIR`, and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace` open a list and choose a row at `RUST_LOG=trace` and read 0 needles on stderr, the option needles from the shared `common::option_needles()`; `mcp_log`'s test records nothing and no `ml-popup` directory exists, recording there needing a `cfg!(unix)` site; `stand_act_spans` still reads the span's eight fields (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; examples/seven_guis/tests/common/mod.rs:425-453) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, the list's option needles read at `trace` on the host and the `mcp` role."
    rationale: "Report Outcome (security): `host_log`, `mcp_log` 0 needles at `trace`, `stand_act_spans` eight fields; Reverted: no `ml-popup`; Deviations: `#[cfg(unix)]` on the new `host_log` test; Symbols: `hl-popup` (`host_log.rs:296`)."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record after this line, first of three: «- **Session ↔ held instance, a single select's list:** since 2026-10-31-select-popup `stand_act_popup` (12) drives the list through the driver, in both layout modes — a `click` opening it and a `click` on a row choosing, keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows' `bounds` back at the select's box, two `click`s on one point opening then closing, each focusing, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open and choosing by its text, a shot of a row cropping to it with the list left open, the lint reading no `overlap` on a row with the planted pair still read, the open screen under its budget with every actionable element keyed, and a screen read after a step reading the new state with no `advance`; on the stand `stand_lint`'s `an_open_lists_rows_read_no_overlap_on_the_stand` (+1) and `stand_keyboard`'s `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it` (+2); through `paint_scene` alone, the windowed list read and not run (tests/blitz-tests/tests/stand_act_popup.rs:58-423; tests/blitz-tests/tests/stand_lint.rs:493-519; tests/blitz-tests/tests/stand_keyboard.rs:1082-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `stand_act_popup` (12), `stand_lint` +1, `stand_keyboard` +2 over the list."
    rationale: "Report Coverage, select list: integ `stand_act_popup` 12, `stand_keyboard` +2, `stand_lint` +1; Outcome `v030-17` and the (arch), (design), (layouts) and (lint) rows; New text lists every test named."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record, second of three after line 44: «- **Harness ↔ event pipeline, a single select's list:** since 2026-10-31-select-popup `select_popup` (25) reads the engine's list through harness input, in both layout modes — a click, Enter and Space opening it on its chosen option and firing nothing; keys moving the highlight over enabled rows and choosing nothing; Enter, Space, Tab and a click choosing a row, one `input` then one `change`, Tab before focus moves either way, the chosen option firing nothing and a disabled row changing nothing; Escape, a second click, a click outside and the page scrolling closing with no event; a label's click focusing the select and opening nothing; a disabled, inert, hidden, singular or empty select opening nothing, and one made inert, disabled, hidden or removed with its list open closing; an option dropped with the list open no row; the tree reading the list open on the select, its focus staying there, an author's `aria-expanded` and `open` steering nothing; a box that clips the select clipping no row; a pointer at a row's centre landing on it, scrolled or not; groups read as headers, a header choosing nothing; a long list showing twenty rows and scrolling by key and wheel; hostile markup (`size`, 2,000 options, a 5,500-character label) keeping every box finite; `size` on a `multiple` select making it that many rows tall; each open, move, scroll and close asking a redraw. One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1775`, for any control, with no list opened, the test's comment saying why (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `select_popup` (25), with its one skipped key press."
    rationale: "Report Coverage, select list: integ `select_popup` 25; Spec claims disproved `panic-pos` and Outcome (security): `select_popup` skips the key press in that one row; New text lists the 25 tests at 202-1186."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record, third of three after line 44: «- **Paint ↔ harness, a single select's list:** since 2026-10-31-select-popup `select_popup_painted` (6), in both layout modes, reads the list on frames — the highlight and a disabled row against `SelectListView`'s constants, the plain pair measured at the default sheet and at the stand, a list painted whole past an `opacity` ancestor and a box that clips its select, a closed select's frame byte-equal before opening and after a close with no choice, a group's header darker than a row by weight, and a scrolled list's thumb painted and taking no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351); no local gate builds `blitz-paint/font-embolden`, so the header's weight read red on `Test (macos)` alone at CI#41233234207 through `apple-font-embolden`, and `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` is its linux witness — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `select_popup_painted` (6) and the `font-embolden` flag as its linux witness."
    rationale: "Report Coverage, select list: integ `select_popup_painted` 6; Decisions: a local gate read green over a defect only macOS showed, the flag reproducing it on linux; `os-sensitive`: `apple-font-embolden` red on macOS alone; New text lists the 6 tests at 119-351."
note: D-tests-coverage, D-tests-framework and D-tests-obs-harness fire nothing here (the new paths carry tests at the tier, `cargo test` under the built-in harness, the 5-command harness and log format unmoved); no Retired item stands in this file; every proposal is new detail or a moved count; `host_log` 5 and `mcp_log` 3 rest on the report's +1 over the file's own 4 and 2, not on a recount.
