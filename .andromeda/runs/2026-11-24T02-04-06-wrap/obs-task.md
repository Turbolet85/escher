@task_prose part=session
The session painted the stand sample Circle Drawer's circles and their diameter labels from Rust on one keyed canvas widget element (role group, named Drawing area), replacing the per-circle DOM nodes and their CSS, which are retired.
It changed the engine for every document behind the custom-widget feature: a click on a widget element now also reaches the app's click and double-click handlers after the widget hears the pointer event; no public item, verb, carrier, field, cause, session error, exit, log field or span was added.
Circles now leave the snapshot, tree, graph, lint and logs and cross to the caller as shot pixels alone; their data rides in a data-circles attribute parsed inside the sample, malformed text read as no circle.
It added Circle Drawer's in-process stand boot (served by no escher-session role), two new silent stand checks, and a second canvas row table in the cross-OS canvas digest check, read green on linux, windows and macOS after a fix.
The macOS CI leg first read red because the CPU renderer snaps glyph origins differently on Apple hosts; the fix draws the label with glyph origins on whole device pixels, and the capability statement gained two stated limits (a click lands at an element's centre; canvas text across hosts).
The sample crate gained four workspace dependencies (anyrender, blitz-dom, parley, peniko), with no new package in the lock.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: a new silent stand check that a click on a custom-widget canvas reaches the app's click and double-click handlers, in both layout modes
@task_file n=2 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: a new silent stand check of the Circle Drawer canvas: boot role, name, keying and lint clean, a click painting a circle and its label, undo and redo, the resize dialog
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling, now holding the click synthesis on a custom-widget target
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch, whose custom-widget arm now calls the click synthesis
@task_file n=5 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the stand sample's Circle Drawer task, now a canvas widget painting circles and diameter labels with glyph origins on whole device pixels
@task_file n=6 path=examples/seven_guis/Cargo.toml
what: the seven_guis sample crate's manifest, gaining four workspace dependencies
@task_file n=7 path=Cargo.lock
what: the workspace lock file, the seven_guis entry gaining four dependency lines
@task_file n=8 path=examples/seven_guis/src/stand.rs
what: the headless stand's boots of the seven_guis tasks, gaining Circle Drawer's in-process boot
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot check by digest, gaining a second table of the Circle Drawer canvas element shot, re-pinned equal on three hosts
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of measured and clickable counts per non-lean task, Circle Drawer's re-pinned from 1 to 0
@task_file n=11 path=docs/capabilities.md
what: the capability statement, updated by hand with Circle Drawer, the widget click, two stated limits and the two new witnesses
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the chunk's journal of record writes, one line committed for the operator pass record
@task_item n=1 part=touches
The silent-checks rule over the stand checks and their shared modules: two new stand checks are added, both silent and sharing the session module, neither declaring the fixtures module.
@task_item n=2 part=touches
The rule that the headless stand runs with no sink: a new in-process Circle Drawer boot joins the stand, installing no sink.
@task_item n=3 part=touches
The rule that the capability statement is updated by hand: it gained Circle Drawer, the click on a widget element, two stated limits and two witnesses; no pinned count moved.
@task_item n=4 part=touches
The rule that a shot image leaves only in its answer: Circle Drawer's circles and diameter labels now cross to the caller only as shot pixels.
@task_item n=5 part=touches
Indirect, through the rule that an open list shot carries labels as pixels: canvas text (the diameter label) is now another label carried as pixels only, not in the snapshot or logs.
@task_item n=6 part=touches
The snapshot node and snapshot text clickable contract items: the Circle Drawer canvas is one clickable group element, and its measured and clickable task counts moved from 1 to 0.
@task_item n=7 part=touches
The graph answer content and the actions answer content contract items: circles no longer appear as DOM nodes in the graph, tree or actions; only the canvas element does.
@task_item n=8 part=touches
The rule that session logs carry no page content: the circle data lives in a sample-internal data-circles attribute and is never logged.
@task_item n=9 part=touches
Indirect, through the engine log-site rules (fixed messages, safe fields, tracing behind a cargo feature): the engine's custom-widget event arm changed and added no log site, event or span.
@task_item n=10 part=touches
The driver command shot answer contract: the canvas element shot now holds painted circles and text, byte-equal across linux, windows and macOS only with glyph origins on whole device pixels.
@task_item n=11 part=touches
The CI on GitHub workflows decision and the per-leg CI log on failure: the macOS test leg read red once on the canvas digest and was read green on all three hosts after the fix.
@task_item n=12 part=touches
The rule that the state registry holds no values: Circle Drawer's canvas state is an attribute read by its widget, not a registered value.
@task_fact n=1 slug=log-coverage
fact: The click synthesis on a custom-widget target (click, context menu, double click after the widget's own handling) adds no log site, event or span, and the two stand checks guarding it and the Circle Drawer canvas are silent.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: Content a widget paints on a canvas (Circle Drawer's circles and diameter labels) crosses only as shot pixels; it stays out of the snapshot, tree, graph, lint and logs, its data held in a sample-internal attribute.
@task_item n=1 part=untouched
Telemetry backends, metrics exporters, metric emission and WPT score publishing: no telemetry or metric surface changed.
@task_item n=2 part=untouched
escher's sink log line format, the telemetry sink's stream, RUST_LOG handling, scrub sets and resource keys: no sink or field changed.
@task_item n=3 part=untouched
The driver command and tool_call spans, their fields and the refusal cause names: no verb, field or cause was added.
@task_item n=4 part=untouched
The cold-agent and agent-run harness events, verdict fields, counted verbs and transcripts: no harness changed.
@task_item n=5 part=untouched
escher-session's roles, exit codes, error object, host wire, coverage line and mcp stdout: no session surface changed.
@task_item n=6 part=untouched
Script error surfaces, window error events, panic hooks and the WPT report: no script or error path changed.
@task_item n=7 part=untouched
Recording files, session state and recording directories, session lifecycle, and the build-directory sweep line: none changed.
@task_item n=8 part=untouched
Select and optgroup rows, multiple select entries and page script change events: no form control changed.
