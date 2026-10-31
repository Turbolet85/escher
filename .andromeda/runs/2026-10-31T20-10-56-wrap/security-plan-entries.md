## 2026-10-31-select-popup — `covered`'s closed-option clause and the select verbs' help
**Section:** §Error Handling (error-handling.md: the refusal row)
**Change:**
- `covered`'s meaning was its two measured limits, a box clipped on one axis and a box with no width or no height; now also an option of a closed select, in the words `an option of a closed select lies under it: click the select first, then the option`, measured by `stand_act_popup::a_closed_option_stays_refused_covered_and_changes_nothing`.
- `click`'s help now says a click on a select opens its list; `press`'s states the keys on a closed select.
- `type`'s help now also says `type` on any select focuses with no click, so it fires no DOM focus events on a single select either, and that a text opening on a space types nothing there.
- Each stays a fixed literal of the verb table holding nothing a call supplies.
**Why:** an option of a closed select stays refused `covered` on the founder's F9, given by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator. The answer card reads the clause and the restated help as the program's own fixed text under the rule of 2026-10-12, outside the widening.
**Kept:** the cause set at eight and the cause names; the verb count and `describe`'s count row.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list's readers and its close add no panic
**Section:** §Error Handling (error-handling.md: new rows `Select list readers`, `Select list closing`)
**Change:**
- New row, readers: `BaseDocument`'s public readers `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` and its public writer `close_select_list` go through `get_node`, a row with no styles reading `None`, and call no `layout_style()`, so they add no panic path.
- New row, closing: a select removed, disabled, hidden or made inert with its list open closes the list with no panic (`a_select_made_inert_disabled_hidden_or_removed_with_its_list_open_closes`), `clear_interaction_state_for_removed_node` clearing the field for a removed select; an option dropped with the list open is no row (`an_option_dropped_with_the_list_open_is_no_row`).
- The closing row holds of the close alone: a key pressed after an ancestor gained `hidden` is the panic of the `Key press after a hidden ancestor` row.
**Why:** the list is a new surface on every document, remote pages included, and the chunk measured these paths; its security criterion was met to its letter with a defect beside it, which the closing row names so the row is not read past it.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the key-press panic after a hidden ancestor
**Section:** §Error Handling (error-handling.md: new row `Key press after a hidden ancestor`); §Threat Model Summary (threat-model-summary.md: the remote web content vector's trust boundary)
**Change:**
- The panic-path list was without this row; now a key press panics in `position_in_containing_block` (`node.rs`), the stack through `handle_dom_event`'s `absolute_position`, when the focused control's anonymous block was dropped because an ancestor gained `hidden`. Any control: reproduced on a `select` and a plain `button` with no list opened. A remote page can reach it.
- That it predates the select list rests on `node.rs` and `events/mod.rs` unchanged against the chunk's base and on the stack, no controlled run at that base taken. `select_popup` skips the key press in that one row.
- Not fixed. Owner: a CARRY on «Upstream sync ahead of the UI graph», the fix the founder's fork at that entry's P4.
- The threat model's remote content boundary now names the panic and points to Error Handling's panic paths.
**Why:** measured at implement and owed a row, the list holding none. The owner is the founder's answer Q2 on this wrap's answer card, the next upstream sync, by dialog 2026-10-31, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — citations moved with their lines
**Section:** §Error Handling (error-handling.md); §Input Validation (input-validation.md); §Logging & Monitoring (logging-monitoring.md); §Secret Management (secret-management.md); §Threat Model Summary (threat-model-summary.md); security-plan.md §Data Protection, §Dependency Security, §Secret Management, §Error Handling, §Logging & Monitoring
**Change:**
- Line citations into `document.rs`, `pointer.rs`, `keyboard.rs`, `accessibility.rs`, `scrolling.rs`, `stylo.rs`, `resolve.rs`, `snapshot.rs`, `render.rs`, `execute.rs`, `refusal.rs`, `schema.rs`, `select_interaction.rs`, `stand_act_select.rs`, `stand_snapshot_state.rs`, `cli_commands.rs`, `host_log.rs`, `mcp_log.rs` and `ci-leg.sh` now read where those lines stand; on the rows these digits alone moved on, no rule moved.
**Why:** the chunk's edits shifted the cited lines.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a shot of an open list holds labels as pixels
**Section:** §Input Validation (input-validation.md: Driver session socket, the `screenshot` clause); §Threat Model Summary (threat-model-summary.md: the session socket and MCP stdio vectors)
**Change:**
- The socket row's `screenshot` clause now adds: a shot of an open select list also holds as pixels an option's label and a group's `label`, page text in the pixels on this carrier, the shot's fields unmoved, a hosted shot pinned equal to an in-process one by `cli_commands`, a boundary widening ratified.
- The threat model's socket vector states the same of an answer's `screenshot`, on the socket and MCP carriers alike.
- The MCP vector states it of a result's `image` block, what a client does with it, a model provider included, being the client's.
**Why:** every shot of an open list carries new pixel content. Ratified by the founder on this wrap's answer card (Q1), by dialog 2026-10-31, relayed verbatim by the operator, over his forks F1 and F7, given by dialog in the overseer session that day and relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — what else of the list crosses the carriers
**Section:** §Input Validation (input-validation.md: Driver session socket, the state-readings clause; new row, an open select list's highlighted row); §Threat Model Summary (threat-model-summary.md: the session socket and MCP stdio vectors)
**Change:**
- An open select list's row now reads its own `bounds` in the snapshot and the diff, `differs` naming `bounds`, no widening; `NodeState` holds no field for it, `NODE_FIELDS` thirteen rows.
- New row: the highlighted row crosses no structured carrier, in no snapshot and no tree, and the lint does not read its contrast.
- The threat model's socket vector: the list crosses the ratified carriers only as a row's own `bounds` and as pixels in a shot, and no `expanded` node key stands. Its MCP vector: a snapshot or diff holds a row's own `bounds`, the crossing the socket group states.
**Why:** a closed set inside the ratified crossing is no widening under the founder's field rule of 2026-10-10, as the answer card reads it. No `expanded` node key is the operator's call, as the card records it.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `size` on a `multiple` select
**Section:** §Input Validation (input-validation.md: new row under Markup numeric attributes, the `select` row's limits; security-plan.md: the Markup numeric attributes line)
**Change:**
- New row, in every document, remote pages included: `size_rows` parses `size` through `parse_unsigned_integer`, capped at 65534, `0` or a non-numeric value giving no hint; a valid value hints a `height` of `size` × 1.2em, carrying no text out.
- `default.css` gives such a select `overflow-y: auto` and its rows `line-height: 1.2`; `apply_row_gesture` calls `scroll_into_view` on the current row only where `carries_valid_size`.
- Pinned by the unit `size_reads_a_whole_number_above_zero_capped` and, both layout modes, `hostile_markup_keeps_every_box_finite_and_panics_nowhere` and `size_makes_a_multiple_select_that_many_rows_tall_and_a_key_brings_the_next_in`.
- The `select` row's limit was `size` is not read; now a single select's `size` is not read.
- The master's line now names the read, capped at 65534 with `0` or a non-numeric value no hint, and states it an engine change for every document and a boundary widening, ratified.
**Why:** a new attribute read on a validated surface. The founder's F8, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the widening ratified by him on this wrap's answer card (Q1), by dialog that day, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a single select's `expanded`, and an author's `open` and `aria-expanded`
**Section:** §Input Validation (input-validation.md: `aria-expanded` · `aria-disabled` · `aria-level`; new row, an author's `open` and `aria-expanded` on a select)
**Change:**
- `aria-expanded` was a boolean on every element; now on every element but a single select, which writes `set_expanded(open)` from the engine's own state and reads no `aria-expanded`.
- New row, in every document, remote pages included: an author's `open` on a select and `aria-expanded` on a single select steer nothing; `flush_open_state` reads `open` on a `details` or a `dialog` alone, so `open` on a select does not match `:open`; no attribute text is carried out; pinned by `an_author_aria_expanded_and_open_steer_nothing_on_a_select_and_open_does_not_match`, both layout modes; an engine change for every document.
**Why:** the tree's reading is now the engine's own state. The answer card reads a single select's native `expanded` as a closed bool of engine state inside the states class, outside the widening.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a label's click on a bound single select
**Section:** §Input Validation (input-validation.md: `aria-label` · `<label for>`)
**Change:**
- Was a click on a select's label re-dispatches to the select; now a click on a label re-dispatches to its select, but for a bound single select, whose label's click focuses it and opens nothing, asserted by `select_popup`'s `a_labels_click_focuses_the_select_and_opens_nothing`.
**Why:** measured: the single-select half no longer re-dispatches and is now asserted. The answer card holds a label's click opening nothing among the leans shown to the founder, to which he did not object, outside the widening.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — which select opens a list, what a row takes, and hostile markup
**Section:** §Input Validation (input-validation.md: new rows, which select opens a list and what a row takes; hostile markup under an open select list)
**Change:**
- New row, in every document, remote pages included: a disabled, inert or hidden select and a select with no row to show open no list, and a non-invertible transform is held by the same check set, `select_list_allowed`; a disabled row changes nothing and leaves the list open; a group header chooses nothing. Pinned by `a_disabled_inert_hidden_singular_or_empty_select_opens_nothing`, `a_disabled_row_changes_nothing_and_leaves_the_list_open` and `groups_read_as_headers_their_options_under_them_and_a_header_chooses_nothing`, both layout modes.
- New row, in every document, remote pages included: a hostile `size`, 2,000 options and a 5,500-character label keep every box finite and panic nowhere, asserted by `hostile_markup_keeps_every_box_finite_and_panics_nowhere`; the list shows at most `MAX_ROWS` 20 rows, those past that scrolled, pinned by `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel`, both layout modes.
- Each an engine change for every document and a boundary widening, ratified.
**Why:** the list admits a new input class on every document, remote pages included. Ratified by the founder on this wrap's answer card (Q1), by dialog 2026-10-31, relayed verbatim by the operator, over his forks, given by dialog in the overseer session that day and relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the `select` row: Enter, Space and a click open the list
**Section:** §Input Validation (input-validation.md: keyboard activation and a button click's focus; `select` (single-choice))
**Change:**
- Keyboard activation row: a click on a `select` focusing it now also, on a single select, opens its list or closes one standing; a click on an open list's row reaches `click_list_row`, and the `option` arm covers the select list's rows beside the `select` with `multiple` row.
- `select` row: its keys now read with its list closed. Was Enter and Space open nothing; now Enter opens its list and a Space with no typed text standing opens it (`press_select_key`).
- `select` row: a click now focuses it and on a single select opens its list or closes one standing.
- Retired limits: «no popup is painted» and «Enter and Space open nothing on a single select».
**Why:** the chunk built the list, retiring both limits. The founder's F1, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the row's widening ratified by him on this wrap's answer card (Q1), by dialog that day, relayed verbatim by the operator.
**Kept:** an option of a closed single select has no click arm and stays refused `covered`, under a group too, on his F9.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list's state, keys, pointer and close
**Section:** §Input Validation (input-validation.md: new rows, the select list's state; an open select list's keys; a pointer in and around an open select list, and what closes it)
**Change:**
- State, in every document, remote pages included: one `BaseDocument` field, `select_list`, `None` at construction, read through `live_select_list` but for `reconcile_select_list`, which must see a list that reader no longer answers, and the removal arm, each reading the field past it, and `hover_list_row`, which writes `current` directly; an engine change for every document.
- Keys: one closed match, `press_open_list_key`; Tab runs `choose_current_and_close` before focus moves, in either direction; `click_list_row` and `choose_current_and_close` write only through `apply_select_choice`, the one write path. Pinned by `the_keys_move_the_highlight_over_enabled_rows_and_choose_nothing`, `enter_space_tab_and_a_click_choose_the_row_one_input_then_one_change` and `tab_chooses_and_closes_before_focus_moves_either_way`, both layout modes.
- Pointer: in the list a press starts no text drag and the release chooses the row under it; a press elsewhere closes it through `close_list_outside`; a page scroll or a box move closes it through `reconcile_select_list` at the next resolve, the `fresh` field keeping a list a click just opened through a `:focus` rule's restyle. Pinned by `a_press_on_a_row_selects_no_text_and_a_drag_across_rows_chooses_where_it_ends`, `escape_a_second_click_a_click_outside_and_the_page_scrolling_close_with_no_event` and `a_focus_rule_that_moves_the_selects_box_leaves_the_list_a_click_opened`, both layout modes.
- The keys and pointer rows each an engine change for every document and a boundary widening, ratified.
**Why:** the founder's F2 and F4, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the widening ratified by him on this wrap's answer card (Q1), by dialog that day, relayed verbatim by the operator. The page's scroll closing stands on the card among the leans shown to him, to which he did not object.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `optgroup`'s `label` painted as a header
**Section:** §Input Validation (input-validation.md: `optgroup`, its `label`)
**Change:**
- A group's non-empty `label` was its name in the tree, unpainted; now its name in the tree and a header painted in an open list, leaving also as pixels in a `screenshot`, a boundary widening ratified.
- Its pins gain `select_popup` and `select_popup_painted`.
- Ratification was as to the list through a group and not as to the `label` as a name; now as to both.
**Why:** the founder's F7, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator, whose question told him the answer settles a group's name in the tree too; the widening ratified by him on this wrap's answer card (Q1), by dialog that day, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the driver's `type` on any select
**Section:** §Input Validation (input-validation.md: the driver's `type` row; the `select` row's `type` clause)
**Change:**
- The row was the driver's `type` on a `multiple` select; now on any select.
- `focus_multiple_select` is now `focus_select`, which focuses any select with no click, so `type` fires no DOM focus events on a single select either; it clears the typeahead and closes any list.
- A text opening on a space was typing nothing on a `multiple` select; now on any select.
- `Target` held `multiple`; now holds `select`.
- The `select` row: `type` on a single select is the same typeahead, now reached through `focus_select` with no click, firing no DOM focus events and leaving no list open, a text opening on a space typing nothing there; its help fixed text.
**Why:** the chunk renamed the field and the function and moved every select onto the one path. The answer card reads `type`'s restated help as the program's own fixed text under the rule of 2026-10-12.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select readers and the list's public surface
**Section:** §Input Validation (input-validation.md: the public select readers; new row, the select list's public surface)
**Change:**
- The readers now include `dioxus_native_dom::is_select`, its one caller `execute.rs` `locate`; `is_multiple_select`'s one caller within the workspace is `closed_select_of`, its re-export standing; `clear_select_typeahead`'s one non-test caller is `focus_select`.
- `closed_select_of` now answers `None` for a select whose list is open; `lint.rs`'s `Entry.in_select_box` reads true also for an open list's row or group, `clip_region` taking an open row's region from `visible_region` alone, an option of a closed select still refused `covered`.
- New row: `BaseDocument::open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` are read-only and `close_select_list` the one public writer, its callers keyboard, pointer, reconcile and the driver's `focus_select`; the read types `SelectListView` and `SelectListRow` are re-exported from the crate root; `SelectListRow.label` holds page text and is never logged.
**Why:** the list's readers are new public surface, and paint needs a row's fields, hence two read types. An option of a closed select stays `covered` on the founder's F9 and the lint's reading of an open list's rows is his F10, each by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list's visible region, paint and scroll
**Section:** §Input Validation (input-validation.md: a clipping box's visible region; new rows under Paint values and Scrolling)
**Change:**
- `visible_region` now, for a row or a group of an open select list, answers the list's scrollport; `lint.rs` `clip_region` takes an open row's region from it alone, no `contain: paint` cut.
- New row, Paint values: `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays; a non-finite box is left unpainted and only rows in the scrollport are laid out; colours are the engine's own constants, no page value.
- New row, Scrolling: `scroll_into_view` on a row writes the list's `scroll_y` at once and returns, moving no chain or viewport; a wheel over the list scrolls it, and a wheel elsewhere closes it and then scrolls; its offset stays within its rows (`Geometry::clamped`). Pinned by `a_scroll_stays_within_the_rows_and_brings_a_row_in_by_the_least_movement` and `a_long_list_shows_twenty_rows_and_scrolls_by_key_and_by_wheel`.
- Each new row an engine change for every document and a boundary widening, ratified.
**Why:** the list paints and scrolls apart from the page's boxes. The two rows stand with the widening the founder ratified on this wrap's answer card (Q1), by dialog 2026-10-31, relayed verbatim by the operator. The colours as engine constants are his F6, by dialog in the overseer session that day, relayed verbatim by the operator, recorded on the card without a widening.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list in the log
**Section:** §Logging & Monitoring (logging-monitoring.md: new rows, a select list through the host and the role; the select list's silence)
**Change:**
- New row: `host_log` and `mcp_log` each hold a test that opens a select's list and chooses in it, through the host or the `mcp` role, with 0 option needles at `RUST_LOG=trace`; `host_log`'s runs in the state directory `hl-popup` under `#[cfg(unix)]`, as every host-driving test there does; `mcp_log`'s records nothing and no `ml-popup` directory exists.
- New row: the list holds no log site, span or field. blitz-dom's `node::select_list` and blitz-paint's `render::select_list` hold no tracing token by gate, `packages/escher-driver/src` holds two `info_span!`, `stand_act_spans` reads eight span fields, and an option's or a group's label leaves only as a painted pixel and the snapshot's existing `name`, `SelectListRow.label` never logged.
**Why:** measured: page text in the list reaches no log, under the host and under the role.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a page's markup reaches the select list
**Section:** §Threat Model Summary (threat-model-summary.md: the remote web content vector's trust boundary); §Input Validation (security-plan.md: the Markup attributes line)
**Change:**
- The threat model now says a page's markup, a remote page's included, reaches the engine-painted select list on every document: its option labels as the list's rows, its group labels as their headers, and `size` on a `multiple` select as a capped whole number, each held under Input Validation.
- The master's Markup attributes line now names Enter and Space opening a single select's engine-painted list, Space only with no typed text standing, and an option's and a group's label also painted in an open list; the list's keys and painted labels are an engine change for every document and a boundary widening, ratified.
**Why:** the list is an engine overlay on every document, remote pages included. The founder's forks, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator; the widening ratified by him on this wrap's answer card (Q1), by dialog that day, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
