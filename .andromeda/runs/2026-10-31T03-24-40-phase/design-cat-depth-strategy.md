# design · Depth Strategy — category read

## Carried
- (.andromeda/design-system-detail/depth-strategy.md:26) Whatever the transformed-scrolling-box fork decides, a target inside a box both transformed and scrolled must be hit where its rows show, so a `visible_region` fix must agree with this hit answer and must not move it.
  > a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content
- (.andromeda/design-system-detail/depth-strategy.md:26) A `visible_region` fix, and the reading of upstream `0980cce5f` that CARRY 1 owes, must keep this ratified stop at the padding box of a paint-clipped node, the root excepted, in both layout modes.
  > since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box
- (.andromeda/design-system-detail/depth-strategy.md:20) The pan/zoom proof fixture's container must be treated as a stacking-context root under a translate and under a scale alike, so its absolutely positioned nodes' paint and hit answers are asserted inside that context.
  > A node is a stacking-context root for opacity not equal to 1, fixed or sticky position, z-index on relative or absolute (or static flex/grid items), any transform/rotate/scale/translate, atomic paint effects, or `isolation: isolate` (packages/blitz-dom/src/node/node.rs:1332-1375)
- (.andromeda/design-system-detail/depth-strategy.md:32) Absolutely positioned nodes in the pan/zoom fixture that hold `z-index: auto` must be expected at one paint level in tree order, so any overlap the fixture asserts follows that order.
  > Positioned descendants with `z-index: auto` share one paint level per CSS 2.1 Appendix E and paint in tree order (tests/blitz-tests/tests/paint_order.rs:1-2)
- (.andromeda/design-system-detail/depth-strategy.md:26) If the founder's word takes a painted select popup, the popup must layer through this stacking order rather than a separate paint path, and the fork must state that cost.
  > Children paint in stacking order: negative z-index hoisted children, regular paint children, then positive z-index hoisted children (packages/blitz-paint/src/render.rs:1106-1169)
- (.andromeda/design-system-detail/depth-strategy.md:26) A `click` on an option, refused `covered` under the stated limit, must be answered through this walk if a popup is taken, with the popup's options reachable in the same order they paint.
  > hit-testing walks positive-z hoisted children, then paint children in reverse, then negative-z hoisted children (packages/blitz-dom/src/node/node.rs:1568-1632)
- (.andromeda/design-system.md:78) A painted popup's elevation has no recorded approach to inherit, so its z-index and any shadow must reach the founder inside the popup's fork rather than be chosen by the plan.
  > **Chosen approach:**
  > > NO RECORDED INTENT
- (.andromeda/design-system.md:91) No project-wide elevation step may be cited for a painted popup or a fixture, since no such scale has been measured.
  > NOT YET MEASURED — a project-wide elevation scale: the examples, engine-resolve, layout and WPT slices (s01, s05, s07, s13) recorded it as out of slice.

## Bindings
- **Hit stop at the padding box ↔ the body §Depth Strategy**: the detail run and the body's broad statement state one rule, so a change to either moves both, `(.andromeda/design-system-detail/depth-strategy.md:26)` and `(.andromeda/design-system.md:85)`.
- **Hit walk ↔ architecture §Standard Contracts → Scrolling, selection, tree**: the walk's skip of a node dropped since the last resolve is held in architecture, and a fix to the transformed box must honour both sides, `(.andromeda/design-system-detail/depth-strategy.md:26)`.
- **Stand shadows ↔ layout-templates §Surface: desktop-native → Primary screens**: a stand or fixture depth value the chunk adds or moves belongs in layout-templates, not in this category, `(.andromeda/design-system-detail/depth-strategy.md:10)`.
