
## 2026-10-10-scrolling-box-bounds-and-hit — the hit walk's edge rule
**Section:** §Depth Strategy → Engine depth behavior
**Change:** the hit-testing sentence, which stated the walk's order only (positive-z hoisted children, paint children in reverse, negative-z hoisted children), now also says where the walk stops: at the padding box of a node that clips by `overflow`, the root element excepted — a point outside that box reaches none of the node's hoisted children, paint children or inline content, as paint clips them, and the node itself is still answered inside its border box.
**Why:** the engine's hit was fixed to stop where paint clips by `overflow`; until then a hit reached content scrolled out of its box. The order inside the box is unchanged.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/
