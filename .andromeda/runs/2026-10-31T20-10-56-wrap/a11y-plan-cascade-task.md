# Task — a11y-plan · 2026-10-31-select-popup

## The session
A closed single select now opens an engine-painted list of its options, over the page, in every document, chosen by pointer and by key.
Click, Enter and Space open it; keys and the pointer move a highlight; Enter, Space, Tab and a click on a row choose through the one write path `apply_select_choice`; Escape, a second click, a click outside and a page scroll close it with no event.
The tree now reads the list's state: a single select writes `expanded` from the open state, reads no `aria-expanded`, and focus stays on the select; the highlight is in neither the tree nor the snapshot.
A `multiple` select's `size` is now read, a group's `label` is painted as a header, and the limits that no popup is painted, that Enter and Space open nothing, that a group's label is unpainted and that a `multiple` select's `size` is unread are retired, with `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`.
Measured on CI#41235503227, green 17/17: the a11y leg 14 · 114 · 0 · 0, stand repeat 228 · 1155 · 0 · 21, SC 2.1.1, 2.4.3 and 1.4.3 asserted in the harness; what an assistive technology reads stays unmeasured and the windowed list was read and not run.
The report's Changes name 44 files and this list holds 40: `.gitignore` and the chunk's three evidence records are left out as carrying no rule.

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
- The retired limit that no popup is painted: any keyboard or scope text stating it (the report's sites: a11y-plan body line 123, a11y-scope-summary:23, keyboard-navigation:41).
- Enter and Space on a single select: they now open its list, and a released Space does nothing more; any text saying they open nothing or do nothing on a select (a11y-scope-summary:23, keyboard-navigation:41, :119).
- The Enter and Space activation rows: a focused single select now takes Enter on press, and Space on press with no typed text standing, ahead of the rule for other controls; the rule for buttons, summaries, links and inputs is unmoved.
- A pointer click on a select: a single select's click now focuses it and opens its list, or closes one standing, each click focusing; a row of an open list takes a click, while an option of a closed single select still takes none and a driver click on one still refuses `covered` (body line 113, keyboard-navigation:26).
- A click on a label bound to a single select: it focuses the select, opens nothing and no longer re-dispatches to it; `a_labels_click_focuses_the_select_and_opens_nothing` now asserts this (keyboard-navigation:55).
- The driver's `type` on a select: any select is now focused with no click, so no DOM focus events fire on a single select either, and a text opening on a space types nothing there (keyboard-navigation:48).
- `aria-expanded`: on a single select the attribute is no longer read, while every other element keeps the read, so the merged-state text needs that exception (body line 55); `:open` and `flush_open_state` are unmoved, so an author's `open` on a select steers nothing.
- The engine's changed set: opening and closing a list marks the select, a new kind of site; a page scroll or a move that closes a list marks it through `reconcile_select_list` at the next resolve, and a wheel elsewhere marks it through `close_select_list`; a highlight or a wheel move inside the list marks nothing; with no list open the old rule holds (body line 29, a11y-strategy:10).
- A group's `label`: it is now painted as a header on an open list, so text saying named in the tree and unpainted is stale (body line 202, aria-patterns-roles:21).
- A `multiple` select's `size`: it is now read as a height hint of `size` × 1.2em through `size_rows`, and a single select's `size` stays unread (a11y-scope-summary:23).
- `closed_select_of` in the bridge answers `None` for a select whose list is open, and the snapshot's builder reads `select_list_bounds` before the client rect, so an open row's snapshot bounds are its painted row's; text saying a single select's options read the select's box needs the open case (screen-reader-support:11, keyboard-navigation:26).
- The lint's select exclusion: `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group, and `clip_region` takes an open row's region from `visible_region` alone (visual-design-verification:16).
- Hit-testing: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, hover and pointerdown read the list; `hit_inner` is not edited, inert pass-through is unmoved, and the stated thumb hazard stands as written (visual-design-verification:85).
- `stand_contrast`'s `STATE_ROWS` 18 → 20, tally (19, 0, 1, 0): any text stating 18 (body line 64, visual-design-verification:12).
- `stand_keyboard` tests 21 → 23: any text stating 21 (a11y ci-integration:10, :11, keyboard-navigation:120).
- The a11y leg's tally 14 · 112 · 0 → 14 · 114 · 0, the leg itself staying fourteen files with the same names (a11y ci-integration:11).
- The per-OS CI record moved from CI#41192433140 to CI#41235503227: linux 217 · 1236 · 0 · 12, macos and coverage 198 · 1233 · 0 · 12, windows 198 · 1185 · 0 · 9, a11y 14 · 114 · 0 · 0, stand repeat 228 · 1155 · 0 · 21 (a11y ci-integration:11).
- The `stand-repeat` leg's named `--test` words fifteen → seventeen, adding `select_popup` and `select_popup_painted` (a11y ci-integration:9); `stand_act_popup` joins by its prefix.
- Stand-wide counts, wherever a11y-plan repeats them: `stand_act_*` files fifteen → sixteen, `stand_*.rs` 50 → 51, `run stand` 237 · 0 · 7, `run all` 727 · 0 · 12.
- The test inventory: `select_interaction::enter_and_space_open_nothing_on_a_single_select` is renamed `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`, and its `KEYS` table moved one row, Enter → `PageDown`, the count still 14.
- `stand_keyboard`'s closed-arrow test passes untouched, yet its Enter and Space rows now open and then choose; a description of what that walk reads needs that.
- Line citations into edited files: `accessibility.rs` after the insert at 238-242 (the HTML-AAM mapping citation moves; 146-156 and 149-150 sit above it and hold), every `stand_contrast.rs` citation (lines added from 12 on), `stand_snapshot_state.rs` after 121, and citations into `select_interaction.rs`, `stand_act_select.rs`, `document.rs`, `events/keyboard.rs`, `events/pointer.rs`, `scrolling.rs`, `stylo.rs`, `default.css`, `snapshot.rs` and `lint.rs` past their inserts; `stand_keyboard.rs` citations above its append at 1055 hold.
- Held claims a classifier should read as true where they name only these: a closed select's arrows, Home, End and typeahead; one `input` then one `change`, none for a key that changes nothing; `press_multiple_select_key` and `click_select_row`; the counts `select_interaction` 18 and `stand_act_select` 10; only citation digits may have moved.
- Indirect, through `docs/capabilities.md`: its line that a closed select's contrast pair is unmeasured is gone because the open list's plain pair is measured by `select_popup_painted`; the closed select's own pair stays unread, so any such a11y text must keep that split.
- Indirect, through `covered`'s meaning, `click`, `type` and `press` help: the driver's fixed text is restated; a11y text that restates the old help moves, while `VERBS` 12, causes 8 and `NODE_FIELDS` thirteen stand.
- Indirect, through a key press that panics at `packages/blitz-dom/src/node/node.rs:1775` when the focused control's anonymous block was dropped because an ancestor gained `hidden`, on a `select` or a plain `button`, with no list open: security-plan §Error Handling owns the row; flag a11y text only if a standing row claims such a key press is safe.

## New facts
- F1 · a11y-scope-summary · The select list is a new accessibility-scope entity: an engine overlay on every document, remote pages included, asserted for SC 2.1.1, 2.4.3 and 1.4.3 in the harness.
- F2 · keyboard-navigation · An open list takes its own closed key match, `press_open_list_key`: its move keys carry the highlight over enabled rows and choose nothing, and the list opens with the highlight on the chosen option, firing no event.
- F3 · keyboard-navigation · Enter, Space, Tab and a click on a row choose the highlighted or clicked row and close, one `input` then one `change`, and choosing the already chosen option fires nothing.
- F4 · keyboard-navigation · With a list open, Tab runs `choose_current_and_close` before moving focus, in either direction.
- F5 · keyboard-navigation · Escape, a second click, a click outside and the page scrolling each close the list with the value unchanged and no event, a wheel outside the list closes it and then scrolls, and a close clears the typed text.
- F6 · keyboard-navigation · A disabled row changes nothing and leaves the list open; a disabled, inert or hidden select, or one with no row to show, opens nothing; a select made inert, disabled, hidden or removed while open closes its list.
- F7 · keyboard-navigation · Focus stays on the select while its list is open, and the Tab walk holds with the list open and after each close, pinned by `the_flight_type_walk_holds_with_its_list_open_and_after_each_close` and `a_click_opens_the_list_focus_stays_on_the_select_and_the_walk_goes_on_from_it`.
- F8 · keyboard-navigation · A pointer over an enabled row moves the highlight and a later key goes on from it; a press on a row starts no text drag, and the release chooses the row under it.
- F9 · keyboard-navigation · A list shows at most `MAX_ROWS` 20 and scrolls by key and by wheel; a row brought into view writes the list's own `scroll_y`, moving no scroll chain or viewport.
- F10 · keyboard-navigation · The list's thumb is painted, takes no hit and is not draggable, so keys, the wheel and `scroll` are the ways to move a long list.
- F11 · keyboard-navigation · On a `multiple` select with a valid `size`, a key brings the current row into view through `scroll_into_view`, gated by `carries_valid_size`, and its other row semantics are unchanged.
- F12 · keyboard-navigation · New checks: `select_popup` 25, `select_popup_painted` 6 and `stand_act_popup` 12, in both layout modes, with `select_list` 6 unit tests, and `cli_commands`, `host_log` and `mcp_log` +1 each.
- F13 · aria-patterns-roles · A single select's pattern is `ComboBox` with `expanded` written from the list's open state and the chosen option `selected`.
- F14 · aria-patterns-roles · A group's header row in an open list chooses nothing, its options stand under it, and a group with no label reads the whole list's box.
- F15 · screen-reader-support · `the_tree_reads_the_list_open_on_the_select_its_focus_staying_there` and `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match` pin the tree's reading of a list; the builder takes `expanded` from a closed bool of engine state, no author attribute.
- F16 · screen-reader-support · The highlighted row is in no snapshot and no tree, a stated limit, so an agent or assistive technology reading either cannot see which row is current.
- F17 · screen-reader-support · No `expanded` node key exists: `NodeState` gained no field, so the open state reaches the tree and not the snapshot, on the operator's word.
- F18 · screen-reader-support · An option's label and a group's `label` leave an open list only as painted pixels in a `screenshot` and the snapshot's existing `name`, with no log site.
- F19 · screen-reader-support · What an assistive technology reads of an open list stays unmeasured, recorded as `unrunnable-here`, and the windowed list was read and not run.
- F20 · visual-design-verification · The list's colours are engine constants on `SelectListView`, not tokens: `HIGHLIGHT_TEXT` #000000, `HIGHLIGHT_BACKGROUND` #c8c8c8, `DISABLED_TEXT` #6d6d6d, `BORDER` #999999.
- F21 · visual-design-verification · `select_popup_painted` reads the highlight and a disabled row as painted against those constants and measures the open list's plain pair at the default sheet and at the stand.
- F22 · visual-design-verification · The lint reads no `overlap` on an open list's row, the planted pair still reads, `at_boot` stays `[0, 0, 0, 0]`, and a highlighted row's contrast is unread by the lint, a stated limit.
- F23 · visual-design-verification · A group's header is heavier than a row by a synthesized bold, on plain and emboldening builds, pinned by `a_groups_header_is_darker_than_a_row_by_weight`.
- F24 · visual-design-verification · The list has no motion and needs no `advance`: a screen read after a step reads the new state, pinned by `a_screen_read_after_a_step_reads_the_new_state_with_no_advance`.
- F25 · visual-design-verification · The list paints whole past an `opacity` ancestor and a box that clips its select, a point outside it reads what shows, and an element under it refuses `covered`.
- F26 · visual-design-verification · The row height is the select's font on one line, so a larger authored font clips, a stated limit.
- F27 · ci-integration · `select_popup`, `select_popup_painted` and `stand_act_popup` run in the workspace test leg and three rounds in `stand-repeat`, and none joins the a11y leg, though `stand_keyboard` carries the list's walk there.

## Untouched
- §1's Dioxus feature reach and the platform adapter's residual: no manifest moved, and `packages/accesskit_xplat`, `packages/blitz-shell` and `packages/dioxus-native` are outside the change, so an adapter still owed stays owed.
- §7's `Adapter` creation rule before a window is first shown: `packages/accesskit_xplat` is unedited.
- `contrast_ratio`, `required_ratio`, `Harness::text_contrast` and the 3.0 and 4.5 thresholds: `packages/blitz-test-harness` is unedited, and only a table's row count in `stand_contrast` moved.
- The harness's focused and hovered queries in `packages/blitz-test-harness/src/inspect.rs`: unedited, and no new harness query is required of a11y-plan.
- The inventories of `accessibility_hidden`, `accessibility_roles`, `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_active_descendant_unread`, `inert_subtree` and `focusability_updates`: none of these files was edited, so their test counts and citations stand.
- seven_guis CRUD and Cells keyboard rows: only `examples/seven_guis/tests` was edited, so the app's `crud-list` and `cells-grid` behaviour stands.
- §8 Cognitive Accessibility's invalid-date cue and its orientation cues: the flight booker's own source is not edited and no such cue moved.
- Language, direction and text-alternative rows over fixtures: no file under `examples/assets` was edited.
- NOT YET MEASURED and absence blocks on skip links, modal focus traps, route-change restoration, live regions, heading hierarchy, timeouts and plain language: the session measures none of these, and a list is not a modal trap.
