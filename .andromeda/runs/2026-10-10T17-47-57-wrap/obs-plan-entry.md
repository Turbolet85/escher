
## 2026-10-10-snapshot-states — the no-subscriber census: twelve driver-action checks, one more engine check, 17 readers
**Section:** §3 Observability Harness Contract → Logging stack (the headless-stand bullet)
**Change:**
- Was "the eleven driver-action checks `stand_act_*`"; now twelve, `stand_act_states` the twelfth.
- The census gains the engine's state check `accessibility_states`: no env read, no print, no shared module.
- Was "the module sixteen of the seventeen checks that hold a session read"; now seventeen of the eighteen.
- Was "15 of which read the shared module `tests/blitz-tests/tests/common/mod.rs`"; now 17 — the count was wrong before this chunk, which added no reader of it.
- The two restatements of what an answer and an `Outcome`'s `Debug` hold gain the three state readings, as closed words and booleans; neither is a log line and nothing logs or fields one.
**Why:** two check files were added, neither installing a subscriber; the 15 was disproved by a count at the chunk's base and now. No log site was added, the command span keeps its eight fields, and the host-log figures did not move.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/
