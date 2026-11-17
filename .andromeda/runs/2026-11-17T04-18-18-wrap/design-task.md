@task_prose part=session
The session added the fifteenth driver verb, layout with one required id, on every carrier: the session socket call, the escher-session command line, the MCP role and a kept check; it reads why an element sits where it does at its size and changes nothing.
The layout answer is one line of ids, closed kebab-case words and numbers only: id, bounds, laid_out, placed (by, container, canvas, at, relative), shifts (by, holder, dx, dy), width and height (px, by, size, min, max, flex).
An element the snapshot lists that has no layout box now answers placed by unexplained at its snapshot bounds, a new placer word, in place of block.
The actions answer now lists layout as each node's fifth verb, with no cause; the Cells actions answer at boot grew from 106,088 to 118,292 bytes (measured, both layout modes).
A missing id is refused malformed with status 1, not a usage error; no new cause, session error, exit status, node key or span field; the MCP tool list is fifteen, layout after actions and help last; explored verbs stay click, type and scroll.
The cold-agent pipe stays at fourteen verbs (a provisional founder fork); the in-process-only statement of the layout read is now false.
@task_file n=1 path=packages/escher-driver/src/layout.rs
what: the driver's layout read, the explanation of an element's placement, shifts and sizes, now with an unexplained placer for an element with no box
@task_file n=2 path=packages/escher-driver/src/layout_tests.rs
what: unit checks of the driver's layout read, its unexplained element
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver's verb table, its fifteen verbs with their arguments, result fields and levels, now holding the layout row
@task_file n=4 path=packages/escher-driver/src/schema_tests.rs
what: checks of the verb table, the fifteen verbs named in order each at its level
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's field kinds, gaining rect, placement, shifts and size
@task_file n=6 path=packages/escher-driver/src/command.rs
what: the driver's command set and its argument validation, gaining the layout command
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the session's run of a command into an outcome, the layout outcome and the non-acting outcomes recorded in the command span
@task_file n=8 path=packages/escher-driver/src/json.rs
what: the one-line JSON writer of every verb's answer, now writing the layout answer's closed words and numbers
@task_file n=9 path=packages/escher-driver/src/explore.rs
what: the explorer's choice of target verbs, leaving out a verb whose result holds an image or a layout reading
@task_file n=10 path=packages/escher-driver/src/explore_tests.rs
what: checks of the explored verbs by field kind
@task_file n=11 path=packages/escher-driver/src/actions.rs
what: the actions answer, each listed node's verbs and their causes, now with layout fifth
@task_file n=12 path=packages/escher-driver/src/describe.rs
what: the describe document built from the verb table, its verbs entries
@task_file n=13 path=packages/escher-driver/src/mcp.rs
what: the escher-session MCP role, its tool listing of the driver commands as tools
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
what: checks of the MCP role's tool listing, the layout tool after actions and help last
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
what: checks of the escher-session command line, its usage line and its refusals
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: checks of the kept check recording, a kept layout step
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: the driver crate's doc, naming the fifteen verbs
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the integration check of the layout read and the layout verb on the served screens, both layout modes
@task_file n=19 path=tests/blitz-tests/tests/stand_actions.rs
what: the integration check of the actions answer, its per-node verbs and the Cells answer size bound
@task_file n=20 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS check of answer digests, its actions digests re-read
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the integration check of the command span each verb call leaves, its verb and cause fields
@task_file n=22 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the integration check of MCP tool calls and their tool_call span
@task_file n=23 path=tests/blitz-tests/tests/stand_replay.rs
what: the integration check of kept check replay, stopping on a changed answer
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: shared test helpers for driver sessions, gaining a layout call and its explanation reader
@task_file n=25 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the integration check of the scroll verb, pinning the verb count
@task_file n=26 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the requirement sweep naming each requirement's witness test
@task_file n=27 path=examples/seven_guis/tests/cli_commands.rs
what: the end-to-end check of the escher-session command line on the seven GUIs tasks, the layout command's one-line answer
@task_file n=28 path=examples/seven_guis/tests/mcp_server.rs
what: the end-to-end check of the escher-session MCP role, its tool list and a layout read equal to the command line's
@task_file n=29 path=examples/seven_guis/tests/host_log.rs
what: the check that the session host's log holds no id, name or layout keys at trace
@task_file n=30 path=examples/seven_guis/tests/mcp_log.rs
what: the check that the MCP role's log holds no id, name or layout keys at trace
@task_file n=31 path=docs/capabilities.md
what: the public capability statement, the fifteen verbs and the actions answer size
@task_file n=32 path=README.md
what: the project readme, the fifteen driver commands as MCP tools
@task_item n=1 part=touches
The escher-session commands sheet gains a verb: layout --id answers one line, a sibling of the screenshot, snapshot, actions, graph and coverage answer and argument rows, which hold no layout row yet.
@task_item n=2 part=touches
The actions answer row and the actions line nodes row: each listed node's verbs move from four to five, layout last, with no cause; a row naming four verbs or the node verb list is stale.
@task_item n=3 part=touches
The Cells task row and any row holding the Cells actions answer size: it moved from 106,088 to 118,292 bytes.
@task_item n=4 part=touches
The help command and the MCP help tool result: the listing now names fifteen verbs, layout after actions; any row counting fourteen verbs or fourteen tools is stale.
@task_item n=5 part=touches
The MCP role's other tool results and stdout rows: a layout tool result writes the same one line as the command line.
@task_item n=6 part=touches
The command line output and stdout rows: the layout command writes one line of ids, closed words and numbers, exit 0 on an answer and 1 on a refusal.
@task_item n=7 part=touches
The exit status and exit code rows 0 and 1, and the rule that exit statuses come from the schema's table: layout adds no status; a missing id reads malformed with status 1, not a usage error.
@task_item n=8 part=touches
The usage error rows on stderr: the usage line gains layout --id after actions; a missing --id is not a usage error.
@task_item n=9 part=touches
The session error object and its remedy: layout refuses an unread id not-found and a deleted row's id stale, changing nothing; no new session error.
@task_item n=10 part=touches
The decision that actions are decided by the verbs' own checks: layout's target check checks nothing, so every listed node carries it with no cause.
@task_item n=11 part=touches
The decisions that escher-session is read by an agent and that driver command line output is read by agents: the new answer is closed words and numbers, no colour, for an agent.
@task_item n=12 part=touches
The driver verbs and the virtual clock rows and the decision that the load wait moves no clock: layout changes nothing, moves no clock and leaves delivered Timer ticks unapplied.
@task_item n=13 part=touches
The select list overlay and its placement inside the viewport: the layout answer names an open select's list with the placer words select and select-list.
@task_item n=14 part=touches
The snapshot rows, including a row of an open select list's bounds: an element the snapshot lists with no layout box answers unexplained at its snapshot bounds.
@task_item n=15 part=touches
The decision that a focused select with no box keeps focus: a boxless element now has a layout answer, placed by unexplained.
@task_fact n=1 slug=surface
fact: escher-session layout --id answers one line {id, bounds, laid_out, placed, shifts, width, height}: bounds and laid_out as x, y, width, height; placed as by, container, canvas (app alone), at, relative; shifts as by, holder, dx, dy; width and height as px, by, size, min, max and flex basis, grow, shrink.
@task_fact n=2 slug=surface
fact: The layout answer holds ids, closed kebab-case words and numbers only: placers block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained, app; shifts scroll, transform; size causes style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content, layout; lengths auto, none, min-content, max-content, fit-content, stretch or px, percent, calc.
@task_fact n=3 slug=surface
fact: The layout answer writes a fallback word for an engine set's later member (unexplained, transform, layout, auto) rather than failing.
@task_fact n=4 slug=surface
fact: escher-session layout takes one required --id; with none, past the id bound or with a second argument it is refused malformed, status 1.
@task_fact n=5 slug=surface
fact: An element the snapshot lists that has no layout box answers placed by unexplained at its snapshot bounds, its causes and sizes read as before.
@task_fact n=6 slug=surface
fact: The MCP role lists layout as a tool after actions with help last, fifteen tools, its result equal to the command line's line.
@task_fact n=7 slug=surface
fact: Layout is a target verb of the actions answer, listed fifth on every node with no cause, and is not an explored verb; explored verbs stay click, type and scroll by field kind.
@task_fact n=8 slug=motion-calibrated-to-expression-level-unrecorded
fact: The layout verb changes nothing and moves no virtual clock; delivered Timer ticks stay unapplied across it.
@task_fact n=9 slug=surface-cli
fact: The cold-agent pipe stays at fourteen verbs, layout left out, a provisional founder choice pending his word.
@task_item n=1 part=untouched
Color palette, engine colours and color-scheme rows: no colour or token changed.
@task_item n=2 part=untouched
Typography and font rows: no font, fallback or screenshot text rule changed.
@task_item n=3 part=untouched
Depth, paint order, hit-testing and stacking rows: the layout answer reads placement, it changes no paint or hit order.
@task_item n=4 part=untouched
Iconography, cursor and favicon rows, and the shell provider's window rows: untouched.
@task_item n=5 part=untouched
WPT runner output and result colour rows: untouched.
@task_item n=6 part=untouched
Select list events (open, close, choose, highlight) and select accessibility rows: no event or tree changed.
@task_item n=7 part=untouched
JS window, MountedData, dioxus-native config and the web-spa surface: untouched.
@task_item n=8 part=untouched
Spacing presentational hints, motion timings of scrollbars and smooth scroll: no engine layout or scroll behaviour changed.
