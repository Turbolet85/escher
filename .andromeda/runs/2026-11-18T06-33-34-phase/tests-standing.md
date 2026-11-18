# tests · standing conventions

- S1 · each new Python script test is written under unittest · 3-test-harness-contract
  decision/ci-scripts-unit-tested · CI Python scripts unit-tested — The CI Python scripts are unit-tested by unittest discovery over the CI scripts directory, run as the fast ci-scripts leg, contract tests of the CI install, coverage floor and blitz-tests manifest scripts among them. · guard: ci-scripts leg
  decision/python-script-tests-under-unittest · Python script tests under unittest — The Python tests of the CI scripts run under the standard library's unittest, by discovery over the CI scripts directory.
- S2 · each new cold-agent test runs offline on the shim and the fake server, never reaching a model provider · unit-test-strategy
  decision/test-tier · Test tier — The project's test tier is 0.
- S3 · each new test is controlled red before green · unit-test-strategy
- S4 · each new table-driven test asserts its row count, pins its expected value as a literal and prints no argument value on failure · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
  decision/layout-reason-words-each-read · Every layout reader word read on a fixture — Every placing word and every size cause of the layout reader is read on a fixture node in both layout modes, from tables that count their rows. · guard: layout_reason
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
