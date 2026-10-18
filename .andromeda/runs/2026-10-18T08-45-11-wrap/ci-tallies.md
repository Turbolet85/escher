# Per-OS tallies — CI#39111579925, read at this wrap's reconcile (inputs#I4 (b))

- The run: CI#39111579925, push, `completed` / `success`, head `87f1207940e15913f200e11922a5837ae70b5a33` (the pre-CI commit `87f120794`); `gh run view 39111579925 -R Turbolet85/escher --json headSha,conclusion,status,event`, read 2026-10-18T08:54Z.
- The read: the previous chunk's report-only tally entry, its `run` text verbatim (`escher-0.2.0/chunks/2026-10-18-upstream-sync-determinism-and-traces/plan.md:317`), on `39111579925` — `gh run view {id} -R Turbolet85/escher --log`, colour codes stripped, the libtest `test result:` lines summed per job. Exit 0 on all three stages of the pipe. Its known-positive control is the one that entry recorded (CI#39100630123, fired 2026-10-18T06:06Z).
- Columns: lines · passed · failed · ignored.

| job | CI#39104364530 (`275950dc`, the masters' reading) | CI#39111579925 (`87f12079`) | moved |
|---|---|---|---|
| Test [default features] | 192 · 991 · 0 · 12 | 194 · 1001 · 0 · 12 | +2 lines · +10 passed |
| Test (macos) | 173 · 988 · 0 · 12 | 175 · 998 · 0 · 12 | +2 lines · +10 passed |
| Coverage report | 173 · 988 · 0 · 12 | 175 · 998 · 0 · 12 | +2 lines · +10 passed |
| Test (windows) | 173 · 964 · 0 · 9 | 175 · 974 · 0 · 9 | +2 lines · +10 passed |
| Accessibility (a11y) tests | 6 · 43 · 0 · 0 | 6 · 43 · 0 · 0 | — |
| Stand checks repeated | 144 · 582 · 0 · 21 | 147 · 588 · 0 · 21 | +3 lines · +6 passed |

- The +2 lines are the two new blitz-tests targets (`virtual_clock_engine`, `stand_virtual_clock`); the +10 passed are blitz-dom +3 (`clock.rs`), `virtual_clock_engine` +4, `stand_virtual_clock` +2 and seven_guis +1 — the same +10 as the local `fast` tally (report Counts item 12: 192 · 991 → 194 · 1001).
- Stand checks repeated: `stand_virtual_clock` × 3 rounds, +3 lines · +6 passed — the local `stand-repeat` reading (report Counts item 13: 144 · 582 → 147 · 588).
- Failed and ignored are unmoved in every job. No reading lies between CI#39104364530 and this run: CI#39107039862 on `f45bb570` (the operator's own read, inputs#I3) is the previous wrap's commit, the same tree as `275950dc` plus bookkeeping.
