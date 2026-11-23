# tests · standing conventions

- S1 · each new test target is one file, one behaviour, run in both layout modes on the bundled font · test-strategy · 2-test-strategy
- S2 · each new test is controlled red before green, its failure printing no argument value · unit-test-strategy · 4-unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
- S3 · each new stand check holds under the zero-failure stand-repeat budget · quality-gates-coverage-targets · 10-quality-gates-coverage-targets
- S4 · each new test's data is stated in code, offline, with a hand-written stand-in and no mocking library · test-data-fixtures · 7-test-data-fixtures
  decision/blitz-dom-fixtures-built-manually · blitz-dom fixtures built without the HTML parser — blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML. · why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
  decision/hand-written-stand-ins · Hand-written stand-ins, no mocking library — Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into their targets, by boot for the blitz-tests targets and by the run's own cargo call for the session selection's seven_guis target.
