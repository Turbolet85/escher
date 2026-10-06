
## 2026-10-06-stand-test-contract — the agent-run argv boundary and harness log
**Section:** §Input Validation · §Logging & Monitoring
**Change:**
- §Input Validation: a `CLI arguments (agent-run.sh)` row — a verb outside `boot run status cleanup logs`, a wrong argument count, or a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem prints usage and exits 2 before any cargo call.
- §Logging & Monitoring: the agent-run contract logs harness metadata only — no content-named field, no captured test output or panic text in its events; `target/agent-run/run.log` holds raw cargo/libtest output, unscrubbed like `target/ci-logs/`, never printed, gitignored.
**Why:** the stand test contract chunk added an agent-invoked script whose argument is the one new input it takes and whose events are a new log.
**Kept:** no boundary widened — a selection reaches cargo only as an existing test-file stem; no port, socket or env var.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
