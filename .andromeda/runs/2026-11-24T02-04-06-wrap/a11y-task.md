@task_prose part=session
Circle Drawer in the seven_guis stand now paints its circles and each circle's diameter label from Rust on one custom-widget canvas, replacing the per-circle DOM nodes and their CSS rules, which are retired.
The canvas is one object element, role group, named Drawing area, keyed, clickable, reading no lint finding at boot in both layout modes; its circles reach no DOM node, snapshot, accessibility tree, UI graph or lint and leave only as shot pixels.
The engine gained a click synthesis on a custom-widget target: after the widget hears the pointer event and focus is set, pointerup on the main button fires the UA click, the secondary a context menu, and a second click fires a double click, with no click walk and no focus clear; an app's onclick and ondoubleclick on a widget element now fire for every document behind the custom-widget feature.
Measured: the stand's non-lean tables read Circle Drawer 1 to 0 for measured and clickable nodes; the canvas element shot joined the cross-OS digest table and reads equal on linux, windows and macOS only after the label's glyph origins were snapped to whole device pixels.
The capability statement gained Circle Drawer, the click on a widget element and two stated limits, a driver click lands at an element's centre and canvas text is byte-equal across hosts only on whole-pixel glyph origins; no pinned count moved.
The canvas answers (diameter as canvas text, centre-only driver click, role group Drawing area, the data-circles attribute) and the glyph snap and its stated limit are the escher overseer's word left standing, so rows resting only on them are PROVISIONAL; the engine click synthesis is the founder's own answer.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: a new engine check: one click and two clicks on a custom-widget canvas reach the app's click and double-click handlers in both layout modes, the widget still hearing every pointer event first
@task_file n=2 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: a new stand check of Circle Drawer's canvas: boot role group, name Drawing area, clickable, keyed, lint clean; a click paints a circle and its diameter label; undo and redo repaint; two clicks open the resize dialog
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the click, context menu and double click synthesised on a custom-widget target
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch, its custom-widget arm now calling the click synthesis
@task_file n=5 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven_guis Circle Drawer task — its drawing area now one canvas widget painting circles and their diameter labels on whole device pixels, role group, named Drawing area
@task_file n=6 path=examples/seven_guis/Cargo.toml
what: the seven_guis sample's dependency list, gaining the renderer, engine, text and paint crates
@task_file n=7 path=Cargo.lock
what: the workspace lock file, the seven_guis entry gaining four dependency lines
@task_file n=8 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots, gaining Circle Drawer's in-process boot, served by no session role
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot digest check, gaining a second table holding Circle Drawer's canvas element shot in both layout modes
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand's non-lean task tables of measured and clickable nodes per task, Circle Drawer moved 1 to 0
@task_file n=11 path=docs/capabilities.md
what: the capability statement edited by hand — the canvas claim, its witnesses, the proven section and the stated limits
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the chunk's journal of record lands, committed so the push's clean-tree guard read clean
@task_item n=1 part=touches
Stand reads no lint finding at boot: Circle Drawer's canvas now boots lint clean and is checked in both layout modes, a new sample under that rule.
@task_item n=2 part=touches
Actionable-key rule and one actionable rule: the canvas is one actionable keyed node with role group, where each circle node was before; the per-circle actionable nodes are gone.
@task_item n=3 part=touches
Interactive nodes carry a name: the canvas is named Drawing area through aria-label.
@task_item n=4 part=touches
Role from role attribute, element, else Unknown and role mapping: an object element with role group now reads Group in the tree.
@task_item n=5 part=touches
Snapshot clickable and the snapshot node's clickable: the canvas reads clickable; the per-circle clickable nodes no longer exist.
@task_item n=6 part=touches
Stand expectations stated once and expected roles and names stated once: the non-lean tables' measured and clickable readings for Circle Drawer moved 1 to 0 because the canvas states role group.
@task_item n=7 part=touches
Synthetic click is a primary click at the centre: the capability statement now states as a limit that a driver click lands at an element's centre; Circle Drawer's driver click on the canvas lands only at its centre until a drag verb exists (overseer's word, provisional).
@task_item n=8 part=touches
Double click threshold and double click read on the document's clock: a double click on a custom-widget target is now synthesised from the click count, the third click after the threshold reading a single click.
@task_item n=9 part=touches
A click focuses a button and the click-walk rules (disabled control stops the click walk): the widget-target synthesis fires the click with no click walk and no focus clear, an exception beside those rules.
@task_item n=10 part=touches
Text runs label their parent element and option labels leave only as pixels: Circle Drawer's diameter text is no DOM text run any more and leaves only as canvas pixels, a second case beside the open list's labels.
@task_item n=11 part=touches
UI graph mirrors the snapshot and snapshot reads the tree: circles leave the snapshot, tree and graph; only the canvas node remains.
@task_item n=12 part=touches
Stand contrast measured from computed styles, SC 1.4.3 asserted on the stand and lint contrast: the canvas label and circle fill are painted by the widget, not styled, so their contrast is not read by the harness or the lint.
@task_item n=13 part=touches
Stand accent colour: the canvas strokes circles in the stand accent, now hardcoded in the widget with the label colour and fill alpha the retired CSS held.
@task_item n=14 part=touches
A screenshot changes nothing and the driver's element shot: Circle Drawer's canvas element shot is the reading a click, undo and redo are checked against.
@task_item n=15 part=touches
Hidpi or zoom change reshapes text: the canvas label is drawn with the font size times the device scale, glyph origins rounded to whole device pixels.
@task_item n=16 part=touches
Stand focus order against a shared table and stated Tab order walked: the canvas has no keyboard path in; circles are added only by pointer.
@task_item n=17 part=touches
Accessibility reach edited by hand: the capability statement gained Circle Drawer, the widget click and two stated limits, with no reach change and no pinned count moved.
@task_item n=18 part=touches
Stand checks repeated three rounds and the accessibility test leg: two new stand checks join the repeated stand rounds and the slow tier, green three of three.
@task_fact n=1 slug=keyboard-navigation
fact: A click on a custom-widget element is delivered to the widget first, then the engine fires the UA click on main-button pointerup, a context menu on the secondary button and a double click on the second click, with no click walk and no focus clear, so an app's onclick and ondoubleclick on a widget element fire.
@task_fact n=2 slug=screen-reader-support
fact: Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable; its circles and diameter labels leave the DOM, snapshot, accessibility tree, UI graph and lint and cross only as shot pixels (overseer's word, provisional).
@task_fact n=3 slug=keyboard-navigation
fact: Absence: no keyboard path into Circle Drawer's canvas; a circle is added, selected or resized only by pointer (Canvas keyboard path, a later entry).
@task_fact n=4 slug=visual-design-verification
fact: Text and fills a widget paints on a canvas are outside the stand's contrast reading and the lint; Circle Drawer's label colour and fill alpha are fixed in the widget, not taken from tokens.
@task_item n=1 part=untouched
The platform adapter, AccessKit adapter creation order and the no-adapter-on-stand rule: no adapter or tree construction changed.
@task_item n=2 part=untouched
ARIA state and property reads (checked, pressed, selected, invalid, expanded, describedby, labelledby, level, disabled): no ARIA read changed.
@task_item n=3 part=untouched
Single and multiple select, open list, range input and their key contracts: no select or range behaviour changed.
@task_item n=4 part=untouched
Cells grid and CRUD list keyboard contracts: no other seven_guis task's keys or focus changed.
@task_item n=5 part=untouched
Hidden and inert content exclusion, hit-testing of inert elements and focus fixup: unchanged.
@task_item n=6 part=untouched
Password masking, IME and caret rules: unchanged.
@task_item n=7 part=untouched
The driver schema, actions answer, refusal causes, MCP results, node ids and snapshot text form: no verb, carrier, field, cause, session error or exit added.
@task_item n=8 part=untouched
The obs log format and the a11y assertion harness section's log notes: no log field or span added.
