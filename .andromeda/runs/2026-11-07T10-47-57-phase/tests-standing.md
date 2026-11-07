# tests · standing conventions

- S1 · a new unit test sits in a #[cfg(test)] mod tests block inside the source file under test, or in a #[path] child file declared #[cfg(test)] at its parent with its test path unchanged · §4 Unit Test Strategy, Conventions (unit-test-strategy)
  (test-plan/integration-test-strategy fct-ssa4tu2c) integration
  (test-plan/test-data-fixtures fct-243i7pja) data-seeds
  (test-plan/test-strategy fct-7m7lxsmy) naming
  (test-plan/unit-test-strategy fct-dssptnyw) unit
- S2 · a regression test is named for the bug it guards and documents the bug it guards · §2 Test Strategy, Test directory and naming conventions (test-strategy)
- S3 · a dioxus-native-dom unit test builds its DioxusDocument from DocumentConfig::default() with initial_build, drives updates with mark_dirty and poll(None), and gains no constructor, pub or cfg(test) shim on the code under test · §3 Test Harness Contract, Crate-local test helpers
- S4 · a check over a Dioxus document runs in both layout modes, incremental and non-incremental · §5 Integration Test Strategy (integration-test-strategy)
- S5 · a check holds no sleep and no wall-clock read unless wall time is its subject · §5 Integration Test Strategy, Wall time in the session checks
- S6 · a new source module counts toward its crate's line coverage floor in the coverage leg · §10 Quality Gates and Coverage Targets (quality-gates-coverage-targets)
  (test-plan/ci-integration fct-na5rpohe) ci
