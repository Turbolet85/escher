# Start readings — 2026-10-18-upstream-sync-determinism-and-traces

Read by /andromeda-implement, run `.andromeda/runs/2026-10-18T06-16-11-implement/`, 2026-10-18T06:16Z, before the take.

## HEAD and the tree
- `git rev-parse HEAD`: `fb842671137fc4f5e48059f269e796b57cf25017`, as the plan's step 1 names.
- `git status --short`: no tracked source file is modified. The tree holds this chunk's folder (untracked) and the
  pipeline's own bookkeeping from the runs after the engine-debts wrap. Those are three modified ledgers under
  `.andromeda/` (`friction-log.ndjson`, `master-route.md`, the engine-debts wrap's `evolve-…json`), plus
  `.claude/session-handoff.md` and `escher-0.2.0/working-route.md`. Four run dirs and one file are untracked under
  `.andromeda/runs/`: the engine-debts wrap's `health-…json`, `2026-10-18T05-30-22-evolve-diagnose/`,
  `2026-10-18T05-37-25-code-audit/` and `2026-10-18T05-47-23-phase/`. Every one is excluded from the scope read
  (gate-contract §Scope), and none is a source, manifest, check or CI file.

## Upstream
- `git ls-remote upstream refs/heads/main`: `fb8d6c15e9ce062b3b41e71cd48988f0093506cc` — unmoved since P3's read
  (inputs#I2); the take stays `fb8d6c15e`.
- `git merge-base HEAD fb8d6c15e`: `23b9b26a9b6af5947b8f003d2c2cc768bff10b8a` — the merge base the last sync merged.

## Local workspace tally (the engine-debts wrap's `target/ci-logs/test.log`, mtime 2026-10-18T05:26:10Z)
- `test result:` lines summed: **192 lines · 990 passed · 0 failed · 12 ignored**.
- blitz-dom's lib line (`Running unittests src/lib.rs (target/debug/deps/blitz_dom-…)`): `97 passed; 0 failed; 0 ignored`.
- CI scripts (`target/ci-logs/ci-scripts.log`, mtime 2026-10-18T05:26:18Z): `Ran 100 tests`, `OK`.

## Per-OS tallies at the chunk start (CI#39100630123, fork CI on `fb842671`, green)
P4's read through the plan's per-OS entry (the operator entry's known-positive control, fired 2026-10-18T06:06Z,
quoted from `plan.md` `## Test Commands`), as lines · passed · failed · ignored:

| job | lines | passed | failed | ignored |
|---|---|---|---|---|
| Test [default features] | 192 | 990 | 0 | 12 |
| Test (macos) | 173 | 987 | 0 | 12 |
| Coverage report | 173 | 987 | 0 | 12 |
| Test (windows) | 173 | 963 | 0 | 9 |
| Accessibility (a11y) tests | 6 | 43 | 0 | 0 |
| Stand checks repeated | 144 | 582 | 0 | 21 |
