
## 2026-10-06-upstream-sync-element-identity — upstream main 23354585 merged; normal alignment keyword; sync base recorded
**Section:** §Established Decisions → [CSS approximations] · §Project Intent · every section citing a merged upstream file's lines
**Change:**
- CSS approximations: was "container `align-items`/`justify-items: normal` left unset"; now `align-items`/`justify-items`/`align-self`/`justify-self: normal` map to Taffy's `NORMAL` keyword, which Taffy resolves by the aligned box's layout mode — a container's `auto` or unknown flag too.
- CSS approximations: the `safe` positional content-alignment default of a block container now covers a table cell.
- Project Intent: a new `Upstream sync:` line beside `Origin:` — DioxusLabs/blitz `main` last merged at `2335458530518cdf167c55ce635fae99323e0789` (merge commit `f00b0216`), the next sync's merge base; the `Origin:` line unchanged.
- 113 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585` (11 commits, conflict-free, our changes additive); upstream moved the alignment mapping to Taffy's first-class `normal` keyword and extended `align-content` to table cells. The sync base is where the next epoch's Upstream sync reads its merge base.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/
