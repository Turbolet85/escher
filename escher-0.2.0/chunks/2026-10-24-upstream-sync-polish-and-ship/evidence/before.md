# Before — the base readings (step 1)

- **Base:** `a356f908661f55cf866d3341337a54a79ca7ac25` (build/escher-0.2.0 HEAD; `git rev-parse HEAD` at /implement Setup, 2026-10-24T11:39Z, the tree carrying only the chunk's take-up documents).
- **CI on the base:** CI#40081725681 red by the job Dependency audit alone, 17/17 checks (inputs#I3 item 5).
- **P5's baseline readings of this plan's entries, as the plan records them on the untouched tree (2026-10-24T11:28Z–11:34Z):**
  - audit — `bash .github/scripts/ci-leg.sh audit` exit 1: advisories FAILED on RUSTSEC-2026-0386 (tokio 1.53.2), licenses ok;
  - workspace tally — lines 205 · passed 1098 · failed 0 · ignored 12 (`target/ci-logs/test.log` of the last `fast` run);
  - `bash scripts/agent-run.sh run stand` — run.end passed 195 · failed 0 · ignored 7; `run all` — passed 616 · failed 0 · ignored 12;
  - a11y leg — lines 6 · passed 43 · failed 0 (6 + 6 + 4 + 8 + 5 + 14);
  - `tests/blitz-tests/tests/` — 144 `.rs` files, 48 `stand_*.rs` files;
  - seven_guis per-package — 12 result lines, 36 passed, no FAILED;
  - blitz-dom lib — 106 passed; the integration entry's 19 targets — 106 passed, 0 failed.
- **The lock on the base:** tokio 1.53.2 (`Cargo.lock:8357`), 987 packages.
