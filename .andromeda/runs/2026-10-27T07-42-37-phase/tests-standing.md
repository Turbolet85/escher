# tests · standing conventions

- S1 · A blitz-tests test is one file and one target per behaviour, auto-discovered, its `mod` line in the unbuilt `all.rs`, and three shared modules are no target · test-strategy.md
- S2 · Pipeline and stand tests run in both layout modes, `for incremental in [false, true]`, and the differential oracle compares every node after each step · test-strategy.md
- S3 · Older font-dependent tests skip at runtime with no usable font, while the headless-stand checks take no skip path under the bundled font · test-strategy.md
  (.andromeda/test-plan-detail/test-strategy.md:40) strategy
- S4 · A shot's PNG bytes differ by OS, so a check compared byte for byte across OSes holds no screenshot step · test-strategy.md
- S5 · Unit tests sit in `#[cfg(test)]` modules in the source file or in `#[path]` child files, their test paths unchanged · unit-test-strategy.md
- S6 · A check that reads a sink's capture counts lines by target, never in total · ci-integration.md
- S7 · Every cargo leg runs through `ci-leg.sh` with `--locked`, and each leg's log goes to `target/ci-logs/{leg}.log` · ci-integration.md
- S8 · The stand checks, the ten named targets and seven_guis' process checks hold a zero-failure budget with no retry · quality-gates-coverage-targets.md
- S9 · A new test data seed is stated in code, with no seeded store · test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:20) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:25) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:26) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:27) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:28) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:29) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:30) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:31) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:34) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:35) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:37) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:39) data-seeds
