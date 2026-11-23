@task_prose part=chunk
Places a custom widget at its content box (upstream issue 1578): its paint is handed the content-box size, not the border-box size, and its scene lands at the content-box origin inside the element's own transform; no clip is added, a widget's scene stays unclipped as today.
Moves a custom widget's forwarded pointer events (move, down, up, cancel, leave) into the same content-box frame, the border-box origin plus left/top border and padding in CSS px, still with no transform applied; a sub-document's events keep the border-box origin.
Lets canvas text draw from the bundled fonts through the document's own font context: a new defaulted method on the widget trait hands each widget that context before it paints, and the document gains a public accessor of its font context, both behind the custom-widget feature.
Every new check is run red first (before its fix or with the fix reverted) and its red reading kept beside its green, in both layout modes; the existing canvas shot and widget layout checks are re-read green, and the stand's served tasks' readings stand.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint tree walk that hands a custom widget its paint size and scale — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: the element painter that appends a custom widget's returned scene into the frame under the element's transform — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, forwarding pointer events to a custom widget or a sub-document — edits: the custom-widget arm of handle_dom_event, the widget's event origin
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: upstream's Widget trait, the custom widget's paint and event surface behind custom-widget — edits: a new defaulted method handing the document's font context
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the document (BaseDocument), owner of the font context every glyph run is shaped from — edits: a new public font context accessor
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the engine test package's manifest — edits: parley as a dev-dependency, already in the lock
@task_file n=7 path=tests/blitz-tests/tests/stand_canvas_shot.rs mark=re-reads
what: the stand's canvas shot check, read green with the fix
@task_file n=8 path=tests/blitz-tests/tests/custom_widget_layout.rs mark=re-reads
what: the custom widget layout sizes check, read green with the fix
@task_item n=1 part=needs
A custom widget's paint size becomes its content-box size and its scene is placed at the content-box origin inside the element's transform, in both layout modes; read on the stand with padding, a border and a scale transform.
@task_item n=2 part=needs
A custom widget's pointer events (PointerMove, PointerDown, PointerUp, PointerCancel, PointerLeave) carry coordinates relative to its content-box origin in CSS px, no transform applied; a sub-document's events stay at the border-box origin. Rows on pointer event delivery, hit-testing and pointer coordinate frames bind it.
@task_item n=3 part=needs
No clip to the content box is added: a widget's scene stays unclipped, as today, unlike a replaced image.
@task_item n=4 part=needs
Canvas text is shaped from the document's own font context, the same bundled set the document's text reads, so canvas text on the stand reads the same font in both layout modes; no font file or asset enters the tree.
@task_item n=5 part=needs
Two new public items behind custom-widget (a defaulted Widget trait method and a document font context accessor), each with a doc comment, diverging from upstream and flagged upstreamable; no implementor changes.
@task_item n=6 part=needs
Canvas text leaves only as pixels in a screenshot, inside the ratified canvas crossing; it gains no other crossing.
@task_item n=7 part=needs
The screenshot's contract stands: an 800 x 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable before anything is painted, a password input painted masked, and a shot changes nothing and runs no step.
@task_item n=8 part=needs
The stand's served tasks keep their snapshot, lint and actions readings, and the committed kept checks and stand checks keep theirs unless a ruling moves one, then re-recorded, never loosened.
@task_item n=9 part=needs
Every new check is run red first and its red reading kept in evidence beside its green, every kept log path respelled with no temp or home path.
@task_item n=10 part=needs
Every new check runs in both layout modes, incremental and non-incremental.
@task_item n=11 part=needs
No new verb, argument, carrier, cause, session error, exit, log field or span; no content in a log field.
@task_item n=12 part=needs
Every build, test and CI cargo leg passes --locked; parley enters only as a dev-dependency already in the lock, no package enters it; the doc leg joins the local gate for the new public items.
@task_item n=13 part=needs
The last wrap's CI verdict, still in progress, is re-read before this chunk's push and never folded as green.
@task_standing n=1
convention: each new engine or stand test is a Rust test asserting over what the engine produces, run in the test tier its kind belongs to
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new stand check runs within the stand checks repeated three rounds in CI
home: ci-integration
@task_item n=1 part=not_needed
Accessibility tree roles, names, states, ARIA property reads and hidden-content rules: the chunk adds no node, role, name or state to the tree.
@task_item n=2 part=not_needed
Keyboard navigation, focus, Tab order and keyboard activation of form controls, selects, range inputs and the seven_guis grids and lists: the chunk changes no key handling or focus.
@task_item n=3 part=not_needed
Text contrast thresholds, lint contrast findings and stand contrast assertions: canvas pixels are not read for contrast by this chunk.
@task_item n=4 part=not_needed
Snapshot text form, snapshot diff, actions answer, handler coverage, MCP results and node id rows: the chunk adds no snapshot reading or answer field.
@task_item n=5 part=not_needed
Platform accessibility adapter and the accessibility feature's defaults: no adapter or feature default moves.
@task_item n=6 part=not_needed
Cognitive accessibility rows on invalid state and the shared tab order table: no form validation or tab order is touched.
@task_item n=7 part=not_needed
Open select list colours, motion and row heights: the chunk paints no select list.
