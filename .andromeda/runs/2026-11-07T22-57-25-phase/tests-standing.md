# tests · standing conventions

- S1 · unit tests sit in #[cfg(test)] mod tests in the source file, or in a #[path] file declared #[cfg(test)] at its parent, their test paths unchanged · §4 Unit Test Strategy (unit-test-strategy)
  (test-plan/integration-test-strategy fct-ssa4tu2c) integration
  (test-plan/mocking-stubbing-discipline fct-n5usxpjf) mocking
  (test-plan/unit-test-strategy fct-dssptnyw) unit
  (test-plan/unit-test-strategy fct-wdvc4drt) unit
  (test-plan/unit-test-strategy fct-7qct2f7a) unit
- S2 · blitz-tests holds one test-target file per behaviour under tests/blitz-tests/tests/, no [[test]] table added (BlitzTestsTargetsTest), a stand check named with the stand_ prefix so run stand selects it · §2 Test Strategy → Directory pattern (test-strategy)
  (test-plan/unit-test-strategy fct-l2ouwaqy) unit
- S3 · a regression test is named for the bug it guards and documents it · §2 Test Strategy → Test function naming (test-strategy)
- S4 · a stand or session check runs in both layout modes, incremental and not · §5 Integration Test Strategy (integration-test-strategy)
- S5 · no sleep and no wall-clock read in a session check unless wall time is its subject · §5 Integration Test Strategy → Wall time in the session checks (integration-test-strategy)
- S6 · a check declares mod common; / mod session_common; / mod fixtures; after its use block and reads the shared tables and helpers rather than restating them · §3 Test Harness Contract → Crate-local test helpers
- S7 · a check that needs a session process bounds every wait, kills and reaps its host by guard, removes its state directory, nulls the child's streams unless read, and is cfg(unix)-gated when it holds the socket · §2 Test Strategy → Process-lifecycle checks (test-strategy)
  (test-plan/test-strategy fct-jqnoqslu) strategy
- S8 · an assertion over a driver call carries a row index only, so a failing test's log prints no argument value · §4 Unit Test Strategy → escher-driver (unit-test-strategy)
