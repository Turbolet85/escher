@task_prose part=chunk
Moves the seven_guis stand's Circle Drawer drawing area onto a real canvas: an object element carrying an app widget painted from Rust, painting the circles from the app's circle state and each circle's diameter as canvas text, fed by one data-circles attribute.
Changes the engine (fork F2 A): a pointer press and release on a custom-widget target still synthesise click and double click after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire, for every document behind the custom-widget feature.
Adds an in-process stand boot of Circle Drawer and its checks in both layout modes: the skeleton flow by driver calls, boot lint clean, every actionable element keyed, a shot showing the painted circles and canvas text.
Adds Circle Drawer's canvas element shot to the canvas cross-OS digest table and states the canvas sample and its centre-only click limit in the capability statement; no served task, carrier, verb or lean list moves.
Each new control is shown with its red reading.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the seven_guis stand's Circle Drawer sample — its drawing area, circle state, undo, redo and resize dialog — edits: the drawing area becomes a canvas object with its widget, the data-circles attribute, the canvas role and name, the canvas text
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand and its boots of the sample tasks — edits: a new in-process boot of Circle Drawer
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the seven_guis example manifest — edits: the widget's crates at workspace versions
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, where a pointer event on a custom-widget target is handed to the widget — edits: the dispatch of a custom-widget target
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer handling and its click and double-click synthesis on pointer up — edits: the click and double-click synthesis for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's canvas cross-OS check, comparing canvas element shots across hosts by digest — edits: its committed digest table
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand checks of the tasks with no lean boot, opened from Home by their card, Circle Drawer among them
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables and fixtures in one shared module — edits: Circle Drawer's rows in the shared stand tables
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the canvas sample and its stated limit
@task_item n=1 part=needs
An app widget painted from Rust on an object element shows in a headless shot: Circle Drawer's circles are painted by the widget from the app's circle state, no longer one DOM element per circle.
@task_item n=2 part=needs
Canvas text painted by the widget from the document's own font context: each circle's diameter, numbers only, at its centre; at boot, with no circle, no canvas text; the shot's text comes from the bundled fonts.
@task_item n=3 part=needs
The circle list crosses to the widget as one data-circles attribute, parsed in the widget's attribute-changed hook; malformed text reads as no circle and never panics; an attribute write is a tracked mutation, so a circle-state change must reach the widget through that attribute to repaint.
@task_item n=4 part=needs
The canvas states itself as role group named «Drawing area»; a canvas object reads clickable and takes focus on a click.
@task_item n=5 part=needs
Engine behaviour change in blitz-dom's events: a pointer press and release on a custom-widget target still queue click and double click after the widget hears the event; the app's onclick and ondoubleclick on a canvas fire; the widget still hears every event; one engine check of a click on a canvas, in both layout modes, shown red before the change.
@task_item n=6 part=needs
Circle Drawer's in-process stand boot, read by its checks: a new boot of the headless stand alongside the existing boots; Circle Drawer stays outside the served tasks and the lean task list, its rows moving in the shared stand tables.
@task_item n=7 part=needs
A driver click lands at the target element's centre only: every circle an agent places sits at the canvas centre, and a second centre click with no advance reads as a double click that opens the resize dialog — a stated limit until drag.
@task_item n=8 part=needs
The sample's skeleton flow reached by driver calls: add a circle by a click on the canvas, undo, redo, the resize dialog with its diameter slider; every element an agent can act on keyed, the unkeyed-actionable reader finding none on its screens.
@task_item n=9 part=needs
Boot lint clean: the session lint on Circle Drawer's boot reads 0 findings, in both layout modes.
@task_item n=10 part=needs
Circles moved off the DOM leave the snapshot, the accessibility tree, the UI graph and the lint: an agent reads no circle; canvas content crosses only as screenshot pixels.
@task_item n=11 part=needs
Canvas shots compared across hosts by digest: Circle Drawer's element shot of its canvas joins the committed digest table, while its whole-screen shot holds page text and stays outside every cross-OS byte comparison.
@task_item n=12 part=needs
Handler coverage over a canvas: Circle Drawer's click and double-click handlers move onto a canvas element, which falls under the handler-coverage carve-out for canvas handlers.
@task_item n=13 part=needs
The capability statement gains the canvas sample and its centre-only click limit, its claims naming their checks and held to the driver schema; no served-task count or verb count moves.
@task_item n=14 part=needs
The engine change and the sample stay inside their bounds: no new driver verb, carrier, served task or field, no offset argument on click, Dioxus kept at 0.7.3, the widget's crates at workspace versions.
@task_item n=15 part=needs
Matrix row v040-09 (a canvas painted headless) is claimed by this chunk when its checks prove the whole acceptance: a shot of Circle Drawer showing circles and canvas text from the bundled fonts.
@task_standing n=1
convention: every new stand check and layout assertion runs in both layout modes
home: test-scope-summary · 1-test-scope-summary
@task_standing n=2
convention: every new test file is one integration test target of its own; shared tables and fixtures live in the one shared module, never a target
home: test-strategy · 2-test-strategy
@task_standing n=3
convention: every new stand check runs on the bundled pinned font and never skips; pixel reads on a CPU buffer
home: test-strategy · 2-test-strategy
@task_standing n=4
convention: every new control is shown red before green, its expected values pinned as literals and its failures printing no argument value
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=5
convention: test data stated in code, nothing seeded, the stand booting offline
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=6
convention: no sleep or wall-clock read in a check; time moves only through the virtual clock
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=7
convention: every new stand check holds the zero-failure budget across the stand-repeat rounds
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
Cold-agent pipe rows — its verdict fields, events, exit codes, invocation and stand tasks: Circle Drawer is not served in this chunk and no cold-agent run is made.
@task_item n=2 part=not_needed
Session wire framing, bounds and version rows, and the MCP protocol revision, tools list and protocol-error rows: no carrier, served task, verb or field is added.
@task_item n=3 part=not_needed
Committed recorded-check scripts and session recording rows — record directory, permissions, check file fields: the chunk records no check.
@task_item n=4 part=not_needed
Password masking, typed-password and snapshot text-budget rows: Circle Drawer holds no password field and the chunk changes no snapshot text.
@task_item n=5 part=not_needed
Timer and network stand-in rows — timer ticks, load page stand-in, counted network loads, the absence of a mocking library: the sample has no timer and loads nothing.
@task_item n=6 part=not_needed
CI workflow plumbing rows — actions pinned to SHAs, secrets, read-only permissions, the apt-install script's exit codes, cache save branches: no CI job or workflow is changed.
@task_item n=7 part=not_needed
Engine paint rows outside the canvas — select list painting, chosen select row colours, password caret, overlay scrollbar fade, box-shadow expectations: the engine change is click synthesis in events alone.
@task_item n=8 part=not_needed
Mutation-run and complexity-ceiling host rows — cargo-mutants under nextest, the verb-anchored mutation filter, the dev-host-only runs: no mutation run is scoped by this chunk.
