@task_prose part=chunk
Fixes upstream issue 1578 in the engine: a custom widget's paint is handed its content-box size and its scene is appended at the content-box origin inside the element's own transform; its scene stays unclipped, as today (founder ruling F1, options A and C).
Moves a custom widget's pointer events into the same content-box frame: the coordinates forwarded to the widget start from its border-box origin plus its left and top border and padding, in CSS px; no transform is applied to them; a sub-document's events are unmoved.
Lets canvas text draw from the bundled fonts through the document's own font context (founder ruling F2, option A): a new defaulted method on the custom widget trait, through which the paint crate hands each widget the font context before it paints, and a new public font-context accessor on the document, both behind the custom-widget feature and flagged upstreamable; no font file or asset enters the tree.
Adds stand checks for placement, pointer frame and canvas text, each run red first and kept with its red and green readings, in both layout modes; re-reads the existing canvas-shot stand check and the custom widget layout check green, and re-reads the last wrap's CI verdict before the push.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint crate's walk over custom widget nodes, which hands a widget its paint size and scale — edits: process_custom_widget_node
@task_file n=2 path=packages/blitz-paint/src/render.rs mark=edits
what: the paint crate's element renderer, which appends a custom widget's returned scene under the element's transform at its content-box frame — edits: draw_custom_widget
@task_file n=3 path=packages/blitz-dom/src/events/mod.rs mark=edits
what: blitz-dom's event dispatch, which maps pointer events to a custom widget's UI events — edits: handle_dom_event (its custom-widget arm and the origin it hands map_dom_event_to_ui_event)
@task_file n=4 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget trait, upstream's public Widget surface a widget paints through — edits: Widget (a new defaulted method taking the document's font context)
@task_file n=5 path=packages/blitz-dom/src/document.rs mark=edits
what: the document, which holds its own font context, the one every glyph run in it is shaped from — edits: BaseDocument (a new public font_ctx accessor)
@task_file n=6 path=tests/blitz-tests/Cargo.toml mark=edits
what: the unpublished test harness crate's manifest and its dev-dependencies — edits: dev-dependencies (parley added, already in the lockfile)
@task_file n=7 path=tests/blitz-tests/tests/stand_canvas_shot.rs mark=re-reads
what: the canvas headless shot stand check, re-read green under the fix
@task_file n=8 path=tests/blitz-tests/tests/custom_widget_layout.rs mark=re-reads
what: the custom widget layout check, re-read green under the fix
@task_item n=1 part=needs
A custom widget's paint size and placement: it receives its content-box size and its scene lands at the content-box origin inside the element's own transform, so what it paints covers its content box with padding, border and a transform alike.
@task_item n=2 part=needs
A custom widget's scene stays unclipped, as today: the fix adds no clip or layer to its content box.
@task_item n=3 part=needs
Pointer events forwarded to a custom widget (move, down, up, cancel, leave) carry coordinates relative to its content-box origin in CSS px; a transform stays unapplied to them and a sub-document's events keep the border-box origin.
@task_item n=4 part=needs
An engine change for every document behind the custom-widget feature, remote pages included where the feature is built: the class ratified per change, as the custom widget's existing engine rules (its initial attribute replay) were.
@task_item n=5 part=needs
Two new public items diverging from upstream, a defaulted widget trait method and a public accessor of the document's font context, both behind the custom-widget feature, each with its doc comment; this widens a public surface and passes the boundary-widening rule as ruled by the founder.
@task_item n=6 part=needs
App code gains a shared, mutable handle to the document's font collection: the measured cost of the F2 ruling, which the widget method and accessor carry.
@task_item n=7 part=needs
Canvas text is shaped from the document's own font context and leaves only as pixels in a shot, inside the ratified crossing for canvas content; no new carrier, crossing or content class.
@task_item n=8 part=needs
No font file or asset enters the tree: canvas text shapes from the stand's existing bundled font bytes.
@task_item n=9 part=needs
The screenshot's contract stands: an 800 by 600 shot over opaque white, byte-identical across processes on one host, fonts-unavailable before anything is painted, a password input painted masked, no step run and nothing changed by a shot.
@task_item n=10 part=needs
No new verb, argument, carrier, refusal cause, session error, exit, log field or span; no content in a log field.
@task_item n=11 part=needs
The stand's served tasks keep their snapshot, lint and actions digests, and the committed kept checks and stand checks their readings, unless a ruling moves one, then re-recorded, never loosened.
@task_item n=12 part=needs
The test harness turns on the paint crate's custom-widget feature; the new checks' widgets are built through it.
@task_item n=13 part=needs
parley enters as a direct dev-dependency of the unpublished test harness crate, a package already in the lockfile, so no package enters the lock; every build, test and CI cargo leg passes the locked flag.
@task_item n=14 part=needs
The doc leg joins the local gate because the new public items carry doc comments.
@task_item n=15 part=needs
The last wrap's CI verdict is unread and is re-read before this chunk's push, never folded as green.
@task_item n=16 part=needs
Each new check is run red first, on the tree before its fix or with the fix reverted, and that red reading is kept in the chunk's evidence beside its green.
@task_item n=17 part=needs
Every kept evidence log has its temp and home paths respelled: no raw log with a temp or home path is saved into evidence.
@task_item n=18 part=needs
The canvas is painted from Rust through the scene API: no web Canvas 2D context for page scripts and no wgpu canvas; the CPU renderer's custom-paint gap is left as it is.
@task_standing n=1
convention: each new stand check runs in both layout modes, incremental and non-incremental
home: input-validation
@task_standing n=2
convention: each new test builds and runs under locked builds with workspace-inherited dependency versions, in the unpublished test harness crate
home: dependency-security
@task_standing n=3
convention: each kept evidence log holds no credentials and no temp or home path
home: secret-management
@task_item n=1 part=not_needed
The MCP server's protocol, request and result bounds, protocol errors and tool results: the chunk adds no tool and changes no MCP line.
@task_item n=2 part=not_needed
The session socket's request grammar, replies, connection bounds and lifecycle, and the escher-session flags, roles and exit codes: no wire, flag or role moves.
@task_item n=3 part=not_needed
Recorded check files, kept check files, the coverage answer and line, and cold-agent runs and recordings: the chunk records and replays nothing.
@task_item n=4 part=not_needed
Outbound HTTP, script and sub-resource fetches, CORS and same-origin absences, request timeouts, per-host limits and file URL rules: no network path is touched.
@task_item n=5 part=not_needed
Select lists, range inputs, checkbox and radio change events and their keyboard models: the chunk changes no form control.
@task_item n=6 part=not_needed
Stable element id derivation, snapshot text state words and the graph and actions answers' fields: no id, state word or answer field changes.
@task_item n=7 part=not_needed
Browser history in SQLite, cookies, the HTTP cache, clipboard and file dialogs: no stored data is touched.
@task_item n=8 part=not_needed
CI secrets, signing material, publish guards and release publishing: no secret or publish job changes.
@task_item n=9 part=not_needed
The WPT runner, its quarantine entries and post-results workflow: no WPT change.
@task_item n=10 part=not_needed
Accessibility adapter content classes, accessible names and the accessibility tree refresh: a canvas exposes no new accessible node in this chunk.
