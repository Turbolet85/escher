# Cascade compare — test-plan · wrap · 2026-10-31-select-popup
E items 62 · held 25 · lost at classify 4 · lost at strong 3 · beside a placed fact 4 · cascade only 7 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### ci-integration#9.1 · lost at strong · beside a placed fact F8 (near 28, ci-integration#4.1) · .andromeda/test-plan-detail/ci-integration.md:62
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 62
    section: "## 9. CI Integration"
    change: "A new continuation line after :62 reads: Since 2026-10-31-select-popup a paint change under an OS-gated feature is known to read green on every local gate and red on one OS: `apple-font-embolden` made CI#41233234207 at `8cd9c1286` red on `Test (macos)` alone, sixteen jobs success, no local gate building that feature; `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`), the local witness of such a change; `cargo` stops at the first failing target, so a red job's later targets go unread on that run — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md"
    sidecar: "§9 Engine features by runner: new line, an OS-gated paint feature no local gate builds, its local witness flag, and a red job's later targets unread (2026-10-31-select-popup)."
    rationale: "The report's Decisions & corrections state the local gate block green over a defect only macOS showed and the flag that reproduces it; Spec claims disproved (`os-sensitive`) and Cross-project claims record the first run's red. §9's Engine features by runner states what differs between builds and holds no such feature."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md

### e2e-test-strategy#2.1 · lost at classify · .andromeda/test-plan-detail/e2e-test-strategy.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 18
    section: "## 6. E2E Test Strategy"
    change: "New item after Cells' handle calls: - **The select list, headless:** the list is painted through `paint_scene` alone, no shell or renderer moved, and driven through the harness, the session and the binary; its windowed path read and not run (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)"
    sidecar: "E2E Test Strategy: new item, the select list measured headlessly, its windowed path read and not run (2026-10-31-select-popup)."
    rationale: "Report, Outcome's (arch) row on `paint_scene` (`packages/blitz-shell` and `packages/dioxus-native` absent from the 41, windowed list read and not run) and Coverage of new surfaces, the same limit; it is the shape of line 18's Cells limit and serves the body's NOT YET MEASURED on windowed runs."

### integration-test-strategy#4.2 · lost at classify · .andromeda/test-plan-detail/integration-test-strategy.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 32
    section: "## 5. Integration Test Strategy"
    change: "new record after this line: «- **Session host ↔ session socket, a select's list:** since 2026-10-31-select-popup `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in the state directory `cc-popup` under `CARGO_TARGET_TMPDIR`, opens the list through a held host, reads its shot equal to the in-process one and chooses a row by a click (examples/seven_guis/tests/cli_commands.rs:419-464) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `cli_commands`' hosted list and shot."
    rationale: "Report Coverage, the list's pixels on the screenshot carriers: `cli_commands` hosted shot equal to in-process; Symbols names `cc-popup` (`cli_commands.rs:442`); New text lists the test at 419-464."

### integration-test-strategy#5.1 · lost at strong · .andromeda/test-plan-detail/integration-test-strategy.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record after this line, first of three: «- **Session ↔ held instance, a single select's list:** since 2026-10-31-select-popup `stand_act_popup` (12) drives the list through the driver, in both layout modes — a `click` opening it and a `click` on a row choosing, keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows' `bounds` back at the select's box, two `click`s on one point opening then closing, each focusing, a row past the scrollport `off-screen` until `scroll` brings it in, `type` leaving no list open and choosing by its text, a shot of a row cropping to it with the list left open, the lint reading no `overlap` on a row with the planted pair still read, the open screen under its budget with every actionable element keyed, and a screen read after a step reading the new state with no `advance`; on the stand `stand_lint`'s `an_open_lists_rows_read_no_overlap_on_the_stand` (+1) and `stand_keyboard`'s `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it` (+2); through `paint_scene` alone, the windowed list read and not run (tests/blitz-tests/tests/stand_act_popup.rs:58-423; tests/blitz-tests/tests/stand_lint.rs:493-519; tests/blitz-tests/tests/stand_keyboard.rs:1082-1134) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `stand_act_popup` (12), `stand_lint` +1, `stand_keyboard` +2 over the list."
    rationale: "Report Coverage, select list: integ `stand_act_popup` 12, `stand_keyboard` +2, `stand_lint` +1; Outcome `v030-17` and the (arch), (design), (layouts) and (lint) rows; New text lists every test named."

### integration-test-strategy#5.1 · lost at strong · .andromeda/test-plan-detail/integration-test-strategy.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record, second of three after line 44: «- **Harness ↔ event pipeline, a single select's list:** since 2026-10-31-select-popup `select_popup` (25) reads the engine's list through harness input, in both layout modes — a click, Enter and Space opening it on its chosen option and firing nothing; keys moving the highlight over enabled rows and choosing nothing; Enter, Space, Tab and a click choosing a row, one `input` then one `change`, Tab before focus moves either way, the chosen option firing nothing and a disabled row changing nothing; Escape, a second click, a click outside and the page scrolling closing with no event; a label's click focusing the select and opening nothing; a disabled, inert, hidden, singular or empty select opening nothing, and one made inert, disabled, hidden or removed with its list open closing; an option dropped with the list open no row; the tree reading the list open on the select, its focus staying there, an author's `aria-expanded` and `open` steering nothing; a box that clips the select clipping no row; a pointer at a row's centre landing on it, scrolled or not; groups read as headers, a header choosing nothing; a long list showing twenty rows and scrolling by key and wheel; hostile markup (`size`, 2,000 options, a 5,500-character label) keeping every box finite; `size` on a `multiple` select making it that many rows tall; each open, move, scroll and close asking a redraw. One row skips its key press: a key pressed after an ancestor gained `hidden` panics at `packages/blitz-dom/src/node/node.rs:1775`, for any control, with no list opened, the test's comment saying why (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `select_popup` (25), with its one skipped key press."
    rationale: "Report Coverage, select list: integ `select_popup` 25; Spec claims disproved `panic-pos` and Outcome (security): `select_popup` skips the key press in that one row; New text lists the 25 tests at 202-1186."

### integration-test-strategy#5.1 · lost at strong · .andromeda/test-plan-detail/integration-test-strategy.md:44
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 44
    section: "## 5. Integration Test Strategy"
    change: "new record, third of three after line 44: «- **Paint ↔ harness, a single select's list:** since 2026-10-31-select-popup `select_popup_painted` (6), in both layout modes, reads the list on frames — the highlight and a disabled row against `SelectListView`'s constants, the plain pair measured at the default sheet and at the stand, a list painted whole past an `opacity` ancestor and a box that clips its select, a closed select's frame byte-equal before opening and after a close with no choice, a group's header darker than a row by weight, and a scrolled list's thumb painted and taking no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351); no local gate builds `blitz-paint/font-embolden`, so the header's weight read red on `Test (macos)` alone at CI#41233234207 through `apple-font-embolden`, and `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted` is its linux witness — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»"
    sidecar: "integration-test-strategy §5: new record, `select_popup_painted` (6) and the `font-embolden` flag as its linux witness."
    rationale: "Report Coverage, select list: integ `select_popup_painted` 6; Decisions: a local gate read green over a defect only macOS showed, the flag reproducing it on linux; `os-sensitive`: `apple-font-embolden` red on macOS alone; New text lists the 6 tests at 119-351."

### mocking-stubbing-discipline#1.2 · lost at classify · beside a placed fact F13 (near 10, mocking-stubbing-discipline#1.1) · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 11
    section: "## 8. Mocking & Stubbing Discipline"
    change: "Add after this line the row: | Shell | `CountingShell`, `select_popup`'s in-file `ShellProvider` fake since 2026-10-31-select-popup, counts `request_redraw` calls, read by `opening_moving_scrolling_and_closing_each_ask_a_redraw` | (tests/blitz-tests/tests/select_popup.rs:1034-1043; tests/blitz-tests/tests/select_popup.rs:1045-1090) |"
    sidecar: "§8 Mocking: new Shell row, `select_popup`'s `CountingShell`, a hand-written `ShellProvider` fake counting redraw requests (2026-10-31-select-popup)."
    rationale: "The report's New text, by line lists `struct CountingShell` at 1034-1037, `impl ShellProvider for CountingShell` with `request_redraw` at 1039-1043 and its reader at 1045-1090, in the new file `tests/blitz-tests/tests/select_popup.rs` that Changes names; the Shell rows at lines 10-11 name no such stand-in."
    basis: tests/blitz-tests/tests/select_popup.rs:1034-1043

### test-data-fixtures#3.1 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 22
    section: "## 7. Test Data & Fixtures"
    change: "| Hostile `select` markup | `hostile_html`, a page built in code with `format!`: 2,000 options, a 5,500-character label and a hostile `size`, read by `hostile_markup_keeps_every_box_finite_and_panics_nowhere` in both layout modes | (tests/blitz-tests/tests/select_popup.rs:913-930; tests/blitz-tests/tests/select_popup.rs:938-975) |"
    sidecar: "Test Data & Fixtures: new seed row, the hostile select page `select_popup` generates in code."
    rationale: "The report's Coverage of new surfaces (`size on a multiple select`: hostile `size`) and Outcome (security: hostile `size`, 2,000 options, a 5,500-character label) bring a generated page this table's seeds do not hold; its lines are in New text, by line."

### test-data-fixtures#3.2 · lost at classify · beside a placed fact F12 (near 24, test-data-fixtures#3.3) · .andromeda/test-plan-detail/test-data-fixtures.md:23
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 23
    section: "## 7. Test Data & Fixtures"
    change: "| Select list pages (`rsx!`, each in its check's own file) | `select_popup`'s `page`, `stated_page` (an author's `aria-expanded` and `open`), `clipped_page`, `long_page`, `sized_page` (`size` on a `multiple` select), `focus_moved_page` (a `:focus` rule moving the select's box) and `chosen_disabled_page` (a disabled chosen option); `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long` | (tests/blitz-tests/tests/select_popup.rs:32-107; tests/blitz-tests/tests/select_popup.rs:616-626; tests/blitz-tests/tests/select_popup.rs:653-675; tests/blitz-tests/tests/select_popup.rs:857-868; tests/blitz-tests/tests/select_popup.rs:977-993; tests/blitz-tests/tests/select_popup.rs:1092-1103; tests/blitz-tests/tests/select_popup.rs:1124-1134; tests/blitz-tests/tests/select_popup_painted.rs:22-34; tests/blitz-tests/tests/select_popup_painted.rs:189-209; tests/blitz-tests/tests/select_popup_painted.rs:255-267; tests/blitz-tests/tests/select_popup_painted.rs:299-310; tests/blitz-tests/tests/stand_act_popup.rs:168-196) |"
    sidecar: "Test Data & Fixtures: new seed row, the `rsx!` pages the three new select list checks state in their own files."
    rationale: "The report's Changes name three new check files and its New text, by line lists their pages; the table holds no row for them. Deviations names `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, the reader of `focus_moved_page`."

### test-scope-summary#7.4 · lost at strong · beside a placed fact F2 (near 37, test-scope-summary#4.1) · .andromeda/test-plan-detail/test-scope-summary.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 58
    section: "## 1. Test Scope Summary"
    change: "Add after this line: «- **blitz-dom (node) — `node/select_list.rs`:** 6 inline tests of the list's state — a list opens where its rows fit and else on the side with more room; it is at least its select's width, at most the viewport's, and stays inside it; every edge of a placed list is finite; `size` reads a whole number above zero, capped; a scroll stays within the rows and brings a row in by the least movement; a point reads the row it lies on where the rows are scrolled (packages/blitz-dom/src/node/select_list.rs:386-539); `BaseDocument`'s public `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout` and `close_select_list` are read through the checks `select_popup`, `select_popup_painted` and `stand_act_popup` (below) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 blitz-dom (node): `node/select_list.rs` and its 6 unit tests."
    rationale: "Report, Crates / modules: new module blitz-dom `node::select_list`; Coverage: tests unit (`select_list` 6); New text: select_list.rs 386-539."

### test-scope-summary#25.1 · lost at strong · beside a placed fact F3 (near 76, test-scope-summary#10.1) · .andromeda/test-plan-detail/test-scope-summary.md:216
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `select_popup`:** 25, its own target with no `stand_` prefix, each test in both layout modes, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand` or the `a11y` leg: a click, Enter and Space open the list on its chosen option and fire nothing; the keys move the highlight over enabled rows and choose nothing; Enter, Space, Tab and a click choose the row, one `input` then one `change`, a choice of the chosen option firing nothing; Tab chooses and closes before focus moves, either way; a disabled row changes nothing and leaves the list open; Escape, a second click, a click outside and the page scrolling close with no event; a label's click focuses the select and opens nothing; a disabled, inert, hidden, singular or empty select opens nothing; a select made inert, disabled, hidden or removed with its list open closes, the ancestor-hidden row skipping its key press, which panics for any control at packages/blitz-dom/src/node/node.rs:1775, with no list opened; an option dropped with the list open is no row; the tree reads the list open on the select, its focus staying there; an author's `aria-expanded` and `open` steer nothing on a select; a box that clips the select clips no row; a pointer at a row's centre lands on it, the page scrolled or not; a press on a row selects no text and a drag across rows chooses where it ends; groups read as headers, a header choosing nothing; a long list shows twenty rows and scrolls by key and wheel; a hostile `size`, 2,000 options and a 5,500-character label keep every box finite; `size` makes a `multiple` select that many rows tall; opening, moving, scrolling and closing each ask a redraw; a focus rule that moves the select's box leaves the list a click opened; a click in the select's box closes and chooses nothing (tests/blitz-tests/tests/select_popup.rs:202-1186) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `select_popup` 25, the list's behaviours, with its one skipped key press."
    rationale: "Report, Coverage: integ `select_popup` 25; Harness surface: `ci-leg.sh` `stand-repeat` gains the word, the `a11y` leg stays fourteen; Spec claims disproved, `panic-pos`; New text: select_popup.rs."

### test-scope-summary#25.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:216
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `select_popup_painted`:** 6, its own target with no `stand_` prefix, each test in both layout modes on frames, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand` or the `a11y` leg: the highlight and a disabled row read as painted against `SelectListView`'s constants; the plain pair measured at the default sheet and at the stand; a list paints whole past an `opacity` ancestor and a box that clips its select; a closed select's frame byte-equal before opening and after a close with no choice; a group's header darker than a row by weight, green on both builds, plain and `blitz-paint/font-embolden`; a scrolled list paints its thumb, which takes no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351); `render/select_list.rs` holds no unit test, read on these frames — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `select_popup_painted` 6, the list on frames."
    rationale: "Report, Coverage: integ `select_popup_painted` 6; Crates / modules: new blitz-paint `render::select_list`; Outcome: read on both builds, 6 · 0 each; `ua-pair`: the plain pair measured there."

### test-scope-summary#25.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:216
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 216
    section: "## 1. Test Scope Summary"
    change: "Add in its own detail block after this entry's: «- **tests/blitz-tests — `stand_act_popup`:** 12, the sixteenth `stand_act_*` file, declaring `mod session_common;`, each test in both layout modes through the driver, picked up by `run stand` and the stand-repeat leg by its prefix, not run by the `a11y` leg: a click opens the list and a click on a row chooses it; the keys alone open, move and choose; Escape closes with the value unchanged and the rows back at the select's box; a closed option stays refused `covered`, changing nothing; a row past the scrollport is off-screen until `scroll` brings it in; a button under the list is refused `covered`; `type` leaves no list open and chooses by its text; a shot of a row crops to it and leaves the list open; the lint reads no overlap on a row and the planted pair still reads; the open screen stays under its budget, every actionable element keyed; two clicks on one point open and close, each focusing; a screen read after a step reads the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423) — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 tests/blitz-tests: `stand_act_popup` 12, the list through the driver."
    rationale: "Report, Coverage: integ `stand_act_popup` 12; Harness surface: joins `run stand` and `stand-repeat` by its prefix, the `a11y` leg stays fourteen; `mod-decl`: `session_common` 31 → 32; New text: stand_act_popup.rs."

### test-scope-summary#28.2 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:235
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 235
    section: "## 1. Test Scope Summary"
    change: "Add after this line: «- **The open list through the process:** seven_guis' `cli_commands` `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, in `cc-popup`, the hosted shot equal to the in-process one (14 tests until 2026-10-31-select-popup; examples/seven_guis/tests/cli_commands.rs:419-464); `host_log` `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in `hl-popup`, `#[cfg(unix)]` as every host-driving test there (examples/seven_guis/tests/host_log.rs:282-393); `mcp_log` `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing and holds no state directory (examples/seven_guis/tests/mcp_log.rs:68-113) — the two log checks reading 0 needles at `trace` — as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md»."
    sidecar: "§1 apps: the three process tests of the open list."
    rationale: "Report, Coverage: process (`cli_commands`, `host_log`, `mcp_log` +1 each); Symbols, Test-only state directories: `cc-popup`, `hl-popup`, no `ml-popup`; Reverted: `mcp_log`'s test records nothing; Outcome (security): 0 needles at `trace`."

### test-strategy#5.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:34
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 34
    section: "## 2. Test Strategy"
    change: "New bullet after Pixel tests: **Painted select list (since 2026-10-31-select-popup):** `select_popup_painted` reads an open list on a frame's `Raster` in both layout modes, a highlighted and a disabled row within one channel of the engine's constants, a closed select's frame byte-equal before opening and after a close with no choice, and a group's header darker than a row by its ink; `font-embolden` is built by no local gate, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced on linux the header-versus-row numbers that made CI#41233234207 red on `Test (macos)` alone (tests/blitz-tests/tests/select_popup_painted.rs:119-159; tests/blitz-tests/tests/select_popup_painted.rs:237-253; tests/blitz-tests/tests/select_popup_painted.rs:282-297; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)"
    sidecar: "Pixel tests: a new bullet for `select_popup_painted` and the local `blitz-paint/font-embolden` witness, from 2026-10-31-select-popup."
    rationale: "The report adds `select_popup_painted` (New text, lines 119-159, 237-253, 282-297) and records under Decisions & corrections that a local gate block read green over a defect only macOS showed, with this flag its local witness; line 34 states the frame strategy and names neither."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:131

## Held

- ci-integration#1.1
- ci-integration#10.1
- ci-integration#11.1
- ci-integration#2.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- integration-test-strategy#4.1
- integration-test-strategy#4.3
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- test-scope-summary#12.2
- test-scope-summary#16.1
- test-scope-summary#2.1
- test-scope-summary#2.4
- test-scope-summary#21.1
- test-scope-summary#24.1
- test-scope-summary#27.1
- test-scope-summary#28.4
- test-scope-summary#6.1
- test-strategy#4.1
- test-strategy#8.1
- unit-test-strategy#11.1
- unit-test-strategy#3.2
- unit-test-strategy#5.1
- unit-test-strategy#6.1

## Cascade only

- ci-integration#4.1
- mocking-stubbing-discipline#1.1
- quality-gates-coverage-targets#5.1
- test-scope-summary#10.1
- test-scope-summary#20.1
- test-scope-summary#23.1
- test-scope-summary#4.1
