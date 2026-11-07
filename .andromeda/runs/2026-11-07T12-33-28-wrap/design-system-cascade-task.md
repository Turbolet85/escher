@task_prose part=session
The Dioxus 0.7 bridge in dioxus-native-dom now renders into a crate-private mutation buffer while the document is unborrowed, then replays it into the document through one MutationWriter, under one borrow, in recorded order.
This holds for every Dioxus document: the stand, the windowed dioxus-native launch and any app on the bridge. A component body or a task polled during render can call MountedData set_focus, get_client_rect, scroll_to, scroll, get_scroll_offset and get_scroll_size without the RefCell already borrowed panic. That panic is retired.
Limit: a task polled during render sees the document as the previous render left it, so a node created in the same render is not yet reachable.
The probe's two should_panic pins became two tests that assert the spawned set_focus acts. The engine focus, the snapshot's one focused node and the accessibility tree's focus agree, in both layout modes.
Measured: dioxus-native-dom has 83 unit tests (was 82) and 86.71 percent coverage. CI#42371051720 was green 17/17. The stand still reads 237 passed, 0 failed, 7 ignored. Upstream's ported test passes but does not detect the borrow bug on Dioxus 0.7.
No dependency, manifest, token, stylesheet, CSS default or UI element changed; Dioxus stays 0.7.3 and Cells keeps its synchronous scroll_to handles.
@task_file n=1 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=4 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs
@task_item n=1 part=touches
Any desktop-native stack or launch statement that describes how a Dioxus app opened through dioxus_native::launch renders into the document. Initial build and each poll now render through a buffer, then replay under one borrow. Public signatures are unchanged.
@task_item n=2 part=touches
Any statement about programmatic scroll or into-view scroll requested through MountedData (scroll_to, scroll) that assumes it is called synchronously from a handler. It may now also be called from a task polled during render. It acts on the previous render's nodes. The scroll behaviour itself is unchanged.
@task_item n=3 part=touches
Any surface or component-pattern note on focus moved by MountedData set_focus, for example a custom widget or a Dioxus app focusing an element. The call now succeeds from a task a key handler spawned. It returns false for a node created in the same render.
@task_item n=4 part=touches
Any recorded hazard, caveat or should_panic pin about MountedData calls panicking during render, on any surface that runs on the Dioxus bridge. The hazard is closed, and the pins now assert that the focus acts.
@task_item n=5 part=touches
Indirect: the mobile-native and web-spa surfaces run the browser app and dioxus-native on the same bridge. Any line there about the Dioxus render path or MountedData limits is reached through the bridge change.
@task_item n=6 part=touches
Any citation of dioxus-native-dom line numbers in dioxus_document.rs (field near 90, constructor 155, initial_build 166-171, poll 302-307, tests after 846) may have moved. So may a statement of that crate's module list, which now holds the private mutation_buffer module beside keyboard_compat.
@task_item n=7 part=touches
Any test inventory or count that cites dioxus-native-dom unit tests (82 to 83) or the dioxus_runtime_probe test names. The probe still holds 10 tests.
@task_fact n=1 slug=surface
fact: On a Dioxus app (desktop-native dioxus_native::launch and every other dioxus-native surface), user code that runs during a render, such as a component body or a task it polls, can call MountedData set_focus, scroll_to, scroll and the rect and scroll queries without panicking. The call acts on the document as the previous render left it, so a node created in that same render is not yet reachable and set_focus_to returns false for it.
@task_item n=1 part=untouched
Colour palette, semantic colours, border progression and the open select list's engine colour constants: no colour, token or stylesheet changed.
@task_item n=2 part=untouched
Typography, font loading, system-fonts, the fonts-unavailable screenshot refusal and engine text mapping: the session touched no text measuring or font path.
@task_item n=3 part=untouched
Spacing values and the absence of a spacing scale: no layout or CSS value moved.
@task_item n=4 part=untouched
Depth strategy and hit-testing at the paint-clipping padding box: hit-testing and paint are unchanged; only the order in which mutations reach the document changed, and their content did not.
@task_item n=5 part=untouched
Border radius values and their absence records: untouched.
@task_item n=6 part=untouched
The animation and settle runtime: harness settle, the VirtualClock, the driver's advance and its load wait. No harness, gate or clock changed. The nested-box into-view scroll rule is unchanged.
@task_item n=7 part=untouched
Iconography, cursors and favicons, and the browser toolbar icon assets: no icon or cursor rule changed.
@task_item n=8 part=untouched
The cli surface: escher-session's JSON-only stdout, its exit classes, owo-colors in the WPT runner. Driver output did not change.
@task_item n=9 part=untouched
Platform and packaging lines (publish workflow bundles, CI ios and android build-only targets, flake package): no workflow or manifest moved.
