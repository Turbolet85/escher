@task_prose part=chunk
Fixes upstream issue 1578 in the engine: a custom widget's paint is handed its content-box size and its scene is appended at the content-box origin inside the element's own transform, so it covers its content box with padding, border and a transform alike; no clip is added (founder ruling F1, options A and C).
Moves a custom widget's forwarded pointer events (move, down, up, cancel, leave) into the same content-box frame in CSS px, transform still unapplied; a sub-document's events keep the border-box origin.
Lets a widget draw text from the bundled fonts through the document's own font context: a new defaulted method on the Widget trait hands it that context before paint, and the document gains a public accessor of its font context, both behind the custom-widget feature (founder ruling F2, option A).
Adds stand checks for placement, pointer frame and canvas text, each run red first and kept with its red and green readings in the chunk's evidence, in both layout modes.
Re-reads the existing canvas shot and custom-widget layout checks and the stand's cross-OS readings, which keep their readings, and re-reads the last wrap's CI verdict before the push.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: blitz-paint's paint pass over the document's nodes — edits: process_custom_widget_node, the size handed to a widget's paint and the font context handed before it
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: blitz-paint's element renderer, the frame and transform each element paints in — edits: draw_custom_widget, where a widget's scene is appended
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, mapping DOM events to a custom widget's UI events — edits: the custom-widget arm of handle_dom_event, the origin handed to map_dom_event_to_ui_event
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the Widget trait a custom widget implements, its paint returning a scene — edits: a new defaulted method handing the document's font context
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: BaseDocument, the document and its font context — edits: a new public accessor of font_ctx behind custom-widget
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the blitz-tests package manifest — edits: parley as a dev-dependency, a package already in the lock
@task_file n=7 path=tests/blitz-tests/tests/stand_canvas_shot.rs mark=re-reads
what: the stand's canvas shot check, read green with the fix
@task_file n=8 path=tests/blitz-tests/tests/custom_widget_layout.rs mark=re-reads
what: the custom-widget layout sizes check, read green with the fix
@task_file n=9 path=tests/blitz-tests/tests/stand_cross_os_readings.rs mark=re-reads
what: the seven served tasks' snapshot, lint and actions digests, which keep their readings
@task_item n=1 part=needs
The screenshot answer's contract stands for a canvas shot: an 800 x 600 image over opaque white, byte-identical across processes on one host, and nothing run or changed by a shot.
@task_item n=2 part=needs
The fonts-unavailable refusal cause still precedes any painting; canvas text drawn from the document's font context changes no refusal cause and adds none.
@task_item n=3 part=needs
Canvas text leaves only as pixels inside the shot image, within the ratified crossing that canvas content crosses as shot pixels; the shot image leaves in its answer alone and its text is never logged.
@task_item n=4 part=needs
No new verb, argument, carrier, refusal cause, session error, exit code, log field or span; the driver's command and tool_call spans keep their closed field sets, and no content enters a log field.
@task_item n=5 part=needs
Any engine log site the placement, pointer-frame or font-context change touches writes a fixed message with safe fields only, compiled in only behind the engine's tracing feature; a widget's size, coordinates or text is never logged.
@task_item n=6 part=needs
The headless stand runs the new checks with no telemetry sink installed, and the driver library installs no subscriber.
@task_item n=7 part=needs
New checks are silent and checks asserting empty stderr unset RUST_LOG, so a red-first or green run prints nothing beyond the test harness's own lines.
@task_item n=8 part=needs
Every log kept in the chunk's evidence (each red reading beside its green) is respelled so no temp or home path is saved, and committed checks and fixtures hold no private data.
@task_item n=9 part=needs
A widget's pointer events forwarded by the dispatch carry content-box-relative coordinates in CSS px for PointerMove, PointerDown, PointerUp, PointerCancel and PointerLeave; transforms stay unapplied and a sub-document's events are unmoved.
@task_item n=10 part=needs
The two new public items (the Widget trait's defaulted font-context method and the document's font-context accessor) carry doc comments, are flagged upstreamable, and hand app code a shared mutable handle to the document's font collection.
@task_item n=11 part=needs
The stand's served tasks' snapshot, lint and actions digests and the committed kept checks keep their readings, re-recorded only where a ruling moves one, never loosened.
@task_item n=12 part=needs
The last wrap's CI verdict is unread and is re-read before this chunk's push, never folded as green; the build and every CI cargo leg pass with --locked and the doc leg joins the local gate.
@task_standing n=1
convention: each new test is a silent check over shared modules, printing nothing of its own and unsetting RUST_LOG where it asserts empty stderr
home: 3-observability-harness-contract
@task_standing n=2
convention: each new headless stand test runs with no telemetry sink installed
home: 3-observability-harness-contract
@task_standing n=3
convention: each new committed test, fixture or kept log holds no secrets or private data, local state staying local
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
Metric emission, metrics exporters and telemetry backends: the chunk adds no counter, gauge, histogram or export.
@task_item n=2 part=not_needed
Error capture and reporting: panic hooks, script error source labels, the window error event and WPT report messages are untouched.
@task_item n=3 part=not_needed
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, transcripts and cleanup are not reached by an engine paint or event change.
@task_item n=4 part=not_needed
Session recording, the session lifecycle, the recording file and the session state directory: no session behaviour changes.
@task_item n=5 part=not_needed
The select, optgroup and multiple-select rows, the page script change events and the snapshot node and text clickable rows: no form control or snapshot field changes.
@task_item n=6 part=not_needed
The MCP role's stdout and exit statuses, escher-session's exit codes and commands, and the coverage role's line: no command or role changes.
@task_item n=7 part=not_needed
The build-directory sweep's stdout line, the package-install diagnostic line, WPT score publishing and the fork CI's uploaded artifacts: no CI job or workflow is added.
@task_item n=8 part=not_needed
The telemetry sink's line format, resource keys, scrub set and target allowlist: no sink, target or field is added.
