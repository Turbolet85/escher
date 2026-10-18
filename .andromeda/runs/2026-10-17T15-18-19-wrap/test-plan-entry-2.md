
## 2026-10-17-upstream-sync-engine-debts — the tallies after the merge: workspace, per-OS, agent-run (§3, §9)
**Section:** §3 Test Harness Contract (Agent-run contract → Proof) · §9 CI Integration (Per-OS test tallies · the workspace tally history)
**Change:**
- Workspace tally: was 186 · 958 · 0 · 12; now 187 result lines, 971 passed · 0 failed · 12 ignored — +13 passed in the two merged files' new result lines, the bump crate's empty `unittests src/main.rs` line gone.
- Per-OS tallies: was the 2026-10-14 reading (linux 185 · 951 · 0 · 12); now re-read on the final HEAD's green 17/17 run — linux 187 · 971 · 0 · 12, macos and coverage 168 · 968 · 0 · 12, windows 168 · 944 · 0 · 9, the a11y job 6 · 43 · 0 · 0, the stand repeat 141 · 576 · 0 · 21, ios and android build only, each equal to the merge's first green run.
- Agent-run: `run stand` unmoved at 168 · 0 · 7 over 40 files; `run all` 546 · 0 · 12, +13 over 533, with no script change.
**Why:** the merge's two test files and bump's departure, every moved count explained by name.
**Ref:** .andromeda/runs/2026-10-17T15-18-19-wrap/
