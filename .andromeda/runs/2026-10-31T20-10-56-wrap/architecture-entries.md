## 2026-10-31-select-popup — citations re-pointed under the moved code
**Section:** every file of the set: architecture-detail/conventions.md, cross-cutting-patterns.md, design-philosophy.md, established-decisions.md, existing-scopes.md, inherited-defaults.md, occupied-resources.md, project-intent.md, stack-and-technologies.md, standard-contracts.md; architecture.md
**Change:**
- Line citations re-pointed where the code under them moved, the claim beside each unchanged. inherited-defaults.md and project-intent.md take this change and no other.
**Why:** the chunk's edits moved the cited code. A cited range that no longer held its claim was raised, and that claim's rewrite stands under its subject's entry here.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list's checks and the counts they moved
**Section:** §Conventions (conventions.md: the driver-action checks); §Existing Scopes (existing-scopes.md: blitz-tests, its shared modules); §Standard Contracts → CI contracts (standard-contracts.md: the `stand-repeat` leg)
**Change:**
- The driver-action checks `stand_act_*`: was fifteen; now sixteen (conventions.md, existing-scopes.md). The sixteenth is `stand_act_popup`, 12 tests in both layout modes, in `stand-repeat` by its prefix.
- New in existing-scopes.md: the engine checks `select_popup`, 25 tests, and `select_popup_painted`, 6 tests, both in both layout modes, named `--test`s of `stand-repeat`, outside the `a11y` leg's fourteen files.
- `stand_keyboard`: was 21 tests; now 23, the two new ones the flight type walk with the list open and after each close, and a click opening the list with focus staying on the select.
- `select_interaction`'s row: was Enter and Space opening nothing on a single select; now opening its list.
- `STATE_ROWS`: was 18, 17 pass and 1 exempt; now 20, 19 pass and 1 exempt.
- Declarers of `tests/session_common/mod.rs`: was 31; now 32. Of `tests/fixtures/mod.rs`: was 28, 22 `stand_*` files; now 31, 23 `stand_*` files with `stand_act_popup`, plus `select_popup` and `select_popup_painted`. Both read as measured at this chunk's report.
- `stand-repeat`'s named targets outside the prefix: was fifteen; now seventeen, `test_stand_repeat_leg_runs_three_rounds` asserting `select_popup` and `select_popup_painted` among them.
**Why:** the chunk added three check files and two `stand_keyboard` tests; each count is its report's measurement. The fixtures breakdown was read at this wrap by `git grep` on the three new files.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list in the redraws and the changed set
**Section:** §Cross-cutting Patterns (cross-cutting-patterns.md: Service wiring, Invalidation and state integrity; architecture.md: Invalidation and state integrity); §Standard Contracts → Document core (standard-contracts.md: the changed set's pointer)
**Change:**
- Service wiring: `ShellProvider::request_redraw` is also asked by a select list opening, moving, scrolling or closing.
- The changed set is written at: was four kinds of site; now five. The fifth is a select's list: its opening or closing marks the select, a page scroll or a move of its box that closes the list marks it through `reconcile_select_list` at the next resolve, a wheel elsewhere marks it through `close_select_list`, and a highlight or a wheel move inside the list marks nothing.
- Was: written by no node creation, hover, scroll, resize, load, animation or layout, without condition; now: that holds with no select list standing.
- The `apply_select_choice` site: was a click on a row of a `multiple` select; now also of a single select's open list.
- Document core's pointer to the changed set names a list's open and close.
**Why:** the report disproved the unconditional claim by measurement: false where a list stands, true with none open, a highlight or a wheel inside marking nothing.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list's paint and a group's header
**Section:** §Design Philosophy (design-philosophy.md: the modular engine); §Established Decisions (established-decisions.md: `optgroup`); §Existing Scopes (existing-scopes.md: blitz-paint's render); §Standard Contracts → Mutation, query and CSSOM, Nodes and widgets, Paint and shell (standard-contracts.md; architecture.md: Paint and shell)
**Change:**
- `paint_scene` paints an open select's list last through `paint_select_list`, after the root's render and before the devtools overlays, read from `SelectListView` and past any clip or `opacity` on its select's ancestors, reaching a shell or renderer through `paint_scene` alone.
- The carve-out to paint reading resolved layout: a row's text is laid out at paint through the public `select_row_layout`, only the rows in the scrollport.
- `render::select_list`, a child module of `render`: a non-finite box unpainted, the thumb drawn only under the `scrollbars` feature.
- `stroke_text` delegates to the `pub(crate)` `stroke_text_as`, taking `RunPaint { color, synthesize_bold }`, every other caller passing `RunPaint::default()`; a synthesized bold adds one embolden weight to a build's own.
- A group's `label`: was unpainted; now an open list paints it as a header heavier than a row on every build. A header chooses nothing, and a group with no label reads the whole list's box. It stays named in the tree.
- The list's colours are `SelectListView` associated constants, `HIGHLIGHT_TEXT` `#000000`, `HIGHLIGHT_BACKGROUND` `#c8c8c8`, `DISABLED_TEXT` `#6d6d6d`, `BORDER` `#999999`: engine constants, no tokens.
**Why:** the founder's forks F6 (the list's colours as a select's) and F7 (a group's label drawn as a header, which also settles its name in the tree), his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4). Heavier on every build is the operator's direction past implement, after one embolden vector for both arms read a header no heavier than a row on an emboldening build and `Test (macos)` went red.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — an open list's exceptions to scrolling and the hit
**Section:** §Design Philosophy (design-philosophy.md: the one scrolling primitive); §Established Decisions → Scrolling and input (established-decisions.md: the scroll, wheel and scrollbar-hazard records; architecture.md); §Standard Contracts → Scrolling, selection, tree, Test harness, Driver session (standard-contracts.md; architecture.md: Scrolling, selection, tree)
**Change:**
- `scroll_into_view`: was every box on the chain and then the viewport; now a row of an open select list writes the list's `scroll_y` at once and returns, moving no chain and no viewport. The test harness's helper and the driver's `scroll` state the same.
- The wheel: over an open list it scrolls the list alone; elsewhere it closes the list and then scrolls.
- The hit: `hit_with_scrollbar` answers `select_list_hit` first and no scrollbar with it, `hit_inner` holding none of it, so `hit`, the driver's `covered` through it, `set_hover_to` and `handle_pointerdown` all read the list; a row lies past any box that clips its select.
- `visible_region`: for a row or a group of an open list it answers the list's scrollport, in place of the clipping boxes.
- The overlay scrollbar's hazard: a row is never taken as a thumb drag; it stands at every other overlay scrollbar.
**Why:** the report's symbols. The citation read found the old text of `scroll_into_view` and `visible_region` false for a row, and its rewrite was raised by main. An overlay hit in `hit_inner` is one of the plan's rejected approaches, unbuilt.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select list recorded, the select model's limits retired
**Section:** §Established Decisions (established-decisions.md: the select model and its new records; architecture.md: the select model's line and the new line beside it)
**Change:**
- Retired on the select model: no popup is painted, with its owner phrase; Enter and Space open nothing on a single select. Now Enter, and a Space with no typed text standing, open its list, and a released Space does nothing more.
- A click: was focusing the select; now also opens a single select's list or closes one standing. A bound label's click focuses it and opens nothing.
- New: the list, painted by the engine over the page, a direct engine change for every document, remote pages included, ratified.
- New, what closes it, with no event: Escape, a second click on the select, a press outside, a wheel outside the list, a page scroll or a move of the select's box; removal, `disabled`, `hidden` or `inert` on the select too.
- New, how it chooses: Tab chooses the current row and closes before focus moves, in either direction; Enter, Space and a click on a row choose through `apply_select_choice`; a disabled row changes nothing and leaves the list open; the move keys shift the highlight over enabled rows and choose nothing.
- New, rejected and unbuilt: `:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, the colours in `default.css`.
- New, its standing limits: the thumb painted and not draggable, a highlighted row's contrast unread by the lint, a larger authored font clipping in the one row height, the highlight absent from the snapshot and the tree, a single select's `size` unread.
- New, its merge surface: sixteen files, the line owned by Epoch 3's Upstream sync ahead of the UI graph.
**Why:** the founder's forks F1 (what opens it), F2 (a highlight, then a choice) and F4 (a click outside closes it and still acts), his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4); the validate record reads the engine change as ratified by the founder on his forks F1 to F10. A label opening nothing and the page's scroll closing are leans he was shown and did not object to, standing on the operator's word (inputs#I6). The rejected approaches, the limits and the merge surface are the report's.
**Kept:** an option of a closed single select, under a group too, takes no click and stays refused `covered`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — `size` on a `multiple` select
**Section:** §Established Decisions (established-decisions.md: the select model, a `multiple` select's keys, the default sheet; architecture.md: the select model's line)
**Change:**
- Was: `size` unread; now a `multiple` select's `size` hints a `height` of `size` × 1.2em through `size_rows`, capped at 65534, `0` or a bad value no hint, with `select[multiple][size] { overflow-y: auto; }` and `line-height: 1.2` on its rows in the default sheet.
- A key brings the current row into view only where the select carries a valid `size`.
- The limit narrowed: a single select's `size` is unread.
**Why:** the founder's fork F8 (`size` read on `multiple`), his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list on the carriers, a boundary widening
**Section:** §Established Decisions (established-decisions.md: the new record of the crossing); §Occupied Resources → Network ports and listeners (occupied-resources.md: a new detail group); §Standard Contracts → Dioxus DOM bridge (standard-contracts.md: the snapshot's keys)
**Change:**
- New: the select list crosses the ratified carriers only as a row's own `bounds` in the snapshot and the diff, `differs` naming `bounds`, a closed set inside the ratified crossing, and as pixels in a `screenshot`.
- New: a shot of an open list carries its option labels and group `label`s, a page's text as painted pixels, on both carriers; the body names this a boundary widening, ratified.
- No carrier, listener, port, exit, cause, session error, key, node key, field, log field or span is added.
- New: the snapshot carries no `expanded` node key and `NodeState` holds no field for a list, `NODE_FIELDS` keeping its thirteen keys.
**Why:** the widening is ratified by the founder: his forks F7 and F1, by dialog in the overseer session (inputs#I4), and his confirmation of this wrap's answer card, Q1, by dialog at P2 Validate, 2026-10-31, each relayed verbatim by the operator, who entered Q1 as his word. A row's `bounds` the card reads as outside the class, inside the ratified crossing. No `expanded` node key is the operator's call (inputs#I6).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list's state and placement in the engine
**Section:** §Existing Scopes (existing-scopes.md: blitz-dom interaction, blitz-dom node); §Standard Contracts → Document core, Nodes and widgets (standard-contracts.md; architecture.md); §Infrastructure Patterns → Runtime threading and event flow (architecture.md: the resolve's order)
**Change:**
- `BaseDocument` holds one field `select_list: Option<SelectList>`, `None` at construction. Public read-only: `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout`. Public writer: `close_select_list`, its callers keyboard, pointer, reconcile and the driver's `focus_select`. Crate-private: `live_select_list`, `select_list_for`, `open_select_list`, `reconcile_select_list`, `select_list_options`, `move_select_list_current`, `scroll_select_list_to`, `scroll_select_list_by`, `select_list_hit`, `select_list_region`. Private: `select_list_allowed`, `measure_select_list`, `text_row_layout`.
- `live_select_list`, `pub(crate)` with every caller in `document.rs`, is the state's reader, with two reads past it: `reconcile_select_list` and the removal arm. `hover_list_row` writes `current` directly.
- `resolve` calls `reconcile_select_list` after layout and before `refresh_hover`; `clear_interaction_state_for_removed_node` clears the field for its select.
- `node::select_list`: crate-private state and geometry, 6 unit tests; public read types `SelectListView` and `SelectListRow`, re-exported from the crate root as `ScrollbarRef` is. `row_disabled` is the list's reading of a disabled row.
- `SelectList.fresh` holds a just-opened list against a `:focus` rule's restyle, so a box-moving rule cannot close the list a click opened.
- Placement: at most `MAX_ROWS` (20) rows, opening where its rows fit and else on the side with more room, at least its select's width and at most the viewport's; a select with no row to show opens nothing.
**Why:** the report's symbols and its deviations from the plan: `fresh` a seventh field, two public read types where the plan says one, `live_select_list` `pub(crate)` where it says private, the field read past it in two places. Placement is a lean the founder was shown and did not object to, standing on the operator's word (inputs#I6); no row to show opening nothing was chosen at implement.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the bridge's select tests
**Section:** §Existing Scopes (existing-scopes.md: dioxus-native-dom's re-exports); §Standard Contracts → Document core, Dioxus DOM bridge (standard-contracts.md; architecture.md: the masked-input line)
**Change:**
- `closed_select_of` answers `None` for a select whose list is open.
- `is_multiple_select`: was called by `closed_select_of` and `execute.rs` `locate`, and re-exported for the driver; now called by `closed_select_of` alone within the workspace, still re-exported.
- New: public `is_select`, the driver's select test, called by `execute.rs` `locate` alone, re-exported.
- The snapshot's `bounds`: `BaseDocument::select_list_bounds` first, a row's or a group's place in an open list, else the client rect, so a row reads its select's rect again once the list closes.
**Why:** the report's symbols. The citation read found the old callers and the reason for the re-export false of `is_multiple_select`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the select-list checks' test-only resources
**Section:** §Occupied Resources (occupied-resources.md: filesystem, process-wide state and threads)
**Change:**
- The session checks' state directories gain `hl-popup` and `cc-popup` under `CARGO_TARGET_TMPDIR`; `mcp_log`'s select list test records nothing and no `ml-popup` exists.
- `host_log`'s spawned host also runs for a select's list opened and chosen through the host.
- The seven_guis checks' shared module gains `option_needles`, the needles of a select's options.
**Why:** the report's test-only directories and negative fact; the citation read found the shared module's list missing `option_needles`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — taffy is no re-export of blitz-dom
**Section:** §Stack and Technologies (stack-and-technologies.md: taffy's role)
**Change:**
- Was: re-exported in blitz-dom's public API; now part of it through its signatures, `taffy_node_id` and `dom_node_id` among them, with no `pub use` of it, the comment at the head of the cited range heading none.
**Why:** a stale claim the citation read found beside this chunk's edit, read and raised by main: no `pub use` of taffy stands in blitz-dom's `lib.rs`. No chunk fact moved it.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the list's default actions
**Section:** §Standard Contracts → Events (standard-contracts.md)
**Change:**
- `handle_pointerdown` returns on a press in the list and else calls `close_list_outside`.
- `handle_click`: its `select` arm opens a single select's list or closes one standing, its focusing in the private `focus_select`; its `option` arm calls `click_list_row` behind `click_lands_on_row`; an `optgroup` arm stands beside them; its `label` arm focuses a bound single select and opens nothing, with no re-dispatch.
- `handle_pointermove` calls `hover_list_row`, and a press in the list starts no text drag (`pressed_in_list`).
- `handle_wheel` scrolls the list over it, else calls `close_select_list` and then scrolls.
- Tab runs `choose_current_and_close` before moving focus, either direction. `press_select_key` opens on Enter and on a Space with no typed text standing, a released Space doing nothing more. `press_open_list_key` is the open list's closed key match.
- A choice from the list goes through `apply_select_choice`, the same `input` then `change`.
**Why:** the report's symbols, under the founder's forks F1, F2 and F4, his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4). `a_labels_click_focuses_the_select_and_opens_nothing` asserts the label's arm.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — a single select's `expanded` in the tree
**Section:** §Standard Contracts → Mutation, query and CSSOM (standard-contracts.md; architecture.md)
**Change:**
- The `aria-expanded` read: was on every element; now a single select reads none and writes `set_expanded(open)` from its list, true while the list stands open, every other element keeping the read.
- The names class: a single select's `expanded` is a closed bool from engine state, inside the states class, unmeasured at an assistive technology.
- An author's `open` on a select steers nothing, `flush_open_state` and `:open` holding to a `details` or a `dialog`.
**Why:** the report's symbols. The answer card reads this bool of engine state as inside the states class, outside the widening.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the driver's verbs on a select
**Section:** §Standard Contracts → Document core, Events, Driver session (standard-contracts.md; architecture.md: Document core's `clear_select_typeahead`, Events)
**Change:**
- `Target` carries: was `multiple`; now `select`. `focus_multiple_select`: now `focus_select`, also closing any list, the one non-test caller of `clear_select_typeahead`.
- `type` focuses with no click: was on a `multiple` select; now on any select, single or `multiple`. A click would land on a `multiple` select's row at the centre and open a single select's list.
- `type` on a single select fires no DOM focus events, and a text opening on a space types nothing on any select.
- `covered`'s meaning gains: an option of a closed select lies under it: click the select first, then the option. The name, remedy and order are unchanged, the eight causes standing.
- `click`'s help says a click on a select opens its list; `click`'s, `type`'s and `press`'s help each read as a fixed literal of the verb table.
**Why:** the report's symbols and its retired `Target.multiple` and `focus_multiple_select`. The added clause and the restated help are the program's own fixed text, which the answer card reads as outside the widening.
**Kept:** a driver click on an option of a closed select stays refused `covered`: the founder's fork F9, his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4).
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — the lint on an open list
**Section:** §Standard Contracts → Scrolling, selection, tree, Driver session (standard-contracts.md: `visible_region`'s callers, the lint's clip region and `overlap`)
**Change:**
- `Entry.in_closed_select`: now `Entry.in_select_box`, reading a closed select's option through `closed_select_of` and true also for an open list's row or group.
- The `overlap` reading: was leaving out a closed select's options; now also an open list's rows and groups, which read their own bounds in the list and read no `overlap` on the stand, `an_open_lists_rows_read_no_overlap_on_the_stand` and `the_lint_reads_no_overlap_on_a_row_and_the_planted_pair_still_reads` pinning it.
- `clip_region` takes an open row's region from `visible_region` alone, with no `contain: paint` cut.
**Why:** the founder's fork F10 (rows with no overlap), his own word by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I4); the report retires `Entry.in_closed_select`.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/

## 2026-10-31-select-popup — no local gate builds `font-embolden`
**Section:** §Standard Contracts → CI contracts (standard-contracts.md)
**Change:**
- New: no local gate builds `blitz-paint/font-embolden`, which CI's macOS leg alone exercises. `--features blitz-paint/font-embolden` on `cargo test -p blitz-tests` is the local witness for a paint change under that feature, `--test select_popup_painted` reproducing macOS's two readings on linux.
- Its owner: a CARRY on Upstream sync ahead of the UI graph.
**Why:** a local gate block read green over a defect only macOS showed, CI's first read red on `Test (macos)` alone. The owner is the founder's answer to Q2 on this wrap's answer card, both owners on the sync, by dialog at P2 Validate, 2026-10-31, relayed verbatim by the operator.
**Ref:** .andromeda/runs/2026-10-31T20-10-56-wrap/
