@task_prose part=chunk
Builds a read of one canvas's content as data: the draw commands an app widget's paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), for one canvas element named by its stable element id, on the held instance, running no step and changing nothing.
The read answers at two levels: a summary (a count per command kind and the canvas's size, or a closed digest of the scene) and a full level (one line or object per command in paint order, layers as nesting).
It is deterministic and hardware-independent (same bytes in process, across processes and on the linux, windows and macOS CI hosts, numbers at a stated rounding) and diffable (a before/after diff names the changed lines; a no-change step reads an empty diff).
A driver verb carries it (a sixteenth verb or an argument of an existing one) on the carriers the founder rules; what crosses, including to a model provider, is written as the founder's forks and no default is taken.
It measures the dump's size for Circle Drawer's canvas at each level, at boot and after circles are added, against the snapshot's 10,000-byte budget; it adds no canvas lint, no virtual nodes, no new painting.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the engine's paint path, where custom widget scenes are built and dropped after the paint — edits: paint_scene, build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget component — the Widget trait whose paint returns a recorded scene
@task_file n=3 path=packages/blitz-test-harness/src/ mark=edits
what: the test harness — a read of one canvas's scene beside the screenshot read
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's closed verb schema — the verb table and its argument kinds
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver verb specifications — the verb table's entries
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command — the verb or argument a call names
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution — running a verb on the held instance and its target check
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's answer writer — the result shape written as JSON
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's help text — the description of each verb
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server — the tool list and tool results over stdio
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the recorder of kept checks — a call and its answer written into a recorded check file
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: the replay of a kept check — each recorded step run through the session and its answer compared
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs mark=edits
what: the Circle Drawer task — the stand's one canvas widget
@task_file n=14 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven GUIs stand — the app that holds the tasks and the canvas
@task_file n=15 path=tests/blitz-tests/tests/ mark=edits
what: the engine's integration tests — the canvas read's checks
@task_file n=16 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables used by the integration tests
@task_file n=17 path=docs/capabilities.md mark=edits
what: the committed capability statement — the list of driver verbs and MCP tools
@task_file n=18 path=README.md mark=edits
what: the README — its driver tool count
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check that the README's tool count and the capability statement match the driver's verbs
@task_item n=1 part=needs
Canvas content today crosses only as shot pixels: the new read is a new crossing of canvas content as data, and every carrier it rides (the session socket, the command line, the MCP server, a kept check, a model provider in a cold-agent run) is a widening the founder rules on as a fork, under the boundary widening rule.
@task_item n=2 part=needs
A glyph run carries glyph ids and font data, not text; a dump that writes a label as text, or as glyph ids, still carries authored page text, so where that text may leave (which ratified exits, whether it may reach a model provider) is the founder's fork and must be stated against the rules for attribute text and accessible names.
@task_item n=3 part=needs
Font data and image data in a draw command cross only by reference (a digest or an index), never inline: no font bytes or image bytes in the answer.
@task_item n=4 part=needs
The read changes nothing and runs nothing: no step, no settle, no move of session state, no clock read — like the actions read — and two reads of one state answer the same bytes.
@task_item n=5 part=needs
The new verb or argument joins the closed driver verb schema and passes the single validation gate shared by every carrier (command line, session socket, MCP); its arguments use the existing closed argument kinds.
@task_item n=6 part=needs
The canvas is named by its stable element id, looked up anew on each call; an unknown id is refused as stale or not-found, a non-canvas or unfit element is refused by the shared target check in the fixed refusal order; no NodeId or other process-local handle crosses in the answer.
@task_item n=7 part=needs
A refusal of the new verb uses the closed refusal cause set with its fixed texts and holds nothing the call supplied.
@task_item n=8 part=needs
The answer stays inside the existing bounds: the driver command line's answer bounds, the session socket's oversize reply and the MCP tool result's size bound; the full level's size at boot and after circles are added is measured against the snapshot's 10,000-byte budget, and whether the full level is paged or bounded is decided against those measures.
@task_item n=9 part=needs
Determinism across hosts: numbers written at a stated rounding, no host-dependent value (glyph positions from shaping may differ by host and are measured, not assumed); the Circle Drawer canvas shot's SAMPLE digest held equal on the three CI hosts in stand_canvas_cross_os is kept.
@task_item n=10 part=needs
The read takes the widget's recorded scene without a raster and changes no painting: the paint path, the shot pixels and the custom widget's frame stand as built.
@task_item n=11 part=needs
Diffability: a before/after comparison names the dump lines added, removed or changed; any log of a diff carries its sizes only, never its lines.
@task_item n=12 part=needs
No dump content (labels, glyph ids, coordinates, commands) is logged: the command span and the MCP tool_call span stay content-free with the verb name and cause only, and escher targets withhold content-named fields.
@task_item n=13 part=needs
If the verb is recorded into a kept check, the recorded check file gains the canvas answer: kept whole, owner-only and uncommitted, read back strictly as untrusted bytes and replayed through the session; whether the canvas answer may enter a kept check at all is part of the carrier fork.
@task_item n=14 part=needs
The MCP tool for the read: the tool listing and help stay fixed program text, the tool's arguments use the closed MCP value kinds, and its result follows the tool result shape and isError rule.
@task_item n=15 part=needs
The committed capability statement and the README's tool count state the new verb or tool, and the capability statement check holds them equal to the driver's verbs.
@task_item n=16 part=needs
Circle Drawer is not served by escher-session: a command-line, MCP or kept-check carrier reaches no stand canvas until it is served or a fixture serves one; a fixture state name added for it joins the closed fixture table.
@task_item n=17 part=needs
A cold-agent run carries the screen to the model provider; whether the canvas dump may join what reaches the provider is the founder's fork, and the model's reply stays untrusted input.
@task_item n=18 part=needs
If the dump reuses anyrender's serialize feature or anyrender_serialize, it is taken at the workspace-inherited version under locked builds, the licence gate and the dependency audit; the test harness keeps taking no tracing and no style and keeps blitz-paint's custom-widget feature on.
@task_item n=19 part=needs
Untrusted input: hostile or huge page content under a canvas (NaN or infinite floats, deep layer nesting, many commands) is written without panic and inside the bounds; caller input is never echoed in the answer.
@task_item n=20 part=needs
Evidence written for the founder's forks holds no raw log with a temp or home path and no credential; live evidence follows the operator's-own-login rule.
@task_standing n=1
convention: each new log line or span names a verb or a cause only and carries no screen content, caller input or id
home: logging-monitoring
@task_standing n=2
convention: each new verb, argument or answer field passes the one validation path for every carrier and is a closed field where it can be
home: input-validation
@task_standing n=3
convention: each new answer a carrier returns stays inside that carrier's request and result bounds
home: api-security
@task_standing n=4
convention: each new refusal is a cause from the closed set with fixed texts
home: error-handling
@task_standing n=5
convention: each new kept-check content class is kept whole and owner-only and never committed
home: data-protection
@task_standing n=6
convention: each new test or dependency builds locked, at workspace-inherited versions, under the audit and licence gates
home: dependency-security
@task_item n=1 part=not_needed
Outbound network rows — HTTP through blitz-net or reqwest, per-host limits, request methods, CORS, CSP, same-origin, TLS, cookies and the HTTP cache: the read fetches nothing.
@task_item n=2 part=not_needed
HTML, CSS and script parsing rows — stylesheet loading, selectors, @import and iframe nesting, script fetchers, JavaScript opt-in, bootstrap scripts, CSSOM errors: the chunk parses no page.
@task_item n=3 part=not_needed
Form-control behaviour rows — select lists, multiple selects, range inputs, password bullets, checkbox change events, Enter and Space activation: the canvas holds no form control and no such behaviour changes.
@task_item n=4 part=not_needed
Canvas lint, virtual-node grouping and the virtual-node form of the stable id, a canvas keyboard path, drag, pan and zoom: later entries build them.
@task_item n=5 part=not_needed
Browser app storage and OS integration rows — sqlite history, clipboard, native file dialog, OS drag-and-drop, dropped-file reads: not reached.
@task_item n=6 part=not_needed
CI secrets, publish and release jobs, signing material and the WPT runner and its post-results workflow: the chunk changes no workflow or release path.
@task_item n=7 part=not_needed
Session lifecycle rows — the state directory, socket ownership, hello and stop requests, idle expiry, session label: the chunk adds no session state and changes no lifecycle.
@task_item n=8 part=not_needed
The CDP devtools server and its environment variables: an opt-in feature the chunk does not touch.
@task_item n=9 part=not_needed
The Dioxus native bridge and its version pin: Dioxus stays at 0.7.3 and its bridge is not changed.
