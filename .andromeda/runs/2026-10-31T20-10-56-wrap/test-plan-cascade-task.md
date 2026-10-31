# Task — test-plan · 2026-10-31-select-popup

## The session
A closed single select now opens a list the engine paints over the page, opened by a click, Enter or Space, moved by keys, pointer and wheel, and chosen through the existing `apply_select_choice` path; a `multiple` select's `size` is now read, and a group's `label` paints as a header.
Added three check targets, `select_popup` 25, `select_popup_painted` 6 and `stand_act_popup` 12, six unit tests in blitz-dom's `node/select_list.rs`, `stand_keyboard` +2, `stand_lint` +1, and `cli_commands`, `host_log` and `mcp_log` +1 each; the `stand-repeat` leg names two more targets.
Measured `run stand` 237 · 0 · 7 over 51 files, `run all` 727 · 0 · 12, the a11y leg 14 · 114 · 0, and floors 3 of 3; CI#41235503227 is green 17/17 at `715798ce2`, after CI#41233234207 went red on `Test (macos)` alone over a defect in the emboldening build.
Retired the limits that no popup is painted, that Enter and Space open nothing on a single select, that a group's label is unpainted and that a `multiple` select's `size` is unread, along with `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`; one `select_interaction` test was renamed.
No dependency, manifest, schema shape, verb, cause, node key, log field or span moved; `VERBS` stays 12, causes 8, `NODE_FIELDS` thirteen and `describe`'s count row unmoved.
The report counts 41 changed files; Reached files lists forty, leaving out `.gitignore`, whose two lines are a route and incubator matter, and the chunk's own evidence records.

## Reached files
- packages/blitz-dom/src/node/select_list.rs
- packages/blitz-paint/src/render/select_list.rs
- tests/blitz-tests/tests/select_popup.rs
- tests/blitz-tests/tests/select_popup_painted.rs
- tests/blitz-tests/tests/stand_act_popup.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/resolve.rs
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/stylo.rs
- packages/blitz-dom/src/lib.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/node/mod.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-paint/src/render.rs
- packages/blitz-paint/src/text.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/lib.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/refusal.rs
- packages/escher-driver/src/schema.rs
- packages/escher-driver/src/schema_tests.rs
- packages/escher-driver/src/json.rs
- packages/escher-driver/src/lib.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_id_persistence.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- docs/capabilities.md
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- The named `--test` targets of `stand-repeat` move from fifteen to seventeen, adding `select_popup` and `select_popup_painted` at `ci-leg.sh:59-60`, asserted at `test_ci_workflows.py:367`; the report changes test-plan:377 and :413, ci-integration:9, :10 and :16, quality-gates:22 and unit-test-strategy:46.
- The `stand_act_*` files move from fifteen to sixteen; the report changes test-plan:146 and :149, integration-test-strategy:31, test-scope-summary:94, test-strategy:53 and unit-test-strategy:26, while test-plan:150 is an ordinal and stays.
- The `stand_*.rs` files move from 50 to 51 and `run stand` from 222 · 0 · 7 to 237 · 0 · 7, changing test-plan:176 and :190, ci-integration:8 and quality-gates:22; the new re-count is +12 `stand_act_popup` by its prefix, +2 `stand_keyboard` and +1 `stand_lint`.
- `run all` now reads 727 · 0 · 12, changing test-plan:176 and ci-integration:75; the dated history rows that state older totals stay.
- The a11y leg's tally moves from 14 · 112 · 0 to 14 · 114 · 0 on the same fourteen files, changing ci-integration:75 and :99.
- `stand_keyboard` moves from 21 to 23 tests, changing ci-integration:99 and test-scope-summary:151.
- `STATE_ROWS` moves from 18 to 20 with tally (19, 0, 1, 0), changing test-scope-summary:151.
- `cli_commands` moves from 14 to 15, changing e2e-test-strategy:8 and unit-test-strategy:26; test-scope-summary:16 shares the token and states no count.
- Any text that counts `stand_lint`, `host_log` or `mcp_log` tests moves by +1 each.
- The per-OS CI tally moves from CI#41192433140 to CI#41235503227: linux 217 · 1236 · 0 · 12, macos and coverage 198 · 1233 · 0 · 12, windows 198 · 1185 · 0 · 9, a11y 14 · 114 · 0 · 0, stand repeat 228 · 1155 · 0 · 21; this changes ci-integration:75 and quality-gates:16 and :22.
- The coverage reading moves to escher-driver 4959/5058 = 98.04 %, dioxus-native-dom 2791/3240 = 86.14 % and blitz-test-harness 968/1008 = 96.03 %, floors 3 of 3, changing quality-gates:16.
- Checks declaring `mod session_common;` move from 31 to 32 and `mod fixtures;` from 28 to 31, counted anchored at line start, changing test-plan:151 and test-strategy:52 and :53; the split of 22 `stand_*` and six other fixture readers stated at test-plan:151 and :190 can move with that count, and the report gives only the totals.
- test-scope-summary:24 states limits this session retired: no popup, Enter and Space opening nothing and a `multiple` select's `size` unread are all false now, and a single select's `size` stays unread.
- test-scope-summary:24 names `select_interaction::enter_and_space_open_nothing_on_a_single_select`, now `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`.
- test-scope-summary:24 says a click on a select's label re-dispatched to the select is asserted by no check; the single-select half no longer re-dispatches and is asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`.
- test-scope-summary:237 states the lint's `overlap` exclusion; `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group, and an open row's clip comes from `visible_region` alone.
- `covered`'s meaning gains the clause that an option of a closed select lies under it and must be reached by clicking the select first, with name, remedy and order unchanged, changing integration-test-strategy:30.
- `type` on any select now focuses with no click and so fires no DOM focus events on a single select, and a text opening on a space types nothing there; the report reads integration-test-strategy:29 and :31 as still true.
- The select coverage statements at test-plan:17, :19, :25 and :227 name the select's limits and rules through `select_interaction`, `stand_act_select` and `node/select.rs`; the list's behaviour now also rests on `node/select_list.rs` and the three new checks, while `select_interaction` 18 and `stand_act_select` 10 stay unmoved.
- `select_interaction`'s `KEYS` table moved one row, Enter to `PageDown`, because Enter now opens the list; its count stays 14.
- A row's own `bounds` in the snapshot and the diff now read the list's row box while it is open, so `differs` names `bounds`, reaching the snapshot and session check descriptions; `NodeState` gains no field.
- The capability statement's pin reads the edited `docs/capabilities.md`; its verb, cause and node-key counts are unmoved, and the line saying a closed select's contrast pair is unmeasured was removed there, though that pair stays unread.
- Pixel-test statements in test-strategy hold, but five frame hashes moved where a synthesized header stands on the emboldening build, and a shot of an open list now carries option and group labels as new pixels, with `fonts-unavailable` unmoved.
- Line citations below added ranges shift in `document.rs` (added 10-11, 343-344, 562, 761-1089, 1408-1410, 2404-2406), `stylo.rs` (1124-1139), `render.rs` (7, 246-247) and `ci-leg.sh` (59-60, shifting :82-87 and :90-91).
- Line citations also shift in `snapshot.rs` (147, 152-155, 245-246, 250-255), `host_log.rs` (17-20, 282-394), `mcp_log.rs` (12-15, 68-114) and `stand_snapshot_state.rs` (121-123, 131).
- The blitz-tests `.rs` targets move from 153 to 156; the report found no site stating this count in that wording.
- The `focussed node` log-line count is not settled: 0 lines through the sink on the workspace build with no base reading, so it reads neither moved nor unmoved, and no standing reading of it should be restated as moved.
- The registry contract `contracts/test-plan/session-lifecycle.md` moves with these counts at :6 and :11 (`stand-repeat`, `stand_act_*`, `cli_commands`) and with `type` focusing at :5.

## New facts
- F1 · unit-test-strategy · blitz-dom's `node/select_list.rs` holds six inline unit tests: where a list opens, its width bounds, every edge finite, `size` read as a capped whole number, scroll bounds with least movement, and a point reading its row under scroll.
- F2 · test-scope-summary · `select_popup`, 25 tests in both layout modes, no `stand_` prefix, reads the list's engine behaviour: opening, key and pointer moves, choosing, closing, inert, disabled, hidden and removed selects, the tree's `expanded`, clips, groups, the twenty-row cap, hostile markup, `size` on a `multiple` select and redraw requests.
- F3 · test-scope-summary · `select_popup_painted`, 6 tests, reads the list on frames: highlight and disabled row against their constants, the plain contrast pair, painting past clips and `opacity`, a closed frame byte-equal after a close with no choice, a header darker than a row by weight, and a thumb taking no hit; blitz-paint's `render/select_list.rs` holds no unit test of its own.
- F4 · test-scope-summary · `stand_act_popup`, 12 tests, drives the list through a held driver session: click and keys, Escape, a closed option and a button under the list refused `covered`, a row off screen until `scroll`, `type` leaving no list open, a row's shot, the lint, the snapshot budget, two clicks opening then closing, and a read with no `advance`.
- F5 · test-scope-summary · `stand_keyboard`'s closed-arrow test passes untouched, yet its Enter and Space rows now open and then choose, so it reads true but no longer reads what its row order suggests.
- F6 · test-scope-summary · `select_popup` skips the key press in its one row where an ancestor gains `hidden`, because a key press there panics at `node.rs:1775` in `position_in_containing_block` for any focused control, list or no list; no check reads that panic and its owner is left to this wrap's P5.
- F7 · test-scope-summary · The list's unmeasured limits stand stated: the windowed list read and not run, an assistive technology's reading `unrunnable-here`, a highlighted row's contrast unread by the lint, the highlight absent from snapshot and tree, the thumb not draggable, and a larger authored font clipping.
- F8 · ci-integration · No local gate builds `blitz-paint/font-embolden`, so `apple-font-embolden` turned CI red on macOS alone; the local witness for a paint change under that feature is `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`.
- F9 · ci-integration · `cargo` stops at the first failing target, so a red job's later targets go unread on that run.
- F10 · integration-test-strategy · `host_log` gains a `#[cfg(unix)]` test that opens and chooses in a list through the host at `trace` under state directory `hl-popup` with no option logged, and `mcp_log`'s popup test records nothing, leaving no `ml-popup` directory, since recording would have needed a `cfg!(unix)` site.
- F11 · e2e-test-strategy · `cli_commands`' `a_hosted_click_opens_a_list_a_shot_shows_it_and_a_click_on_a_row_chooses`, under state directory `cc-popup`, reads a hosted shot of an open list equal to the in-process one.
- F12 · test-data-fixtures · seven_guis' `tests/common/mod.rs` gains `option_needles()`, the needle list of a select's options read from a snapshot.
- F13 · mocking-stubbing-discipline · `select_popup` holds its own hand-written `CountingShell`, a `ShellProvider` counting `request_redraw`, reading that opening, moving, scrolling and closing each ask a redraw.
- F14 · unit-test-strategy · `schema_tests`, `refusal` and `json` pin the restated help of `click`, `type` and `press` and `covered`'s added clause, fixed `'static` text.
- F15 · new · `harness.center_of` on an inline element reads no box, so `select_popup` reads client rects through its own `client_point`; this belongs with the harness inspection helpers in §3, which has no detail file.

## Untouched
- WPT: the runner, its scoring, workflows and fixtures sit outside the changed files.
- apps/browser, blitz-vibey-script, blitz-html, blitz-net and wpt/runner tests: no file of theirs changed, so their test inventories and citations stand.
- The blitz-test-harness API: construction, pump, `settle`, `screenshot`, input and inspection helpers are unedited; only the crate's coverage reading moved.
- `tests/blitz-tests/tests/fixtures/mod.rs`, `session_common/mod.rs` and blitz-tests' `common/mod.rs` are unedited: their pages, `HeldNet`, builders and tables stand, and only reader counts moved.
- The agent-run contract: `scripts/agent-run.sh`'s verbs, exit grammar, events and state are unedited; `stand_act_popup` joins by the existing prefix rule.
- The cold-agent pipe and its recorded live run: `scripts/cold-agent.sh` is unedited and the run was not fired here, so `wrong_calls` stays unread and belongs to `v030-15`.
- The coverage floors of 97 %, 83 % and 90 % and `coverage_floor.py`: only the reading moved, not a floor.
- ci.yml's triggers, jobs, caches and package installs and `apt-install.sh` are unedited.
- The free-disk step's record at ci-integration:40 still reads unproven; the WATCH's count of 2/3 is route state, not test-plan text.
- Manifests and dev-dependencies: `Cargo.toml`, `Cargo.lock`, `deny.toml`, test-that and `BlitzTestsTargetsTest`'s one-target-per-file rule are unmoved.
- escher-telemetry's format and the `telemetry_*` checks: no log field or span was added and the sink is unedited.
- `tests/blitz-tests/checks/`, `stand_replay` and the recording flow: unedited.
- The mutation tier and `.config/nextest.toml`: unedited, and the report states no mutation reading.
- Citations into `node.rs` and `stand_actionable_keys.rs:153-155`: both files are unedited, and the latter's digits are not the target count 153.
- §11 Test Anti-Patterns and §12 Test Decisions Log hold no recorded intent, and the session records none for them.
