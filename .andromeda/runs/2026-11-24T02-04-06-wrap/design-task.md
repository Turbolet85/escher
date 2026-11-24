@task_prose part=session
The seven GUIs Circle Drawer sample now paints its circles and their diameters from Rust on one canvas element (a custom widget), replacing its per-circle DOM nodes and their CSS rules, which are retired.
The canvas is an object element with role group named Drawing area, keyed, lint clean at boot in both layout modes; the circles ride on one data-circles attribute and leave the DOM, snapshot, accessibility tree, UI graph and lint, crossing only as screenshot pixels.
The engine now synthesises the app's click, context menu and double click on a custom-widget target after the widget hears each pointer event, for every document behind the custom-widget feature (every Dioxus native app); before, a widget element swallowed them.
Measured: a widget paints in its content box while its client rect is its border box, so the sample reads a click point against the content box (first circle 348,198 not 350,200); the CPU renderer snaps glyph origins differently on Apple hosts, so canvas text is byte-equal across hosts only with glyph origins on whole device pixels, and the diameter label is now drawn that way.
The canvas-shot cross-OS digest table gained a second table holding Circle Drawer's canvas element shot (with text), green on linux, windows and macOS; the capability statement gained Circle Drawer, the widget click and two stated limits; the sample gained four workspace dependencies.
No driver verb, argument, field, session error, exit status, log field or span was added; Circle Drawer gets an in-process stand boot served by no escher-session task role.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: new check, the engine's click on a custom-widget element: one click reaches the app's click handler, two clicks with no clock advance reach its double-click handler, the widget hearing every pointer event first, both layout modes
@task_file n=2 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: new check, Circle Drawer's canvas: boots named, keyed and lint clean; a click paints a circle and its diameter; undo and redo repaint; two centre clicks open the resize dialog; label dark-pixel count re-pinned 40 to 41 after implement
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling; gains the click, context-menu and double-click synthesis on a custom-widget target
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch; its custom-widget arm now calls the click synthesis after the widget's own handling
@task_file n=5 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven GUIs Circle Drawer task: its drawing area is now one canvas custom widget painting circles, fill, stroke and the diameter label (glyph origins on whole device pixels after implement), with role group, name Drawing area and the data-circles attribute; the circle DOM nodes and their CSS removed
@task_file n=6 path=examples/seven_guis/Cargo.toml
what: the seven GUIs sample's manifest; gains four workspace dependencies (the renderer interface, the engine DOM, text layout, paint types)
@task_file n=7 path=Cargo.lock
what: the workspace lock file; the seven GUIs entry gains four dependency lines, no package added
@task_file n=8 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots of the seven GUIs tasks; gains Circle Drawer's in-process boot, served by no escher-session role
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot digest check; gains a second table of the sample's canvas element shot (Circle Drawer, both layout modes), re-pinned after implement to the digest equal on linux, windows and macOS, 5 to 6 tests
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the non-lean tasks check's measured and clickable readings per task; Circle Drawer moves 1 to 0 on each
@task_file n=11 path=docs/capabilities.md
what: the capability statement: the canvas claim gains Circle Drawer and the click on a widget element, two witnesses, and two stated limits (a driver click lands at an element's centre; canvas text across hosts needs glyph origins on whole device pixels)
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the chunk's journal log; one line of the operator pass record committed so the push saw a clean tree
@task_item n=1 part=touches
The click on a custom-widget target: rows on hit testing and event delivery (hit-testing stops at a paint clip, stacking contexts group paint and hit-testing) sit beside a new engine behaviour where a widget element now also yields the app's click, context menu and double click after the widget hears the pointer events.
@task_item n=2 part=touches
The driver click contract and the virtual clock advance contract: a driver click on a widget element now reaches the app's click handler; two clicks with no advance give a double click and an advance of 600 ms separates them; a driver click lands at its target's centre, now a stated limit.
@task_item n=3 part=touches
Actions decided by the verbs' own checks, and the actions line nodes: Circle Drawer's canvas is now one clickable node with role group, its measured and clickable non-lean readings moving 1 to 0.
@task_item n=4 part=touches
The snapshot, the snapshot part and the UI graph line content: Circle Drawer's circles no longer appear as nodes; they cross only as screenshot pixels of the canvas element.
@task_item n=5 part=touches
The screenshot command with an id: Circle Drawer's canvas element shot is now a checked artifact, byte-equal across linux, windows and macOS in both layout modes.
@task_item n=6 part=touches
Screenshot never paints with text skipped: the canvas element shot now holds text painted by an app widget, the diameter label, and its digest depends on that text.
@task_item n=7 part=touches
Tests built with system fonts and test engine built with system fonts: the canvas label is drawn 12 px sans-serif through the system font context handed to the widget, and its cross-host equality rests on glyph-origin snapping, not on the font set.
@task_item n=8 part=touches
No generic family lays out as sans-serif and unavailable families fall back to the default font: the widget names sans-serif for its label and draws through the same font fallback.
@task_item n=9 part=touches
Engine colours hardcoded and color tokens absent: the sample widget hardcodes its stand accent stroke, label colour and fill alpha in code, carried over from the removed CSS rules, outside any token set.
@task_item n=10 part=touches
Escher-session tasks contracts: Circle Drawer gains an in-process stand boot but no task role; no task name is taken and the unknown-task answer still covers circle drawer.
@task_item n=11 part=touches
The coverage line and the coverage check files: Circle Drawer's interactive surface is now one keyed, named canvas, unkeyed actionables empty, lint zero at boot.
@task_item n=12 part=touches
Hit-testing stops where paint clips: the widget's scene is clipped to its paint box, standing in for the removed container's overflow hidden, so paint and hit area agree at the canvas edge.
@task_fact n=1 slug=typography
fact: Text an app paints on a canvas is byte-equal across hosts only with each glyph origin on a whole device pixel, because the CPU renderer snaps glyph origins on Apple hosts to a different grid; the stand's canvas label is drawn so, pinned by the cross-OS canvas digest check (the escher overseer's word, provisional).
@task_fact n=2 slug=surface
fact: On a custom-widget element the engine delivers every pointer event to the widget first, then synthesises the app's click (main button), context menu (secondary button) and double click (second click within the window), with no click walk and no focus clear; every Dioxus native app behind custom-widget gets it (the founder's word).
@task_fact n=3 slug=surface
fact: Circle Drawer's drawing area is one canvas element with role group named Drawing area, keyed and lint clean at boot; its circles are carried by one data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle) and painted, never DOM nodes (the escher overseer's word, provisional).
@task_fact n=4 slug=surface
fact: A widget paints in its element's content box while the element's client rect is its border box, so an app placing widget content from a click point subtracts border and padding (2 px on Circle Drawer's canvas).
@task_fact n=5 slug=surface
fact: No keyboard path into a canvas widget is built: Circle Drawer's canvas takes circles only by pointer, a later entry.
@task_fact n=6 slug=color-palette
fact: Circle Drawer's canvas colours are constants in the sample's widget code (stand accent stroke, dark label, accent fill at alpha 38/255), as the removed CSS rules held them, an exception to colours living in stylesheets.
@task_item n=1 part=untouched
Brand identity, spacing, border radius and the anti-pattern, self-validation and design-decisions-log areas: no scale, token or rule there moved.
@task_item n=2 part=untouched
Iconography: cursor icon crate, cursor resolution, favicon and the shell provider set-cursor contract are not reached.
@task_item n=3 part=untouched
The select list rows (overlay, list events, accessibility, colours, motion, placement, hit order, screenshot and snapshot rows of an open list): no select behaviour changed.
@task_item n=4 part=untouched
Escher-session exit statuses and exit codes, stderr, session error remedy, mcp role results and stdout lines, help and command arguments: no verb, argument, field, cause, session error, exit or log field added.
@task_item n=5 part=untouched
WPT runner output and result colours, and the surface-cli navigation and platform notes: not reached.
@task_item n=6 part=untouched
Scrollbars, smooth scroll, into-view scroll, reduced-motion handling, animation on the virtual clock and settle or load waits: no scroll or animation behaviour changed.
@task_item n=7 part=untouched
The JS window object contracts, the shell provider window contracts (maximize, minimize, decorations, drag), dioxus-native configuration items (base color, alpha mode, font context on WASM) and the web-spa page canvas: not reached; the web-spa canvas is the page surface, not a widget element.
