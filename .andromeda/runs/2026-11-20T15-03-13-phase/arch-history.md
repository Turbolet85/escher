@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list recorded, the select model's limits retired) — recorded the open list as a ratified direct engine change for every document, remote pages included, and named its standing limits: the thumb not draggable, the highlighted row's contrast unread by the lint, a larger authored font clipping in the one row height, the highlight absent from the snapshot and the tree, a single select's `size` unread. Four of this chunk's five debts are these limits, so their fixes retire this record's limit lines. It also records the rejected designs (`:open` plus rows as boxes, a second OS window, an overlay hit in `hit_inner`, colours in `default.css`), which no fix may bring back, and the list's sixteen-file merge surface.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's paint and a group's header) — `paint_scene` paints the open list last through `paint_select_list`. The row text is laid out at paint through the public `select_row_layout`, only for the rows in the scrollport, which is the stated exception to paint-assumes-resolved-layout. `stroke_text_as`/`RunPaint` gives a header synthesized bold. The list colours are engine constants on `SelectListView` (`HIGHLIGHT_TEXT` #000000, `HIGHLIGHT_BACKGROUND` #c8c8c8, `DISABLED_TEXT` #6d6d6d, `BORDER` #999999), not tokens. Debt 1 (highlight contrast) reads these constants. Debt 2 (larger font) changes how rows are laid out in `packages/blitz-paint/src/render/select_list.rs`.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list's state and placement in the engine) — `BaseDocument.select_list`, its public readers (`open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout`) and crate-private machinery (`measure_select_list`, `text_row_layout`, `select_list_region`, …) in `node::select_list`. Placement is at most 20 rows (`MAX_ROWS`), opening where the rows fit, and `fresh` guards against a `:focus` restyle. This is where the one row height a larger authored font clips in is computed (debt 2). Any new reader of the highlight (debt 3) also starts here.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list on the carriers, a boundary widening) — the list crosses the carriers only as a row's own `bounds` in the snapshot and the diff, `differs` naming `bounds`, and as pixels in a shot. The snapshot carries no `expanded` node key, `NodeState` holds no field for the list, and `NODE_FIELDS` keeps its keys (an operator's call). A highlight reading (debt 3) either stays a closed value inside this ratified crossing or is a new key/field and so a widening for the founder. The decision row select-list-crossing is the row this entry created.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's `expanded` in the tree) — a single select reads no `aria-expanded` and writes `set_expanded(open)` from its list, a closed bool of engine state read as inside the adapter's states class and outside the widening. This is the precedent for putting the highlighted row in the accessibility tree (debt 3, for example an active-descendant or selected-row state) without widening. An author's `open` on a select steers nothing.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`size` on a `multiple` select) — F8 «Читать на multiple»: a `multiple` select's `size` hints `size` × 1.2em through `size_rows` (capped at 65534; `0` or a bad value gives no hint), with `select[multiple][size] { overflow-y: auto; }` and `line-height: 1.2` in the default sheet. A key brings the current row into view only where `size` is valid. The limit was narrowed to "a single select's `size` is unread". Debt 4 reopens this founder ruling (see also n=11).

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the list in the redraws and the changed set) — the list's open and close mark the select in the changed set, and a page scroll, a box move or a wheel elsewhere marks it on close. A highlight move or a wheel inside the list marks nothing, and a redraw is asked on open, move, scroll and close. If a highlight reading (debt 3) reaches the snapshot or the tree, it must decide whether a highlight move now marks the changed set, which today it explicitly does not.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint on an open list) — `Entry.in_select_box` skips an open list's rows and groups, and a closed select's options, in `overlap` (F10 «Строки без overlap»). `clip_region` takes an open row's region from `visible_region` alone. This is the lint's existing handling of list rows, which debt 1 (the lint reading the highlighted row's contrast) extends to the `contrast` defect.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's exceptions to scrolling and the hit; the list's default actions; the bridge's select tests; the driver's verbs on a select) — the hit answers `select_list_hit` first. `scroll_into_view` on a row writes the list's `scroll_y` alone, and the wheel over the list scrolls only the list. A row is never taken as a thumb drag. The default actions are `handle_pointerdown`, `handle_click`, `hover_list_row` and `press_open_list_key`, with Tab choosing and closing. `closed_select_of` answers `None` for an open list. `type` focuses any select with no click, through `focus_select`. A row height change (debt 2) or a `size`-driven list box (debt 4) moves these hit, scroll and action paths.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (his words on the select list; no local gate builds `font-embolden`) — the founder's verbatim fork labels F1–F10 (F6 «Как у select» for the list's colours, F8 «Читать на multiple»), which this chunk's forks revisit. A local gate once read green over a defect only macOS showed under `blitz-paint/font-embolden`, and `select_popup_painted` reproduces macOS's two readings on linux. This is a trap for debts 1 and 2, which paint rows (n=31 records the gate it led to).

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the select limits that stand) — `size` was sent to the popup's chunk by the founder's P5 word. An option of a closed single select, under a group too, takes no click and stays refused `covered`. This is the lineage of the single-select `size` debt (debt 4) and of the closed-select-options-covered row.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's rows and chosen-row pair) — the default sheet holds a `multiple` select's chosen-row pair `select[multiple] > option:checked … { background-color: #c8c8c8; color: #000000; }`, measured at 12.56:1 by `select_interaction`. It is Chromium's inactive selected pair, hardcoded since the engine holds no tokens. The open list's highlight uses the same pair as engine constants (n=2), which is the tie debt 1 relies on (the multiple-select-chosen-row-colours row, guarded by `select_interaction`).

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select's chosen set, keys and pointer; optgroup) — F6/F7 built the `multiple` select (`apply_select_choice` taking the set, `press_multiple_select_key`, `click_select_row`) and `optgroup` (`option_disabled`, a group's `Role::Group` named by its `label`). The default sheet gives a multiple select's rows `display: block`. A `multiple` select's chosen rows and their colours (debt 1) and a single select rendered as a list box by `size` (debt 4) build on this model.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a multiple select in the bridge and the snapshot; the public select readers) — a `multiple` select's rows and groups read their own client rects as `bounds`. A single select's choice names the new option with `selected` in `differs`. `select_values`, `select_of` and `closed_select_of` are the public readers. A single select turned into a list box by `size` (debt 4) must decide whether its options read their own rects like a `multiple` select's or the closed select's rect.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's overlap leaves out a closed select's options) — `in_closed_select` (later `in_select_box`) reads through `closed_select_of` by the element, never the role, so CRUD's `role="option"` rows are not read as a select's. A `multiple` select's rows are linted as usual. This is the precedent for how a lint rule singles out select rows (debt 1).

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' grid, its keys and its count) — `cells-grid` is one Tab stop (F13) with `aria_label` Cells, `tabindex` 0, an `onkeydown` and no role. Arrows move `current`, which is drawn only by `.cells-grid:focus .cell.current` (an accent outline, `2px solid #3a5ce5`). Enter opens the editor, and the count of elements with no role is 677. Debt 5 (no selected reading and no grid role on Cells) changes exactly this record. The cells-grid-single-tab-stop row cites its pins.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the blitz-tests records' counts) — `select_interaction` covers the chosen row's ratios, `stand_contrast` covers the load page's and Cells' ratios, and `stand_act_select` and `stand_keyboard` cover Cells' walk and arrows. These are the checks that pin today's readings of debts 1 and 5 and that move when they change.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (every stand task keyed) — Cells' author keys `cell-{A..Z}{0..25}` and `cell-input` were named here, with `unkeyed_actionable()` empty on every stand task. Debt 5's selected reading is stated on these ids, which must stay stable.

@history_item n=19 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the snapshot model reads selected, pressed and invalid) — `NodeState.selected` is the accessibility node's own `is_selected()`, never parsed from the element. It is written as `selected`/`unselected` in the text and left out of the JSON when absent. The flight booker and CRUD state selected by attribute this way. This is the reading debt 5 would give Cells' current cell, and debt 3 could give the highlighted row.

@history_item n=20 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (the accessibility builder reads three state attributes) — `aria-selected` is read on every element whatever its role, trimmed and lower-cased exactly. The adapter gained the states class (closed enums and booleans, never a value as typed), ratified as a widening of the adapter crossing, and later attribute reads must keep this shape. This bears on a Cells `aria-selected` or grid role (debt 5) and on a highlight state in the tree (debt 3).

@history_item n=21 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the select model, an engine change for every document) — a select's choice is held as `CHECKED` through `apply_select_choice`. The builder reads `aria-selected` on every element but a select's option, whose `selected` is its chosen state. The user-agent select box and the select model's merge surface are in `node/select.rs`. A highlight reading must not be confused with a select option's `selected`, which already means "chosen" (debt 3).

@history_item n=22 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the builder's merged ARIA reads) — upstream's `parse::<bool>` `aria-selected` read was dropped so each attribute is read once. The builder also reads `aria-expanded`, `aria-describedby` and others, and descriptions go to the adapter only. Rows each-aria-attribute-read-once and a11y-aria-states rest on this. Any Cells `aria-selected` or select highlight state (debts 3 and 5) must keep the one-read rule.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the toggle per role in the builder) — `aria-checked` is read on checkable roles alone and `aria-pressed` off them, with `"switch"` mapped to `Role::Switch`. This is the precedent for role-dependent state reads, relevant if Cells takes a grid or gridcell role (debt 5).

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a native control's toggled in the tree) — the builder writes `toggled` from engine state under a checkable role. The adapter's states class gained a native control's `toggled` and a `multiple` select's multiselectable. This is the precedent for a state written from engine state rather than an attribute, as a highlight's selected or active reading in the tree would be (debt 3).

@history_item n=25 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's `clickable` reading) — it added a closed bool to `NodeState`, `Reading` and `NODE_FIELDS` as "a closed bool inside the ratified carrier (playbook rule `rul-cepujoy6`)", growing every served screen's text (Cells to 6 parts). This is the direct precedent for the scope's rule that a closed-value field is no widening while a new key or open value is (debt 3, and debt 5's selected on 676 cells).

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Cells screen in six parts) — Cells' screen reads 53,859 bytes in 740 lines, 6 parts of `SNAPSHOT_TEXT_BUDGET`, after the `clickable` token. A `selected`/`unselected` token on 676 cells (debt 5) moves this measured reading and `stand_cells_served`'s `PARTS`.

@history_item n=27 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** (the harness's contrast reader) — `Harness::text_contrast` and `contrast_ratio` (WCAG 2.x, lighter over darker, alpha ignored) read an element's text over its composited background, with `TextContrast` and `required_ratio`. It is not a paint-time reading, so an open list's highlighted row, painted by `paint_select_list` rather than as boxes, is outside what it reaches. This is the instrument debt 1 has to extend or replace.

@history_item n=28 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`text_contrast` past a blend; blending painted) — `text_contrast` and the lint's `contrast` read `None` (not measured) past a `mix-blend-mode` other than `normal`, through `paints_more_than_colour`, with no second formula. A viewport-wide blend layer is popped last so an open list paints past the blend layers. Debt 1's contrast reading of list rows must keep the one formula and this not-measured rule.

@history_item n=29 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the in-process visual lint and the gated part: `lint` the twelfth verb) — `Defect` is closed at six, `contrast` among them, with `Reason` keys `ratio` and `required`. The lint's own `Bounds` type holds no page text, colour or value, and the clip region comes from `visible_region`. Its findings cross every carrier as ids, closed defect words and numbers, "no new carrier, exit, cause, field or dependency". Reading the highlighted row's contrast (debt 1) is a new `contrast` finding inside this ratified crossing, and it must not carry a colour.

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand checks re-pinned to the flight booker's select) — the flight booker's `flight-type` select was given `stand_contrast` rows (#1a1a1a on #ffffff, 17.40) and `STATE_ROWS`. Its options `flight-type-one-way`/`flight-type-return` are refused `covered` and read at their select's client rect. These stand rows move if the select's list contrast or `size` handling changes (debts 1 and 4).

@history_item n=31 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` ends on `embolden`, which runs `select_popup_painted` under `blitz-paint/font-embolden`, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. This replaced the macOS-only witness. Any change to row painting or row height (debts 1 and 2) must stay green under this leg (the font-embolden-local-witness row).

@history_item n=32 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** (stand typography and cross-OS readings) — every stand boot lays out in the bundled DejaVu Sans by fallback, guarded by `stand_bundled_font_families`. The served tasks' snapshot, lint and actions digests must read identical on linux, windows and macOS in both layout modes (later seven tasks, 2026-11-18-served-error-page). A larger authored font in a fixture (debt 2), a new snapshot token on Cells (debt 5) or a new lint finding (debt 1) must hold `stand_cross_os_readings`.

@history_item n=33 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Established Decisions: `select-list-crossing` amended) — the select list now also crosses as the placing words `select` and `select-list` in a layout answer, and the closed list of what crosses a session gained layout readings. It is the latest form of the crossing row a highlight reading (debt 3) must amend or stay within.

@history_item n=34 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** (the a11y leg's targets) — `select_interaction` and `stand_act_select` run in the `a11y` leg and in `stand-repeat` (the leg's file count was later held by the capability-statement pin, 2026-11-18). A new select or Cells accessibility check (debts 3 and 5) decides its leg membership against this list.

@history_item n=35 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Standard Contracts: focus change; Stack: parley 0.13, Stylo 0.24) — a focus change dispatches the four focus events, with only the inert fix-up and the driver's select focus dispatching none. Text layout moved to Parley 0.13 and Stylo 0.24 just before this chunk. The latter bears on the row-height and row-layout computation of debt 2, and the former on any select focus path the debts touch.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the merge surfaces' reading and owner at `e42ee1f25`) — the select model's and the select list's merge surfaces are flagged upstreamable with nothing sent, their owner moved to a later upstream-sync CARRY. Every fix this chunk makes to the list or the select model widens that surface and must be recorded as a direct engine change for every document (the select-list-engine-change and direct-engine-changes-flagged-upstreamable rows).
