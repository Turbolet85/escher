@task_prose part=session
The engine now synthesizes the UA click on a custom-widget target: behind the custom-widget feature, after the widget's own handle_event and the focus set, a pointerdown is recorded, a pointerup yields Click on the main button or ContextMenu on the secondary, and a Click with a count of two queues DoubleClick, with no click walk and no focus clear; an app's onclick and ondoubleclick on a widget element now fire, the widget still hearing every pointer event first (the founder's word, F2; upstreamable, not sent).
The stand sample's Circle Drawer replaced its div canvas and its circle DOM nodes with one object element, role group, named Drawing area, keyed circle-canvas, painted by a custom widget from Rust that reads a data-circles attribute (x,y,d triples, semicolon-joined; malformed text read as no circle, no panic) and paints each circle and its diameter as canvas text, the scene clipped to the paint box.
Circles leave the DOM, snapshot, tree, graph and lint and cross only as shot pixels; no verb, argument, carrier, field, refusal cause, session error, exit code, log field or span was added.
The sample crate gained four workspace-inherited dependencies (anyrender, blitz-dom, parley, peniko), no new package in the lock; build, audit (advisories ok, licenses ok) and the fast leg green.
After implement the diameter label is drawn with its glyph origins on whole device pixels, so the canvas shot is byte-equal on linux, windows and macOS; the capability statement gained Circle Drawer, the click on a widget element and two stated limits (a click lands at an element's centre; canvas text across hosts).
The circle-drawer answers F3 to F5, data-circles, the fourth dependency, the glyph snap and the stated limit are the escher overseer's word, PROVISIONAL; F1 and F2 are the founder's.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling - pointerdown recording, pointerup into click and context menu, double click - now with the click synthesis on a custom-widget target
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: the engine's DOM event dispatch, its custom-widget arm now calling the click synthesis after the widget hears the event
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the stand sample's Circle Drawer app - its canvas custom widget, the data-circles attribute parse, the circle and label paint, the click point read against the content box
@task_file n=4 path=examples/seven_guis/Cargo.toml
what: the stand sample crate's manifest - its dependencies, now four more inherited from the workspace
@task_file n=5 path=Cargo.lock
what: the workspace lockfile the locked builds and the dependency audit read - the sample's entry gains four dependency lines, no package added
@task_file n=6 path=examples/seven_guis/src/stand.rs
what: the headless stand's in-process boots of the sample tasks, now with Circle Drawer's boot, served by no escher-session role
@task_file n=7 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: the engine check that a click and a double click on a custom-widget element reach the app's handlers in both layout modes
@task_file n=8 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the Circle Drawer canvas checks - boot role, name, key and clean lint, click paints a circle and its diameter, undo and redo, the dialog's slider
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot digest table, now with a second table of Circle Drawer's canvas element shot
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand's per-task measured and clickable readings, Circle Drawer's now 0 each
@task_file n=11 path=docs/capabilities.md
what: the committed capability statement - the canvas claim, its witnesses and its stated limits
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the chunk's process journal, one line committed so the push's clean-tree guard read clean
@task_item n=1 part=touches
The custom-widget engine rows: an engine change for every document behind the custom-widget feature - a click, context menu and double click are now synthesized on a custom-widget target after the widget's handle_event, with no click walk and no focus clear; standing custom-widget rows stand, the synthesis is an add beside them.
@task_item n=2 part=touches
decision/boundary-widening-rule: the click synthesis reads as the boundary-widening pattern; it adds no carrier, verb, field or log site and no input crosses, and it rests on the founder's answer F2.
@task_item n=3 part=touches
decision/event-is-trusted-by-source: the synthesized click, context menu and double click on a widget target are engine-dispatched UA events from a real pointer event.
@task_item n=4 part=touches
decision/button-click-focuses and decision/cancelled-pointerup-still-clicks: the pointerup-into-click rules now also run on a custom-widget target, but without the click walk and without clearing focus.
@task_item n=5 part=touches
decision/custom-widget-content-box-frame: the sample reads its click point against the canvas's content box (2 px border subtracted) because the client rect is the border box while the widget paints in its content box.
@task_item n=6 part=touches
decision/custom-widget-initial-attribute-replay and decision/custom-widget-document-font-context: Circle Drawer's widget reads data-circles through attribute_changed and keeps the document's font context to paint its label.
@task_item n=7 part=touches
decision/attribute-readers-no-log-or-panic and decision/attribute-text-leaves-by-ratified-carriers: data-circles is parsed by the sample's widget as finite triples, malformed text read as no circle, no panic, no log; its text crosses no carrier.
@task_item n=8 part=touches
decision/canvas-content-crosses-as-shot-pixels: Circle Drawer's circles and diameter text now cross only as shot pixels, out of the DOM, snapshot, tree, graph and lint.
@task_item n=9 part=touches
decision/screen-pixels-in-band, decision/screenshot-in-band and contract/screenshot-answer/png: Circle Drawer's canvas element shot is read as an in-band png, now with canvas-painted text in it.
@task_item n=10 part=touches
decision/accessible-name-exits and decision/stable-id-exits: the canvas is a new keyed element, id circle-canvas, role group, accessible name Drawing area, leaving by the existing exits.
@task_item n=11 part=touches
decision/lint-findings-crossing and decision/lint-findings-closed-content: Circle Drawer boots with no lint finding; the circles are no longer elements the lint reads.
@task_item n=12 part=touches
decision/no-screen-content-in-logs and the other no-log-site rows: the click synthesis and the two new checks add no log site, field or span.
@task_item n=13 part=touches
decision/workspace-inherited-dependency-versions and decision/locked-builds: the sample crate gains anyrender, blitz-dom, parley and peniko, each inherited from the workspace; the lock gains four lines, no package.
@task_item n=14 part=touches
decision/dependency-audit-ci-gate, decision/licence-gate and decision/cargo-deny-audit-tool: the audit leg read the changed lock green, advisories ok and licenses ok.
@task_item n=15 part=touches
decision/capability-statement-mcp-crossing: the capability statement gained Circle Drawer, the click on a widget element and two stated limits; no pinned count moved and the MCP crossing it states is unchanged.
@task_item n=16 part=touches
decision/harness-blitz-paint-custom-widget: the custom-widget feature the harness turns on now also carries the engine's click synthesis on widget targets.
@task_fact n=1 slug=input-validation
fact: Behind the custom-widget feature the engine synthesizes the UA click on a custom-widget target after the widget hears the pointer event - pointerdown recorded, pointerup into Click on the main button or ContextMenu on the secondary, a two-count Click queuing DoubleClick - with no click walk and no focus clear, so an app's click and double-click handlers on a widget element fire; no input crosses and no carrier, verb, field or log site is added (the founder's word, F2; upstreamable, not sent).
@task_fact n=2 slug=input-validation
fact: The stand sample's Circle Drawer canvas carries its circles in one data-circles attribute that its custom widget parses as finite x,y,d triples, malformed text read as no circle without panic or log; the circles cross no carrier but the shot pixels (PROVISIONAL, the escher overseer's word).
@task_item n=1 part=untouched
The driver session socket and MCP server: their transport, request grammar, line and result bounds, idle expiry and protocol errors are unchanged.
@task_item n=2 part=untouched
Driver verbs, argument kinds, refusal causes and their JSON, session errors and the escher-session exit codes, flags and roles: none added, removed or moved.
@task_item n=3 part=untouched
Outbound requests and script fetches: fetch methods, schemes, per-host limits, CORS and same-origin absences, CSP and response bounds are not reached.
@task_item n=4 part=untouched
Secret management, environment variables, CI tokens, signing material and publish guards: no secret, env read or workflow changed.
@task_item n=5 part=untouched
Value masking of password and file inputs, the select list and its crossing, and range input rows: no form control changed.
@task_item n=6 part=untouched
Recording, kept checks, the coverage answer and the cold-agent runs: their files, owner-only directories and readers are unchanged.
@task_item n=7 part=untouched
Data protection storage: the history database, HTTP cache, cookies, TLS and clipboard are not reached.
@task_item n=8 part=untouched
Log sink, telemetry allowlist and the span line contracts: no target, field or span added.
