## 2026-10-31-select-popup — `type` focuses any select with no click
**Section:** §3 Test Harness Contract (registry session-lifecycle: session-start); §4 Unit Test Strategy (unit-test-strategy.md: escher-driver `type`'s help)
**Change:**
- session-start: `type` replaces after the focusing click, a select, single or `multiple`, focused with none and no DOM focus event fired, so a text opening on a space types nothing on a single select (was: a `multiple` select focused with none).
- `type`'s help: `schema_tests` pins it on any select, which `type` focuses with no click (was: on a `multiple` select).
**Why:** `Target.multiple` and `focus_multiple_select` are retired for `Target.select` and `focus_select`, and `type` on any select now focuses with no click, so it fires no DOM focus events on a single select either.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — sixteen driver-action files, `stand_act_popup` the sixteenth
**Section:** §3 Test Harness Contract (registry session-lifecycle: session-proof; test-plan.md: the `mod common;` and `mod session_common;` lines); §5 Integration Test Strategy (integration-test-strategy.md: the acting-call record); §1 Test Scope Summary (test-scope-summary.md: escher-driver); §2 Test Strategy (test-strategy.md: Directory pattern); §4 Unit Test Strategy (unit-test-strategy.md: the executor's drivers)
**Change:**
- The `stand_act_*` count reads sixteen (was fifteen) in each of those places.
- session-proof: after `stand_act_select` 10, the fifteenth (61 in all), stands `stand_act_popup` 12, the sixteenth (73 in all).
- §1: `stand_act_popup` is named the sixteenth beside `stand_act_select` the fifteenth.
- test-plan.md: six of the sixteen driver-action checks read `mod common;` (was six of the fifteen).
**Why:** `stand_act_popup` is a new `stand_act_*` file, 15 to 16 in the tree. A `git grep` for `mod common;` over those files still reads six.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the open list through the process checks
**Section:** §3 Test Harness Contract (registry session-lifecycle: session-proof); §6 E2E Test Strategy (e2e-test-strategy.md: Command line; new item Command line, a select's list; the `mcp` role); §5 Integration Test Strategy (integration-test-strategy.md: the log records; new records Session host ↔ what it writes and Session host ↔ session socket, a select's list); §1 Test Scope Summary (test-scope-summary.md: apps; new record The open list through the process); §4 Unit Test Strategy (unit-test-strategy.md: the driver's process checks)
**Change:**
- `cli_commands` reads 15: was 14 at session-proof, §4 and §6; was 13 at §1, already behind at the base.
- `host_log` reads 5 (was 4) and `mcp_log` 3 (was 2), at session-proof, §1 and §5; §6 reads `mcp_log` (3).
- `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, state directory `cc-popup` under `CARGO_TARGET_TMPDIR`: a held host opens the list, its shot equals the in-process one, a click on a row chooses.
- `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, in `hl-popup`, `#[cfg(unix)]` as every host-driving test there: 0 option needles at `trace`.
- `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`: 0 option needles at `trace`, recording nothing, no `ml-popup` directory existing, since recording there would need a `cfg!(unix)` site.
- §1's shared module listing gains `option_needles()`, the needle list of a select's options read from a snapshot, called by `host_log` and `mcp_log`.
**Why:** the chunk added one test each to `cli_commands`, `host_log` and `mcp_log`; the two log checks read 0 needles at `trace`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — citations re-pointed at the lines the chunk moved
**Section:** registry session-lifecycle; ci-integration.md; e2e-test-strategy.md; integration-test-strategy.md; mocking-stubbing-discipline.md; quality-gates-coverage-targets.md; test-data-fixtures.md; test-scope-summary.md; test-strategy.md; unit-test-strategy.md; test-plan.md
**Change:**
- Cited line ranges read where the lines now stand, the cited claim unmoved, into blitz-dom `document.rs`, `stylo.rs`, `scrolling.rs` and `default.css`; blitz-paint `render.rs`; dioxus-native-dom `snapshot.rs`; escher-driver `execute.rs`, `refusal.rs`, `schema.rs`, `schema_tests.rs`, `lint.rs` and `json.rs`; `ci-leg.sh` and `test_ci_workflows.py`; `docs/capabilities.md`; `select_interaction`, `stand_act_select`, `stand_contrast` and `stand_snapshot_state`; seven_guis `cli_commands`, `host_log`, `mcp_log` and `common/mod.rs`.
**Why:** the chunk's edits to those files moved the cited lines.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the stand-repeat leg, seventeen named words and 51 files
**Section:** §9 CI Integration (ci-integration.md: Legs, Legs reaching `pseudo_element_flow_classification`; test-plan.md: Legs); §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md, test-plan.md: Flakiness budget); §1 Test Scope Summary (test-scope-summary.md: The 0.2.0 sweep test's reach); §4 Unit Test Strategy (unit-test-strategy.md: the `stand-repeat` cases)
**Change:**
- One `--test` per `stand_*.rs` reads 51 files (was 50), `stand_act_popup` joining by its prefix, at §9 and §10.
- The named `--test` words read seventeen (was fifteen), `select_popup` and `select_popup_painted` the two added, wherever the body counts them; `test_stand_repeat_leg_runs_three_rounds` names the glob and the seventeen.
- `pseudo_element_flow_classification` is none of the seventeen.
- The sweep test's reach: `select_interaction` and seven more of the targets the leg names (was five more), `select_popup` and `select_popup_painted` among them.
**Why:** `ci-leg.sh`'s `stand-repeat` list gains the two words and `test_ci_workflows.py` asserts them; `stand_act_popup` joins by its prefix.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the a11y leg 114 tests, the tier-0 witnesses 88
**Section:** §9 CI Integration (ci-integration.md: Legs, the local baseline's a11y reading)
**Change:**
- The a11y leg reads 14 files and 114 tests (was 112).
- The tier-0 witnesses alone read 9 files and 88 tests (was 59).
- The local baseline reads 14 result lines · 114 passed · 0 failed · 0 ignored (was 112 passed), `stand_keyboard` 23 in its list (was 21), as measured at this chunk's report (was 2026-10-31-engine-debts').
**Why:** `stand_keyboard` gained two tests, the leg moving 14 · 112 · 0 to 14 · 114 · 0, still fourteen files. The chunk's +2 does not explain 88: 59 was already behind at the base, a11y-plan's ci-integration reading 86 there, and 88 is 114 less the five tree-reading checks' 26, this pass's arithmetic and not a reading the report states.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the macOS-only red and `blitz-paint/font-embolden`'s local witness
**Section:** §9 CI Integration (ci-integration.md: new records Red on `Test (macos)`, and its local witness; A red job's later targets); §2 Test Strategy (test-strategy.md: new bullet Painted select list)
**Change:**
- New record: CI#41233234207 at `8cd9c1286` ended red on `Test (macos)` alone, sixteen jobs `success`, through `apple-font-embolden`; no local gate builds `blitz-paint/font-embolden`, owner a CARRY on «Upstream sync ahead of the UI graph», so a local gate block reads green over a defect only macOS shows.
- A paint change under that feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced macOS's two numbers on linux (`the header reads 70972, the row 70972`).
- New record: `cargo` stops at the first failing target, so a red job's later targets go unread on that run.
- §2's new bullet: `select_popup_painted` reads an open list on a frame's `Raster` against the engine's constants, in both layout modes, and names the same witness.
**Why:** a local gate block read green over a defect only macOS showed, fixed at `715798ce2`. The owner is the founder's word by dialog at P2 Validate, 2026-10-31, relayed verbatim by the operator (answer card, Q2): both owners on the sync.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the per-OS and stand-repeat tallies on CI#41235503227
**Section:** §9 CI Integration (ci-integration.md: Per-OS tallies, the local baseline's workspace leg); §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: Flakiness budget)
**Change:**
- New line, re-read on CI#41235503227 at `715798ce2`, green 17/17: linux 217 · 1236 · 0 · 12; macos and coverage 198 · 1233 · 0 · 12; windows 198 · 1185 · 0 · 9; the a11y job 14 · 114 · 0 · 0; the stand repeat 228 · 1155 · 0 · 21. Over CI#41192433140: +3 result lines and +55 passed on linux, macos and coverage, +3 · +53 on windows, no new line and +2 on the a11y job, +9 · +147 on the stand repeat.
- The workspace leg's re-count: 217 result lines, 1236 passed · 0 failed · 12 ignored (was 214 · 1181): `select_popup` 25, `select_popup_painted` 6 and `stand_act_popup` 12 the three new lines, and +55 being those 43, blitz-dom's lib `select_list` 6, `stand_keyboard` +2, `stand_lint` +1, `cli_commands`, `host_log` and `mcp_log` +1 each; timings not re-measured.
- Flakiness budget: the repeat reads 228 · 1155 · 0 · 21 on CI#41235503227 (was 219 · 1008 · 0 · 21 on CI#41192433140); its `+15 · +144` and `stand-repeat: 3 of 3 rounds green` clauses left that reading.
**Why:** CI#41235503227 is the run the chunk's verdict reads, its final HEAD; each move is named in the chunk's `evidence/ci.md`.
**Kept:** the free-disk step's record of CI#41192433140 stays as it stood: it is that run's record, and the step stays unproven as the cure.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — what the list leaves unmeasured
**Section:** §6 E2E Test Strategy (e2e-test-strategy.md: new item The select list, headless); §1 Test Scope Summary (test-scope-summary.md: new record blitz-dom (select) — the list's unmeasured limits)
**Change:**
- §6: the list paints through `paint_scene` alone, no shell or renderer code of its own; the harness, the session and the binary drive it, measured headlessly; its windowed path read and not run.
- §1: the windowed list is read and not run, and what an assistive technology reads is `unrunnable-here`; stated limits: the thumb is not draggable, a highlighted row's contrast is unread by the lint, a larger authored font clips, the highlight is in no snapshot and no tree. Owner of those four: each limit's line in `.andromeda/residuals.md`.
**Why:** the chunk's diff leaves `packages/blitz-shell` and `packages/dioxus-native` untouched, and the list ran in the harness alone. The residual lines are the operator's word on the P5 card, relayed verbatim by the operator, 2026-10-31 (inputs#I7), for this wrap's P5 to write.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `covered`'s closed-option clause and the restated help
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the `covered` readings); §4 Unit Test Strategy (unit-test-strategy.md: new row escher-driver, the list's help and `covered`)
**Change:**
- `covered`'s fixed meaning also says an option of a closed select lies under it and to click the select first, then the option.
- New row: `schema_tests`, `refusal` and `json` pin the help of `click`, `type` and `press` and that clause, each fixed `'static` text, with `VERBS` at 12, causes at 8 and `describe`'s count row `[12, 13, 12, 3, 8, 4, 15]`.
**Why:** an option of a closed select stays `covered` on the founder's F9, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (answer card; inputs#I4). The clause and the help are the program's own fixed text; `covered`'s name, remedy and order did not move.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `stand_act_popup`, the list through a held session
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: new record Session ↔ held instance, a select's list); §1 Test Scope Summary (test-scope-summary.md: new record tests/blitz-tests — `stand_act_popup`)
**Change:**
- New records of `stand_act_popup`, 12 tests, each in both layout modes: a click opening the list and a click on a row choosing; the keys alone opening, moving and choosing; Escape closing with the value unchanged and the rows back at the select's box; a closed option and a button under the list refused `covered`; a row past the scrollport `off-screen` until `scroll` brings it in; `type` leaving no list open; a shot of a row cropping to it and leaving the list open; the lint reading no `overlap` on a row; the open screen under its budget with every actionable element keyed; two clicks on one point opening then closing, each focusing; a read after a step with no `advance`.
- Run by `run stand` and the `stand-repeat` leg by its prefix, not among the `a11y` leg's fourteen.
**Why:** the chunk added the check, 12 · 0 on both builds, and its prefix picks it up; the `a11y` leg stays fourteen files.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `select_popup`, the list's engine behaviour
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: new record Harness ↔ event pipeline, a single select's list); §1 Test Scope Summary (test-scope-summary.md: new record tests/blitz-tests — `select_popup`)
**Change:**
- New records of `select_popup`, 25 tests, its own target with no `stand_` prefix, each in both layout modes, through harness input.
- It reads: a click, Enter and Space opening the list on its chosen option and firing nothing; the keys and the pointer moving the highlight over enabled rows, choosing nothing; Enter, Space, Tab and a click choosing a row, one `input` then one `change`, Tab before focus moves either way, the chosen option firing nothing, a disabled row changing nothing; Escape, a second click, a click outside and the page scrolling closing with no event; a label's click focusing the select and opening nothing.
- It reads: a disabled, inert, hidden, singular or empty select opening nothing, one made inert, disabled, hidden or removed with its list open closing, an option dropped with the list open no row; the tree reading the list open on the select, focus staying there, an author's `aria-expanded` and `open` steering nothing; a box that clips the select clipping no row; a pointer at a row's centre landing on it, scrolled or not; groups as headers, a header choosing nothing; twenty rows, scrolling by key and wheel; hostile markup (`size`, 2,000 options, a 5,500-character label) keeping every box finite; `size` making a `multiple` select that many rows tall; each open, move, scroll and close asking a redraw.
- Run by the workspace `test` leg, `run all`, by name and as a named `--test` of `stand-repeat`, not by `run stand` or the `a11y` leg.
**Why:** the chunk added the check, 25 · 0 on both builds.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the key press `select_popup` skips and the panic no check reads
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: Harness ↔ event pipeline, a single select's list); §1 Test Scope Summary (test-scope-summary.md: new record blitz-dom (select) — the key press `select_popup` skips)
**Change:**
- `select_popup` skips the key press in its one row where an ancestor gains `hidden`, its comment saying why: a key press there panics at `position_in_containing_block` (`packages/blitz-dom/src/node/node.rs:1775`) for any focused control, list or no list, the control's anonymous block dropped.
- No check reads that panic. Owner: a CARRY on «Upstream sync ahead of the UI graph».
**Why:** the panic reproduced at implement on a `select` and a plain `button`, with no list opened; a remote page can reach it. The owner is the founder's word by dialog at P2 Validate, 2026-10-31, relayed verbatim by the operator (answer card, Q2): both owners on the sync.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `select_popup_painted`, the list on frames
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: new record Paint ↔ harness, a single select's list); §1 Test Scope Summary (test-scope-summary.md: new record tests/blitz-tests — `select_popup_painted`)
**Change:**
- New records of `select_popup_painted`, 6 tests, its own target with no `stand_` prefix, each in both layout modes: the highlight and a disabled row against `SelectListView`'s constants; the plain pair measured at the default sheet and at the stand; a list painted whole past an `opacity` ancestor and a box that clips its select; a closed select's frame byte-equal before opening and after a close with no choice; a group's header darker than a row by weight; a scrolled list's thumb painted and taking no hit.
- The header's weight under `blitz-paint/font-embolden` points at §2's Painted select list for its local witness.
- Run as `select_popup` is, a named `--test` of `stand-repeat`. blitz-paint's `render/select_list.rs` holds no unit test of its own.
**Why:** the chunk added the check, 6 · 0 on both builds, and the new module `render::select_list` with no inline test.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `select_popup`'s `CountingShell`
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: new Shell row)
**Change:**
- New Shell row: `select_popup`'s own hand-written `CountingShell`, a `ShellProvider` counting redraw requests, reads that opening, moving, scrolling and closing a select's list each ask a redraw.
**Why:** the new check states its own fake, and the Shell rows named no such stand-in.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the coverage reading on CI#41235503227
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the current reading)
**Change:**
- escher-driver reads 4959/5058 = 98.04 % (was 4950/5049 = 98.04 %).
- dioxus-native-dom reads 2791/3240 = 86.14 % (was 2786/3235 = 86.12 %).
- blitz-test-harness reads 968/1008 = 96.03 % (was 956/1008 = 94.84 %).
- Floors 3 of 3, read on CI#41235503227 (was the `Coverage report` job of CI#41192433140).
**Why:** the reading of the run the chunk's verdict reads, in the chunk's `evidence/ci.md`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the complexity reading before the list's clearing
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: Complexity ceiling)
**Change:**
- `clear_interaction_state_for_removed_node`'s 17.0 and the count of 85 now read as read before that function took the clearing of the select list's field, no later reading claimed (was: 17.0 with no such limit).
**Why:** the chunk edited that function to clear the field for its select, and no new complexity reading was taken.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list's pages and needles
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: new rows Hostile `select` markup, Select list pages, Select option needles)
**Change:**
- Hostile `select` markup: `hostile_html`, a page generated in code with `format!`, 2,000 options, a 5,500-character label and a hostile `size`, read by `hostile_markup_keeps_every_box_finite_and_panics_nowhere` in both layout modes.
- Select list pages, `rsx!`, each in its check's own file: `select_popup`'s `page`, `stated_page`, `clipped_page`, `long_page`, `sized_page`, `focus_moved_page` and `chosen_disabled_page`; `select_popup_painted`'s `page`, `clipped_page`, `grouped_page` and `long_page`; `stand_act_popup`'s `long_page`, held through `hold_long`.
- Select option needles: `option_needles()` in seven_guis' shared test module, the needle list of a select's options, read from a snapshot.
**Why:** the three new checks and the two log tests bring seeds the table did not hold.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select limits the list retires
**Section:** §1 Test Scope Summary (test-scope-summary.md: the select model's stated limits)
**Change:**
- «no popup is painted» and «Enter and Space open nothing on a single select» end: a closed single select now opens a list the engine paints over the page on a click, Enter or Space, read by `select_popup` and `select_interaction`, whose `enter_and_space_open_nothing_on_a_single_select` is now `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`.
- «a `multiple` select's `size` is unread» ends: its `size` hints a height of `size × 1.2em` through `size_rows`, capped at 65534, `0` or a bad value hinting nothing, read by `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in` and `size_reads_a_whole_number_above_zero_capped`.
- «a group's label is unpainted» ends: it is named in the tree and painted as a header over its options, read by `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`.
- A click on a single select's label focuses it and opens nothing, with no re-dispatch, asserted by `a_labels_click_focuses_the_select_and_opens_nothing` (was: re-dispatched, asserted by no check).
**Why:** the report retires the limits. What opens the list, a group's label and `size` are the founder's F1, F7 and F8, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (answer card; inputs#I4). A label's click opening nothing is no fork: the card reads it as a lean he was shown and did not object to.
**Kept:** a closed single select's option stays refused `covered`, under a group too (his F9, the same record); a single select's `size` stays unread; a click on a `multiple` select's label re-dispatched to it stays asserted by no check.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the lint on an open list, `Entry.in_select_box`
**Section:** §1 Test Scope Summary (test-scope-summary.md: `stand_lint`'s count; tests/blitz-tests — `stand_lint` on selects; The lint's select-option reading)
**Change:**
- `stand_lint` reads 9 (was 8), gaining `an_open_lists_rows_read_no_overlap_on_the_stand`, in both layout modes; `stand_act_popup`'s `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` reads a row the same through a held session, the planted pair still reading.
- The lint skips an entry in a select's box, `Entry.in_select_box` (was `Entry.in_closed_select`): true inside a closed select, through `closed_select_of`, and true for an open list's row or group.
- An open row's `clip_region` comes from `visible_region` alone, with no `contain: paint` cut.
**Why:** `Entry.in_closed_select` is retired. An open list's rows reading no `overlap` is the founder's F10, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (answer card; inputs#I4).
**Kept:** the lint still lints a `multiple` select's rows as usual, and `stand_lint` still pins the stand at `[0, 0, 0, 0]` at boot.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the standing select checks, read through the list
**Section:** §1 Test Scope Summary (test-scope-summary.md: `stand_keyboard`, `select_interaction`, `stand_act_select`)
**Change:**
- `stand_keyboard` reads 23 (was 21), gaining `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`.
- Its closed-arrow test's Enter and Space rows now open the list and then choose, so it reads true and not what its row order suggests.
- `select_interaction`'s 14-row table, `KEYS`, holds `PageDown` and no Enter, which opens the list.
- `stand_act_select`'s click then key: the click opens the select's list and the key moves or chooses in it.
**Why:** Enter on a single select now opens, so `KEYS` moved one row, Enter to `PageDown`, its count still 14; `stand_keyboard`'s closed-arrow test passes untouched while its rows read otherwise; `stand_act_select` stays 10.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `STATE_ROWS` 20 reached states
**Section:** §1 Test Scope Summary (test-scope-summary.md: `every_colour_state_meets_its_ratio`)
**Change:**
- `STATE_ROWS` reads 20 reached states (was 18): the six classes stated and two more.
- The tally reads 19 pass and 1 exempt (was 17 and 1).
- The lowest pass, 5.11, now reads as the lowest of the eighteen in those six classes.
**Why:** the chunk moved `STATE_ROWS` 18 to 20, tally (19, 0, 1, 0); the report names neither the two new rows' class nor a new lowest, so the body states neither.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `node/select_list.rs` and its six unit tests
**Section:** §1 Test Scope Summary (test-scope-summary.md: new record blitz-dom (node) — `node/select_list.rs`; test-plan.md: blitz-dom (node)); §4 Unit Test Strategy (unit-test-strategy.md: new row blitz-dom (node, a select's list); test-plan.md: blitz-dom (node))
**Change:**
- New records: six inline tests: a list opening where its rows fit and else on the side with more room; its width at least its select's and at most the viewport's; every edge of a placed list finite; `size` a whole number above zero, capped; a scroll staying within the rows and bringing a row in by the least movement; a point reading the row it lies on where the rows are scrolled.
- §1's record adds that `BaseDocument`'s public `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout` and `close_select_list` are read through `select_popup`, `select_popup_painted` and `stand_act_popup`.
- test-plan.md's two blitz-dom (node) broad lines name `node/select_list.rs`.
**Why:** the chunk added the module `node::select_list` with six unit tests.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the shared-module readers and the blitz-tests targets
**Section:** §2 Test Strategy (test-strategy.md: Directory pattern); §3 Test Harness Contract (test-plan.md: blitz-tests (fixtures))
**Change:**
- `mod session_common;` is read by 32 checks (was 31), `stand_act_popup.rs` the latest with `stand_cells_served.rs` and `stand_load_page.rs`.
- `mod fixtures;` is read by 31 checks (was 28), counted anchored at line start: 23 `stand_*` files (was 22), `stand_act_popup` the new one, with `select_popup` and `select_popup_painted` among the rest.
- blitz-tests holds 156 `.rs` files (was 153), 155 test targets (was 152) and the unbuilt `all.rs`, `select_popup.rs`, `select_popup_painted.rs` and `stand_act_popup.rs` the latest.
**Why:** the three new checks: 31 to 32 and 28 to 31 at both revisions, 153 to 156 in the tree. An unanchored `mod fixtures;` count reads one more, each shared module's doc comment naming the line.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the capability statement and the broad line on the list
**Section:** §2 Test Strategy (test-strategy.md: Claims mapped to checks); §1 Test Scope Summary (test-plan.md: blitz-dom covered behaviours)
**Change:**
- `docs/capabilities.md`'s `select` model limits read four bullets (was two); the list a click, Enter and Space open is read by `select_popup`, `select_popup_painted` and `stand_act_popup`, beside its keys, rows and groups pinned by `select_interaction` and `stand_act_select`.
- The covered behaviours name the `select` model, single and `multiple`, with `optgroup` and a single select's painted list, its list read by the same three checks.
**Why:** the chunk's `docs/capabilities.md` adds the bullets, and the three checks are the list's readers.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `run stand` 237 and `run all` 727
**Section:** §3 Test Harness Contract (test-plan.md: the `run stand` and `run all` re-counts; Test-data bootstrap of the 5-command contract)
**Change:**
- New re-count, `scripts/agent-run.sh` unchanged: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files (was 222 · 0 · 7 over 50), +15 being `stand_act_popup` 12, `stand_keyboard` +2 and `stand_lint` +1.
- `run all` reads passed 727 · failed 0 · ignored 12, `select_popup` and `select_popup_painted` carrying no `stand_` prefix and picked up by it with no script change.
- `run stand` runs the 23 `stand_*` readers of the shared module (was 22); `run {file}` the eight non-stand readers (was six), gaining `run select_popup` and `run select_popup_painted`.
**Why:** the chunk's last full gate block read `run stand` 237 · 0 · 7 and `run all` 727 · 0 · 12.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `scroll_into_view` on a row of an open list
**Section:** §3 Test Harness Contract (test-plan.md: Input helpers)
**Change:**
- `scroll_into_view(node_id)` keeps its order, every scrolling box that holds the node and then the viewport, and adds that a row of an open select list moves the list's `scroll_y` alone.
**Why:** `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, no chain or viewport moving.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
