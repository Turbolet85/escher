@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's plain pair is measured, its value unstated): added the `blitz-dom select list plain row` row, measured by `select_popup_painted` at the default sheet and the stand, and said outright that a highlighted row's contrast is unread by the lint and a closed select's own pair is unread. This is where debt 1 (`res-tym5wois`) comes from. Any lint `contrast` reading of the highlight, or of a closed select, rewrites this row.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants): added the `SelectListView` constants `HIGHLIGHT_BACKGROUND`/`HIGHLIGHT_TEXT` `#c8c8c8`/`#000000`, `DISABLED_TEXT` `#6d6d6d` and `BORDER` `#999999`. They are hardcoded and kept out of `default.css` and out of tokens on the founder's F6. The highlight is the same pair as a `multiple` select's chosen row. Colours in `default.css` stand rejected. Debt 1's shared contrast fix must keep this split (rows decision/select-list-colours-engine-constants, decision/engine-colours-hardcoded).

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair): added the `blitz-dom default chosen row` pair `#c8c8c8`/`#000000`, Chromium's inactive selected pair, 12.56:1 as measured by `select_interaction`. It is coloured by the sheet rule `select[multiple] > option:checked, … > optgroup > option:checked` (F6). The working entry ties debt 1 to this row (decision/select-chosen-row-chromium-pair). A change of colour here moves the list's highlight too.

@history_item n=4 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height): `MAX_ROWS` = 20, plus the `BORDER_WIDTH`/`ROW_PADDING_X`/`ROW_PADDING_Y`/`GROUP_INDENT` constants. `place` keeps the list inside the viewport with finite edges, between the select's width and the viewport's. A row's height is the select's font on one line, laid out through `select_row_layout`, and "a larger authored font clips, a standing limit": the row height was chosen at implement, where the plan left it open. This is debt 2 (`res-2iu7qn2e`). A fix moves this Typography item, and it must keep placement finite (decision/select-list-placed-in-viewport).

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list): the highlighted row is in no snapshot and no tree. The tree reads the open select `ComboBox`/`expanded` and the chosen option `selected`, and an author's `open` steers nothing. A row carries its own `bounds`, and a shot of a row crops to it. The answer card held row `bounds` and `expanded` inside the boundary as a closed set or a closed bool. This is debt 3 (`res-caouqg3t`), and its precedent for when a new reading is no widening (contracts select-accessibility/*, snapshot/select-list-row-bounds, screenshot/select-list-row).

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a `multiple` select with `size`): the founder's F8 has `size` read on a `multiple` select only. `size_rows` hints a height of `size × 1.2em`, capped at 65534, and a bad value gives no hint. `select[multiple][size]` gets `overflow-y: auto` and a Typography line-height of 1.2. This is the basis of debt 4 (`res-ysz67imk`). Reading `size` on a single select overturns F8, so it is a founder fork.

@history_item n=7 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay): replaced "no select popup is painted". The list opens on a click, Enter, or a Space with no typed text. A group's `label` is painted as a header. `size` is read on a `multiple` and "stays unread on a single select". On desktop-native the list is an engine overlay through `paint_scene` alone, and a second OS window or `:open` plus rows as boxes stand rejected (decision/select-list-engine-overlay, decision/single-select-open-gestures). Debt 4's list-box rendering has to fit this.

@history_item n=8 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (no select popup is painted, and who owns it): the earlier F8 line: Enter and Space opened nothing, and `size` was not read. It handed the owner to the Select popup entry. It is now superseded by the 2026-10-31-select-popup overlay entry. It is the history behind "`size` read on `multiple` alone".

@history_item n=9 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups): `multiple` rows are `display: block`, `nowrap`, padding 0 2px. A single select's group is `display: contents`, and its chosen option under a group is `display: inline` (F6/F7). These are the sheet rules that a single select rendered as a list box (debt 4), or a chosen-row colour change (debt 1), would touch.

@history_item n=10 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the user-agent select box): in the UA sheet a `select` takes `1px solid #999`, `2px` padding, a white background and `inline-block`. A single select shows its chosen option alone, so it lays out as a box holding the option's label (F2), from engine defaults with no token (decision/single-select-box-shows-chosen-label). Debt 4's single `size` rendering changes this.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes): the open, highlight-move, choose, disabled-row, close and close-on-select-change events, and the list of selects that open nothing. A `multiple` select opens no list. This is the F2/F4 base for contracts select-list-events/* and decision/selects-that-open-no-list. Debts 3 and 4 must keep these events (an added highlight reading, or a sized single select that opens no list).

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row): a group reads as a header, painted heavier through `RunPaint { color, synthesize_bold }` in `stroke_text_as`. The emboldening and hinting line was reworded. A larger-font fix (debt 2) touches the row and header layout in the same painter.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`visible_region` on a row of an open list): for a row or group of an open list, `visible_region` answers the list's scrollport, and the driver's lint takes an open row's region from it alone. This is the path any lint `contrast` reading of the highlighted row (debt 1) would read.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk): `select_list_hit` in `hit_with_scrollbar` answers first, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read the list, and a box clipping the select clips no row (decision/open-select-list-hit-first, the except clause of decision/hit-testing-stops-at-paint-clip). Larger rows (debt 2) change the hit geometry.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list paints last, whole past clips and opacity): `paint_select_list` runs last in `paint_scene`, whole past an `opacity` ancestor and a clipping box, and only rows in the scrollport are laid out (decision/select-list-painted-after-root). This is the painter debts 1 and 2 edit.

@history_item n=16 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation` in the depth rules): the open list's paint-last row now also paints past the blend layers. This is the current wording of decision/select-list-painted-after-root.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion): a wheel over the list scrolls it, a wheel elsewhere closes it first, and a key brings the current row into view at once. The list has no motion (the founder's F5), so no `advance` is needed (decision/select-list-wheel-and-key-scroll, decision/select-list-opens-without-motion). A taller row (debt 2) changes the scroll steps.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (into view on a row of an open list): an exception to the into-view rule: on a list row, `scroll_into_view` writes the list's `scroll_y` at once and moves no chain or viewport. This is the except clause of decision/into-view-scroll-instant-in-nested-boxes.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (an into-view scroll never animates a nested box): the base into-view rule, ratified by the founder 2026-10-07. Smooth applies to the viewport alone. The list-row exception and the Cells grid's into-view move both rest on it.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's thumb): the thumb paints under `feature = "scrollbars"`, takes no hit and cannot be dragged. Its drag is `res-ammkrycp` (v040-12), out of this chunk's scope. This is the boundary not to cross.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block): a focused select whose container lost its box keeps focus, takes a key with no panic, and the key still moves its choice (the founder's F1 A). This is decision/boxless-focused-select-keeps-focus. A select-state change in debts 3 and 4 must keep it.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand accent's use in the Cells grid): the Cells grid's current-cell focus outline `.cells-grid:focus .cell.current` uses the stand accent, held in layout-templates (F13, one Tab stop). This is the current-cell state that debt 5 (`res-wvatzhpe`, Cells' selected cell) would expose as a selected reading.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation): the Cells arrow-key handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously. The async form panicked with `RefCell already borrowed`. This is a trap for any Cells selection change in debt 5.

@history_item n=24 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (render-time MountedData calls on every Dioxus document): a render-time `MountedData` call no longer panics, but it acts on the previous render's document. This is the limit a Cells selected-state change (debt 5) must respect.

@history_item n=25 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select, the served load page and the scrollbar's band): the flight booker's `flight-type` select overrides the UA select defaults. The stand serves Cells. A click on a Cells cell in the scrollbar band takes an `advance` first. These are stand facts for the proofs of debts 1, 2, 4 and 5 on the stand.

@history_item n=26 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flight-type select overrides and the load page's group leave design-system): the stand's select overrides moved to layout-templates' desktop-native screens on the founder's word, and the engine's select-defaults item stays here. Stand select values touched by this chunk belong in layout-templates, not design-system.

@history_item n=27 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the shot's answer line, the MCP image block, no font refused): a screenshot whose font context resolves no `sans-serif` is refused `fonts-unavailable`. It also set the screenshot answer line and the MCP image block (contracts driver-screenshot/fonts-unavailable, escher-session-commands/screenshot, escher-session-mcp/screenshot-result). These carriers prove the larger-font fix (debt 2) and the highlight colours (debt 1).

@history_item n=28 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (the paged `snapshot` answer line on the cli surface): the snapshot answer line `{"text"}` or `{"text","part","parts"}` and its MCP result (contracts escher-session-commands/snapshot, escher-session-mcp/snapshot-result). Any highlight reading added in the snapshot (debt 3) crosses this carrier.

@history_item n=29 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the driver command line's output form): `escher-session` prints one uncoloured JSON line per command, keyed by the schema's words (decision/escher-session-agent-read-surface). A new reading or key for debts 3 and 5 is bound by this form, and by the scope's "new key = widening" rule.

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Surface: desktop-native: 2 rows): + contract/escher-session-commands/layout-answer. This is the `layout --id` answer, which a row-height proof (debt 2) can read.

@history_item n=31 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (§Motion: the harness drives animation from the documents' clock): `click`, `type` and `press` leave the `VirtualClock` where it was. Ratified 2026-10-18 (2026-10-18-engine-debts, the PROVISIONAL batch). These are contracts driver-verbs-and-virtual-clock/click and /press, which the list's keyed and clicked proofs rely on.

@history_item n=32 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the hit stops at a `contain: paint` box): the paint-clip hit rule was widened to a computed `contain` including `paint` (F5 A, later ratified). Together with 2026-10-10-scrolling-box-bounds-and-hit's padding-box stop, this is the main clause of decision/hit-testing-stops-at-paint-clip, which the list's exception qualifies.

@history_item n=33 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows): + decision/unavailable-families-use-default-font. An unavailable family falls back to the default font, and font-relative units measure against it. A larger authored font in the list row (debt 2) is measured through this.

@history_item n=34 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read after the graph chunk's code): re-pointed the single select's list citation to docs/capabilities.md:157-162 and an open list's row bounds to snapshot.rs:163-166. These are where the select-list facts and the row-bounds snapshot code are cited.

@history_item n=35 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows): turned the design master into rows, verified against the code, with fates set by the founder's approval. Every picked select row's current handle and wording dates from this, so the chunk's amendments name rows by handle (`+`/`~`/`−`).
