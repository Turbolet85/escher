@carried n=1 label=architecture/design-philosophy fact=fct-do226dcu
why: Both render sites of `DioxusDocument`, `poll` among them, must render through the buffer and still flush the vdom's changes into the `BaseDocument` it integrates.
quote: `DioxusDocument` integrates `BaseDocument` with `VirtualDom`; UI events are pushed with `handle_ui_event` and changes flushed with `poll` (packages/dioxus-native-dom/src/dioxus_document.rs:57; packages/dioxus-native-dom/src/dioxus_document.rs:83-84)
@carried n=2 label=architecture/design-philosophy fact=fct-rxrxih7j
why: The buffered render must keep incremental and non-incremental layout identical and dropped ids non-resolving, which incremental_oracle, stale_node_mapping and the inverted pins in both layout modes re-assert.
quote: Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2624-2641; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
@carried n=2 run=2 label=architecture fact=fct-57klyykw
quote: both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot
@carried n=3 label=architecture/design-philosophy fact=fct-dpohxdsn
why: The inverted pins must reach their precondition (the key target focused by Tab) and the key-handler-spawned task through synthesized input on the headless harness's real dispatch, with no window.
quote: The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19)
@carried n=3 run=2 label=architecture fact=fct-wromimpb
quote: the engine is exercised without a window through a headless harness
@carried n=4 label=architecture/stack-and-technologies fact=fct-mcgc5v2e
why: The ported test and the probe's key-driven pins must name the workspace's keyboard-types as `::keyboard_types` where they glob-import the Dioxus prelude, and upstream's keyboard-types 0.8 move must not be ported.
quote: keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
@carried n=4 run=2 label=architecture fact=fct-j2ltw335
quote: Two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7, so a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
@carried n=5 label=architecture/stack-and-technologies fact=fct-mcwsssza
why: The buffer must be written against dioxus-core 0.7's `WriteMutations` and the Dioxus requirement must stay "0.7.3", none of upstream's 0.8-only parts ported.
quote: Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
@carried n=5 run=2 label=architecture fact=fct-d4plx3kh
quote: winit, AccessKit, Boa, Dioxus 0.7, tokio and reqwest
@carried n=6 label=architecture/stack-and-technologies fact=fct-ovms5x7l
why: The new buffer module and the reshaped render sites must keep dioxus-native-dom at or above its 83 % line floor, and the stand checks the chunk names its witness must hold the stand-repeat leg's zero-failure budget.
quote: a line-coverage report, `cargo llvm-cov`, gated since 2026-10-14-quality-gates by per-crate line floors through `.github/scripts/coverage_floor.py` — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, read 98.34 · 84.25 · 91.69 % at that chunk — the `audit` and `coverage` legs (.github/scripts/ci-leg.sh:40-49; as measured at escher-0.1.0/chunks/2026-10-14-quality-gates/evidence/coverage-floor.md);
@carried n=6 run=2 label=architecture/stack-and-technologies fact=fct-ovms5x7l
quote: the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:50-69)
@carried n=7 label=architecture fact=fct-54e4fuk7
why: The windowed dioxus-native launch reaches `DioxusDocument::poll` after every window event and every waker wake, so the buffered render must hold on that path too, read and not run.
quote: every window event is followed by a Poll (packages/blitz-shell/src/application.rs:162-165); a futures waker sends Poll events (packages/blitz-shell/src/event.rs:105-126)
@carried n=7 run=2 label=architecture fact=fct-aj6cmiwc
quote: dioxus-native creates the event loop and runs `run_app` (packages/dioxus-native/src/lib.rs:152-154; packages/dioxus-native/src/lib.rs:244-245)
@carried n=7 run=3 label=architecture fact=fct-aj6cmiwc
quote: `DioxusNativeApplication` delegates to an inner `BlitzApplication` (packages/dioxus-native/src/dioxus_application.rs:114-213)
@binding n=1
**stack-and-technologies (Dioxus "0.7.3") ↔ design-philosophy (`DioxusDocument` flushes through `poll`)**: the buffer records dioxus-core 0.7's `WriteMutations` calls at both render sites, `initial_build` and `poll`, and replays them into the existing `MutationWriter` with no Dioxus version moved (architecture/stack-and-technologies fct-mcwsssza) (architecture/design-philosophy fct-do226dcu).
@binding n=2
**design-philosophy (one incremental pipeline) ↔ design-philosophy (headless, testable)**: the inverted pins and the re-read witnesses run headless in both layout modes, and the replayed buffer must leave the two modes' layouts identical and a removed node's id unresolving (architecture/design-philosophy fct-rxrxih7j) (architecture/design-philosophy fct-dpohxdsn).
@binding n=3
**stack-and-technologies (coverage floor, stand-repeat) ↔ the plan (new buffer module, ported test)**: the buffer module's lines count toward dioxus-native-dom's 83 % floor, which the ported test and the inverted pins must keep met, and the stand checks re-run as witnesses stay at zero failures over three rounds (architecture/stack-and-technologies fct-ovms5x7l).
@binding n=4
**stack-and-technologies (two keyboard-types) ↔ stack-and-technologies (Dioxus 0.7.3)**: Dioxus 0.7.3 carrying keyboard-types 0.7 is why the workspace's 0.8 crate is named `::keyboard_types` in the tests, and why upstream's keyboard-types 0.8 move and `keyboard_compat.rs` removal stay unported (architecture/stack-and-technologies fct-mcgc5v2e) (architecture/stack-and-technologies fct-mcwsssza).
@binding n=5
**the body (shell loop, launch paths) ↔ design-philosophy (`poll`)**: the windowed dioxus-native launch drives `DioxusDocument::poll` after each window event and waker, so it takes the buffered render as an engine change for every Dioxus document, its path read and not run (architecture fct-54e4fuk7) (architecture/design-philosophy fct-do226dcu).
