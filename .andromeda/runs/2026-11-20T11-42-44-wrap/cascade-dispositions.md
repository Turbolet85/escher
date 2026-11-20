# Cascade dispositions — 2026-11-20-readme-s-0-3-0-closed

## Sweep
No masters TX landed: the one operation (`ops-architecture.md`, decision/readme-closed-version-heading) was rejected by check 7 (`not_master` type 5), and `rec.py rows-land --dry-run` read `would land 0 operation(s)` and `patterns: none — the call sets and retires no row`. No `cascade-patterns-*.toml` was written and main raised no pattern of its own, so the step-2 sweep is skipped: no pattern in any file. The lateral binds (test-plan §3 ↔ obs-plan §3 · §6, a11y-plan schema ↔ the log schema) are untouched: no plan changed.

Search for the leaf's claim in the masters: `rec.py find --doc architecture --regex 'default branch|build/escher-0\.[34]\.0'` and the same over test-plan — 0 hits each: no master states the fork's default branch.

## Leaves
- mode: pass · `cascade.py leaves --add .claude/docs/workflow.md` — `epoch · «Epoch 2 — Debts of 0.1.0–0.3.0» · markerless 5 · gated 0 · open` · `pass: no source changed — no group`.
- `not compared .claude/docs/workflow.md · no source changed — no added group`. Its line 8 states the fork's default branch is `build/escher-0.3.0`; it is `build/escher-0.4.0` (operator entry `gh repo view … defaultBranchRef`, inputs#I3). No master states the default branch, so no comparer of this pass can reach it. Carried to the route: a `CARRY:` on the first markerless entry, pinned at this wrap's route-resolve (the P5 card's named alternative, inputs#I2). Findings 0 · applied 0 · by hand 0 · declined 0.
