@carried n=1 label=test-plan/test-scope-summary fact=fct-e2zfwpza
why: The MountedData hazard is the task-polled-during-render case; the reading must name this absent upstream test and its untaken commit as part of the fix's options brought to the founder, never take it in this chunk.
quote: upstream's `task_polled_during_render_can_access_document`, its commit `286874cc1` not taken, has no counterpart in the fork's dioxus_document.rs test module — as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md
@carried n=2 label=test-plan/test-scope-summary fact=fct-ybmlkzsq
why: A fix option that takes `286874cc1` brings this test back and touches the untested `flush_queued_mounted_events` guard; the fork's options must state that test consequence.
quote: upstream's `element_removed_before_its_mounted_event_is_flushed` is left out, because it reads 1 mounted event on Dioxus 0.7 where it asserts 0; it returns with `286874cc1`, so the `6ef4b532d` guard in `flush_queued_mounted_events` has no test of its own (packages/dioxus-native-dom/src/dioxus_document.rs:235-239)
@carried n=3 label=test-plan/test-scope-summary fact=fct-i5a6vv3t
why: The probe reads the bridge (`dioxus-native-dom`); any probe check kept as a unit test of the crate must restate this count, and the probe's readings of mounted data and listeners must start from the existing bridge and document tests.
quote: 82 unit tests in nine files (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)
@carried n=3 run=2 label=test-plan/test-scope-summary fact=fct-i5a6vv3t
quote: four in dioxus_document.rs's test module, `removed_nodes_are_dropped`, `suspended_nodes_are_dropped_and_recreated`, `todo_like_stress` and `mounted_data_of_removed_element_reports_error` (packages/dioxus-native-dom/src/dioxus_document.rs:518-835)
@carried n=3 run=3 label=test-plan/test-scope-summary fact=fct-i5a6vv3t
quote: seven on the bridge (six until 2026-10-16-fast-failing-mutants), in the `cfg(test)` child module `dioxus_document::bridge_tests`, under no feature gate — a Dioxus key holding `/` and an empty one each reading the positional `{tag}:{n}` segment beside a usable key's `{tag}[{key}]`, `create_head_element` appending one element under `<head>` with its attribute and text, `mounted` listeners firing after the build and after a poll, `Document::id` being the inner document's id, a handled event reporting cancel and stop as its listener left them
@carried n=4 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: A probe check kept in `tests/blitz-tests` must be a target file of its own for its behaviour, and the existing `mounted`-handler settle reading is the standing check nearest the hazard's path.
quote: holds Blitz's integration tests, one test-target file per behavior under `tests/`, and three shared modules that are no target
@carried n=4 run=2 label=test-plan/test-scope-summary fact=fct-z2ynupl3
quote: and on in-file fixtures a write made in a `mounted` handler is present after the step
@carried n=5 label=test-plan/test-scope-summary fact=fct-tz35iqmq
why: The probe's served task beside the lean one must be booted the way the served-page checks boot it, and a kept `stand_` file is picked up by `run stand` and the stand-repeat leg.
quote: `tests/blitz-tests` holds two checks on the pages the stand serves beside the lean four, each declaring `mod session_common;` and no `mod fixtures;`, each test in both layout modes, picked up by `run stand` and the stand-repeat leg by their prefix
@carried n=6 label=test-plan/test-scope-summary fact=fct-cyo4xvcu
why: The probe runs on the stand; any stand-side probe code kept must keep `the_stand_options_stay_offline` green and restate this count if a unit test is added.
quote: the stand module holds three more unit tests — `the_stand_options_stay_offline` (every boot but `boot_load` keeps `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight` — `cargo test -p seven_guis --locked --lib stand::` reading 5 passed (examples/seven_guis/src/stand.rs:283-329)
@carried n=7 label=test-plan/test-scope-summary fact=fct-k5y3m6cu
why: A probe check kept as a seven_guis integration target moves this count and must read the shared `tests/common/mod.rs`.
quote: `examples/seven_guis` holds eight integration-test targets (seven until 2026-10-19-session-recording added `tests/recording.rs`)
@carried n=8 label=test-plan/test-scope-summary fact=fct-rjsmqegm
why: The windowed shell's `poll` path has no test, so the reading must mark the windowed MountedData path as read and not run.
quote: **dioxus-native and stylo_taffy:** no tests — observed absent
@carried n=9 label=test-plan fact=fct-qaaszuyh
why: The tier sets how broad the probe's checks need to be.
quote: **Test tier:** `0`
@carried n=10 label=test-plan fact=fct-i47d76zr
why: The probe's headless reading of the hazard must use the document test's build-and-poll path, the same `poll(None)` the hazard was measured through; it must read the private handler through the `bridge_tests` child module and add no constructor, `pub` or shim.
quote: the document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:467-477)
@carried n=10 run=2 label=test-plan fact=fct-i47d76zr
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep, the one way to read `propagation_is_stopped()`, a flag nothing else in the workspace reads; no function under test gained a constructor, a `pub` or a `cfg(test)` shim
@carried n=11 label=test-plan fact=fct-vu43sciq
why: The probe's handler and state links (2) and (3) must be measured through input that reaches the VirtualDom, never through `dispatch_recorded`, which skips it.
quote: dispatch_recorded drives events against the underlying BaseDocument, bypassing document-specific handling such as Dioxus VirtualDom forwarding (packages/blitz-test-harness/src/harness.rs:174-212)
@carried n=12 label=test-plan fact=fct-ubyqdssx
why: The probe's stand runs must use no sleep and read no wall clock.
quote: no sleep and no wall-clock read in the checks, except the two whose subject is wall time
@carried n=13 label=test-plan fact=fct-kudmubnq
why: The probe must write nothing it reads (component names, handlers, signal values) to any stream or file.
quote: the session library and the harness path write nothing of the screen — typed text, a stable id, an accessible name — to stdout, to stderr or into the state directory
@carried n=14 label=test-plan fact=fct-ywhfajcc
why: The probe's stand runs must keep the pinned bundled font with no skip path.
quote: The headless-stand checks keep a real but pinned font — the bundled DejaVu Sans woff2, decoded through seven_guis' native `dioxus-native` `woff` feature, with system fonts off and no skip
@carried n=15 label=test-plan fact=fct-temy7g7i
why: A kept `stand_*` probe check enters the three-round, zero-failure, no-retry budget.
quote: the stand checks hold a zero-failure budget — the `stand-repeat` leg and its job `Stand checks repeated` run the `stand_*.rs` files, the seventeen named blitz-tests targets and seven_guis' process checks three times with no retry (§9 Legs)
@carried n=16 label=test-plan fact=fct-4t3fuckc
why: Any probe code kept inside `dioxus-native-dom` must hold the crate's coverage floor.
quote: the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %
@carried n=17 label=test-plan fact=fct-glytb3w3
why: These are the newest agent-run totals; a kept probe check must restate them, and the chunk must end with no failures in either selection.
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 · failed 0 · ignored 12
@carried n=18 label=test-plan fact=fct-he2hzpm5
why: A probe check's green must be read under the workspace build, not only `-p blitz-tests`.
quote: a green per-package run is not the workspace leg's green
@carried n=19 label=test-plan fact=fct-ovlcl3hy
why: The chunk must pass the local pre-push gate before it pushes.
quote: `bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden
@binding n=1
**Absent upstream tests ↔ the scope's MountedData fork**: the fix is the founder's fork, brought with its options and not decided. An option that takes upstream `286874cc1` brings back `task_polled_during_render_can_access_document` and `element_removed_before_its_mounted_event_is_flushed`, and it is an engine change for every document. The reading must name this test consequence beside the option; this chunk adds neither test (test-plan/test-scope-summary fct-e2zfwpza) (test-plan/test-scope-summary fct-ybmlkzsq)
@binding n=2
**Bridge test access ↔ the scope's no-fork-level-change boundary**: the probe reads the listener map and the handler through what the bridge already holds. A probe check of the private `DioxusEventHandler` sits in the `bridge_tests` child module and adds no `pub`, constructor or shim. A read that needs one is the verdict «exposed with a cost» or «not exposed» (test-plan fct-i47d76zr) (test-plan/test-scope-summary fct-i5a6vv3t)
@binding n=3
**Kept probe code ↔ the scope's "anything kept of it is the plan's choice"**: if a probe check is kept, it is its own target in `tests/blitz-tests` (or a seven_guis target), runs in both layout modes, and joins the stand-repeat budget when its file has the `stand_` prefix. The counts it moves are restated from their newest readings (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-glytb3w3) (test-plan fct-temy7g7i)
@binding n=4
**Nothing crosses ↔ obs-plan's masking rule**: what the probe reads never reaches stdout, stderr, a log or a field. This matches the standing rule that the harness path writes nothing of the screen (test-plan fct-kudmubnq)
