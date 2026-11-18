# tests · standing conventions

- S1 · a CI Python script and its tests run under unittest in the ci-scripts leg · 1-test-scope-summary · 3-test-harness-contract
- S2 · a new assertion is shown red before green against a mutated input · unit-test-strategy
  decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
- S3 · an expected value is pinned as a literal, and a table-driven test asserts its row count · unit-test-strategy
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
