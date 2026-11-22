# tests · standing conventions

- S1 · every CI job or step pins its actions to commit SHAs, runs with read-only permissions and uses no secrets · 4-unit-test-strategy · unit-test-strategy
- S2 · every CI leg runs through the CI leg script, host-reproducible, its cargo commands run locked · 9-ci-integration · ci-integration
- S3 · every CI download or package install is bounded and retried · 9-ci-integration · ci-integration
  decision/apt-install-retries · CI package install retried — A failed install attempt is tried again from `update`, a stalled call ends at its bound and is tried again, and an install failed by its bound is repaired before the next attempt. · guard: InstallScriptTest
- S4 · every change to a CI script or the workflow is held by the CI Python scripts' unit tests, run under unittest · 1-test-scope-summary · test-scope-summary
- S5 · every new test is controlled red before green and, where table-driven, asserts its row count · 4-unit-test-strategy · unit-test-strategy
  decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
  decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, a case it leaves green widened until it does, and the restored tree reads green; a control removing a bound loosens it to twice the budget instead, so the mutated run stays bounded.
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
