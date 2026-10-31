# tests · standing conventions

- S1 · A new blitz-tests check is one target per behaviour under `tests/blitz-tests/tests/`, with no new `[[test]]` table, as `BlitzTestsTargetsTest` pins; regression tests are named for the bug they guard · §2 Test Strategy and §4 Unit Test Strategy
  (.andromeda/test-plan-detail/test-scope-summary.md:8) scope-entities
  (.andromeda/test-plan-detail/unit-test-strategy.md:56) unit
- S2 · A new unit test sits in a `#[cfg(test)] mod tests` block in its source file, or in a `#[path]` file declared `#[cfg(test)]` at its parent · §4 Unit Test Strategy
  (.andromeda/test-plan-detail/unit-test-strategy.md:8) unit
- S3 · A new stand or session check runs in both layout modes, boots through the stand's pinned options with the bundled DejaVu Sans and system fonts off, and holds no skip path · §3 Test Harness Contract and test-data-fixtures.md
  (.andromeda/test-plan-detail/integration-test-strategy.md:8) integration
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:39) mocking
  (.andromeda/test-plan-detail/test-data-fixtures.md:34) data-seeds
  (.andromeda/test-plan-detail/test-strategy.md:40) strategy
- S4 · A fixture page more than one check reads is stated in `tests/blitz-tests/tests/fixtures/mod.rs`; a page with one reader stays in its check's file, and the module holds no print, `tracing` call, env read, file, socket or listener · §3 Test Harness Contract and §7 Test Data & Fixtures
  (.andromeda/test-plan-detail/test-data-fixtures.md:20) data-seeds
  (.andromeda/test-plan-detail/test-data-fixtures.md:25) data-seeds
- S5 · A shared stand table in `common/mod.rs` is stated expectation, never a second role or name mapping; a check declares `mod common;` or `mod session_common;` after its `use` block and its reader counts are kept · §3 Test Harness Contract
- S6 · A session check holds no sleep and no wall-clock read unless wall time is its subject · §5 Integration Test Strategy
- S7 · A frame compared byte for byte stays on one host; a check compared across OSes holds no shot, and a shot of a document with no resolved font is refused with no skip path · §2 Test Strategy
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:39) mocking
- S8 · A stand check holds the zero-failure budget of the `stand-repeat` leg, three rounds with no retry, and a target with no `stand_` prefix that must repeat joins that leg's named list · §9 CI Integration and §10 Quality Gates & Coverage Targets
  (.andromeda/test-plan-detail/test-scope-summary.md:228) scope-entities
  (.andromeda/test-plan-detail/unit-test-strategy.md:45) unit
- S9 · New code stays above the per-crate line floors of `coverage_floor.py`, `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % · §10 Quality Gates & Coverage Targets
  (.andromeda/test-plan-detail/ci-integration.md:105) ci
  (.andromeda/test-plan-detail/quality-gates-coverage-targets.md:14) gate-coverage-floor
- S10 · A check holds under both the workspace and the `-p blitz-tests` engine features, every cargo leg but `fmt` runs `--locked`, and a green per-package run is not the workspace leg's green · §9 CI Integration
- S11 · A chunk that adds stand checks states the new `run stand` passed, failed, ignored and file counts, a new `stand_` file being picked up by its prefix with no script change · §3 Test Harness Contract
- S12 · A check or a log path writes no stable id, accessible name or typed text to stdout, stderr or the state directory, and no third-party test or mocking framework is added · §5 Integration Test Strategy and §8 Mocking & Stubbing Discipline
  (.andromeda/test-plan-detail/integration-test-strategy.md:33) integration
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:45) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:46) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:47) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:48) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:49) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:50) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:51) mocking
  (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:52) mocking
  (.andromeda/test-plan-detail/test-scope-summary.md:88) scope-entities
