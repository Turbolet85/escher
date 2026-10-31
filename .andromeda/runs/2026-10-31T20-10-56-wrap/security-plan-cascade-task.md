# Task — security-plan · 2026-10-31-select-popup

## The session
A closed single select now opens an engine-painted list of its options, on every document including remote pages, chosen by pointer and key through the one write path `apply_select_choice`; 41 files changed against base `3af5a9bcf`, 5 of them new, no dependency, crate, carrier, listener, port, env var, log field or span added.
Added the attribute read `size` on a `multiple` select, native `expanded` on a single select, a painted group `label`, new pixel content in every shot of an open list, a row's own `bounds` in the snapshot and diff, and five public `BaseDocument` readers with two public read types.
Retired the limits that no popup is painted, that Enter and Space open nothing on a single select, that a group's label is unpainted and that a `multiple` select's `size` is unread; retired the names `Entry.in_closed_select`, `Target.multiple` and `focus_multiple_select`.
Measured a panic at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`) on a key press after an ancestor of the focused control gained `hidden`, reproduced on a `select` and a plain `button` with no list open, reachable from a remote page, not fixed here and with no owner placed yet.
The reached list below holds 40 of the 41 changed files, its bound: `.gitignore`, which the report assigns to route and incubator matters, and the chunk's three evidence records are left off.

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
- input-validation line 249, the select model's long row: the retired limits on the painted popup, Enter and Space and a `multiple` select's `size` end there and `type` is restated; its closed-option, changed-set and select-model clauses stay true.
- input-validation line 248: Enter opens a single select's list, and Space opens it when no typed text stands.
- input-validation line 256: a group's `label` is now painted as a header, and the clause reserving the `label` as a name ends on the founder's recorded fork (inputs#I4).
- input-validation line 210: a label's click on a bound single select focuses it and opens nothing, no longer re-dispatches, and is now asserted by `a_labels_click_focuses_the_select_and_opens_nothing`.
- input-validation line 219: a single select writes `set_expanded(open)` from engine state and reads no `aria-expanded`; every other element keeps that read.
- input-validation lines 261 and 262: `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and a text opening on a space types nothing there.
- input-validation lines 262, 264 and 265, the renames and readers: `Target.multiple` is `Target.select`; `focus_multiple_select` is `focus_select`, which also closes any list; new public `is_select`, called by `execute.rs` `locate` alone; `is_multiple_select` is called by `closed_select_of` alone and stays re-exported; `clear_select_typeahead` has the one non-test caller `focus_select`.
- input-validation lines 264 and 265, the closed-option reading: `closed_select_of` answers `None` for a select whose list is open; `Entry.in_closed_select` is `in_select_box`, true also for an open list's row or group; `clip_region` takes an open row's region from `visible_region` alone; an option of a closed select is still refused `covered`.
- Where security-plan reads `covered` (the report counts one site in its set) and the Error Handling refusal row: the meaning gains a clause naming an option of a closed select, with name, remedy and order unchanged, the text still fixed `'static`, the causes still eight.
- The Driver command schema rows: twelve verbs, seven argument kinds and `describe`'s count row `[12, 13, 12, 3, 8, 4, 15]` are unmoved; only the fixed help of `click`, `type` and `press` is restated.
- The Markup attributes row on in-process readers: Enter and Space now open a single select rather than only activating buttons and links; `size` joins the attributes read; option and group labels now also leave as pixels.
- The Markup numeric attributes row: `size` on a `multiple` select joins it, through `parse_unsigned_integer`, capped at 65534, the same ceiling the row states for `rowspan`.
- The Paint values row: the list paints last in `paint_scene`, before the devtools overlays, a non-finite box unpainted and only rows in the scrollport laid out.
- The Scrolling row: `visible_region` answers the list's scrollport for an open list's row or group; `scroll_into_view` on a row writes the list's `scroll_y` and moves no chain or viewport; a wheel scrolls the list over it, else closes the list and scrolls.
- threat-model-summary boundary-session-socket and boundary-mcp-stdio, and the socket's carrier rows in input-validation: the listed screenshots now hold an option's label and a group's label in every shot of an open list, and the snapshot and diff hold a row's own `bounds`; `NodeState` gains no field and `NODE_FIELDS` stays thirteen.
- The widening rule row: `differs` naming `bounds` is a closed set inside a ratified carrier and so no widening, while painted page labels on the screenshot carriers are the boundary widening that halts at P2 (inputs#I4, CARRY `cry-musoudbr`).
- threat-model-summary boundary-remote-content: remote pages reach the new list, its `size` read and the key-press panic.
- error-handling panic paths, the groups at lines 18, 24, 30 and 36: none holds the `position_in_containing_block` panic, so a row is owed; `packages/blitz-dom/src/node/node.rs` itself is unchanged, so its standing citations hold.
- error-handling graceful degradation: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, so `hit`, `set_hover_to` and `handle_pointerdown` read the list; `hit_inner` is not edited.
- Citations into `packages/blitz-dom/src/events/keyboard.rs` (`:50-68`, `:65`, `:505-510`) moved: lines were added at 3, 7, 32-34 and later, reaching Data Protection and the error-handling rows.
- Citations into `packages/blitz-dom/src/events/pointer.rs` (`:761-771`, `:1024-1030`, `:1054-1057`) moved: lines were added from 220 onward, reaching Logging & Monitoring and Data Protection.
- Citations into `packages/blitz-dom/src/document.rs` (`:610-613`, `:1612-1619`, `:1802-1823`, `:1807-1813`, `:1883`, `:2396-2403`) moved: lines were added at 10-11, 343-344, 562, 761-1089 and later, reaching error-handling and Logging & Monitoring.
- Citations into `packages/blitz-dom/src/resolve.rs` (`:258-262`, `:307-311`, `:327-352`) moved by the three lines at 133-135; `:40-47` sits above them and holds.
- The citations `packages/blitz-paint/src/render.rs:853-860` and `packages/dioxus-native-dom/src/snapshot.rs:266-293` moved, by additions at 7 and 246-247 and at 147, 152-155, 245-246 and 250-255.
- secret-management, the `CARGO_TARGET_TMPDIR` row: its citation `examples/seven_guis/tests/host_log.rs:27` moved by the lines at 17-20; the test-only state directories `cc-popup` and `hl-popup` join that directory, no env var added.
- dependency-security and supply chain, the citation `.github/scripts/ci-leg.sh:20-67`: `stand-repeat`'s list gains `select_popup` and `select_popup_painted` at 59-60, inside the range; `test_ci_workflows.py:367` asserts them.
- These citations sit above the additions and hold: `packages/blitz-dom/src/stylo.rs:446`, `packages/blitz-dom/src/lib.rs:26-29`, `examples/seven_guis/tests/common/mod.rs:17-19`, `.github/scripts/test_ci_workflows.py:186-192`.
- Data Protection, the masked file path row: the mask reading holds, since the snapshot's builder only reads `select_list_bounds(node)` before the client rect and `closed_select_of` answers `None` for an open list; only its citations move.
- The one `command` span row and Logging & Monitoring's escher sink rows hold, re-read through `stand_act_spans` at eight fields and two `info_span!` in `packages/escher-driver/src`.
- Re-read and unmoved: input-validation lines 56, 65, 200 and 203, and the select model's claims, `apply_select_choice` still the one path, `select_interaction` still 18 and `stand_act_select` still 10; only their citation digits can move.

## New facts
- F1 · error-handling · A key press panics at `packages/blitz-dom/src/node/node.rs:1775` (`position_in_containing_block`, through `handle_dom_event`'s `absolute_position`) when the focused control's anonymous block was dropped because an ancestor gained `hidden`; any control, no list needed, reachable by a remote page; predating this chunk rests on unchanged `node.rs` and `events/mod.rs` and the stack, with no controlled run at the base; `select_popup` skips the key press in that one row.
- F2 · error-handling · A select removed, disabled, hidden or made inert with its list open closes the list with no panic, `clear_interaction_state_for_removed_node` clearing the field, and an option dropped with the list open is no row.
- F3 · error-handling · The list's public readers go through `get_node`, answer `None` for a row with no styles and call no `layout_style()`, so they add no panic path.
- F4 · input-validation · The select list is one `BaseDocument` field, `select_list`, `None` at construction, read through `live_select_list`, except `reconcile_select_list` and the removal arm, which read the field past it, and `hover_list_row`, which writes `current` directly.
- F5 · input-validation · A disabled, inert or hidden select and a select with no row to show open no list; a disabled row changes nothing and leaves the list open; a group header chooses nothing; a non-invertible transform is held by the same check set.
- F6 · input-validation · An open list's keys pass one closed match, `press_open_list_key`; Tab runs `choose_current_and_close` before focus moves in either direction; `click_list_row` and `choose_current_and_close` write only through `apply_select_choice`.
- F7 · input-validation · In the list, a press starts no text drag and the release chooses the row under it; a press elsewhere closes through `close_list_outside`; a page scroll or a box move closes it through `reconcile_select_list` at the next resolve.
- F8 · input-validation · `size` on a `multiple` select is parsed by `size_rows` through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint, hinting `height` of `size` × 1.2em and carrying no text out; a single select's `size` stays unread.
- F9 · input-validation · Hostile markup is bounded and asserted by `hostile_markup_keeps_every_box_finite_and_panics_nowhere`: a hostile `size`, 2,000 options and a 5,500-character label; the list shows at most `MAX_ROWS` 20 rows.
- F10 · input-validation · An author's `open` or `aria-expanded` steers nothing on a select: `:open` and `flush_open_state` are unmoved, asserted by `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`.
- F11 · input-validation · The highlighted row crosses no structured carrier: it is in no snapshot and no tree, and the lint does not read its contrast.
- F12 · input-validation · New public surface: `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout`, `close_select_list`, and the read types `SelectListView` and `SelectListRow` re-exported from the crate root; `SelectListRow.label` holds page text and is never logged.
- F13 · threat-model-summary · An option's label and a group's `label` leave as pixels in every `screenshot` of an open list, on the socket and MCP carriers, a boundary widening for the founder's answer card at P2; the shot's fields are unmoved and `cli_commands` pins a hosted shot equal to an in-process one.
- F14 · threat-model-summary · The list crosses the ratified carriers only as a row's own `bounds` in the snapshot and diff and as pixels in a shot; there is no `expanded` node key, on the operator's word (inputs#I6).
- F15 · logging-monitoring · `host_log` and `mcp_log` each hold a test that opens and chooses in a list through the host or role and finds 0 option needles at `trace`; `mcp_log`'s records nothing and no `ml-popup` directory exists.
- F16 · logging-monitoring · The list adds no log site, span or field: the two new `select_list.rs` modules hold no tracing token by gate, and a label leaves only as a painted pixel and the snapshot's existing `name`.

## Untouched
- Authentication & Authorization: no handshake, token or peer check moved; the report names no new carrier, listener, port or session error.
- Dependency Security's audit tool, pins and the `serde_json`, `png` and `color` entries: `Cargo.toml`, `Cargo.lock`, `deny.toml` and every manifest are unmoved, and no package entered.
- dependency-security line 51: the free-disk step's record of CI#41192433140 stands as that run's record, the report's new CI run notwithstanding.
- Secret Management's storage, secret sources and what counts as secret: no secret or env var was read or added; only the test-directory row above moves.
- API Security's outbound controls and the absences it records in blitz-net: per-host limit, typed statuses, absent CORS, CSP, size limit and timeout; the list makes no request.
- The script execution vector and the script rows of Input Validation: `blitz-vibey-script` was not changed, and the list adds no script path.
- The net and fetched-resource rows of Input Validation, `file:` and `data:` URLs, fetched stylesheets, images, fonts and SVG data: the list fetches nothing, and blitz-net is not reached.
- The CLI rows for `agent-run.sh`, `cold-agent.sh`, `apt-install.sh` and `coverage_floor.py`: none of these scripts changed; the new `covered` clause's effect on the cold-agent run belongs to `v030-15`.
- Data Protection at rest, the history database, HTTP cache and the two committed checks: no medium moved, and no check gains a pixel.
- `escher-telemetry`'s allowlist and its field sets: the crate is not reached, and no log field was added.
- The upstream apps, the WPT runner, fixtures and the `.github/workflows` guards: no file of theirs changed.
