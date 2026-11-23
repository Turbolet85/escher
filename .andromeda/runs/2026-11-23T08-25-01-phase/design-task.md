@task_prose part=chunk
Fixes upstream issue 1578: a custom widget's paint is handed its content-box size, not its border-box size, and its scene is appended at the content-box origin inside the element's own transform, so it covers its content box exactly with padding, border and a transform; no clip is added, the scene stays unclipped.
Moves a custom widget's pointer events (move, down, up, cancel, leave) into the same content-box frame: coordinates relative to the content-box origin (border-box origin plus left/top border and padding, in CSS px), still with no transform applied; a sub-document's events keep the border-box origin.
Lets canvas text draw from the bundled fonts through the document's own font context: a new defaulted method on the custom widget trait hands each widget that context before it paints, and the document gains a public accessor of its font context, both behind the custom-widget feature, documented and flagged upstreamable.
Adds stand checks for the placement, the pointer frame and canvas text (a test widget shaping text with parley), each run red first and kept with its red reading beside its green, in both layout modes.
Re-asserts the existing canvas shot and custom widget layout checks, the seven served tasks' readings and the screenshot's contract, unmoved by the fix.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint crate's custom widget node processing, which sizes a widget's paint and calls it — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: the paint crate's element renderer, which appends a widget's scene into the element's frame and transform — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, which forwards pointer events to a custom widget mapped from the element's origin — edits: handle_dom_event (its custom-widget arm)
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget trait, the public surface a widget implements to paint a scene and hear events — edits: the Widget trait (a new defaulted method handing the font context)
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the base document, which holds the document's font context — edits: BaseDocument (a public accessor of its font context)
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the engine test crate's manifest, its dev-dependencies — edits: parley added as a dev-dependency
@task_file n=7 path=Cargo.lock mark=edits
what: the workspace lockfile, read by every locked build — edits: the test crate's dependency list (no package enters)
@task_item n=1 part=needs
A custom widget's paint size: the widget is handed its content-box size times the scale, not its border-box size, in both layout modes.
@task_item n=2 part=needs
A custom widget's scene placement: appended at the content-box origin inside the element's own transform (the content offset applied before the transform, not in output axes), so it lands inside its content box under padding, border and a CSS transform.
@task_item n=3 part=needs
No clip on a widget's scene: the fix pushes no layer and does not clip to the content box, so what a widget paints outside it still shows, as today; hit-testing and paint grouping around the widget stay as they are.
@task_item n=4 part=needs
A custom widget's pointer event coordinates: the pointer events forwarded to a widget are given relative to its content-box origin in CSS px (border-box origin plus left and top border and padding), no transform applied; a sub-document's events keep the border-box origin.
@task_item n=5 part=needs
The custom widget trait gains a new public defaulted method through which the paint crate hands each widget the document's font context before its paint; no existing implementor changes, it is behind the custom-widget feature, carries its doc comment and is flagged as diverging from upstream.
@task_item n=6 part=needs
The document gains a public accessor of its font context behind the custom-widget feature, with its doc comment; app code then holds a shared, mutable handle to the document's font collection.
@task_item n=7 part=needs
Canvas text is shaped from the document's own font context, the same bundled set every glyph run of the document's text is shaped from, so canvas text on the stand reads the same font as the document's text in both layout modes; no font file or other asset enters the tree.
@task_item n=8 part=needs
The document's font context as configured by the native configuration's font context (and the bundled context on WASM) is the one a widget receives; the chunk adds no separate font context for canvas text.
@task_item n=9 part=needs
The screenshot's contract stands: an 800 x 600 shot over opaque white, byte-identical across processes on one host, refused with fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot; canvas text leaves as pixels in the shot and the screenshot never paints with text skipped.
@task_item n=10 part=needs
The driver's commands, exit statuses, session errors and log fields are unchanged: no new verb, argument, carrier, cause, session error, exit, log field or span, and no content in a log field.
@task_item n=11 part=needs
The seven served tasks' snapshot, lint and actions readings and the committed kept and stand checks keep their readings, re-recorded never loosened if a ruling moves one; the canvas shot and custom widget layout checks read green under the fix.
@task_item n=12 part=needs
Each new check is run red first, on the tree before its fix or with the fix reverted, and its red reading kept beside its green in the chunk's evidence, every path in a kept log respelled with no temp or home path.
@task_item n=13 part=needs
Every build, test and CI cargo leg passes locked; parley enters only as the test crate's dev-dependency, already in the lockfile, and the doc leg joins the local gate for the two new public items.
@task_standing n=1
convention: each new test that paints or shapes text is built under the tests' own font build and draws from the bundled fonts, never from fonts of the host
home: typography
@task_standing n=2
convention: each new screenshot check reads an 800 x 600 shot over opaque white, byte-identical across processes on one host, in both layout modes
home: surface
@task_item n=1 part=not_needed
Colour palette rows (engine colours, colour scheme, select list colours, colour tokens): the chunk sets no colour; the shot's white ground is held by the screenshot contract.
@task_item n=2 part=not_needed
Spacing rows (body margins, select list placement, table cellspacing and cellpadding hints): the chunk reads a widget's border and padding as laid out and changes no spacing rule.
@task_item n=3 part=not_needed
Motion rows (virtual clock, driver advance, scroll behaviour, scrollbar fade, select list motion, reduced motion): the chunk animates nothing and moves no clock.
@task_item n=4 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon, set cursor): the chunk sets no cursor or icon.
@task_item n=5 part=not_needed
Select list rows (its overlay, open gestures, list events, accessibility, its row in the snapshot and the shot): the chunk touches no select.
@task_item n=6 part=not_needed
The WPT runner's output and result colours: the chunk adds no WPT test and changes no runner output.
@task_item n=7 part=not_needed
The shell provider and window rows (maximize, minimize, decorations, window drag, compositing alpha mode) and the JS window's sizes and device pixel ratio: the chunk changes no window or script surface.
@task_item n=8 part=not_needed
The web-spa surface and a web Canvas 2D context or wgpu canvas: no page script gets a canvas context and no wgpu canvas is built in this version.
@task_item n=9 part=not_needed
Typography rows on presentational and cascade font rules (font element face and size, lang locale, generic family fallback, root-relative units, unavailable family fallback, document.fonts): the chunk changes no font selection or cascade, only who reads the document's context.
@task_item n=10 part=not_needed
The escher-session actions, coverage, graph and layout answers and the MCP role's results: the chunk adds no verb or answer and changes none.
