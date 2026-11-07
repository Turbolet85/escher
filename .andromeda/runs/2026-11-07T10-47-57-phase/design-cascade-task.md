@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge: DioxusDocument's two render sites (initial_build's vdom.rebuild, poll's vdom.render_immediate) record the vdom's WriteMutations calls into a buffer with the document unborrowed, then take the document's borrow once, replay the buffer through the existing MutationWriter and flush the queued mounted events as before.
User code the vdom runs during render (a component body, a task it polls) can then call MountedData (set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset, get_scroll_size) with no RefCell already borrowed panic; an engine change for every Dioxus document, the stand, the windowed dioxus-native launch and any app on the bridge.
Ports upstream's regression test task_polled_during_render_can_access_document as the witness, and inverts dioxus_runtime_probe's two should_panic hazard pins into tests that the key-handler-spawned task's set_focus moves focus without panic in both layout modes, their precondition still asserted first.
Re-asserts behaviour otherwise unchanged (document built, element ids, snapshot and diff, mounted event order, incremental/non-incremental layout identity) through the existing stand_*, stale_node_mapping, incremental_oracle and the probe's other eight tests; Dioxus stays at 0.7, nothing crosses any boundary.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: initial_build and poll render sites, flush_queued_mounted_events' two callers, mod tests (the ported test)
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it; MutationWriter otherwise kept byte-identical to upstream 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic pins (one per layout mode) inverted, the module text
@task_item n=1 part=needs
Focus as the engine holds it: the ported test and the inverted pins assert that set_focus(true) from a render-polled task moves focus to the named input, the key target first focused by Tab.
@task_item n=2 part=needs
Element geometry read during render: get_client_rect on a held MountedData must answer from the document (box of the node) without panic; the ported test calls it.
@task_item n=3 part=needs
Programmatic scroll reached through MountedData (scroll_to, scroll, get_scroll_offset, get_scroll_size): the rule that a programmatic into-view scroll writes every nested scrolling box at once, behaviour applying to the viewport alone, must stay as it is once these calls are reachable during render.
@task_item n=4 part=needs
The desktop-native surface: Dioxus apps open through dioxus_native::launch into a winit window; that windowed launch renders through the new buffer, its behaviour otherwise unchanged.
@task_item n=5 part=needs
The web-spa surface: on wasm32 dioxus-native renders into a canvas on the page; as a Dioxus document it also renders through the buffer, unchanged in what it paints.
@task_item n=6 part=needs
The browser app on desktop-native and mobile-native runs on the Dioxus bridge (any app on the bridge): its documents, element ids and mounted events must come out identical through the buffered render.
@task_item n=7 part=needs
The two layout modes (incremental and non-incremental) the inverted pins run in, and their layout identity, which the change must keep.
@task_item n=8 part=needs
The test harness's VirtualClock and settle as the probe's tests drive them: the inverted pins and the ported test run through the same harness and must not advance the clock or wait on animation beyond what the harness already does.
@task_item n=9 part=needs
Text measurement in the blitz-tests crate: get_client_rect and focus assertions over text inputs depend on blitz-dom's system-fonts feature, without which text measures 0x0 and geometry assertions pass vacuously.
@task_item n=10 part=needs
The buffer's content (ids, tag and attribute names, text) is in process only, never printed or logged, so no driver output, CLI line or log field changes.
@task_standing n=1
convention: a test in the blitz-tests crate builds blitz-dom with the system-fonts feature, so text measures and font-dependent geometry assertions do not pass vacuously
home: Typography section of the body (Loading)
@task_standing n=2
convention: a harness-driven test reads animation time only through the VirtualClock its documents read; settle never advances it, only advance moves it
home: Motion section of the body (Animation runtime)
@task_item n=1 part=not_needed
The color palette (core colors, surface scale, text hierarchy, semantic colors, border progression, token sets): the chunk changes how mutations are applied, not any colour.
@task_item n=2 part=not_needed
The open select list's engine colour constants on SelectListView (highlight text and background, disabled text, border): no select list is touched.
@task_item n=3 part=not_needed
Typography's type scale and engine text mapping, and the screenshot refusal fonts-unavailable: no font, text style or screenshot path changes.
@task_item n=4 part=not_needed
Spacing values and the absent spacing scale: no spacing is set or read.
@task_item n=5 part=not_needed
Border radius handling and its absent scale: untouched.
@task_item n=6 part=not_needed
Depth strategy's elevation values and absent elevation scale: no shadows or stacking change.
@task_item n=7 part=not_needed
Iconography: the browser toolbar SVG icons, IconButton, tab glyphs, icon size grid and favicons are not touched.
@task_item n=8 part=not_needed
Cursor selection (CSS cursor keyword, Text, Pointer, Default): no pointer or cursor logic changes.
@task_item n=9 part=not_needed
Mobile-native's component patterns (the urlbar, the back button, capture): the chunk changes no browser UI component.
@task_item n=10 part=not_needed
Brand identity, anti-patterns, self-validation protocol and the design decisions log: no recorded intent, nothing for an engine render-path change to act on.
