@task_prose part=chunk
The seven_guis stand's Circle Drawer moves its drawing area onto a real canvas: an object element carrying a custom widget painted from Rust, the circles and each circle's diameter text painted from the app's circle state and the document's font context, no longer one DOM div per circle.
The circle list reaches the widget as one data-circles attribute; the canvas states itself as a group named Drawing area; the stand gains an in-process boot of Circle Drawer, which stays unserved by escher-session.
One engine change in blitz-dom's event dispatch: a pointer press and release on a custom-widget target still synthesise click and double click after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire, for every document behind the custom-widget feature.
It re-asserts the sample's skeleton (add, undo, redo, resize dialog), the boot lint at 0 findings and no unkeyed actionable element in both layout modes, and adds the canvas element shot to the stand's cross-OS digest table; it closes the epoch and claims the headless-canvas acceptance (v040-09).
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis stand's Circle Drawer sample, its circle state and its drawing area — edits: the drawing area, moved onto a canvas object with a custom widget painting the circles and their diameter text, its data-circles attribute, its role and name
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand, the in-process boot its checks read — edits: an in-process stand boot of Circle Drawer
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the seven_guis crate manifest, its dependencies at workspace versions — edits: the widget's crates
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's DOM event dispatch, where an event on a custom-widget target is forwarded to the widget — edits: the dispatch of a custom-widget target
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, the pointer-derived click and double-click synthesis — edits: the click and double-click synthesis on pointer release for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's canvas cross-OS check, element shots of a canvas held to a committed digest table identical on every CI OS — edits: its committed digest table, Circle Drawer's canvas element shot
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand checks of the seven_guis tasks that are not lean tasks, opened from Home by their card, including Circle Drawer's flow and the two-clicks-without-advance double click
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared test module and its shared task tables — edits: Circle Drawer's rows in the shared tables
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement, what escher can do and its measured limits — edits: the canvas sample and its stated limit of a driver click at the canvas's centre
@task_item n=1 part=needs
The engine change: a pointer press and release on a custom-widget target are handed to the widget's handle_event and then still run the click and double-click synthesis, so click and double click reach the app's handlers on the canvas; the widget still hears every pointer event; written in a function of escher's own, flagged upstreamable, and binding every document behind blitz-dom's custom-widget feature.
@task_item n=2 part=needs
Pointer-derived event order and the repeated-click window as the engine dispatch states them: the click and double click synthesised for a canvas target follow the same order, targeting and bubbling as for any other element, and focus on a click of the object stays as today.
@task_item n=3 part=needs
A new engine check of a click and a double click on a custom-widget canvas, in both layout modes: the app reads its onclick and ondoubleclick fired and the widget still hears down and up.
@task_item n=4 part=needs
The canvas as custom content on an object element: a Widget attached through the custom widget attribute, painted at the object's content box by blitz-paint, its object data as the custom widget embedding states it; the object stays outside the replaced elements.
@task_item n=5 part=needs
Canvas text painted from the document's own font context handed to the widget (Widget::fonts): each circle's diameter, numbers only, at its centre, from the bundled fonts; at boot with no circle the canvas holds no text.
@task_item n=6 part=needs
The data-circles attribute on the object carries the circle list to the widget, parsed in the widget's attribute_changed; malformed text reads as no circle and never panics; the attribute write goes through the document mutator as a tracked mutation, so a windowed repaint follows it.
@task_item n=7 part=needs
The canvas's accessibility reading: role group named Drawing area in the accessibility tree and the snapshot; the canvas reads clickable as an actionable element and carries an author key.
@task_item n=8 part=needs
Circles moved off the DOM leave the snapshot, the accessibility tree, the UI graph and the lint: their content crosses only as screenshot pixels, the canvas-crosses-as-pixels rule.
@task_item n=9 part=needs
Circle Drawer's circle state stays app state in seven_guis, its signals registered with the UI graph as the other seven_guis tasks' are, with its undo and redo history.
@task_item n=10 part=needs
The in-process stand boot of Circle Drawer: the seven_guis headless stand boot, offline, with its pinned environment, timer step and no telemetry; Circle Drawer is no lean task and not among the tasks escher-session serves, no carrier, count or served list moving.
@task_item n=11 part=needs
A driver click on the canvas lands at the element's centre through the per-verb target check, so every circle an agent places sits at the centre, and a second centre click with no advance reads as a double click that opens the resize dialog; the limit is stated where an agent reads the tool, until drag.
@task_item n=12 part=needs
The sample's skeleton holds: add a circle by a click on the canvas, undo, redo, the resize dialog with its diameter slider (a range input); every element an agent can act on keyed, DioxusDocument::unkeyed_actionable reading none on its screens.
@task_item n=13 part=needs
Boot lint clean: Session::lint on Circle Drawer's boot reads 0 findings, in both layout modes.
@task_item n=14 part=needs
Stand checks in both layout modes, in process: the headless screenshot of Circle Drawer, on the CPU renderer over white with custom widgets painted and the stand's bundled typography, shows the painted circles and canvas text; the sample's flow reaches its states through driver calls.
@task_item n=15 part=needs
Cross-OS digests: Circle Drawer's element shot of its canvas joins the committed digest table, identical on every CI OS; its whole screen holds page text and stays outside every cross-OS byte check, the carve-out of the no-screenshot-in-cross-os-byte-check rule.
@task_item n=16 part=needs
Handler coverage of the canvas: the app's click and double-click handlers on the canvas are reached handlers once the engine synthesises the click, read by the handler coverage reader's canvas handlers and reached record.
@task_item n=17 part=needs
The capability statement names the canvas sample and its stated limit, held to the driver schema, with no served-task count and no MCP tool count moving.
@task_item n=18 part=needs
The widget's crates join seven_guis at workspace versions, declared once in the workspace; Dioxus stays at 0.7.3.
@task_item n=19 part=needs
Each new control is shown with its red reading: every new check is first seen failing against the code as it stands before the change makes it pass.
@task_standing n=1
convention: each new check runs its scenario in both layout modes
home: conventions
@task_standing n=2
convention: each new integration test lives in blitz-tests, one test target per file, one behaviour per file, declared in the blitz-tests manifest's one test table
home: design-philosophy · conventions · standard-contracts
@task_standing n=3
convention: each new test has a descriptive name and asserts its fixture's condition first
home: conventions
@task_standing n=4
convention: each new stand check takes its shared helpers and tables from the shared fixtures module, stated once, helpers in non-target modules
home: conventions
@task_standing n=5
convention: each new stand check boots offline under the stand's pinned environment and bundled typography, its readings identical on every CI OS
home: standard-contracts · infrastructure-patterns
@task_standing n=6
convention: new code carries module and public item docs, justified expect and unwrap, annotated unsafe and locally scoped lint allows
home: conventions
@task_standing n=7
convention: an engine change cites the spec at its implementation and its regression check names the upstream behaviour it changes
home: design-philosophy · conventions
@task_item n=1 part=not_needed
The WPT runner, its suites, reftests, result kinds, statuses, outputs and CI artifacts: the chunk runs no WPT work.
@task_item n=2 part=not_needed
The browser app: its chrome, tabs, history store, about pages, persistence and bookmarks.
@task_item n=3 part=not_needed
The JavaScript runtime: Boa, script globals, script fetching, the script document and its microtasks.
@task_item n=4 part=not_needed
Networking and loading: blitz-net, the net provider, the HTTP cache, stylesheet and image fetching, per-host request caps.
@task_item n=5 part=not_needed
The session socket's wire, the session host, the MCP server and mcp role, the escher-session command line and its exit codes: no carrier, served task, verb or field moves.
@task_item n=6 part=not_needed
Cold-agent and agent-run scripts, their events, exit codes, verdict fields and state directories: Circle Drawer is served only at a later entry.
@task_item n=7 part=not_needed
The select model and open select list: its keyboard, painting, rows, default actions and readers.
@task_item n=8 part=not_needed
Layout engine internals: Taffy, tables as grid, inline layout, writing modes, scrolling, anonymous blocks, and the absent CSS features.
@task_item n=9 part=not_needed
Kept check files, session recordings, check replay and driver explore: the chunk records and replays nothing.
@task_item n=10 part=not_needed
Upstream sync: the two-parent merge, cherry-picks and per-take ratification of upstream behaviour changes.
@task_item n=11 part=not_needed
Later canvas entries of this version: draw-command verbs, canvas lint, virtual nodes, a canvas keyboard path, drag, pan and zoom.
@task_item n=12 part=not_needed
CI workflow plumbing: the package-install script, skia archive verification, cache save refs, CI permissions and the wpt diff-to-PR script.
@task_item n=13 part=not_needed
The Dioxus 0.8 move and any Dioxus version change.
