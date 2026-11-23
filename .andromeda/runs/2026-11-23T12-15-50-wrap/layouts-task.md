@task_prose part=session
The session fixed where a custom canvas widget paints (upstream #1578): its paint size is now its content box times the scale (the border box less border and padding, floored at 0), and its scene is drawn at the content origin inside the element's own transform, still unclipped.
A widget's pointer events now arrive in its content-box frame (the position less its left and top border and padding, in CSS px); a sub-document's pointer forwarding is unchanged.
The widget trait gained a defaulted method that hands each widget the document's own shared font context before each paint, and the document gained a public accessor for that font context, so canvas text can be shaped from the bundled fonts; no existing widget changed.
Three new headless-stand checks witness these, in both layout modes and under the embolden feature: content-box placement, the pointer frame and canvas text.
The lock moved cookie 0.18.2 to 0.18.3 for RUSTSEC-2026-0594 by hand, and the test crate took parley as a dev-dependency; no package added, no capability claimed, nothing retired.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: the paint pipeline's custom-widget scene building, which sizes each widget's canvas and hands it fonts before paint
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: the element renderer that places a custom widget's scene within the element's transform
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: the DOM event dispatch that forwards pointer events to custom widgets and sub-documents with adjusted coordinates
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait, the contract a canvas widget implements to paint and hear events
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the base document, owner of the shared font context and the node tree
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: the integration test crate's manifest, the list of its dependencies and dev-dependencies
@task_file n=7 path=Cargo.lock
what: the workspace lock file, the pinned version of every dependency
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: a new headless-stand check that a padded or transformed canvas widget paints its content box at the content origin
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: a new headless-stand check that a padded canvas widget hears pointer events from its content box
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: a new headless-stand check that canvas text is shaped from the document's own bundled fonts
@task_item n=1 part=touches
The circle drawer's canvas element id: that canvas, as a widget, now paints at its content box and hears pointer positions relative to its content box, so a padded or bordered canvas places circles and clicks differently than before.
@task_item n=2 part=touches
The headless stand's pinned viewport: three new checks boot on the headless stand in both layout modes; the pinned viewport size itself is unchanged and any row holding its values stays true.
@task_item n=3 part=touches
Visible region through clipping boxes: a widget's scene is now drawn inside its element's transform and stays unclipped, so a rule on what region is visible or clipped for a canvas may read differently.
@task_item n=4 part=touches
The session's screenshot answer and the MCP screenshot result: a canvas widget's pixels in a shot now land in its content box, under its border and padding and inside its transform, rather than at the border-box origin.
@task_item n=5 part=touches
The session's actions answer: a pointer action that lands on a canvas widget now reaches the widget in its content-box frame; actions on a sub-document are forwarded as before.
@task_item n=6 part=touches
The fonts-unavailable refusal: canvas widgets can now shape text from the document's own font context handed before each paint; no refusal is added or renamed, but a row tying canvas text to font availability is beside this.
@task_item n=7 part=touches
The session's layout answer: the box a layout reading reports for a canvas widget is now exactly the size the widget is handed for paint (content box times scale); the layout reading itself is unchanged.
@task_item n=8 part=touches
Any row on custom canvas widgets as a capability of a page: the widget contract gained a defaulted fonts method and the document a public font-context accessor, so app code holds a shared, mutable handle to the document's fonts.
@task_item n=9 part=touches
Open list rows read their own bounds and into-view scrolling: only where such a row reads a canvas widget's bounds or position; the session changed a widget's paint and pointer frame, not element bounds.
@task_item n=10 part=touches
Stand names, roles and states by attributes alone and stand ids as attributes alone: the new checks mount their own canvas components and add no stand control, id or role; flag only where a row counts the stand's checks.
@task_item n=1 part=untouched
The shell provider window chrome controls (drag, maximize, minimize, close, decorations): no window chrome behaviour changed.
@task_item n=2 part=untouched
The stand element ids for the counter, temperature converter, flight booker, timer, CRUD, cells, load and order tasks, and their task cards: no stand page, control or id was added, renamed or removed.
@task_item n=3 part=untouched
The agent-run, cold-agent and multi_doc command lines, and the paint_bench arguments and backend: no argument, exit code, output or environment variable changed.
@task_item n=4 part=untouched
The session command line, its exit codes, help, graph, lint, snapshot, coverage role, recording and served tasks, and the MCP server's protocol revisions, tools, stdio, end and result bound: no verb, field, exit, refusal or tool was added or moved.
@task_item n=5 part=untouched
The cold-agent decisions, the no-caller-input-echoed decision and session recording as a replayable check: no agent, prompt or recording path changed.
@task_item n=6 part=untouched
The fixed document skeleton, the load page's network provider, the flight type select boot reading, the CRUD list keyboard region, the order page controls, the stand accent colour and lints clean at boot: no page structure, style or boot path changed.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates, and the mobile-native and web-spa surfaces: nothing in the session reaches them.
