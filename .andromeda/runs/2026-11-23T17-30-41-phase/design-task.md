@task_prose part=chunk
Moves the Circle Drawer sample's drawing area onto a real canvas: an object element carrying a widget painted from Rust, the circles and each circle's diameter (numbers only, at its centre) painted from the app's circle state with the document's own font context.
The circle list reaches the widget as one data-circles attribute, malformed text read as no circle; the canvas states itself as role group named Drawing area.
Changes the engine: a pointer press and release on a custom-widget target still makes a click and a double click after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire, for every document behind the custom-widget feature.
Adds an in-process stand boot of Circle Drawer (not served by escher-session), checks in both layout modes of its flow, keys and boot lint, and joins its canvas element shot to the committed cross-OS digest table.
Re-reads the verification harness leaf's stand reader count at the epoch-close leaf pass; the click lands at the canvas centre only, a stated limit until drag.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer sample of the seven GUIs stand — its drawing area, add a circle by a click, undo, redo, the resize dialog with its diameter slider — edits: the drawing area (a canvas widget in place of one div per circle), the data-circles attribute, the canvas's role and name
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless boots of the sample tasks — edits: an in-process boot of Circle Drawer
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the seven GUIs example's crate manifest — edits: the widget's crates at workspace versions
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: the engine's event dispatch, including the hand-off of a pointer event to a custom widget's event handler — edits: the dispatch of a custom-widget target
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: the engine's pointer event handling and its click and double-click synthesis on pointer up — edits: the click and double-click synthesis for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas check and its committed digest table of element shots — edits: the digest table, a row for Circle Drawer's canvas element shot
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand's checks of the non-lean sample tasks, opened from Home by their card, including two driver clicks without advance read as a double click on Circle Drawer
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables and helpers read by the stand checks — edits: Circle Drawer's rows (no served or lean list grows)
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the canvas sample and its stated limit (driver click at the canvas centre only)
@task_item n=1 part=needs
The engine's click and double-click synthesis on a custom-widget target: a pointer press and release on a canvas still queues click and double click after the widget is handed the event, the widget still hears every event; proved by one engine check in both layout modes.
@task_item n=2 part=needs
The driver click verb on a canvas: it lands at the element's centre, so every circle an agent places is at the centre; a second centre click with no advance reads as a double click, which opens the resize dialog.
@task_item n=3 part=needs
The driver advance verb and the one virtual clock: what separates two clicks from a double click in the sample's flow checks.
@task_item n=4 part=needs
The screenshot verb's shot of Circle Drawer and of its canvas element: painted circles and canvas text from the bundled fonts, never painted with text skipped; at boot with no circle the canvas holds no text.
@task_item n=5 part=needs
The document's font context handed to a custom widget for its canvas text (the native configuration's font context), and the screenshot refusal when fonts are unavailable.
@task_item n=6 part=needs
The canvas's accessible statement: role group with the name Drawing area, read by the snapshot and accessibility tree; circles moved off the DOM leave the snapshot, the accessibility tree, the UI graph and the lint and cross only as screenshot pixels.
@task_item n=7 part=needs
Every element an agent can act on in the sample keyed: the unkeyed-actionable read finds none on its screens; the actions line and the UI graph read the canvas as one actionable element.
@task_item n=8 part=needs
Boot lint clean: the session lint on Circle Drawer's boot reads 0 findings in both layout modes.
@task_item n=9 part=needs
Hit-testing of a pointer event onto the canvas object element and focus taken on a click: the target that receives the press and release the engine now turns into a click.
@task_item n=10 part=needs
The data-circles attribute parsed by the widget when the attribute changes: a tracked mutation that reaches the widget's paint, malformed text read as no circle and never a panic.
@task_item n=11 part=needs
The cross-OS byte check of element shots: Circle Drawer's canvas element shot joins the committed digest table, while its whole screen holds page text and stays outside every cross-OS byte check.
@task_item n=12 part=needs
escher-session's served task list stays as it is: Circle Drawer is booted in process by the stand only, no carrier, count, verb, field or served task moves, no offset argument on click.
@task_item n=13 part=needs
The capability statement of the canvas sample and its stated limit, with no served-task count moving.
@task_standing n=1
convention: each new engine or stand check is built with system fonts for its text, and a shot with text never paints with text skipped
home: typography
@task_standing n=2
convention: each new check drives the sample through the driver verbs on the one virtual clock, the clock moving only by advance
home: motion-calibrated-to-expression-level-unrecorded · motion
@task_standing n=3
convention: driver and session output read by an agent, with no colour and no terminal test
home: surface · surface-cli
@task_item n=1 part=not_needed
Colour rows — engine colour constants, the colour scheme, select list colours, base colour: the chunk paints circles in the sample and sets no engine colour.
@task_item n=2 part=not_needed
The single select's list, its events, its accessibility and its overlay paint and hit order: Circle Drawer's resize dialog uses a slider, not a select.
@task_item n=3 part=not_needed
The WPT runner's output and result colours: the chunk adds no WPT run.
@task_item n=4 part=not_needed
The shell provider's window controls, cursor icons and favicon: a headless canvas sample moves no window, cursor or icon.
@task_item n=5 part=not_needed
Scrolling motion — smooth scroll, into-view scroll, overlay scrollbar fades and thumb drag: the canvas has no pan, zoom or scroll in this chunk.
@task_item n=6 part=not_needed
Spacing hints — table cellspacing and cellpadding, body margins — and font element or lang presentational hints: the sample's layout carries none of them.
@task_item n=7 part=not_needed
The JS window object and the script runtime DOM: the sample is a Dioxus native app with no script runtime.
@task_item n=8 part=not_needed
The web-spa surface and the WASM font context: the chunk's checks run natively in process, not on WASM pages.
@task_item n=9 part=not_needed
escher-session's MCP role results, stderr errors and exit statuses: the chunk adds no verb, field or served task and changes no session answer.
@task_item n=10 part=not_needed
Mobile targets: the chunk adds no mobile build or test.
