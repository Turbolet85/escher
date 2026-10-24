# Cascade compare — test-plan · phase · 2026-10-24-stand-requirement-sweep
E items 193 · held 33 · lost at classify 4 · lost at strong 0 · cascade only 11 · body 102 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#5.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:41
- (.andromeda/test-plan-detail/ci-integration.md:41) The stand-repeat job's tally is the reading the chunk's added or moved witnesses change, re-read from CI at the wrap.
  > the a11y job 6 · 43 · 0 · 0 and the stand repeat 168 · 684 · 0 · 21, unmoved

### ci-integration#6.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:61
- (.andromeda/test-plan-detail/ci-integration.md:61) The workspace count the chunk's `ci-leg.sh fast` run moves from must be re-counted against this reading.
  > re-counted with upstream's `hit_test_dropped_nodes`: 206 result lines, 1099 passed · 0 failed · 12 ignored

### ci-integration#6.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:63
- (.andromeda/test-plan-detail/ci-integration.md:63) The local `coverage` leg reading is stale, so "coverage floors held" needs a fresh local read of the leg.
  > `ci-leg.sh coverage` exit 0, its instrumented test run 404 · 0 · 3 (cargo-llvm-cov runs no doctests), before 2026-10-06-telemetry-bootstrap's +9 tests / +1 ignored, not re-run locally since

### mocking-stubbing-discipline#1.6 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:15
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:15) The load-wait requirement's witness, `stand_load_wait`, stays the one in-file self-answering stand-in with a wall-time sleep, and it must hold green three rounds under `stand-repeat`'s zero-failure budget; a sweep row naming it names this stand-in and its never-answered arm on `HeldNet`, and no second self-answering stand-in is added.
  > `LateNet`, `stand_load_wait`'s in-file self-answering stand-in since 2026-10-20-load-wait: `fetch` answers each request from its own thread after a 200 ms sleep, so a load is in flight when the step settles and is answered inside `LOAD_WAIT`; the never-answered arm uses the shared `HeldNet`, and its threads end with the test process

### unit-test-strategy#6.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:49
- (.andromeda/test-plan-detail/unit-test-strategy.md:49) These are the unit witnesses of 0.2.0's range and select engine changes, two of the four files the scope names for a possible blitz-dom coverage floor. If the sweep record names them, it must say that they run in the `test` leg, not inside `stand-repeat`.
  > since 2026-10-13-range-input-interaction seven `node/range.rs` tests pin the range value rules — the HTML float grammar, attribute defaults and bounds, the seeded value finite, bounded and on the grid, the moves on the grid, degenerate attributes, decimal formatting, and a re-seed and a re-sanitize on attribute writes (packages/blitz-dom/src/node/range.rs:303-529); four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), a step skipping disabled options and stopping at the ends, typed text choosing the first enabled option whose label starts with it and nothing for an empty text, and the strip and collapse of an option's text (packages/blitz-dom/src/node/select.rs:195-301)

## Held

- ci-integration#1.1
- ci-integration#4.1
- ci-integration#7.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- e2e-test-strategy#2.2
- integration-test-strategy#2.1
- integration-test-strategy#2.3
- mocking-stubbing-discipline#1.17
- mocking-stubbing-discipline#1.5
- quality-gates-coverage-targets#2.1
- quality-gates-coverage-targets#3.1
- quality-gates-coverage-targets#4.1
- test-data-fixtures#2.1
- test-data-fixtures#3.1
- test-data-fixtures#3.8
- test-data-fixtures#3.9
- test-scope-summary#11.1
- test-scope-summary#2.1
- test-scope-summary#2.3
- test-scope-summary#3.1
- test-scope-summary#5.1
- test-scope-summary#7.2
- test-scope-summary#9.1
- test-strategy#2.1
- test-strategy#2.2
- test-strategy#3.1
- test-strategy#5.1
- test-strategy#7.1
- unit-test-strategy#2.2
- unit-test-strategy#3.1
- unit-test-strategy#4.1
- unit-test-strategy#8.4

## Cascade only

- ci-integration#2.1
- integration-test-strategy#2.2
- mocking-stubbing-discipline#1.11
- mocking-stubbing-discipline#3.1
- test-scope-summary#1.1
- test-scope-summary#2.2
- test-scope-summary#3.2
- test-scope-summary#4.1
- test-strategy#1.1
- test-strategy#4.1
- unit-test-strategy#1.1
