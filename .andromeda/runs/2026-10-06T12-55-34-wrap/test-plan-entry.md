
## 2026-10-06-accessibility-tree-identity — two new test files; stand and workspace re-counts
**Section:** §1 → tests/blitz-tests · §3 → Agent-run contract → Proof · §9 → Local baseline · citations re-pointed
**Change:**
- §1: the blitz-tests coverage list gains `stand_accessibility_ids.rs` (the stable id carried as AccessKit `author_id` on every element node and no other, the 15 controls' roles and names, the ids after a re-render, the Tab order unchanged) and `accessibility_names.rs` (accessible names from `aria-label` and `<label>` association; a plain document's tree carries no `author_id`).
- §3 Proof: re-counted `run stand` 25 `ok` stand events (+5 `stand_accessibility_ids`, picked up by its `stand_` prefix with no script change); the 20 at id-persistence stays as history.
- §9 Local baseline: re-counted `cargo test --workspace` 454 passed · 0 failed · 5 ignored (+5 `accessibility_names`, +5 `stand_accessibility_ids`, two new result lines); result lines and timings not re-measured.
- 22 citations into the changed files re-pointed by the measured line map.
**Why:** the chunk added the two files, and its `ci-leg.sh fast` runs (at /implement and on its pre-CI commit) and `run stand` measured the counts. The new files ride the workspace test leg, not the a11y leg, which keeps its three files.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/
