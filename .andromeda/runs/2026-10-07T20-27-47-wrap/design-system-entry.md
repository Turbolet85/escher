
## 2026-10-07-refusal-detection — an into-view scroll never animates a nested box
**Section:** §Motion
**Change:** a new bullet: `BaseDocument::scroll_into_view` writes every scrolling box that holds its target at once, innermost first, whatever behaviour was asked, and the requested behaviour — `Smooth` included — applies to the viewport alone; the driver's `scroll` is instant in every box and in the viewport, inside one settled step. The touch-fling citation is re-pointed (`scrolling.rs:724-747` → `:875-898`).
**Why:** the engine method was widened for every document, ratified by the founder (2026-10-07), his own choice relayed verbatim by the overseer and confirmed by the operator at this wrap's escalation. The document holds one scroll animation at a time, so smooth travel of a nested box was not built.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/
