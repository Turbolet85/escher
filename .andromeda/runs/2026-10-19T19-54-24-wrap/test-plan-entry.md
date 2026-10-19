
## 2026-10-19-upstream-sync-visual-lint-and-real-app-readiness — upstream's two focus targets, the directory, the tallies
**Section:** §1 Test Scope Summary (the blitz-tests bullet) · §2 Test Strategy (Directory pattern) · §3 Test Harness Contract (Proof — the agent-run re-counts) · §9 CI Integration (Per-OS test tallies; the local workspace baseline)
**Change:**
- §1: upstream's `focus_within` 7 and `focus_redraw` 4 are each their own auto-discovered target, run by the workspace `test` leg and `run all`, not by the `a11y` leg's six or `run stand`. The behaviour they pin is PROVISIONAL.
- §2: blitz-tests was 136 `.rs` files / 135 targets, now 138 / 137, with `all.rs` gaining their two `mod` lines and staying unbuilt.
- §3 Proof: `run stand` 178 · 0 · 7 over 44 files, unmoved by this chunk; `run all` 586 · 0 · 12, +11 over 575. The series was last written at 546 (`run stand` 168 over 40), and the chunks between were not re-counted there.
- §9 per-OS: re-read on CI#39312031677 (`79603d1ef`, green 17/17): linux 199 · 1045 · 0 · 12; macos and coverage 180 · 1042; windows 180 · 1000 · 0 · 9; a11y 6 · 43; stand repeat 156 · 624 · 0 · 21. Against CI#39302945352 that is +2 · +11 on the four test jobs (exactly the two targets). The two 2026-10-19 chunks' moves since CI#39111579925 are attributed per target: session-recording +2 · +26 (windows +2 · +7, stand repeat +6 · +24); recorded-check-replay +1 · +7 (windows +1 · +8, stand repeat +3 · +12).
- §9 local baseline: was 194 · 1001, now 199 · 1045 (197 · 1034 before the take).
**Why:** this chunk's own counts (the playbook's «Accurate this-chunk addition»). The report's claim that no master site states the `run all` figure was disproved at Validate: the §3 Proof series stands, and its sweep pattern could not meet it. The two earlier chunks' tallies had not been written, so they are attributed here from the job logs.
**Ref:** .andromeda/runs/2026-10-19T19-54-24-wrap/
