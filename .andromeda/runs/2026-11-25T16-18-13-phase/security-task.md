@task_prose part=chunk
Circle Drawer's canvas widget declares one virtual node per circle it paints (author key, AccessKit role, non-empty name, region in its content box), making Circle Drawer the first served app to declare virtual nodes, so each circle reads as a node `{owner}///{key}` in the accessibility tree, snapshot, diff, UI graph, actions, layout and exploration.
A click on a circle's node acts on its circle and its diff names it; a covered, off-screen, disabled or removed circle node is refused with that cause, proved on the stand.
The canvas reading and the canvas verb group a canvas's draw commands under the virtual node that covers them, and a changed command reads with an identity in the reading's diff (CARRY 1).
It adds stand checks in both layout modes and the binary-level reading of a virtual node's id and name on the socket host, the command line's stdout and a cold-agent live run (CARRY 4), and fixes the windows-only unused snapshot import (CARRY 5).
It re-verifies CARRYs 2, 3, 6 and 7 (canvas label region, a virtual node's covered, handler coverage of a virtual node, the lint reading no virtual node) against overlapping circles, and sweeps every committed stand pin and literal pin that reads Circle Drawer's screen.
@task_file n=1 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: Circle Drawer's canvas widget, a served example app whose canvas carries an onclick listener — edits: CircleCanvas::virtual_nodes
@task_file n=2 path=packages/escher-driver/src/canvas.rs mark=edits
what: the driver's canvas reading and its line diff, the answer of the canvas verb — edits: the grouping of draw commands under a virtual node, a command's identity in the diff
@task_file n=3 path=packages/escher-driver/src/lint.rs
what: the driver's lint, which reads no virtual node (no overlap, clip or off-screen finding on a declared region)
@task_file n=4 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint, which reads a canvas label's region against the content box cut by clip, layer or mask paths
@task_file n=5 path=packages/escher-driver/src/coverage.rs
what: handler coverage, which lists no handler for a virtual node and counts a click on one as its canvas's own listener under the canvas's id
@task_file n=6 path=packages/escher-driver/src/execute.rs
what: the driver's command execution, where a click's target check and its refusal causes (covered, off-screen, disabled, stale, not-found) are read
@task_file n=7 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the virtual node component — a canvas's declared virtual nodes, their validation by a closed refusal set and their covered reading
@task_file n=8 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs mark=edits
what: the check of virtual nodes in every read (graph, actions, layout, exploration) on fixture canvases — edits: the snapshot import put behind cfg(unix)
@task_file n=9 path=examples/seven_guis/tests/cli_commands.rs mark=edits
what: seven_guis' process check of the escher-session command line's stdout — edits: the binary-level reading of a virtual node's id and name
@task_file n=10 path=examples/seven_guis/tests/mcp_server.rs mark=edits
what: seven_guis' process check of the MCP server's tool results — edits: the binary-level reading of a virtual node's id and name
@task_file n=11 path=docs/capabilities.md mark=edits
what: the committed capability statement — edits: its stated sizes and counts read off Circle Drawer's screen, the section on a virtual node's covered
@task_item n=1 part=needs
A virtual node now crosses on a served app: each circle's id, role and name leave by every carrier the virtual node crossing ratifies — snapshot, diff, graph, actions, layout, exploration report, socket reply, command line stdout and MCP result — and by no other.
@task_item n=2 part=needs
The stable element id's virtual-node form `{owner}///{key}`: the circle's author key must be stable across reads, unique within its canvas and hold no process-local handle or NodeId, though a circle has no id of its own.
@task_item n=3 part=needs
Declared virtual nodes are validated by a closed refusal set: a circle near the canvas edge whose region spills the content box meets the region-outside refusal, an empty name is refused; the declaration must stay inside those bounds.
@task_item n=4 part=needs
A click on a virtual node passes the one target check shared with the actions read, and refusals keep the fixed order and closed cause set (covered, off-screen, disabled, stale, not-found), each holding nothing the call supplied and logged by cause name only.
@task_item n=5 part=needs
A virtual node reads covered only by an element over its region's centre, never by another virtual node of its canvas (CARRY 3); overlapping circles put this rule on the stand.
@task_item n=6 part=needs
The canvas answer changes: grouping draw commands under a virtual node puts a node's id into the canvas answer, a crossing change to the canvas content crossing, which today holds no virtual node; a changed command's identity in the line diff is new content in that answer.
@task_item n=7 part=needs
The canvas answer, graph, actions and layout answers are kept whole in kept checks; a virtual node's id and name in them fall under the kept check's owner-only, uncommitted rule.
@task_item n=8 part=needs
The lint reads no virtual node (CARRY 7) and its findings hold closed content only; a canvas label's region read as the shape or virtual node under it (CARRY 2) touches the canvas label crossing in a lint finding.
@task_item n=9 part=needs
Handler coverage of a virtual node (CARRY 6): no handler of its own, a click counted as the canvas's own listener under the canvas's id; the coverage answer line carries no user data.
@task_item n=10 part=needs
A circle's accessible name is an accessible name: it leaves only by ratified exits and is kept out of logs, as stable ids are; the command span and tool_call span hold no content.
@task_item n=11 part=needs
The diff and snapshot text carry the node's id field, name field and state words; a click's diff names the acted circle through the diff node keys.
@task_item n=12 part=needs
CARRY 4's cold-agent live run carries the screen and canvas answers, now holding circle node ids and names, to the provider; its events and logs stay run metadata only, its recordings owner-only and its raw transcript and server output unprinted.
@task_item n=13 part=needs
Evidence from the live and binary-level runs enters with no raw log carrying a temp or home path and no credentials; the recording or state directory is never named.
@task_item n=14 part=needs
The windows build fix (CARRY 5) is checked by a locked cargo check of the test crate for the windows target.
@task_item n=15 part=needs
Committed stand pins and literal pins that read Circle Drawer's screen are swept: the cross-OS digest tables, the canvas commands table, the lint and non-lean checks, and the capability statement's sizes and counts move with the circles becoming nodes.
@task_item n=16 part=needs
The driver session socket and the MCP server answers stay within their size bounds when Circle Drawer's answers grow by its circle nodes; the actions answer stays unpaged and the graph answer paged.
@task_standing n=1
convention: each new test or check reads no environment beyond its ratified variables, prints no temp or home path and puts no credential into evidence
home: secret-management
@task_standing n=2
convention: each new log line or span holds no screen content — no accessible name, stable id or caller input — only cause names, verbs and counts
home: logging-monitoring
@task_standing n=3
convention: each new process check drives the binary through the closed verb schema and the one validation gate, never a back door
home: input-validation
@task_item n=1 part=not_needed
Authentication and authorization: CI tokens, publish guards and the absence of application auth are untouched by a canvas declaring nodes.
@task_item n=2 part=not_needed
Outbound network rules — script and sub-resource fetches, per-host limits, CORS and same-origin, request timeouts, TLS, external navigation schemes: Circle Drawer's circles load nothing.
@task_item n=3 part=not_needed
Select lists, range inputs, password and file masking, optgroups and select keyboards: Circle Drawer's canvas holds none of these controls.
@task_item n=4 part=not_needed
HTML, CSS and SVG parsing rules — invalid declarations, selectors, @import and iframe nesting, font-face sources, inline SVG serialization: the chunk parses no new markup.
@task_item n=5 part=not_needed
Browser history, cookies, the HTTP cache, the history database and SQL placeholders: no served app storage is touched.
@task_item n=6 part=not_needed
CI secrets, signing material, publishing and the WPT workflows: the chunk adds no release or WPT job.
@task_item n=7 part=not_needed
JavaScript opt-in, the dioxus-native JS evaluation and the CDP devtools server: the seven_guis app runs no script and the devtools feature is not built.
@task_item n=8 part=not_needed
Keyboard reach of canvas virtual nodes and virtual nodes of rotated or scaled canvases: they belong to later entries, not built here.
