# tests · standing conventions

- S1 · a stand check sits in its own tests/blitz-tests/tests/stand_*.rs target, picked up by the stand_ prefix with no script change, declaring mod common, mod session_common or mod fixtures after its use block · test-strategy
- S2 · a fixture page more than one check reads is stated once in tests/fixtures/mod.rs and booted through fixtures::boot with the stand's options; a page with one reader stays in its check's file · test-data-fixtures
  (test-plan/test-data-fixtures fct-ay5qvau6) data-seeds
  (test-plan/test-data-fixtures fct-x42jilyi) data-seeds
- S3 · a session check is proven in both layout modes, incremental and not, with no sleep or wall-clock read and every wait bounded · integration-test-strategy
- S4 · a unit test sits in a #[cfg(test)] module in the source file, or a #[path] test file declared at its parent, its assertions over a call carrying a row index only so a failing log prints no argument value · unit-test-strategy
  (test-plan/integration-test-strategy fct-ssa4tu2c) integration
  (test-plan/unit-test-strategy fct-dssptnyw) unit
  (test-plan/unit-test-strategy fct-qjt5fv4i) unit
  (test-plan/unit-test-strategy fct-7qct2f7a) unit
- S5 · a check that holds a session process starts and stops its own host with bounded waits and a guard that kills and reaps it, its child's streams to null unless read · test-strategy
  (test-plan/test-strategy fct-jqnoqslu) strategy
- S6 · a new span or log line names fixed words and counts only, never an id, name, value or the read's content · ci-integration
  (test-plan/integration-test-strategy fct-222mhgz6) integration
- S7 · a new check carries a regression-style name stating the behaviour it guards · test-strategy
