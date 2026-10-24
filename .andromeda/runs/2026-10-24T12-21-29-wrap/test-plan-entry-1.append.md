
## 2026-10-24-upstream-sync-polish-and-ship — `hit_test_dropped_nodes`, the counts it moves, and the gaps the take leaves
**Section:** §1 Test Scope Summary (body: blitz-dom covered behaviours; test-scope-summary.md: apps, blitz-dom covered behaviours, blitz-dom (node), blitz-tests) · §2 Test Strategy (test-strategy.md: Directory pattern) · §3 Agent-run contract (body: Proof) · §9 CI Integration (ci-integration.md: Per-OS test tallies, Local baseline)
**Change:**
- Covered behaviours gain hit testing past a node dropped since the last resolve, by blitz-tests' `hit_test_dropped_nodes` 1 — upstream's check, its own auto-discovered target, run by the workspace `test` leg and `run all`, not by `run stand` or the `a11y` leg, read red with its engine hunks reversed and green with them.
- Stated gaps: the process-wide `PARALLEL_TRAVERSAL_LOCK` is asserted by no check (no headless document sets `Parallel`; the windowed smoke its one local reading); `Node::out_of_flow`, `record_out_of_flow`, `style_is_out_of_flow`, `ElementData::classes` and `parse_classes` have no unit test of their own, read through the pipeline and stand checks; the root example `multi_doc` holds no test.
- Directory pattern: 145 `.rs` files and 144 test targets (was 143 and 142, one short of the 144 · 143 the tree held since `stand_snapshot_parts.rs`).
- Agent-run Proof: `run stand` unmoved at 195 · 0 · 7 over 48 files; `run all` 617 · 0 · 12.
- Per-OS tallies on CI#40092092468: linux 206 · 1099 · 0 · 12, macos and coverage 187 · 1096 · 0 · 12, windows 187 · 1052 · 0 · 9, the a11y job and the stand repeat unmoved; local workspace 206 result lines, 1099 · 0 · 12.
**Why:** upstream's check arrives with the dropped-node hit skip, an engine change the founder's own word «Десять коммитов (Рекомендую)» ratified (relayed verbatim by the overseer, 2026-10-24, chunk inputs#I4). Each count is measured and moves by that one check alone; the 143 · 142 the strategy stated was a stale count.
**Ref:** .andromeda/runs/2026-10-24T12-21-29-wrap/
