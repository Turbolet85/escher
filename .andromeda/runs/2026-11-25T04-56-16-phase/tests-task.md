@task_prose part=chunk
A canvas's declared virtual nodes (key, AccessKit role, name, content-box region, id `{owner}///{key}`) enter the accessibility tree as children of their canvas's tree node, each under an even-versioned tree id no DOM id can take.
The snapshot reads each virtual node as a node (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id.
The driver's click takes a virtual node's id, acts as a pointer event at its region's centre and returns once settled with the diff; the refusals stale or not-found, then disabled, off-screen and covered hold for it in the run path's one order, with nothing run.
The declaration gains a declared disabled flag read as the snapshot node's enabled state; the UI graph, the lint and exploration exclude virtual nodes; every other id-taking verb reads a virtual id not-found.
It inverts the snapshot half of the earlier check that a declaring canvas puts nothing on the snapshot or graph into this entry's check and re-asserts its graph half.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the engine's Widget trait and the virtual node declaration it returns (key, role, name, region) — edits: VirtualNodeDeclaration, gaining a declared disabled flag
@task_file n=2 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the document's accessibility tree builder, giving each tree node its DOM node's id — edits: build_accessibility_tree, adding a canvas's virtual nodes as children of its tree node
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot builder reading the accessibility tree into snapshot nodes and the snapshot diff — edits: the builder's tree-id decoding, a virtual-node arm checked first
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the UI graph, built from the snapshot — edits: graph, excluding virtual nodes
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the canvas lint over a screen's snapshot — edits: its snapshot read, excluding virtual nodes
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: exploration and its screen digest over the snapshot text — edits: the digest, excluding virtual nodes
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path: id resolution, the refusal checks (stale, not-found, disabled, off-screen, covered) and the acting verbs — edits: resolve, a virtual arm for click alone; the off-screen and covered checks over a virtual node's region
@task_file n=8 path=docs/capabilities.md mark=edits
what: the capability statement pinned to the driver schema — edits: the stated limits of virtual nodes (click only, covered by DOM coverage of the region's centre, clickable false)
@task_item n=1 part=needs
The element id's virtual form `{owner}///{key}`: a virtual node's stable id, carried wherever a stable id rides today.
@task_item n=2 part=needs
The snapshot node a virtual node reads as: id, role, name, state and bounds, its bounds the region moved to the page by its canvas's content-box origin and transforms, its enabled state the declared disabled flag inverted.
@task_item n=3 part=needs
A virtual node's clickable reads false, a stated limit.
@task_item n=4 part=needs
The snapshot diff and the acting call's result name a virtual node added, removed or changed by its id.
@task_item n=5 part=needs
The driver's click on a virtual id acts at its region's centre as a pointer event, the widget hearing it in its content-box frame, and returns once settled with the diff, as on an element.
@task_item n=6 part=needs
The refusal causes for a virtual id — stale for an id an earlier screen read now gone, or not-found, then disabled, off-screen and covered — in the run path's one refusal order, changing nothing and echoing no input.
@task_item n=7 part=needs
Off-screen reads the region's centre against the canvas's visible region; covered reads DOM coverage of the region's centre only, walking to the canvas element — a stated limit (another virtual node over it is not seen).
@task_item n=8 part=needs
Only click resolves a virtual id: type, scroll, screenshot by id, layout by id and canvas by id read it not-found, a stated limit in the capability statement.
@task_item n=9 part=needs
The UI graph, the lint and exploration's digest hold virtual nodes out until a later entry; the graph half of the earlier assertion that a declaring canvas puts nothing on the graph stands.
@task_item n=10 part=needs
The earlier check that a declaring canvas puts nothing on the snapshot inverts for the snapshot into this entry's check; its mutation control is owed and enters the plan with its red reading.
@task_item n=11 part=needs
A virtual tree node's id is even-versioned (canvas slot index low, 2·(n+1) high) so it collides with no DOM id; each reader that decodes a tree id (the snapshot builder, the author-id loop, the focus writer) checks the virtual map first, so none aliases a live DOM node.
@task_item n=12 part=needs
A declaration change raises no invalidation signal and writes no changed-set mark; every driver reading is current because the snapshot rebuilds the tree per read.
@task_item n=13 part=needs
A virtual node's name and id ride today's exits only — the snapshot text and diff on the session socket's call answer, the command line, the MCP server, the session recording and kept-check replay, the cold-agent pipe — with no new carrier, refusal cause, exit code, session error, log field or span.
@task_item n=14 part=needs
Stand logs, the driver's log line, the session host's stderr and the MCP role's stderr carry no virtual node id or name: the first author-written canvas-side text on a carrier stays off every log.
@task_item n=15 part=needs
The snapshot text of a screen holding virtual nodes stays inside the snapshot byte budget.
@task_item n=16 part=needs
The proof surface is a fixture canvas a test declares (keyed, named, role-bearing regions, one declared disabled), stated in code.
@task_item n=17 part=needs
The capability statement, pinned to the driver schema, names the virtual-node limits and its claim names and counts stay held to the schema.
@task_item n=18 part=needs
A row resting only on a delegate's answer (the even-versioned id, the graph/lint/exploration exclusion, click alone, the covered and clickable limits) lands provisional.
@task_item n=19 part=needs
No plan step deletes a worktree or build directory; a probe worktree stands outside the build directory.
@task_standing n=1
convention: each new test is mutation-controlled, red before green, its control entering the plan with its red reading and leaving earlier test targets green
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=2
convention: each new expected value is pinned as a literal, and a test failure prints no argument value
home: unit-test-strategy
@task_standing n=3
convention: each new unit test sits in an inline test module of its source file
home: test-strategy · 2-test-strategy
@task_standing n=4
convention: each new integration test file is one test target, a shared test module no target
home: test-strategy · 2-test-strategy
@task_standing n=5
convention: each new layout or bounds assertion runs in both layout modes, on the bundled font, with no skip
home: integration-test-strategy
@task_standing n=6
convention: each new fixture is test data stated in code, nothing seeded, shared fixture pages stated once
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=7
convention: each new session check owns its host, is unix-gated and uses no sleep or wall-clock read
home: session-lifecycle
@task_item n=1 part=not_needed
The WPT conformance runner, its CLI, statuses, outputs and CI workflow: the chunk changes no web-platform behaviour.
@task_item n=2 part=not_needed
The agent-run five-verb contract, its events, exit codes and selections: the chunk adds no verb or event to it.
@task_item n=3 part=not_needed
Handler coverage and its limits: virtual nodes reach handler coverage only in a later entry.
@task_item n=4 part=not_needed
The harness contrast reading and its required ratio: the chunk adds no contrast reading.
@task_item n=5 part=not_needed
The virtual clock, advance and timer-tick stand-ins: the chunk changes no time behaviour.
@task_item n=6 part=not_needed
Package installs, cache, actions pinning, secrets and the apt-install script exit codes: the chunk adds no CI job.
@task_item n=7 part=not_needed
The cold-agent verdict, its events, exit codes and live-run bounds: the cold-agent pipe carries what it carries today, unchanged.
@task_item n=8 part=not_needed
Typed-password masking and select-list painting: virtual nodes take no typed text and no select.
@task_item n=9 part=not_needed
Upstream buffered-render and mounted-flush tests and upstream oversize files: the chunk touches no upstream test.
