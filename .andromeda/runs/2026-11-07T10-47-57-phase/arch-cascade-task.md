@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: DioxusDocument's two render sites, initial_build (vdom.rebuild) and poll (vdom.render_immediate), run the vdom into a buffer of mutations while the document is not borrowed, then borrow the document once, replay the buffer through the existing MutationWriter, and flush the queued mounted events as before.
The point is that user code the vdom runs during render (a component body, a task it polls) can call MountedData (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) with no RefCell already borrowed panic. This is an engine change for every Dioxus document: the stand, the windowed dioxus-native launch and any app on the bridge.
Ports upstream's regression test task_polled_during_render_can_access_document into dioxus_document.rs's mod tests and makes it pass. It also inverts the probe's two should_panic hazard pins into tests that set_focus moves focus without a panic, in both layout modes, with their precondition still asserted first.
Re-verifies that behaviour is otherwise unchanged (the document built, element ids, snapshot and diff, the mounted events' order, incremental and non-incremental layout identity) through the existing stand_*, stale_node_mapping, incremental_oracle and the probe's other eight tests. Dioxus stays at 0.7, and none of upstream's 0.8-only parts are ported.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the two render sites, initial_build and poll, and mod tests (the ported test task_polled_during_render_can_access_document)
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs mark=edits
what: the new buffer module that records 0.7's WriteMutations calls and replays them into MutationWriter
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it as upstream's does; otherwise the file stays byte-identical to upstream at 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic RefCell already borrowed pins (one per layout mode), inverted, and the file's module text
@task_file n=6 path=tests/blitz-tests/tests/stale_node_mapping.rs mark=re-reads
what: stale_node_mapping, re-run as a witness that the bridge's node mapping is unchanged
@task_file n=7 path=tests/blitz-tests/tests/incremental_oracle.rs mark=re-reads
what: incremental_oracle, re-run as a witness of incremental and non-incremental layout identity
@task_item n=1 part=needs
The Dioxus DOM bridge (packages/dioxus-native-dom): DioxusDocument, MutationWriter, DioxusState and the queued mounted events. The render path changes and must keep building the same document, element ids and mounted-event order.
@task_item n=2 part=needs
DioxusDocument::element_id and element_ids (the four-tier stable ids), plus the accessibility snapshot, its text form and its diff. These must read the same after a buffered render, and the stand_* checks witness it.
@task_item n=3 part=needs
A node the Dioxus bridge removes is dropped with its descendants and no longer resolves. Interaction state on removed nodes is cleared or retargeted. The replayed buffer must keep both.
@task_item n=4 part=needs
Mutations go through DocumentMutator, which flushes deferred work on Drop and requests a redraw only for in-document mutations. Replaying the buffer under one borrow of the document must keep that flush and redraw behaviour.
@task_item n=5 part=needs
Focus: MountedData set_focus reaches the engine's focus change (blur, focusout, focus, focusin) and the focus rules. The inverted pins and the ported test assert that focus moves to the named element.
@task_item n=6 part=needs
The Dioxus version pin: Dioxus stays at "0.7.3" with dioxus-core 0.7.10 in the lock. No 0.8 move, no patched or vendored Dioxus crate and no pin bump. The buffer is written against dioxus-core 0.7's WriteMutations trait and VirtualDom::rebuild / render_immediate.
@task_item n=7 part=needs
Two keyboard-types versions stand in the graph. A module that glob-imports dioxus::prelude::* names the workspace's crate as ::keyboard_types. Upstream's keyboard-types 0.8 move and keyboard_compat.rs removal are not ported.
@task_item n=8 part=needs
The upstream-sync record: upstream 286874cc1 is read by git show in this checkout and ported in part. mutation_writer.rs stays byte-identical to upstream at 434e0661 where the port allows, and any change there is named with its reason.
@task_item n=9 part=needs
Scenarios run in both layout modes, incremental and non-incremental, and must produce identical layouts. The inverted pins hold in both modes.
@task_item n=10 part=needs
Nothing crosses: no verb, argument, node key, result field, cause, session error, exit, carrier, listener, port, env var, workspace crate, log field or span is added. The buffer holds ids, tag and attribute names and text in process only, and never prints or logs them.
@task_item n=11 part=needs
Tracing in the engine and upstream crates is gated per call site with a cfg(not(feature = "tracing")) fallback. Any trace the buffer module keeps follows this rule and carries no mutation content.
@task_item n=12 part=needs
Engine changes for every document are recorded as such, flagged upstreamable or not, with their merge surface. The buffered render is ratified as the founder's choice of fix and is an engine change for every Dioxus document.
@task_item n=13 part=needs
Borrowing: the hazard is DioxusDocument::poll holding inner.borrow_mut() across vdom.render_immediate, while a polled task's set_focus borrows through doc_mut. Shared state is Rc<RefCell<…>>, and the fix takes the document's borrow only after the vdom returns.
@task_item n=14 part=needs
The windowed dioxus-native launch and the browser and about pages built on Dioxus also render through DioxusDocument, so they take the buffered render. The windowed path is read, not run.
@task_standing n=1
convention: Unit tests sit in a #[cfg(test)] mod tests at the end of the source file, or in a #[path] child file of their own.
home: conventions
@task_standing n=2
convention: Each integration test file covers one behavior and opens with a //! doc naming it. Scenarios run in both layout modes. Tests first assert that the fixture produces the condition. blitz-tests keeps one test target per file.
home: conventions
@task_standing n=3
convention: Modules and files open with //! docs, public items carry ///, and doc comments state the rationale for constants. This applies to the new mutation_buffer.rs.
home: conventions
@task_standing n=4
convention: Formatting and lints: cargo fmt --all --check, cargo clippy --workspace --locked -- -D warnings, and rustdoc warnings as errors, each run as a .github/scripts/ci-leg.sh leg. clippy::expect_used and clippy::unwrap_used each sit next to a justification.
home: conventions
@task_standing n=5
convention: Crate internals are pub(crate) or pub(super). The new buffer type stays crate-private unless the bridge's public API needs it.
home: conventions
@task_standing n=6
convention: unsafe blocks carry allow unsafe_code or a // SAFETY: comment. Borrow conflicts are worked around by taking a value out and restoring it.
home: conventions
@task_item n=1 part=not_needed
The WPT runner, its outputs, bucketing and execution. The chunk changes only the Dioxus bridge, and WPT documents are not Dioxus documents.
@task_item n=2 part=not_needed
The browser app's history store, SQLite persistence, favicon cache and URL suggester. None of these is touched by the render path.
@task_item n=3 part=not_needed
JavaScript on Boa (blitz-vibey-script), its timers, module loader and bootstrap. A Dioxus document runs no page script.
@task_item n=4 part=not_needed
blitz-net networking, its User-Agent, per-host semaphore and cache, and resource loading of stylesheets, images and fonts.
@task_item n=5 part=not_needed
Painting and renderer backends: blitz-paint, the password mask, layer caps and the vello and skia selection.
@task_item n=6 part=not_needed
The select model and the open select list: apply_select_choice, typeahead, reconcile_select_list and paint_select_list.
@task_item n=7 part=not_needed
CSS approximations, unsupported features, inline SVG and the fonts decisions.
@task_item n=8 part=not_needed
Deployment and packaging: publish-browser bundles, the Nix flake, Cross.toml and cross-compilation, signed builds and the WPT CI environments.
@task_item n=9 part=not_needed
Release versioning, build profiles and the MSRV build-only job. No version, profile or rust-version moves.
@task_item n=10 part=not_needed
The accesskit_xplat adapter and blitz-shell's windowing, IME and keyboard shortcuts.
@task_item n=11 part=not_needed
HTML parsing through html5ever and xml5ever, and the HTML fixtures and fixture forms under examples/assets.
@task_item n=12 part=not_needed
The project intent's UI graph capability. The UI graph is not built here.
