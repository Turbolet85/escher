@task_prose part=chunk
The seven_guis stand's Circle Drawer moves its drawing area onto a real canvas: an object element carrying a Rust-painted widget that paints the circles from one data-circles attribute and each circle's diameter as canvas text from the document's own fonts, stating itself as a group named Drawing area.
One engine change in blitz-dom's events: a pointer press and release on a custom-widget target still make a click and a double click after the widget hears the event, so an app's onclick and ondoubleclick on a canvas fire, for every document behind the custom-widget feature.
Circle Drawer gets an in-process stand boot read by its checks, not served by escher-session; a driver click lands at the canvas's centre only, a stated limit until drag.
It re-asserts the sample's skeleton (add, undo, redo, resize dialog, every actionable element keyed) and a clean boot lint in both layout modes, and its canvas element shot joins the cross-OS committed digest table.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the stand's Circle Drawer sample, its drawing area and circle state — edits: the drawing area, now a canvas object with a custom widget, its data-circles attribute and its role and name
@task_file n=2 path=examples/seven_guis/src/stand.rs mark=edits
what: the stand's headless in-process boot of the seven_guis tasks — edits: a stand boot of Circle Drawer
@task_file n=3 path=examples/seven_guis/Cargo.toml mark=edits
what: the seven_guis sample's crate manifest — edits: the widget's crates, at workspace versions
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, the forwarding of a pointer event to a custom-widget target — edits: the custom-widget dispatch path
@task_file n=5 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer handling and its click and double-click synthesis — edits: the click and double-click synthesis for a custom-widget target
@task_file n=6 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=edits
what: the stand's cross-OS canvas shot check and its committed digest table — edits: the digest table, a row for Circle Drawer's canvas element shot
@task_file n=7 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the stand checks of the tasks outside the lean list, Circle Drawer's flow among them
@task_file n=8 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables of tasks and their rows — edits: Circle Drawer's rows (no served or lean list grows)
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the canvas sample and its stated centre-only click limit
@task_item n=1 part=needs
The driver's click verb on the canvas: it lands at the element's centre, two clicks on one point with no advance read as a double click, and the command answer reports it accepted; no offset argument and no new verb.
@task_item n=2 part=needs
The engine's pointer event path for a custom-widget target: the widget still hears every event, and the click and double-click events are now queued after it, a behaviour change for every document behind the custom-widget feature.
@task_item n=3 part=needs
The screenshot answer of Circle Drawer: the whole screen and the canvas element shot show the painted circles and canvas text; the circles cross to an agent only as screenshot pixels and the shot image leaves only in its answer.
@task_item n=4 part=needs
The snapshot, accessibility tree, UI graph and lint lose the per-circle nodes: the canvas object reads as one clickable node stating role group named Drawing area, and an agent reads no circle.
@task_item n=5 part=needs
The lint and the unkeyed-actionable read on Circle Drawer's boot and screens: 0 findings and none unkeyed, in both layout modes.
@task_item n=6 part=needs
The data-circles attribute: the circle list carried as page content on the canvas element, parsed by the widget on attribute change, malformed text read as no circle and never a panic; its value must not reach any log line or span field.
@task_item n=7 part=needs
Any log or tracing site the engine change or the widget adds: engine tracing stays behind its cargo feature, engine log sites write fixed messages with safe fields only, and no attribute value, circle list or painted text is logged.
@task_item n=8 part=needs
The driver's command span for the click and screenshot calls on the canvas: its fields carry no call content and the same span per command, unchanged by the canvas.
@task_item n=9 part=needs
The headless stand boot of Circle Drawer runs in process with no telemetry sink installed, and its checks stay silent.
@task_item n=10 part=needs
The cross-OS digest table: Circle Drawer's canvas element shot is a committed digest; its whole screen holds page text and stays outside every cross-OS byte check.
@task_item n=11 part=needs
The capability statement, updated by hand: the canvas sample and its centre-only click limit, with no served-task count moving.
@task_item n=12 part=needs
escher-session's served task list stays as it is: Circle Drawer is not served, so no carrier, count, coverage line or served list moves in this chunk.
@task_standing n=1
convention: each new check and shared test module is silent, printing nothing on a pass, and a check that reads stderr as empty unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=2
convention: each committed check artifact, such as a digest table row, holds no private data or secret
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new engine log line writes a fixed message with safe fields, compiled in only behind the engine's tracing feature
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_item n=1 part=not_needed
Metric emission, exporters and telemetry backends: the chunk adds no counter, gauge, histogram or export.
@task_item n=2 part=not_needed
The cold-agent pipe, its counted verbs, verdict fields and reasons: no served task or agent run changes in this chunk.
@task_item n=3 part=not_needed
The MCP role, the tool_call span and its endings, and escher-session's exit codes and commands: no MCP tool or session command changes.
@task_item n=4 part=not_needed
The agent-run harness events and their JSON lines: the chunk adds no harness run.
@task_item n=5 part=not_needed
Script error surfaces, window error events, script diagnostics drains and the WPT report: the sample paints from Rust and runs no page script.
@task_item n=6 part=not_needed
Session recording files, the recording directory and the session lifecycle: no recording or session state changes.
@task_item n=7 part=not_needed
Select lists, optgroup labels, multiple-select form entries and checkbox or radio change events: the chunk touches no form control.
@task_item n=8 part=not_needed
The build-directory sweep line, the package-install diagnostic line and CI artifact uploads: no CI job or workflow changes.
@task_item n=9 part=not_needed
The WASM panic hook and the native panic hook's previous-hook chaining: no panic handling changes.
