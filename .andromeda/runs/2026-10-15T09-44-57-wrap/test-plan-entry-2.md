
## 2026-10-15-cold-agent-test — the cold-agent contract tests and the CI-scripts tally (§1, §4)
**Section:** §4 What unit tests cover (Cold-agent pipe · CI workflows and leg script, the CI-scripts tally) · §1 Coverage scope (CI Python scripts)
**Change:**
- Cold-agent pipe: test_cold_agent.py 33 tests (was 27); `ColdAgentStubTest` (6) retired with the stub; `ColdAgentPipeTest` 33 (was 21) under a `claude` shim and a fake `target/debug/escher-session` that writes `tool_call` span lines and records its spawns — no live model, credential or built binary; the case list restated (the check's cases, the missing binary, the spacer call, four planted markers, the replay's 3 spawns per run, 1 for a malformed check); the 33 read red against the parent's pipe.
- The CI-scripts leg runs 100 tests over seven files (was 94), test_cold_agent.py's 33; green at CI#38713461945. The 94 reading is kept as history.
**Why:** the stub's deletion and the pipe's swap; the counts as measured by the chunk's gates.
**Ref:** .andromeda/runs/2026-10-15T09-44-57-wrap/
