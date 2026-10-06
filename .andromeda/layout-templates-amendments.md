# layout-templates — amendments

One entry per amendment to `layout-templates.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-06-headless-stand — the headless stand's TaskShell mount; seven_guis citations re-pointed
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** the seven_guis entry adds the headless stand: it skips Home and mounts one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell` — `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` — at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans; its `app.rs` citations re-pointed to Home, TaskShell and the CSS constants after the chunk's line shift.
**Why:** the headless stand chunk added a second way into TaskShell; the windowed Home → TaskShell flow is unchanged.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — agent-run.sh on the cli surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/agent-run.sh` — usage `agent-run.sh <verb> [selection]` (verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`), usage on stderr with exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`; `scripts/agent-run.ps1` its Windows pass-through; the exit grammar and event schema stay in test-plan §3.
**Why:** the stand test contract chunk added a repository CLI entry point beside `paint_bench` and `bump`.
**Kept:** the surface's NOT YET MEASURED marker (tooling context, expression level, signature placement) stands.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent.sh CLI surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/cold-agent.sh` beside agent-run.sh — usage `cold-agent.sh <verb> [task]`, verbs `run · status · cleanup · logs`, the one task `counter`; a usage error prints to stderr with empty stdout and exits 2 before any precondition; stdout is JSON lines only; the raw session transcript stays in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` its Windows pass-through; the exit grammar, schema and stub in test-plan §3.
**Why:** the cold-agent run pipe chunk added an agent-invocable CLI.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/
