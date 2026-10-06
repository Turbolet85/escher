
## 2026-10-06-cold-agent-run-pipe — the cold-agent.sh CLI surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/cold-agent.sh` beside agent-run.sh — usage `cold-agent.sh <verb> [task]`, verbs `run · status · cleanup · logs`, the one task `counter`; a usage error prints to stderr with empty stdout and exits 2 before any precondition; stdout is JSON lines only; the raw session transcript stays in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` its Windows pass-through; the exit grammar, schema and stub in test-plan §3.
**Why:** the cold-agent run pipe chunk added an agent-invocable CLI.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
