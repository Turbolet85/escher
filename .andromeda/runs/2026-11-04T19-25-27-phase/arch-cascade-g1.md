@carried n=1 label=architecture/design-philosophy fact=fct-mntz5vy6
why: the «exposed with a cost» verdict must name its cost and boundary as an escher-side change inside this bridge crate, never a Dioxus fork or an engine change for every document.
quote: `dioxus-native-dom` is the "Core headless native renderer for Dioxus based on blitz" (packages/dioxus-native-dom/Cargo.toml:6; packages/dioxus-native-dom/src/lib.rs:3)
@carried n=1 run=2 label=architecture fact=fct-p43yq5ww
quote: the engine is a set of separately usable crates around a headless DOM that external code drives and any renderer can paint
@carried n=2 label=architecture/design-philosophy fact=fct-do226dcu
why: links (1) to (4) must be read through the bridge's own entry points, and the hazard's mechanism must be re-verified at HEAD in the `poll` that flushes changes.
quote: `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:57; packages/dioxus-native-dom/src/dioxus_document.rs:83-84)
@carried n=3 label=architecture/design-philosophy fact=fct-rxrxih7j
why: a verdict whose reading layout touches must be run in both layout modes, and its reading must agree across them.
quote: Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2624-2641; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
@carried n=3 run=2 label=architecture fact=fct-57klyykw
quote: both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`
@carried n=4 label=architecture/design-philosophy fact=fct-dpohxdsn
why: each verdict must rest on a headless run whose synthesized input reaches the listeners through the real event-dispatch pipeline, never on reading the code alone.
quote: The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19); integration tests live in the `tests/` directory of the blitz-tests crate (tests/blitz-tests/lib.rs:1)
@carried n=4 run=2 label=architecture fact=fct-wromimpb
quote: the engine is exercised without a window through a headless harness
@carried n=5 label=architecture/stack-and-technologies fact=fct-mcgc5v2e
why: probe code that drives the hazard's key handler from a module glob-importing `dioxus::prelude::*` must name the workspace's keyboard-types as `::keyboard_types`.
quote: keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
@carried n=5 run=2 label=architecture fact=fct-j2ltw335
quote: Two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7, so a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
@carried n=6 label=architecture/stack-and-technologies fact=fct-mcwsssza
why: the probe must read these runtime crates only through their public or bridge-held APIs, naming each in its verdict, with Dioxus held at 0.7.3 and no crate patched, vendored or re-pinned.
quote: Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
@carried n=6 run=2 label=architecture fact=fct-d4plx3kh
quote: Boa, Dioxus 0.7, tokio and reqwest
@carried n=7 label=architecture/stack-and-technologies fact=fct-du6rwwjz
why: a probe check in blitz-tests must reach the lean stand task and the served task through the stand and driver dev-dependencies already listed, adding no workspace crate.
quote: blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23), anyrender, anyrender_vello_cpu, dioxus, test-that, usvg, tracing, tracing-log
@carried n=8 label=architecture/stack-and-technologies fact=fct-ovms5x7l
why: any stand check the probe keeps must hold the zero-failure budget over three rounds, so a measured verdict never rests on a flaky run.
quote: the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:50-69)
@carried n=8 run=2 label=architecture fact=fct-e4papwn6
quote: stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no job-level permission; .github/workflows/ci.yml:300-329)
@carried n=9 label=architecture fact=fct-54e4fuk7
why: the reading must state that the windowed shell runs the same `poll` after every window event, so the hazard is expected there, as read from the code and not run unless a reading needs it.
quote: every window event is followed by a Poll (packages/blitz-shell/src/application.rs:162-165); a futures waker sends Poll events (packages/blitz-shell/src/event.rs:105-126)
@carried n=9 run=2 label=architecture fact=fct-aj6cmiwc
quote: dioxus-native creates the event loop and runs `run_app` (packages/dioxus-native/src/lib.rs:152-154; packages/dioxus-native/src/lib.rs:244-245)
@binding n=1
**design-philosophy ↔ stack-and-technologies**: each link's verdict must be read through the bridge's `handle_ui_event` / `poll` surface (architecture/design-philosophy fct-do226dcu) and the public APIs of the Dioxus 0.7.3 crates (architecture/stack-and-technologies fct-mcwsssza); where neither exposes a link, the cost must be bounded inside `dioxus-native-dom` (architecture/design-philosophy fct-mntz5vy6) or the verdict must read «not exposed without a fork-level change», never a Dioxus pin move.
@binding n=2
**design-philosophy ↔ stack-and-technologies**: the headless harness's real event-dispatch pipeline (architecture/design-philosophy fct-dpohxdsn) must be driven from blitz-tests through its existing seven_guis and escher-driver dev-dependencies (architecture/stack-and-technologies fct-du6rwwjz), and any kept stand check must pass the `stand-repeat` zero-failure budget (architecture/stack-and-technologies fct-ovms5x7l).
@binding n=3
**design-philosophy ↔ the scope**: a verdict whose reading layout touches must be measured in both layout modes and must agree across them (architecture/design-philosophy fct-rxrxih7j), the probe adding no `NodeId` to anything that crosses (architecture fct-57klyykw).
@binding n=4
**Infrastructure Patterns ↔ design-philosophy**: the hazard's re-verified mechanism sits in `DioxusDocument::poll` (architecture/design-philosophy fct-do226dcu), and the windowed shell's Poll after every window event (architecture fct-54e4fuk7) must be stated in the reading as the same path, read and not run.
@binding n=5
**stack-and-technologies ↔ the hazard**: a probe that re-measures the key-handler hazard must name the workspace's `::keyboard_types` beside Dioxus's own 0.7 (architecture/stack-and-technologies fct-mcgc5v2e), Dioxus staying 0.7.3 (architecture/stack-and-technologies fct-mcwsssza).
