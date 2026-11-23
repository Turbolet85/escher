@task_prose part=chunk
Moves the stand's Circle Drawer drawing area onto a real canvas: an object element carrying a widget painted from Rust, circles and each circle's diameter painted from the app's own circle state, no longer one div per circle.
Changes the engine so a pointer press and release on a custom-widget target still make a click and a double click after the widget hears the event, for every document behind the custom-widget feature.
Gives Circle Drawer an in-process stand boot, not served by escher-session; the canvas states itself as a group named Drawing area and takes its circle list as one data-circles attribute.
Re-asserts the sample's skeleton and keys, boot lint clean in both layout modes, and checks in both layout modes; its canvas element shot joins the committed cross-OS digest table.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the stand's Circle Drawer sample, its drawing area, undo, redo and resize dialog — edits: the drawing area, moved onto a canvas object with a custom widget, its data-circles attribute and its role and name
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot of its samples — edits: an in-process boot of Circle Drawer read by its checks
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the sample app's crate manifest — edits: the widget's crates, at workspace versions
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: the engine's event dispatch, including the hand-off of an event to a custom-widget target — edits: the dispatch of a custom-widget target
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer handling and its click and double-click synthesis — edits: the click and double-click synthesis on a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas shot check and its committed digest table — edits: the digest table, a row for Circle Drawer's canvas element shot
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand's checks of the samples opened from Home by their card, including Circle Drawer's two-click flow
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables of samples and their boots — edits: Circle Drawer's rows
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the canvas sample and its stated limit
@task_item n=1 part=needs
Circle Drawer's drawing area as a canvas: an object element with a custom widget attribute, the widget painting the circles from the app's own circle state; the canvas keeps its stable element id circle-canvas.
@task_item n=2 part=needs
Canvas text painted from the document's own font context and the bundled fonts: each circle's diameter, numbers only, at its centre; at boot, with no circle, the canvas holds no text.
@task_item n=3 part=needs
The circle list reaches the widget as one data-circles attribute on the canvas, parsed when the attribute changes; malformed text reads as no circle and never panics.
@task_item n=4 part=needs
A change to the circle state reaches the widget's paint through an attribute write on the canvas, a tracked mutation; a headless shot paints the scene fresh at each shot.
@task_item n=5 part=needs
The canvas states its role and name by attributes alone: role group, named Drawing area; the circles themselves leave the snapshot, the accessibility tree, the UI graph and the lint and cross only as screenshot pixels.
@task_item n=6 part=needs
Engine behaviour change: a pointer press and release on a custom-widget target still runs click and double-click synthesis after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire while the widget still hears every event; in a function of escher's own, upstreamable.
@task_item n=7 part=needs
A new engine check of a click and double click on a canvas, run in both layout modes.
@task_item n=8 part=needs
Circle Drawer's in-process stand boot, read by its checks, sharing the stand's headless boot with its pinned viewport and reading as a fresh boot.
@task_item n=9 part=needs
The sample's skeleton holds: add a circle by a click on the canvas, undo, redo, the resize dialog with its diameter slider; the stable element ids circle-canvas, circle-undo, circle-redo, circle-diameter, circle-close and task-card-circle-drawer stay, and every control an agent can act on is keyed, the unkeyed-actionable read finding none.
@task_item n=10 part=needs
Boot lint clean: the lint of Circle Drawer's boot reads 0 findings in both layout modes, the canvas object included.
@task_item n=11 part=needs
A driver click lands at the canvas's centre only, a stated limit until drag: every circle an agent places lands at the centre, and a second centre click with no advance reads as a double click.
@task_item n=12 part=needs
Stand checks in both layout modes, in process: the shot shows the painted circles and the canvas text, and the sample's flow reaches its states through driver calls.
@task_item n=13 part=needs
Cross-OS digests: Circle Drawer's element shot of its canvas joins the committed digest table; its whole screen holds page text and stays outside every cross-OS byte check.
@task_item n=14 part=needs
The canvas object reads clickable and takes focus on a click; its accent and look follow the stand's own colour.
@task_item n=15 part=needs
The capability statement names the canvas sample and its stated centre-click limit; no served-task count moves.
@task_standing n=1
convention: every control a new or changed stand screen adds is keyed with a stable element id, and its name, role and state are stated by attributes alone
home: surface-desktop-native
@task_standing n=2
convention: every new stand check runs on the headless stand boot at its pinned viewport in both layout modes, and a stand screen lints clean at boot
home: surface-desktop-native
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the chunk opens and drives no window chrome.
@task_item n=2 part=not_needed
The escher-session served tasks list and its MCP server, recording, help, graph, layout and snapshot answers, exit codes and command line: Circle Drawer is not served in this chunk and no verb, carrier, served task or field is added.
@task_item n=3 part=not_needed
The cold-agent and agent-run scripts and the cold-agent decisions: the sample reaches a cold agent only at a later entry.
@task_item n=4 part=not_needed
The multi_doc and paint_bench command lines: no benchmark or multi-document tool is touched.
@task_item n=5 part=not_needed
The mobile-native and web-spa surfaces: the chunk builds on the desktop stand and the engine only.
@task_item n=6 part=not_needed
The other samples' element ids and their own decisions (Counter, Temperature Converter, Flight Booker, Timer, CRUD, Cells, Load, Order; the CRUD list keyboard region, the flight type select, the load page network, the order page, open list rows, into-view scrolling, clipping boxes): none of those screens changes.
@task_item n=7 part=not_needed
Arbitrary index.html templates for a document: the chunk adds no document template.
@task_item n=8 part=not_needed
Draw-command verbs, canvas lint, virtual nodes, a canvas keyboard path, drag, pan and zoom: each a later entry of this version.
