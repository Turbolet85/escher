@task_prose part=session
Circle Drawer's circles became declared virtual nodes on the stand: each circle reads circle-canvas///c{id}, role Image, named Circle {id}, {d} px, its bounds its region, its id given by the app and stable across undo and redo.
The canvas read now groups its draw commands by the canvas's virtual nodes: every line of the canvas verb's full answer and every part closes with node={k} (the 1-based index of the node whose region holds the command's centre, nearest centre first) or node=- for none; the summary's shape is unchanged, its digest now the tagged text's.
The tag carries a number only: no virtual node id or name crosses into any canvas answer; the read still opens no span and the command span is unchanged.
The handler coverage line now lists a virtual node's click under its {owner}///{key} id, owned by the canvas, its form unchanged; blitz-dom re-exports the accessibility role type as VirtualNodeRole; the UI graph gains Circle Drawer's registered state next_id.
Measured: Circle Drawer's canvas full text 59, 562, 6,190 and 51,656 bytes at boot, one, 12 and 100 circles; its content box 696 x 396; the unexercised-after-drives row for Circle Drawer 1 to 4 handlers. Retired: the tag spelling n={k}, never committed. No dependency, IPC method, env var, port or crate added.
@task_file n=1 path=tests/blitz-tests/tests/stand_circle_nodes.rs
what: the stand check of Circle Drawer's circle nodes: the tree and snapshot, stale and not-found after undo, the graph, actions, layout and exploration by id, the canvas node tag, the lint, and a kept check's coverage of an unclicked circle's click
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: blitz-dom's public surface, the engine crate's re-exports, now the virtual node role type
@task_file n=3 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the Circle Drawer seven_guis task: its canvas widget, its data-circles attribute, its registered graph state and its declared virtual nodes
@task_file n=4 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas read (Session::canvas, CanvasReading): reading a canvas's draw commands and grouping them by its virtual nodes
@task_file n=5 path=packages/escher-driver/src/json/canvas.rs
what: the canvas answer's writer: the canvas verb's summary, full answer and part lines
@task_file n=6 path=packages/escher-driver/src/json_tests.rs
what: the driver's answer-writer unit tests, the canvas line table and the no-key-twice check among them
@task_file n=7 path=packages/escher-driver/src/schema.rs
what: the driver's verb and tool schema: each verb's arguments and field help, the canvas verb's text field help among them
@task_file n=8 path=packages/escher-driver/src/canvas_tests.rs
what: the canvas read's unit tests, the grouping of commands under virtual node regions among them
@task_file n=9 path=packages/escher-driver/src/coverage.rs
what: the handler coverage computation behind escher-session coverage: which handlers a kept check's replay exercised, now a virtual node's click
@task_file n=10 path=packages/escher-driver/src/execute.rs
what: the driver's command execution: each verb's run under the command span, the canvas verb's part cut among them
@task_file n=11 path=packages/escher-driver/src/canvas_lint.rs
what: the canvas lint: findings over a canvas's painted labels, its glyph ink box
@task_file n=12 path=tests/blitz-tests/tests/virtual_nodes_in_every_read.rs
what: the stand check that the UI graph, actions, layout, exploration and a replayed click read a canvas's virtual node by its id
@task_file n=13 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand's shared session helpers for blitz-tests targets
@task_file n=14 path=tests/blitz-tests/tests/common/mod.rs
what: the stand's shared fixtures: the unexercised-after-drives handler table, the 100-circle spill table and the graph drives
@task_file n=15 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: the stand check of Circle Drawer's canvas painting and its data-circles attribute
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand check of the canvas verb's answers: the measured table of summary, full and part sizes and digests
@task_file n=17 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: the stand check of the canvas answer across its carriers, the glyph line's key pin among them
@task_file n=18 path=docs/capabilities.md
what: the capability statement, kept by hand: what escher does, its canvas read as data with Circle Drawer's full-text sizes, and its kept limits
@task_item n=1 part=touches
decision/canvas-read-opens-no-span: the canvas read now also reads the canvas's virtual nodes and the viewport scale to group commands; it still opens no span of its own.
@task_item n=2 part=touches
contract/command-span/canvas and the canvas verb's command span: the verb's answer now carries the node tag on every line; the span's fields are unchanged.
@task_item n=3 part=touches
decision/declared-node-read-logs-nothing: the declared-node read (virtual_nodes) gains a caller in the canvas read; it still logs nothing.
@task_item n=4 part=touches
decision/answers-echo-nothing-typed and contract/escher-session-stdout/command-answer: the canvas answer's lines gain node={k} or node=-, a number the driver writes, no id, name or caller text.
@task_item n=5 part=touches
contract/kept-check/answer and contract/kept-check/answer-digest: a kept check's canvas answer now holds the tagged lines and its summary digest moved to the tagged text's.
@task_item n=6 part=touches
contract/coverage-role-stdout/coverage-line, contract/coverage-line/shape, contract/coverage-line/owner and contract/coverage-line/content: the coverage line now carries a virtual node's {owner}///{key} id with the canvas as owner and event click; its form and content classes unchanged.
@task_item n=7 part=touches
decision/reached-record-ids-and-events-only and decision/coverage-line-never-logged: the reached record and coverage now hold virtual node ids and click; still ids and event names only, never logged.
@task_item n=8 part=touches
contract/escher-session-coverage/accepted and contract/escher-session-coverage/command-shape: a replayed kept check's virtual node click is counted exercised for the virtual node's id.
@task_item n=9 part=touches
contract/graph-answer/content, contract/command-answer/graphed and decision/state-registry-holds-no-values: the UI graph gains Circle Drawer's registered state next_id, declared written by the canvas's click; the registry still holds names, no values.
@task_item n=10 part=touches
decision/no-state-page-skips-registry: Circle Drawer now registers graph state (next_id).
@task_item n=11 part=touches
contract/refusal-cause/stale and contract/refusal-cause/not-found: an undone circle's node now answers stale, a circle never drawn not-found; covered is not answered for a circle under another.
@task_item n=12 part=touches
contract/actions-answer/content, contract/actions-answer/owner and contract/layout-answer/content: actions and layout now list each circle node by its id, owned by the canvas.
@task_item n=13 part=touches
decision/exploration-adds-no-span: exploration now reaches a path clicking a circle's virtual id; it adds no span.
@task_item n=14 part=touches
decision/capability-statement-updated-by-hand: the capability statement was updated by hand with the node tag, Circle Drawer's re-measured full-text sizes and two kept limits.
@task_item n=15 part=touches
decision/session-logs-carry-no-page-content and decision/driver-spans-carry-no-content: circle node names (app text, a circle's id and diameter) reach the tree, snapshot and graph answers, never a span or log.
@task_item n=16 part=touches
decision/committed-checks-hold-no-private-data and decision/committed-kept-checks-clean: kept checks now hold circle node ids and tagged canvas answers; no secret or private data added.
@task_fact n=1 slug=pii-scrubbing-compliance
fact: The canvas answer's node tag carries only a 1-based node index or -, written by the driver; no virtual node id or name crosses into any canvas answer, full, part or summary.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The handler coverage line lists a canvas virtual node's click under its {owner}///{key} id, owned by its canvas; a canvas click inside a node is counted for the canvas alone.
@task_item n=1 part=untouched
The telemetry sink, its log line format, RUST_LOG, resource keys and target allowlist: no sink, event or span field changed.
@task_item n=2 part=untouched
The tool_call span and its ending, and the refusal cause name set: no name added or renamed.
@task_item n=3 part=untouched
The cold-agent pipe, its verdict fields, counted verbs, events and transcript: not reached.
@task_item n=4 part=untouched
The agent-run harness events and logs: not reached.
@task_item n=5 part=untouched
Metrics, metric exporters, telemetry backends and OTel export: still absent, nothing added.
@task_item n=6 part=untouched
Error capture: the panic hook, script error surfaces, window error events and WPT reports: not reached.
@task_item n=7 part=untouched
CI integration: workflows, artifacts, the build-directory sweep, package-install diagnostics; no gate, leg or threshold moved.
@task_item n=8 part=untouched
The recording directory, session state directory, recording bounds, session lifecycle and the host wire: unchanged.
@task_item n=9 part=untouched
Engine tracing features and the select, optgroup and form-entry rows: not reached.
