# Counts — 2026-10-20-load-wait (/implement P2, run `.andromeda/runs/2026-10-20T02-02-07-implement/`)

Read from the gate trail `.andromeda/runs/2026-10-20T02-02-07-implement/gate-2026-10-20-load-wait.json` (first
firing of the whole block, 16 green · 0 red · 4 not run — leg operator).

| reading | predicted (plan) | measured | entry |
|---|---|---|---|
| escher-driver unit tests | 94 → 99 (+5) | `test result: ok. 99 passed; 0 failed; 0 ignored` | 1 |
| `stand_load_wait` | 2 passed | `test result: ok. 2 passed; 0 failed; 0 ignored … finished in 10.10s` | 2 |
| `run stand` | 185 · 0 · 7 over 46 files | `"selection": "stand", "passed": 185, "failed": 0, "ignored": 7` · `ls tests/blitz-tests/tests/stand_*.rs` 46 | 12 |
| a11y leg per file | 6 · 6 · 4 · 8 · 5 · 14, 43 | 6 · 6 · 4 · 8 · 5 · 14, 43 | 13 |
| workspace tally (`target/ci-logs/test.log`, fresh from entry 14) | lines 202, passed 1062, failed 0, ignored 12 | `lines 202 passed 1062 failed 0 ignored 12` | 15 |
| blitz-tests `.rs` files | 140 → 141 | 141 (`ls tests/blitz-tests/tests/*.rs`) | — |

The probes (entries 3–10) read as their `expect` states: the wall read in `session.rs` beside `client.rs`, `host.rs`
and `mcp.rs`; 0 wall reads in blitz-dom outside `clock.rs`; no harness, blitz-dom, manifest, lock or shared-fixtures
change; 2 `info_span!` sites; 0 event sites; a silent check; no changed or untracked code file outside the plan's six.
