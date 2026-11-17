@task_prose part=session
The session added a fifteenth escher-session verb, layout --id <id>, on every carrier: the session socket call, the command line, the MCP server's tools and a kept check in a recording.
The layout answer is one line holding the id, bounds, laid_out, placed, shifts, width and height, written only as ids, closed kebab-case words and numbers; it changes nothing in the session.
An element the snapshot lists but that has no layout box now reads placed by unexplained at its snapshot bounds.
Counts moved: driver verbs 14 to 15, instance verbs 10 to 11, MCP tools listed 14 to 15 (layout after actions, help last), per-node verbs in the actions answer 4 to 5 (layout last, no cause).
Measured: the Cells actions answer at boot grew from 106,088 to 118,292 bytes in both layout modes; the twelve actions digests of the cross-OS readings were re-read and held on linux, windows and macOS.
Nothing retired in code; no new exit status, refusal cause, session error, node key, span field, IPC method or env var. The cold-agent pipe stays at fourteen verbs (provisional, awaiting the founder).
@task_file n=1 path=packages/escher-driver/src/layout.rs
what: the driver's layout explanation read (why an element sits where it does at its size), now with an unexplained placer for an element with no box
@task_file n=2 path=packages/escher-driver/src/layout_tests.rs
what: unit tests of the driver's layout explanation read
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver's verb table, the list of escher-session verbs with their arguments, result fields and level, now fifteen with layout after actions
@task_file n=4 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's field kinds, gaining rect, placement, shifts and size
@task_file n=5 path=packages/escher-driver/src/schema_tests.rs
what: unit tests pinning the verb table's verbs in order, each at its level
@task_file n=6 path=packages/escher-driver/src/command.rs
what: the escher-session command form, parsing and validating a verb and its arguments (layout requires one id; without it the call is malformed)
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the session's run of a command and its outcome, and which outcomes a recording keeps as non-acting
@task_file n=8 path=packages/escher-driver/src/json.rs
what: the one-line JSON answer each verb writes, now with the layout answer's writers and its closed words
@task_file n=9 path=packages/escher-driver/src/explore.rs
what: the explorer's choice of verbs to try on a node, by field kind (no image, no layout reading)
@task_file n=10 path=packages/escher-driver/src/explore_tests.rs
what: unit tests of the explorer's explored verbs
@task_file n=11 path=packages/escher-driver/src/actions.rs
what: the actions answer's per-node verb list, the target verbs of the verb table
@task_file n=12 path=packages/escher-driver/src/describe.rs
what: the describe document listing the verbs for help and tools
@task_file n=13 path=packages/escher-driver/src/mcp.rs
what: the escher-session MCP server over stdio and its tools/list
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
what: unit tests of the MCP server's tool listing and order
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
what: unit tests of the escher-session command line, its usage line and its malformed refusals
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: unit tests of the session recording and its kept check files
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: the driver crate's front doc naming the verbs and the public placer set
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the layout read and the layout verb over the served screens, both layout modes
@task_file n=19 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer, its per-node verbs and the Cells answer size bound
@task_file n=20 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings check, a table of answer digests that must agree on every OS
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check of each verb call's command span and the instance verbs read back
@task_file n=22 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of each MCP tool call, its answer and its tool_call span
@task_file n=23 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check that a kept check replays and stops on a changed answer
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: shared helpers of the stand session checks, now with a layout call and its explanation
@task_file n=25 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of scroll, which also pins the verb table's length
@task_file n=26 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the requirement sweep naming each requirement's witness test inside stand-repeat
@task_file n=27 path=examples/seven_guis/tests/cli_commands.rs
what: the seven_guis check of the escher-session command line, each verb answering one line
@task_file n=28 path=examples/seven_guis/tests/mcp_server.rs
what: the seven_guis check of the escher-session MCP server's tools/list and a tool's text equal to the command line's
@task_file n=29 path=examples/seven_guis/tests/mcp_log.rs
what: the seven_guis check that the MCP server's log at trace holds no id, name or layout key
@task_file n=30 path=examples/seven_guis/tests/host_log.rs
what: the seven_guis check that the session host log at trace holds no id, name or layout key
@task_file n=31 path=docs/capabilities.md
what: the public capability statement, naming the fifteen verbs and the actions answer's size
@task_file n=32 path=README.md
what: the project front page, naming the fifteen driver commands as MCP tools
@task_item n=1 part=touches
contract/escher-session-command/form: the command line gains layout --id <id> after actions; a row stating fourteen verbs is now stale (fifteen), and a missing --id reads malformed, status 1.
@task_item n=2 part=touches
contract/escher-session-mcp/tools: tools/list moves from fourteen to fifteen tools, layout after actions, help still last; a row stating fourteen is stale.
@task_item n=3 part=touches
contract/escher-session-actions/answer: each listed node's verbs move from four to five, layout last with no cause; the Cells actions answer at boot grows from 106,088 to 118,292 bytes, a value a row may hold.
@task_item n=4 part=touches
contract/escher-session-mcp/result-bound: the Cells actions answer the bound is measured against is now 118,292 bytes, still within the bound; a row holding the old size or its margin moves.
@task_item n=5 part=touches
contract/escher-session-help/answer: the describe document behind help and tools now lists fifteen verbs, layout among the instance verbs (eleven, from ten).
@task_item n=6 part=touches
contract/escher-session-refusals/unknown-verb: layout is now a known verb, no longer refused unknown-verb; the refusal set itself gains no cause.
@task_item n=7 part=touches
contract/escher-session-exit-codes/1: a layout call with no id, an over-long id or a second argument exits 1 malformed; no exit status added.
@task_item n=8 part=touches
decision/session-recording-replayable-check and contract/escher-session-record/check-file: a kept check can now hold a layout step; its replay stops on a changed placed, naming it.
@task_item n=9 part=touches
decision/no-caller-input-echoed: the layout answer writes back the id it was asked for, alongside closed words and numbers only; a picker should check whether the rule names ids as allowed.
@task_item n=10 part=touches
decision/cold-agent-uses-driver-mcp and the cold-agent rows: the server now lists fifteen tools while the cold-agent pipe stays at fourteen without layout (provisional, the founder's F1); until its list widens or reads the listing less layout, a live run reads not-isolated.
@task_item n=11 part=touches
contract/escher-session-snapshot/answer: an element the snapshot lists with no layout box (a display: contents group) now answers layout with placed by unexplained at its snapshot bounds.
@task_item n=12 part=touches
decision/held-instance-reads-as-fresh-boot: the actions answer's fifth verb, layout, is compared on the held session as screenshot is.
@task_item n=13 part=touches
decision/visible-region-through-clipping-boxes, decision/scroll-into-view-moves-every-scroller and decision/open-list-rows-own-bounds: the layout answer reports an element's bounds and laid_out box and its shifts by scroll or transform with their holder, beside these bounds rules.
@task_item n=14 part=touches
decision/session-roles-share-stand-boot: every node of the six served screens answers the layout verb in both layout modes, the verb equal to the in-process read.
@task_item n=15 part=touches
contract/escher-session-host/log-service-name and the session's log rows: an accepted layout call's command span records verb alone, a refused one verb and cause; logs at trace hold no layout key (laid_out, placed, shifts).
@task_item n=16 part=touches
Names taken: the verb and MCP tool name layout; the placing word unexplained and the field kind names rect, placement, shifts and size.
@task_fact n=1 slug=surface-cli
fact: escher-session layout --id <id> answers one line with id, bounds and laid_out (x, y, width, height), placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy) and width and height (px, by, size, min, max, flex with basis, grow, shrink); ids, closed words and numbers only.
@task_fact n=2 slug=surface-cli
fact: The layout answer's placing words are block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained and app; shift words scroll and transform.
@task_fact n=3 slug=surface-cli
fact: The layout answer's size causes are style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content and layout; a length is auto, none, min-content, max-content, fit-content, stretch, or px, percent or calc with a number.
@task_fact n=4 slug=surface-cli
fact: An engine value later than this build writes a fallback word: unexplained for a placer, transform for a shift, layout for a cause, auto for a length; the writer never panics.
@task_fact n=5 slug=surface-cli
fact: An element the snapshot lists but the layout engine gives no box answers placed by unexplained at its snapshot bounds.
@task_fact n=6 slug=surface-cli
fact: The layout verb changes nothing in the session and leaves delivered Timer ticks unapplied; an unread id is refused not-found and a deleted row's id stale, nothing changed.
@task_fact n=7 slug=surface-cli
fact: The layout MCP tool's text equals the command line's one line for the same id; every carrier writes the same answer line.
@task_fact n=8 slug=surface-cli
fact: The explorer tries only click, type and scroll; a verb whose result holds an image or a layout reading is left out by its field kind, not its name.
@task_fact n=9 slug=surface-cli
fact: The cold-agent pipe's verb list stays at fourteen without layout while the MCP server lists fifteen (provisional, the founder's F1 conservative option), so a live run reads not-isolated until the pipe widens or reads the listing less layout.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts (drag, maximize, minimize, close, decorations): no window chrome control changed.
@task_item n=2 part=untouched
The stand stable element ids and the stand's own look and markup decisions (accent colour, document skeleton, ids, names, roles and states by attributes, lints clean at boot): no stand id, attribute or style changed.
@task_item n=3 part=untouched
The agent-run.sh test contract, the multi_doc and paint_bench command lines: none of their arguments, outputs or exits moved.
@task_item n=4 part=untouched
The escher-session coverage role, graph, lint and screenshot answers and the served tasks: their shapes and content are unchanged.
@task_item n=5 part=untouched
The MCP server's protocol revisions, stdio transport and end, and the recording's record-dir-not-private and recording-failed refusals: unchanged.
@task_item n=6 part=untouched
The mobile-native and web-spa surfaces and the arbitrary index.html templates absence: nothing reached them.
