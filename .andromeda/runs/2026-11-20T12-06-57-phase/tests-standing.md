# tests · standing conventions

- S1 · a new unit test sits in an inline test module in its source file · 2-test-strategy · test-strategy
- S2 · a new unit test pins its expected value as a literal, is controlled red-before-green, and its failure prints no argument value · 4-unit-test-strategy · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
- S3 · a new test uses hand-written stand-ins, never a mocking library · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
- S4 · a new test's data is stated in code, nothing seeded · 7-test-data-fixtures · test-data-fixtures
- S5 · a regression test documents and is named for what it guards · 2-test-strategy · test-strategy
  decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
  decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
