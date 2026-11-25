@task_prose part=chunk
A canvas's declared virtual nodes (key, role, name, content-box region, id owner///key) enter the accessibility tree as children of their canvas's tree node, each under an even-versioned tree id that no DOM id collides with.
The snapshot reads them as nodes (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id; the UI graph, the lint and exploration hold them out.
The driver's click takes a virtual node's id, acts at its region's centre and returns once settled with the diff; the refusals stale or not-found, then disabled, off-screen and covered, hold for it in the one order, nothing run.
The declaration gains a declared disabled flag read by the snapshot as its enabled state; type, scroll, screenshot, layout and canvas by id read a virtual id not-found.
It re-reads the last wrap's CI verdict (green) and turns the earlier control that a declaring canvas puts nothing on the snapshot into its inverse, keeping the graph half.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the engine's custom widget trait and its virtual node declaration — edits: the VirtualNodeDeclaration struct (gains a declared disabled flag)
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the document's accessibility tree build, one tree node per DOM node under its u64 id — edits: build_accessibility_tree (virtual tree nodes as children of their canvas)
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot builder that reads the accessibility tree into the snapshot and its diff — edits: the tree-id decode and the builder's arm for a virtual node
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph read on demand from the snapshot — edits: graph (excludes a virtual node)
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the canvas lint over the screen's snapshot nodes — edits: the snapshot node loop (excludes a virtual node)
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration and its screen digest of the snapshot text — edits: screen_digest (excludes a virtual node)
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb run path, its one resolve path and its refusals — edits: resolve (a virtual arm for click alone), the off-screen and covered checks
@task_file n=8 path=docs/capabilities.md mark=edits
what: the capabilities document that states what the driver reads and does, and its stated limits
@task_item n=1 part=needs
The snapshot verb and its text and diff writers: a virtual node now appears as a node with id owner///key, its role, name, state and bounds, and the diff names it added, removed or changed by id.
@task_item n=2 part=needs
The snapshot's bounds for a virtual node: its region moved to the page by its canvas's content-box origin and transforms.
@task_item n=3 part=needs
The snapshot's enabled state for a virtual node reads the declaration's declared disabled flag; its clickable reads false, a stated limit.
@task_item n=4 part=needs
The driver's click verb: it takes a virtual id, lands a pointer event at the region's centre heard by the canvas widget, and returns once settled with the diff, as on an element, under the virtual clock's click contract.
@task_item n=5 part=needs
The driver's refusals on a virtual id in the run path's one order with nothing run: stale for an id an earlier screen read now gone, not-found, then disabled, off-screen (region centre against the canvas's visible region), covered (DOM coverage of the region centre only, a stated limit).
@task_item n=6 part=needs
The verbs type, scroll, screenshot by id, layout by id and canvas by id read a virtual id not-found; only click resolves one, a stated limit.
@task_item n=7 part=needs
The UI graph line, the lint and exploration exclude a virtual node until a later entry; the graph's content is unchanged by a declaring canvas.
@task_item n=8 part=needs
The escher-session command answers (snapshot answer, call answer on the session socket) and its stdout lines for the command line and the MCP role carry what they carry today, now holding a virtual node's id and name; no new exit, cause, session error or exit status.
@task_item n=9 part=needs
The MCP role's snapshot tool result and other tool results carry the virtual node's id and name in today's form.
@task_item n=10 part=needs
The session recording, kept-check replay and the cold-agent pipe's model provider carry virtual ids and names in today's form; no new log field or span.
@task_item n=11 part=needs
The accessibility tree's tree ids: a virtual node's id is even-versioned (canvas slot index low, 2 times (n+1) high) so it aliases no DOM id; every reader that decodes a tree id (the snapshot builder, the author-id loop, the focus writer) checks the virtual map first.
@task_item n=12 part=needs
The platform accessibility adapter receives virtual tree nodes with their role and accessible name as children of the canvas.
@task_item n=13 part=needs
A declaration change raises no invalidation signal and writes no changed-set mark; every driver reading is current because the snapshot rebuilds the tree per read.
@task_item n=14 part=needs
Hit-testing on a click at a region's centre stops at the canvas element; the covered check walks the hit's ancestors to the canvas only.
@task_item n=15 part=needs
A mutation control: the snapshot half of the earlier assertion (a declaring canvas puts nothing on the snapshot) inverts into this entry's check, the graph half stands; each control enters the plan with its red reading.
@task_item n=16 part=needs
The proof surface is a fixture canvas a test declares, with keyed, named, role-bearing regions, one of them declared disabled.
@task_item n=17 part=needs
Rows resting only on a delegate's answer (tree-id versioning, the graph, lint and exploration exclusion, click alone resolving, covered centre-only) land provisional.
@task_standing n=1
convention: each new test is built against an engine using the system fonts
home: typography
@task_standing n=2
convention: each driver command line output line a new test reads is read by an agent, not a person
home: surface-cli
@task_standing n=3
convention: escher-session output carries no colour and gets no terminal test
home: surface
@task_item n=1 part=not_needed
Colour palette rows (engine colours, select list colours, Circle Drawer canvas colours, color-scheme recascade): no colour is drawn or changed.
@task_item n=2 part=not_needed
Font face, family fallback, locale and root-unit rows: no text layout or font choice changes.
@task_item n=3 part=not_needed
Spacing rows (body margins, table cell spacing, select list placement): no spacing changes.
@task_item n=4 part=not_needed
Iconography rows (cursor crate, cursor resolution, favicon) and the shell provider's window controls (decorations, maximize, minimize, window drag).
@task_item n=5 part=not_needed
Select list overlay rows (list events, open gestures, wheel and key scroll, motion, painting order, chosen row colours): no select behaviour changes.
@task_item n=6 part=not_needed
Overlay scrollbar fade timings, smooth scroll behaviour and the scrollbar thumb drag.
@task_item n=7 part=not_needed
The WPT runner's output and result colours.
@task_item n=8 part=not_needed
The JS window object's size and pixel ratio, the web-spa WASM pages and mobile targets.
@task_item n=9 part=not_needed
Brand identity and the empty sections (border radius, anti-patterns, self-validation, design decisions log).
