
## 2026-10-06-cold-agent-run-pipe — the cold-agent pipe registered; its crossings PROVISIONAL
**Section:** §Standard Contracts → CI contracts · §Occupied Resources → Filesystem · Process-wide state · Outbound hosts · Names · §Stack and Technologies → CI/CD · §Infrastructure Patterns → Directory structure
**Change:**
- CI contracts: adds the cold-agent run pipe beside agent-run — `bash scripts/cold-agent.sh {run <task> | status | cleanup | logs}`, task `counter`, exit `0` · `1` failed verdict · `2` usage before any precondition · `3` precondition; python3-`json` events `run.start` · `run.end` · `status` · `cleanup`; `run` starts one isolated `claude -p` session from a per-run `mktemp -d` dir with only the stub's MCP tools and writes `verdict.json` (`passed` only on positive evidence); the stdlib stdio MCP stub (`list` · `read` · `press`; `not-found` · `disabled` · `not-pressable` · `malformed`); pinned by `test_cold_agent.py` in the `ci-scripts` leg.
- Filesystem: `target/cold-agent/` (verdict, events, raw transcript, client log, call log, stub state), the per-run session dir, the live run's committed `evidence/live-*`.
- Process-wide state: one spawned `claude` client per run and its stdio stub. Outbound hosts: the model provider via the operator's own Claude Code login, live run only, never CI.
- Names: the three scripts and the MCP server name `stub`. Stack CI/CD: the Claude Code CLI as the host-only agent client (read at 2.1.288 on the dev host). Directory structure: a `scripts/` line.
- Network ports and listeners stays none.
**Why:** the cold-agent run pipe chunk built the cold-agent gate's reachability pipe. The client, its stdio stub, the outbound path and the login are a Boundary widening: answered at P4 and ratified at this wrap by the overseer under the founder's standing delegation, kept PROVISIONAL in the body until the founder's own word.
**Kept:** "the one fork-CI artifact outside `target/ci-logs/`" stays true — `target/cold-agent/` is uploaded by no CI job.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
