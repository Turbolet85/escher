@task_prose part=session
The stand sample's Circle Drawer now paints its circles and each circle's diameter as canvas text from Rust: its drawing area is one object#circle-canvas element (a custom widget) with role group, name Drawing area, and a data-circles attribute holding x,y,d per circle joined by semicolons.
The .circle DOM nodes and their CSS rules are retired; circles no longer appear in the snapshot, tree, graph or lint and cross only as screenshot pixels.
The engine now synthesizes the app's click, context menu and double click on a custom-widget target after the widget hears the pointer event, so an app's onclick and ondoubleclick on a widget element fire (founder's word F2).
The app reads a click point against the canvas's content box (2 px border subtracted); the first circle reads 348,198,40, not the predicted 350,200,40.
Circle Drawer boots clean of lint in both layout modes, keyed, in process only and served by no escher-session role; its canvas element shot joins the cross-OS digest table, equal on linux, windows and macOS once the label's glyph origins sit on whole device pixels.
The canvas role, name, data-circles attribute, the diameter as canvas text, the glyph snap and the stated limit Canvas text across hosts are the escher overseer's word, PROVISIONAL.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: new check that a click and a double click on a custom-widget canvas reach the app's click and double-click handlers, the widget hearing every pointer event first, in both layout modes
@task_file n=2 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: new check of Circle Drawer's canvas: boot role group, name Drawing area, keyed, lint clean; a click paints a circle and its diameter; undo and redo repaint; two clicks open the diameter dialog
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling; gains the click synthesis on a custom-widget target
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch; its widget arm now calls the click synthesis
@task_file n=5 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the stand sample's Circle Drawer task: its drawing area, now a canvas widget painting the circles and their diameters, its undo, redo and diameter dialog
@task_file n=6 path=examples/seven_guis/Cargo.toml
what: the stand sample's manifest; gains four rendering dependencies
@task_file n=7 path=Cargo.lock
what: the workspace lock; the stand sample's entry gains four dependency lines
@task_file n=8 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots of the sample tasks; gains Circle Drawer's in-process boot
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot digest table; gains a second table of Circle Drawer's canvas element shot in both layout modes
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the per-task measured and clickable readings of the stand's non-lean tasks; Circle Drawer's re-pinned 1 to 0 each
@task_file n=11 path=docs/capabilities.md
what: the capability statement; the canvas claim gains Circle Drawer and the click on a widget element, two stated limits added
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the chunk's journal log; one line of the operator pass record committed
@task_item n=1 part=touches
decision/stand-lints-clean-at-boot: Circle Drawer now joins the tasks that lint clean at boot, in both layout modes (lint 0, unkeyed actionable empty).
@task_item n=2 part=touches
contract/stand-element-ids/circle-canvas: the id now names an object element with class cd-canvas, role group, name Drawing area and a data-circles attribute, no longer a div with circle div children.
@task_item n=3 part=touches
decision/every-stand-control-keyed: Circle Drawer's canvas is keyed and its circles are no longer elements, so no per-circle control remains to key.
@task_item n=4 part=touches
decision/stand-a11y-by-attributes-alone: the canvas states role group and name Drawing area by attributes; its circles carry no name, role or state.
@task_item n=5 part=touches
decision/stand-ids-as-attributes-alone: the circle-canvas id stays an attribute; the circles lost their DOM nodes and carry no id.
@task_item n=6 part=touches
decision/stand-accent-colour: the circle stroke in the accent #3a5ce5, the label colour #1a1a1a and the fill alpha are now hardcoded in the canvas widget as the removed .circle CSS had them, not stated through styles.
@task_item n=7 part=touches
decision/session-roles-share-stand-boot: Circle Drawer gains a stand boot that is in process only and served by no escher-session role, an exception beside the roles that share the stand boot.
@task_item n=8 part=touches
decision/visible-region-through-clipping-boxes: the canvas scene is clipped to the widget's paint box, standing in for the removed div's overflow hidden.
@task_item n=9 part=touches
decision/headless-stand-pinned-viewport: the canvas is a 700 by 400 border box with a 696 by 396 content box; a click point read against the client rect is off by the border, so placements read against the content box.
@task_item n=10 part=touches
decision/held-instance-reads-as-fresh-boot: Circle Drawer's undo reads byte-equal to its boot canvas shot and redo to the after-click shot.
@task_item n=11 part=touches
contract/stand-element-ids/circle-diameter, circle-close, circle-undo and circle-redo: ids unchanged, but the dialog now opens through the engine-synthesized double click on the canvas and its slider repaints the canvas rather than a circle node.
@task_item n=12 part=touches
contract/escher-session-snapshot/answer, contract/escher-session-graph/answer, contract/escher-session-lint/answer and contract/escher-session-layout/answer: for Circle Drawer these no longer hold circles; circles cross as screenshot pixels alone.
@task_item n=13 part=touches
contract/escher-session-screenshot/answer: Circle Drawer's canvas element shot now carries the painted circles and diameter text, pinned by digest across linux, windows and macOS.
@task_item n=14 part=touches
contract/escher-session-served-tasks rows: Circle Drawer stays absent from the served tasks; its new boot is served by no role.
@task_item n=15 part=touches
contract/escher-session-actions/answer: a driver click lands at an element's centre, now a stated limit; on the canvas a click reaches the app's handler through the engine's click synthesis.
@task_fact n=1 slug=surface-desktop-native
fact: Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle), not as DOM nodes; they leave the snapshot, tree, graph and lint and cross as shot pixels alone (PROVISIONAL, the escher overseer's word).
@task_fact n=2 slug=surface-desktop-native
fact: An app placing widget content from a click reads the point against the widget's content box, subtracting border and padding, because the client rect is the border box while a widget paints in its content box.
@task_fact n=3 slug=surface-desktop-native
fact: Text an app paints on a canvas is byte-equal across hosts only with its glyph origins on whole device pixels; Circle Drawer's diameter label is drawn so (PROVISIONAL, the escher overseer's word).
@task_fact n=4 slug=surface-desktop-native
fact: Circle Drawer's canvas shows each circle's diameter as an integer, 12 px sans-serif, centred on the circle (PROVISIONAL, the escher overseer's word).
@task_fact n=5 slug=surface-desktop-native
fact: Circle Drawer's canvas has no keyboard path into it; a keyboard path for the canvas is a later entry.
@task_fact n=6 slug=surface-desktop-native
fact: An app's onclick and ondoubleclick on a custom-widget element fire, the engine synthesizing click, context menu and double click after the widget hears each pointer event (the founder's word F2).
@task_item n=1 part=untouched
The cold-agent and agent-run CLI contracts and the cold-agent decisions: no pipe, script or prompt changed.
@task_item n=2 part=untouched
The multi_doc and paint_bench command line contracts: no argument, variable or output changed.
@task_item n=3 part=untouched
The escher-session exit codes, refusals, help, coverage, recording, host log and MCP server contracts: no verb, argument, cause, session error, exit, log field or span added.
@task_item n=4 part=untouched
The ShellProvider window chrome contracts: no window control changed.
@task_item n=5 part=untouched
Stand element ids of the Counter, Temperature Converter, Flight Booker, Timer, CRUD, Cells, Load and Order tasks: unchanged.
@task_item n=6 part=untouched
Decisions on the CRUD list keyboard region, the flight type select, the load page network provider, the order page controls, open list rows, into-view scrolling and the fixed document skeleton: no change reaches them.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates, and the mobile-native, web-spa and decisions-log areas: not reached.
