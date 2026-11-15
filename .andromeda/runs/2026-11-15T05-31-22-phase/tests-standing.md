# tests · standing conventions

- S1 · a regression test added for a bug names the bug and documents it · 2-test-strategy
  decision/regression-tests-document-bug · Regression tests document their bug — A regression test documents the bug and the real-world page that exposed it.
  decision/regression-tests-named-for-bug · Regression tests named for their bug — A regression test is named for the bug it guards.
- S2 · a new test is controlled red before green · unit-test-strategy
  decision/mutation-controlled-tests · Tests controlled red-before-green — A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again; a control that removes a bound loosens it to twice the budget rather than removing it, so the mutated run stays bounded.
- S3 · a unit test sits in an inline test module of its source file · 2-test-strategy · test-strategy
  decision/bridge-tests-child-module · Bridge tests a child of the dioxus document — The dioxus document's bridge tests sit in a child module of the dioxus document module. · why: The event handler's two fields are private to that module and it has no constructor.
  decision/inline-cfg-test-modules · Unit tests in inline test modules — Unit tests sit in cfg(test) modules inside their source files. · except: cfg(test) modules held in files of their own through a path attribute at their parent.
  decision/inline-unit-test-modules · Inline unit test modules — Unit tests are Rust test functions in cfg(test) modules beside the code under test. · except: modules held in files of their own through a path attribute
  decision/unit-tests-in-source-file · Unit tests sit in their source file — Unit tests sit in a test-only module inside the source file; a module moved out of line sits in its own file declared test-only by path at its parent, its test paths unchanged. · except: the Dioxus bridge tests are a child of the Dioxus document module, because the event handler's two fields are private to it and it has no constructor
- S4 · a new test asserts no sleep or wall-clock read and drives time through the virtual clock · integration-test-strategy
  decision/no-sleep-or-wall-clock-in-checks · No sleep or wall-clock read in checks — Session checks take no sleep and read no wall clock; time moves by `advance` alone. · except: stand_virtual_clock's 800 ms sleep, stand_load_wait's clock reads and 200 ms stand-in sleep, stand_session_lifecycle's idle-expiry wait and stand_contrast's bound on a load's answer.
  decision/no-wall-time-in-session-checks · No wall time in session checks — Session checks hold no sleep and no wall-clock read; a check waits on a load through the driver's load wait, and the Timer flow advances by the driver only. · except: The checks whose subject is wall time: the virtual-clock check's 800 ms sleep, the load-wait check's clock reads and stand-in sleep, and the lifecycle check's wait on the host's idle expiry.
- S5 · a test failure prints no argument value · unit-test-strategy
  contract/record-checks-script/failure-output · The committed checks' recording script · failure output — Prints only step N on failure.
  decision/assertions-print-no-argument-value · Test failures print no argument value — An assertion over a driver call names only its row and index and never an argument value. · why: A failing test log then prints no argument value.
  decision/call-assertions-carry-row-index · Call assertions carry a row index only — The driver's assertions over a call carry a row index only, never an argument value. · why: so a failing log prints no argument value
