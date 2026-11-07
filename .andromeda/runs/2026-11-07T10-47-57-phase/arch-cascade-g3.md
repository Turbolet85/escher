@carried n=1 label=architecture/conv-known-gaps fact=fct-uzp72hei
why: the buffer must replay 0.7's WriteMutations calls into MutationWriter in their recorded order and the ordering-sensitive block of mutation_writer.rs must stay as it stands, marker included, byte-identical to upstream at 434e0661 save a named DioxusState change.
quote: an ordering-sensitive block is marked "WARNING: DO NOT REORDER" (packages/dioxus-native-dom/src/mutation_writer.rs:203-207)
@carried n=1 run=2 label=architecture fact=fct-vi4hnram
quote: an ordering-sensitive block is marked "WARNING: DO NOT REORDER"
@carried n=2 label=architecture/conv-feature-gating fact=fct-bhxvo7z2
why: any trace the new mutation_buffer.rs keeps must be gated per call site with the not-tracing fallback, since dioxus-native-dom is an upstream crate.
quote: in the engine and upstream crates tracing is gated per call site with `#[cfg(feature = "tracing")]`, with a `#[cfg(not(feature = "tracing"))] let _ = …;` fallback
@carried n=2 run=2 label=architecture fact=fct-6yr3qnpo
quote: in the engine and upstream crates tracing is gated per call site with a `cfg(not(feature = "tracing"))` fallback
@carried n=3 label=architecture/conv-feature-gating fact=fct-bhxvo7z2
why: a buffer trace through the crate-local trace! macro must not rely on its 4-argument arm carrying more than two items, and the macro in the edited lib.rs must stay as it stands while the module list changes.
quote: `dioxus-native-dom` defines a crate-local `trace!` macro expanding to `tracing::debug!` under `tracing` (packages/dioxus-native-dom/src/lib.rs:56-79), whose 4-argument arm passes only the first two items (packages/dioxus-native-dom/src/lib.rs:70-73)
@carried n=4 label=architecture/conv-tests fact=fct-zd5gb466
why: the ported task_polled_during_render_can_access_document must take its home among dioxus-native-dom's bridge tests, whose child module at the end of dioxus_document.rs reaches the document's private fields with no shim, against upstream's mod tests placement the scope names.
quote: dioxus-native-dom's bridge tests, declared at the end of `dioxus_document.rs` as `#[cfg(test)] #[path = "dioxus_document_tests.rs"] mod bridge_tests;`, a child of the module under test so they reach its private fields with no constructor, `pub` or `cfg(test)` shim added (packages/dioxus-native-dom/src/dioxus_document.rs:838-840)
@carried n=4 run=2 label=architecture fact=fct-wqndq65v
quote: unit tests in `#[cfg(test)] mod tests` at the end of the source file, or in a `#[path]` child file of their own
@carried n=5 label=architecture/conv-tests fact=fct-zd5gb466
why: dioxus_runtime_probe.rs's module text must be re-said to name the behaviour it now holds — set_focus moves focus with no panic — and the ported regression test must cite upstream's commit.
quote: each integration test file covers one behavior and opens with a `//!` doc naming the behavior and any guarded bug (tests/blitz-tests/tests/anonymous_block_leak.rs:1-7; tests/blitz-tests/tests/comment_layout.rs:1-7); regression tests cite upstream issue or PR URLs
@carried n=6 label=architecture/conv-tests fact=fct-zd5gb466
why: the two inverted pins must hold in both layout modes, one per mode as the pins stand.
quote: scenarios run in both layout modes `for incremental in [false, true]`
@carried n=7 label=architecture/conv-tests fact=fct-zd5gb466
why: each inverted pin must first assert its precondition — the MountedData handle held and the key target focused by Tab — before asserting focus moved.
quote: tests first assert the fixture produces the condition under test
@carried n=7 run=2 label=architecture fact=fct-wqndq65v
quote: tests first assert the fixture produces the condition
@binding n=1
**conventions (known gaps) ↔ upstream sync / the plan's buffer**: the replay into MutationWriter must keep the recorded call order the ordering-sensitive block depends on, and mutation_writer.rs must stay byte-identical to upstream at 434e0661 but for a named DioxusState change, so the next sync's merge surface stays as it is (architecture/conv-known-gaps fct-uzp72hei) (architecture fct-7lmrueug)
@binding n=2
**conventions (feature gating) ↔ boundaries (nothing crosses)**: any trace in the buffer module must be gated per call site with the not-tracing fallback and must carry no ids, tag or attribute names or text, because the buffer's content stays in process, never logged (architecture/conv-feature-gating fct-bhxvo7z2) (architecture fct-tev7ieys)
@binding n=3
**conventions (tests) ↔ contracts (Dioxus DOM bridge)**: the ported regression test must live with the bridge tests in dioxus_document.rs's child module and assert that the focused node is the input it named; the existing bridge tests (element ids, snapshot) must still pass after the buffered render (architecture/conv-tests fct-zd5gb466) (architecture fct-srb7slak)
@binding n=4
**conventions (tests) ↔ decisions (one incremental pipeline)**: the inverted pins must hold in both layout modes, precondition first, so that incremental and non-incremental runs stay identical under the buffered render (architecture/conv-tests fct-zd5gb466) (architecture fct-57klyykw)
