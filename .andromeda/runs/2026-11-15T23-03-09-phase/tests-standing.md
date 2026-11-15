# tests · standing conventions

- S1 · each new Python script test runs under unittest, offline, in the CI scripts' unittest leg · 3-test-harness-contract · 1-test-scope-summary
  decision/cold-agent-tests-offline · Cold-agent pipe tested offline — The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary. · guard: ColdAgentPipeTest
- S2 · each new test is controlled red-before-green, prints no argument value on failure, and a table-driven test asserts its row count · unit-test-strategy
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
  decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
  decision/table-tests-count-their-rows · Table-driven tests assert their row count — Each table a driver test reads is stated in the test itself and its row count asserted.
- S3 · each new test states its data in code and seeds nothing · 3-test-harness-contract
  decision/test-data-in-code · Test data stated in code — Test data is stated in code: inline HTML, literals and byte arrays, directly injected images, fixed viewports, embedded fonts and pages generated in the test. · except: one external benchmark page supplied through PAINT_TREE_BENCH_HTML
  decision/test-data-is-code · Test data is code, nothing seeded — Nothing is seeded for the agent-run contract: the fixtures are code compiled into each target by `boot`, so `boot` is the bootstrap.
- S4 · each new check uses no sleep or wall-clock read; waits are bounded · integration-test-strategy
  decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
  decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
  decision/worker-wait-bounded-in-tests · Bounded worker wait in tests — A test that waits on the URL-suggestions worker bounds the wait with a timeout. · why: So a regression would not hang CI.
