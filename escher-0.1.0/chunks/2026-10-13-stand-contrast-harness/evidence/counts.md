# Counts — 2026-10-13-stand-contrast-harness

Read from `/andromeda-implement`'s gate run (`.andromeda/runs/2026-10-13T17-25-26-implement/`, first firing, every
runnable entry green: 14 green · 0 red · 3 not run, the operator legs).

## `bash scripts/agent-run.sh run stand` (gate entry 11)
- `run.start`: 37 files (standing 36; `stand_contrast` the new one).
- `run.end`: passed 141 · failed 0 · ignored 7 · cargo_exit 0 · outcome passed (`ts` 2026-10-13T17:34:01Z).
- Against the standing 139 · 0 · 7 over 36 files: +2 passed, the two `stand_contrast` tests, both `ok`. As predicted.

## Workspace test leg (gate entry 13, `bash .github/scripts/ci-leg.sh fast`)
Read from `target/ci-logs/test.log`, written 2026-10-13T17:35:53Z, after the gate run began (17:31Z): this run's log.
- 180 result lines · 900 passed · 0 failed · 12 ignored.
- Against the standing 179 · 893 · 0 · 12: +1 result line (the new `stand_contrast` target) and +7 passed. The seven are
  the five `contrast` unit tests in blitz-test-harness and the two `stand_contrast` tests. Exactly the new tests, as
  predicted.

## `mod common;` readers
`grep -l 'mod common;' tests/blitz-tests/tests/*.rs | wc -l` reads 18 (17 at P3, predicted 18).

## `Cargo.lock` delta (step 1)
One line: `"color",` added to blitz-test-harness's `dependencies` list. No `[[package]]` enters. Gate entry 5 read
count 0 at exit 1.

## Audit (gate entry 12)
`advisories ok`, with `deny.toml` unchanged.
