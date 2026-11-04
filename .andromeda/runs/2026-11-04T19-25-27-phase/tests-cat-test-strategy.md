@carried n=1 label=test-plan/test-strategy fact=fct-yivrglnx
why: each probe verdict must rest on a run on the stand driven through the harness over a DioxusDocument, not on reading the code.
quote: the harness wraps any blitz_dom Document, such as HtmlDocument or DioxusDocument, with deterministic construction defaults, a pump/tick loop, `settle` (that pass repeated until no work is due, returning `Settled` or `NotSettled` naming the `Busy` class), inspection helpers and input synthesis (packages/blitz-test-harness/src/lib.rs:1-19)
@carried n=2 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a probe check placed in tests/blitz-tests must be a target file of its own, auto-discovered with no manifest line, and it moves the stated file and target count.
quote: `tests/blitz-tests` holds integration tests, one test-target file per behavior under `tests/`, and three shared modules that are no target
@carried n=2 run=2 label=test-plan/test-strategy fact=fct-7m7lxsmy
quote: every other `.rs` file directly under `tests/` stays its own auto-discovered target: 157 `.rs` files, 156 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md
@carried n=2 run=3 label=test-plan/test-strategy fact=fct-7m7lxsmy
quote: the fork keeps one target per file by the operator's decision of 2026-10-10, the two-line difference pinned by `BlitzTestsTargetsTest` (§4)
@carried n=3 label=test-plan/test-strategy fact=fct-7m7lxsmy
why: a probe of the bridge's private handler, written as a unit test, must follow the out-of-line `#[path]` module pattern beside `bridge_tests`, and a new such module moves the count of eight.
quote: inline `#[cfg(test)]` modules in source files (as above), bar eight `cfg(test)` modules held in files of their own through `#[path]` — `dioxus_document::bridge_tests` (packages/dioxus-native-dom/src/dioxus_document.rs:838-840)
@carried n=4 label=test-plan/test-strategy fact=fct-2vwamopj
why: a probe check that boots the stand (Counter or CRUD) must assert with no skip path, under the stand's bundled font with system fonts off.
quote: the headless-stand checks have no skip path — they assert unconditionally under the bundled DejaVu Sans `seven_guis::stand::font_ctx()` registers with system fonts off, passed through `HarnessOptions.font_ctx` (examples/seven_guis/src/stand.rs:59-61; tests/blitz-tests/tests/stand_boot.rs:51-65)
@carried n=5 label=test-plan fact=fct-i47d76zr
why: re-verifying the MountedData mechanism at HEAD must go through the poll path the unit tests already drive, and a probe of the listener map must not add a constructor, a `pub` or a `cfg(test)` shim to the bridge.
quote: the document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:467-477)
@carried n=5 run=2 label=test-plan fact=fct-i47d76zr
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep, the one way to read `propagation_is_stopped()`, a flag nothing else in the workspace reads; no function under test gained a constructor, a `pub` or a `cfg(test)` shim
@carried n=6 label=test-plan fact=fct-e36kow75
why: the hazard reading must name which harness path polls the document (pump, not dispatch), because that poll is where the task spawned from a key handler is first polled.
quote: pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump (packages/blitz-test-harness/src/harness.rs:157-189)
@carried n=7 label=test-plan fact=fct-temy7g7i
why: a probe check named `stand_*` joins the stand-repeat leg and must hold its zero-failure budget over three rounds with no retry; a check that is meant to read the panic must not be a red stand check.
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)
@carried n=8 label=test-plan fact=fct-4t3fuckc
why: any probe code kept inside dioxus-native-dom (an escher-side change with a cost) must keep that crate's line floor.
quote: three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
@binding n=1
**Harness-driven tests ↔ the scope's «measured, not read»**: each link's verdict, and the MountedData mechanism re-verified at HEAD, is a run through the harness over a DioxusDocument on the stand (test-plan/test-strategy fct-yivrglnx), whose pump is the poll the hazard sits in (test-plan fct-e36kow75; test-plan fct-i47d76zr).
@binding n=2
**Directory pattern ↔ §4 blitz-tests targets**: a probe check under tests/blitz-tests is its own auto-discovered file with no manifest line, so `BlitzTestsTargetsTest` stays green and the 157/156 count moves by each file added (test-plan/test-strategy fct-7m7lxsmy).
@binding n=3
**Bridge test discipline ↔ the scope's no-fork-level-change boundary**: a probe reading the listener map or the handler through private bridge state does so from a `#[path]` `cfg(test)` child module, adding no `pub`, constructor or shim, which keeps the probe inside the verdict classes and leaves the bridge's surface unchanged (test-plan/test-strategy fct-7m7lxsmy; test-plan fct-i47d76zr).
@binding n=4
**Stand fonts and naming ↔ §10 flakiness gate**: a probe check on a stand task asserts under the bundled font with no skip (test-plan/test-strategy fct-2vwamopj), and if it is named `stand_*` it runs in `run stand` and stand-repeat under the zero-failure budget (test-plan fct-temy7g7i); a reading that panics by design stays out of that prefix or is caught inside the check.
