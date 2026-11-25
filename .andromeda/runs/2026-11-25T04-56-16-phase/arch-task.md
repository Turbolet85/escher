@task_prose part=chunk
A canvas's declared virtual nodes (key, AccessKit Role, name, content-box region, id `{owner}///{key}`) enter the accessibility tree as children of their canvas's tree node, under an even-versioned tree id no DOM id collides with.
The snapshot reads each as a node (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id.
The driver's click takes a virtual id, acts at its region's centre and returns settled with the diff; stale, not-found, disabled, off-screen and covered refusals hold for it in the run path's one order.
The declaration gains a declared disabled flag read as the node's enabled state; the UI graph, the lint and exploration's digest hold virtual nodes out.
The owed mutation control of the virtual-nodes-declared check is re-asserted: its snapshot half inverts into this chunk's check, its graph half stands.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the Widget trait's virtual-node declaration, Widget::virtual_nodes and VirtualNodeDeclaration — edits: VirtualNodeDeclaration gains a declared disabled field
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the blitz-dom accessibility tree builder, build_accessibility_tree, and its focus writer — edits: virtual tree nodes as children of their canvas node with even-versioned ids, disabled written on them, the focus writer checking the virtual map first
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the Dioxus snapshot builder that reads the accessibility tree's kept DOM elements into snapshot nodes, and the snapshot diff — edits: a builder arm for a virtual tree node (id, role, name, enabled, bounds moved to the page through the canvas's content-box origin and transforms), the tree-id decode checking the virtual map first
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph reader, DioxusDocument::graph, built from the snapshot — edits: excludes virtual nodes
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's lint verb, reading the snapshot for its findings — edits: excludes virtual nodes
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: the driver's exploration, its screen digest read from the snapshot text — edits: screen_digest excludes virtual nodes
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's one run path: target resolve, the per-verb target check and the refusals stale, not-found, disabled, off-screen and covered — edits: resolve gains a virtual arm for click alone; off-screen reads the region centre against the canvas's visible region; covered reads DOM coverage of the region centre
@task_file n=8 path=docs/capabilities.md mark=edits
what: the capability statement held to the driver schema — edits: virtual nodes in the snapshot and click, and the stated limits (only click resolves a virtual id; covered reads the region centre's DOM coverage only; a virtual node's clickable reads false)
@task_item n=1 part=needs
The accessibility tree: build_accessibility_tree puts each taken virtual node under its canvas's tree node with role, name and a disabled state; ids are even-versioned (canvas slot index low, 2·(n+1) high) so they alias no odd-versioned DOM id.
@task_item n=2 part=needs
The three readers that decode a tree id — the snapshot builder, the author-id loop, the focus writer — check the virtual map first, so NodeId decoding never aliases a virtual id onto a live DOM node.
@task_item n=3 part=needs
The snapshot: a virtual node is a SnapshotNode with id `{owner}///{key}`, AccessKit role, name, NodeState enabled from the declared disabled flag, clickable false, and bounds as its region moved to the page by the canvas's content-box origin and transforms; snapshot membership widens to include it.
@task_item n=4 part=needs
The snapshot text and the snapshot diff: a virtual node's line carries today's line fields, quoting and state tokens; the diff names it added, removed or changed by its id, with differs from the closed set.
@task_item n=5 part=needs
The Dioxus bridge's virtual readers: DioxusDocument::virtual_nodes and virtual_node_id are the source the tree and snapshot read; the stable-element-id virtual-node-id form `{owner}///{key}` is the snapshot id.
@task_item n=6 part=needs
The Widget trait's VirtualNodeDeclaration gains a declared disabled bool (F2 A with F8 A); the declaration stays on the engine's Widget trait; that is a direct engine change in blitz-dom.
@task_item n=7 part=needs
The click verb on a virtual id: resolve's virtual arm is click's alone; the click lands as a pointer event at the region's centre through the canvas element and the widget's handle_event; it returns settled with the diff as on an element; canvas placement is at the click centre.
@task_item n=8 part=needs
Refusals on a virtual id: stale (an id an earlier screen read, now gone) or not-found, then disabled, off-screen, covered, in call validation's one first-failure order, with nothing run; the refusal causes stay closed and the seen-ids record holds virtual ids as it holds element ids.
@task_item n=9 part=needs
off-screen for a virtual node is the region's centre against visible_region of the canvas; covered reads only DOM coverage of the region's centre (a hit walked to the canvas element) — a stated limit, as is clickable false; visibility is read from geometry.
@task_item n=10 part=needs
F10 B: type, scroll, screenshot with an id, layout with an id and canvas with an id read a virtual id not-found — a stated limit; the per-verb target checks for those verbs stay element-only.
@task_item n=11 part=needs
F9 A: the UI graph, the lint and exploration's screen digest exclude a virtual node until «Virtual nodes in every read»; the graph verb's nodes, the lint's findings and the explore digest read as today.
@task_item n=12 part=needs
F3 A, a boundary widening answered by the founder: a virtual node's name rides today's accessible-name exits and its id today's stable-id exits — the platform adapter, the snapshot text and diff on the session socket, the command line, MCP, kept checks at rest and replay, and the cold-agent live run's model provider; no new carrier, cause, exit, session error, log field or span; role, bounds and state are closed values inside the ratified carriers; the wrap halts once on it as the answer card.
@task_item n=13 part=needs
Logs and telemetry: a virtual node's name and id are content — no snapshot text, diff content or node name enters a log line or span field; the driver opens no new span or event.
@task_item n=14 part=needs
F11 A: a declaration change raises no invalidation signal and writes no changed-set mark; the changed-set write sites and the decision that a declaration change marks and signals nothing stay as ratified; every reading is current because the snapshot rebuilds the tree per read.
@task_item n=15 part=needs
The owed mutation control of virtual_nodes_declared's «a declaring canvas puts nothing on the snapshot or graph»: the snapshot half inverts into this chunk's check (a declared virtual node reads in the snapshot), the graph half stands; a control enters the plan with its red reading.
@task_item n=16 part=needs
The accessibility feature gate: the virtual tree and snapshot arm stay behind the accessibility feature as the snapshot readers do; the driver names only that feature.
@task_item n=17 part=needs
The fixture canvas a test declares is the proof surface; the capability statement states the limits where an agent reads the tool, held to the driver schema.
@task_item n=18 part=needs
A row resting only on a delegate's answer (F7 A, F9 A, F10 B, F12 A) lands PROVISIONAL; F3 A, F11 A, F8 A with F2 A are the founder's.
@task_item n=19 part=needs
No plan step deletes a worktree or build directory; a probe worktree goes outside the build directory.
@task_standing n=1
convention: each new integration test lives in blitz-tests, one behaviour per test file and one target per file, descriptive names, asserting its fixture's condition first, with numbered fixture cases
home: conventions
@task_standing n=2
convention: each new unit test sits at the end of its source file, or a split test module stays a child of its module; driver unit tests in path children
home: conventions · existing-scopes
@task_standing n=3
convention: each new public item and module carries its docs, rustdoc warnings are errors, expect and unwrap carry a justification, and spec URLs are cited at implementations
home: conventions
@task_standing n=4
convention: each direct engine change is in a function of escher's own and flagged upstreamable
home: stack-and-technologies · established-decisions
@task_standing n=5
convention: each new code path stays under the cognitive complexity ceiling and the per-crate coverage floor, and passes fmt and clippy
home: stack-and-technologies · standard-contracts
@task_item n=1 part=not_needed
Layout, style, paint and text engine rows (Taffy, Stylo, Parley, damage, scrolling, paint scene): the chunk adds no layout or paint behaviour.
@task_item n=2 part=not_needed
Script and JavaScript rows (Boa, script fetchers, script globals, DOM wrappers, microtasks): no script surface is reached.
@task_item n=3 part=not_needed
Networking, HTTP cache, image and font loading rows: no resource is fetched.
@task_item n=4 part=not_needed
Browser app, history, persistence and about-page rows: the browser app is not reached.
@task_item n=5 part=not_needed
WPT runner, its results, diff and CI artifacts: no WPT surface changes.
@task_item n=6 part=not_needed
Select list, select keyboard, range input and form submission rows: no form control behaviour changes.
@task_item n=7 part=not_needed
Keyboard reach, Tab stops, keyboard activation and focus traversal onto a virtual node: deferred to «Canvas keyboard path».
@task_item n=8 part=not_needed
Handler coverage, the actions verb, the layout verb's explanation and exploration's own path expansion beyond holding virtual nodes out: deferred to «Virtual nodes in every read».
@task_item n=9 part=not_needed
Circle Drawer's circles in the seven_guis stand: deferred to «Circle Drawer's circles as virtual nodes».
@task_item n=10 part=not_needed
Build tooling, Nix flake, package install scripts, release and publish workflows, Android, iOS and wasm entry rows: no build or platform entry changes.
