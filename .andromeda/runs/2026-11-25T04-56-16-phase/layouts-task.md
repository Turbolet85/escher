@task_prose part=chunk
A canvas's declared virtual nodes (key, role, name, region, id `{owner}///{key}`) enter the accessibility tree as children of their canvas's tree node, each with an even-versioned tree id that no DOM id collides with.
The snapshot reads each one as a node (id, role, name, state, bounds) and the snapshot diff names a virtual node added, removed or changed by its id; a declared `disabled` reads as its `enabled`.
The driver's `click` takes a virtual id, lands a pointer event at its region's centre and returns settled with the diff; the run path's refusals (stale, not-found, disabled, off-screen, covered) hold for it in their one order.
The UI graph, the visual lint and exploration's digest hold virtual nodes out; every other driver verb reads a virtual id `not-found`.
The previous chunk's assertion that a declaring canvas puts nothing on the snapshot inverts into this chunk's check, its graph half standing, the control entering with its red reading.
@task_file n=1 path=packages/blitz-dom/src/accessibility.rs mark=edits
what: the accessibility tree builder that gives each tree node its id and role — edits: the tree build, which gains a canvas's virtual children, and the tree focus write, which checks the virtual map before decoding an id
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs mark=edits
what: the engine's canvas widget trait and its virtual-node declaration — edits: the virtual-node declaration, which gains a declared disabled flag
@task_file n=3 path=packages/dioxus-native-dom/src/snapshot.rs mark=edits
what: the snapshot model built from the accessibility tree, its text form and its diff — edits: the snapshot builder, which gains an arm for a virtual tree node and checks the virtual map before decoding a tree id as a DOM node
@task_file n=4 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: the read-only UI graph, one node per snapshot node — edits: the graph read, which excludes a virtual node
@task_file n=5 path=packages/escher-driver/src/lint.rs mark=edits
what: the driver's visual lint over the snapshot's nodes (overlap, truncation, contrast findings) — edits: the lint's walk of the snapshot, which excludes a virtual node
@task_file n=6 path=packages/escher-driver/src/explore.rs mark=edits
what: the driver's exploration and its screen digest — edits: the screen digest, which excludes a virtual node
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for a verb: id resolution, the refusals and their order, and the click — edits: the one resolve path (a virtual arm for click alone), the click on a region's centre, the off-screen and covered refusals
@task_file n=8 path=docs/capabilities.md mark=edits
what: the published capability statement of what the driver reads and does, with its stated limits — edits: the virtual-node reach and its stated limits
@task_item n=1 part=needs
The snapshot node and its fields (id, role, name, state, bounds) and the snapshot answer on the session socket, the command line and MCP: each carries a virtual node's id and name with no new field, carrier or part.
@task_item n=2 part=needs
The snapshot diff: names a virtual node added, removed or changed by its id, its bounds being the region moved to the page by the canvas's content-box origin and transforms.
@task_item n=3 part=needs
The driver's click verb: accepts a virtual id and acts as a pointer press at the region's centre on the canvas element, returning once settled with the diff, as on an element.
@task_item n=4 part=needs
The driver's refusals for a virtual node, in the run path's one order with nothing run: stale (an id read earlier, now gone) or not-found, then disabled, off-screen (the region's centre against the canvas's visible region, read through clipping boxes) and covered.
@task_item n=5 part=needs
Covered on a virtual node reads DOM coverage of the region's centre only (another virtual node over it in the same canvas is unseen), and its clickable reads false — both stated limits, written into the capability statement.
@task_item n=6 part=needs
Every verb but click — type, scroll, screenshot by id, layout by id, canvas by id — reads a virtual id not-found, a stated limit; the MCP canvas tool and the layout and screenshot answers keep what they answer today.
@task_item n=7 part=needs
Stable element ids: a virtual node's id is `{owner}///{key}`, riding today's stable-id exits, and the author-id write over the tree checks the virtual map first so no virtual node is read as a DOM element's id.
@task_item n=8 part=needs
The disabled state: a declared-disabled virtual node is written disabled on its tree node and read by the snapshot as enabled false, so the disabled refusal fires on it.
@task_item n=9 part=needs
No invalidation signal and no changed-set mark on a declaration change; the snapshot rebuilds the tree on every read, so every driver reading is current.
@task_item n=10 part=needs
The UI graph answer, the lint answer and exploration exclude a virtual node until a later chunk; the graph's and lint's answers stay as they are on every existing screen.
@task_item n=11 part=needs
The session recording and kept-check replay: a check at rest carries a virtual node's id and name as it carries an element's, and replays.
@task_item n=12 part=needs
The cold-agent pipe and its model provider carry a virtual node's name and id as they carry an element's; no new log field, span, cause, exit code or session error.
@task_item n=13 part=needs
Caller input is never written back: a virtual node's name crossing to a carrier is author-written canvas-side text, a boundary widening answered by the founder, and the wrap halts once on it as the answer card.
@task_item n=14 part=needs
The proof surface is a test fixture canvas declaring virtual nodes; Circle Drawer's circles stay painted on a canvas and declare none in this chunk.
@task_item n=15 part=needs
A row resting only on a delegate's answer (the even-versioned id, the graph/lint/exploration exclusion, click alone resolving, the covered and clickable limits) lands provisional.
@task_item n=16 part=needs
No plan step deletes a worktree or build directory; a probe worktree goes outside the build directory; a control enters the plan with its red reading.
@task_standing n=1
convention: each new driver check that reads the snapshot or runs a verb is a session recording that replays as a kept check
home: surface-cli
@task_standing n=2
convention: each new headless check reads its screen at the pinned headless viewport, the held instance reading as a fresh boot
home: surface-desktop-native
@task_standing n=3
convention: a new refusal reading or verb answer keeps the session's exit codes and refusal words closed, writing no caller input back
home: surface-cli
@task_item n=1 part=not_needed
Shell window chrome controls (drag, maximize, minimize, close, decorations): the chunk touches no window chrome.
@task_item n=2 part=not_needed
The multi-document and paint-bench command lines: the chunk adds no argument, variable or output to either.
@task_item n=3 part=not_needed
The served tasks and their stand ids for counter, CRUD, flight booker, cells, timer, temperature converter, load and order pages: no stand control is added, renamed or rekeyed.
@task_item n=4 part=not_needed
Select and list decisions (the flight-type select's boot reading, open-list rows' bounds, the CRUD list as one keyboard region): no select or list behaviour changes.
@task_item n=5 part=not_needed
The load page's network provider, the order page built to cause, and the stand accent colour: none is read or changed by this chunk.
@task_item n=6 part=not_needed
The fixed document skeleton and arbitrary index.html templates: the chunk writes no document template.
@task_item n=7 part=not_needed
The into-view scroll across holding scrollers: scroll on a virtual id is not-found and scroll's own behaviour does not change.
@task_item n=8 part=not_needed
The agent-run test script's contract (usage, exit 2, run log, Windows pass-through): the chunk adds nothing to it.
