
## 2026-10-06-stand-test-contract — the agent-run 5-command contract measured
**Section:** §3 Test Harness Contract · §4 Unit Test Strategy
**Change:**
- §3: a new "Agent-run contract" block — `bash scripts/agent-run.sh {boot | run {selection} | status | cleanup | logs}` from the repository root, `scripts/agent-run.ps1` a pass-through Windows entry with no logic (not run on the dev host); exit grammar `0` success · `1` failed (a build failure, a failing run, an empty run) · `2` usage, checked before · `3` precondition unmet; selections `stand` · `all` · `{name}` (`^[a-z0-9_]+$` plus an existing file); `run.start.files` is `[]` under `all`; `run stand` with no `stand_*.rs` is an empty run (no cargo call, `cargo_exit` null, exit 1); JSON-line events `boot` · `run.start` · `test` · `run.end` (printed and appended to `events.jsonl`) and `status` · `cleanup` (printed only), never captured test output; state in `target/agent-run/{status.json, events.jsonl, run.log}` only; no daemon, PID file, status endpoint, port or env var.
- §3 marker: was "no product boot, status, cleanup or logs command, status endpoint shape, PID file or test-data bootstrap mechanism … was gathered"; now the test-data bootstrap mechanism alone.
- §4: `test_agent_run.py` (`AgentRunTest`, 14 cases under a `cargo` shim) joins "What unit tests cover"; the CI-scripts leg was 23 tests, now 37.
**Why:** the stand test contract chunk wrote the scripts and their contract tests; the three points the plan left open (`files` under `all`, the stand-less empty run, usage before not-booted) stand as built on the operator's ruling in the implementing session (2026-10-06).
**Kept:** the event schema is harness metadata bound to obs-plan §3, not escher's telemetry line format.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
