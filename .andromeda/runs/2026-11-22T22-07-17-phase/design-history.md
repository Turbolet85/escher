@history_item n=1 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — 17 `file:line` citations into merged upstream files re-pointed by the merge's measured line map, no claim text changed. Precedent for this sync: the merge moves cited lines in upstream files (here blitz-paint, stylo_taffy, blitz-dom layout), and the wrap re-points them, claims unchanged.

@history_item n=2 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — the default link rule narrowed by upstream from `a` to `a[href]`, recorded as a row edit with the value unchanged. Precedent: an upstream default-sheet or engine behaviour change taken by a sync is recorded in the design master as upstream's behaviour.

@history_item n=3 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the default user-agent sheet arrives once through `DocumentConfig::default()` (upstream #1189), the founder confirming the reconcile. This is the origin of the picked row `decision/default-ua-sheet-via-default-config`. A merge that touches document config or the default sheet must keep it.

@history_item n=4 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — upstream #1220 (the cursor over no box reads Default), an engine behaviour change for every document, was taken PROVISIONAL on a fork answered by the founder's delegate. Precedent: an upstream engine change a sync carries is a fork for the founder, not a default. This chunk's operator directive sends every such widening to `evidence/founder-forks.md`.

@history_item n=5 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the hit-testing walk skips a node dropped since the last resolve, as the paint walk does. Upstream's engine change, ratified on the founder's word on the taken commits. The rows `decision/hit-test-stops-at-paint-clip` and `decision/stacking-context-groups-paint-and-hit` rest on it. The `display: contents` and `is_or_contains_block` commits touch the same layout-children and paint walk.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations follow the script runtime's split) — the take of eleven upstream commits split blitz-vibey-script's runtime and moved blitz-shell lines, so the citations were re-pointed with claims unchanged. The `document.fonts` stub went to `runtime/bootstrap/fonts.js`, behind the picked row `contract/script-runtime/document-fonts`. This merge's `DOMTokenList.replace` commit touches the same crate.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation` in the depth rules) — upstream #1316 extended the stacking-root list. The open select list now paints after the blend layers too, which is the picked row `decision/select-list-painted-after-root`. A take touching blitz-paint's render order (this merge's IME preedit underline) must keep the list painting last.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip) — upstream #1319: an inline box holding only block children (`is_inline_with_only_block_children`) paints no box of its own, and no frame test reads the skip. This merge's `is_or_contains_block` through `display: contents` and its `display: contents` pseudo-element and inline-wrapping commits change the same classification, a frame gap the record already names.

@history_item n=9 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — escher's fix in `deallocate_anonymous_block` (`layout_parent` cleared) for a select whose anonymous block was dropped. A conflict trap: upstream's `display: contents` and anonymous-block changes in this merge touch the same layout-children code, and the fix must survive the merge.

@history_item n=10 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the citations re-pointed past the take) — citation digits only, with offsets measured per file (`render.rs`, `document.rs`, `node/node.rs`, `default.css`, `select_popup.rs`, …). The most recent sync precedent for re-pointing citations after a take.

@history_item n=11 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — `+ decision/unavailable-families-use-default-font` and the font element's presentational hints, both ratified as upstream's, came in through a sync. The picked typography row's origin and the row-era precedent: an upstream take that adds a behaviour with no row gets a new row "ratified as upstream's".

@history_item n=12 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Spacing: 1 row) — `+ decision/table-cell-spacing-presentational-hints`, upstream's mapping, taken as a row. Precedent: in the row era a sync's new upstream layout behaviour (here float keyword sizes and logical float/clear sides) is weighed for a row.

@history_item n=13 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master was turned into 137 rows, each checked against the code. Every picked row's handle dates from here. Later changes are row edits (`+`/`~`/`−`), so any amendment this sync raises takes that form.

@history_item n=14 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node that clips by `overflow`, root excepted. Origin of `decision/hit-test-stops-at-paint-clip` and `decision/hit-testing-stops-at-paint-clip`. "The hit stops and scrolled-box bounds" are in the scope's keep-both-sides list.

@history_item n=15 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured the hit at an element's own box (`img`, `iframe` srcdoc, a filled text `input`, transformed and scrolled boxes) in both layout modes and both builds. These readings must hold after the merge, which touches the textarea's layout.

@history_item n=16 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the hit stops at a `contain: paint` box) — widened the hit stop to a computed `contain` that includes `paint`. PROVISIONAL, then ratified at the Epoch 2 close (the entry of the PROVISIONAL batch). It holds the current wording of both hit-stop rows.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder) — ratified the `contain: paint` hit stop, the virtual clock's `advance`, and upstream #1220's cursor fix. This is the ratification standing behind the picked hit-stop and `decision/driver-advance-moves-one-clock` rows, and it is how a PROVISIONAL sync take is discharged.

@history_item n=18 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — `Node::hit_inner` answers `None` for a subtree under a non-invertible transform, an engine change ratified by the founder. Part of the hit model the merge must keep.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — `select_list_hit` in `hit_with_scrollbar` answers first, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and `hit_inner` was left unedited. This is the origin of `decision/open-select-list-hit-first` and of the select-row exception in `decision/hit-testing-stops-at-paint-clip`.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list paints last, whole past clips and opacity) — `paint_select_list` is called in `paint_scene` after the root's render and before the devtools overlays. This is the origin of `decision/select-list-painted-after-root`. blitz-paint edits in this merge (the IME preedit underline) must not move that call.

@history_item n=21 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`visible_region` on a row of an open list) — `BaseDocument::visible_region` answers a row's list scrollport. Part of the scrolled-box bounds that the scope's conflict list keeps.

@history_item n=22 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants) — the `SelectListView` constants (`HIGHLIGHT_BACKGROUND`, `HIGHLIGHT_TEXT`, `DISABLED_TEXT`, `BORDER`) are hardcoded, are not in `default.css`, and add no token. This is the origin of `decision/engine-colours-hardcoded` and `decision/select-list-colours-engine-constants`.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair) — `#c8c8c8`/`#000000` is Chromium's inactive selected pair, set in the default sheet by the `select[multiple] > option:checked` rule. This is the origin of `decision/select-chosen-row-chromium-pair`. A merge touching `default.css` must keep the rule.

@history_item n=24 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (Color Palette: 1 row) — `~ decision/select-chosen-row-chromium-pair` widened to every list box: a multiple select, or a single select sized above 1. This is the row's current wording.

@history_item n=25 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (Surface: desktop-native: 3 rows) — `~ decision/single-select-open-gestures`, `~ decision/selects-that-open-no-list` and `~ contract/select-list-events/close-on-select-change`: a single select sized above 1 opens no list, and a size write closes it. The current select model, which the conflicts must keep.

@history_item n=26 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (Surface: desktop-native: 1 row) — `− decision/single-select-box-shows-chosen-label`: the drop-down's own box is now a mechanism the code holds and no longer a row. Upstream's `display: contents` changes reach the sheet rules that build that box.

@history_item n=27 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the user-agent `select` box and the rule that shows a single select's chosen option alone (`option { display: none; }`, `select:not([multiple]) > option:checked { display: inline; }`) are engine defaults with no token. Sheet rules that a merge touching `default.css` or display handling must keep.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups) — a single select's `optgroup` is `display: contents` (`select:not([multiple]) > optgroup:not([hidden])`) and `multiple` rows are `display: block`, F6 and F7 by the founder. A direct trap: upstream's `display: contents` commits (pseudo-elements, inline-only wrapping, `is_or_contains_block`) change how these groups lay out.

@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a `multiple` select with `size`) — `size_rows` hints the height and `select[multiple][size]` takes `overflow-y: auto` with line-height 1.2. Part of the list-box select model the merge must keep.

@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height) — `MAX_ROWS` = 20, and `place` keeps the list inside the viewport with every edge finite. This is the origin of `decision/select-list-placed-in-viewport`.

@history_item n=31 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — the list reaches the screen through `paint_scene` alone, not through a second OS window or rows as boxes, and opens on a click, Enter or Space. This is the origin of `decision/select-list-engine-overlay` and `decision/single-select-open-gestures`.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes) — the origin of the picked `contract/select-list-events/*` rows (open, choose, choose a disabled row, choose the chosen option, close, close on its select's change) and of `decision/selects-that-open-no-list`.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — `handle_wheel` scrolls the list or closes it, and the list has no motion (the founder's F5). This is the origin of `decision/select-list-wheel-and-key-scroll` and `decision/select-list-opens-without-motion`.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (into view on a row of an open list) — `scroll_into_view` on a row writes that list's `scroll_y` and returns. This is the exception in `decision/into-view-scroll-instant-in-nested-boxes`.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — `RunPaint { color, synthesize_bold }` in `stroke_text_as`. A synthesized bold adds one embolden weight, and hinting stays `!FONT_EMBOLDEN_ENABLED`. This is the `embolden` witness the scope keeps after the merge. Changes to blitz-paint's text strokes (the IME preedit underline) sit beside it.

@history_item n=36 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — the tree reads `ComboBox`/`expanded`/`selected`, an author's `open` steers nothing, a row carries its own `bounds`, and a shot of a row crops to it. This is the origin of `contract/select-accessibility/*`, `contract/snapshot/select-list-row-bounds` and `contract/screenshot/select-list-row`.

@history_item n=37 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ contract/select-accessibility/highlighted-row`, `~ contract/select-accessibility/open-select` (active descendant, page `aria-activedescendant` not read) and `~ contract/select-list-events/highlight-move` (a move to another row marks its select changed). The current wording of three picked rows, part of the accessibility state reads in the scope's conflict list.

@history_item n=38 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — `BaseDocument::scroll_into_view` writes every nested scrolling box at once, innermost first, and Smooth applies to the viewport alone, ratified by the founder because the document holds one scroll animation. This is the origin of both into-view rows. "The into-view readings" are in the scope's conflict list.

@history_item n=39 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every keeps-animating member the same way and reads animation time only through the harness clock, never advancing it. This is the origin of `decision/settle-does-not-wait-on-animation`, part of the clock invariant the merge must keep.

@history_item n=40 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves the app's own time, while `click`, `type` and `press` leave the animation clock alone. The earlier form of the clock rows, superseded by the one-virtual-clock entry. It explains the picked `contract/driver-verbs-and-virtual-clock/*` rows.

@history_item n=41 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read (scrollbar fade, smooth scroll, fling), and `advance` moves it by exactly its `ms` together with app time. This is the origin of `decision/animation-driven-by-virtual-clock`, `decision/driver-advance-moves-one-clock` and the `driver-verbs-and-virtual-clock` contracts. An upstream change reading wall-clock time in an animation path would break it.

@history_item n=42 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting step waits on a load, for at most `LOAD_WAIT`, the `VirtualClock` stands still. This is the origin of `decision/load-wait-moves-no-clock`, part of the clock invariant.

@history_item n=43 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — every Dioxus document records a render into a mutation buffer and replays it under one borrow. This is ported onto the Dioxus 0.7 bridge, and a call during a render reads the previous render. This is the origin of `decision/buffered-render-replay` and the `mounteddata-during-render` contracts. It bears directly on the Dioxus 0.7.3 hold fork: a 0.8 move would have to re-port the buffered render.

@history_item n=44 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Surface: desktop-native: 1 row, `~ contract/mounteddata-during-render/document-state`) — re-stated as the reachable case: a task awaiting a node's handle focuses it once mounted, and a removed target leaves focus off. The current wording of the picked row.

@history_item n=45 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Surface: desktop-native: 1 row, `− decision/boxless-focused-select-keeps-focus`) — retired: the post-layout focus fixup now gives up unshown focus, and the rule's home is a11y-plan. Upstream's `display: contents` and anonymous-block changes touch the same box-loss path.

@history_item n=46 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation) — the stand's handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously, because the async form panicked with `RefCell already borrowed`. This is the trap the buffered render later removed, and it is relevant to the Dioxus hold.

@history_item n=47 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a `screenshot` is refused `fonts-unavailable` with nothing painted when no `sans-serif` family resolves. The answer is one uncoloured line `{"width","height","png"}`, and MCP returns an image block and then a text block. This is the origin of the picked screenshot and fonts-unavailable contracts and of `decision/screenshot-never-skips-text`. The screenshot readings are re-measured after the merge and never loosened.

@history_item n=48 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans through `build_single_font_ctx(DEJAVU_SANS)`, so text measures the same on the dev host and CI. Behind `contract/native-config/font-context` and `contract/native-config/wasm-font-context`, and behind the stability of the stand's screenshot readings across the merge.

@history_item n=49 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font registration is held in architecture §Headless stand. The WASM bundled-font item and the custom `FontContext` item stay here as the picked typography contracts. Ratified on 2026-10-27.

@history_item n=50 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` writes one uncoloured JSON line per command, with no ESC byte and no terminal test, and exit statuses 0/1/2/3. The form under the picked `escher-session-commands` screenshot, snapshot and layout-answer contracts, which the merge must leave untouched.

@history_item n=51 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text"}` or `{"text","part","parts"}`, and the MCP paged result is that JSON line. This is the origin of `contract/escher-session-commands/snapshot` and `contract/escher-session-mcp/snapshot-result`.

@history_item n=52 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer` (and layout arguments): `layout --id` answers one uncoloured JSON line on every carrier. The picked row's origin. Upstream layout changes (float sizes, `display: contents`, the textarea width) move the layout readings but not this shape.
