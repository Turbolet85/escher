@task_prose part=session
The session split the driver's hand-written answer writer by answer family into seven private child modules: snapshot, diff, caused, layout, graph, actions and lint.
The parent writer keeps the primitives (string, number, ratio, list, object), the screenshot answer without its image, coverage, the one outcome dispatch, refusals, session errors and the session lines; every crate-private item that moved is widened to pub(super) at most.
No written key, field order or byte of any answer moved: the 49 literal JSON pins pass, and the stand replay replays both committed Counter checks with every kept answer compared whole (5 passed).
The writer's 9 unit tests moved out of line into a #[path] test file with their test names unchanged; no public symbol, crate re-export, dependency or caller path changed, and nothing was retired.
CI read green 17/17 at attempt 3 after two iOS jobs cancelled with no runner; escher-driver coverage reads 97.68 % against its 97 % floor.
@task_file n=1 path=packages/escher-driver/src/json.rs
what: edited — the driver's answer writer, written by hand: the JSON primitives, the screenshot answer without its image, the coverage answer, the outcome dispatch to each verb's answer, refusals, session errors and the session lines
@task_file n=2 path=packages/escher-driver/src/json/snapshot.rs
what: added — the snapshot answer's node writer: a node's schema fields, its readings, bounds and differs
@task_file n=3 path=packages/escher-driver/src/json/diff.rs
what: added — the acted answer after an action: settled, busy and the snapshot diff's added, removed and changed nodes
@task_file n=4 path=packages/escher-driver/src/json/caused.rs
what: added — the caused and caused-past lists an answer carries: panics, errors and logs with their source, target, file, line, boundary, owner and event, and cut text
@task_file n=5 path=packages/escher-driver/src/json/layout.rs
what: added — the layout answer: placer, shift and size-cause words, lengths, placement, box and offset of the layout explanation
@task_file n=6 path=packages/escher-driver/src/json/graph.rs
what: added — the graph answer: graph nodes with owner and component, graph states, and their declared and confirmed writes
@task_file n=7 path=packages/escher-driver/src/json/actions.rs
what: added — the actions answer: one action node with its owner
@task_file n=8 path=packages/escher-driver/src/json/lint.rs
what: added — the lint answer: one finding with its id, defect, bounds and reason
@task_file n=9 path=packages/escher-driver/src/json_tests.rs
what: added — the answer writer's unit tests with the 49 literal JSON pins: strings, numbers, nodes, every outcome's fields in order, layout words, refusals, session errors, session answers and the coverage line
@task_item n=1 part=touches
The escher-session snapshot answer contract (contract/escher-session-snapshot/answer, and --part): its writer moved into its own family module; the written fields and order are unchanged, so the row should read as still true.
@task_item n=2 part=touches
The escher-session layout answer contract (contract/escher-session-layout/answer): the layout writer moved into its own family module with its words and lengths byte-for-byte unchanged.
@task_item n=3 part=touches
The escher-session graph answer contract (contract/escher-session-graph/answer): graph node and graph state writers moved into a family module; no field or order moved.
@task_item n=4 part=touches
The escher-session lint answer contract (contract/escher-session-lint/answer): the finding writer moved into a family module; no reason or bounds field moved.
@task_item n=5 part=touches
The escher-session actions answer contract (contract/escher-session-actions/answer): the action node writer moved into a family module unchanged.
@task_item n=6 part=touches
The escher-session screenshot answer and MCP screenshot result (contract/escher-session-screenshot/answer, contract/escher-session-mcp/screenshot-result): the answer without its image stays in the parent writer with its doc comment now on its own item; bytes unchanged.
@task_item n=7 part=touches
The escher-session coverage answer (contract/escher-session-coverage/answer): its line writer stays in the parent writer; its test, keying each handler and leaving out an absent owner, moved out of line unchanged.
@task_item n=8 part=touches
The escher-session refusals and exit codes (contract/escher-session-refusals/*, contract/escher-session-exit-codes/*) and help answer: refusals and session errors are still written by the parent writer from their fixed strings; nothing moved.
@task_item n=9 part=touches
Indirect: decision/no-caller-input-echoed rules over what the answer writer writes; the split moved writers between modules without changing any written value, so it still holds.
@task_item n=10 part=touches
Indirect: decision/session-recording-replayable-check and the recording contracts (contract/escher-session-record/*): the stand replay of the committed Counter checks passed with every kept answer compared whole, and the checks are untouched.
@task_item n=11 part=touches
Indirect: decision/session-roles-share-stand-boot and the session's served tasks: agent-run session passed after boot; no role or task changed.
@task_item n=12 part=touches
A value a row may hold: escher-driver's coverage floor of 97 % held at 97.68 %; and the writer's inline unit test count moved to a #[path] test file (9 tests) — a touch only where a row's cell holds either.
@task_item n=1 part=untouched
The stand's stable element ids (contract/stand-element-ids/*) and the stand decisions on keys, ids, accent colour, a11y attributes, viewport and lints at boot: no stand app, view or attribute changed.
@task_item n=2 part=untouched
The ShellProvider window chrome controls (contract/shell-provider-window-chrome/*): no shell or window code changed.
@task_item n=3 part=untouched
The agent-run.sh and cold-agent.sh command contracts and the cold-agent decisions: no script, usage line, exit code or record dir changed.
@task_item n=4 part=untouched
The multi_doc and paint_bench command lines: no benchmark or example binary changed.
@task_item n=5 part=untouched
The MCP server's transport, protocol revisions, tools list, result bound and end (contract/escher-session-mcp/stdio, protocol-revisions, tools, result-bound, end) and the host log service name: no server or transport code changed.
@task_item n=6 part=untouched
The stand's interaction decisions (CRUD keyboard region, flight type select, open list rows, into-view scroll, visible region through clipping, held instance, load page network): no DOM, layout engine or stand behaviour changed.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates and the fixed document skeleton: the document crate is untouched.
