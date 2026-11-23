@task_prose part=session
The session moved a custom widget's painted surface to its content box (upstream issue 1578): the paint size handed to the widget is the border box less border and padding, times scale, and its scene is placed at the content origin inside the element's own transform, unclipped.
A custom widget now hears pointer events in its content-box frame (the position offset by its left and top border plus padding, in CSS px); the sub-document forwarding arm is unchanged.
A custom widget is now handed the document's own shared font context before each paint, through a new defaulted Widget::fonts method and a public BaseDocument::font_ctx accessor, so canvas text shapes from the same fonts as the document's text, the bundled route included; no existing widget implementor changed.
Three new integration checks witness it: stand_canvas_placement, stand_canvas_pointer_frame and stand_canvas_text, each in both layout modes, the text check also under the font-embolden feature.
The lock moved cookie 0.18.2 to 0.18.3 for RUSTSEC-2026-0594 by a two-line edit, and blitz-tests took parley as a dev-dependency; no package, manifest feature, verb, exit status, log field or carrier was added.
@task_file n=1 path=packages/blitz-paint/src/lib.rs
what: the paint crate's custom-widget scene building: the paint size a widget is handed and the font context handed to it before it paints
@task_file n=2 path=packages/blitz-paint/src/render.rs
what: the element renderer that draws a custom widget's scene into the page, now at the content origin inside the element's transform
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs
what: DOM event dispatch that forwards pointer events to a custom widget or a sub-document, with coordinates in the receiver's own frame
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait a canvas-style widget implements (paint, events, and now the font-context handoff)
@task_file n=5 path=packages/blitz-dom/src/document.rs
what: the base document, which owns the document's shared font context and now exposes it to widgets
@task_file n=6 path=tests/blitz-tests/Cargo.toml
what: the engine integration-test crate's manifest, now taking the text layout crate as a dev-dependency
@task_file n=7 path=Cargo.lock
what: the workspace lockfile: the cookie patch move and the test crate's new dependency line
@task_file n=8 path=tests/blitz-tests/tests/stand_canvas_placement.rs
what: the check that a padded or transformed widget paints its content box at the right pixels and size
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_pointer_frame.rs
what: the check that a padded widget hears pointer positions from its content-box origin
@task_file n=10 path=tests/blitz-tests/tests/stand_canvas_text.rs
what: the check that canvas text is shaped from the document's own fonts and matches the bundled-font route in a screenshot
@task_item n=1 part=touches
The typography contract on the dioxus-native configuration's FontContext: the one font context the document is built with now also reaches custom widgets, handed to each before paint, so that contract gains a second consumer.
@task_item n=2 part=touches
The typography decision on the bundled font context on WASM and the WASM font-context configuration contract: widget text now shapes from the same document context, so whatever fonts the document resolves (bundled or system) are the widget's fonts too.
@task_item n=3 part=touches
The typography decisions that tests and the test engine are built with system fonts: the new canvas text check runs on that build and compares the document route byte-equal to the bundled route, in both layout modes and under font-embolden.
@task_item n=4 part=touches
The typography decision that a screenshot never paints with text skipped: the canvas text check reads text a widget drew into a screenshot region as non-blank, a new witness of text in a shot.
@task_item n=5 part=touches
The decisions that unavailable families fall back to the default font and that no generic family lays out as sans-serif: canvas text now resolves families through the same document context, so those fallbacks now govern widget text as well (indirect, through the shared context).
@task_item n=6 part=touches
The depth-strategy decision that a stacking-context root groups paint and hit-testing: a widget's scene is now appended inside the element's transform rather than outside it, so a transformed element's widget paints inside that transform, unclipped.
@task_item n=7 part=touches
The depth-strategy decisions that hit-testing stops at a paint clip: the widget's scene stays unclipped while its pointer frame moved to the content box; hit-testing itself is unchanged, but the coordinates a widget hears are now content-box relative.
@task_item n=8 part=touches
The screenshot verb contracts on the desktop-native surface: a shot of a page with a padded custom widget now shows the widget's paint inside its content box, padding and border left to the element's own background.
@task_item n=9 part=touches
The spacing area: padding and border around a custom widget now lie outside its painted surface and outside its pointer frame, a box-model rule no spacing row states yet.
@task_fact n=1 slug=typography
fact: A custom widget is handed the document's own shared font context before each paint, so text it draws is shaped from the same fonts as the document's text, the bundled route included; a widget that ignores it keeps its own fonts.
@task_fact n=2 slug=spacing
fact: A custom widget paints and hears pointer events in its content box: its paint size is the border box less border and padding times scale, its scene sits at the content origin inside the element's transform, unclipped, and pointer positions are offset by its left and top border plus padding.
@task_item n=1 part=untouched
The color-palette area: no colour, colour token, base colour or color-scheme handling moved; the widget fills in the checks are test fixtures, not engine colours.
@task_item n=2 part=untouched
The motion areas and the virtual clock: no animation, scroll behaviour, clock or driver verb timing changed.
@task_item n=3 part=untouched
The select list rows (overlay, colours, events, accessibility, placement): the select list is not a custom widget and none of its paint or events moved.
@task_item n=4 part=untouched
The escher-session command, exit status, exit code, stdout, stderr, mcp role, tasks, actions, coverage and UI graph contracts: no verb, argument, answer, exit or carrier was added or changed.
@task_item n=5 part=untouched
The web-spa surface's page canvas on wasm32: the canvas the dioxus-native surface appends to the page body is not the custom widget canvas, and its sizing from host CSS is unchanged.
@task_item n=6 part=untouched
The iconography area: cursor icons, cursor resolution and favicon reading are unchanged.
@task_item n=7 part=untouched
The WPT runner output and result colour contracts, and the shell provider window contracts: nothing in the runner or window chrome moved.
@task_item n=8 part=untouched
The absences of a project type scale and of engine colour tokens: the session adds neither a type scale nor tokens; it only shares the existing font context.
