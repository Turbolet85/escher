### depth-strategy#1.6 · values · depth · ## Depth Strategy · lines 13–13
- blitz-dom default dialog backdrop `rgba(0, 0, 0, 0.1)` (packages/blitz-dom/assets/default.css:1027-1030)
### depth-strategy#2.1 · values · depth · ## Depth Strategy · lines 19–19
- Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:121-125; packages/blitz-dom/src/lib.rs:89)
### depth-strategy#2.2 · values · depth · ## Depth Strategy · lines 20–20
- A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1332-1375); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1377-1406)
### depth-strategy#3.1 · values · depth · ## Depth Strategy · lines 26–26
- Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1106-1169); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1574-1638), skipping a node dropped since the last resolve as the paint tree's walk does (architecture §Standard Contracts → Scrolling, selection, tree), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box (packages/blitz-dom/src/node/node.rs:1540-1573; as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md); since 2026-10-14-stand-requirement-sweep its answer at an element's own box is measured, in both layout modes and both builds — an `img` holding raster data answers the `img`, a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter the sub-document, pinned as it reads) and a filled text `input` answers the `input` — and a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content (tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs:431-584; as measured at escher-0.1.0/chunks/2026-10-14-stand-requirement-sweep/report.md)
### depth-strategy#3.2 · values · depth · ## Depth Strategy · lines 27–27
- A transform whose determinant is 0 or not finite answers no hit for the whole subtree (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
### depth-strategy#3.3 · values · depth · ## Depth Strategy · lines 28–28
- `BaseDocument::visible_region` reads each clipping box through its transforms: exact under a translate and a scale; under a rotation the bounding box of the rotated padding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; with no transform on the chain the padding box in the client rects' frame (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/visible_region_transformed.rs:177-199; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
### depth-strategy#4.1 · values · depth · ## Depth Strategy · lines 34–34
- Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)
### depth-strategy#4.4 · values · depth · ## Depth Strategy · lines 37–37
- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)
### depth-strategy#6.1 · absence · depth · ## Depth Strategy · lines 51–51
- z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
### depth-strategy#6.2 · absence · depth · ## Depth Strategy · lines 52–52
- shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed s08 files (no match)
### depth-strategy#6.3 · absence · depth · ## Depth Strategy · lines 53–53
- shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 s10 slice files
### depth-strategy#6.4 · absence · depth · ## Depth Strategy · lines 54–54
- shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
### border-radius#1.3 · values · radius · ## Border Radius · lines 12–12
| seven_guis stand | — | The stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
### border-radius#1.6 · values · radius · ## Border Radius · lines 15–15
| Checkbox frame (engine) | 2 × control scale | (packages/blitz-paint/src/render/form_controls.rs:28-33) |
### border-radius#1.7 · values · radius · ## Border Radius · lines 16–16
| Scrollbar thumb (engine) | half the thumb thickness (fully rounded) | (packages/blitz-paint/src/render.rs:823-833) |
### border-radius#2.1 · values · radius · ## Border Radius · lines 22–22
- Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1395-1412)
### border-radius#4.1 · absence · radius · ## Border Radius · lines 39–39
- border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files
### border-radius#4.2 · absence · radius · ## Border Radius · lines 40–40
- border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
### border-radius#4.3 · absence · radius · ## Border Radius · lines 41–41
- border radius values · searched: `radius` over the 8 s07 slice files
### border-radius#4.4 · absence · radius · ## Border Radius · lines 42–42
- border radius values · searched: `border.radius|radius` over the 16 listed s08 files (no match)
### border-radius#4.5 · absence · radius · ## Border Radius · lines 43–43
- radius definitions · searched: `border-radius|radius` over the 32 s10 slice files
### border-radius#4.6 · absence · radius · ## Border Radius · lines 44–44
- border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
