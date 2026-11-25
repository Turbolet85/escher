@task_prose part=chunk
A canvas's declared virtual nodes (key, AccessKit role, name, content-box region, id `{owner}///{key}`) enter the accessibility tree as children of their canvas's tree node, each with an even-versioned tree id that no DOM id collides with.
The snapshot reads each virtual node from the tree as a node (id, role, name, state, bounds), and the snapshot diff names a virtual node added, removed or changed by its id.
The driver's `click` takes a virtual node's id, acts at its region's centre and returns once settled with the diff; its refusals (stale or not-found, then disabled, off-screen, covered) hold for a virtual node in the run path's one order.
A virtual node declaration gains a declared disabled flag, written on the tree node and read by the snapshot as its enabled; the UI graph, the visual lint and exploration hold virtual nodes out.
The owed mutation control of the declared-nodes check is re-read: its snapshot half inverts into this chunk's check, entering with its red reading, and its graph half stands.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree builder (the document's own tree method) and its focus writer — edits: the build of a canvas's virtual nodes as child tree nodes, their even-versioned tree ids, the declared disabled written on the node, the focus writer checking the virtual map first
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget trait and the virtual node declaration a canvas makes — edits: the declaration's new declared disabled field
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot model built from the accessibility tree, its text form and its diff — edits: the snapshot builder's arm for a virtual tree node and its tree-id decode checking the virtual map first
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the read-only UI graph, one node per snapshot node — edits: the graph read, which excludes a virtual node
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's visual lint, reading the snapshot's nodes — edits: the lint's node walk, which excludes a virtual node
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: the driver's exploration and its screen digest over the snapshot text — edits: the screen digest, which excludes a virtual node
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path — id resolution and the click refusals stale, not-found, disabled, off-screen and covered — edits: the one resolve path (a virtual arm for click alone), the off-screen and covered readings for a virtual node's region
@task_file n=8 path=docs/capabilities.md mark=edits
what: the published capability statement, edited by hand — edits: the stated limits of virtual nodes (click alone resolves a virtual id, covered reads the region's centre only, a virtual node's clickable reads false)
@task_item n=1 part=needs
The accessibility tree gains virtual tree nodes as children of their canvas's node, each carrying its declared role and name; the tree still never names a node it did not build, and its root and focus rules hold.
@task_item n=2 part=needs
A virtual tree node's AccessKit id is even-versioned (canvas slot index low, 2·(n+1) high) so it collides with no DOM node id; the three readers that decode a tree id — the snapshot builder, the author-id loop, the focus writer — check the virtual map before decoding a DOM id.
@task_item n=3 part=needs
The snapshot takes a virtual node's role, name and state from the tree, never from a second mapping; its id is `{owner}///{key}`, the reserved separator `///` with the key inside it, and its bounds are its region moved to the page by its canvas's content-box origin and transforms.
@task_item n=4 part=needs
A virtual node's enabled reading comes from its declared disabled flag written on the tree node, so the driver's disabled refusal fires on a declared-disabled virtual node; the snapshot's enabled for elements still follows the disabled attribute's presence.
@task_item n=5 part=needs
A virtual node's clickable reads false in the snapshot node and its text form, a stated limit; its other state readings are closed values inside the existing node fields — no new snapshot node field or state token.
@task_item n=6 part=needs
The snapshot diff names a virtual node added, removed or changed by its id, through the existing diff entries and differs fields.
@task_item n=7 part=needs
The driver's click resolves a virtual id and lands a primary pointer click at its region's centre on the canvas, the widget reading the point; it returns once settled with the diff, as on an element.
@task_item n=8 part=needs
The driver's refusal causes hold for a virtual node in the one run-path order — stale or not-found, then disabled, off-screen, covered — with nothing run; off-screen reads the region's centre against the canvas's visible region, and covered reads DOM coverage of the region's centre only (another virtual node over it inside the canvas unseen), a stated limit.
@task_item n=9 part=needs
Only click resolves a virtual id: type, scroll, screenshot by id, layout by id and canvas by id read a virtual id not-found, a stated limit.
@task_item n=10 part=needs
A virtual node's name and id ride today's accessible-name and stable-id exits — the platform adapter, the snapshot text and diff on the session socket, the command line, MCP results, the session recording and kept-check replay, the cold-agent pipe — with no new carrier, refusal cause, exit, session error, log field or span; a boundary widening answered by the founder (F3 A).
@task_item n=11 part=needs
A declaration change raises no invalidation signal and writes no changed-set mark (F11 A); the changed-set marks and the tree refresh on change stay as ratified, every driver reading being current because the snapshot rebuilds the tree per read.
@task_item n=12 part=needs
The UI graph, the visual lint and exploration exclude a virtual node until a later chunk (F9 A), so the graph stays one node per element snapshot node and the lint reads no finding on a virtual node.
@task_item n=13 part=needs
The snapshot model stays the single source its readers share; the canvas read and the virtual-node read remain no readers of it, and the readers stay behind the accessibility feature.
@task_item n=14 part=needs
The owed mutation control of the declared-nodes check «a declaring canvas puts nothing on the snapshot or graph» is proved: the snapshot half inverts into an assertion that the virtual node is on the snapshot, the graph half stands, and the control enters the plan with its red reading.
@task_item n=15 part=needs
The proof surface is a fixture canvas a test declares; checks assert over the built accessibility tree and the snapshot, and no plan step deletes a worktree or build directory.
@task_item n=16 part=needs
A row resting only on a delegate's answer (F7 A, F9 A, F10 B, F12 A) lands provisional; the founder's own answers (F3 A, F11 A, F2 A with F8 A) are ratified ground.
@task_standing n=1
convention: each new accessibility check is a Rust test asserting over the AccessKit tree and the snapshot model, in both layout modes where the stand checks run so
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each new accessibility test file runs in the a11y CI leg and the accessibility checks gate CI
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Keyboard reach and focus movement — Tab order, keyboard activation by Enter and Space, range input, select and list keys, the Cells grid and CRUD list keys: canvas keyboard path is a later chunk.
@task_item n=2 part=not_needed
Text contrast thresholds, the stand's contrast tables, stand accent colour and the select list's colours and motion: the chunk paints nothing new.
@task_item n=3 part=not_needed
ARIA attribute reads and hidden-content rules (aria-checked, aria-pressed, aria-labelledby, aria-describedby, aria-expanded, hidden, display none, inert): a virtual node carries no attributes and is read from its declaration.
@task_item n=4 part=not_needed
Select and list box semantics — options, optgroups, open-list rows, expanded, highlighted row, select value: untouched.
@task_item n=5 part=not_needed
Circle Drawer's drawing area as one named canvas node: its circles become virtual nodes in a later chunk.
@task_item n=6 part=not_needed
The actions answer, handler coverage, and the reached-handler record: actions and handler coverage of virtual nodes are a later chunk.
@task_item n=7 part=not_needed
Password and file input values, cognitive accessibility and the stand's shared Tab-order and expectation tables: no stand screen or input value changes.
