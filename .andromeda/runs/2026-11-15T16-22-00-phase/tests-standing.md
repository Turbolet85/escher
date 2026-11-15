# tests · standing conventions

- S1 · unit tests sit in an inline test module in their source file · 2-test-strategy · test-strategy
- S2 · each new test is controlled red-before-green · 4-unit-test-strategy · unit-test-strategy
- S3 · test data stated in code, nothing seeded · 7-test-data-fixtures · test-data-fixtures
- S4 · hand-written stand-ins, no mocking library; network stand-ins keep their loads counted · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
- S5 · regression tests document and are named for their bug · 2-test-strategy · test-strategy
- S6 · test failures print no argument value · 4-unit-test-strategy · unit-test-strategy
- S7 · Rust tests on the built-in harness · 3-test-harness-contract
