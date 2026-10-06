
## 2026-10-06-cold-agent-run-pipe — the cold-agent run pipe contract; CI-scripts 37 → 64
**Section:** §3 Test Harness Contract · §4 Unit Test Strategy (What unit tests cover)
**Change:**
- §3 gains a cold-agent run pipe block beside the agent-run contract: invocation and `.ps1` pass-through; exit grammar `0` · `1` failed verdict · `2` usage before any precondition · `3` precondition; `run counter` (per-run `mktemp -d` session, isolation flags, `timeout 600`, stdin `/dev/null`, no propagated 124); the stdio MCP stub; the verdict fields, the positive-evidence `passed`, the `reasons` tokens, `isolated`, `wrong_calls` recorded but never deciding; events; state `target/cold-agent/` only; proof = `test_cold_agent.py` + one live run (passed, 5 calls, 0 wrong, tokens and cost recorded), never in CI.
- §4 gains the `test_cold_agent.py` bullet (`ColdAgentStubTest` 6 · `ColdAgentPipeTest` 21 under a `claude` shim, no live model).
- §4 CI-scripts count: was 37 (23 + 14) at CI run 37409303977; now 64 (23 + 14 + 27) at CI run 37413576977.
**Why:** the cold-agent run pipe chunk added the cold-agent gate's reachability pipe and its contract tests.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
