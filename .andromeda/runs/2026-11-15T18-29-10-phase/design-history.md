@history_item n=1 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 rows) — added `+ decision/unavailable-families-use-default-font` and `+ decision/font-element-face-and-size`, each "taken from upstream, ratified as upstream's" on the operator's relay of «Утвердить все 19 (Рекомендую)». This is the precedent for how an upstream take's behaviour change for every document lands as a row. The git-Taffy chain changes layout for every document, and the scope leaves open whether each layout change needs its own ratification, so the measurement must name the rows it moves in the same way.

@history_item n=2 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Spacing: 1 row) — added `+ decision/table-cell-spacing-presentational-hints` (cellspacing, cellpadding, valign), ratified as upstream's. Table and cell spacing is resolved in layout, which Taffy does, so a Taffy move can change what this row's guard reads. It is the same ratified-as-upstream precedent.

@history_item n=3 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned every area's master text into rows (137 rows, with guards; the code holds the rest), each row checked against the code. Any change this chunk makes to the design-system is a `+`, `~` or `−` row entry against a picked row's handle and guard, not a prose edit.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the Typography → Loading rule that a driver `screenshot` whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted. This is the origin of `decision/screenshot-never-skips-text` and the two `fonts-unavailable` contracts. The founder ratified it (fork 1). A Taffy move must leave this refusal and text measurement unchanged.

@history_item n=5 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — moved the stand's bundled DejaVu Sans registration (`build_single_font_ctx` through `HarnessOptions.font_ctx`) to architecture §Standard Contracts → Headless stand. The WASM bundled-font item and the custom `FontContext` item stayed (`decision/wasm-bundled-font-context`, `contract/native-config/font-context`, `contract/native-config/wasm-font-context`). The single bundled font is what makes stand layout deterministic across hosts, so any layout drift a Taffy move causes shows against it. The `examples/wasm_hello` leg rests on the WASM font-context rows.

@history_item n=6 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — set the hit walk's edge rule: the walk stops at the padding box of a node that clips by `overflow` (the root element excepted), as paint clips. This is the origin of `decision/hit-test-stops-at-paint-clip` and `decision/hit-testing-stops-at-paint-clip`. Padding and border boxes come from Taffy's layout output, so the move must keep `hit_clipped_at_scrolling_box` green.

@history_item n=7 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — widened the hit stop to a computed `contain` that includes `paint` (first PROVISIONAL, later ratified by the founder, 2026-10-18). This is the current wording of the paint-clip hit rows.

@history_item n=8 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the hit walk skips a node dropped since the last resolve, as the paint walk does. It was an upstream engine change for every document, ratified by the founder's own word «Десять коммитов (Рекомендую)». This is a precedent for upstream behaviour changes in the depth area being ratified by the founder, which bears on the open fork about Taffy's layout changes.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — added the exception that a row of an open select list is answered before the walk, so a box clipping the select clips no row and a button under the list is refused `covered`. This is the origin of `decision/open-select-list-hit-first` and the except-clause of `decision/hit-testing-stops-at-paint-clip`.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list paints last, whole past clips and opacity) — the list paints last in `paint_scene`, after the root render, and whole past an `opacity` ancestor and a box clipping its select; a non-finite box is unpainted. This is the origin of `decision/select-list-painted-after-root`.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height) — capped the list at `MAX_ROWS` = 20 and set `place`: at least the select's width, at most the viewport's, inside the viewport, every edge finite. Row height is one line of the select's font. This is the origin of `decision/select-list-placed-in-viewport`. Placement reads the select's laid-out box, so a Taffy move must keep `every_edge_of_a_placed_list_is_finite` and `hostile_markup_keeps_every_box_finite_and_panics_nowhere` green.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — a wheel over the list scrolls it, a wheel elsewhere closes it, a key brings the current row into view at once, and there is no motion. This is the origin of `decision/select-list-wheel-and-key-scroll`.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — on desktop-native the list is an engine overlay reaching the screen through `paint_scene` alone. This is the origin of `decision/select-list-engine-overlay`.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — a row of an open list carries its own `bounds` in the snapshot, and a `screenshot` of a row crops to it. This is the origin of `contract/snapshot/select-list-row-bounds` and `contract/screenshot/select-list-row`. These bounds come from layout, so a Taffy move can shift them.

@history_item n=15 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the user-agent `select` is an `inline-block` box showing only its chosen option (`option { display: none; }` with `option:checked` inline), from engine defaults. This is the origin of `decision/single-select-box-shows-chosen-label`, ratified by the founder (F2). The box's layout comes from Taffy's inline-block and inline handling, which `1a11761bc`'s `inline.rs` edit touches.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups) — `multiple` select rows and optgroups lay out `display: block`, and a single select's group is `display: contents`. These are engine defaults that Taffy's block and contents handling lays out, next to the single-select box row.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (Motion item) — right after a scroll, the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver click there needs an `advance` past the fade first. This is the origin of `decision/shown-scrollbar-takes-thumb-drag`. The band's position comes from the scroll container's laid-out size.

@history_item n=18 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation` in the depth rules) — added `mix-blend-mode` other than `normal` to the stacking-root list and made the select list paint past the blend layers. This is the current wording behind `decision/stacking-context-groups-paint-and-hit` and `decision/select-list-painted-after-root`. It came from an upstream take ratified by the founder's F2 A.

@history_item n=19 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip) — an inline box holding only block children, flagged in `collect_layout_children_with_wrap`, paints no box of its own; no frame test reads the skip. This is layout-tree construction that feeds Taffy, next to the inline layout path the Taffy chain's `inline.rs` edit touches. It is a trap: without a frame test, a regression would go unseen.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the citations re-pointed past the take) — an upstream take that edited `document.rs`, `node/node.rs` and other engine files moved the citations of every area, with no claim changed. This is a precedent: the Taffy chain's edits to engine files may need the same re-pointing of any guard or citation they shift.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through a clipping box's transforms) — `BaseDocument::visible_region` reads each clipping box's padding box through its transforms, `None` for a non-finite edge. These are depth-area readings built on laid-out boxes, measured in both layout modes. A Taffy move must keep them finite and unchanged.
