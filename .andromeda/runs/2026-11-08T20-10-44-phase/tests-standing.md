# tests · standing conventions

- S1 · a new integration check is one test-target file per behavior under tests/blitz-tests/tests, a stand check carrying the stand_ prefix so agent-run's run stand selects it, with no new [[test]] table in the manifest · §2 Test Strategy (Test directory + naming conventions) and §4 Unit Test Strategy (blitz-tests targets)
  (test-plan/unit-test-strategy fct-l2ouwaqy) unit
- S2 · a check that holds a session declares mod session_common; and reads the shared builders, and a page more than one check reads is stated once in tests/fixtures/mod.rs; tables are stated expectation, never a second mapping · §3 Test Harness Contract (Crate-local test helpers)
  (test-plan/test-data-fixtures fct-y2fxawue) data-seeds
  (test-plan/test-strategy fct-xqiavvkf) naming
- S3 · a session check runs in both layout modes, incremental and not · §5 Integration Test Strategy (Session ↔ held instance)
- S4 · no sleep and no wall-clock read in a session check, and no pass in a Timer flow but the driver's · §5 Integration Test Strategy (Wall time in the session checks)
- S5 · unit tests sit in #[cfg(test)] mod tests in the source file, or in a #[path] file declared at its parent with test paths unchanged · §4 Unit Test Strategy (Conventions)
  (test-plan/integration-test-strategy fct-ssa4tu2c) integration
  (test-plan/unit-test-strategy fct-dssptnyw) unit
  (test-plan/unit-test-strategy fct-wdvc4drt) unit
  (test-plan/unit-test-strategy fct-7qct2f7a) unit
- S6 · a check writes and logs nothing of the screen — no stable id, accessible name or typed text on stdout, stderr or a state directory; assertion messages carry a row index only · §5 Integration Test Strategy (Session host ↔ what it writes) and §3 Stand log format
  (test-plan/integration-test-strategy fct-222mhgz6) integration
  (test-plan/test-scope-summary fct-ejy2b2a5) scope-entities
- S7 · a check that needs a session process bounds every wait, holds the guard that kills and reaps the host, removes its state directory and is cfg(unix)-gated · §2 Test Strategy (Process-lifecycle checks)
  (test-plan/test-strategy fct-jqnoqslu) strategy
- S8 · a recorded check committed as data is kept fresh by a byte-for-byte comparison with what the binary records and holds no typed text, host path, secret or screenshot step · §7 Test Data & Fixtures (tests/blitz-tests/checks) and §2 Test Strategy (Recorded-check replay)
- S9 · a chunk adding stand checks restates agent-run's run stand run.end count, as measured in its report · §3 Test Harness Contract (Agent-run contract, Proof)
- S10 · a check reading a sink's capture counts lines by target, never in total, since the workspace and package-alone builds differ in engine features · §9 CI Integration (Engine features by runner)
  (test-plan/ci-integration fct-prdavblg) ci
