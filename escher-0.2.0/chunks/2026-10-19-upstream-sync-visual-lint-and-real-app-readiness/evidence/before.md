# The tree before the take

Read at /implement P1 step 1, 2026-10-19T19:17Z, before any cherry-pick.

- `git rev-parse HEAD`: `67f17b9d632ceb66f6c01b347d9f9ba036dd29b6` — the plan's base; HEAD has not moved.
- `git status --short -- packages tests wpt examples apps Cargo.toml Cargo.lock deny.toml .github scripts`: empty — no code path
  carries an uncommitted change. (The tree's other changes are phase's: `.andromeda/friction-log.ndjson`,
  `.andromeda/master-route.md`, `.claude/session-handoff.md`, `escher-0.2.0/working-route.md`, the phase run dir and
  this chunk folder.)
- `upstream/main` reads `17de049b11095808e0d093be2a8f0520e4dcbc0e` "Bump WPT pin to 9d8acee (#1228)" — the measured sha
  (inputs#I1); it has not moved.

## Residue tallies (logs of earlier runs, read as they stand)
- `target/ci-logs/test.log` (mtime 2026-10-19T10:54:58Z), by the tally entry's awk:
  `lines 197 passed 1034 failed 0 ignored 12` — as P4 read it.
- `target/ci-logs/a11y.log` (mtime 2026-10-19T19:10:12Z), by the a11y tally entry's awk: `lines 6 passed 43 failed 0`.
- `target/ci-logs/ci-scripts.log` (mtime 2026-10-19T10:55:08Z): `Ran 100 tests in 9.238s` · `OK`.

## File counts (`ls … | wc -l`)
- `tests/blitz-tests/tests/stand_*.rs`: 44
- `tests/blitz-tests/tests/*.rs`: 136

## The stand's agent-run baselines (P5's untouched-tree readings, plan.md)
- `run stand`: run.end passed 178 failed 0 ignored 7.
- `run all`: run.end passed 575 failed 0 ignored 12.
