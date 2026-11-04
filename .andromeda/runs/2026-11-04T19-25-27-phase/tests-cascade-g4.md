@carried n=1 label=test-plan/test-strategy fact=fct-yivrglnx
why: every probe run, and the MountedData re-measurement through poll(None), must go through the harness's construction and pump loop over a DioxusDocument booted on the stand.
quote: the harness wraps any blitz_dom Document, such as HtmlDocument or DioxusDocument, with deterministic construction
  defaults, a pump/tick loop, `settle` (that pass repeated until no work is due, returning `Settled` or `NotSettled`
  naming the `Busy` class), inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-19)
@carried n=1 run=2 label=test-plan fact=fct-e36kow75
quote: pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded
  do not pump (packages/blitz-test-harness/src/harness.rs:157-189)
@carried n=1 run=3 label=test-plan fact=fct-65u33zgx
quote: The seven_guis headless stand boots each lean task (Counter, FlightBooker, Timer, Crud) through
  `seven_guis::stand::boot(LeanTask, HarnessOptions)` or `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>,
  TimerTicks)`, each a fresh `VirtualDom` over `from_vdom` rendering the task in TaskShell, with
  `stand::options(incremental)` the pinned options
@carried n=2 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a probe check the plan keeps under tests/blitz-tests/tests/ must be counted into the blitz-tests target count, its own auto-discovered target with no manifest line.
quote: every other `.rs` file directly under `tests/` stays its own auto-discovered target: 157 `.rs` files, 156 test
  targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
@carried n=2 run=2 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 ·
  failed 0 · ignored 12
@carried n=3 label=test-plan/unit-test-strategy fact=fct-dssptnyw
why: a probe reading a handler from inside dioxus-native-dom must sit in a cfg(test) child of dioxus_document, and the handler's privacy must be read as evidence for the handler links' verdicts, never opened up.
quote: `dioxus_document::bridge_tests` in `dioxus_document_tests.rs`, declared `#[cfg(test)] #[path =
  "dioxus_document_tests.rs"] mod bridge_tests;` at the end of its parent, a child of `dioxus_document` because
  `DioxusEventHandler`'s two fields are private to that module and it has no constructor
  (packages/dioxus-native-dom/src/dioxus_document.rs:838-840; packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-6)
@carried n=3 run=2 label=test-plan fact=fct-i47d76zr
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file
  `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no
  constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they
  keep, the one way to read `propagation_is_stopped()`, a flag nothing else in the workspace reads; no function under
  test gained a constructor, a `pub` or a `cfg(test)` shim
@carried n=3 run=3 label=test-plan fact=fct-tkvntwtm
quote: `dioxus_document::bridge_tests` is a child of `dioxus_document` because `DioxusEventHandler`'s two fields are
  private to that module and it has no constructor
@carried n=4 label=test-plan/unit-test-strategy fact=fct-klxgwgcy
why: a kept probe check's assertions must carry fixed words or a row index only, so no component name, handler or signal value the probe reads reaches a failure message.
quote: with every assertion over a call carrying a row index only, so a failing log prints no argument value
@carried n=4 run=2 label=test-plan fact=fct-mnyw2bw5
quote: command.rs's assertions over a call carry a row index only, so a failing log prints no argument value
@carried n=5 label=test-plan/integration-test-strategy fact=fct-322qvjpg
why: each verdict whose reading layout touches must be run in both layout modes, incremental false and true.
quote: runs `for incremental in [false, true]` as the pipeline tests do, and `incremental_oracle`, green beside it, is
  the layout-identity witness (§2 Differential oracle)
@carried n=6 label=test-plan/e2e-test-strategy fact=fct-kgita7a3
why: the chunk must re-verify this hazard's mechanism and its events.rs:162 coordinate at HEAD, headlessly, with the windowed path read and not run, and name it in the founder's reading.
quote: Cells calls its `MountedData` handles synchronously, measured headlessly, their windowed path read from the same
  lines and not run; a `MountedData` call in a task spawned from a key handler panics with `RefCell already borrowed`
  at `events.rs:162` (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md)
@carried n=7 label=test-plan/test-data-fixtures fact=fct-qbzx34mx
why: the served task beside the lean one is a fork the plan settles between boot_cells and boot_load, and a load-page run must meet LoadNet's 200 ms threaded answer without a sleep or a wall-clock read.
quote: `stand::boot_load`'s stand-in `LoadNet` answers every fetch of the fixed URL `http://stand.test/load.css` with
  the fixed bytes `LOADED_CSS` (`.load-body .load-done { display: block; }`) from a thread of its own after
  `LOAD_ANSWER_DELAY` (200 ms); every other stand boot keeps `net_provider: None`
@carried n=7 run=2 label=test-plan/mocking-stubbing-discipline fact=fct-eeplrkyg
quote: `stand::boot_load` replaces its options' `net_provider` with `Some(Arc::new(LoadNet))` through `..options`, every
  other boot keeping `net_provider: None` (`the_stand_options_stay_offline`); each `fetch` answers the fixed sheet
  `LOADED_CSS` for `http://stand.test/load.css` from a thread of its own after `LOAD_ANSWER_DELAY` (200 ms); it does not
  override `is_noop`, so the harness counts its load
@carried n=7 run=3 label=test-plan fact=fct-65u33zgx
quote: beside the lean tasks it boots Cells through `stand::boot_cells(HarnessOptions)` (`Task::Cells` in TaskShell, no
  `LeanTask`) and the load page through `stand::boot_load(HarnessOptions)` (TaskShell titled "Load", its options'
  `net_provider` replaced by the stand's `LoadNet`, every other field kept)
@carried n=8 label=test-plan/mocking-stubbing-discipline fact=fct-bz6tsqol
why: link (2) and the click reading must not rest on a recorded event name alone, since dispatch_recorded drives the BaseDocument and runs no Dioxus handler.
quote: `RecordingHandler` EventHandler installed by dispatch_recorded records each dispatched DOM event name
@carried n=8 run=2 label=test-plan fact=fct-vu43sciq
quote: dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as
  Dioxus VirtualDom forwarding (packages/blitz-test-harness/src/harness.rs:174-212)
@carried n=8 run=3 label=test-plan fact=fct-gix7v44z
quote: `dispatch_recorded([UiEvent, ...])` returns the list of dispatched event names
@binding n=1
**test-strategy (harness pump) ↔ e2e-test-strategy (MountedData hazard)**: the hazard's re-verification must drive
  a key handler through the harness's `press` on a stand boot and reach the panic through `pump`'s waker-less poll,
  the poll(None) path it was measured on (test-plan/test-strategy fct-yivrglnx) (test-plan fct-e36kow75); reproducing
  it changes no Dioxus or engine code, and the coordinate `events.rs:162` is re-read at HEAD
  (test-plan/e2e-test-strategy fct-kgita7a3)
@binding n=2
**mocking-stubbing-discipline (RecordingHandler) ↔ test-strategy (input helpers)**: link (2)'s answer for `click`
  must separate a listener read from the bridge's listener map and a Dioxus handler run through a pumping input helper
  from a name `dispatch_recorded` returns, which bypasses VirtualDom forwarding (test-plan/mocking-stubbing-discipline
  fct-bz6tsqol) (test-plan fct-vu43sciq)
@binding n=3
**unit-test-strategy (bridge_tests) ↔ the probe's verdict classes**: a handler or state reading made from inside the
  bridge sits in a cfg(test) child of `dioxus_document`, and where it would need a constructor, a `pub` or a cfg(test)
  shim the verdict is «exposed with a cost», named and not taken (test-plan/unit-test-strategy fct-dssptnyw) (test-plan
  fct-i47d76zr)
@binding n=4
**test-data-fixtures (LoadNet) ↔ mocking-stubbing-discipline (LoadNet) ↔ integration-test-strategy (both layout
  modes)**: whichever served task the plan picks, it boots through `boot_cells` or `boot_load` with the stand's pinned
  options in both layout modes where layout touches the reading, a load-page run meeting LoadNet's counted 200 ms load
  through the harness's own passes (test-plan/test-data-fixtures fct-qbzx34mx) (test-plan/mocking-stubbing-discipline
  fct-eeplrkyg) (test-plan/integration-test-strategy fct-322qvjpg)
@binding n=5
**test-strategy (blitz-tests targets) ↔ the body's agent-run proof ↔ unit-test-strategy (assertion discipline)**: a
  probe check the plan keeps is its own auto-discovered target that re-counts the target total and the newest proof
  reading, its assertions carrying fixed words or a row index only (test-plan/test-strategy fct-7m7lxsmy) (test-plan
  fct-glytb3w3) (test-plan/unit-test-strategy fct-klxgwgcy)
