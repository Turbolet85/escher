
## 2026-10-06-id-persistence — id persistence coverage; run stand and workspace re-counts
**Section:** §1 Test Scope Summary → tests/blitz-tests · §3 Test Harness Contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1: stand coverage adds the same id across a re-render, a remount (fresh `NodeId`s bar the four document-skeleton elements) and a fresh process. The test binary re-executes its own `#[ignore]` child `ids_in_a_child_process`. `stand_id_persistence` is the seventh stand file.
- §3 Proof: re-count appended — `run stand` 20 `ok` stand events (+3 `stand_id_persistence`, picked up by its `stand_` prefix).
- §9 Local baseline: re-count appended — 122 result lines, 444 passed · 0 failed · 5 ignored (+3 stand checks, +1 ignored child).
**Why:** v010-02's witness landed as a new stand file. The counts are measured at the chunk's `ci-leg.sh fast` and `run stand` runs.
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/
