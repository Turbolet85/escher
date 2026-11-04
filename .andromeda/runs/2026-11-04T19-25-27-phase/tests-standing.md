# tests · standing conventions

- S1 · unit tests sit in a #[cfg(test)] mod tests inside the source file, or in a file of their own declared #[cfg(test)] with #[path] at its parent, their test paths unchanged · unit-test-strategy
  (test-plan/integration-test-strategy fct-ssa4tu2c) integration
  (test-plan/unit-test-strategy fct-wdvc4drt) unit
  (test-plan/unit-test-strategy fct-7qct2f7a) unit
- S2 · blitz-tests keeps one test-target file per behaviour under tests/blitz-tests/tests/, picked up by autotests with no [[test]] table added beside the unbuilt all, as BlitzTestsTargetsTest pins · §2. Test Strategy (Directory pattern) and §4. Unit Test Strategy (blitz-tests targets)
  (test-plan/test-scope-summary fct-jal6nbq2) scope-entities
  (test-plan/test-scope-summary fct-j7rhr47j) scope-entities
  (test-plan/unit-test-strategy fct-l2ouwaqy) unit
- S3 · a stand check declares mod common; after its use block and reads the shared tables and helpers of tests/common/mod.rs, a fixture page more than one check reads lives in tests/fixtures/mod.rs, and a page with one reader stays in its check's file · §3. Test Harness Contract (Crate-local test helpers) and test-data-fixtures
  (test-plan/test-data-fixtures fct-y2fxawue) data-seeds
  (test-plan/test-data-fixtures fct-zkjxcvz6) data-seeds
  (test-plan/test-data-fixtures fct-x42jilyi) data-seeds
- S4 · a regression test is named for the bug it guards and documents the bug and what exposed it · §2. Test Strategy (Test function naming)
- S5 · a headless-stand check asserts unconditionally under the bundled DejaVu Sans with system fonts off and has no skip path · §2. Test Strategy (Font-dependent tests) and mocking-stubbing-discipline
  (test-plan/mocking-stubbing-discipline fct-ya5o4ovd) mocking
  (test-plan/test-data-fixtures fct-afgwdd3i) data-seeds
  (test-plan/test-strategy fct-2vwamopj) strategy
- S6 · a check reads no wall clock and sleeps not, time moving only through the harness's virtual clock, bar the checks whose subject is wall time · integration-test-strategy
  (test-plan/mocking-stubbing-discipline fct-7t2tqfsg) mocking
- S7 · no third-party test framework or mocking library is added; Rust checks run under the standard built-in harness, with hand-written stand-ins only · §3. Test Harness Contract (Third-party test frameworks) and §8. Mocking & Stubbing Discipline
  (test-plan/mocking-stubbing-discipline fct-uyy2kykf) mocking
  (test-plan/mocking-stubbing-discipline fct-duynbxnl) mocking
  (test-plan/mocking-stubbing-discipline fct-5fficxxt) mocking
  (test-plan/mocking-stubbing-discipline fct-7agflx3x) mocking
  (test-plan/mocking-stubbing-discipline fct-uhjgy72w) mocking
  (test-plan/mocking-stubbing-discipline fct-pma3gjdm) mocking
  (test-plan/mocking-stubbing-discipline fct-quhgbzyl) mocking
  (test-plan/mocking-stubbing-discipline fct-e6u5yu6y) mocking
- S8 · every stand_ check holds the zero-failure budget, run three rounds with no retry in the stand-repeat leg · quality-gates-coverage-targets
- S9 · a check that reads a sink's capture counts lines by target, never in total, since the workspace and package-alone builds differ in engine features · ci-integration
