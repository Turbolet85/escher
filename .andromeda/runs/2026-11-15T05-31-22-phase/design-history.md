@history_item n=1 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows; Spacing: 1 row) — added `+ decision/unavailable-families-use-default-font`, `+ decision/font-element-face-and-size` and `+ decision/table-cell-spacing-presentational-hints`, each an upstream engine change "ratified as upstream's" on the operator's Q1. This is the direct precedent for this take's new presentational hints (`hr` `color`/`noshade`/`size`, SVG presentation attributes and the inline SVG `transform`, the hint-helpers refactor `0bdfe4fa3`). Each hint the take carries changes how every document renders, so it needs a row of its own, worded the same way. It also extends the hint rows this refactor rewrites.

@history_item n=2 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** (§Iconography: cursor over no box) — an upstream engine change for every document (#1220), taken PROVISIONAL on the founder's delegate and ratified later in the epoch-boundary batch (2026-10-18-engine-debts ratification). This is the pattern for this chunk's PROVISIONAL engine widenings while the founder is away.

@history_item n=3 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (Core Colors: default link rule `a[href]`) — an upstream change to the default sheet, recorded as a design amendment with the value unchanged. This is the precedent for upstream `default.css` or hint edits in this take, such as `hr` styling.

@history_item n=4 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (Surface Tokens: default sheet through the config) — moved the UA sheet's path to `DocumentConfig::default()` after upstream #1189. This is the origin of `decision/default-ua-sheet-via-default-config`, a resolution the take keeps if upstream touches document construction.

@history_item n=5 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — 17 `file:line` citations re-pointed by the merge's measured line map, with no claim changed. This is the standing sync precedent: any row citation into a merged file is re-pointed at wrap.

@history_item n=6 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations follow the taken commits' line moves) — an earlier take's re-point of moved paths and line ranges (blitz-vibey-script split, blitz-shell, `tab.rs`). This is a sync precedent for the citation sweep after this take's SVG, paint and hint edits.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations re-pointed past the take; `mix-blend-mode` and `isolation` in the depth rules; the inline box's paint skip) — the last full sync's design entries. Upstream #1316 made the stacking-root list and the open list's paint-last row (`decision/select-list-painted-after-root`) read "past the blend layers". This take's SVG isolated layer (`beaa796f4`) and SVG image paint clipping (`d176ac4e6`) touch the same paint order, so check that the list still paints last and that the stacking rows hold.

@history_item n=8 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a focused select past its dropped block) — the escher-owned `BaseDocument::deallocate_anonymous_block` panic fix, which is the source of `decision/boxless-focused-select-keeps-focus`. It is a merge surface named in the folded freight (`cry-oglnc26y`), so any conflict resolution must keep it.

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (hit walk's edge rule) — the hit stops at the padding box of a node clipping by `overflow`, with the root excepted. This is the origin of `decision/hit-test-stops-at-paint-clip` and `decision/hit-testing-stops-at-paint-clip`, escher's `clips_to_padding_box` arm in `Node::hit_inner`. The inline-root coordinate carry (`cry-irb7ecz4`) is owed if this take touches `hit_inner`.

@history_item n=10 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (hit stops at a `contain: paint` box; the PROVISIONAL batch ratified at the Epoch 2 close) — widened the hit stop to a computed `contain` that includes `paint`. It was ratified together with the one-virtual-clock and cursor entries. This is a constraint on any `hit_inner` conflict resolution.

@history_item n=11 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the hit walk skips a dropped node) — an upstream hit-walk change taken and ratified, with `node.rs` cites re-pointed. It is the latest upstream edit to the hit walk that a sync carried, and the precedent for reconciling upstream `hit_inner` hunks with escher's stop.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit; `visible_region` through transforms) — `Node::hit_inner` answers `None` for a subtree whose transform is singular. `0980cce5f`'s `hit_inner` hunk is a textual conflict with this answer (folded merge surface `cry-oglnc26y`), so the resolution keeps both.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a `multiple` select's chosen-row pair; the default sheet's `multiple` rows and a select's groups) — added `#c8c8c8`/`#000000` as the chosen row's pair and the `select[multiple]`/`optgroup` sheet rules. This is the source of `decision/select-chosen-row-chromium-pair` and `decision/engine-colours-hardcoded`. Any upstream `default.css` edit has to keep these rules.

@history_item n=14 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the user-agent select box) — the UA sheet gives `select` the input border, padding and background, and a single select shows its chosen option alone. This is the origin of `decision/single-select-box-shows-chosen-label`, part of the select model that the take's resolutions must keep.

@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants; its plain pair measured) — `SelectListView` constants that are not in `default.css` and not tokens. This is the source of `decision/select-list-colours-engine-constants` and `decision/engine-colours-hardcoded`. Colours in the sheet stand rejected.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list answers the hit before the page's walk; `visible_region` on a row; paints last, whole past clips and opacity) — `select_list_hit` runs first in `hit_with_scrollbar` while `hit_inner` stays unedited, and `paint_select_list` runs last in `paint_scene`. This is the source of `decision/open-select-list-hit-first`, `decision/select-list-painted-after-root` and the `except` on `decision/hit-testing-stops-at-paint-clip`. Upstream paint and hit hunks in this take must keep both call sites.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (wheel and key, and no motion; into view on a row; row cap, placement and row height; `multiple` with `size`) — the source of `decision/select-list-wheel-and-key-scroll`, `decision/select-list-opens-without-motion`, the `except` on `decision/into-view-scroll-instant-in-nested-boxes`, and `decision/select-list-placed-in-viewport` with its finite-edge guards.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay; how it opens, moves, chooses and closes; what the tree, snapshot and shot read; the open list's thumb) — the source of `decision/select-list-engine-overlay`, `decision/single-select-open-gestures`, `decision/selects-that-open-no-list`, and the `contract/select-list-events/*`, `contract/select-accessibility/*`, `contract/snapshot/select-list-row-bounds` and `contract/screenshot/select-list-row` rows. These form the select model the scope says every resolution keeps. The model replaced the earlier 2026-10-31-engine-debts "no select popup is painted" line.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — group headers are synthesized bold through `RunPaint { color, synthesize_bold }` in `stroke_text_as`, and hinting is off on an emboldening build. The scope lists the `embolden` witness among the proofs that must still pass. This take's `@font-face` `font-stretch` change (`f5ea1caf1`) touches font selection near this path.

@history_item n=20 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (no font refused) — a `screenshot` whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted. This is the source of `decision/screenshot-never-skips-text` and the `contract/driver-screenshot*/fonts-unavailable` rows. The take's font-loading change (`f5ea1caf1`) must not break that resolution.

@history_item n=21 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** (the stand registers the bundled DejaVu Sans) — a single bundled face, set through `HarnessOptions.font_ctx`, so text measures the same on the dev host and the CI runner. The registration later moved to architecture (2026-10-27-readme-and-records, PROVISIONAL, then ratified). It bears on `contract/native-config/font-context` and on per-OS tallies if `@font-face` loading moves.

@history_item n=22 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's font is now a contract of its headless boot in architecture, not a design-system value. Read it alongside `decision/no-generic-family-lays-out-sans` when the `@font-face` change is reconciled.

@history_item n=23 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (an into-view scroll never animates a nested box) — the source of `decision/into-view-scroll-instant-in-nested-boxes`, `decision/into-view-scroll-nested-boxes-instant` and `decision/driver-scroll-instant`, ratified by the founder. It is an engine widening in `scroll_into_view`, which the take's resolutions must keep.

@history_item n=24 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (settle's one answer for the keeps-animating set) — settle never waits on or advances animation, and reads time only through the harness clock. This is the source of `decision/settle-does-not-wait-on-animation`, part of the clock invariant the scope names.

@history_item n=25 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (`advance` moves app time, leaving the animation clock) — the first form of the driver-verbs clock rule. The 2026-10-18 one-virtual-clock entry replaced it.

@history_item n=26 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the harness drives animation from the documents' clock) — `VirtualClock` drives the fade, smooth scroll and fling. `advance` moves it and app time together, and `click`/`type`/`press` leave it alone. This is the source of `decision/animation-driven-by-virtual-clock`, `decision/driver-advance-moves-one-clock` and the `contract/driver-verbs-and-virtual-clock/*` rows, ratified at the Epoch 2 close. It is the clock invariant every resolution keeps.

@history_item n=27 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (the load wait moves no clock) — the source of `decision/load-wait-moves-no-clock`, which the clock invariant also covers.

@history_item n=28 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (Motion: the overlay scrollbar's band) — a shown overlay scrollbar takes a pointerdown in its band as a thumb drag, so driver clicks need an `advance` past the fade. This is the source of `decision/shown-scrollbar-takes-thumb-drag`, which bears on stand checks re-run after the take.

@history_item n=29 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (render-time MountedData calls and their previous-render limit) — escher ported the buffered render onto the Dioxus 0.7 bridge. This is the source of `decision/buffered-render-replay` and the `contract/mounteddata-during-render/*` rows. It is a merge surface flagged not upstreamable (`cry-2hvzi4ns`), and `dioxus_document.rs` must keep it through the take.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — the graph line carries ids, names, closed flag words and booleans only. This is the source of `contract/ui-graph-line/content`, which was ratified at 2026-11-15T05-21-42-wrap. The take must not add a value-bearing field to dioxus-native-dom's graph.

@history_item n=31 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified: the graph answer line; ratified: the coverage role) — the latest entries. The PROVISIONAL graph line is now ratified, so `contract/ui-graph-line/content` stands as settled and the take's resolutions keep it.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** (the masters as rows) — the master was converted to rows, each checked against the code. Amendments from this chunk name rows by handle (`+`/`~`/`−`), so any new presentational-hint or paint rows the take needs are added in that form, as entry n=1 did.
