@history_item n=1 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list, the engine's select list in every document) — added the open-list bullet behind decision/open-list-rows-own-bounds: the list painted by `paint_scene` after the root's render, no OS window and no row a layout box; the row height on one line, up to twenty rows, placement, keys, focus staying on the select; while open each row reads its own `bounds` in snapshot and diff, and the highlight is in NEITHER the snapshot nor the tree. This is the exact source of debts 2 (row font/height), 3 (the highlight's reading) and 1 (highlight painted, measured only by `select_popup_painted`). F1–F4 and F7 are the founder's forks. A row's own `bounds` was read as no widening, a closed set inside the ratified crossing by the 2026-10-10 field rule. That is the precedent any highlight reading must meet, or be argued as a widening for the founder. The thumb stays non-draggable here and is out of scope.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the `except` clauses of decision/visible-region-through-clipping-boxes and decision/scroll-into-view-moves-every-scroller: an open list's row or group is seen through the list's scrollport alone, and `scroll_into_view` on a row writes the list's `scroll_y` at once. Any change to row height (debt 2) or to a single select rendered as a list box (debt 4) must keep these exceptions true or amend them. No ruling names them; they record the code.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_closed_select` retired for `Entry.in_select_box`) — `overlaps` skips options of a closed select and an open list's rows and groups. `closed_select_of` reads the element and never the role, so CRUD's `role="option"` rows are not a select's. A `multiple` select's rows are linted as usual, and `clip_region` takes an open row's region from `visible_region` alone. This was the founder's F10. Debt 1 (the highlighted row's and a multiple's chosen row's `contrast` unread by the lint) extends this lint path. A single select with `size` > 1 (debt 4) would change which entries `in_select_box` and `closed_select_of` cover.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — the flight booker's boot `overlap` (`flight-type-one-way` × `flight-type-return`) was retired by F9: `at_boot` `[0, 0, 0, 0]` in both layout modes, and the snapshot still gives each closed option its select's client rect. This underlies decision/stand-lints-clean-at-boot and decision/flight-type-select-boot-reading. Any lint or select change in this chunk must keep the stand's boot at zero findings and keep the options' boot reading.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` takes an `id`, `tabindex="0"`, `aria-label` "Cells", an `onkeydown` and NO role (contract/stand-element-ids/cells-grid). The arrows move `current`, shown only by the CSS outline `.cells-grid:focus .cell.current` in the stand accent. The grid became the 677th reacting element. One Tab stop is founder F13, and the handles are synchronous because the async form panicked. Debt 5 (Cells' selected cell, no grid role) builds directly on this: a role or a selected state is a new reading on a grid the founder shaped. Trap: the reacting-element count and `CELLS_TAB_ORDER` can move.

@history_item n=6 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand_contrast` over the load page and Cells) — `stand_contrast` reads Cells in both layout modes, its `cells-grid` outline included. If debt 5 adds a styled selected state, or debt 1 adds contrast reading, this is the check that guards the stand's colour pairs.

@history_item n=7 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed Cells' 676 data cells `cell-{A..Z}{0..25}` and `cell-input`, adding `id:` lines alone with no CSS selecting by them, and `unkeyed_actionable()` reads 0 on all seven tasks. These are the rows contract/stand-element-ids/cell-column-row, contract/stand-element-ids/cell-input and decision/every-stand-control-keyed that debt 5 rests on. Any selected-state attribute must follow the same attributes-alone form.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand states roles and states by attributes alone: `crud-list` `role="listbox"`, the rows `role="option"` with `aria-selected`, and the classes that style the states staying. This is the source of decision/stand-a11y-by-attributes-alone, and the precedent the scope cites for debt 5 ("the flight booker and CRUD state selected by attribute").

@history_item n=9 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs are named by `for` and `aria-label` alone, with no element, class, style or order changed. This is the origin of the attributes-alone rule that a Cells grid role or a selected state (debt 5) must keep.

@history_item n=10 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — the CRUD list takes focus and its keys, and the tree's focus is the list, never the row: no `aria-activedescendant` is read. The tree-focus gap was stated plainly at the operator's F5. This is the same gap as debt 3 (the tree hears the list open and not which row) and debt 5 (the grid focused, its current cell unread). It is a precedent for how a current-row reading was deferred before.

@history_item n=11 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named `People` by `aria-label` alone, and the current-option gap stayed stated (no `aria-activedescendant` read). That gap is still open, so it bears on debts 3 and 5.

@history_item n=12 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the flight booker's mode became the select `flight-type` with options `flight-type-one-way` and `flight-type-return`, stating the choice by `selected` (was toggle buttons with `aria-pressed`). Cells and the flight booker became served tasks (the served-tasks rows). The boot lint's select-option `overlap` was first owned as a residual. These are the origins of contract/stand-element-ids/flight-type*, contract/escher-session-served-tasks/cells and flight-booker, and of the stand on which the select debts are proven.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines on the desktop-native screens) — the mode-region bullet states the select's styling at 15px, weight 400, its two options showing the chosen one alone by the engine's select rule. This is the measured font that the list's one-line row height comes from today. Debt 2 (a larger authored font clips) needs a fixture beyond this 15px stand select, or it changes this row.

@history_item n=14 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (PROVISIONAL, later ratified) — moved the stand's colour rows here: the accent `#3a5ce5` with white text reading 5.47 against 4.5, `#4a6cf7` failing at 4.39, guarded by `stand_contrast` (decision/stand-accent-colour). Kept is the select row: `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, the `#3a5ce5` focus border. These are the colour pairs that debt 1's highlighted-row contrast reading would measure against.

@history_item n=15 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2 ratified: the stand rows' gap marker drops) — the founder ratified F2, so the stand's colour rows stand on this surface. Any colour change from debt 1 or 5 amends ratified rows.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint, `contrast` failure among its defects, at boot in both layout modes. `crud-person-14` reads `clipped` after twelve Creates. This is the origin of decision/stand-lints-clean-at-boot, the lint path debt 1 extends.

@history_item n=17 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — the `lint` verb answers `{"findings":[…]}` (contract/escher-session-lint/answer) and is refused `fonts-unavailable` with no font. Its F1 was ratified by the founder. A highlighted-row `contrast` finding (debt 1) would cross this carrier's answer and must stay inside its closed words and numbers.

@history_item n=18 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native: 8 rows) — `~ decision/stand-lints-clean-at-boot` now names the order page. That is the row's current list, and this chunk must keep all six pages at zero.

@history_item n=19 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the founder ratified, for every document (2026-10-07), the rule that an into-view scroll moves every holding scroller and that `visible_region` narrows through clipping boxes. These are the bodies of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, whose select-list exceptions this chunk may touch. An engine change reaches every document.

@history_item n=20 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — what a scrolled list reads, and that a click lands on the row at its centre after the bounds reader and hit fix. It bears on the readings of a single `size` list box (debt 4) and of the open list's rows when the row height changes (debt 2).

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role is the founder's ratified crossing (2026-10-10), booting through the same stand boot (decision/session-roles-share-stand-boot). The scope's "closed-value field inside a ratified carrier is no widening" rule for debt 3's highlight reading dates from this ratification.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the snapshot answer `{"text"}` / `{"text","part","parts"}` and `--part` 1–1000, on the founder's F1 and F2 (contract/escher-session-snapshot/answer and contract/escher-session-snapshot/part). A highlight reading in the snapshot (debt 3) crosses this carrier and must keep its shape.

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Surface: cli: 4 rows) — `+ contract/escher-session-layout/answer` and `~ contract/escher-session-mcp/tools` (fifteen tools, layout after actions). These rows are the current carrier forms, and a reading added by debts 2–4 must not open a new key on them without the founder.

@history_item n=24 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the screenshot answer, `fonts-unavailable`, the MCP image block and the result bound of 1,048,576 bytes (the rows contract/escher-session-screenshot/answer, contract/escher-session-refusals/fonts-unavailable, contract/escher-session-mcp/screenshot-result and contract/escher-session-mcp/result-bound). The highlight is seen today only in a shot (debt 3), and the founder's fork 1 ratified the image in band.

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps a session as one owner-only replayable check: `check-{n}.json` `0600` in a `0700` directory. These are decision/session-recording-replayable-check and contract/escher-session-record/check-file. A boundary widening ratified by the founder (F1 A, F2 B). Any reading added by debt 3 lands in the kept check.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds each call's masked answer and digest, replayed by `Check::replay` (founder F3 B). A new snapshot or diff reading changes the recorded answers and their digests, the "kept check" carrier named in the scope.

@history_item n=27 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role, and the coverage role a further consumer of the TaskShell mount) — `coverage <app> <check>...` reads each check bounded at 1,048,576 + 1 bytes and replays it on a fresh boot through the same stand boot. These are contract/escher-session-coverage/check-bound and contract/escher-session-coverage/answer, and decision/session-roles-share-stand-boot. Both were ratified at 2026-11-15T05-21-42-wrap. Kept checks replayed after a select or Cells change must still replay.

@history_item n=28 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host is a consumer of the TaskShell mount, adding no markup, and a held instance reads as a fresh boot with the mount and pinned viewport holding (decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot). Proofs in both layout modes run on this mount.

@history_item n=29 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand mounts one task in TaskShell at a pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans (decision/headless-stand-pinned-viewport). Headless proofs of the select fixes run here, and the font is fixed, which bears on debt 2's larger-font fixture.

@history_item n=30 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — author ids are attributes alone, with no class, style, wrapper or order and no CSS selecting by them. This is the origin of decision/stand-ids-as-attributes-alone, which any Cells or select markup change in this chunk must keep.

@history_item n=31 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the founder's rule that every element an agent can act on reads an author key (decision/every-stand-control-keyed). A list box rendered for a single `size` select (debt 4), or new Cells markup, must leave nothing unkeyed.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows, each verified against the code, with fates approved by the founder. Every picked row's current wording dates from here, so amendments in this chunk name rows by handle (`+`/`~`/`−`).
