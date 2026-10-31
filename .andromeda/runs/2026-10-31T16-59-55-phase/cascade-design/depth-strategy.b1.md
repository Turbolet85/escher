### depth-strategy#1.1 · values · depth · ## Depth Strategy · lines 8–8
- Browser z-index: tooltip, menu and suggestions 100; FPS overlay 50; status bar 10 (apps/browser/assets/browser.css:69; apps/browser/assets/browser.css:218; apps/browser/assets/browser.css:274; apps/browser/assets/browser.css:304; apps/browser/assets/browser.css:324)
### depth-strategy#1.2 · values · depth · ## Depth Strategy · lines 9–9
- Browser menu and suggestions share `box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15)` (apps/browser/assets/browser.css:217; apps/browser/assets/browser.css:323)
### depth-strategy#1.3 · values · depth · ## Depth Strategy · lines 10–10
- The seven_guis stand's card and circle-dialog shadows are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values
### depth-strategy#1.4 · values · depth · ## Depth Strategy · lines 11–11
- todomvc uses layered shadows on the app and footer (examples/todomvc/src/todomvc.css:48; examples/todomvc/src/todomvc.css:279)
### depth-strategy#1.5 · values · depth · ## Depth Strategy · lines 12–12
- wgpu_texture layers overlay z-index 10, underlay -10, header 100 (examples/wgpu_texture/src/styles.css:26; examples/wgpu_texture/src/styles.css:34; examples/wgpu_texture/src/styles.css:45); the custom-widget example stacks header 100, overlay 10, underlay -10 (examples/custom_widget.rs:195-198; examples/custom_widget.rs:201-206; examples/custom_widget.rs:213-217); the transforms example's `.overlay` uses z-index 99 (examples/transforms.rs:277-282)
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
### depth-strategy#4.2 · values · depth · ## Depth Strategy · lines 35–35
- Outset box shadows are clipped when opacity is below 1 or the background is not opaque, and blurred shadows use the averaged border radius per a TODO (packages/blitz-paint/src/render/box_shadow.rs:15-24; packages/blitz-paint/src/render/box_shadow.rs:77-82); an outset shadow takes the element's shape corner for corner, so one with no blur and no spread hides behind the element, with expected pixels from Chromium (tests/blitz-tests/tests/outset_box_shadow_shape.rs:1-6)
### depth-strategy#4.3 · values · depth · ## Depth Strategy · lines 36–36
- Inset box shadows fill the padding box then cut a blurred hole with Compose::DestOut (packages/blitz-paint/src/render/box_shadow.rs:89-156)
### depth-strategy#4.4 · values · depth · ## Depth Strategy · lines 37–37
- Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)
### depth-strategy#5.1 · values · depth · ## Depth Strategy · lines 43–43
- `filter: drop-shadow(...)` and `backdrop-filter: blur(10px)` (examples/assets/filters.html:37; examples/assets/filters.html:80); a GitHub fixture's fixed progress bar at `z-index: 2147483647` (examples/assets/github_profile_reduced2.html:3-10)
### depth-strategy#5.2 · values · depth · ## Depth Strategy · lines 44–44
- google search box active shadow 0 1px 6px rgba(32,33,36,.28) and layered shadows such as 0 1px 3px 1px rgba(66,64,67,.15),0 1px 2px 0 rgba(60,64,67,.3); google bar z-index 986 and graphite header z-index 1000 (examples/assets/google.html:3151; examples/assets/google.html:364-365; examples/assets/google.html:47; examples/assets/graphite.html:961)
### depth-strategy#5.3 · values · depth · ## Depth Strategy · lines 45–45
- hr fixture inset shadow and outset glow (examples/assets/hr.html:137; examples/assets/hr.html:146); servo.css `.box` two-layer shadow (examples/assets/servo.css:3041-3044); Preact TodoMVC `.card` `0 2px 4px rgba(0,0,0,.15)` (examples/preact/index.html:18); box-shadow fixtures cover outset, spread, layered and inset shadows (examples/box_shadow.rs:26-38; examples/assets/shadow.html:31-114)
### depth-strategy#6.1 · absence · depth · ## Depth Strategy · lines 51–51
- z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
### depth-strategy#6.2 · absence · depth · ## Depth Strategy · lines 52–52
- shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed s08 files (no match)
### depth-strategy#6.3 · absence · depth · ## Depth Strategy · lines 53–53
- shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 s10 slice files
### depth-strategy#6.4 · absence · depth · ## Depth Strategy · lines 54–54
- shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
