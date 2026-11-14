# tests · standing conventions

- S1 · every new test is controlled red-before-green — the mutation or hand edit that turns it red, and the reading that it did · unit-test-strategy · 4-unit-test-strategy
- S2 · unit tests sit in an inline test module of their own source file, not in a new test target · 2-test-strategy · test-strategy
  decision/blitz-tests-manifest-difference · blitz-tests manifest's difference from upstream — The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: The fork's standing two-line difference from upstream's manifest. · guard: BlitzTestsTargetsTest
  decision/blitz-tests-single-test-target · blitz-tests manifest holds one test target — The blitz-tests manifest does not set `autotests` to false and holds exactly one `[[test]]` table, `all`, with `test = false`. · why: the fork's standing two-line difference from upstream's manifest · guard: BlitzTestsTargetsTest
- S3 · a regression test documents and is named for the bug it pins · 2-test-strategy
  decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
  decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
- S4 · test data is stated in code — fixture URLs and documents written in the test, nothing seeded or read from disk · 7-test-data-fixtures · test-data-fixtures
  decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
- S5 · test failures print no argument value, and a table-driven test asserts its row count · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
- S6 · stand-ins are hand-written, with no mocking library · 8-mocking-stubbing-discipline · mocking-stubbing-discipline
  decision/hand-written-stand-ins · Hand-written stand-ins, no mocking library — Test doubles are hand-written stand-ins; no mocking library is used in CI, the build configuration or the apps.
- S7 · no sleep or wall-clock read in a check · integration-test-strategy
  decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
  decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
- S8 · workspace tests run in the fast CI leg across the native platform matrix · 1-test-scope-summary
