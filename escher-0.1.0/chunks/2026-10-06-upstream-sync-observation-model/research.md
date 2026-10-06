# Codebase Research — 2026-10-06-upstream-sync-observation-model

## Scope
- **Depth:** minimal (a measured no-op sync, zero source delta) · **Reads:** 6 · **Globs/Greps:** 5
- **Harness rules consulted:** none — no live leg in this chunk. The chunk drives no agent-run or cold-agent leg, only the `ci-leg.sh` legs and git reads.
- **Platform issues consulted:** searched nothing, because there was no failure signature to search for. The plan's one non-operator CI read, `ci.py conclusion --sha 42b80ad9…`, reads a run that is settled and green: CI#37495205882, `completed/success`, 16/16 at Setup 5a. Scope carries no runner-only bullet, and both Setup 5a shas read green.
- **External inputs:**
  - `inputs#I1` — upstream/main measured at `23354585` (ls-remote), 0 commits past the merge base
  - `inputs#I2` — the founder's ruling: measured no-op, no merge commit, no work beyond what the gates require

## Files inspected
- `.andromeda/architecture.md` (line 202): the `Upstream sync:` line reads "DioxusLabs/blitz `main` last merged at 2335458530518cdf167c55ce635fae99323e0789 (merge commit f00b021610bbd4ace587372e446809bc6de963dc, 2026-10-06) — the next sync's merge base". This chunk's pin is that sha, so the line stays true unchanged.
- `target/ci-logs/test.log` (15:14Z, the accessibility-tree-identity run): 124 `test result:` lines, 454 passed · 0 failed · 5 ignored. Re-derived: `grep -E '^test result:' target/ci-logs/test.log | awk '{p+=$4; f+=$6; i+=$8; n++} END {…}'`. This matches the test-plan §9 Local baseline the tests extract and history cite.
- `target/ci-logs/ci-scripts.log` (15:14Z): `Ran 64 tests in 3.381s` / `OK`.
- `target/ci-logs/doc.log` (15:14Z): the leg finished and generated docs for 28 crates. Its exit is the verdict, not this log.
- `escher-0.1.0/chunks/2026-10-06-upstream-sync-element-identity/plan.md`: the previous sync's gate shape (merge witness, guards, fast/doc/audit/a11y, operator push and CI read). Every entry that read a merge has no subject here.
- `escher-0.1.0/chunks/2026-10-06-project-readme/plan.md`: the precedent for a zero-Rust-delta chunk. It ran `fast` and `doc` rather than deferring them, and pinned "no other file changes" with a sha-based `git diff --name-only` guard.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- No query run. The chunk changes, adds and calls no symbol, so there is no caller set or impact to cite The graph does not apply: the change surface lies outside every indexed plane. This is not an unavailable or skipped plane.

## Patterns detected
- **Upstream measurement**: `git ls-remote upstream refs/heads/main` (remote truth, writes nothing) plus `git merge-base --is-ancestor 2335458530518cdf167c55ce635fae99323e0789 HEAD` (exit 0 = reachable). Together they prove 0 ahead: the remote tip equals the pin, and the pin is in our history. Measured at P3: `2335458530518cdf167c55ce635fae99323e0789 refs/heads/main`, ancestor exit 0.
- **Code untouched since the last chunk**: `git log -1 --format=%H -- packages examples apps tests wpt Cargo.toml Cargo.lock` = `9285fe75` (the accessibility-tree-identity pre-CI commit). `git diff --name-only 47bbf38f 42b80ad9` lists only `.andromeda/`, `escher-0.1.0/` and `.claude/` paths. So CI#37495205882 (green 16/16 on `42b80ad9`) measured the same code tree this chunk certifies.
- **Fixed-sha CI read**: `ci.py conclusion --sha {full sha}` reads a named commit's runs (plan-template §Tool — ci.py). Setup 5a's read: `42b80ad94e1c verdict: green · checks 16/16 · wall 491 s · runs CI#37495205882 completed/success`.

## Conventions to follow
- **Gate verdicts from the leg script**: `bash .github/scripts/ci-leg.sh {leg}`, logs in `target/ci-logs/{leg}.log` (CLAUDE.md Key commands; architecture §Infrastructure Patterns, CI/CD).
- **Fork CI reads name the fork**: `ci.py` resolves the push remote `origin` = `Turbolet85/escher` (its header line at Setup 5a). It is never a bare `gh run`, which reads upstream (CLAUDE.md Session Learnings 2026-10-05).
- **A git read of this chunk's change names a sha base**: `42b80ad94e1c96014c0325e2a3f4a7135ff1f9ca` is HEAD at take-up (plan-template §Twelve keys, `run`).

## New files to create
- `escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/evidence/`

## Files to modify
- none

## Open questions
- none
