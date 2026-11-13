# tests · standing conventions

- S1 · each new frame or pixel test is driven headless through the harness, renders on a CPU buffer and compares frames whole · test-strategy · 2-test-strategy
- S2 · each new layout or frame assertion runs in both layout modes · integration-test-strategy · 5-integration-test-strategy
- S3 · each new test target is one file holding one behaviour, joining the pinned target list · test-scope-summary · 1-test-scope-summary
- S4 · each new font-dependent test runs on a real pinned font and never skips · mocking-stubbing-discipline · 8-mocking-stubbing-discipline
  decision/font-metric-tests-never-skip · Font-metric tests never skip — A font-metric test takes no skip path: it asserts its text-size precondition and fails rather than skipping. · why: The skip came from the fixture's no-namespace element, not from missing system fonts. · guard: hover_state_tests
  decision/font-tests-skip-without-font · Pre-existing font tests skip without a font — The pre-existing font-dependent tests skip at runtime when no usable font is available.
- S5 · each new regression test is named for and documents its bug · 2-test-strategy
  decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
  decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
- S6 · each new test states its data in code, reads no wall clock and sleeps never · test-data-fixtures · 7-test-data-fixtures
  decision/blitz-dom-fixtures-built-manually · blitz-dom fixtures built without the HTML parser — blitz-dom tests build their DOM fixtures manually through the document mutator rather than by parsing HTML. · why: the HTML parser lives in blitz-html, which would be a circular dev-dependency
  decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
  decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
  decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
- S7 · each new test is controlled red-before-green and its failure prints no argument value · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
  decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again.
- S8 · each new or edited CI workflow line pins its actions to a commit SHA, holds read-only permissions and uses no secrets · unit-test-strategy · 4-unit-test-strategy
