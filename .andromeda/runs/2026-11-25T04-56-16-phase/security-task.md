@task_prose part=chunk
Virtual nodes as elements (v040-11): a canvas's declared virtual nodes (key, role, name, content-box region, id {owner}///{key}) enter the accessibility tree as children of their canvas's tree node.
The snapshot reads them as nodes (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id.
The driver's click takes a virtual node's id and acts on its region's centre, returning settled with the diff; the refusals stale / not-found, disabled, off-screen, covered hold for it in the one order, with nothing run.
The virtual node declaration gains a declared disabled flag, read by the snapshot as enabled; the UI graph, the lint and exploration hold virtual nodes out.
The old assertion that a declaring canvas puts nothing on the snapshot inverts for the snapshot and stands for the graph; its control enters with its red reading.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree builder, the author-id loop and the focus writer — edits: virtual tree nodes as children of their canvas's node, with even-versioned AccessKit ids the readers check against the virtual map first
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the custom widget's virtual node declaration on the engine's Widget trait — edits: the declaration gains a declared disabled flag
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot builder and the snapshot diff — edits: an arm reading a virtual tree node as a snapshot node (id, role, name, state, bounds through the canvas's content-box origin and transforms)
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph read over the snapshot — edits: excludes a virtual node
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the canvas lint read over the snapshot — edits: excludes a virtual node
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration's screen digest over the snapshot — edits: excludes a virtual node
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb run path and its target resolve, with the off-screen and covered refusals — edits: a virtual arm in resolve for click alone; the other id-taking verbs read a virtual id not-found
@task_file n=8 path=docs/capabilities.md mark=edits
what: the committed statement of what the driver's carriers hold — edits: a virtual node's id and name on them, and the stated limits (click only, covered by the region's centre, clickable false)
@task_item n=1 part=needs
Boundary widening: a virtual node's author-written name rides the ratified accessible-name exits and its id the ratified stable-id exits (platform adapter, snapshot text and diff on the session socket, command line, MCP, kept checks at rest, a cold-agent run's model provider) — the founder's F3 A answer; no new carrier, exit or field.
@task_item n=2 part=needs
The stable element id's virtual-node form {owner}///{key}: carried as a stable id, unique against element ids, holding no process-local NodeId, and looked up anew each call.
@task_item n=3 part=needs
The recorded absence of virtual nodes on any carrier no longer holds for the snapshot, the snapshot diff and the accessibility tree; it still holds for the graph, the canvas answer and the actions answer until a later entry.
@task_item n=4 part=needs
A virtual tree node's AccessKit id collides with no DOM id and is decoded by no DOM-decoding reader; the snapshot builder, the author-id loop and the focus writer check the virtual map first, and accessibility focus still names only a built node.
@task_item n=5 part=needs
Snapshot text and diff fields for a virtual node: id field, name field, role and the state words; its role, bounds and state readings cross as closed values, and its clickable reads false.
@task_item n=6 part=needs
The declared disabled flag is written on the virtual tree node and read as the snapshot's enabled, so the disabled refusal fires on a declared-disabled virtual node; the three state readings per control hold for it.
@task_item n=7 part=needs
Target refusals on a virtual id: stale for an id an earlier read held and now gone, not-found, then disabled, off-screen, covered, in the fixed order of target refusals, with nothing run; a refusal holds nothing the call supplied and logs only its cause name.
@task_item n=8 part=needs
Only click resolves a virtual id; type, scroll, screenshot --id, layout --id and canvas --id read it not-found, a stated limit, through the one validation path and the closed driver verb schema.
@task_item n=9 part=needs
A click on a virtual node lands as a pointer event at its region's centre: off-screen against the canvas's visible region through transforms; covered reads DOM coverage of the centre only (another virtual node over it in the same canvas is unseen), a stated limit; click synthesis on the custom-widget target in its content-box frame.
@task_item n=10 part=needs
A declaration change raises no invalidation signal and writes no changed-set mark (F11 A); the tree is rebuilt per snapshot read, so every driver reading is current — the accessibility-tree-refresh-on-change rule is not extended.
@task_item n=11 part=needs
Virtual node declarations keep their closed validation refusal set; the declared disabled flag is a closed boolean.
@task_item n=12 part=needs
No new log field or span: a virtual node's name and id are kept out of logs like accessible names and stable ids; the command span stays content-free and only diff sizes reach a log.
@task_item n=13 part=needs
The session socket carries what the screen reads, now holding virtual nodes; session socket replies keep no NodeId, no attribute text and closed state readings; MCP results and the command line carry the same.
@task_item n=14 part=needs
Kept checks and the session recording hold a virtual node's id and name at rest, under the owner-only kept-check rules; replay of a recorded click on a virtual id runs through the session and the recorded check stays read as untrusted.
@task_item n=15 part=needs
Cold-agent runs carry the screen, now with virtual node names and ids, to the model provider; the cold-agent recordings stay private and its logs metadata-only.
@task_item n=16 part=needs
The UI graph, the lint and exploration exclude a virtual node (F9 A): the graph answer, lint findings and the exploration report gain no virtual node.
@task_item n=17 part=needs
The inverted assertion of the declared-virtual-nodes check (snapshot half now holds virtual nodes, graph half still holds none) has its mutation control, entered with its red reading; no plan step deletes a worktree or build directory.
@task_item n=18 part=needs
A row resting only on a delegate's answer (F7 A, F9 A, F10 B, F12 A) lands provisional.
@task_standing n=1
convention: each new test or fixture canvas keeps the per-crate line coverage floors and reaches the driver through the session as any caller does
home: input-validation
@task_standing n=2
convention: each new test harness or fixture takes no tracing subscriber and no style dependency of its own
home: dependency-security
@task_standing n=3
convention: each new refusal path a test asserts reads its cause from the closed refusal cause set, with no call input in it
home: error-handling
@task_item n=1 part=not_needed
Authentication, authorization, CI tokens and publish-job secret guards: the chunk adds no access path and no CI job.
@task_item n=2 part=not_needed
Outbound HTTP, script and sub-resource fetches, CORS and same-origin, request timeouts and response size limits: no network request is touched.
@task_item n=3 part=not_needed
CSS and HTML parsing, stylesheet loading, font sources, serialization and inline SVG: no parsing path is touched.
@task_item n=4 part=not_needed
Select list, range input, multiple select and password painting rows: no form control behaviour changes.
@task_item n=5 part=not_needed
Browser history, cookies, HTTP cache, TLS and the history database: no stored browsing data is touched.
@task_item n=6 part=not_needed
WPT runner, its quarantine and post-results workflow, and the CDP devtools server: neither is reached.
@task_item n=7 part=not_needed
Environment variables, credential probes and CI secret storage: the chunk reads no new environment and holds no secret.
@task_item n=8 part=not_needed
Keyboard reach and focus movement onto a virtual node, and Circle Drawer's circles: later entries, not this one.
