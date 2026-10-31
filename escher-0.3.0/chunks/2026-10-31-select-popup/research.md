# Codebase Research — 2026-10-31-select-popup

## Scope
- **Depth:** deep · **Reads:** 19 (10 whole or section reads of `packages/`, the sheet, the harness's `contrast.rs` and `events/driver.rs`, the playbook's rules) · **Globs/Greps:** 6
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 2 additions applied (a boot smoke's exit 124 · a CI job stated from its own log) · `.claude/rules/host-linux.md` — 5 additions applied
- **Platform issues consulted:** none — no runner-only bullet: the take-up's CI read is green at `da536ea7a714`, and the WATCH is observe-only
- **External inputs:** `inputs#I1` — the founder lifts the pause for this one chunk · `inputs#I2`, `inputs#I3` — the operator's run terms and their correction

## Files inspected
- `packages/blitz-dom/src/node/select.rs` (full) — the rules module: `options`, `readings`, `stepped`, `matched`, `gestured`; no open state. Unit tests inline, 8.
- `packages/blitz-dom/src/events/keyboard.rs` (full) — `press_select_key` takes every pressed key of a focused single select, Enter included, before `activates_on_enter`; a pressed Space reaches it as `Key::Character(" ")` and is typeahead today, `select_step` the four step keys. `handle_key_up` reaches `activates_on_space`, which holds no `select`.
- `packages/blitz-dom/src/events/pointer.rs` (full) — `handle_pointerdown` reads `doc.hit`, and an option hit falls in `ClickTarget::SelectableText`, so `find_text_position` would select page text under a list; `handle_click`'s `select` arm focuses, its `option` arm filters on `multiple` and walks on; the `label` arm re-dispatches to the control through `handle_click`, so a label click would open a list unless told apart.
- `packages/blitz-dom/src/events/driver.rs` (full) — `set_hover_to` sets the target of a pointer event; `PointerUp` dispatches `Click` to the hover node, so a row hit is the click's target; `adjust_element_coords` reads `get_client_bounding_rect(target)`.
- `packages/blitz-dom/src/events/mod.rs` (full) — `handle_dom_event`'s default actions.
- `packages/blitz-dom/src/node/node.rs` (1415-1654) — `hit_inner`: inert, visibility, a non-invertible transform, the stop at the padding box of a clipping box, hoisted and paint children.
- `packages/blitz-dom/src/document.rs` (715-774, 1160-1209, 1755-1785, 1860-2110, 2530-2600) — the public select readers, `can_paint_text`, `layout_as_text_input`, `hit` → `hit_with_scrollbar` → `root_element().hit_inner`, focus writes, `set_hover_to`, `get_client_bounding_rect`.
- `packages/blitz-dom/src/scrolling.rs` (655-812) — `scroll_into_view` and `visible_region` walk `containing_block_chain`; for an option under `display: none` both read a box that does not exist.
- `packages/blitz-dom/src/accessibility.rs` (150-297) — `aria-expanded` is read from the attribute alone; `is_hidden_from_accessibility_tree` excepts a select's option; no native `expanded` write.
- `packages/blitz-dom/assets/default.css` (70-194) — `option { display: none }`, the chosen option inline, groups `display: contents`, `multiple` rows, `#c8c8c8` / `#000000`.
- `packages/blitz-paint/src/render.rs` (100-290, 725-850, 975-1049) — `paint_scene` paints the root, then the debug overlays, last and unclipped: the seam a list paints at; `draw_scrollbars`, an engine-painted overlay with constant colours chosen by `viewport().color_scheme`; `draw_masked_input_text`, a text laid out by blitz-dom and stroked with `crate::text::stroke_text`.
- `packages/blitz-test-harness/src/contrast.rs` (1-120), `src/input.rs` (98-115, 202-219), `src/inspect.rs` (96-105) — `text_contrast` reads the first text parent's computed `color` over its ancestors' `background-color`s, so it reads an option's plain pair and cannot read a colour paint alone holds; `click` is `click_at(center_of)`; `Harness::hit` is `BaseDocument::hit`.
- `packages/dioxus-native-dom/src/snapshot.rs` (100-275) — `closed_select_of` boxes an option in its select at line 148; `enabled` is read only where `can_be_disabled()` holds.
- `packages/escher-driver/src/execute.rs` (full) — `action_point`: `enabled`, `in_view` through `visible_region`, `covered` through `harness.hit`; `type` clicks the centre but on a `multiple` select; `Scroll` calls `harness.scroll_into_view(node)`.
- `packages/escher-driver/src/lint.rs` (150-410) — `overlaps` skips `in_closed_select`, set through `closed_select_of`; `clip_region` is `visible_region` cut by `contain: paint` ancestors.
- `packages/blitz-dom/src/node/element.rs` (543-549) — `flush_open_state` writes `ElementState::OPEN` on `details` and `dialog` alone and clears it on anything else.
- `tests/blitz-tests/tests/select_interaction.rs`, `stand_act_select.rs`, `stand_contrast.rs` (415-430), `stand_id_persistence.rs` (90-102), `stand_snapshot_state.rs` (120-130), `stand_act_ids.rs` (55-66) — the rows a click-opens change moves, below.
- `.andromeda/playbook.md` (rules) — `Boundary widening` escalates; `An engine behaviour change for every document` is routine on the founder's own word at this P4; `A field inside a ratified carrier` is routine; `The program's own fixed text` is routine.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-31T16-59-55-phase/tree-query-2026-10-31-select-popup.json`)
- **`closed_select_of`** — 2 callers: `visit` at `packages/dioxus-native-dom/src/snapshot.rs:148`, `flatten` at `packages/escher-driver/src/lint.rs:338`; re-exported at `packages/dioxus-native-dom/src/lib.rs:36`. Its answer once a list is open moves both.
- **`visible_region`** — 3 non-test callers: `in_view` at `execute.rs:467`, `crop` at `execute.rs:478`, `clip_region` at `lint.rs:400`; 6 test callers, none on a select's option.
- **`apply_select_choice`** — 2 callers: `press_select_key` at `keyboard.rs:193`, `apply_row_gesture` at `keyboard.rs:378`. A list's choice is a third, on the one write path.
- **`hit_with_scrollbar`** — 3 callers: `hit` at `document.rs:1764`, `set_hover_to` at `document.rs:2084`, `handle_pointerdown` at `pointer.rs:421`. A row's hit put here reaches every hit reader, hover and the driver's `covered` included.
- **`select_of`** — `handle_click` at `pointer.rs:892` and `closed_select_of` at `snapshot.rs:245`.
- **`paint_scene`** — one body: `blitz-paint/src/lib.rs:69` calls `BlitzDomPainter::paint_scene`; reached by the window (`blitz-shell/src/window.rs:350`, `:424`), the harness (`blitz-test-harness/src/screenshot.rs:84`), a sub-document (`render.rs:1346`), the apps and the WPT runner. A list painted there reaches every renderer through one site.

## Patterns detected
- **An engine-painted overlay** (`render.rs:735`, `document.rs:2055`): scrollbars paint constant colours by scheme, and their hit shares the descent in `hit_with_scrollbar`, a pointerdown in the thumb's band taking precedence over dispatch. The nearest shape for a list; a thumb's hazard, a driver `click` in a just-scrolled band dispatching nothing, is the row's to avoid.
- **Interaction state referencing DOM nodes** (`document.rs:1962`, the debug assert): hover and active hold canonical DOM ids. The list's rows are the option nodes, so every option keeps its stable id open and closed.
- **A closed key match with one write path** (`keyboard.rs:176`, `:385`): `press_select_key` then `apply_select_choice`, firing one `input` then one `change`, nothing for the chosen option.
- **A native state written from engine state** (`accessibility.rs:205`): a native checkable reads no author attribute and writes its own `toggled`. The shape for a native select's `expanded`.

## Conventions to follow
- **Handler arms are private named functions** (`pointer.rs:1016` `click_select_row`, `keyboard.rs:155` `press_range_input_key`): parents stay at their upstream floor, the audit's complexity ceiling 15 in view.
- **A public reader through `get_node`, writers crate-private** (`document.rs:748`, `:753`).
- **One behaviour per test file, both layout modes, the fixture's condition asserted first** (`select_interaction.rs:111`), a failure message a mode and a row index, never an id or a value.

## New files to create
- `packages/blitz-dom/src/node/select_list.rs` — the open list: its state, geometry and row reader, the rules unit-tested inline
- `packages/blitz-paint/src/render/select_list.rs` — the list's paint
- `tests/blitz-tests/tests/select_popup.rs` — open, move, choose and close by pointer and key, in both layout modes
- `tests/blitz-tests/tests/select_popup_painted.rs` — the list's frames and painted colour pairs
- `tests/blitz-tests/tests/stand_act_popup.rs` — the list through the driver

## Files to modify
- `.gitignore` — the CARRY's `escher-0.4.0-incubator/` line
- `packages/blitz-dom/src/node/mod.rs` — the new module and its read type
- `packages/blitz-dom/src/lib.rs` — the read type's re-export, if the root lists it
- `packages/blitz-dom/src/resolve.rs` — the reconcile call
- `packages/blitz-dom/src/stylo.rs` — `size`'s presentational hint
- `packages/blitz-dom/src/events/keyboard.rs` — the open list's keys
- `packages/blitz-dom/src/events/pointer.rs` — opening, a row's click, no text selected under a row, a label's click
- `packages/blitz-dom/src/document.rs` — the open state, the public readers, the row's hit in `hit_with_scrollbar`
- `packages/blitz-dom/src/scrolling.rs` — `visible_region` and `scroll_into_view` for a row
- `packages/blitz-dom/src/accessibility.rs` — a native select's `expanded`
- `packages/blitz-dom/assets/default.css` — `size`'s two rules
- `packages/blitz-paint/src/render.rs` — the list painted last in `paint_scene`
- `packages/dioxus-native-dom/src/snapshot.rs` — a row's bounds, `closed_select_of`
- `packages/dioxus-native-dom/src/lib.rs` — re-exports
- `packages/escher-driver/src/execute.rs` — `type` on a single select
- `packages/escher-driver/src/lint.rs` — the restated `overlaps` exclusion
- `packages/escher-driver/src/refusal.rs` — `covered`'s restated meaning
- `packages/escher-driver/src/schema.rs` — `type`'s and `click`'s fixed help
- `packages/escher-driver/src/schema_tests.rs` — the pins of that help
- `packages/escher-driver/src/json.rs` — the refusal text's pin
- `packages/escher-driver/src/lib.rs` — the crate docs' `covered`
- `docs/capabilities.md` — the select limits and the accessibility reach, by hand
- `tests/blitz-tests/tests/select_interaction.rs` — `focused_pick` clicks then keys; `enter_and_space_open_nothing_on_a_single_select` ends
- `tests/blitz-tests/tests/stand_act_select.rs` — a click then a key
- `tests/blitz-tests/tests/stand_contrast.rs` — `Reach::ReturnChosen`, the new rows
- `tests/blitz-tests/tests/stand_id_persistence.rs` — a click then End
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — `choose_flight_type`
- `tests/blitz-tests/tests/stand_keyboard.rs` — the open list's walk and keys
- `tests/blitz-tests/tests/stand_lint.rs` — an open list's reading
- `examples/seven_guis/tests/host_log.rs` — the popup flow
- `examples/seven_guis/tests/mcp_log.rs` — the popup flow
- `examples/seven_guis/tests/cli_commands.rs` — the hosted open
- `examples/seven_guis/tests/common/mod.rs` — the option needles
- `.github/scripts/ci-leg.sh` — a non-`stand_` target's literal `--test` word in `stand-repeat`
- `.github/scripts/test_ci_workflows.py` — that word's pin

## Sweeps
- `flight-type` over `tests`, `examples`, `scripts`, `.github` (`git grep -n "flight-type"`, `tests/blitz-tests/checks` left out): 45 hits · 3 changed (`stand_contrast.rs:419`, `stand_id_persistence.rs:96`, `stand_snapshot_state.rs:124`, each a click before a key) · 42 no-change (`type_into("flight-type", …)`, unmoved since `type` stays typeahead on his F1; `stand_act_ids.rs:60-61`, closed options staying `covered` on his F9, forecast unmoved; `stand_keyboard.rs:432-516`, its route by Tab and ArrowDown holding on his F1; id tables, ids unmoved).
- The select tests a restatement renames, `enter_and_space_open_nothing_on_a_single_select` · `a_click_then_a_key_chooses_an_option` · `an_option_under_a_closed_selects_group_is_refused_covered` · `the_flight_type_select_moves_its_choice_by_its_arrows`, over `tests`, `examples`, `docs`, `.github`, `scripts` (`git grep -n`): 4 hits, each its own definition · 2 changed · 2 no-change; no `WITNESSES` row names one.
- `flight` over `examples/seven_guis/tests/flows/flight.sh` (`grep -n`): the flow touches `flight-start` and `flight-book` and no select, so its 7 steps are forecast unmoved.
- `size` as an attribute over `packages/blitz-dom/src` (`git grep -n 'local_name!("size")\|"size"'`): 0 hits, so `size` is unread, as the limit states.

## Open questions
- none — the two carried to P4 are answered: his ten forks all option A (inputs#I4), and no `expanded` node key on the operator's word (inputs#I6). The file lists above are closed to them.

## Measured for `size`
- `packages/blitz-dom/src/stylo.rs:1105-1122` maps a presentational attribute through `push_style`, the `svg` `height` the shape `size`'s hint takes. `stylo_to_parley.rs:407-412` maps a `line-height` number to `FontSizeRelative`, so the sheet's `line-height: 1.2` makes a row `1.2em`, the equality the hint's `size × 1.2em` rests on; not yet read on a frame, so `select_popup`'s `size` row is that equality's first reading.
