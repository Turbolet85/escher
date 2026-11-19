@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — 17 `file:line` citations into merged upstream files re-pointed by the merge's measured line map, no claim text changed. Precedent: a two-parent upstream merge moves cited lines across the design areas, and this chunk's 282-file merge owes the same measured re-point.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers bundled DejaVu Sans via `build_single_font_ctx` so text measures the same on host and CI. Rests under the typography rows (wasm-bundled-font-context, screenshot-never-skips-text). The parley 0.12 → 0.13 and skrifa 0.44 → 0.49 move this merge carries must keep that pinned measurement, and any moved text number is re-measured, never loosened.

@history_item n=3 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers the whole animating set the same way, reading animation time only through the harness clock and never advancing it. This is the origin of decision/settle-does-not-wait-on-animation. Upstream scroll and animation commits in the take must not reopen that rule.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` was first split from the animation clock, with click, type and press leaving it alone. Later superseded by one-virtual-clock (n=14). It explains why the driver-verbs-and-virtual-clock contracts read as they do.

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `scroll_into_view` writes every nested scrolling box at once, innermost first, with Smooth applying to the viewport alone, and the driver's `scroll` is instant (founder-ratified 2026-10-07). This is the source of decision/into-view-scroll-instant-in-nested-boxes, decision/into-view-scroll-nested-boxes-instant and decision/driver-scroll-instant. It is an engine-wide widening that merge resolutions in `scrolling.rs` must keep (the "reversed scroll origins and into-view readings" of the scope).

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — upstream narrowed the default sheet's link rule from `a` to `a[href]`, recorded as a design amendment. Precedent: an upstream `default.css` change arriving by sync is amended into the design rows. The take's sheet edits must be read the same way.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — fixes the cli surface's output form: one uncoloured JSON line per command, no ESC byte, no terminal test, and exit classes 0/1/2/3. The escher-session command rows (actions, graph, coverage, snapshot answers) rest on it. The merge's new upstream crates (CDP server, parley_code_editor) must not pull in colour or terminal dependencies on that surface.

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node that clips by overflow, root excepted. This is the origin of decision/hit-test-stops-at-paint-clip and decision/hit-testing-stops-at-paint-clip. It is one of the "hit stops and scrolled-box bounds" every conflict resolution must keep.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout carries only newline-delimited JSON-RPC lines, and a snapshot's tool result is the screen text. This underlies contract/escher-session-mcp/snapshot-result. A Dioxus 0.8 move that touches the session must keep it.

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a screenshot with no sans-serif family is refused `fonts-unavailable` with nothing painted, and the shot's answer line and MCP image block are set (founder fork 1). This is the source of the fonts-unavailable contracts and decision/screenshot-never-skips-text. The parley and skrifa bump in the take must keep `can_paint_text` reading the sans-serif generic.

@history_item n=11 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default UA sheet now arrives once through `DocumentConfig::default()` after upstream #1189 removed `DioxusDocument::new`'s own call. The claim stood and only its path changed (founder-confirmed). This is the origin of decision/default-ua-sheet-via-default-config. Precedent: a sync that reroutes a mechanism is reconciled, not worked around.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured hit answers at an element's own box (img, iframe srcdoc, text input, a transformed and scrolled box) in both layout modes and both builds. These are readings the hit rows rest on, and they must hold after upstream's paint-order and positioned-descendant commits (e.g. `f9d0e8c76`, #1445).

@history_item n=13 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — the `bump` app left §Surface: cli because of upstream `23b9b26a9` "Delete the bump crate". That commit is this merge's merge base. Precedent: an upstream crate added or removed by sync amends the cli and surface sections. This applies to `blitz-vibey-cdp-server` and `parley_code_editor` arriving now.

@history_item n=14 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (hit stops at `contain: paint`) the hit stop widened from overflow alone to a computed `contain` that includes paint, so hit and paint agree (PROVISIONAL, then ratified). This is the current form of the hit-stop rows, which resolutions in `node.rs` and hit code must keep.

@history_item n=15 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — upstream #1220 (`fb8d6c15e`) taken: a pointer over no box reads Default cursor while `cursor: none` still maps to None. It was taken on F1 B by delegate as an engine-wide change, later ratified. This is the source of decision/cursor-resolution and contract/shell-provider/set-cursor. Precedent: an upstream behaviour change for every document is a fork, not taken silently.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read (scrollbar fade, smooth scroll, fling). `advance` moves it by exactly its ms together with app time, and click, type and press leave it. Founder forks F1 B and F2 A, later ratified. This is the source of decision/animation-driven-by-virtual-clock, decision/driver-advance-moves-one-clock and the driver-verbs contracts. It is "the clock invariant" the merge must keep across upstream scroll and animation edits.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (ratification batch) the PROVISIONAL marks on the contain-paint hit stop, the one virtual clock and the cursor fix ratified by the founder (2026-10-18). Confirms those rows stand as founder-ratified constraints on this merge.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting step waits on a load (at most `LOAD_WAIT`), the `VirtualClock` stands still. This is the source of decision/load-wait-moves-no-clock, and the scope keeps it alongside "the stamped sub-resource request and the `file:` refusal". Upstream net and loader commits must not move the clock.

@history_item n=19 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — UA sheet: `select` takes the input border, padding and background, and a single select shows its chosen option alone (F2, founder-ratified). This is the source of decision/single-select-box-shows-chosen-label. Upstream `default.css` edits in the take must keep these rules.

@history_item n=20 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text"}` or `{"text","part","parts"}`, and the mcp paged result is that JSON line (founder F1). This is the source of contract/escher-session-commands/snapshot and contract/escher-session-mcp/snapshot-result.

@history_item n=21 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — upstream's change makes the hit walk skip a node dropped since the last resolve, as paint does, ratified by the founder («Десять коммитов»). Precedent of an upstream engine change landing in the depth rows on the founder's word. It also shows `node.rs` citations re-pointed by a sync.

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (motion item) right after a scroll, the shown overlay scrollbar takes a pointerdown in its band as a thumb drag, so a driver click there needs an advance past the fade first. This is the source of decision/shown-scrollbar-takes-thumb-drag. Upstream scrollbar or scroll commits in the take must keep it.

@history_item n=23 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (citations follow the script runtime's split) eleven upstream commits split blitz-vibey-script into `runtime/` and `dom/element/`. Citations for the JS viewport sizes, `devicePixelRatio`, scroll-behaviour and rAF were re-pointed, and blitz-shell `lib.rs`/`window.rs` lines moved (founder: «Одиннадцать коммитов»). Precedent for the js-window rows and shell rows when a sync restructures files: claims are re-read at their new sites.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (a `multiple` select's chosen-row pair) adds `#c8c8c8`/`#000000` as Chromium's inactive selected pair, hardcoded in the default sheet because the engine holds no tokens (F6). This is the source of decision/select-chosen-row-chromium-pair and decision/engine-colours-hardcoded. Upstream `default.css` changes must keep it.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (non-invertible transform takes no hit; `visible_region` through transforms) two engine-wide depth rules, founder-ratified: a transform with a zero or non-finite determinant takes no hit for its subtree, and `visible_region` reads each clipping box through its transforms. These are depth-area constraints that merge resolutions in `hit_inner`/`visible_region` must keep.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (default sheet `multiple` rows and single-select groups) `select[multiple]` rows render as blocks with nowrap and padding 0 2px, and a single select's `optgroup` takes `display: contents` (F6/F7). These are engine defaults in `default.css` that an upstream sheet merge must keep.

@history_item n=27 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (Cells into-view, no animation) the stand's Cells grid calls `MountedData::scroll_to(Instant)` synchronously because the async form panicked with `RefCell already borrowed`. This is the trap behind the later buffered render (n=44), which the Dioxus 0.8 move in this take must keep.

@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (list colours are engine constants) `SelectListView` constants HIGHLIGHT_BACKGROUND/TEXT, DISABLED_TEXT and BORDER are not tokens and not in `default.css`, and the highlight is the chosen-row pair (founder F6). This is the source of decision/select-list-colours-engine-constants and decision/engine-colours-hardcoded. Putting colours in `default.css` stands rejected.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (plain pair measured, value unstated) the open list's plain-row pair is measured by `select_popup_painted` and no constant holds it. A text or paint move from the parley or skrifa bump is re-read by this check.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (open list answers the hit first) `hit_with_scrollbar` answers `select_list_hit` before the page's walk, so `hit`, `set_hover_to`, `handle_pointerdown` and `covered` read it, and a box clipping the select clips no row. This is the source of decision/open-select-list-hit-first and the except clause of decision/hit-testing-stops-at-paint-clip. It is "the select model and its list" the merge must keep.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`visible_region` on a row of an open list) answers the list's scrollport for a row or group, and the driver's lint takes an open row's region from it. A merge conflict in `visible_region` must keep it.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (open list paints last) `paint_select_list` runs in `paint_scene` after the root render and before the devtools overlays, whole past opacity and clipping ancestors. This is the source of decision/select-list-painted-after-root. Upstream paint-order commits (e.g. #1445 `f9d0e8c76`) in the take must keep this place.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (wheel, key and no motion) a wheel over the list scrolls it and a wheel elsewhere closes it, then scrolls. A key brings the current row into view at once, and the list has no motion (founder F5). This is the source of decision/select-list-wheel-and-key-scroll and decision/select-list-opens-without-motion. `handle_wheel` resolutions must keep it.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (into view on a row of an open list) `scroll_into_view` on a row writes the list's `scroll_y` and returns, moving no chain and no viewport. This is the except clause of decision/into-view-scroll-instant-in-nested-boxes, and must be kept in any `scrolling.rs` conflict.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (row cap, placement and row height) MAX_ROWS = 20, and `place` keeps the list inside the viewport with every edge finite, between its select's width and the viewport's. Row height comes from `select_row_layout`. This is the source of decision/select-list-placed-in-viewport. A parley 0.13 layout change may move the row-height readings, which are re-measured, not loosened.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`multiple` select with `size`) `size_rows` hints `size × 1.2em` capped at 65534, and `select[multiple][size]` takes overflow-y auto with line-height 1.2. These are default-sheet and engine readings that upstream sheet and style edits must keep.

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (single select's list is an engine overlay) the list reaches the screen through `paint_scene` alone: not a second OS window, not `:open`, not rows as boxes (founder F1/F7/F8). This is the source of decision/select-list-engine-overlay and decision/single-select-open-gestures. Those alternatives stand rejected, so an upstream select or `:open` approach arriving by merge is a conflict to resolve in favour of the overlay.

@history_item n=38 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (open, move, choose and close events) the open, highlight-move, choose, choose-disabled and choose-chosen events, closing with no input or change, and the selects that open nothing (founder F2/F4). This is the source of the select-list-events contracts and decision/selects-that-open-no-list.

@history_item n=39 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (group header heavier than a row) `RunPaint { color, synthesize_bold }` in `stroke_text_as`, with hinting off only on an emboldening build. This is the `embolden` witness the scope requires after the merge, and the parley and skrifa bump touches this text path directly.

@history_item n=40 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (tree, snapshot and shot of an open list) the highlighted row is in no tree, the open select reads ComboBox expanded and its chosen option selected, and `open` steers nothing. Rows carry their own `bounds`, and a screenshot crops to a row and leaves the list open. This is the source of the select-accessibility contracts, contract/snapshot/select-list-row-bounds and contract/screenshot/select-list-row (the "accessibility state reads" and "the snapshot" the merge keeps).

@history_item n=41 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list's thumb) a scrolled list paints a thumb under `feature = "scrollbars"` that takes no hit and is not draggable. Upstream scrollbar changes must keep it.

@history_item n=42 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed past the take) the previous sync re-pointed `render.rs`, `document.rs`, `node.rs`, `stylo.rs`, `default.css`, `select_popup.rs` and blitz-paint `lib.rs` by measured offsets, with no value moved. This is the direct precedent for this merge's citation re-point.

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation`, and the inline box's paint skip) upstream #1316 and #1319 were taken on founder F2 A. `mix-blend-mode` other than normal makes a stacking root, and the list now paints past the blend layers. An inline box holding only block children paints no box of its own, and no frame test reads either. These are the current forms of decision/stacking-context-groups-paint-and-hit and decision/select-list-painted-after-root. Precedent: upstream paint changes enter the depth rows with their coverage gaps named.

@history_item n=44 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (a focused select past its dropped block) escher's own fix in `deallocate_anonymous_block` keeps focus and keys working with no panic (founder F1 A). This is the source of decision/boxless-focused-select-keeps-focus. Upstream layout-construction edits must keep the fix.

@history_item n=45 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow, ported onto the Dioxus 0.7 bridge. A render-time MountedData call sees the previous render, and `set_focus_to` is false for a node created in the same render. This is the source of decision/buffered-render-replay and the mounteddata-during-render contracts. The take moves Dioxus 0.7.3 → 0.8.0-alpha.3, so the buffered render must be re-ported and re-measured, never assumed (res-xpkmxnvl and res-xchmbfjr).

@history_item n=46 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cli surface's graph answer line) `graph` answers one uncoloured JSON line of ids, owners, component names, state names, handler events and closed flags, the same on every carrier. This is the source of contract/escher-session-commands/graph-answer and contract/ui-graph-line/content. It reads dioxus-native-dom's ids and graph, which the Dioxus 0.8 merge must keep.

@history_item n=47 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (citation ranges re-read) moved citations in `dioxus_document.rs` (MountedData during render), `snapshot.rs` (row bounds), `schema.rs` and `json.rs`. These are the files the Dioxus 0.8 and dioxus-native-dom resolutions will move again.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line) `actions` lists actionable nodes in snapshot pre-order, deciding each verb by the same `validate` and `check_target` the verbs meet, with `stale` for an unresolved node (founder-ratified boundary widening). This is the source of decision/actions-share-the-verbs-checks and the actions-line contracts. Its "listener for an event other than mounted" reads Dioxus listeners that the 0.8 move may reshape.

@history_item n=49 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (coverage role's answer line, endings and refusals) `coverage <app> <check>...` replays kept checks and answers `{"unexercised","checks","held"}`, adding no exit status. This is the source of contract/escher-session-commands/coverage-answer and contract/coverage-line/content.

@history_item n=50 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography, 2 rows) added decision/unavailable-families-use-default-font and decision/font-element-face-and-size as upstream's behaviour, ratified («Утвердить все 19»). This is the last sync's precedent: upstream behaviour arriving by merge becomes "ratified as upstream's" rows through the operator's Q1. The fallback row reads font-relative units against the default font, which the parley and skrifa bump may move.

@history_item n=51 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Spacing, 1 row) added decision/table-cell-spacing-presentational-hints (cellspacing, cellpadding and valign) as upstream's, ratified. This is the same precedent, for new upstream presentational hints in this take (e.g. the Stylo 0.24 move).

@history_item n=52 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — the graph answer line and the coverage role are ratified by the founder's own word, replacing the overseer's delegate confirm. Those cli contract rows stand founder-ratified, and the merge must not alter them.
