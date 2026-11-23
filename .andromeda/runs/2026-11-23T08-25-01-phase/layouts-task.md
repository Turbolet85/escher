@task_prose part=chunk
Fixes upstream issue 1578 in the engine: a custom widget's paint is handed its content-box size and its scene lands at the content-box origin inside the element's own transform, with no clip (F1 option A).
Moves a custom widget's pointer events into the same content-box frame its paint draws in (F1 option C), in blitz-dom's event dispatch.
Lets canvas text draw from the bundled fonts through the document's own font context: a new defaulted Widget trait method and a public BaseDocument font context accessor (F2 option A).
Adds stand checks for placement, pointer frame and canvas text, each shown red first, in both layout modes; re-reads the existing canvas shot and custom widget layout checks and the stand's standing readings.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: blitz-paint's custom widget paint pass, handing each widget its paint size and scale — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: blitz-paint's element renderer, appending a widget's scene at its frame under the element transform — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, forwarding pointer events to a custom widget — edits: handle_dom_event (its custom-widget arm and the origin it maps pointer coordinates from)
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget boundary, upstream's Widget trait a canvas paints through — edits: the Widget trait (a new defaulted method handing the document's font context)
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the document, BaseDocument and its font context — edits: a public font_ctx accessor
@task_item n=1 part=needs
A custom widget's paint is handed its content-box size times scale, not its border-box size, with padding and border alike.
@task_item n=2 part=needs
A custom widget's scene is appended at its content-box origin inside the element's own transform (M then T of the content origin), so under a CSS transform it lands where the transform puts the content box.
@task_item n=3 part=needs
A custom widget's scene stays unclipped, as today: no layer and no content-box clip is pushed for it (F1 option B not taken).
@task_item n=4 part=needs
Pointer events forwarded to a custom widget (move, down, up, cancel, leave) carry coordinates relative to its content-box origin, the border-box origin plus left and top border and padding in CSS px; no transform is applied to them, and a sub-document's events keep the border-box origin.
@task_item n=5 part=needs
The Widget trait gains one defaulted method through which blitz-paint hands each widget the document's font context before its paint, behind the custom-widget feature; no implementor changes.
@task_item n=6 part=needs
BaseDocument gains a public accessor of its font context, behind the custom-widget feature; each new public item carries its doc comment, is flagged upstreamable, and the doc leg joins the local gate.
@task_item n=7 part=needs
Canvas text is shaped from the bundled fonts through the document's own font context, so a widget's text on the stand reads the same font as the document's text; no font file or other asset enters the tree.
@task_item n=8 part=needs
Canvas text leaves a shot as pixels, inside the ratified crossing of canvas content as shot pixels; nothing of it crosses as text or data.
@task_item n=9 part=needs
An app paints the canvas from Rust through the widget's paint returning a scene; no web Canvas 2D context for page scripts and no wgpu canvas.
@task_item n=10 part=needs
The screenshot's contract stands: an 800 by 600 shot over opaque white on the pinned headless stand viewport, byte-identical across processes on one host, fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=11 part=needs
The stand's served tasks and their snapshot, lint and actions digests keep their readings, as do the committed kept checks, including the canvas shot and custom widget layout checks; a reading a ruling moves is re-recorded, never loosened.
@task_item n=12 part=needs
No new verb, argument, carrier, cause, session error, exit, log field or span; no content in a log field.
@task_item n=13 part=needs
Every build, test and CI cargo leg passes locked; parley enters only as a blitz-tests dev-dependency for a check's widget, already in the lock, and no package enters the lock.
@task_item n=14 part=needs
Every evidence log kept beside a check has every temp or home path respelled.
@task_standing n=1
convention: every new stand check is run in both layout modes, incremental and non-incremental
home: surface-desktop-native
@task_standing n=2
convention: every new check is shown with its red reading first, on the tree before its fix or with the fix reverted, kept beside its green
home: surface-desktop-native
@task_item n=1 part=not_needed
The mobile-native and web-spa surfaces: the chunk reaches no mobile layout and no web app.
@task_item n=2 part=not_needed
The agent-run and cold-agent pipe command contracts and their decisions: the chunk changes no test runner script or cold-agent pipe.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no benchmark or multi-document example's arguments or output change.
@task_item n=4 part=not_needed
The ShellProvider window chrome controls: no window decoration, maximize, minimize, drag or close behaviour is touched.
@task_item n=5 part=not_needed
The session recording and replayable-check rows: the chunk records no session and changes no recording file.
@task_item n=6 part=not_needed
The stand's task-specific page decisions (CRUD keyboard region, flight type select, load page network, order page causes) and the element ids of those tasks: no stand page or control of theirs is changed.
@task_item n=7 part=not_needed
The decisions log: it holds no recorded intent.
