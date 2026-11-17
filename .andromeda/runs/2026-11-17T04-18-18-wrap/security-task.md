@task_prose part=session
Added layout, the fifteenth driver verb, at the instance level after actions: it takes one required id and answers why that element sits where it does at its size, using only ids, closed words and numbers. The verb changes nothing.
The layout answer now crosses every ratified carrier: the session socket call, the command line, the MCP tool (after actions, with help last) and a kept check at rest that replay compares. Before this session the layout read was in process only.
Driver verbs went from 14 to 15, instance verbs from 10 to 11, MCP tools from 14 to 15 and the per-node verbs in the actions answer from 4 to 5, with layout last and no cause. The Cells actions answer at boot was measured at 118,292 bytes, up from 106,088.
An element the snapshot lists but the engine cannot explain now answers the new unexplained placer at its snapshot bounds. Each JSON writer has a fallback word for later engine set members, so there is no panic path.
Exploration still covers only click, type and scroll, now chosen by field kind, so verbs whose results hold an image or a layout reading are left out. The cold-agent pipe's verb list stays at fourteen (founder fork F1, provisional).
No new IPC method, socket, environment variable, crate, dependency, refusal cause, session error, exit status, node key or span field was added.
@task_file n=1 path=packages/escher-driver/src/layout.rs
what: edited the in-process layout read on a session, which explains an element's placement, shifts and sizes; it now answers the unexplained placer
@task_file n=2 path=packages/escher-driver/src/layout_tests.rs
what: edited the unit tests of the layout read and its unexplained element
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: edited the closed driver verb schema, the verb table and its single validation path, adding the layout row with its id argument and seven result fields
@task_file n=4 path=packages/escher-driver/src/schema/spec.rs
what: edited the verb table's field kinds, adding the rect, placement, shifts and size kinds
@task_file n=5 path=packages/escher-driver/src/schema_tests.rs
what: edited the tests of the verb table and its order, which now count fifteen verbs and eleven instance verbs
@task_file n=6 path=packages/escher-driver/src/command.rs
what: edited the driver command enum and its validation by the schema, adding the layout command
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited the session's command run, which checks targets and records the result; layout is added as a non-acting outcome that records only verb and cause
@task_file n=8 path=packages/escher-driver/src/json.rs
what: edited the outcome's one written line used by every carrier, adding the layout answer's writer with closed words, numbers and fallback words
@task_file n=9 path=packages/escher-driver/src/explore.rs
what: edited the exploration of target verbs, which now leaves out a verb by its image or layout field kind
@task_file n=10 path=packages/escher-driver/src/explore_tests.rs
what: edited the exploration tests, including the renamed test for verbs left out by field kind
@task_file n=11 path=packages/escher-driver/src/actions.rs
what: edited the actions read, whose per-node verbs list now includes layout fifth with no cause
@task_file n=12 path=packages/escher-driver/src/describe.rs
what: edited the describe document built from the verb table, which now has fifteen verbs entries
@task_file n=13 path=packages/escher-driver/src/mcp.rs
what: edited the MCP server over stdio and its fixed tool listing, which now holds fifteen tools
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
what: edited the MCP server unit tests, so the listing is the fifteen verbs and layout's tool comes after actions
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
what: edited the command line's closed argv reader tests and the usage line, which now includes layout with --id
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: edited the recorded check tests, so a kept check holds a layout step and its answer
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: edited the driver crate doc that states the verb count and the Placer enum's unexplained variant
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: edited the layout stand, which checks that the verb answers the in-process read, refuses not-found and stale ids, changes nothing and answers unexplained
@task_file n=19 path=tests/blitz-tests/tests/stand_actions.rs
what: edited the actions answer stand, covering the per-node verbs and the Cells answer size bound pinned at 118,292 bytes
@task_file n=20 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited the cross-OS answer digest table, where the twelve actions digests were re-read
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited the command span stand, where calls per mode go from 38 to 40 and instance verbs read back eleven
@task_file n=22 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: edited the MCP tool stand, so a layout tool_call parents a command span that records only its verb
@task_file n=23 path=tests/blitz-tests/tests/stand_replay.rs
what: edited the kept check replay stand, so a kept layout step replays and a changed placed value stops it
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited the shared session test helpers, adding the layout call and its explanation reader
@task_file n=25 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: edited the scroll stand, whose pin on the verb table count moves from 14 to 15
@task_file n=26 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: edited the requirement witness sweep, which now names the renamed verb table test
@task_file n=27 path=examples/seven_guis/tests/cli_commands.rs
what: edited the escher-session command line tests, covering the layout answer line and exit codes 0 and 1
@task_file n=28 path=examples/seven_guis/tests/mcp_server.rs
what: edited the escher-session mcp role tests, covering the tool listing and the layout tool's text matching the command line's
@task_file n=29 path=examples/seven_guis/tests/mcp_log.rs
what: edited the mcp role's log check, which confirms that no id, name or layout key reaches the log at trace
@task_file n=30 path=examples/seven_guis/tests/host_log.rs
what: edited the session host's log check, which confirms that no id, name or layout key reaches the log at trace
@task_file n=31 path=docs/capabilities.md
what: edited the committed capability statement, which now has fifteen verbs, names layout and gives the Cells actions answer size
@task_file n=32 path=README.md
what: edited the public README line about the fifteen driver commands served as MCP tools
@task_item n=1 part=touches
decision/session-layout-read-in-process is stale: the layout read now leaves the process through the layout verb on the socket, the command line, MCP and a kept check.
@task_item n=2 part=touches
decision/socket-carries-screen-reading gains the layout answer (ids, closed words and numbers) among what the session socket carries.
@task_item n=3 part=touches
contract/actions-answer/verbs moves: every listed node now carries layout fifth, with no cause.
@task_item n=4 part=touches
The row that names a kept check's tool as one of the ten instance verbs is stale: there are now eleven, with layout added.
@task_item n=5 part=touches
decision/closed-driver-verb-schema and decision/single-validation-gate change: the closed verb schema grows to fifteen verbs with a layout row (a required id), and four new result field kinds (rect, placement, shifts, size) take the field kinds from 13 to 17.
@task_item n=6 part=touches
The contract/driver-verbs rows (actions, graph) gain a sibling, since no layout row exists yet. The layout verb is accepted with an id and refused as malformed without one, with an id past the length bound, or with a second argument.
@task_item n=7 part=touches
decision/mcp-listing-fixed-text and the MCP tool listing rows: the listing grows from 14 to 15 tools, with layout after actions and help last.
@task_item n=8 part=touches
decision/stable-id-exits, decision/attribute-text-leaves-by-ratified-carriers and contract/stable-element-id/carriers: the stable id now also leaves in the layout answer's id field and in placed container and shift holder ids, on every ratified carrier.
@task_item n=9 part=touches
decision/closed-value-field-no-widening and decision/boundary-widening-rule apply to the layout answer: it adds closed placing, shift, cause and length words plus numbers to every carrier.
@task_item n=10 part=touches
decision/kept-check-answer-whole, decision/kept-check-call-content, contract/kept-check-file/contents and contract/recorded-check/answer: a kept check now holds a layout step and its whole answer at rest, and replay stops on a changed placed value.
@task_item n=11 part=touches
decision/one-validation-path-for-every-carrier and contract/driver-argument-kinds/id: layout's id passes the same Id-kind check on the socket, the command line and MCP.
@task_item n=12 part=touches
contract/refusal-causes/malformed, not-found and stale and decision/unknown-id-refused-stale-or-not-found: layout refuses an unread id as not-found and a deleted row's id as stale, changing nothing. A missing --id on the command line reads malformed with status 1, not a usage error.
@task_item n=13 part=touches
decision/actions-read-shares-target-check and decision/target-refusal-order: layout has a target-check arm that checks nothing beyond resolving the id, and it does no second resolution.
@task_item n=14 part=touches
decision/command-span-content-free, contract/command-span/verb, contract/driver-command-span/verb and contract/tool-call-span/tool: the verb and tool word sets grow by layout. An accepted call records only its verb, a refused one records verb and cause, and no span field is added.
@task_item n=15 part=touches
decision/no-screen-content-in-logs, decision/stable-id-not-logged and decision/driver-outcomes-never-logged: the host and mcp logs carry no id, name or layout key at trace, as measured.
@task_item n=16 part=touches
decision/actions-answer-crossing, decision/actions-answer-unpaged and contract/mcp-tool-result/size-bound: the Cells actions answer at boot grows from 106,088 to 118,292 bytes and still holds within the bound.
@task_item n=17 part=touches
decision/capability-statement-mcp-crossing: the committed capability statement now gives fifteen verbs and names layout.
@task_item n=18 part=touches
contract/escher-session-exit-codes/0 and 1, contract/cli-exit-codes/0 and 1, contract/cli-flags/id and contract/escher-session-stdout/command-line: layout --id --session answers one line with exit 0 and refuses with exit 1, and the usage line now includes layout.
@task_item n=19 part=touches
decision/cold-agent-call-validation and decision/cold-agent-wrong-call-bar: the cold-agent pipe keeps its fourteen verbs (F1, provisional), so a live run reads not-isolated until the pipe accounts for layout.
@task_item n=20 part=touches
decision/no-node-id-between-calls, decision/id-lookup-anew-each-call and decision/no-process-handle-crosses hold for the new verb: it looks up its id anew, and only stable ids cross.
@task_fact n=1 slug=threat-model-summary
fact: The layout answer crosses every ratified carrier (the session socket, the command line, MCP and a kept check at rest). It holds only stable ids, closed kebab-case words and numbers, and no text, attribute or name.
@task_fact n=2 slug=input-validation
fact: The layout verb is an instance verb with one required id argument. It changes nothing in the session and leaves delivered Timer ticks unapplied. It answers not-found or stale for an unknown id, and malformed without an id or with a second argument.
@task_fact n=3 slug=input-validation
fact: The layout answer's words are closed sets. The placing words are block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained and app. The shift words are scroll and transform. The cause words are style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content and layout. A later engine member writes a fallback word, never a panic.
@task_fact n=4 slug=input-validation
fact: An element the snapshot lists that the engine cannot explain answers the unexplained placer at its snapshot bounds, not an engine placer.
@task_fact n=5 slug=data-protection
fact: A kept check holds a layout step and its whole answer, and replay stops on a changed placed value.
@task_fact n=6 slug=input-validation
fact: Exploration covers only target verbs whose results hold no image and no layout reading, by field kind, so layout is never explored.
@task_fact n=7 slug=input-validation
fact: As a provisional exception (founder fork F1), the cold-agent pipe keeps its verb list at fourteen without layout. Until the founder decides, a live run reads not-isolated.
@task_item n=1 part=untouched
Dependency security is untouched: no crate, dependency, lockfile pin or audit and licence gate changed.
@task_item n=2 part=untouched
Secret management is untouched: no environment variable, CI secret, signing material or login handling changed.
@task_item n=3 part=untouched
Outbound network rows are untouched: script fetch, sub-resource requests, CORS and same-origin absences, per-host limits, TLS and the URL-taking driver verb absence.
@task_item n=4 part=untouched
Browser storage and dialogs are untouched: history in sqlite, cookies, HTTP cache, clipboard and the file dialog.
@task_item n=5 part=untouched
The select list painting and keys, password and file masking, and range input rows are untouched: no control behaviour or mask changed.
@task_item n=6 part=untouched
The session socket's transport, bounds, request grammar, connections, idle expiry and owner-only state directory are untouched: there is no new IPC method or reply shape.
@task_item n=7 part=untouched
The CI workflows, WPT runner, release publishing, apt-install and agent-run harness rows are untouched: no leg or script changed.
@task_item n=8 part=untouched
The coverage role, coverage floors and coverage answer line are untouched.
@task_item n=9 part=untouched
The authentication and authorization absences and the no-peer-authentication decisions are untouched.
