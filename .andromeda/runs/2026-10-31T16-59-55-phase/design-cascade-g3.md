# design · cascade read · group 3 of 4

## Carried
- (.andromeda/design-system-detail/depth-strategy.md:19) The list's place in the paint order must be set against the paint children and stacking contexts built after layout and transforms, so the plan names where the overlay enters that build and what it reads from it.
  > Paint children and stacking contexts are built after layout and transforms; `StackingContext` and `HoistedPaintChild` are public re-exports (packages/blitz-dom/src/resolve.rs:121-125; packages/blitz-dom/src/lib.rs:89)
- (.andromeda/design-system-detail/depth-strategy.md:20) A select under any of these stacking-context roots must still open a list above the page, so the overlay cannot be ordered only within the root that holds its select.
  > A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1332-1375); atomic paint effects are opacity, filter, clip-path and mask-image (packages/blitz-dom/src/node/node.rs:1377-1406)
- (.andromeda/design-system-detail/depth-strategy.md:26) The overlay must paint above every level of this order and answer first in the hit walk, as what shows answers, and a select inside an `overflow` or `contain: paint` ancestor must have its rows answer past that ancestor's padding box or the plan must say why not.
  > Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1106-1169); hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1574-1638), skipping a node dropped since the last resolve as the paint tree's walk does (architecture §Standard Contracts → Scrolling, selection, tree), and since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box (packages/blitz-dom/src/node/node.rs:1540-1573; as measured at escher-0.1.0/chunks/2026-10-10-scrolling-box-bounds-and-hit/report.md)
  (.andromeda/design-system.md:85)
  > Hit-testing stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, and the node itself is still answered inside its border box.
- (.andromeda/design-system-detail/depth-strategy.md:27) The overlay's hit walk must keep this answer or state its own for a select under a non-invertible transform, whose subtree answers no hit, so no row answers a point that its select's subtree refuses without the plan saying so.
  > A transform whose determinant is 0 or not finite answers no hit for the whole subtree (packages/blitz-dom/src/node/node.rs:1472-1477; tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-56; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
- (.andromeda/design-system-detail/depth-strategy.md:28) An open option's real box must be read against each clipping box through its transforms, including the select's clipping ancestors and the long list's new scrolling box, so the driver's act on an option does not read a row the overlay shows as clipped, or the plan says how it does.
  > `BaseDocument::visible_region` reads each clipping box through its transforms: exact under a translate and a scale; under a rotation the bounding box of the rotated padding box, so it holds corners the box does not show; `None` for a mapped box with a non-finite edge; with no transform on the chain the padding box in the client rects' frame (packages/blitz-dom/src/scrolling.rs:790-812; tests/blitz-tests/tests/visible_region_transformed.rs:177-199; tests/blitz-tests/tests/visible_region_transformed.rs:221-244; tests/blitz-tests/tests/visible_region_transformed.rs:246-276; as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)
- (.andromeda/design-system-detail/depth-strategy.md:34) The list must not rest on `z-index: auto` for its place above the page, since it would then share one level with the page's positioned boxes and a later one would paint over it.
  > Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)
- (.andromeda/design-system-detail/depth-strategy.md:37) A list opened from a select under an opacity, filter or backdrop-filter ancestor must not be cut by that ancestor's layer, or the plan must say how it reads there, including whether the ancestor's effect applies to the list.
  > Opacity, filter and backdrop-filter are applied via a layer clipped to the border box expanded by the filter area (packages/blitz-paint/src/render.rs:496-528)
- (.andromeda/design-system-detail/depth-strategy.md:51) Any shadow, elevation or z-index value on the list must be set as a new engine default of this chunk's own and named on its fork, since no token or scale exists to read it from.
  > z-index or shadow handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
  (.andromeda/design-system-detail/depth-strategy.md:52)
  > shadow or elevation tokens · searched: `shadow|elevation` over the 16 listed s08 files (no match)
  (.andromeda/design-system-detail/depth-strategy.md:53)
  > shadow or elevation definitions · searched: `box-shadow|shadow|elevation|z-index` over the 32 s10 slice files
  (.andromeda/design-system-detail/depth-strategy.md:54)
  > shadows, elevation or z-index tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
  (.andromeda/design-system.md:91)
  > NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.
- (.andromeda/design-system-detail/border-radius.md:12) The stand's popup is `select#flight-type`, so a row or pixel check that reads its radius must read it from layout-templates, and the plan must not record it here as a design-system value.
  > | seven_guis stand | — | The stand's card, button, input, task-card, tag and flight-type select radii are held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not as design-system values |
- (.andromeda/design-system-detail/border-radius.md:15) The list's corners must be set against this engine-painted control radius, which must keep its value, and any relation between them must be stated rather than assumed.
  > | Checkbox frame (engine) | 2 × control scale | (packages/blitz-paint/src/render/form_controls.rs:28-33) |
- (.andromeda/design-system-detail/border-radius.md:16) A long list that scrolls must paint its thumb with this radius, and the list's own corners must be set so that thumb reads inside them.
  > | Scrollbar thumb (engine) | half the thumb thickness (fully rounded) | (packages/blitz-paint/src/render.rs:823-833) |
- (.andromeda/design-system-detail/border-radius.md:22) Corners the list takes from its computed style must pass through this resolution and be scaled to device pixels, and a radius the overlay paints by another path must match it at every scale.
  > Per-corner elliptical radii are resolved from computed border-*-radius values and scaled to device pixels (packages/blitz-paint/src/render.rs:1395-1412)
- (.andromeda/design-system-detail/border-radius.md:39) The list's radius must be set as a new engine default of this chunk's own, since no value, token or scale exists to read it from.
  > border-radius values or tokens · searched: `border-radius|border_radius` over the 15 s05 files
  (.andromeda/design-system-detail/border-radius.md:40)
  > border-radius handling · searched: `border-radius|border_radius|box-shadow|box_shadow|z-index|z_index` over the 17 listed s06 files
  (.andromeda/design-system-detail/border-radius.md:41)
  > border radius values · searched: `radius` over the 8 s07 slice files
  (.andromeda/design-system-detail/border-radius.md:42)
  > border radius values · searched: `border.radius|radius` over the 16 listed s08 files (no match)
  (.andromeda/design-system-detail/border-radius.md:43)
  > radius definitions · searched: `border-radius|radius` over the 32 s10 slice files
  (.andromeda/design-system-detail/border-radius.md:44)
  > border-radius values or tokens · searched: `radius|shadow|elevation|z-index` over the 21 listed s11 files (only a touch-point `radius` matched)
  (.andromeda/design-system.md:108)
  > NOT YET MEASURED — a project-wide radius scale: the examples, test and WPT slices (s01, s12, s13) recorded radius tokens as out of slice.

## Bindings
- **Paint order ↔ hit walk**: the overlay's place above the page and its place in the hit walk must be one decision, so a row that shows is the row that answers, at (.andromeda/design-system-detail/depth-strategy.md:26) with the stacking-context roots of (.andromeda/design-system-detail/depth-strategy.md:20).
- **Effect layer and paint clip ↔ list's corners**: where the overlay paints decides what clips it, so its radius from (.andromeda/design-system-detail/border-radius.md:22) must show whole, past the layer of (.andromeda/design-system-detail/depth-strategy.md:37) and the padding-box stop of (.andromeda/design-system-detail/depth-strategy.md:26).
- **Clipping-box reading ↔ the driver's act and the plan's body on nested scrolling**: an open option in a long list must read true through (.andromeda/design-system-detail/depth-strategy.md:28) once the into-view rule at (.andromeda/design-system.md:121) writes the list's new scrolling box.
- **Absent depth and radius values ↔ the colour fork**: the list's shadow, z-index and radius, absent at (.andromeda/design-system-detail/depth-strategy.md:54) and (.andromeda/design-system-detail/border-radius.md:44), are new engine defaults as its colours are, recorded together in the plan's engine values, and the stand's own radius stays at (.andromeda/design-system-detail/border-radius.md:12).
