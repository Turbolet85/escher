# Apply — design-system · part 1 of 2

## Verdicts
- fanout-design-system--cascade-g2.md 2 · applied · .andromeda/design-system.md:85#1
- fanout-design-system--cascade-g2.md 9 · applied · .andromeda/design-system.md:121#1
- fanout-design-system.md 1 · applied · .andromeda/design-system.md:28+#1
- fanout-design-system--cascade-g1.md 1 · applied · .andromeda/design-system-detail/color-palette.md:29+#1
- fanout-design-system--cascade-g1.md 2 · applied · .andromeda/design-system-detail/color-palette.md:29+#2
- fanout-design-system--cascade-g1.md 3 · applied · .andromeda/design-system-detail/color-palette.md:120#1
- fanout-design-system--cascade-g1.md 4 · applied · .andromeda/design-system-detail/typography.md:51#1
- fanout-design-system--cascade-g1.md 5 · applied · .andromeda/design-system-detail/typography.md:51+#1
- fanout-design-system--cascade-g1.md 6 · applied · .andromeda/design-system-detail/typography.md:44+#1
- cascade-compare-design-system.md 12 · applied · .andromeda/design-system-detail/typography.md:17+#1
- fanout-design-system--cascade-g1.md 7 · applied · .andromeda/design-system-detail/spacing.md:21+#1
- fanout-design-system--cascade-g1.md 8 · applied · .andromeda/design-system-detail/spacing.md:29+#1
- fanout-design-system--cascade-g1.md 9 · applied · .andromeda/design-system-detail/spacing.md:29+#2
- fanout-design-system--cascade-g2.md 1 · applied · .andromeda/design-system-detail/depth-strategy.md:26#1
- fanout-design-system--cascade-g2.md 3 · applied · .andromeda/design-system-detail/depth-strategy.md:26+#1
- fanout-design-system--cascade-g2.md 4 · applied · .andromeda/design-system-detail/depth-strategy.md:28#1
- fanout-design-system--cascade-g2.md 5 · applied · .andromeda/design-system-detail/depth-strategy.md:34+#1
- fanout-design-system--cascade-g2.md 6 · applied · .andromeda/design-system-detail/depth-strategy.md:37+#1

## Edits
- line: .andromeda/design-system.md:85
  old: «the node itself is still answered inside its border box.»
  new: «the node itself is still answered inside its border box; a row of an open select list stands outside this, answered before the walk, so a box clipping the select clips no row.»
- line: .andromeda/design-system.md:121
  old: «the driver's `scroll` is instant in every box and in the viewport.»
  new: «the driver's `scroll` is instant in every box and in the viewport; into view on a row of an open select list writes that list's own scroll at once, moving no other box and no viewport.»
- after: .andromeda/design-system.md:28
  anchor: «**Engine-painted defaults:**»
  add: «»
  add: «The open select list's colours are engine constants, the associated constants of `SelectListView` (`HIGHLIGHT_TEXT`, `HIGHLIGHT_BACKGROUND`, `DISABLED_TEXT`, `BORDER`), not design tokens and not in `default.css`, the engine holding no tokens; its highlight is the pair of a `multiple` select's chosen row.»
- after: .andromeda/design-system-detail/color-palette.md:29
  anchor: «| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-199; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md) |»
  add: «| blitz-dom select list highlighted row | `#c8c8c8` background, `#000000` text | An open select list's highlighted row: `SelectListView::HIGHLIGHT_BACKGROUND` and `HIGHLIGHT_TEXT`, fixed constants, hardcoded as the engine holds no tokens, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123; tests/blitz-tests/tests/select_popup_painted.rs:119-159) |»
  add: «| blitz-dom select list disabled row | `#6d6d6d` text | An open select list's disabled row: `SelectListView::DISABLED_TEXT`, a fixed constant, hardcoded as the engine holds no tokens, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123; tests/blitz-tests/tests/select_popup_painted.rs:119-159) |»
  add: «| blitz-dom select list border | `#999999` | An open select list's border: `SelectListView::BORDER`, a fixed constant, hardcoded as the engine holds no tokens, not in the default sheet (packages/blitz-dom/src/node/select_list.rs:110-123) |»
- after: .andromeda/design-system-detail/color-palette.md:29
  anchor: «| blitz-dom default chosen row | `#c8c8c8` background, `#000000` text | A `multiple` select's chosen row: Chromium's inactive selected pair, hardcoded as the engine holds no tokens, 12.56:1 as measured by `select_interaction` (packages/blitz-dom/assets/default.css:161-199; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md) |»
  add: «| blitz-dom select list plain row | no `SelectListView` constant; value not stated | An open select list's plain pair, measured by `select_popup_painted` at the default sheet and at the stand; a highlighted row's contrast is unread by the lint and a closed select's own pair is unread (tests/blitz-tests/tests/select_popup_painted.rs:161-187; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md) |»
- line: .andromeda/design-system-detail/color-palette.md:120
  old: «over the 21 listed s11 files»
  new: «over the 21 listed s11 files; the open select list's colours are fixed constants on `SelectListView`, hardcoded, and add no token or custom property»
- line: .andromeda/design-system-detail/typography.md:51
  old: «with hinting off when emboldening»
  new: «with hinting off on a build that emboldens; a synthesized group header on a plain build is emboldened with hinting on, and on an emboldening build it takes twice the stated strength»
- after: .andromeda/design-system-detail/typography.md:51
  anchor: «- Font emboldening is enabled by the font-embolden feature, or apple-font-embolden on macOS and iOS (packages/blitz-paint/Cargo.toml:18-19; packages/blitz-paint/src/lib.rs:22-26); strength is 0.015125 and 0.0121 times the CSS font size, each capped at 0.3, with hinting off when emboldening (packages/blitz-paint/src/text.rs:678-695)»
  add: «- A group's `label` in an open select list paints as a header heavier than a row on every build, through `RunPaint { color, synthesize_bold }` in `stroke_text_as`; a synthesized bold adds one embolden weight to a build's own, and every other caller passes `RunPaint::default()`, adding none (packages/blitz-paint/src/text.rs:586-593; packages/blitz-paint/src/text.rs:683-689; tests/blitz-tests/tests/select_popup_painted.rs:282-297)»
- after: .andromeda/design-system-detail/typography.md:44
  anchor: «- Line height maps `normal`, number and length to Parley line heights (packages/blitz-dom/src/stylo_to_parley.rs:407-412); the layout engine resolves `line-height: normal` as 1.2 × font size (packages/blitz-dom/src/layout/mod.rs:232-236)»
  add: «- An open select list's row height is the select's font on one line, a row's text laid out through `select_row_layout`; a larger authored font clips, a standing limit (packages/blitz-dom/src/document.rs:1054-1064; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/typography.md:17
  anchor: «| blitz-dom default code/pre | `-moz-fixed` | not stated | not stated | not stated | (packages/blitz-dom/assets/default.css:88-96; packages/blitz-dom/assets/default.css:350-396; packages/blitz-dom/assets/default.css:407-415; packages/blitz-dom/assets/default.css:711-716) |»
  add: «| blitz-dom default `select[multiple][size]` rows | not stated | not stated | line-height 1.2 | not stated | (packages/blitz-dom/assets/default.css:183-193) |»
- after: .andromeda/design-system-detail/spacing.md:21
  anchor: «- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)»
  add: «- blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)»
- after: .andromeda/design-system-detail/spacing.md:29
  anchor: «- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)»
  add: «- An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/spacing.md:29
  anchor: «- Overlay scrollbar thumb geometry: thickness 10.0, thin thickness 6.0, margin 2.0 and minimum length 32.0 CSS px (packages/blitz-dom/src/node/scrollbar.rs:122-126)»
  add: «- `place` sets an open select list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:399-457)»
- line: .andromeda/design-system-detail/depth-strategy.md:26
  old: «scrolling-box-bounds-and-hit/report.md)»
  new: «scrolling-box-bounds-and-hit/report.md); this walk is the page's: a row of an open select list stands outside it, answered before the walk by `select_list_hit` in `hit_with_scrollbar`, so a box clipping the select clips no row (packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:677-716; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:26
  anchor: «- Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1109-1172); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1574-1638), skipping a node dropped since the last resolve as the paint tree's walk does (architecture §Standard Contracts → Scrolling, selection, tree), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box (packages/blitz-dom/src/node/node.rs:1540-1573; as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md); since 2026-10-14-stand-requirement-sweep its answer at an element's own box is measured, in both layout modes and both builds — an `img` holding raster data answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter the sub-document, pinned as it reads) and a filled text `input` answers the `input` — and a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:431-584; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)»
  add: «- The open select list answers the hit first in `hit_with_scrollbar`, with no scrollbar answered with it, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered` (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/stand_act_popup.rs:228-245; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- line: .andromeda/design-system-detail/depth-strategy.md:28
  old: «tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)»
  new: «tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md); for a row or a group of an open select list it answers that list's scrollport (packages/blitz-dom/src/scrolling.rs:754-756; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:34
  anchor: «- Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)»
  add: «- The open select list paints last in `paint_scene`, after the root's render and before the devtools overlays, so it stands above every box of the page (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»
- after: .andromeda/design-system-detail/depth-strategy.md:37
  anchor: «- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:499-531)»
  add: «- The open select list paints whole past an `opacity` ancestor and past a box that clips its select; a non-finite box is unpainted and only rows in the scrollport are laid out (packages/blitz-paint/src/render/select_list.rs:21-52; tests/blitz-tests/tests/select_popup_painted.rs:211-235; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)»

## Patterns
[[pattern]]
id = "ds1-01"
fixed = 'with hinting off when emboldening'
