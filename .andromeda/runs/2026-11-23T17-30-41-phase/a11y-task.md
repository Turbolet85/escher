@task_prose part=chunk
Moves the seven_guis Circle Drawer's drawing area onto a real canvas: an object element carrying a widget painted from Rust, circles and each circle's diameter painted by the widget, the circle list carried in one data-circles attribute.
The canvas states itself as role group named «Drawing area»; the circles leave the snapshot, accessibility tree, UI graph and lint and cross only as screenshot pixels.
One engine change: a pointer press and release on a custom-widget target still synthesise click and double click after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire, for every document behind the custom-widget feature.
Adds an in-process stand boot of Circle Drawer (not a served task), checks in both layout modes, boot lint clean, every actionable element keyed, and the canvas element shot joining the cross-OS digest table.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis Circle Drawer sample screen — edits: its drawing area (a canvas widget in place of one div per circle), its click and double-click handlers, the resize dialog with its diameter slider
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boot of the seven_guis tasks — edits: an in-process boot of Circle Drawer (no served or lean task list grows)
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the seven_guis sample's crate manifest — edits: the widget's crates at workspace versions
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: the engine's event dispatch, including the hand-off of a pointer event to a custom-widget target — edits: the custom-widget dispatch path
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer handling and its click and double-click synthesis on pointer up — edits: click and double-click synthesis for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas element-shot check and its committed digest table — edits: the digest table (Circle Drawer's canvas element shot)
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand checks of the tasks with no lean boot, among them Circle Drawer's flow and its two-clicks double click
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared expectation tables — edits: Circle Drawer's rows
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the canvas sample and its stated limit (driver click only at the canvas centre)
@task_item n=1 part=needs
The canvas object's role comes from its role attribute: role group, not the Unknown an object element reads with no role attribute.
@task_item n=2 part=needs
The canvas's accessible name «Drawing area», given by an author name attribute on the object, so the actionable canvas carries a name.
@task_item n=3 part=needs
Content painted on a canvas crosses only as screenshot pixels: circles and canvas text appear in no snapshot node, accessibility tree node, UI graph node or lint finding.
@task_item n=4 part=needs
Engine click synthesis on a custom-widget target: after the widget hears pointer down and up, the engine still queues click and double click, so app click and double-click handlers on the canvas fire; the widget keeps hearing every event; behaviour for every document behind the custom-widget feature, checked in both layout modes.
@task_item n=5 part=needs
Double click read on the document's clock and its threshold: two driver clicks on one point with no advance read as a double click, now reaching the canvas object as they did the old div.
@task_item n=6 part=needs
A synthetic driver click is a primary click at the node's centre: every driver click on the canvas lands at its centre, a stated limit until drag; no offset argument on click.
@task_item n=7 part=needs
A click on the canvas object gives it focus (the object is focusable on click); what the focus and the focus order read with the canvas on screen.
@task_item n=8 part=needs
The snapshot's clickable reading and the actionable-node listing for the canvas: it lists as clickable and actionable, keyed by its author id.
@task_item n=9 part=needs
Actionable-key rule: every element an agent can act on in Circle Drawer is keyed; unkeyed actionable reads none on its screens (main, resize dialog).
@task_item n=10 part=needs
Boot lint clean: the lint on Circle Drawer's boot reads 0 findings in both layout modes, contrast included for the text and controls still on the page.
@task_item n=11 part=needs
The resize dialog's diameter slider is a range input: its pointer, key, value and input event behaviour the sample's flow drives through driver calls.
@task_item n=12 part=needs
Handler coverage and the reached-handler record: the app's click and double-click handlers on the canvas now reached through the engine's synthesised events.
@task_item n=13 part=needs
The driver's element shot of the canvas: a screenshot changes nothing, the element shot shows the painted circles and diameter text from the bundled fonts, and joins the committed cross-OS digest table; the whole screen holds page text and stays outside every cross-OS byte check.
@task_item n=14 part=needs
The data-circles attribute: an attribute write on the canvas is a tracked mutation reaching the widget's attribute_changed; malformed text reads as no circle, never a panic; at boot the canvas holds no circle and no text.
@task_item n=15 part=needs
The sample's states (circle added, undo, redo, resize dialog open) reached through driver calls in both layout modes, with Circle Drawer's expected rows stated once in the stand's shared tables.
@task_item n=16 part=needs
Circle Drawer is not served by the session host and is no lean task: no carrier, count or served list moves.
@task_item n=17 part=needs
The snapshot diff and UI graph across a circle added or removed: only DOM-side changes (undo and redo enabled-ness, attribute value not exposed) show, the circles themselves do not.
@task_standing n=1
convention: each new accessibility check is a Rust test over the AccessKit tree, run in both layout modes
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand screen's expected roles, names, keys and focus order are stated once in the stand's shared tables and asserted from there
home: screen-reader-support · 7-screen-reader-support
@task_standing n=3
convention: each new stand screen reads no lint finding at boot, contrast asserted from computed styles
home: visual-design-verification · 6-visual-design-verification
@task_standing n=4
convention: each new stand check runs in the accessibility test leg and is repeated three rounds
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Select lists, open or closed, single or multiple, list boxes and optgroups: their keys, rows, colours, motion and snapshot readings; Circle Drawer has no select.
@task_item n=2 part=not_needed
The Cells grid and CRUD list key contracts and the missing active descendant on the CRUD list: other seven_guis samples, untouched.
@task_item n=3 part=not_needed
Native checkbox and radio toggling, checked readings and ARIA checked, pressed, selected or expanded states: the canvas states a group role with no state.
@task_item n=4 part=not_needed
Password masking, file input values and IME rules: no text input of those kinds in this sample.
@task_item n=5 part=not_needed
Platform accessibility adapter reach and assistive-technology tier: no adapter is added to any stand binary and the tier does not move.
@task_item n=6 part=not_needed
Scrollbar thumb strokes and driver clicks in a scrollbar band, scheme, zoom and hidpi changes: the chunk adds no scrolling surface and changes no viewport.
@task_item n=7 part=not_needed
Invalid date state and other cognitive-accessibility rows: no date or invalid-state field in Circle Drawer.
@task_item n=8 part=not_needed
MCP tool results and served node id separators: Circle Drawer is not served by the session host in this chunk.
@task_item n=9 part=not_needed
A canvas keyboard path, virtual nodes for circles, drag, pan and zoom: later entries of this version, outside this chunk.
