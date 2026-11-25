@task_prose part=chunk
Puts a canvas's declared virtual nodes (key, role, name, content-box region, id {owner}///{key}) into the accessibility tree as children of their canvas's tree node, with an even-versioned tree id no DOM-decoding reader aliases.
The snapshot reads them as nodes (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id.
The driver's click takes a virtual node's id, acts at its region's centre and returns once settled with the diff; its refusals (stale or not-found, then disabled, off-screen, covered) hold for a virtual node with nothing run.
The virtual node declaration gains a declared disabled flag read as the snapshot's enabled; the UI graph, the lint and exploration hold virtual nodes out.
Re-reads, as its own check, the earlier assertion that a declaring canvas puts nothing on the snapshot or graph: the snapshot half inverts, the graph half stands, entering with its red reading.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the engine's accessibility tree builder, each tree node keyed by its DOM node's id — edits: build_accessibility_tree, to add a canvas's virtual nodes as its children
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the engine's custom widget trait and the virtual node declaration (key, role, name, region) — edits: VirtualNodeDeclaration, gaining a declared disabled flag
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot builder that reads the accessibility tree into snapshot nodes and decodes tree ids to DOM nodes — edits: the builder's node arm, gaining a virtual node arm checked before the DOM decode
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph built from the snapshot — edits: the read of the snapshot, to exclude virtual nodes
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the canvas lint read from the snapshot — edits: its read of snapshot nodes, to exclude virtual nodes
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: auto-exploration and its digest read from the snapshot — edits: the digest's read of the snapshot, to exclude virtual nodes
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command run path, its one resolve path for ids and its refusal checks (stale, not-found, disabled, off-screen, covered) — edits: resolve, gaining a virtual arm for click alone; the off-screen and covered checks for a virtual node's region
@task_file n=8 path=docs/capabilities.md mark=edits
what: the capability statement, updated by hand — edits: what click and the snapshot do with a virtual node, and the stated limits (other verbs read a virtual id not-found, covered reads the region's centre only, clickable reads false)
@task_item n=1 part=needs
The snapshot node and snapshot text: a virtual node appears as a node with its id {owner}///{key}, role, name, state and bounds; its clickable reads false, a stated limit.
@task_item n=2 part=needs
The snapshot diff and the driver command answer: a virtual node added, removed or changed is named by its id, through the existing diff writer and answer shape; the command span's added, removed and changed counts count it as any node.
@task_item n=3 part=needs
The click verb on a virtual id: it resolves through the one resolve path and lands as a pointer event at the region's centre, settled, with the diff; the command span for it carries the click verb and no call content, as for an element.
@task_item n=4 part=needs
Refusal causes stale, not-found, disabled, off-screen and covered apply to a virtual node in the run path's one order with nothing run; no new refusal cause, session error or exit code is added.
@task_item n=5 part=needs
Every other verb that takes an id (type, scroll, screenshot by id, layout by id, canvas by id) reads a virtual id as the not-found refusal, a stated limit.
@task_item n=6 part=needs
Covered reads DOM coverage of the region's centre only (another virtual node in the same canvas is not seen), and off-screen is the region's centre against the canvas's visible region: stated limits.
@task_item n=7 part=needs
The disabled refusal on a virtual node reads the declared disabled flag, written on the virtual tree node and read by the snapshot as enabled.
@task_item n=8 part=needs
The boundary widening: a virtual node's name (author-written canvas text) and its id with the key inside ride today's accessible-name and stable-id carriers — platform accessibility adapter, snapshot text and diff on the session socket's call answer, the command line, the MCP server's answers, kept checks at rest and session recordings, and the cold-agent pipe's model provider — with no new carrier, exit, log field or span; each carrier's existing rule on what may be logged or stored binds the name as page content.
@task_item n=9 part=needs
Answers and their content are never logged: the snapshot, diff and click answers now holding a virtual node's name stay out of span fields, log lines and diagnostics; session logs carry no page content.
@task_item n=10 part=needs
Session recordings and kept check files hold the answer (now with virtual node names) at rest only, in private directories; the cold-agent transcript and verdict rules stay as they are and the verdict holds no content.
@task_item n=11 part=needs
The declared-node read logs nothing and a declaration change raises no invalidation signal and writes no changed-set mark; every reading is current because the snapshot rebuilds the tree per read.
@task_item n=12 part=needs
The UI graph answer, the lint findings and exploration's digest exclude virtual nodes (graph half of the earlier assertion stands); the cold-agent counted verbs and graph answer shape are unchanged.
@task_item n=13 part=needs
The accessibility tree id of a virtual node is even-versioned (canvas slot low, 2·(n+1) high) and the three tree-id decoders (snapshot builder, author-id loop, focus writer) check the virtual map first, so no virtual id aliases a live DOM node.
@task_item n=14 part=needs
The capability statement is updated by hand for click and the snapshot on virtual nodes and the stated limits.
@task_item n=15 part=needs
The mutation control of the earlier «declaring canvas puts nothing on the snapshot or graph» assertion enters the plan with its red reading; the snapshot half inverts into this chunk's check.
@task_item n=16 part=needs
The driver emits no event and installs no subscriber, and no span is added for the virtual node read or the tree build; the command and tool_call spans keep their closed field sets.
@task_standing n=1
convention: each new test or check runs silent, shares its modules, and an empty-stderr check unsets RUST_LOG
home: 3-observability-harness-contract
@task_standing n=2
convention: each new committed kept check, fixture or recording holds no secrets or private data, and session or recording state stays local and private
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Metric coverage (both metric areas): the chunk adds no counter, gauge, histogram or exporter, and touches no in-process timing.
@task_item n=2 part=not_needed
Error capture and reporting: script error events, window error and onerror arguments, error source labels, the WPT report, stylesheet loads and the panic hook are not reached.
@task_item n=3 part=not_needed
The telemetry strategy areas: tracing crate choice and feature gating of engine tracing are unchanged; no new tracing dependency.
@task_item n=4 part=not_needed
Form control rows of PII scrubbing: select chosen sets, select list labels, optgroup labels, multiple select form entries, checkbox and radio change events and the mutation buffer are not touched.
@task_item n=5 part=not_needed
Obs scope summary tier and the absent telemetry backend or OTel export rows: nothing exports and no backend is added.
@task_item n=6 part=not_needed
Script diagnostics drain (take_js_errors, take_messages) and the engine log sites' fixed messages: the chunk writes no script diagnostic and no engine log site.
