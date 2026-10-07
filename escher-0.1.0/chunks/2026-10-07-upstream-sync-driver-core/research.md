# Codebase Research — 2026-10-07-upstream-sync-driver-core

## Scope
- **Depth:** minimal (a measured no-op sync, zero source delta) · **Reads:** 4 · **Globs/Greps:** 8
- **Harness rules consulted:** none — no live leg in this chunk. The chunk drives no agent-run or cold-agent leg, only the `ci-leg.sh` legs and git reads.
- **Platform issues consulted:** searched nothing, because there is no failure signature to search for. The plan's CI reads are fixed-sha reads of one run that is settled and green: CI#37571032838 on `8d156de1`, `completed/success`, 16/16 at P3. Scope carries no runner-only bullet; Setup 5a read that run in progress and P3 re-read it settled.
- **External inputs:**
  - `inputs#I1` — upstream/main measured at `23354585` (ls-remote), 0 commits past the merge base; ours 43 past it

## Files inspected
- `.andromeda/architecture.md` (line 202) — the `Upstream sync:` line reads "DioxusLabs/blitz `main` last merged at 2335458530518cdf167c55ce635fae99323e0789 (merge commit f00b021610bbd4ace587372e446809bc6de963dc, 2026-10-06) — the next sync's merge base" (`grep -n 'Upstream sync:' .andromeda/architecture.md`). This chunk's pin is that sha, so the line stays true unchanged.
- `.github/scripts/ci-leg.sh` (full, 63 lines) — answers the obs extract's question: `fast` is a loop over `FAST_LEGS=(fmt clippy test ci-scripts)` that re-invokes the script once per sub-leg (lines 54-59), so it leaves **one log per sub-leg** (`target/ci-logs/{fmt,clippy,test,ci-scripts}.log`) and no `fast.log`. `set -euo pipefail` (line 6) ends the loop at the first red sub-leg. `test` is `cargo test --workspace --locked` (line 21); `doc` is `cargo doc --workspace --no-deps --locked` (line 32) under `RUSTDOCFLAGS="-D warnings"` (line 61); each log is truncated at its leg's start by `tee` (line 63).
- `target/ci-logs/test.log` (written 2026-10-07T04:20:15Z, the audit-corrections wrap's light gate) — 131 `test result:` lines, 548 passed · 0 failed · 5 ignored. Re-derived: `grep -E '^test result:' target/ci-logs/test.log | awk '{p+=$4; f+=$6; i+=$8; n++} END {print "lines " n " passed " p " failed " f " ignored " i}'` printed `lines 131 passed 548 failed 0 ignored 5`. This matches the latest re-count at `.andromeda/test-plan.md:315` ("131 result lines, 548 passed · 0 failed · 5 ignored").
- `target/ci-logs/test.log` (same file) — answers the a11y extract's question: the workspace `test` leg runs the three files of the `a11y` leg and the stand's a11y checks. `grep -c -E 'Running tests/(accessibility_hidden|accessibility_roles|focusability_updates)\.rs' target/ci-logs/test.log` = 3; the same log shows `accessibility_names.rs`, `stand_accessibility_ids.rs`, `stand_actionable_keys.rs`, `stand_snapshot.rs`, `stand_snapshot_state.rs` and ten further `stand_*.rs` files running. A separate local `ci-leg.sh a11y` run is therefore not owed.
- `target/ci-logs/ci-scripts.log` (2026-10-07T04:20:18Z) — `Ran 64 tests in 3.396s` / `OK` (`grep -n -E '^Ran |^OK|FAILED'`).
- `target/ci-logs/doc.log` (2026-10-07T04:20:18Z) — the leg finished and generated docs for 28 crates. Its exit is the verdict, not this log.
- `deny.toml` (lines 19-20) — one advisory ignore, `RUSTSEC-2026-0192` (`grep -n -E 'RUSTSEC|^ignore' deny.toml`). The no-change guard covers the file; nothing in this chunk edits it.
- `.andromeda/playbook.md` (lines 36-38) — the one `verdict: escalate` pattern is "Boundary widening" (`grep -n -i 'verdict: escalate' .andromeda/playbook.md` = line 37 only).
- `escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/plan.md` and `research.md` — the previous measured no-op sync: its seven-entry gate block (pin witness, no-change guard, `fast`, count, `Ran 64 tests`, `doc`, fixed-sha CI read) and its `evidence/sync.md` record are the shape this chunk repeats with today's sha, counts and run id.
- `escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/inputs/I2-relay-2.md.txt` — the founder's ruling for that entry: "Measured no-op (Recommended)", with the note "record the measured upstream pin and the 0-ahead read, no merge commit, and no work beyond what the gates require". It is a file of this repository, not a new input; this chunk's scope leans on it and the P5 review confirms or rejects the lean.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- No query run. The chunk changes, adds and calls no symbol, so there is no caller set or impact to cite. The graph does not apply: the change surface lies outside every indexed plane. This is not an unavailable or skipped plane.

## Patterns detected
- **Upstream measurement**: `git ls-remote upstream refs/heads/main` (remote truth, writes nothing) plus `git merge-base --is-ancestor 2335458530518cdf167c55ce635fae99323e0789 HEAD` (exit 0 = reachable). Together they prove 0 ahead: the remote tip equals the pin, and the pin is in our history. Measured at P3 (2026-10-07T04:31Z): `2335458530518cdf167c55ce635fae99323e0789 refs/heads/main`, ancestor exit 0.
- **Code untouched since the last chunk's pre-CI commit**: `git log -1 --format=%h -- packages examples apps tests wpt Cargo.toml Cargo.lock` = `4e90f108`; `git diff --stat 4e90f108 8d156de1 -- packages examples apps tests wpt scripts .github Cargo.toml Cargo.lock deny.toml` is empty. So CI#37571032838 (green 16/16 on `8d156de1`) measured the same code tree this chunk certifies, and so did the leg logs now on disk (written 04:19-04:20Z, before the wrap commit at 04:21:00Z).
- **Working tree at P3**: `git status --short` lists changes only under `.andromeda/`, `escher-0.1.0/` and `.claude/session-handoff.md` (five modified files, two untracked directories — this chunk's run dir and chunk folder).
- **Fixed-sha CI read**: `ci.py conclusion --sha {full sha}` reads a named commit's runs (plan-template §Tool — ci.py). P3's read: `8d156de1fcce verdict: green · checks 16/16 · wall 414 s · runs CI#37571032838 completed/success`. The 16 checks, by name (`gh api repos/Turbolet85/escher/commits/8d156de1fcced9157289286909a2cdb3b5be5558/check-runs`, `total_count` 16, all `completed/success`): Rustfmt · Clippy · Test [default features] · Test CI scripts · Documentation · Build [default features] · MSRV Build [Rust 1.91] · Build counter example · Build wasm examples · Dependency audit · Accessibility (a11y) tests · Coverage report · Test (windows) · Test (macos) · Test (ios) · Test (android).

## Conventions to follow
- **Gate verdicts from the leg script**: `bash .github/scripts/ci-leg.sh {leg}`, logs in `target/ci-logs/{leg}.log` (CLAUDE.md Key commands; `.github/scripts/ci-leg.sh:63`).
- **Fork CI reads name the fork**: `ci.py` resolves the push remote `origin` = `Turbolet85/escher` (its header line at Setup 5a and at P3). It is never a bare `gh run`, which reads upstream (CLAUDE.md Session Learnings 2026-10-05).
- **A git read of this chunk's change names a sha base**: `8d156de1fcced9157289286909a2cdb3b5be5558` is HEAD at take-up (plan-template §Twelve keys, `run`).
- **Evidence states verdicts and counts only**: nothing under `target/ci-logs/` is copied into the tree, and the record holds no host path or credential (the security and obs extracts; the previous sync's `evidence/sync.md` is the form).

## New files to create
- `escher-0.1.0/chunks/2026-10-07-upstream-sync-driver-core/evidence/`

## Files to modify
- none

## Open questions
- none
