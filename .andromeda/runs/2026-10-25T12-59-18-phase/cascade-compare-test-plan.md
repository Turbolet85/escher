# Cascade compare — test-plan · phase · 2026-10-25-cold-agent-bar
E items 155 · held 31 · lost at classify 6 · lost at strong 1 · beside a placed fact - · cascade only 4 · body 58 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### ci-integration#5.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:42
- (.andromeda/test-plan-detail/ci-integration.md:42) The chunk's CI read moves the per-OS tallies from this last recorded reading, every job attributed per target.
  > re-read with the `a11y` leg's fourteen files on CI#40134371519, the pre-CI commit `4cbf0a4c9` (green 17/17): linux 206 · 1105 · 0 · 12; macos and coverage 187 · 1102 · 0 · 12; windows 187 · 1057 · 0 · 9; the a11y job 14 · 84 · 0 · 0; the stand repeat 198 · 837 · 0 · 21

### ci-integration#5.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:43
- (.andromeda/test-plan-detail/ci-integration.md:43) A per-job tally the chunk reads from a CI log must strip both spellings of the escape.
  > A per-job tally read from a CI job log through `gh run view --log` strips the ANSI codes with `(\x1b|\^\[)\[[0-9;]*m`: the log spells the escape as the two literal characters `^[`, so a strip of `\x1b\[[0-9;]*m` alone leaves every cargo `Running tests/{name}.rs` line unattributed

### ci-integration#7.1 · lost at classify · .andromeda/test-plan-detail/ci-integration.md:71
- (.andromeda/test-plan-detail/ci-integration.md:71) The coverage leg's per-crate floors must still hold after the chunk (no driver change is intended, so escher-driver's 97 must not drop).
  > `python3 .github/scripts/coverage_floor.py target/coverage/lcov.info packages/escher-driver=97 packages/dioxus-native-dom=83 packages/blitz-test-harness=90`, the leg exiting with the floor script's status

### quality-gates-coverage-targets#2.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:14
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) The chunk's tree must pass the `coverage` leg at the three per-crate floors unchanged; any line the chunk adds to escher-driver (a session-host or boot path serving `cells`, a load in flight) or to dioxus-native-dom must keep its crate at or above its floor.
  > the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %

### quality-gates-coverage-targets#2.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16
- (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:16) The newest per-crate reading is the baseline the chunk's coverage leg is read against; the leg must end with the same verdict line.
  > read again with the stand checks' named list, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2740/3202 = 85.57 %, blitz-test-harness 949/1008 = 94.15 %, `coverage floor: 3 of 3 crates at or above their floors`

### test-strategy#4.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:28
- (.andromeda/test-plan-detail/test-strategy.md:28) `docs/capabilities.md` names its checks by hand. A stand check this chunk renames or moves must be flagged, because the statement's text is out of scope and is the wrap's to re-read.
  > only its names and counts are held to the schema's tables, by the CI-script pin `test_capability_statement.py` (§4), so a check renamed or moved changes that section by hand

### test-strategy#5.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:34
- (.andromeda/test-plan-detail/test-strategy.md:34) A check compared byte for byte across OSes, such as a committed check or a replayed one over the extended stand, must hold no `screenshot` step.
  > a shot's PNG bytes differ between linux x86_64 and macOS aarch64 (CI#39225375577), so a check compared byte for byte across OSes holds no shot

### unit-test-strategy#7.1 · lost at strong · .andromeda/test-plan-detail/unit-test-strategy.md:56
- (.andromeda/test-plan-detail/unit-test-strategy.md:56) The flight booker's new select must use the engine's select model as it stands, with no engine change. The model's table-pinned unit tests must stay green unchanged.
  > four `node/select.rs` tests pin the select's rules over stated tables — HTML's selectedness setting algorithm (the last chosen, else the first option not disabled), a step skipping disabled options and stopping at the ends, typed text choosing the first enabled option whose label starts with it and nothing for an empty text, and the strip and collapse of an option's text (packages/blitz-dom/src/node/select.rs:195-301)

### unit-test-strategy#6.1 · lost at classify · .andromeda/test-plan-detail/unit-test-strategy.md:50
- (.andromeda/test-plan-detail/unit-test-strategy.md:50) No verb or cause is added, so the capability-statement pin must stay green with its counts unmoved. Its "driver commands as tools" phrase is read from the schema.
  > every verb of `VERBS` (`packages/escher-driver/src/schema.rs`, lower-cased) is a code span of `docs/capabilities.md`; every cause of `Cause::name`'s match arms (`packages/escher-driver/src/refusal.rs`) is one; the statement states five counts in words read from the schema's tables — `{word} verbs` · `{word} refusal causes` · `{word} session errors` (`SESSION_ERRORS`, `error.rs`) · `{word} node keys` (`NODE_FIELDS`) · `{word} exit statuses` (`EXIT_STATUSES`), each word from `NUMBER_WORDS` (one … twenty), none hand-copied; and README.md holds «the {word} driver commands as tools»;

## Held

- ci-integration#1.1
- ci-integration#4.1
- ci-integration#6.1
- e2e-test-strategy#1.1
- e2e-test-strategy#1.2
- integration-test-strategy#2.1
- integration-test-strategy#2.2
- integration-test-strategy#2.3
- mocking-stubbing-discipline#1.17
- mocking-stubbing-discipline#1.5
- mocking-stubbing-discipline#1.6
- mocking-stubbing-discipline#3.1
- quality-gates-coverage-targets#3.1
- test-data-fixtures#2.1
- test-data-fixtures#3.11
- test-data-fixtures#3.2
- test-scope-summary#1.1
- test-scope-summary#11.1
- test-scope-summary#2.1
- test-scope-summary#2.3
- test-scope-summary#7.2
- test-strategy#1.1
- test-strategy#2.1
- test-strategy#2.2
- test-strategy#3.1
- test-strategy#6.1
- test-strategy#8.1
- unit-test-strategy#2.1
- unit-test-strategy#2.2
- unit-test-strategy#3.1
- unit-test-strategy#5.1

## Cascade only

- e2e-test-strategy#2.2
- mocking-stubbing-discipline#1.11
- test-scope-summary#9.1
- unit-test-strategy#9.4
