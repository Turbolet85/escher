
## 2026-10-06-stand-test-contract — agent-run.sh on the cli surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/agent-run.sh` — usage `agent-run.sh <verb> [selection]` (verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`), usage on stderr with exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`; `scripts/agent-run.ps1` its Windows pass-through; the exit grammar and event schema stay in test-plan §3.
**Why:** the stand test contract chunk added a repository CLI entry point beside `paint_bench` and `bump`.
**Kept:** the surface's NOT YET MEASURED marker (tooling context, expression level, signature placement) stands.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/
