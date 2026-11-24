@task_prose part=session
The stand's Circle Drawer now paints its circles and each circle's diameter as text from Rust, on one custom-widget object element (role group, named Drawing area, keyed), in place of its former div canvas and its .circle DOM nodes and CSS rules, which are retired; the circles cross the carriers as shot pixels alone, read by the widget from one data-circles attribute.
The engine gained one direct change behind the custom-widget feature: on a custom-widget target, after the widget's own handle_event and the focus set, a pointerdown is recorded and a pointerup synthesises the UA click (main button) or contextmenu (secondary), and a second click queues a double click; no click walk, no focus clear; an app's onclick and ondoubleclick on a widget element now fire, the widget still hearing every pointer event first. Founder-ratified, flagged upstreamable, nothing sent.
The seven_guis stand gained an in-process Circle Drawer boot served by no escher-session role, and the sample crate gained four workspace dependencies (anyrender, blitz-dom, parley, peniko); the plan's three was disproved by measurement, blitz-dom being needed by the Widget paint signature.
Measured: a widget paints in its content box while the client rect is the border box, so the app reads a click point against the content box (2 px border subtracted; first circle 348,198 not 350,200); the Vello CPU renderer snaps glyph origins differently on Apple hosts, so canvas text is byte-equal across linux, windows and macOS only with glyph origins on whole device pixels, and the label is now drawn that way.
Circle Drawer's canvas element shot joined the cross-OS digest table (a second table, SAMPLE), read green on all three CI hosts after the snap fix; the non-lean measured and clickable readings for Circle Drawer moved 1 to 0; the capability statement gained Circle Drawer, the click on a widget element and two stated limits (A click lands at an element's centre; Canvas text across hosts).
No verb, argument, carrier, field, refusal cause, session error, exit, log field or span was added; the overseer's answers (diameter text, centre click, role group, data-circles, the fourth dependency, the snap and its stated limit) stand PROVISIONAL.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: edited — blitz-dom's pointer event handling and default actions: pointerdown recording, pointerup to click and contextmenu, double-click queuing; gains the click synthesis on a custom-widget target
@task_file n=2 path=packages/blitz-dom/src/events/mod.rs
what: edited — blitz-dom's DOM event dispatch (handle_dom_event), its custom-widget arm that forwards events to widgets
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: edited — the 7GUIs stand's Circle Drawer task: its canvas, now a custom widget painting circles and diameter text, its click and double-click handlers, undo and redo, the resize dialog
@task_file n=4 path=examples/seven_guis/src/stand.rs
what: edited — the seven_guis headless stand: its boots of each task in process (seven_guis::stand), gains the Circle Drawer boot
@task_file n=5 path=examples/seven_guis/Cargo.toml
what: edited — the seven_guis sample crate's manifest and its workspace-declared dependencies
@task_file n=6 path=Cargo.lock
what: edited — the workspace lock file; seven_guis' entry gains four dependency lines, no package added
@task_file n=7 path=docs/capabilities.md
what: edited — the capability statement: its canvas claim, witnesses, stated measured limits and proven section
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: added — an integration check in blitz-tests, both layout modes: a click and a double click on a custom-widget canvas reach the app's click and double-click handlers while the widget hears every pointer event (the engine change's guard)
@task_file n=9 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: added — an integration check in blitz-tests, both layout modes: Circle Drawer boots with a named, keyed canvas and no lint finding; a click paints a circle and its diameter; undo, redo and the resize dialog repaint the canvas
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: edited — the cross-OS canvas shot digest check: canvas element shots pinned by digest, identical on every CI OS; gains a second table SAMPLE holding Circle Drawer's canvas
@task_file n=11 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: edited — the stand's non-lean task readings check: per-task measured and clickable counts pinned
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: edited — the record store's journal for this chunk, one line of the operator pass record committed for a clean tree
@task_item n=1 part=touches
decision/direct-engine-changes-flagged-upstreamable: a new direct engine change, the click synthesis on a custom-widget target, flagged upstreamable and not sent, joins its tally (founder-ratified, F2 of the forks)
@task_item n=2 part=touches
Engine-change tallies such as decision/a11y-input-focus-engine-changes and decision/hit-focus-form-engine-changes: another engine change in the input path (click, contextmenu and double click synthesised on widget targets) may belong in their count or beside them
@task_item n=3 part=touches
decision/sub-document-event-forwarding: events forwarded to widgets no longer end at the widget; after the widget's handle_event the engine now runs its click synthesis for that target
@task_item n=4 part=touches
decision/feature-gated-widgets and decision/harness-paints-custom-widgets: the new synthesis sits behind the custom-widget feature, reaching every document behind it (every Dioxus native app)
@task_item n=5 part=touches
contract/nodes-widgets/widget-trait and contract/nodes-widgets/widget-fonts: a Widget's handle_event still hears every pointer event first; the stand now implements the trait in an app (attribute_changed, fonts, paint), needing blitz-dom's ComputedStyles in the paint signature
@task_item n=6 part=touches
Pointer-derived event contracts (contract/dom-events/pointer-events, contract/event-dispatch/pointer-compatibility-events, contract/event-dispatch/targeting): pointerup on a widget target now yields click or contextmenu, without a click walk and without clearing focus
@task_item n=7 part=touches
decision/double-click-window and decision/repeated-click-window: two clicks with no advance on a widget target now queue a double click; after an advance of 600 ms a third click counts single
@task_item n=8 part=touches
decision/handlers-before-default-action and decision/default-action-event-order: on a widget target the widget's own handler runs, then the focus set, then the synthesised UA events
@task_item n=9 part=touches
decision/custom-widget-object and contract/custom-widget-embed/object-data: Circle Drawer's canvas is now an object element whose data is a custom widget attribute, with role group and an aria-label
@task_item n=10 part=touches
decision/custom-widget-content-box-paint and decision/client-rect-own-box: measured consequence; the client rect is the border box while the widget paints in its content box, so an app placing widget content from a click point subtracts border and padding (2 px here)
@task_item n=11 part=touches
decision/canvas-crosses-as-pixels: Circle Drawer's circles left the DOM, snapshot, tree, graph and lint, and cross as shot pixels alone, now including canvas text
@task_item n=12 part=touches
contract/headless-stand/boot: seven_guis::stand gains a Circle Drawer boot, in process only, served by no escher-session role
@task_item n=13 part=touches
decision/session-apps-one-table: Circle Drawer is booted in process and is not added to the session's app table
@task_item n=14 part=touches
decision/no-screenshot-in-cross-os-byte-check and decision/stand-readings-same-on-every-ci-os: the canvas-only exception and its digest table now hold a canvas that paints text, byte-equal on linux, windows and macOS only by drawing glyph origins on whole device pixels; a second table of two rows joins the first of twelve
@task_item n=15 part=touches
decision/screenshots-render-on-cpu and decision/headless-screenshot-cpu-renderer: the Vello CPU renderer snaps glyph origins on Apple hosts (a 1/3 px x and a whole-pixel baseline) and to 1/4 px x elsewhere, so shot text differs by host unless the app snaps
@task_item n=16 part=touches
decision/capability-statement, contract/capability-statement/sections and decision/measured-limits-stated-in-the-tool: the canvas claim gains Circle Drawer and the click on a widget element, two witnesses, and two stated limits; no pinned count moved
@task_item n=17 part=touches
Actionable and clickable readings (decision/one-actionable-rule, decision/actionable-elements-keyed, contract/snapshot/clickable, contract/dioxus-bridge/clickable): Circle Drawer's canvas reads clickable, role group, keyed, and the stand's unkeyed actionable list stays empty
@task_item n=18 part=touches
contract/handler-coverage/canvas: the canvas's onclick and ondoubleclick now sit on a custom-widget object element and are reached through the engine's synthesis
@task_item n=19 part=touches
decision/workspace-dependencies and decision/anyrender-rendering-abstraction: the sample crate now depends directly on anyrender, blitz-dom, parley and peniko, each declared once in the workspace
@task_item n=20 part=touches
decision/stand-typography-dejavu-sans and decision/stand-uses-bundled-font: the canvas label is painted at 12 px sans-serif through the fonts the widget is handed, a new text surface on the stand
@task_item n=21 part=touches
decision/new-carrier-widens-boundary and decision/upstream-behaviour-changes-ratified-per-take: the engine change alters behaviour for every widget document without adding a carrier; a picker should check whether the boundary-widening or ratification rules name it
@task_fact n=1 slug=established-decisions
fact: On a custom-widget target the engine, behind the custom-widget feature, synthesises the UA click on a main-button pointerup, contextmenu on a secondary one and a double click on a second click, after the widget's own handle_event and the focus set, with no click walk and no focus clear; the widget still hears every pointer event first; a direct engine change flagged upstreamable, the founder's ruling, guarded by stand_canvas_click.
@task_fact n=2 slug=standard-contracts
fact: Text an app paints on a canvas is byte-equal across hosts only with its glyph origins on whole device pixels, because the CPU renderer snaps glyph origins differently on Apple hosts; stated as the limit Canvas text across hosts and pinned by the cross-OS canvas digest check (PROVISIONAL, the escher overseer's word).
@task_fact n=3 slug=established-decisions
fact: A widget paints in its content box while an element's client rect is its border box, so an app that places widget content from a click point reads that point against the content box, subtracting border and padding.
@task_fact n=4 slug=standard-contracts
fact: A driver click lands at the centre of its target element, so an agent can place content on a canvas only at its centre until a drag verb exists; stated as the limit A click lands at an element's centre (PROVISIONAL, the escher overseer's word).
@task_fact n=5 slug=established-decisions
fact: No keyboard path into a custom-widget canvas: the Circle Drawer canvas is reached by pointer alone, the keyboard path a later entry.
@task_item n=1 part=untouched
The session socket, MCP role, command line, kept checks and their wire: no verb, argument, carrier, field, refusal cause, session error or exit code was added or changed.
@task_item n=2 part=untouched
Telemetry, tracing and logging rows: no log field, span or log site was added; the new checks are silent.
@task_item n=3 part=untouched
Style, layout and paint-pipeline rows (Stylo, Taffy, Parley layout, damage, incremental layout, paint tree, select list painting): unchanged.
@task_item n=4 part=untouched
The WPT runner, its outputs and CI contracts, and the browser app with its history, persistence and about pages: not reached.
@task_item n=5 part=untouched
Snapshot text, diff, answer JSON, UI graph and stable element id tiers: no format or rule changed; Circle Drawer's circles simply no longer appear in them.
@task_item n=6 part=untouched
Names: no crate, binary, server or environment variable was taken or freed.
@task_item n=7 part=untouched
CI legs, gates, coverage floors, toolchain and dev-tool rows: no leg, threshold or tool version moved.
@task_item n=8 part=untouched
The JavaScript runtime (Boa), script fetching and networking rows: not reached.
