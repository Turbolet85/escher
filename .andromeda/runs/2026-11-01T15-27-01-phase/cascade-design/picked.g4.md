### spacing#2.4 · values · spacing · ## Spacing · lines 19–19
- The seven_guis stand's card and Home padding is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as a design-system value
### spacing#2.5 · values · spacing · ## Spacing · lines 20–20
- blitz-dom default body margin 8px (packages/blitz-dom/assets/default.css:313-316)
### spacing#2.6 · values · spacing · ## Spacing · lines 21–21
- blitz-dom default `multiple` select rows (`select[multiple] > option`, `select[multiple] > optgroup > option`) padding 0 2px (packages/blitz-dom/assets/default.css:173-177)
### spacing#2.7 · values · spacing · ## Spacing · lines 22–22
- blitz-dom `multiple` select with `size`: `size_rows` hints a `height` of `size × 1.2em`, `size` capped at 65534, `0` or a bad value giving no hint; `select[multiple][size]` takes `overflow-y: auto`, its rows' line height held in §Typography (packages/blitz-dom/src/node/select_list.rs:125-130; packages/blitz-dom/src/stylo.rs:1124-1139; packages/blitz-dom/assets/default.css:183-193)
### spacing#3.1 · values · spacing · ## Spacing · lines 28–28
- `hspace` and `vspace` map to horizontal and vertical margins on `embed`, `img`, `object`, `marquee` and image inputs (packages/blitz-dom/src/stylo.rs:1077-1103)
### spacing#3.2 · values · spacing · ## Spacing · lines 29–29
- Body `marginwidth`, `marginheight`, `leftmargin` and `topmargin` map to pixel margins; `rightmargin` and `bottommargin` are deliberately ignored (packages/blitz-dom/src/stylo.rs:1163-1200)
### spacing#3.4 · values · spacing · ## Spacing · lines 31–31
- An open select list shows at most `MAX_ROWS` = 20 rows and scrolls past them; its box and rows use the engine constants `BORDER_WIDTH`, `ROW_PADDING_X`, `ROW_PADDING_Y` and `GROUP_INDENT` (tests/blitz-tests/tests/select_popup.rs:870-911; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### spacing#3.5 · values · spacing · ## Spacing · lines 32–32
- `place` sets an open select list at least its select's width and at most the viewport's, inside the viewport, opening where its rows fit and else on the side with more room; every edge stays finite (packages/blitz-dom/src/node/select_list.rs:218-263; packages/blitz-dom/src/node/select_list.rs:399-457)
### spacing#3.6 · values · spacing · ## Spacing · lines 33–33
- Single-line text inputs are vertically centered within their content box (packages/blitz-dom/src/node/node.rs:860-878)
### spacing#3.8 · values · spacing · ## Spacing · lines 35–35
- The double text decoration places its second line thickness + 1 CSS px away (packages/blitz-paint/src/text.rs:323-333)
### depth-strategy#1.6 · values · depth · ## Depth Strategy · lines 13–13
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:1038-1041)
### depth-strategy#2.2 · values · depth · ## Depth Strategy · lines 20–20
- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1332-1375); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1377-1406)
### depth-strategy#3.1 · values · depth · ## Depth Strategy · lines 26–26
- Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1109-1172); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1574-1638), skipping a node dropped since the last resolve as the paint tree's walk does (architecture §Standard Contracts → Scrolling, selection, tree), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box (packages/blitz-dom/src/node/node.rs:1540-1573; as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md); this walk is the page's: a row of an open select list stands outside it, answered before the walk by `select_list_hit` in `hit_with_scrollbar`, so a box clipping the select clips no row (packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/select_popup.rs:677-716; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md); since 2026-10-14-stand-requirement-sweep its answer at an element's own box is measured, in both layout modes and both builds — an `img` holding raster data answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter the sub-document, pinned as it reads) and a filled text `input` answers the `input` — and a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:431-584; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)
### depth-strategy#3.2 · values · depth · ## Depth Strategy · lines 27–27
- The open select list answers the hit first in `hit_with_scrollbar`, with no scrollbar answered with it, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it, and a button under the list is refused `covered` (packages/blitz-dom/src/document.rs:962-988; packages/blitz-dom/src/document.rs:2404-2406; tests/blitz-tests/tests/stand_act_popup.rs:228-245; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### depth-strategy#3.3 · values · depth · ## Depth Strategy · lines 28–28
- A transform whose determinant is 0 or not finite answers no hit for the whole subtree (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
### depth-strategy#3.4 · values · depth · ## Depth Strategy · lines 29–29
- `BaseDocument::visible_region` reads each clipping box through its transforms: exact under a translate and a scale; under a rotation the bounding box of the rotated padding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; with no transform on the chain the padding box in the client rects' frame (packages/blitz-dom/src/scrolling.rs:798-820; tests/blitz-tests/tests/visible_region_transformed.rs:177-199; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md); for a row or a group of an open select list it answers that list's scrollport (packages/blitz-dom/src/scrolling.rs:754-756; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### depth-strategy#4.2 · values · depth · ## Depth Strategy · lines 36–36
- The open select list paints last in `paint_scene`, after the root's render and before the devtools overlays, so it stands above every box of the page (packages/blitz-paint/src/render.rs:246-247; packages/blitz-paint/src/render/select_list.rs:21-40; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### depth-strategy#4.6 · values · depth · ## Depth Strategy · lines 40–40
- The open select list paints whole past an `opacity` ancestor and past a box that clips its select; a non-finite box is unpainted and only rows in the scrollport are laid out (packages/blitz-paint/src/render/select_list.rs:21-52; tests/blitz-tests/tests/select_popup_painted.rs:211-235; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md)
### border-radius#1.3 · values · radius · ## Border Radius · lines 12–12
| seven_guis stand | — | The stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
### border-radius#1.5 · values · radius · ## Border Radius · lines 14–14
| blitz-dom default button | 1px | (packages/blitz-dom/assets/default.css:108) |
### border-radius#1.6 · values · radius · ## Border Radius · lines 15–15
| Checkbox frame (engine) | 2 × control scale | (packages/blitz-paint/src/render/form_controls.rs:28-33) |
### border-radius#1.7 · values · radius · ## Border Radius · lines 16–16
| Scrollbar thumb (engine) | half the thumb thickness (fully rounded) | (packages/blitz-paint/src/render.rs:826-836) |
### border-radius#2.1 · values · radius · ## Border Radius · lines 22–22
- Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1398-1415)
### border-radius#2.3 · values · radius · ## Border Radius · lines 24–24
- The `border` attribute on `img`, `object` and image inputs maps to four solid pixel borders (packages/blitz-dom/src/stylo.rs:1140-1161)
