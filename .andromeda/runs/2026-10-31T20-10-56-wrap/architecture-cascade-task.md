# Task — architecture · 2026-10-31-select-popup

## The session
A closed single select now opens a list the engine paints over the page, chosen by pointer and key, in every document; the state, hit, keys, tree, paint, bridge and driver all moved, against base `3af5a9bcf`, commits `8cd9c1286` and `715798ce2`.
Retired: the limits no popup is painted, Enter and Space open nothing on a single select, a group's label is unpainted, a `multiple` select's `size` is unread; also `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`.
Added: modules blitz-dom `node::select_list` and blitz-paint `render::select_list`, checks `select_popup` (25), `select_popup_painted` (6), `stand_act_popup` (12); no crate, dependency, verb, cause, node key, carrier, listener, port, env var, log field or span added.
Measured: CI#41235503227 green 17/17 at `715798ce2`, `run stand` 237 pass 0 fail 7 ignored, `run all` 727 pass 0 fail 12 ignored; `stand_*.rs` 50 to 51, `stand_act_*` 15 to 16, blitz-tests targets 153 to 156, `stand_keyboard` 21 to 23, `cli_commands` 14 to 15, `STATE_ROWS` 18 to 20, the a11y leg 14 files with 112 to 114 tests.
Disproved: the changed set is written by no scroll or layout is false where a list stands; a key press panics at `packages/blitz-dom/src/node/node.rs:1775` on a focused control whose anonymous block was dropped under an ancestor gaining `hidden`, for any control with no list open, in a file this chunk did not edit.
41 files changed; Reached files below holds 40, the list's cap, leaving out `tests/blitz-tests/tests/stand_id_persistence.rs`, a one-line edit at :98. The chunk's three evidence records under `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/` are records, not among the 41.

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
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_snapshot_state.rs
- examples/seven_guis/tests/cli_commands.rs
- examples/seven_guis/tests/host_log.rs
- examples/seven_guis/tests/mcp_log.rs
- examples/seven_guis/tests/common/mod.rs
- docs/capabilities.md
- .gitignore
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py

## Touches
- dec-scrolling-and-input, the select model (established-decisions:78): no popup painted, Enter and Space opening nothing, and `size` unread all end; a single select's `size` stays unread and only a `multiple` select's is read.
- dec-scrolling-and-input, the select model re-read and still true: an option of a closed select, under a group too, takes no click and stays refused `covered`; `apply_select_choice` remains the one path; Ctrl+A stays unmodelled.
- dec-scrolling-and-input, the tally of direct engine changes for every document and their merge-surface line: the list is one more such change; `hit_inner` was not edited, so its conflict note with upstream `0980cce5f` is unmoved.
- dec-scrolling-and-input, the scrolling rules: a wheel over an open list scrolls the list, elsewhere it closes the list and then scrolls; `scroll_into_view` writing every box on the chain now has an exception for a row of an open list.
- established-decisions:88, the scrollbar thumb's neighbouring clause: a list row is never a thumb drag; the thumb hazard itself stands.
- dec-dom-semantics, established-decisions:121: a group's `label` is now painted as a header, ending any statement that it is unpainted.
- dec-driver-session, a new carrier or exit widens the boundary: the list adds no carrier, but every shot of an open list now holds an option's and a group's label as pixels, a boundary widening.
- contract-document-core, standard-contracts:9 and :18: open and close write the changed set, a new kind of site; `clear_select_typeahead`'s one non-test caller is now the driver's `focus_select`.
- contract-mutation-query-and-cssom, standard-contracts:26 and :30: group labels now also paint; a single select's `expanded` comes from engine state, and the state-attributes-read-once rule keeps its `aria-expanded` read on every other element only.
- contract-events: a choice from the list goes through the same `input` then `change`; Enter and a released Space on a single select now open or choose rather than doing nothing, and a press in the list starts no text drag.
- contract-scrolling-selection-tree, standard-contracts:49, :51 and :61: `hit_with_scrollbar` answers the list first; `visible_region`, the one reader of what can be seen, answers the list's scrollport for a row or group of an open list.
- contract-nodes-and-widgets: the select's rules in crate-private `node::select` with no public type now stand beside `node::select_list`, which has two public read types; `row_disabled` joins the disabled readings.
- contract-paint-and-shell, standard-contracts:105: `paint_scene` now paints the list after the root and before the devtools overlays.
- contract-dioxus-dom-bridge, standard-contracts:279: `closed_select_of` answers `None` for a select whose list is open, `is_multiple_select` is reached only by `closed_select_of` within the workspace though still re-exported, and the driver's select test is now `is_select`.
- contract-driver-session, standard-contracts:178: `covered`'s meaning gains the clause that an option of a closed select lies under it, click the select first; name, remedy and order unchanged.
- contract-driver-session, standard-contracts:189 and :190: `Target.multiple` is now `Target.select` and `focus_multiple_select` is now `focus_select`; `type` on any select focuses with no click; the fixed help of `click`, `type` and `press` is restated.
- contract-driver-session, standard-contracts:212, :213 and :220: `Entry.in_closed_select` is now `in_select_box`, true also for an open list's row or group; the `overlap` exclusion is restated.
- contract-driver-session and contract-clis, re-read and unmoved though their files were edited: twelve verbs, eight causes, the `describe` count row `[12, 13, 12, 3, 8, 4, 15]`, `NODE_FIELDS` thirteen; `type` on a target that cannot take it (project-intent:33, standard-contracts:188); the shared `cli_commands` token at standard-contracts:185.
- contract-ci-contracts, standard-contracts:311 and :312: `stand-repeat`'s named `--test` words go fifteen to seventeen at `ci-leg.sh:59-60`, asserted at `test_ci_workflows.py:367`; `stand_act_popup` joins by its prefix; the a11y leg stays fourteen files; `select_interaction` 18 and `stand_act_select` 10 re-read unmoved.
- crosscut-invalidation-and-state-integrity, the changed-set sentence (standard-contracts:9's rule stated at the Cross-cutting body): four kinds of site and no scroll or layout no longer hold where a list stands; with no list open it holds.
- crosscut-invalidation-and-state-integrity, interaction state on a removed node: the list is new such state, cleared in `clear_interaction_state_for_removed_node`.
- crosscut-service-wiring, the list of what requests a redraw: opening, moving, scrolling and closing a list each ask one, indirectly through `opening_moving_scrolling_and_closing_each_ask_a_redraw`.
- existing-scopes:76, :106, :112, :113 and :116: the bridge and `focus_select` wording; `stand_keyboard` 21 to 23; Enter and Space on a single select; `STATE_ROWS` 18 to 20 with tally 19, 0, 1, 0; `stand_act_*` fifteen to sixteen.
- conventions:54: the count of `stand_act_*` files goes fifteen to sixteen.
- conv-tests and scopes, indirect, any count these three new targets move: blitz-tests targets 153 to 156, `stand_*.rs` 50 to 51, `mod session_common;` declarers 31 to 32, `mod fixtures;` declarers 28 to 31; digits such as `stand_actionable_keys.rs:153-155` are citations, not counts.
- scopes and conv-visibility-and-naming, indirect: blitz-dom by area and paint rendering split one module per concern each gain a `select_list` module.
- Runtime threading, the `resolve` order: `reconcile_select_list` is a new step after layout and before the hover refresh, at `resolve.rs:133-135`, moving that citation's end.
- Line citations into edited engine files, moved past these insertions: `document.rs` at 10-11, 343-344, 562, 761-1089, 1408-1410, 2404-2406; `stylo.rs` at 1124-1139; `keyboard.rs` at 3, 7, 32-34; `pointer.rs` at 220, 223-226, 415-420, 595-656; `resolve.rs` at 133-135; `render.rs` at 7.
- `.github/scripts/ci-leg.sh:18-69`, cited in the CI/CD pattern, grows by the two lines at 59-60; citations above line 59 are unmoved.

## New facts
- F1 · established-decisions · A single select opens a list the engine paints over the page by a click, by Enter, or by a Space with no typed text standing, a direct engine change for every document, remote pages included.
- F2 · established-decisions · An open list closes with no event on Escape, a second click on the select, a press outside, a wheel outside the list and a page scroll or move of the select's box; removal, `disabled`, `hidden` or `inert` on the select closes it too.
- F3 · established-decisions · Tab chooses the current row and closes before focus moves, in either direction; Enter, Space and a click on a row choose through `apply_select_choice`, and a disabled row changes nothing and leaves the list open.
- F4 · established-decisions · Rejected and unbuilt: `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, and the list's colours in `default.css`.
- F5 · established-decisions · The list's standing limits: the thumb is painted and not draggable, a highlighted row's contrast is unread by the lint, a larger authored font clips in the one row height, the highlight is absent from the snapshot and the tree, and a single select's `size` is unread.
- F6 · established-decisions · A `multiple` select's `size` hints a `height` of `size` times 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint, with `select[multiple][size]` taking `overflow-y: auto` and its rows `line-height: 1.2` in `default.css:183-193`.
- F7 · established-decisions · A group's `label` paints as a header heavier than a row on every build by a synthesized bold, a header chooses nothing, and a group with no label reads the whole list's box.
- F8 · established-decisions · The list's merge surface is sixteen files, recorded in `escher-0.3.0/chunks/2026-10-31-select-popup/evidence/merge-surface.md`, a line owned by Epoch 3's sync.
- F9 · established-decisions · The list crosses the ratified carriers only as a row's own `bounds` in the snapshot and diff and as pixels in a `screenshot`; labels leaving as pixels is a boundary widening recorded for the founder's answer.
- F10 · standard-contracts · `BaseDocument` holds one field `select_list: Option<SelectList>`, `None` at construction; public read-only `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout`, public writer `close_select_list`, the rest crate-private.
- F11 · standard-contracts · `live_select_list` is the reader of the list's state, with two exceptions reading the field past it, `reconcile_select_list` and the removal arm, while `hover_list_row` writes `current` directly.
- F12 · standard-contracts · `SelectListView` and `SelectListRow` are public read types re-exported from blitz-dom's crate root, with the list's colours as `SelectListView` associated constants `HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT` and `BORDER`, engine constants and no tokens.
- F13 · standard-contracts · `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, the driver's `covered`, `set_hover_to` and `handle_pointerdown` all read the list.
- F14 · standard-contracts · `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`, and `clear_interaction_state_for_removed_node` clears the field for its select.
- F15 · standard-contracts · `scroll_into_view` on a row of an open list writes the list's `scroll_y` at once and returns, moving no chain and no viewport.
- F16 · standard-contracts · `paint_select_list` paints a non-finite box not at all, lays out only rows in the scrollport, and draws its thumb only under `feature = "scrollbars"`.
- F17 · standard-contracts · A single select writes `set_expanded(open)` and reads no `aria-expanded`, and an author's `open` on a select steers nothing, since `:open` and `flush_open_state` are unmoved.
- F18 · standard-contracts · dioxus-native-dom gains public re-exported `is_select`, and the snapshot's builder reads `select_list_bounds(node)` before the client rect for a row.
- F19 · standard-contracts · The snapshot carries no `expanded` node key, by the operator's call, and `NodeState` gains no field for the list.
- F20 · standard-contracts · The driver's `focus_select` also closes any list, and `type` on a single select fires no DOM focus events, a text opening on a space typing nothing there.
- F21 · standard-contracts · `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut.
- F22 · standard-contracts · A label's click focuses a bound single select and opens nothing, no longer re-dispatching, asserted by `a_labels_click_focuses_the_select_and_opens_nothing`.
- F23 · standard-contracts · `SelectList.fresh` holds a just-opened list against a `:focus` rule's restyle, so a box-moving rule cannot close the list a click opened.
- F24 · standard-contracts · The list shows at most `MAX_ROWS` 20 rows, opens where its rows fit and else on the side with more room, is at least its select's width and at most the viewport's, and a select with no row to show opens nothing.
- F25 · standard-contracts · No local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises; `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness for a paint change under that feature.
- F26 · cross-cutting-patterns · Open and close mark the select in the changed set, a page scroll or move that closes a list marks it through `reconcile_select_list` at the next resolve, a wheel elsewhere marks it through `close_select_list`, and a highlight or a wheel move inside the list marks nothing.
- F27 · existing-scopes · blitz-dom's `node::select_list` holds the list's state and geometry with 6 unit tests, blitz-paint's `render::select_list` paints it, and `select_popup`, `select_popup_painted` and `stand_act_popup` are its checks.
- F28 · occupied-resources · Test-only state directories `cc-popup` (`cli_commands.rs:442`) and `hl-popup` (`host_log.rs:296`) sit under `CARGO_TARGET_TMPDIR`; no `ml-popup` exists, `mcp_log`'s popup test recording nothing.

## Untouched
- stack, dec-dependency-pinning, conv-manifests and defaults: no dependency or manifest moved; `Cargo.toml`, `Cargo.lock` and `deny.toml` are unchanged.
- dec-default-features, established-decisions:32: the feature names stand, no manifest moved and no default list changed.
- dec-build-profiles, dec-release-versioning and dec-renderer-backends: no profile, version or renderer selection moved.
- dec-shell-and-windowing, contract-high-level-launch and the platform-tree refresh: `packages/blitz-shell` and `packages/dioxus-native` are absent from the changed files.
- contract-headless-stand: `examples/seven_guis/src` was not edited, only tests beside it; six served tasks and the stand's boots stand.
- contract-test-harness: `packages/blitz-test-harness` was not edited.
- contract-telemetry-bootstrap: escher-telemetry was not edited, and no log field or sink moved.
- res-network-ports-and-listeners, res-outbound-hosts, res-environment-variables and res-names: no port, listener, host, env var or binary added.
- res-ci-infrastructure, occupied-resources:98: the free-disk step's record of its run stands; only a WATCH count outside this document moved.
- dec-javascript and contract-javascript: blitz-vibey-script was not edited.
- dec-browser-app, contract-browser-app, dec-wpt-runner and contract-wpt-runner-outputs: the browser, rdme and the WPT runner were not edited.
- dec-layout and contract-layout: the list is no layout box, and no layout module, Taffy trait or table path moved.
- intent, the Upstream sync line: no merge or cherry-pick happened, so the merge base and the upstream read are unmoved.
- `Node::absolute_position` in contract-scrolling-selection-tree, standard-contracts:51: the panic at `node.rs:1775` is owed to security-plan's Error Handling and an owner at P5; the sentence describing the reader stands.
