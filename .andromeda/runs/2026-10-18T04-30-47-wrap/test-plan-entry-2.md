## 2026-10-18-engine-debts — §2–§10 counts and the harness readings
**Section:** §2 Test Strategy (Directory pattern) · §3 Test Harness Contract (Inspection helpers · Crate-local test helpers · → `session-lifecycle`, its `session-proof` label) · §4 Unit Test Strategy · §5 Integration Test Strategy (Session ↔ held instance) · §9 CI Integration (Legs) · §10 Quality Gates (Flakiness budget)
**Change:**
- blitz-tests: was 127 `.rs` files and 126 targets; now 132 and 131, the five new each its own target with no manifest line.
- `session_common` readers: was 21; now 22. `stand_act_*`: was thirteen; now fourteen, in §2, §3, §4, §5 and the key file, whose total reads 50 (was 48), `stand_act_transformed` 2.
- `stand_*.rs` files run by `stand-repeat`: was 40; now 41.
- Inspection helpers: `layout_rect`, `layout_rect_of` and `center_of` read a scrolled box where it stands (was «less its own scroll offset … (−30, −70) … C14»).
- Session ↔ held instance: `covered`'s two measured readings are the no-area and the one-axis ones (was `contain: paint` and the one-axis); `stand_act_transformed` added.
**Why:** the chunk's new files and F4's fix (PROVISIONAL, delegate operator); no harness verb, status shape or log format moved. The §9 per-OS tallies were not read by the report and stand as dated.
**Ref:** .andromeda/runs/2026-10-18T04-30-47-wrap/
