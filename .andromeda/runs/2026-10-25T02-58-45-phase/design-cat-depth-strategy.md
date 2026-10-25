# design · Depth Strategy — category read

## Carried
- (.andromeda/design-system-detail/depth-strategy.md:26) The statement's account of the hit at a box that clips, the capability behind the driver's `covered` refusal and the stated limit for a box clipped on one axis, must match this rule as written: it names `overflow` and a computed `contain` that includes `paint`, the root element excepted, and the node itself still answered inside its border box.
  > since 2026-10-10-scrolling-box-bounds-and-hit it stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` on a box paint applies it to (ratified by the founder, 2026-10-18) — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box
- (.andromeda/design-system-detail/depth-strategy.md:26) When the limits section is closed at P3 against the masters, it must state that a hit does not enter an `iframe`'s sub-document (a point there answers the `iframe`), since this is a measured limit of the hit, pinned as it reads.
  > a block inside an `iframe`'s `srcdoc` sub-document answers the `iframe` (the parent's walk does not enter the sub-document, pinned as it reads)
- (.andromeda/design-system-detail/depth-strategy.md:26) The capabilities section must state a hit on a transformed and scrolled box the way this rule does, where its rows show, so that it is kept apart from the separate `off-screen` limit for a scrolling box under a transform.
  > and a box both transformed and scrolled is hit where its rows show, not through its scrolled-out content
- (.andromeda/design-system.md:87) The body's short form of the hit rule is the master's broad line that the statement's wording must not contradict.
  > Hit-testing stops at the padding box of a node paint clips — by `overflow`, or by a computed `contain` that includes `paint` — the root element excepted: a point outside that box reaches none of the node's hoisted children, paint children or inline content, and the node itself is still answered inside its border box.

## Bindings
- **The hit at a box that clips ↔ the scope's stated limits (`covered` on a box clipped on one axis) and architecture §Standard Contracts (Scrolling, selection, tree)**: the stated limit follows from the padding-box stop, which applies on both axes even when a box clips only one of them. The statement must word the limit and the rule so they agree with each other (.andromeda/design-system-detail/depth-strategy.md:26) (.andromeda/design-system.md:87)
- **The transformed-and-scrolled hit ↔ the scope's `off-screen` limit under a transformed scrolling box**: hitting where the rows show is a capability, and the `off-screen` reading is a separate visibility limit. The statement must not merge the two (.andromeda/design-system-detail/depth-strategy.md:26)
