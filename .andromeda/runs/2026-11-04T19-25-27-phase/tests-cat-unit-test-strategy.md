@carried n=1 label=test-plan fact=fct-tkvntwtm
why: any probe unit test the chunk adds to a crate must sit in an inline `#[cfg(test)]` module, or in a `#[path]` file declared `#[cfg(test)]` at its parent with its test paths unchanged.
quote: unit tests sit in `#[cfg(test)] mod tests` blocks inside the source file; unit-test modules moved out of line sit in files of their own, each declared `#[cfg(test)]` with `#[path]` at its parent, their test paths unchanged; `dioxus_document::bridge_tests` is a child of `dioxus_document` because `DioxusEventHandler`'s two fields are private to that module and it has no constructor
@carried n=2 label=test-plan/unit-test-strategy fact=fct-dssptnyw
why: a probe unit test of link (2) or (3) that reads `DioxusEventHandler` (the listener map, a handler and the state it writes) must live in the existing `bridge_tests` child of `dioxus_document`, in `dioxus_document_tests.rs`, and not in a new module outside it.
quote: `dioxus_document::bridge_tests` in `dioxus_document_tests.rs`, declared `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;` at the end of its parent, a child of `dioxus_document` because `DioxusEventHandler`'s two fields are private to that module and it has no constructor (packages/dioxus-native-dom/src/dioxus_document.rs:838-840; packages/dioxus-native-dom/src/dioxus_document_tests.rs:1-6)
@carried n=3 label=test-plan fact=fct-i47d76zr
why: a probe unit test in dioxus-native-dom must build its document the crate's way (`DocumentConfig::default()`, `initial_build`, `mark_dirty`, `poll(None)`, a `GlobalSignal` flipped inside `vdom.in_runtime`). This is the same `poll(None)` path that the MountedData mechanism is re-verified through.
quote: the document test builds a `DioxusDocument` with `DocumentConfig::default()`, calls `initial_build`, and drives updates with `mark_dirty` and `poll(None)` (packages/dioxus-native-dom/src/dioxus_document.rs:467-477); the `element_id` unit tests build the same way from a plain `fn() -> Element` app, flip a `GlobalSignal` inside `vdom.in_runtime` to re-render
@carried n=3 run=2 label=test-plan fact=fct-i47d76zr
quote: the bridge tests sit in a `cfg(test)` child module of `dioxus_document` (`bridge_tests`, file `dioxus_document_tests.rs`) because `DioxusEventHandler`'s two fields are private to that module and it has no constructor — they build one over `doc.vdom` and `doc.vdom_state` and call `handle_event` with an `EventState` they keep, the one way to read `propagation_is_stopped()`, a flag nothing else in the workspace reads; no function under test gained a constructor, a `pub` or a `cfg(test)` shim
@carried n=4 label=test-plan/unit-test-strategy fact=fct-l2ouwaqy
why: a probe check placed in `tests/blitz-tests` must be a new file under `tests/` that auto-discovery picks up, with no `[[test]]` table added and `autotests` not turned off, so that `BlitzTestsTargetsTest` stays green.
quote: `BlitzTestsTargetsTest`, one test, reads `tests/blitz-tests/Cargo.toml` with the stdlib `tomllib` and asserts that `[package]` does not set `autotests` to false and that the manifest holds exactly one `[[test]]` table, `all` at `tests/all.rs` with `test = false`: the standing check on the fork's two-line difference from upstream's manifest
@carried n=5 label=test-plan fact=fct-mnyw2bw5
why: a probe assertion must follow the plan's rule that a failing log prints no value. Its message must carry no component name, handler or signal value it reads, only fixed words, rows or indices.
quote: command.rs's assertions over a call carry a row index only, so a failing log prints no argument value
@carried n=6 label=test-plan fact=fct-4t3fuckc
why: any probe code kept in dioxus-native-dom's source must keep the crate at its 83 % line floor. A probe test held in a `#[path]` child file adds nothing to the crate's reading.
quote: `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 %; both `cargo llvm-cov` lines exclude dioxus-native-dom's `keyboard_compat.rs`, upstream's temporary keyboard-types bridge, until escher takes upstream's retirement of it; a `#[path]` test-child file is no record of the per-crate reading
@binding n=1
**bridge test placement ↔ the scope's surfaces and §3 Crate-local test helpers**: the scope's bridge surface (`DioxusDocument`, its listener reads, `dioxus_document.rs`'s `poll`, `events.rs`) is tested in unit tests only through the `bridge_tests` child. §4 sets the module's place (test-plan/unit-test-strategy fct-dssptnyw) (test-plan fct-tkvntwtm), and §3 sets how it builds the handler and the document (test-plan fct-i47d76zr).
@binding n=2
**no constructor, `pub` or `cfg(test)` shim ↔ the scope's verdict classes**: an «exposed» verdict must be measured through the API as it stands, with no constructor, `pub` or shim added to make it readable. A reading that needs one is an «exposed with a cost» verdict that names that change, and the probe's tests do not land it (test-plan fct-i47d76zr).
@binding n=3
**assertion messages ↔ the scope's «nothing crosses» boundary**: the scope says nothing a probe reads is printed, logged or put in a field. §4's row-index-only assertion rule is how a probe check keeps that rule even when it fails (test-plan fct-mnyw2bw5).
@binding n=4
**blitz-tests target ↔ the CI scripts leg**: a probe target in `tests/blitz-tests` is discovered by file and must leave the manifest's one `all` table untouched. `BlitzTestsTargetsTest` runs in the `ci-scripts` leg (test-plan/unit-test-strategy fct-l2ouwaqy).
@binding n=5
**kept probe code ↔ §10 Quality Gates**: whatever the plan keeps of the probe in `packages/dioxus-native-dom/src` is counted against that crate's floor in the `coverage` leg (test-plan fct-4t3fuckc).
