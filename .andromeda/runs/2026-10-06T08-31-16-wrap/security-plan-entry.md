
## 2026-10-06-upstream-sync-element-identity — unknown alignment flags; merge citation re-point
**Section:** §Error Handling · every section citing a merged upstream file's lines
**Change:**
- Error Handling: was "unknown alignment flags are mapped to none rather than panicking"; now an unknown alignment flag is mapped rather than panicking — to Taffy's `AlignContent::NORMAL` for content alignment, to none for item alignment.
- 40 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no other claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; upstream's switch to Taffy's first-class `normal` keyword changed content alignment's fallback. The no-panic guarantee holds.
**Kept:** the three script-reachable accessors the merge brought (`document.children`, `textContent`, `CSSStyleSheet.disabled`) are realizations on the already registered script-to-DOM binding crossing, validated per §Input Validation's JS API rows — not a boundary widening.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/
