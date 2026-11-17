@task_prose part=session
The session added `layout --id` as the fifteenth driver verb, the eleventh instance verb, after `actions`, on every ratified carrier: the session socket call, the command line, the MCP tool and a kept check; it reads why an element sits where it does at its size and changes nothing.
Its answer is one JSON line of ids, closed kebab-case words and numbers only: id, bounds, laid_out, placed, shifts, width, height; a missing id is refused malformed with status 1, an unread id not-found, a deleted row's id stale.
An element with no box that the snapshot lists now answers placed by unexplained at its snapshot bounds, a new driver placer.
Counts moved: driver verbs 14 to 15, instance verbs 10 to 11, MCP tools 14 to 15 with help still last, the actions answer's per-node verbs 4 to 5 with layout last and no cause, Cells actions answer at boot 106,088 to 118,292 bytes, act-spans calls per mode 38 to 40, four field kinds added (13 to 17).
Explored verbs stay click, type and scroll: exploration leaves out a verb by a field of image or layout-reading kind; the twelve actions digests of the cross-OS readings were re-read and held on linux, windows and macOS; the cold-agent pipe's verb list stays at fourteen (founder fork F1, provisional).
Four tests were renamed for the new counts and kinds; no cause, session error, exit status, node key, span field, IPC method, crate or CI leg was added.
@task_file n=1 path=packages/escher-driver/src/layout.rs
what: the driver's layout reader, the in-process Session layout read and its placer set, now with an unexplained placer — edited
@task_file n=2 path=packages/escher-driver/src/layout_tests.rs
what: the layout reader's unit tests, the unexplained element read — edited
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver verb table (VERBS) and its levels, now fifteen verbs with the layout row after actions — edited
@task_file n=4 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's spec types, the field kinds a verb's result holds (four layout kinds added) — edited
@task_file n=5 path=packages/escher-driver/src/schema_tests.rs
what: the verb table's unit tests, the verbs named in order each at its level (renamed for fifteen) — edited
@task_file n=6 path=packages/escher-driver/src/command.rs
what: the driver command set and its validation, the Layout command with one required id — edited
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the driver session's run of a call, its outcomes and kept-check result recording, the Layout outcome — edited
@task_file n=8 path=packages/escher-driver/src/json.rs
what: the driver JSON answers, the written form of every outcome, now the layout answer and its closed words — edited
@task_file n=9 path=packages/escher-driver/src/explore.rs
what: the exploration of a screen's target verbs, which verbs are explored, by field kind — edited
@task_file n=10 path=packages/escher-driver/src/explore_tests.rs
what: the exploration's unit tests, the explored verbs left out by image or layout-reading field (renamed tests) — edited
@task_file n=11 path=packages/escher-driver/src/actions.rs
what: the actions answer, each listed node's verbs, now with layout fifth and no cause — edited
@task_file n=12 path=packages/escher-driver/src/describe.rs
what: the describe document built from the verb table, its verbs entries — edited
@task_file n=13 path=packages/escher-driver/src/mcp.rs
what: the MCP server role, the tools listed from the verb table — edited
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
what: the MCP server's unit tests, the tools/list listing the fifteen verbs with help last (renamed test) — edited
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
what: the driver command line's unit tests, the usage line and refused lines — edited
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: the session recording's unit tests, a kept check's record fields with a layout step — edited
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root and its crate doc naming the verbs and public types — edited
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the layout read, both layout modes, now over the layout verb, its refusals, its no-change and the unexplained element — edited
@task_file n=19 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer, each node's verbs and the Cells answer bound — edited
@task_file n=20 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings of the served tasks by answer digest, its digest table — edited
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check of the command spans of each call, calls per mode and instance verbs read back — edited
@task_file n=22 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP tools in process, the tool_call span parenting a command span — edited
@task_file n=23 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of kept-check replay through the ordinary call path, its stop on a changed step — edited
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: the session checks' shared module, its call helpers (now layout and explained) — edited
@task_file n=25 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of the scroll verb, its pin of the verb table's length (14 to 15) — edited
@task_file n=26 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the requirement sweep naming each requirement's witness inside stand-repeat, a witness renamed — edited
@task_file n=27 path=examples/seven_guis/tests/cli_commands.rs
what: the command-line checks run through the package's own binary, now a layout line — edited
@task_file n=28 path=examples/seven_guis/tests/mcp_server.rs
what: the MCP role tested as a child, the tools/list order and the layout tool's parity with the command line — edited
@task_file n=29 path=examples/seven_guis/tests/mcp_log.rs
what: the mcp role's stderr at trace, the check that no id, name or layout key reaches the log — edited
@task_file n=30 path=examples/seven_guis/tests/host_log.rs
what: the session host's stderr at trace, the check that no id, name or layout key reaches the log — edited
@task_file n=31 path=docs/capabilities.md
what: the capability statement pinned to the driver schema, now fifteen verbs and the layout claim — edited
@task_file n=32 path=README.md
what: the project readme, the fifteen driver commands as MCP tools — edited
@task_item n=1 part=touches
contract/driver-verbs/verb-table: the verb table now holds fifteen verbs, eleven at instance level, layout after actions; any cell stating fourteen or ten instance verbs is stale.
@task_item n=2 part=touches
contract/driver-session/reading-verbs: layout joins the reading verbs, a read that changes nothing.
@task_item n=3 part=touches
contract/session-layout/changes-nothing: the in-process layout read is now also reached by the layout verb through Session run, both modes, changing nothing and leaving delivered Timer ticks unapplied; the row's subject widens to the verb.
@task_item n=4 part=touches
decision/cross-os-readings-by-answer-digest: the twelve actions digests were re-read (the answer grew by the layout verb per node) and held on linux, windows and macOS; the 36 snapshot and lint digests are unmoved.
@task_item n=5 part=touches
contract/escher-session-command-line/usage-line: the usage line gains layout --id <id> after actions.
@task_item n=6 part=touches
contract/mcp-tools-list/order and contract/mcp-server/tools-list: the listing is fifteen tools, layout after actions, help last; a cell stating fourteen tools is stale.
@task_item n=7 part=touches
contract/driver-verbs/actions, contract/cli-actions/output, contract/mcp-actions/argument and decision/actions-answer-holds-no-value: each listed node's verbs move four to five, layout last with no cause; the answer still holds no value.
@task_item n=8 part=touches
A row holding the Cells actions answer's size or bound: the measured size at boot moves 106,088 to 118,292 bytes, both modes, still within its bound.
@task_item n=9 part=touches
contract/command-span/verb, contract/stand-log/command-span and contract/tool-call-span rows: the verb set the span's verb field takes gains layout; an accepted layout call records verb alone, a refused one verb and cause; no span field added and the read opens no span of its own.
@task_item n=10 part=touches
contract/session-recording/exploration-checks and the exploration rows: explored verbs stay click, type and scroll, a verb now left out by a field of image or layout-reading kind (Rect, Placement, Shifts, Size), not by its name.
@task_item n=11 part=touches
contract/check-replay/stop, decision/replay-through-the-call-path and contract/session-recording/record-fields: a kept layout step replays and a changed one stops naming placed.
@task_item n=12 part=touches
contract/driver-session/run-carriers and decision/socket-carries-commands: the layout verb is carried by the socket call, the command line, MCP and a kept check, each writing the one same JSON line.
@task_item n=13 part=touches
decision/session-writes-no-screen-content, decision/host-stderr-no-screen-content, decision/session-logs-no-screen-content and decision/no-ids-or-names-in-stand-logs: the host and mcp logs at trace are now also read for zero layout-answer keys (laid_out, placed, shifts) besides ids and names.
@task_item n=14 part=touches
decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-check-names-held-to-schema and decision/capability-claims-name-their-checks: the statement now says fifteen verbs and names layout; its pin still reads OK.
@task_item n=15 part=touches
decision/driver-instance-verbs-tested-by-stand and decision/stand-checks-in-both-layout-modes: the eleventh instance verb is tested by stand checks in both layout modes.
@task_item n=16 part=touches
decision/layout-reason-words-each-read: the placer set gains unexplained, read on a display-contents group fixture the snapshot lists, both modes.
@task_item n=17 part=touches
contract/refusal-cause/malformed, contract/refusal-cause/not-found, contract/refusal-cause/stale and decision/refusal-leaves-instance-unchanged: the layout verb refuses a missing or oversize id or a second argument malformed (status 1), an unread id not-found, a deleted row's id stale, each changing nothing; no cause added.
@task_item n=18 part=touches
contract/cold-agent-verdict/isolated and decision/cold-agent-session-isolated: the cold-agent pipe's verb list stays fourteen while the server now lists fifteen tools, so a live run reads not-isolated until the pipe is widened or isolated reads the listing less layout (CARRY on Cold-agent bar, F1 provisional).
@task_item n=19 part=touches
A row whose guard cell names a renamed test: the_fourteen_verbs_are_named_in_order_each_at_its_level is now the_fifteen_verbs_..., the_listing_is_the_fourteen_verbs_of_the_table is now the_listing_is_the_fifteen_verbs_of_the_table, the_verbs_explored_are_the_tables_target_verbs_with_no_image gains _and_no_layout_reading, a_verb_whose_result_holds_an_image_is_left_out_... is now a_verb_whose_result_holds_an_image_or_a_layout_reading_is_left_out_by_its_field_not_its_name.
@task_item n=20 part=touches
contract/cli-help/output, contract/escher-session-command-line/help and contract/mcp-help/parity: help, built from the verb table, now names layout among fifteen verbs.
@task_item n=21 part=touches
decision/mcp-tested-as-child-and-in-process and decision/cli-checks-through-own-binary: the layout tool's text read through MCP equals the command line's line, checked as a child and in process.
@task_item n=22 part=touches
decision/stand-repeat-three-rounds and decision/one-test-target-per-file: stand_layout grows 7 to 11 tests and now declares the shared fixtures module, still inside stand-repeat by its prefix; no target added.
@task_fact n=1 slug=integration-test-strategy
fact: The layout verb takes one required id and answers one line of id, bounds, laid_out, placed, shifts, width and height, ids, closed kebab-case words and numbers only, read through Session run on every node of the six served screens in both layout modes and equal to the in-process read.
@task_fact n=2 slug=integration-test-strategy
fact: An element the snapshot lists that has no box answers the layout verb placed by unexplained at its snapshot bounds, with its layout causes and sizes as before.
@task_fact n=3 slug=e2e-test-strategy
fact: The command line's layout --id <id> --session <dir> answers one line and exits 0, or is refused malformed with exit 1 without an id; the MCP layout tool's text equals that line.
@task_fact n=4 slug=3-test-harness-contract
fact: The cold-agent pipe's verb list stays at fourteen without layout (founder fork F1, the conservative option, provisional until his answer); widening it to fifteen is a carry on the cold-agent bar.
@task_fact n=5 slug=session-lifecycle
fact: A kept check's layout step replays through the ordinary call path, and a step whose placed reading changed stops the replay naming placed.
@task_fact n=6 slug=test-strategy
fact: A verb-count change sweeps the tests and examples for VERBS.len() and VERBS[ pins, and a test rename sweeps the old name held as a string literal, such as a requirement's witness.
@task_item n=1 part=untouched
The WPT conformance runner, its suites, statuses, expectations and fork reading: no conformance surface moved.
@task_item n=2 part=untouched
CI legs, jobs, caches, package installs, coverage floors, mutation testing and the zero-failure stand-repeat budget: no leg or script changed and escher-driver held its coverage floor.
@task_item n=3 part=untouched
The agent-run five-command contract, its events, exit codes, selections and state: no agent-run surface changed.
@task_item n=4 part=untouched
The session wire protocol, its version, bounds, hello and lifecycle replies, the session state directory and socket: no IPC method, socket, port or env var was added.
@task_item n=5 part=untouched
Session errors, exit codes and the refusal cause set and its order: none added or reordered.
@task_item n=6 part=untouched
Harness construction, input helpers, settle, virtual clock, timer ticks, contrast and screenshot contracts: the harness was not changed.
@task_item n=7 part=untouched
Lint defects and findings, snapshot node shape and the snapshot text budget: no lint or snapshot rule moved and their digests held.
@task_item n=8 part=untouched
Test data, fixture pages, mocking discipline and network stand-ins: no stand-in or fixture page changed beyond a test-local group element.
@task_item n=9 part=untouched
The cold-agent pipe's events, exit codes, run bounds, replay and scrubbing: the pipe itself was not changed.
@task_item n=10 part=untouched
Handler coverage and its limits, the graph and advance verbs, typed passwords and select painting: none reached.
