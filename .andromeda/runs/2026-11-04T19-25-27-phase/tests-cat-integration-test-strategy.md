@carried n=1 label=test-plan fact=fct-q24sm5dl
why: A probe check kept in `tests/blitz-tests/` must be a Cargo integration test with `#[test]` functions, depending only on dev-dependencies.
quote: all tests are Cargo integration tests in `tests/blitz-tests/tests/` with `#[test]` functions, depending only on dev-dependencies
@carried n=2 label=test-plan fact=fct-vu43sciq
why: The probe's measured reading of link (2) (the click listener) and link (3) (handler to state) must drive events through harness input, the real dispatch pipeline; `dispatch_recorded` bypasses Dioxus VirtualDom forwarding, so it cannot stand as evidence that a Dioxus handler ran.
quote: harness input routes through the document's real event-dispatch pipeline without a window (packages/blitz-test-harness/src/lib.rs:13-14; packages/blitz-test-harness/src/input.rs:1-5); dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as Dioxus VirtualDom forwarding
@carried n=3 label=test-plan fact=fct-5mwtk4y2
why: The probe reads across the VirtualDom-to-DioxusDocument boundary with no Dioxus change, so this existing boundary check must stay green and unchanged.
quote: `keyed_nodes_do_not_crash` runs a real `VirtualDom` against a `DioxusDocument` through 100 inserts, then checks that `<main>` has 100 children
@carried n=4 label=test-plan fact=fct-i47d76zr
why: A probe of the listener map or handlers inside `dioxus-native-dom` must follow the bridge-test pattern, a `cfg(test)` child of `dioxus_document`, and must add no constructor, `pub` or `cfg(test)` shim to the code under test; anything more is an escher-side change, and the verdict must name it as "exposed with a cost".
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor
@carried n=4 run=2 label=test-plan fact=fct-i47d76zr
quote: no function under test gained a constructor, a `pub` or a `cfg(test)` shim
@carried n=5 label=test-plan fact=fct-gq2zcc44
why: The probe's lean-task run (Counter or CRUD) on the stand must use the held-session shape and read in both layout modes.
quote: one `escher_driver::Session` booted through `seven_guis::stand` holds state across commands and reads as a fresh boot, in both layout modes
@carried n=6 label=test-plan/integration-test-strategy fact=fct-322qvjpg
why: Where layout touches a probe reading, the probe must run both layout modes the way the pipeline tests do.
quote: runs `for incremental in [false, true]` as the pipeline tests do, and `incremental_oracle`, green beside it, is the layout-identity witness
@carried n=7 label=test-plan fact=fct-ubyqdssx
why: A probe check on a held session must read no wall clock and take no sleep; the two recorded exceptions are the only ones allowed.
quote: no sleep and no wall-clock read in the checks, except the two whose subject is wall time
@carried n=7 run=2 label=test-plan/integration-test-strategy fact=fct-t7rg232i
quote: with no sleep and no wall-clock read in the checks — bar two recorded exceptions whose subject is wall time
@carried n=8 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: If Cells is the probe's served task, a click after a `scroll` must come after an `advance` past the overlay scrollbar's fade, or the click dispatches nothing and the handler reading is false.
quote: a click on a Cells cell inside a scrolled box right after `scroll` lands on the shown overlay scrollbar, which takes it as a thumb drag and dispatches nothing, and a pointer left on the thumb keeps the scrollbar shown, so an `advance` past the fade comes before the click
@carried n=9 label=test-plan fact=fct-lm4npbmj
why: The probe must leave the driver's one fixed-word log line as it is, with no component name, handler or signal value added to it.
quote: with escher's sink installed at `info`, each call run through `Session::run` leaves exactly one `escher_driver` line holding fixed words and counts only, none of the ids, names, values or typed text it handled, and at the default level no driver line
@carried n=10 label=test-plan fact=fct-kudmubnq
why: Nothing the probe reads may reach stdout, stderr or the state directory, and the standing quiet checks must stay green.
quote: the session library and the harness path write nothing of the screen — typed text, a stable id, an accessible name — to stdout, to stderr or into the state directory
@carried n=10 run=2 label=test-plan/integration-test-strategy fact=fct-adkwyvky
quote: `stand_session_quiet` proves the session library and the harness path write nothing of the screen — typed text, a stable id, an accessible name — to stdout, to stderr or into the state directory, whose listing is `session.sock` alone
@binding n=1
**Harness ↔ event pipeline ↔ the scope's "Measured, not read"**: each link verdict rests on a run on the stand, so handler and click readings must go through harness input, never `dispatch_recorded` (test-plan fct-vu43sciq)
@binding n=2
**Both layout modes ↔ the scope's "in both layout modes where layout touches the reading"**: the probe runs `incremental` false and true like the pipeline and session checks (test-plan/integration-test-strategy fct-322qvjpg; test-plan fct-gq2zcc44)
@binding n=3
**Session host ↔ what it writes ↔ the scope's "Nothing crosses" and obs-plan's masking rule**: a probe adds no log field and prints nothing it reads; the driver's line and the quiet checks stay as they are (test-plan fct-lm4npbmj; test-plan fct-kudmubnq; test-plan/integration-test-strategy fct-adkwyvky)
@binding n=4
**The bridge-test pattern ↔ the scope's verdict "exposed with a cost"**: a reading that needs more than a `cfg(test)` child of `dioxus_document` is an escher-side change inside `dioxus-native-dom`, and the verdict must name its cost and boundary (test-plan fct-i47d76zr)
