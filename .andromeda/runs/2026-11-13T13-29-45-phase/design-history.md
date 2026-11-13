@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — 17 `file:line` citations into merged upstream files re-pointed by the merge's measured line map, no claim text changed. The first sync's precedent: a take that moves upstream-owned lines owes a citation re-point at the wrap; this take (Stylo 0.24, Parley 0.13, 107 new commits) will move cited lines the same way.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans for every generic with system fonts off (`build_single_font_ctx`, `HarnessOptions.font_ctx`) so text measures the same on dev host and CI (later moved to architecture's Headless stand). Bears on the fonts rows: upstream's default-font fallback (`3a146c669`) and Parley 0.13 can move measurement and the `fonts-unavailable` refusal path.

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every member of the animating set alike, none held open, none waited on or advanced, reading time only through the harness clock. Origin of decision/settle-does-not-wait-on-animation; the clock invariant the take must keep through any conflict.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time; `click`, `type`, `press` leave the animation clock alone (superseded in part by one-virtual-clock). Origin of the driver-verbs-and-virtual-clock contracts the take must keep.

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `scroll_into_view` writes every nested scrolling box at once, innermost first, `Smooth` applying to the viewport alone; the driver's `scroll` instant everywhere; ratified by the founder as an engine change for every document. Origin of the into-view-instant and driver-scroll-instant rows; a fork-owned edit to upstream scrolling code the take must preserve (touch-fling citation re-pointed then too).

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — upstream narrowed the default link rule `a` → `a[href]`; the palette row followed with value unchanged. Precedent: an upstream default-sheet change taken by a sync lands as a design amendment at the wrap; this take may edit `default.css` again.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` prints one JSON line per command, no colour, no ESC byte, no terminal test; exit 0/1/2/3. Origin of decision/escher-session-agent-read-surface, escher-session-no-colour, driver-output-for-agents and the exit rows; the sink allowlist re-check and new upstream prints/log calls must keep stdout clean.

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node clipping by `overflow`, the root excepted. Origin of the hit-stop rows; "the hit stops and the scrolled-box bounds" are named surfaces every conflict resolution must keep.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC 2.0 only, uncoloured, no ESC byte; Err ending on stderr with status 3. Origin of the escher-session-mcp contracts the take must leave intact.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table, served by `help`; session errors carry a fixed `remedy`. Origin of decision/exit-statuses-from-schema-table and the help rows.

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot whose font context resolves no `sans-serif` family is refused `fonts-unavailable`, never painting text skipped; the shot's line and the MCP image block. Origin of the screenshot/fonts rows; upstream's default-font fallback (`3a146c669`) touches exactly the font resolution this refusal reads.

@history_item n=12 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default UA sheet now arrives once through `DocumentConfig::default()` after upstream #1189 removed `DioxusDocument::new`'s call; claim kept, path changed, confirmed by the founder. Origin of decision/default-ua-sheet-via-default-config and a precedent of reconciling an upstream mechanism change without changing the claim.

@history_item n=13 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the hit at an element's own box measured (img, iframe srcdoc, filled input, transformed-and-scrolled box) in both layout modes. Measured hit answers that transform/anonymous-box changes in the take (`563d3db73`) could move.

@history_item n=14 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's `23b9b26a9` "Delete the bump crate", taken by the sync, removed the `bump` app from §Surface: cli. Precedent: an upstream crate removal or addition (here `parley_code_editor`, devtools) reaches the cli surface rows by amendment.

@history_item n=15 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — the hit stop widened to a computed `contain` including `paint` (later ratified). Origin of decision/hit-test-stops-at-paint-clip's contain clause; a fork edit in hit code to keep through conflicts (`0980cce5f`'s `hit_inner` hunk).

@history_item n=16 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — upstream #1220 changed the cursor over no box to Default, taken by that sync as an engine behaviour change for every document, PROVISIONAL then ratified. Precedent: an upstream engine behaviour change is a design amendment needing the founder's word; this take's text-shadow, masks, alt-as-decorative and similar will follow the same path.

@history_item n=17 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`; `advance` moves it by exactly its ms with app time; click/type/press leave it; settle's `animating` reads the scrollbar fade off that clock. Origin of the virtual-clock rows; "the clock invariant" every resolution must keep.

@history_item n=18 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (Epoch 2 ratification) — the PROVISIONAL batch (contain-paint hit stop, one-virtual-clock, cursor fix) ratified by the founder at the epoch boundary. Why those rows read "Ratified"; a ruled fork edit kept through a take is named and recorded, not reopened.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` named in the exit-3 class, no new exit. Origin of contract/escher-session-exit-status/3's wording.

@history_item n=20 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting step waits on a load (at most `LOAD_WAIT`), the `VirtualClock` stands still. Origin of decision/load-wait-moves-no-clock; lossy page decoding (`9b9471f2f`) and loader changes in the take must keep it.

@history_item n=21 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — UA sheet: `select` takes the input border/padding/background, `inline-block`, and a single select shows its chosen option alone. Origin of decision/single-select-box-shows-chosen-label; upstream edits to `default.css` meet these fork rules.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text"}` or `{"text","part","parts"}`; over mcp a paged result is the JSON line. Origin of the snapshot command and mcp snapshot-result contracts; snapshot byte sizes may move with the take and are re-measured, not loosened.

@history_item n=23 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the hit walk skips a node dropped since the last resolve, as the paint walk does; an upstream engine change ratified by «Десять коммитов (Рекомендую)». The cherry-pick-with-pins-unchanged precedent the scope cites, and a hit-order change in the area this take's conflicts reach.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — right after a scroll the shown overlay scrollbar takes a pointerdown in its band as a thumb drag until an `advance` passes its fade; the cli serves six tasks, an unknown task a usage ending. Origin of decision/shown-scrollbar-takes-thumb-drag and contract/escher-session-tasks/unknown-task.

@history_item n=25 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations follow the script runtime's split) — the eleven-commit take («Одиннадцать коммитов (Рекомендую)») split blitz-vibey-script's runtime and moved blitz-shell lines; citations re-pointed to new paths, claims unchanged. Precedent for this take's wrap: moved or split upstream files make old paths read claim-false and must be re-pointed.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair) — `blitz-dom default chosen row` `#c8c8c8`/`#000000`, Chromium's inactive selected pair, hardcoded; the `select[multiple] > option:checked` rule colours it. Origin of decision/select-chosen-row-chromium-pair and engine-colours-hardcoded; guarded by `select_interaction`.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — a transform with determinant 0 or non-finite answers no hit for the subtree (`Node::hit_inner` returns `None`), ratified by the founder. The scope's merge-surfaces carry: `0980cce5f`'s `hit_inner` hunk stays a textual conflict with this answer; the resolution must keep it.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through a clipping box's transforms) — `visible_region` reads each clipping box through its transforms (exact under translate/scale, bounding box under rotation, `None` for non-finite). Upstream's pin `563d3db73` "Resolve transforms through anonymous block boxes" touches transform resolution this reading rests on.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups) — default-sheet rules for `multiple` rows (block, nowrap, padding 0 2px) and a single select's `optgroup` `display: contents`. Fork rules in `default.css` that upstream sheet edits must merge around; part of "the select model" to keep.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants) — `SelectListView` constants for highlighted row, disabled row and border, not tokens and not in `default.css`; the highlight shares the chosen-row pair. Origin of decision/select-list-colours-engine-constants and engine-colours-hardcoded.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's plain pair is measured) — the open list's plain row pair has no constant, measured by `select_popup_painted`. A paint reading a Parley/vello move could shift; re-measured, never loosened.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — `select_list_hit` answers first in `hit_with_scrollbar`, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read it; a box clipping the select clips no row; `hit_inner` unedited. Origin of decision/open-select-list-hit-first and the select-row exception in hitting-stops-at-paint-clip.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`visible_region` on a row of an open list) — `visible_region` on a row or group of an open list answers the list's scrollport; the driver's lint reads it alone. A fork answer in a reader the take's transform changes reach.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list paints last) — the list paints last in `paint_scene`, after the root render and before devtools overlays, whole past `opacity` ancestors and clipping boxes. Origin of decision/select-list-painted-after-root; upstream's DevTools/CDP, masks and text-shadow touch `paint_scene` and the overlay order.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — wheel over the list scrolls it, elsewhere closes then scrolls; a key brings the row into view at once; the list has no motion. Origin of decision/select-list-wheel-and-key-scroll and select-list-opens-without-motion.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (into view on a row of an open list) — `scroll_into_view` on an open-list row writes the list's `scroll_y` and returns, moving no chain or viewport. The exception on decision/into-view-scroll-instant-in-nested-boxes.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (row cap, placement and row height) — `MAX_ROWS` 20, engine-constant padding, `place` inside the viewport at least the select's width and at most the viewport's, every edge finite; row height from the select's font via `select_row_layout`. Origin of decision/select-list-placed-in-viewport; Parley 0.13 can move row heights.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — the list is an engine overlay reaching the screen through `paint_scene` alone; click, Enter or Space with no typed text opens it. Origin of decision/select-list-engine-overlay and single-select-open-gestures.

@history_item n=39 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes) — open/highlight/choose/close event rules, and which selects open no list. Origin of the select-list-events contracts and decision/selects-that-open-no-list; "the select model and its list" every conflict resolution must keep.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — group label painted as a header via `RunPaint { color, synthesize_bold }` in `stroke_text_as`; hinting off on emboldening builds. The `embolden` witness the scope requires after the take rests on this; a text-paint path the take (text-shadow, Parley, vello 0.12) can reach.

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — highlighted row in no snapshot or tree; open select `ComboBox` + `expanded`; chosen option `selected`; author `open` steers nothing; row `bounds` in the snapshot and a row screenshot crops. Origin of the select-accessibility, snapshot and screenshot row contracts; upstream's AccessKit refresh (`be60597f6`) meets these reads.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's thumb) — a scrolled list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable. Part of the list's paint and hit kept through the take.

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the citations re-pointed past the take) — the last take's citation re-point across every area, digits only, with measured offsets per file. The direct precedent for this take's wrap amendment.

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation`) — upstream #1316 made both properties paint and stack; the open list paints past the blend layers; no frame test paints a blended element. The carried `cry-jbkftesv` asks for that frame test at this take, both layout modes.

@history_item n=45 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip) — upstream #1319: an inline box holding only block children paints no own shadow, background or border; unit tests only, no frame test. The other half of the carried frame-test work at this take.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — escher's fix in `BaseDocument::deallocate_anonymous_block` keeps a focused select's focus and keys after its container lost its box (F1 A). Origin of decision/boxless-focused-select-keeps-focus; the carried merge surface P3 reads against `563d3db73`'s anonymous-block-box transform change.

@history_item n=47 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — render recorded into a mutation buffer unborrowed, then replayed under one borrow, on every Dioxus document; a render-time `MountedData` call sees the previous render. Origin of decision/buffered-render-replay and the mounteddata-during-render contracts; the carried merge surface (`dioxus_document.rs`, `mutation_buffer.rs`, replaced whole if Dioxus 0.8 is taken).

@history_item n=48 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `escher-session graph` answer line and its content bound (ids, names, closed flags, booleans). Origin of the graph-answer, graph-arguments and ui-graph-line contracts; the UI graph lives in dioxus-native-dom, the most-modified crate the take touches.

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line) — `actions` answer line, its node set in snapshot pre-order, content bound, and verbs decided by the same `validate`/`check_target`. Origin of decision/actions-share-the-verbs-checks and the actions-line contracts.

@history_item n=50 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role's answer line, endings and refusals) — `coverage <app> <check>...` answer line, shape, bounded strict reads and refusals, no new exit status. Origin of the coverage contracts and coverage-check-files rows.

@history_item n=51 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master turned into rows (137), each verified against the code at `edd0cfaea7f0`. Why the picked rows read as handles; any row a take moves is amended by handle (`~`), verified against the merged code.
