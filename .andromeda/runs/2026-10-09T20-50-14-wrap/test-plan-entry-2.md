
## 2026-10-09-audit-corrections-agent-surfaces — a textarea's value read on the fixture; the stand and workspace re-counts
**Section:** §1 Test Scope Summary → tests/blitz-tests · §3 Test Harness Contract → Agent-run contract → Proof · §9 CI Integration → Local baseline
**Change:**
- §1 tests/blitz-tests: the in-file fixture's readings were "a checkbox and a radio reading checked after a click and a typed password reading the fixed mask"; they gain a textarea named by `aria-label` reading empty before typing and its typed text back as its value, exactly it reading focused after the click — `a_textarea_reads_back_typed_text`, the eighth test of `stand_snapshot_state`, read red with the `textarea` arm of the snapshot's `value` deleted and green on the restored tree. It is test-plan's first statement of a textarea reading.
- §3 Agent-run contract → Proof: re-counted — `run stand`'s `run.end` reads passed 110 · failed 0 · ignored 5 over the same 30 files (was 109; +1 `stand_snapshot_state`, no script change).
- §9 Local baseline: re-counted — `cargo test --workspace --locked` 154 result lines, 657 passed · 0 failed · 10 ignored (was 656; +1 in the file's existing result line), timings not re-measured.
**Why:** the Epoch 4 code audit left one mutant alive — no test read a textarea's value from a snapshot. The check sits in `stand_snapshot_state` because the audit runs the snapshot unit against a fixed test set that holds that file; a new stand file would sit outside the set the next audit runs.
**Kept:** the earlier re-count clauses, "+7 `stand_snapshot_state`" included, stand as their chunks' readings. The stand's tables in `tests/common/mod.rs` do not gain the textarea — they hold stand controls, and no stand task holds one.
**Ref:** .andromeda/runs/2026-10-09T20-50-14-wrap/
