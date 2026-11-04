@task_prose part=chunk
Builds a probe of the four links of the UI graph (node to component, node to handlers, handler to state, state to readers) through what Dioxus 0.7.3 and the escher bridge dioxus-native-dom already expose, with no fork-level change.
Each link gets a verdict (exposed, exposed with a cost, not exposed without a fork-level change), measured by runs on the seven_guis stand (Counter or CRUD plus a served task), headless, in both layout modes where layout touches the reading.
Answers whether a node's click listener can be read from the listener map today, as link 2's answer for click.
Re-verifies the MountedData hazard's mechanism and coordinates (events.rs:162, poll in dioxus_document.rs) at HEAD, then brings its fix to the founder as a fork with options, without deciding it.
Writes one plain-words reading for the founder (verdicts, evidence, cost class, forks) before the UI graph is planned. The probe code is test or example code only.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: the poll path where render_immediate holds the document RefCell, the mechanism of the MountedData hazard, re-verified at HEAD
@task_file n=2 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the RefCell already borrowed panic coordinate events.rs:162, re-verified at HEAD
@task_file n=3 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: the section on the hazard measured beside step 15, the carry's measured mechanism, read back
@task_file n=4 path=examples/seven_guis/src/main.rs
@task_file n=5 path=tests/blitz-tests/lib.rs
@task_item n=1 part=needs
The Dioxus launch path of the desktop-native surface (dioxus_native::launch, as examples/seven_guis/src/main.rs uses it): the seven_guis stand is where the probe runs, through its Counter or CRUD task and a served task.
@task_item n=2 part=needs
The bridge DioxusDocument in dioxus-native-dom: its element-id reads and its listener map (event names per node), which links 1 and 2 are read through.
@task_item n=3 part=needs
Link 2 for click: whether a node's click listener can be read from the listener map today (the deferred reacts-to-a-click reading).
@task_item n=4 part=needs
Links 3 and 4 in the Dioxus 0.7.3 runtime crates (read only): which signal a handler writes, and which nodes or components read that signal.
@task_item n=5 part=needs
The MountedData hazard: a task spawned from a key handler that writes a signal is first polled inside render_immediate, and a MountedData call in it panics RefCell already borrowed at events.rs:162. The probe re-verifies this mechanism at HEAD and does not fix it.
@task_item n=6 part=needs
The headless test harness the probe measures through: its poll(None), which measured the panic, and its VirtualClock with settle, advance, click and press, which step handlers and spawned tasks without moving the clock unless advance moves it.
@task_item n=7 part=needs
The two layout modes, where layout touches a link's reading: each verdict is measured in both.
@task_item n=8 part=needs
The windowed shell path (winit window, Vello window renderer) runs the same poll lines. It is read, not run, unless a reading needs it, and the reading says it was read from the code.
@task_item n=9 part=needs
The no-fork-level-change bound: no patch, vendored copy or pin move of a Dioxus crate (Dioxus stays 0.7.3) and no engine change for every document. A link that would need one stops at the verdict not exposed without a fork-level change.
@task_item n=10 part=needs
Nothing crosses: the probe adds no driver verb, argument, node key, result field, cause, session error, exit, listener, env var, workspace crate, log field or span. The escher-session stdout and mcp output stay as they are, and no component name, handler or signal value the probe reads is printed or logged.
@task_item n=11 part=needs
The founder's reading: one document with each link's verdict and its evidence (measured, or marked read-from-code), the cost class of building the UI graph on what is exposed, and the MountedData fix brought as a fork with its options.
@task_standing n=1
convention: a probe check added to the blitz-tests crate builds blitz-dom with the system-fonts feature, so text does not measure 0x0 and layout-dependent assertions do not pass vacuously
home: the Typography section of the body, under Loading
@task_item n=1 part=not_needed
The colour palette: core, surface, text, semantic and border colours, and the SelectListView engine colour constants. The probe paints and styles nothing.
@task_item n=2 part=not_needed
The spacing values and engine spacing: the probe reads components, handlers and state, not spacing.
@task_item n=3 part=not_needed
The depth strategy: observed elevation values, engine depth behaviour and the hit-testing clip rule. The probe does not read or change shadows or hit-testing.
@task_item n=4 part=not_needed
The border-radius values and engine radius handling.
@task_item n=5 part=not_needed
Iconography: the browser toolbar SVG icons, IconButton, the icon size grid, cursors and favicons.
@task_item n=6 part=not_needed
Programmatic scroll-into-view and the driver's scroll behaviour, from the motion section. The probe does no scrolling.
@task_item n=7 part=not_needed
The desktop-native tokens and its titlebar, devtools-overlay and custom-widget component patterns, plus the publish packaging (NSIS, dmg, AppImage).
@task_item n=8 part=not_needed
The mobile-native surface: Android APK, the iOS build, the urlbar, the back button and capture.
@task_item n=9 part=not_needed
The web-spa surface: WASM canvas pages, todomvc's canvas, and bundled WASM font contexts.
@task_item n=10 part=not_needed
The CLI binaries' terminal output colours (owo-colors in the WPT runner, paint_bench and screenshot examples).
@task_item n=11 part=not_needed
The project-wide token sets, type scale, spacing scale, elevation scale and radius scale marked not yet measured, and the sections with no recorded intent (brand identity, anti-patterns, self-validation, design decisions log).
