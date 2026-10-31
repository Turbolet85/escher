# tests · standing conventions

- S1 · a pipeline check runs `for incremental in [false, true]`, the same assertions in each layout mode · §5 Integration Test Strategy
- S2 · blitz-tests holds one target per behaviour file under `tests/`, with no new `[[test]]` table, as `BlitzTestsTargetsTest` pins · §2 Test Strategy
  (.andromeda/test-plan-detail/unit-test-strategy.md:43) unit
- S3 · a regression test is named for the bug it guards · .andromeda/test-plan-detail/test-strategy.md
- S4 · a unit test sits inline in `#[cfg(test)]` or in a `#[path]` child file with its test path unchanged · §4 Unit Test Strategy
  (.andromeda/test-plan-detail/unit-test-strategy.md:8) unit
- S5 · a check reads shared tables and helpers through `mod common;`, `mod session_common;` and `mod fixtures;`, stating no second role or name mapping · §3 Test Harness Contract
- S6 · a fixture is stated in code and seeded by nothing; a page with one reader stays in its check's file and a page with more moves to `tests/fixtures/mod.rs`, which holds no print, env read, socket or `blitz_net` · .andromeda/test-plan-detail/test-data-fixtures.md
  (.andromeda/test-plan-detail/test-data-fixtures.md:20) data-seeds
- S7 · a stand check asserts under the bundled DejaVu Sans with system fonts off and no skip path · §2 Test Strategy
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:38) mocking
  (.andromeda/test-plan-detail/test-data-fixtures.md:34) data-seeds
  (.andromeda/test-plan-detail/test-strategy.md:40) strategy
- S8 · a session check holds no sleep and no wall-clock read unless wall time is its subject · §5 Integration Test Strategy
- S9 · a `stand_` file is picked up by `agent-run.sh` with no script change, any other is read by name and by `run all`, and each change re-counts `run.end` in the contract's proof line · §3 Test Harness Contract
  (.andromeda/test-plan-detail/test-strategy.md:22) strategy
- S10 · a stand check holds the zero-failure budget of `stand-repeat`, three rounds with no retry, whose named targets a new non-prefixed check may join · §10 Quality Gates & Coverage Targets
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:22) gate-flakiness
- S11 · a change passes `bash .github/scripts/ci-leg.sh fast` before push, every cargo command `--locked` · §9 CI Integration
  (.andromeda/test-plan-detail/ci-integration.md:57) ci
- S12 · the per-crate line floors hold for `packages/escher-driver`, `packages/dioxus-native-dom` and `packages/blitz-test-harness`, and a mutation run stays on the dev host · §10 Quality Gates & Coverage Targets
  (.andromeda/test-plan-detail/ci-integration.md:57) ci
  (.andromeda/test-plan-detail/ci-integration.md:79) ci
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) gate-coverage-floor
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:28) gate-mutation
