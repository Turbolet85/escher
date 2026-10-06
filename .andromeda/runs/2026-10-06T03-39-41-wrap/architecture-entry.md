
## 2026-10-06-stand-test-contract — the agent-run contract and its state area registered
**Section:** §Standard Contracts → CI contracts · §Occupied Resources → Filesystem
**Change:**
- CI contracts: adds the agent-run test contract — `bash scripts/agent-run.sh {boot | run {stand|all|name} | status | cleanup | logs}` driving `cargo test -p blitz-tests --locked`, exit grammar `0` · `1` failed (an empty run included) · `2` usage, checked before · `3` precondition unmet, JSON-line events encoded by an embedded python3 `json`, `scripts/agent-run.ps1` a pass-through; pinned by `test_agent_run.py` in the existing `ci-scripts` leg; full contract in test-plan §3.
- Filesystem: adds `target/agent-run/` — `status.json`, `events.jsonl`, `run.log` (raw output, unscrubbed like `target/ci-logs/`) — recreated by `boot`, removed by `cleanup`, local only.
**Why:** the stand test contract chunk added a project-authored harness script and its state area; no port, socket, env var, crate or dependency.
**Kept:** "the one fork-CI artifact outside `target/ci-logs/`" stays true — `target/agent-run/` is never uploaded.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
