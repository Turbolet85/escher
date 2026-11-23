@task_prose part=chunk
Places a custom widget at its content box (upstream issue 1578): its paint is handed the content-box size and its scene lands at the content-box origin inside the element's own transform; the scene stays unclipped, as today.
Moves the pointer events forwarded to a custom widget into the same content-box frame its paint draws in; a sub-document's events and the untransformed event mapping stay as they are.
Hands each widget the document's own font context before it paints, through a new defaulted method on the Widget trait and a public BaseDocument accessor of its font context, so canvas text is shaped from the bundled fonts the document's text uses, in both layout modes.
Every new check is run red first, before its fix or with the fix reverted, and that red reading is kept beside its green; the standing widget checks are re-read green under the fix.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: blitz-paint's paint entry, the custom widget's paint call and the size handed to it — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: blitz-paint's element renderer, the custom widget's scene appended into the paint scene under the element's transform — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's DOM event dispatch, events forwarded to sub-documents and custom widgets with their coordinates — edits: handle_dom_event's custom-widget arm
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: blitz-dom's Widget trait, the custom widget's paint and event surface behind the custom-widget feature — edits: the Widget trait, a new defaulted method handing the font context
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: BaseDocument, the document core and its font context — edits: a new public font context accessor
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the blitz-tests manifest, its one test table and its dev-dependencies — edits: the dev-dependencies, parley added
@task_file n=7 path=tests/blitz-tests/tests/all.rs mark=re-reads
what: the blitz-tests single test target, home of the stand canvas shot check and the custom widget layout check re-read green under the fix
@task_item n=1 part=needs
The Widget trait's paint receives the widget's content-box size times scale, not its border-box size; with padding, a border or neither, what a widget paints covers its content box exactly.
@task_item n=2 part=needs
The widget's scene is appended at its content-box origin inside the element's own transform (the transform then the content-box offset), not offset in output axes; scale and the content origin share units at the append, the frame being built at scale.
@task_item n=3 part=needs
No clip is pushed for a widget's scene: it stays unclipped, unlike a replaced image clipped to its content box, so hit-testing and paint-clip rules over replaced content are not extended to widgets.
@task_item n=4 part=needs
Pointer events forwarded to a custom widget (pointer move, down, up, cancel and leave, the variants mapped to a UiEvent) carry coordinates relative to the widget's content-box origin: the node's absolute position plus its left and top border and padding, in CSS px; a transform stays unapplied, and events forwarded to a sub-document keep the border-box origin.
@task_item n=5 part=needs
A new defaulted method on the Widget trait hands each widget the document's own font context before its paint; blitz-paint calls it; no Widget implementor changes; it stands behind the custom-widget feature.
@task_item n=6 part=needs
BaseDocument gains a public accessor of its font context, behind the custom-widget feature, so app code holds a shared, mutable handle to the document's font collection; a crate internal made public by name, a new item of BaseDocument's public surface.
@task_item n=7 part=needs
Both new public items diverge from upstream: direct engine changes in blitz-dom and blitz-paint flagged upstreamable with the placement fix, each kept as an addition of escher's own where it can be.
@task_item n=8 part=needs
The placement fix and the event frame are an engine behaviour change for every document built with the custom-widget feature, ratified by the founder's ruling; the custom widget embedded as an object element keeps its object data and layout sizes.
@task_item n=9 part=needs
Canvas text is shaped from the document's own font context, the bundled fonts every glyph run of the document is shaped from; on the stand it reads the stand's bundled DejaVu Sans, the same in both layout modes; no font file or other asset enters the tree.
@task_item n=10 part=needs
Canvas content, its text included, crosses only as pixels in a shot, inside the ratified canvas crossing; no new verb, argument, carrier, refusal cause, session error, exit, log field or span is added, and no content enters a log field.
@task_item n=11 part=needs
The screenshot's contract stands: 800 by 600 over opaque white, painted on the Vello CPU renderer with the harness's custom-widget feature on, byte-identical across processes on one host, fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=12 part=needs
The stand's served tasks keep their snapshot, lint and actions digests and the committed kept checks and stand checks keep their readings; the stand canvas shot check and the custom widget layout check read green under the fix, re-recorded only where a ruling moves one, never loosened.
@task_item n=13 part=needs
A check's widget shapes text with parley, a blitz-tests dev-dependency taken from the workspace's shared dependencies; it is already in the lock, so no package enters it; every build, test and CI cargo leg passes locked.
@task_item n=14 part=needs
Each new control is measured on the stand red first, on the tree before its fix or with the fix reverted, and its red reading is kept in the chunk's evidence beside its green, every path in a kept log respelled with no temp or home path.
@task_item n=15 part=needs
The two new public items carry their item docs, so the rustdoc leg joins the local pre-push gate and rustdoc warnings stay errors.
@task_item n=16 part=needs
The last wrap's CI verdict, unread and never folded as green, is re-read before this chunk's push.
@task_standing n=1
convention: every new stand or integration check runs in both layout modes, incremental and non-incremental
home: conventions
@task_standing n=2
convention: new integration tests live in blitz-tests' single test target, one behaviour per file, shared helpers in non-target modules
home: conventions
@task_standing n=3
convention: a new test asserts its fixture's condition first and carries a descriptive name
home: conventions
@task_standing n=4
convention: a regression test of an upstream defect cites the upstream issue
home: conventions
@task_standing n=5
convention: every new public item carries its doc, and rustdoc warnings are errors
home: conventions
@task_standing n=6
convention: optional functionality stays behind its cargo feature, and a dependency is declared once in the workspace
home: conventions
@task_standing n=7
convention: expect and unwrap in new code carry a justification; clippy allows stay local
home: conventions
@task_item n=1 part=not_needed
The driver's session socket wire, MCP server protocol, session errors, exit codes and command-line roles: the chunk adds no verb, call or answer and changes none of them.
@task_item n=2 part=not_needed
Accessibility tree and ARIA state contracts: a widget's placement, events and font context reach no accessibility node or state.
@task_item n=3 part=not_needed
The select list, select readers and form control readers and their default actions: no form control is touched.
@task_item n=4 part=not_needed
The snapshot, its node keys and text form, stable element ids, the snapshot diff and the UI graph: canvas content crosses only as pixels, never as nodes.
@task_item n=5 part=not_needed
The WPT runner, its result kinds, statuses, outputs and CI artifacts: no WPT test or runner behaviour is changed.
@task_item n=6 part=not_needed
The browser app, its history persistence, about pages and toolbar: the chunk works in the engine and the stand alone.
@task_item n=7 part=not_needed
Page script and the JS runtime, its globals and fetchers: no web Canvas 2D context is given to scripts in this version.
@task_item n=8 part=not_needed
Networking, the net provider, the HTTP cache and per-host request caps: a widget reaches no network.
@task_item n=9 part=not_needed
Cold-agent and agent-run scripts, their events, verdicts and exit codes: no live agent run is part of this chunk.
@task_item n=10 part=not_needed
Layout algorithm rows (Taffy, tables as grid, flex and alignment fallbacks): a widget's layout sizes are read, never changed.
@task_item n=11 part=not_needed
Scrolling, scroll into view, smooth scroll and wheel handling: a widget's event frame moves by its border and padding only.
@task_item n=12 part=not_needed
A canvas shot's byte equality across Linux, macOS and Windows, and Circle Drawer on a canvas: both are later entries.
@task_item n=13 part=not_needed
A canvas read as draw commands, canvas lint, virtual nodes, drag, pan and zoom: Epochs 4 and 5.
@task_item n=14 part=not_needed
The CPU renderer's custom-paint gap for textures and resources, and a wgpu canvas: out of this version.
