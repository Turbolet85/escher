# Fork CI on the pushed take — 2026-10-18-upstream-sync-determinism-and-traces

## The run
- The pre-CI commit `275950dc0985f258c4fd26fc98faa80fb29df31d` was pushed to `origin/build/escher-0.2.0` (entry 28).
- Entry 29 (`ci.py conclusion --sha HEAD --wait 1800`):
  ```
  275950dc0985 verdict: green · checks 17/17 · wall 842 s · runs CI#39104364530 completed/success
  ```
- **CI#39104364530**, push, completed/success, every job settled. Checks:
  - Accessibility (a11y) tests · Build [default features] · Build counter example · Build wasm examples · Clippy
  - Coverage report · Dependency audit · Documentation · MSRV Build [Rust 1.91] · Rustfmt · Stand checks repeated
  - Test (android) · Test (ios) · Test (macos) · Test (windows) · Test CI scripts · Test [default features]
- The coverage job green carries the `coverage floor: 3 of 3` reading, with `keyboard_compat.rs` still excluded
  (CARRY 2). The MSRV job green is the 1.91 reading.

## Per-OS tallies (entry 30, `--id 39104364530`, recorded), against the chunk start's CI#39100630123
Columns are lines · passed · failed · ignored, from the job logs' libtest result lines with the colour codes stripped.

| job | CI#39100630123 (`fb842671`) | CI#39104364530 (`275950dc`) | moved |
|---|---|---|---|
| Test [default features] | 192 · 990 · 0 · 12 | 192 · 991 · 0 · 12 | +1 passed |
| Test (macos) | 173 · 987 · 0 · 12 | 173 · 988 · 0 · 12 | +1 passed |
| Coverage report | 173 · 987 · 0 · 12 | 173 · 988 · 0 · 12 | +1 passed |
| Test (windows) | 173 · 963 · 0 · 9 | 173 · 964 · 0 · 9 | +1 passed |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — |
| Stand checks repeated | 144 · 582 · 0 · 21 | 144 · 582 · 0 · 21 | — |

## Every moved count, by name
- The four moved jobs each moved +1 passed, which is upstream's
  `document::hover_state_tests::hovering_below_content_reports_default_cursor` in blitz-dom's lib tests (the take,
  `fb8d6c15e`). It was read in each job's log as one `test … ... ok` line:
  - Test [default features] — 06:34:42Z.
  - Coverage report — 06:42:35Z.
  - Test (macos) — 06:40:40Z.
  - Test (windows) — 06:43:41Z.
- Line counts, failed and ignored are unmoved in every job.
- The a11y and stand-repeat jobs run named blitz-tests targets only, with no blitz-dom unit test. Both are unmoved,
  which matches the local `run stand` and a11y readings (`evidence/counts.md`).
- The +1 on the default-features job equals the local workspace tally's +1 (192 · 991 · 0 · 12).
