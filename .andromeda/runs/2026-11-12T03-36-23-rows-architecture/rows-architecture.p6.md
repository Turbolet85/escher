@row_add n=1 row=decision/session-over-unix-socket area=established-decisions rec=1
name: Driver session over a std-only Unix socket
rule: A session crosses processes on a std-only Unix-domain socket in an owner-only state directory, with no third-party IPC, named-pipe, pid-file or daemonizing crate and no TCP or UDP port.
why: A deliberate transport choice, ratified.
@row_add n=2 row=decision/socket-carries-driver-calls area=established-decisions rec=1
name: Session socket carries driver calls
rule: The session socket carries the session's lifecycle and a driver call and its answer: a call takes an element id and typed text in, an answer takes ids, accessible names, control values (a password's or a file input's as the fixed mask), the snapshot's text and the diff out, and no NodeId crosses.
why: A boundary widening, ratified.
@row_add n=3 row=decision/cross-process-session-unsupported-on-windows area=established-decisions rec=1
name: Cross-process session unsupported on Windows
rule: On Windows the cross-process lifecycle and a call sent across it return Unsupported, while a session held in process works on every target.
why: Windows has no std local socket.
@row_add n=4 row=decision/session-library-depends-on-no-app area=established-decisions rec=1,32
name: Session library depends on no app
rule: The driver's session library takes the app's boot from its caller and depends on no app; no manifest under packages names an example, and the one edge runs from the app to the driver.
@row_add n=5 row=decision/driver-accessibility-feature area=established-decisions rec=1,32
name: Driver names only the accessibility feature
rule: The driver library names one feature, accessibility on dioxus-native-dom, unconditionally and with no features table of its own.
why: Its executor cannot be built without the snapshot model.
@row_add n=6 row=decision/driver-builds-no-platform-adapter area=established-decisions rec=1
name: Driver graph holds no platform adapter
rule: Binaries built through the driver compile dioxus-native-dom's accessibility modules yet build no platform adapter: accesskit_xplat and accesskit_winit are in neither graph.
@row_add n=7 row=decision/driver-uses-tracing area=established-decisions rec=1
name: tracing in the driver
rule: The driver takes tracing from the workspace with no cargo feature.
@row_add n=8 row=decision/serde-json-reader-only area=established-decisions rec=1
name: serde_json as a reader only
rule: The driver takes serde_json from the workspace with no feature, as a reader of MCP requests, of kept checks and of the live answer line a replay compares, writing nothing.
why: Ratified as read only.
@row_add n=9 row=decision/png-encodes-shots area=established-decisions rec=1
name: png encodes a shot, base64 by hand
rule: The workspace's png crate is the encoder of a shot, and the shot's base64 is written by hand with no crate taken for it.
@row_add n=10 row=decision/key-types-through-harness area=established-decisions rec=1
name: Key types through the harness's re-exports
rule: The key types the driver's press needs come through the test harness's re-exports Key, Modifiers and NamedKey, not as a dependency of the driver's own.
@row_add n=11 row=decision/driver-spans-only area=established-decisions rec=1
name: Driver opens spans and emits no event
rule: The driver opens one tracing span per call it runs and one tool_call span per MCP tools/call request, and emits no event, installs no subscriber, reads no env var and has no default state location.
@row_add n=12 row=contract/tool-call-span/tool area=established-decisions rec=1
boundary: the tool_call span
item: tool
promise: One of the only two fields the tool_call span holds.
@row_add n=13 row=contract/tool-call-span/ending area=established-decisions rec=1
boundary: the tool_call span
item: ending
promise: One of the only two fields the tool_call span holds.
@row_add n=14 row=decision/hosted-session-idle-expiry area=established-decisions rec=1
name: Hosted session idle expiry
rule: A hosted session expires: a host a command line starts takes IDLE_EXPIRY of 1,800 s, and a host that has answered no request for that long ends exactly as on stop; every answered request, a refused or malformed one included, starts the count again.
why: Answers the orphaned-host question.
@row_add n=15 row=decision/mcp-stdio-crossing area=established-decisions rec=1
name: Session calls over the MCP role's stdio
rule: A session's calls also cross from an MCP client over the escher-session mcp role's stdio, a child of that client local to the invoking user, holding its session in its own process with no listener.
why: A boundary widening, ratified.
@row_add n=16 row=contract/mcp-stdio/request-id area=established-decisions rec=1
boundary: the mcp role's stdio
item: request id
promise: The request id is echoed, an integer or a string of at most 128 bytes.
@row_add n=17 row=contract/mcp-stdio/request-line area=established-decisions rec=1
boundary: the mcp role's stdio
item: request line
promise: A request line is at most 65,536 bytes.
@row_add n=18 row=contract/mcp-stdio/result area=established-decisions rec=1
boundary: the mcp role's stdio
item: result
promise: A result is at most 1,048,576 bytes.
@row_add n=19 row=decision/fixed-program-text-on-carriers area=established-decisions rec=1
name: Program's fixed text rides the ratified carriers
rule: The command line's stdout and the mcp role's stdio carry the program's own fixed text, static data of the one table (help, a session error's remedy, each tool's description and result fields); text from a page, a call or a screen, a new carrier or a new exit still widens.
why: Ratified; no dependency added.
@row_add n=20 row=decision/differs-closed-set area=established-decisions rec=1
name: A changed node's differs is a closed set
rule: A changed node's differs crosses the carriers as a closed set.
@row_add n=21 row=decision/screenshot-in-band area=established-decisions rec=1
name: Screenshot pixels cross in band
rule: A shot's pixels cross in band, its PNG as base64 in the answer on the session socket and the command line's stdout and as an MCP image block on the mcp stdio, with no new carrier, exit or path argument; the 1,048,576-byte bounds stay, a bigger image ending oversize / AnswerTooLarge.
why: A boundary widening, ratified.
@row_add n=22 row=decision/lint-findings-on-carriers area=established-decisions rec=1
name: Lint findings cross the carriers
rule: A lint's findings cross the ratified carriers and rest in a kept check as the lint verb's answer, holding stable ids, the closed defect words and numbers, never page text, colour, control value or NodeId, with no new carrier, exit, path argument, cause, session error, span field or dependency.
why: A boundary widening, ratified.
@row_add n=23 row=decision/large-screen-in-parts area=established-decisions rec=1
name: Screen over the snapshot budget crosses in parts
rule: A screen over the snapshot budget crosses in parts, a screen within the budget answering whole, with no new verb, carrier, exit, cause, session error, span field or dependency; over MCP a paged answer is its JSON line.
why: A boundary widening, ratified.
@row_add n=24 row=contract/snapshot-verb/part area=established-decisions rec=1
boundary: the snapshot verb
item: part
promise: An optional whole number from 1 to MAX_PART (1000); one part's text comes out with the closed counts part and parts, and a part past the last gives the empty text.
@row_add n=25 row=decision/select-list-crossing area=established-decisions rec=2
name: Select list crosses only as bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff, differs naming bounds, and as pixels in a screenshot, with no new carrier, exit, cause, session error, node key, log field or span.
why: An option's and a group's label leaving as pixels in every shot of an open list is a ratified boundary widening.
@row_add n=26 row=decision/ui-graph-crossing area=established-decisions rec=3
name: UI graph crosses the carriers
rule: The UI graph crosses socket call, command line and MCP as a text block and rests in a kept check as the graph verb's answer: stable ids, owner ids, component, state and handler event names, declared writes, the flag words undeclared and unconfirmed, booleans; never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_add n=27 row=contract/graph-verb/arguments area=established-decisions rec=3
boundary: the graph verb
item: arguments
promise: The graph verb takes no argument; a call given one is refused malformed with nothing run.
@row_add n=28 row=contract/snapshot-node/clickable area=established-decisions rec=3
boundary: the snapshot node
item: clickable
promise: Each snapshot node carries the closed bool clickable.
@row_add n=29 row=decision/actions-answer-crossing area=established-decisions rec=4
name: Actions answer crosses the carriers
rule: The actions answer crosses the session socket's call, the command line and MCP and rests in a kept check as the actions verb's answer, holding stable ids, owner ids, the table's verb words, the closed cause words and booleans, never a value, typed text or NodeId.
why: A boundary widening, ratified; no new carrier, exit, cause, session error, span field or dependency.
@row_add n=30 row=contract/actions-verb/arguments area=established-decisions rec=4
boundary: the actions verb
item: arguments
promise: The actions verb takes no argument; a call given one is refused malformed with nothing run.
@row_add n=31 row=contract/actions-verb/answer area=established-decisions rec=4
boundary: the actions verb
item: answer
promise: The actions verb answers the whole screen at once, unpaged.
@row_add n=32 row=decision/actions-keyed-reading area=established-decisions rec=4
name: Actions answer's keyed reading
rule: The actions answer carries the keyed reading.
state: provisional
@row_add n=33 row=decision/coverage-role-reads-named-checks area=established-decisions rec=5
name: Coverage role reads kept checks its caller names
rule: The command line's coverage role, coverage <app> <check>..., reads kept-check file paths its caller names, each read bounded and parsed as a check; an unreadable, over-bound, non-check or other-app file is refused malformed holding nothing of the path.
why: A boundary widening, ratified; no new socket message, MCP tool, verb, cause, span field or dependency.
@row_add n=34 row=contract/coverage-role/answer area=established-decisions rec=5
boundary: the command line's coverage role
item: answer
promise: One line on stdout of ids, owner ids, event names and two counts.
@row_add n=35 row=decision/recorded-session-kept-check area=established-decisions rec=6
name: Recorded session kept as a replayable check
rule: A session whose launcher names a recording directory keeps its answered calls at rest as one owner-only replayable check, std only with no dependency added, unix only like the socket.
except: The repository's own two scripted stand checks are committed in git, the one exception to owner-only.
@row_add n=36 row=decision/kept-check-masking area=established-decisions rec=6
name: Kept check keeps masking
rule: A kept check records a masked or unresolved type as one fixed stand-in, and keeps each answer as a 64-bit digest and whole, masked as on every carrier, a shot's without its pixels.
@row_add n=37 row=decision/kept-check-never-logged area=established-decisions rec=6
name: Kept check never logged
rule: A kept check's screens, ids, names, values, snapshot text, diffs, findings, graph and actions answers rest at rest only, never logged and never named by a tool call.
@row_add n=38 row=decision/kept-check-replays-through-run area=established-decisions rec=6
name: Kept check replays through the one run path
rule: A kept check replays through the session's one call-running path alone, and a coverage role reading one from a named path bounds the read at the check size limit plus one byte.
@row_none n=1 rec=7 type=5
@row_none n=2 rec=8 type=5
@row_none n=3 rec=9 type=5
@row_none n=4 rec=10 type=5
@row_none n=5 rec=11 type=5
@row_none n=6 rec=12 type=5
@row_none n=7 rec=13 type=5
@row_none n=8 rec=14 type=5
@row_none n=9 rec=15 type=5
@row_none n=10 rec=16 type=5
@row_none n=11 rec=17 type=5
@row_none n=12 rec=18 type=5
@row_none n=13 rec=19 type=5
@row_none n=14 rec=20 type=5
@row_none n=15 rec=21 type=5
@row_none n=16 rec=22 type=5
@row_none n=17 rec=23 type=5
@row_none n=18 rec=24 type=5
@row_none n=19 rec=25 type=5
@row_none n=20 rec=26 type=5
@row_none n=21 rec=27 type=5
@row_none n=22 rec=28 type=5
@row_add n=39 row=decision/keyboard-compat-coverage-exclusion area=existing-scopes rec=29
name: Upstream keyboard_compat out of the coverage leg
rule: The upstream keyboard_compat module of dioxus-native-dom is excluded from the coverage leg by filename.
why: It is upstream's code, kept until escher takes upstream's retirement of it.
@row_none n=23 rec=30 type=5
@row_none n=24 rec=31 type=5
@row_none n=25 rec=33 type=5
@row_none n=26 rec=34 type=5
@row_none n=27 rec=35 type=5
@row_none n=28 rec=36 type=5
