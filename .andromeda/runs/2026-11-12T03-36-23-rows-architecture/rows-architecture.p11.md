@row_add n=1 row=decision/validated-verb-set-widened-by-ratification area=standard-contracts rec=1
name: Validated verb set widened only by ratification
rule: The driver's validated verb set is closed; a verb joins it only as a ratified widening of that set, at the instance, session or tool level.
why: Each verb beyond the first acting verbs joined as a ratified widening of the validated verb set.
@row_add n=2 row=decision/advance-moves-session-clock area=standard-contracts rec=1
name: Advance moves the session's clock
rule: An advance moves the session's clock by exactly the asked milliseconds on every session, moving the engine's animations, the scrollbar fade and the app's own time through a caller's time step together, and reports the clock's move, not what the app gained.
why: Ratified.
@row_add n=3 row=decision/double-click-window area=standard-contracts rec=1
name: Two clicks without an advance read as a double click
rule: Two clicks on one point with no advance of at least 500 ms between them read as a double click, as the click verb's help says.
why: Ratified.
@row_add n=4 row=decision/screenshot-reads-session-clock area=standard-contracts rec=1
name: Screenshots read the session's clock
rule: The overlay scrollbar fade reads the session's clock, so two screenshots of one state with no advance between them read the same pixels.
why: Ratified.
@row_add n=5 row=decision/refusal-causes-closed-at-eight area=standard-contracts rec=1
name: Refusal causes closed at eight
rule: The set of refusal causes is closed at eight, each with a fixed name, meaning and remedy, and a refusal holds nothing a call supplied.
why: Ratified.
@row_add n=6 row=decision/answers-written-by-own-json-writer area=standard-contracts rec=1
name: Answers written by the driver's own JSON writer
rule: Every answer the driver gives has one written form, a line of JSON keyed by the schema's own words and written by the driver's own output-only writer; no serialization crate writes one, and serde_json only reads MCP requests.
why: Ratified as serde_json for reading only.
@row_add n=7 row=decision/no-base64-crate area=standard-contracts rec=1
name: No base64 crate
rule: A screenshot's PNG is base64-encoded by the driver's own padded RFC 4648 encoder; no base64 crate is taken.
@row_add n=8 row=decision/one-self-description-for-cli-and-mcp area=standard-contracts rec=1
name: One self-description for command line and MCP
rule: The command line's help and the MCP server's help tool serve the same bytes, the program's own fixed text read from the schema's tables, reading no environment and printing nothing.
why: Ratified as the one table riding the ratified carriers.
@row_add n=9 row=decision/type-replaces-control-content area=standard-contracts rec=1
name: Type replaces what a control holds
rule: The type verb replaces what the target control holds, selecting all and typing over it, and an empty text clears the control; a target that reads no value is typed into as is.
why: Chosen in place of appending the text after the old value.
@row_add n=10 row=decision/no-node-id-crosses-the-socket area=standard-contracts rec=1
name: Calls and outcomes cross as lines, no NodeId
rule: Across the socket a call travels as a request line and its outcome or refusal comes back as one line of JSON, and no engine NodeId is in either.
why: Ratified.
@row_add n=11 row=decision/screenshot-png-in-band area=standard-contracts rec=1
name: Screenshot image carried in band
rule: A screenshot's PNG crosses inside the answer line as base64, and on MCP as an image block in the tool result.
why: Ratified as the chosen fork.
@row_add n=12 row=decision/tool-call-span-tool-and-ending area=standard-contracts rec=1
name: MCP tool call span with tool and ending
rule: Each MCP tool call leaves one span carrying only the tool's table word and its ending, the command span of a session run nested under it.
why: Ratified.
@row_add n=13 row=decision/mcp-current-revision-not-taken area=standard-contracts rec=1
name: Current MCP revision not taken up
rule: The MCP server does not take up the current MCP revision; a handshake asking it is answered with the latest of the four revisions the server speaks.
why: Deferred by decision, left among the remainders.
@row_add n=14 row=decision/seen-ids-record-bounded area=standard-contracts rec=1
name: Session's record of seen ids bounded
rule: Between calls a session keeps of its screens only id text, at most 4096 ids, never an id longer than a call can name, forgetting the one read longest ago past the bound; the record has no public reader and is never printed or logged.
why: Ratified for ids longer than a call can name.
guard: the_record_holds_no_id_longer_than_a_call_can_name
@row_add n=15 row=decision/refused-call-records-no-ids area=standard-contracts rec=1
name: A call validation refuses records no ids
rule: A call the schema's validation refuses records nothing in the session's record of seen ids; every snapshot a validated call takes is recorded, on a refusal too.
why: Ratified.
@row_add n=16 row=decision/measured-limits-stated-in-the-tool area=standard-contracts rec=1
name: Measured limits stated where an agent reads the tool
rule: A measured limit of the driver is stated where an agent reads the tool, in the fixed help texts of the verb table, the cause texts and the node keys' help.
why: Ratified.
@row_add n=17 row=decision/advanced-ms-is-the-clock-move area=standard-contracts rec=1
name: advanced_ms reports the clock's move
rule: An advance's advanced_ms is exactly the milliseconds asked, the clock's move, and its diff can be non-empty on any animated page.
why: Ratified.
@row_add n=18 row=contract/session-errors/already-running area=standard-contracts rec=1
boundary: session errors
item: already-running
promise: A host already answers on the state directory's socket; start spawns nothing and an MCP start while a session is held reads it.
@row_add n=19 row=contract/session-errors/no-session area=standard-contracts rec=1
boundary: session errors
item: no-session
promise: No socket file is in the state directory, or no session is held for an instance verb.
@row_add n=20 row=contract/session-errors/dead area=standard-contracts rec=1
boundary: session errors
item: dead
promise: A socket file is present that nothing answers on; its remedy repeats no-session's, naming it.
@row_add n=21 row=contract/session-errors/host-exited area=standard-contracts rec=1
boundary: session errors
item: host-exited
promise: The spawned host exited before the session answered, carrying its exit code where there is one.
@row_add n=22 row=contract/session-errors/timeout area=standard-contracts rec=1
boundary: session errors
item: timeout
promise: A bound passed: stop waited past 5 s, start past its ready time with the host killed and reaped, or an answer read past 30 s.
@row_add n=23 row=contract/session-errors/protocol area=standard-contracts rec=1
boundary: session errors
item: protocol
promise: The host answered outside the wire protocol.
@row_add n=24 row=contract/session-errors/invalid-label area=standard-contracts rec=1
boundary: session errors
item: invalid-label
promise: A session label is not 1 to 32 bytes of a-z, 0-9 and -; the boot is not run.
@row_add n=25 row=contract/session-errors/state-dir-too-long area=standard-contracts rec=1
boundary: session errors
item: state-dir-too-long
promise: The state directory's socket path is too long; the message states the bound, at most 94 bytes on Linux and 90 elsewhere, and the remedy names a shorter or a relative path.
@row_add n=26 row=contract/session-errors/state-dir-not-private area=standard-contracts rec=1
boundary: session errors
item: state-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy.
@row_add n=27 row=contract/session-errors/unsupported area=standard-contracts rec=1
boundary: session errors
item: unsupported
promise: On a non-unix target serve, start, attach, stop, call of an admitted call and recording into a directory return it; a session held in process without a recording works everywhere.
@row_add n=28 row=contract/session-errors/io area=standard-contracts rec=1
boundary: session errors
item: io
promise: An I/O failure, its fixed message appending the I/O error kind's name.
@row_add n=29 row=contract/session-errors/not-settled area=standard-contracts rec=1
boundary: session errors
item: not-settled
promise: An in-process act whose instance did not go quiet, the message appending the busy class; the step is not rolled back.
@row_add n=30 row=contract/session-errors/answer-too-large area=standard-contracts rec=1
boundary: session errors
item: answer-too-large
promise: The host replied oversize: the call ran and what it did stands; its remedy repeats not-settled's, naming it.
@row_add n=31 row=contract/session-errors/record-dir-not-private area=standard-contracts rec=1
boundary: session errors
item: record-dir-not-private
promise: One of the fifteen fixed session error kinds, with a fixed message and remedy; a start or mcp given --record checks the recording directory first.
@row_add n=32 row=contract/session-errors/recording-failed area=standard-contracts rec=1
boundary: session errors
item: recording-failed
promise: The host's recording ended after the call; the wire's recording-failed reply reads as it.
@row_add n=33 row=contract/session-error-json/shape area=standard-contracts rec=1
boundary: session error JSON
item: shape
promise: A session error is written {"error":{"kind","message","remedy"}}; every message is a fixed string carrying no path, label, id or value.
@row_add n=34 row=contract/cli-exit-statuses/0 area=standard-contracts rec=1
boundary: CLI exit statuses
item: 0
promise: accepted: the answer on stdout; also the host role's quiet end after a stop or an expiry, and the mcp role's Ok end.
@row_add n=35 row=contract/cli-exit-statuses/1 area=standard-contracts rec=1
boundary: CLI exit statuses
item: 1
promise: refused: the refusal's JSON on stdout.
@row_add n=36 row=contract/cli-exit-statuses/2 area=standard-contracts rec=1
boundary: CLI exit statuses
item: 2
promise: usage: one fixed line on stderr built from the verbs and the app names, stdout empty; includes a --session absent where needed or given twice.
@row_add n=37 row=contract/cli-exit-statuses/3 area=standard-contracts rec=1
boundary: CLI exit statuses
item: 3
promise: session-error: the error object on stdout and its fixed message on stderr.
@row_add n=38 row=contract/refusal-causes/unknown-verb area=standard-contracts rec=1
boundary: refusal causes
item: unknown-verb
promise: The verb is not in the table by exact name, or a session- or tool-level verb is handed where an instance verb is run; returned by validation.
@row_add n=39 row=contract/refusal-causes/malformed area=standard-contracts rec=1
boundary: refusal causes
item: malformed
promise: An argument is unnamed, repeated, missing, of another kind or out of bound, carried with its fault; returned by validation.
@row_add n=40 row=contract/refusal-causes/not-found area=standard-contracts rec=1
boundary: refusal causes
item: not-found
promise: The id names no element on the screen as it reads now and the session's record does not hold it; nothing runs.
@row_add n=41 row=contract/refusal-causes/stale area=standard-contracts rec=1
boundary: refusal causes
item: stale
promise: The id names no element on the screen now but the session's record of read ids holds it; nothing runs.
@row_add n=42 row=contract/refusal-causes/disabled area=standard-contracts rec=1
boundary: refusal causes
item: disabled
promise: A click or type target reads enabled false; checked first of the three screen-level causes.
@row_add n=43 row=contract/refusal-causes/covered area=standard-contracts rec=1
boundary: refusal causes
item: covered
promise: Another element is hit at the point the action would land; its meaning states the closed-select option, one-axis clip and zero-size box readings.
@row_add n=44 row=contract/refusal-causes/off-screen area=standard-contracts rec=1
boundary: refusal causes
item: off-screen
promise: The element lies outside the viewport or a scrolling box's visible part, or a screenshot crop holds no pixel; remedy: bring it into view with scroll, then act again.
@row_add n=45 row=contract/refusal-causes/fonts-unavailable area=standard-contracts rec=1
boundary: refusal causes
item: fonts-unavailable
promise: The app has no font it can paint text with; screenshot and lint refuse with nothing painted or read; remedy: boot the app with a font it can decode.
@row_add n=46 row=contract/refusal-json/shape area=standard-contracts rec=1
boundary: refusal JSON
item: shape
promise: A refusal is written {"refused":{"cause","meaning","remedy"[,"fault"]}} from the cause's fixed strings, holding nothing the call supplied.
@row_add n=47 row=contract/refusal-faults/missing area=standard-contracts rec=1
boundary: refusal faults
item: Missing
promise: A required argument is absent; beside malformed only.
@row_add n=48 row=contract/refusal-faults/unnamed area=standard-contracts rec=1
boundary: refusal faults
item: Unnamed
promise: An argument names nothing the verb takes; beside malformed only.
@row_add n=49 row=contract/refusal-faults/repeated area=standard-contracts rec=1
boundary: refusal faults
item: Repeated
promise: An argument name was passed before in the same call; beside malformed only.
@row_add n=50 row=contract/refusal-faults/wrong-kind area=standard-contracts rec=1
boundary: refusal faults
item: WrongKind
promise: An argument's value is of another kind than the schema's; beside malformed only.
@row_add n=51 row=contract/refusal-faults/out-of-bound area=standard-contracts rec=1
boundary: refusal faults
item: OutOfBound
promise: An argument's value is outside its bound, or a text is no key name; beside malformed only.
@row_add n=52 row=contract/validation-order/first-failure area=standard-contracts rec=1
boundary: call validation
item: first failure
promise: One answer a call, the first failure in order: the verb by exact name, then the call's arguments as passed, then the verb's arguments in the schema's order; a refused call changes nothing.
@row_add n=53 row=contract/driver-verbs/snapshot area=standard-contracts rec=1
boundary: driver verbs
item: snapshot
promise: One optional part (1 to 1000); reads the screen once, dispatches nothing; result text, plus part and parts when read in parts; a part past the last is the empty text.
@row_add n=54 row=contract/driver-verbs/click area=standard-contracts rec=1
boundary: driver verbs
item: click
promise: Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a select opens its list.
@row_add n=55 row=contract/driver-verbs/type area=standard-contracts rec=1
boundary: driver verbs
item: type
promise: Takes id and text; on a select chooses the first option whose label starts with the text, on a multiple select its match alone, a text opening on a space typing nothing.
@row_add n=56 row=contract/driver-verbs/press area=standard-contracts rec=1
boundary: driver verbs
item: press
promise: Takes key and optional shift, false when absent; presses on what holds focus, carrying no Ctrl, so non-adjacent rows of a multiple select cannot be built.
@row_add n=57 row=contract/driver-verbs/advance area=standard-contracts rec=1
boundary: driver verbs
item: advance
promise: Takes ms, 1 to 60000; one settled step; adds advanced_ms, exactly ms.
@row_add n=58 row=contract/driver-verbs/scroll area=standard-contracts rec=1
boundary: driver verbs
item: scroll
promise: Takes id; accepted on a disabled or covered element, scrolls it into view as one settled step and adds in_view, always present, false when no scroll could bring it in view or it was removed.
@row_add n=59 row=contract/driver-verbs/screenshot area=standard-contracts rec=1
boundary: driver verbs
item: screenshot
promise: One optional id; result width, height and png (base64), always present; an element shot is cropped to its visible bounds; no step, nothing changes.
@row_add n=60 row=contract/driver-verbs/lint area=standard-contracts rec=1
boundary: driver verbs
item: lint
promise: No argument; result findings, always present; geometry only, no hit, right-to-left and vertical text read as left-to-right lines; dispatches nothing.
@row_add n=61 row=contract/driver-verbs/graph area=standard-contracts rec=1
boundary: driver verbs
item: graph
promise: No argument; result nodes and states, the screen's UI graph read whole; dispatches nothing, runs no step.
@row_add n=62 row=contract/driver-verbs/actions area=standard-contracts rec=1
boundary: driver verbs
item: actions
promise: No argument; result nodes, what each actionable node can take now; dispatches nothing, runs no step.
@row_add n=63 row=contract/driver-verbs/start area=standard-contracts rec=1
boundary: driver verbs
item: start
promise: Session level; takes app; result label, pid and idle_expiry_s.
@row_add n=64 row=contract/driver-verbs/status area=standard-contracts rec=1
boundary: driver verbs
item: status
promise: Session level; no argument; result label, pid, idle_expiry_s and served.
@row_add n=65 row=contract/driver-verbs/stop area=standard-contracts rec=1
boundary: driver verbs
item: stop
promise: Session level; no argument; result stopped, written {"stopped":true}.
@row_add n=66 row=contract/driver-verbs/help area=standard-contracts rec=1
boundary: driver verbs
item: help
promise: Tool level; no argument; result verbs, node_keys, key_names, busy_classes, causes, exit_statuses and session_errors, each with its help; reaches no session.
@row_add n=67 row=contract/acting-verb-results/settled area=standard-contracts rec=1
boundary: acting verb results
item: settled
promise: A flag; a step that did not go quiet is a result with settled false and its busy class, never a refusal or a session error, and is not rolled back.
@row_add n=68 row=contract/acting-verb-results/busy area=standard-contracts rec=1
boundary: acting verb results
item: busy
promise: One of the busy classes, present only when settled is false.
@row_add n=69 row=contract/acting-verb-results/added area=standard-contracts rec=1
boundary: acting verb results
item: added
promise: Nodes on the screen after the step that were not before, written without differs.
@row_add n=70 row=contract/acting-verb-results/changed area=standard-contracts rec=1
boundary: acting verb results
item: changed
promise: Elements before and after the step whose role, name, state, bounds or parent differ, as they read after it, each carrying differs.
@row_add n=71 row=contract/acting-verb-results/removed area=standard-contracts rec=1
boundary: acting verb results
item: removed
promise: The ids of the elements the step removed.
@row_add n=72 row=contract/busy-classes/render area=standard-contracts rec=1
boundary: busy classes
item: render
promise: A busy class word of an instance that did not go quiet.
@row_add n=73 row=contract/busy-classes/layout area=standard-contracts rec=1
boundary: busy classes
item: layout
promise: A busy class word of an instance that did not go quiet.
@row_add n=74 row=contract/busy-classes/loads area=standard-contracts rec=1
boundary: busy classes
item: loads
promise: A busy class word of an instance that did not go quiet; a load in flight is waited for at most the load wait.
@row_add n=75 row=contract/node-keys/id area=standard-contracts rec=1
boundary: node keys
item: id
promise: The element's stable id, always written.
@row_add n=76 row=contract/node-keys/parent area=standard-contracts rec=1
boundary: node keys
item: parent
promise: The parent element's id, left out when absent.
@row_add n=77 row=contract/node-keys/role area=standard-contracts rec=1
boundary: node keys
item: role
promise: The role as the accessibility tree reads it, always written.
@row_add n=78 row=contract/node-keys/name area=standard-contracts rec=1
boundary: node keys
item: name
promise: The accessible name as the accessibility tree reads it, always written.
@row_add n=79 row=contract/node-keys/enabled area=standard-contracts rec=1
boundary: node keys
item: enabled
promise: Follows the disabled attribute's presence whatever its value; left out when absent.
@row_add n=80 row=contract/node-keys/checked area=standard-contracts rec=1
boundary: node keys
item: checked
promise: A native checkbox's or radio's own state, else the aria-checked toggle of the five checkable roles, a mixed one reading false; left out when absent.
@row_add n=81 row=contract/node-keys/value area=standard-contracts rec=1
boundary: node keys
item: value
promise: The control's value, a fixed mask for a password or file input; left out when absent.
@row_add n=82 row=contract/node-keys/selected area=standard-contracts rec=1
boundary: node keys
item: selected
promise: Read from aria-selected, written only where the element states it.
@row_add n=83 row=contract/node-keys/pressed area=standard-contracts rec=1
boundary: node keys
item: pressed
promise: Read from aria-pressed, left out for the five checkable roles and where unstated; a mixed toggle reads false.
@row_add n=84 row=contract/node-keys/invalid area=standard-contracts rec=1
boundary: node keys
item: invalid
promise: Read from aria-invalid, a boolean on every node; a grammar or spelling error reads true.
@row_add n=85 row=contract/node-keys/focused area=standard-contracts rec=1
boundary: node keys
item: focused
promise: Whether the element holds focus, always written.
@row_add n=86 row=contract/node-keys/clickable area=standard-contracts rec=1
boundary: node keys
item: clickable
promise: True where the element's current listeners hold click and it is no disabled form control; a boolean on every node.
@row_add n=87 row=contract/node-keys/bounds area=standard-contracts rec=1
boundary: node keys
item: bounds
promise: An array of four numbers, where the element stands, the same however far a box's content is scrolled; a non-finite number is null.
@row_add n=88 row=contract/node-keys/differs area=standard-contracts rec=1
boundary: node keys
item: differs
promise: On a changed node only, after bounds: the keys whose reading differs from before the step, in key order; a node that only moved reads bounds alone.
@row_add n=89 row=contract/argument-kinds/id area=standard-contracts rec=1
boundary: argument kinds
item: id
promise: 1 to 1024 bytes of text.
@row_add n=90 row=contract/argument-kinds/text area=standard-contracts rec=1
boundary: argument kinds
item: text
promise: 0 to 4096 bytes of text.
@row_add n=91 row=contract/argument-kinds/key area=standard-contracts rec=1
boundary: argument kinds
item: key
promise: One of the twelve key names, else out of bound.
@row_add n=92 row=contract/argument-kinds/flag area=standard-contracts rec=1
boundary: argument kinds
item: flag
promise: A boolean.
@row_add n=93 row=contract/argument-kinds/milliseconds area=standard-contracts rec=1
boundary: argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_add n=94 row=contract/argument-kinds/name area=standard-contracts rec=1
boundary: argument kinds
item: name
promise: 1 to 32 bytes of a-z, 0-9 and -, the app a start names.
@row_add n=95 row=contract/argument-kinds/part area=standard-contracts rec=1
boundary: argument kinds
item: part
promise: A whole number from 1 to 1000, the snapshot part asked for.
@row_add n=96 row=contract/key-names/tab area=standard-contracts rec=1
boundary: key names
item: tab
promise: A key name press accepts.
@row_add n=97 row=contract/key-names/enter area=standard-contracts rec=1
boundary: key names
item: enter
promise: A key name press accepts.
@row_add n=98 row=contract/key-names/escape area=standard-contracts rec=1
boundary: key names
item: escape
promise: A key name press accepts.
@row_add n=99 row=contract/key-names/backspace area=standard-contracts rec=1
boundary: key names
item: backspace
promise: A key name press accepts; on macOS it also dispatches deleteBackward.
@row_add n=100 row=contract/key-names/delete area=standard-contracts rec=1
boundary: key names
item: delete
promise: A key name press accepts.
@row_add n=101 row=contract/key-names/space area=standard-contracts rec=1
boundary: key names
item: space
promise: A key name press accepts.
@row_add n=102 row=contract/key-names/arrow-up area=standard-contracts rec=1
boundary: key names
item: arrow-up
promise: A key name press accepts.
@row_add n=103 row=contract/key-names/arrow-down area=standard-contracts rec=1
boundary: key names
item: arrow-down
promise: A key name press accepts.
@row_add n=104 row=contract/key-names/arrow-left area=standard-contracts rec=1
boundary: key names
item: arrow-left
promise: A key name press accepts.
@row_add n=105 row=contract/key-names/arrow-right area=standard-contracts rec=1
boundary: key names
item: arrow-right
promise: A key name press accepts.
@row_add n=106 row=contract/key-names/home area=standard-contracts rec=1
boundary: key names
item: home
promise: A key name press accepts.
@row_add n=107 row=contract/key-names/end area=standard-contracts rec=1
boundary: key names
item: end
promise: A key name press accepts.
@row_add n=108 row=contract/session-wire/version area=standard-contracts rec=1
boundary: session wire
item: version
promise: The wire is versioned v3: one request line and one reply line per connection, each newline-terminated.
@row_add n=109 row=contract/session-wire/hello-request area=standard-contracts rec=1
boundary: session wire
item: hello v3
promise: Answered ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}, served counting every request answered before it, refusals included.
@row_add n=110 row=contract/session-wire/stop-request area=standard-contracts rec=1
boundary: session wire
item: stop v3
promise: Answered ok v3 stopping; the stop takes effect once its reply is written.
@row_add n=111 row=contract/session-wire/call-request area=standard-contracts rec=1
boundary: session wire
item: call v3
promise: call v3 <verb> [<name>=<kind>:<value>]…, kinds t text, n whole number, f flag; verb, names and text percent-escaped byte for byte, decoded strictly; at most 16,384 bytes.
@row_add n=112 row=contract/session-wire/refused-version area=standard-contracts rec=1
boundary: session wire
item: refused version
promise: A known request word with another version; changes nothing.
@row_add n=113 row=contract/session-wire/refused-malformed area=standard-contracts rec=1
boundary: session wire
item: refused malformed
promise: Any other request, a call the decoder does not read included; changes nothing.
@row_add n=114 row=contract/session-wire/reply-bound area=standard-contracts rec=1
boundary: session wire
item: reply bound
promise: A hello, stop or refused reply is at most 128 bytes.
@row_add n=115 row=contract/session-wire/ok-accepted area=standard-contracts rec=1
boundary: session wire
item: ok v3 accepted
promise: A call's accepted answer, its JSON line after the words, the JSON bounded at 1,048,576 bytes.
@row_add n=116 row=contract/session-wire/ok-refused area=standard-contracts rec=1
boundary: session wire
item: ok v3 refused
promise: A call's refusal, its JSON line after the words.
@row_add n=117 row=contract/session-wire/ok-oversize area=standard-contracts rec=1
boundary: session wire
item: ok v3 oversize
promise: The answer exceeded its bound; no refusal: the call ran and what it did stands; the client reads it as answer-too-large.
@row_add n=118 row=contract/session-wire/ok-recording-failed area=standard-contracts rec=1
boundary: session wire
item: ok v3 recording-failed
promise: Sent in place of the answer when the host's recording ended after the call; the client reads it as recording-failed.
@row_add n=119 row=contract/session-host/connections area=standard-contracts rec=1
boundary: session host
item: connections
promise: One session per process, one connection at a time with a 2 s read and write bound each; only call requests reach the session.
@row_add n=120 row=contract/session-host/idle-expiry area=standard-contracts rec=1
boundary: session host
item: idle expiry
promise: Once no request is answered for the idle expiry (1,800 s for a command line's host) the session is dropped and the socket and directory removed; every answered request restarts the count, a broken one does not.
@row_add n=121 row=contract/session-host/stale-socket area=standard-contracts rec=1
boundary: session host
item: stale socket
promise: A socket file no process answers on is replaced; one a process answers on is already-running.
@row_add n=122 row=contract/session-client/call-answer-read-bound area=standard-contracts rec=1
boundary: session client
item: call answer read bound
promise: A call is validated before sending and fits the request bound; its answer is read under a 30 s bound on each read, timeout past it.
@row_add n=123 row=contract/session-client/start area=standard-contracts rec=1
boundary: session client
item: start
promise: Attaches first and spawns nothing when answered; else spawns the host, polls every 25 ms until it answers, exits or the ready time passes; it does not check the answering pid is its child.
@row_add n=124 row=contract/answer-json/outcome area=standard-contracts rec=1
boundary: answer JSON
item: outcome
promise: An object keyed by its verb's result fields in the table's order, busy present only when not settled.
@row_add n=125 row=contract/answer-json/node area=standard-contracts rec=1
boundary: answer JSON
item: node
promise: An object keyed by the node keys in order, parent, enabled, checked, value, selected and pressed left out when absent; id, role, name, invalid, focused, clickable and bounds always written.
@row_add n=126 row=contract/answer-json/screenshot area=standard-contracts rec=1
boundary: answer JSON
item: screenshot
promise: {"width":…,"height":…,"png":"<base64>"}, the PNG base64 padded per RFC 4648 §4.
@row_add n=127 row=contract/answer-json/lint-finding area=standard-contracts rec=1
boundary: answer JSON
item: lint finding
promise: {"findings":[…]}, each keyed id, defect, bounds, then its reason's keys; stable ids, closed words and numbers only.
@row_add n=128 row=contract/answer-json/actions area=standard-contracts rec=1
boundary: answer JSON
item: actions
promise: {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent; never a value, typed text or NodeId.
@row_add n=129 row=contract/answer-json/paged-snapshot area=standard-contracts rec=1
boundary: answer JSON
item: paged snapshot
promise: {"text":…,"part":n,"parts":m} for a part read; a whole screen's answer is {"text":…} alone.
@row_add n=130 row=contract/answer-json/string-escaping area=standard-contracts rec=1
boundary: answer JSON
item: string escaping
promise: A string escapes ", \ and U+0000 to U+001F.
@row_add n=131 row=contract/lint-reason-keys/overlap area=standard-contracts rec=1
boundary: lint reason keys
item: overlap
promise: other and area.
@row_add n=132 row=contract/lint-reason-keys/clip area=standard-contracts rec=1
boundary: lint reason keys
item: clip
promise: visible.
@row_add n=133 row=contract/lint-reason-keys/truncation area=standard-contracts rec=1
boundary: lint reason keys
item: truncation
promise: text then visible, visible left out when no part can be seen.
@row_add n=134 row=contract/lint-reason-keys/contrast area=standard-contracts rec=1
boundary: lint reason keys
item: contrast
promise: ratio, its shortest decimal or null when not finite, and required.
@row_add n=135 row=contract/lint-reason-keys/off-screen area=standard-contracts rec=1
boundary: lint reason keys
item: off-screen
promise: No reason keys.
@row_add n=136 row=contract/lint-reason-keys/zero-size area=standard-contracts rec=1
boundary: lint reason keys
item: zero-size
promise: No reason keys.
@row_add n=137 row=contract/command-line/serve area=standard-contracts rec=1
boundary: command line
item: serve
promise: serve <app> --session <dir> [--record <dir>] in exactly that shape is the host role, ending quiet with status 0 after a stop or an expiry; any other shape is usage.
@row_add n=138 row=contract/command-line/mcp area=standard-contracts rec=1
boundary: command line
item: mcp
promise: mcp [--record <dir>] serves MCP on stdin and stdout, writing nothing on stdout at its end: 0 on Ok, 3 with the error's fixed message on stderr.
@row_add n=139 row=contract/command-line/help area=standard-contracts rec=1
boundary: command line
item: help
promise: help with no argument and at most one unread --session answers the self-description on stdout, status 0, reaching no session; an argument is refused malformed.
@row_add n=140 row=contract/command-line/start area=standard-contracts rec=1
boundary: command line
item: start
promise: start <app> [--record <dir>] spawns the running binary as the host with standard streams closed, waits up to 30 s for it to answer and neither kills nor reaps it; an app the binary does not boot is usage.
@row_add n=141 row=contract/command-line/verb-flags area=standard-contracts rec=1
boundary: command line
item: verb flags
promise: --id, --text, --key, --ms and --part each take one value, --shift is bare, and --session <dir> exactly once; the call is validated before the session is asked for.
@row_add n=142 row=contract/command-line/closed-pipe area=standard-contracts rec=1
boundary: command line
item: closed pipe
promise: A write to a closed pipe is dropped, not a panic, and the ending keeps its status; no ending writes back anything the caller typed.
@row_add n=143 row=contract/mcp-server/protocol-revisions area=standard-contracts rec=1
boundary: MCP server
item: protocol revisions
promise: Newline-delimited JSON-RPC 2.0 in the four handshake revisions of PROTOCOL_VERSIONS; initialize answers the asked revision when it is one of the four, else LATEST_VERSION, the newest of them.
@row_add n=144 row=contract/mcp-server/initialize area=standard-contracts rec=1
boundary: MCP server
item: initialize
promise: Answers capabilities {"tools":{}} and serverInfo with name escher and the driver's package version.
@row_add n=145 row=contract/mcp-server/ping area=standard-contracts rec=1
boundary: MCP server
item: ping
promise: Answers {}.
@row_add n=146 row=contract/mcp-server/tools-list area=standard-contracts rec=1
boundary: MCP server
item: tools/list
promise: The fourteen verbs in table order, help last, each with its help and its result fields as description and an inputSchema with additionalProperties false.
@row_add n=147 row=contract/mcp-server/tools-call area=standard-contracts rec=1
boundary: MCP server
item: tools/call
promise: Runs one tool; a string argument is text, an i64 a number, a bool a flag, any other value refused malformed; start, status, stop and help are answered by the server.
@row_add n=148 row=contract/mcp-server/other-methods area=standard-contracts rec=1
boundary: MCP server
item: other methods
promise: Every other method, server/discover included, answers -32601; a request with no id is a notification, answered by nothing.
@row_add n=149 row=contract/mcp-server/request-line-bound area=standard-contracts rec=1
boundary: MCP server
item: request line bound
promise: A request line of at most 65,536 bytes; a longer one is read to its terminator, dropped and answered -32700.
@row_add n=150 row=contract/mcp-server/id-echo area=standard-contracts rec=1
boundary: MCP server
item: id echo
promise: An id is echoed only as an i64 or a string of at most 128 bytes, else -32600 with id null.
@row_add n=151 row=contract/mcp-server/result-bound area=standard-contracts rec=1
boundary: MCP server
item: result bound
promise: A text-only result's text is at most 1,048,576 bytes, a result with an image by its whole written form; past it answer-too-large's result.
@row_add n=152 row=contract/mcp-server/protocol-error area=standard-contracts rec=1
boundary: MCP server
item: protocol error
promise: A fixed code and a fixed message with no data.
@row_add n=153 row=contract/mcp-server/tool-result area=standard-contracts rec=1
boundary: MCP server
item: tool result
promise: {content:[…], isError}, no structuredContent; a refusal or session error one text block with isError true; a screenshot an image block then its line without png.
@row_add n=154 row=contract/mcp-server/idle-expiry area=standard-contracts rec=1
boundary: MCP server
item: idle expiry
promise: Checked when a request arrives with a session held; no timer, no thread.
@row_add n=155 row=contract/command-span/verb area=standard-contracts rec=1
boundary: command span
item: verb
promise: Target escher_driver, span command at INFO: the verb table's word, recorded for a known verb, never the caller's text.
@row_add n=156 row=contract/command-span/cause area=standard-contracts rec=1
boundary: command span
item: cause
promise: The refusal's cause name, recorded for every refused call.
@row_add n=157 row=contract/command-span/settled area=standard-contracts rec=1
boundary: command span
item: settled
promise: Recorded when the outcome carries one.
@row_add n=158 row=contract/command-span/busy area=standard-contracts rec=1
boundary: command span
item: busy
promise: The busy class word, recorded when the outcome carries one.
@row_add n=159 row=contract/command-span/passes area=standard-contracts rec=1
boundary: command span
item: passes
promise: Settle's pass count, recorded when settle answered quiet.
@row_add n=160 row=contract/command-span/added area=standard-contracts rec=1
boundary: command span
item: added
promise: The length of the diff's added list.
@row_add n=161 row=contract/command-span/removed area=standard-contracts rec=1
boundary: command span
item: removed
promise: The length of the diff's removed list.
@row_add n=162 row=contract/command-span/changed area=standard-contracts rec=1
boundary: command span
item: changed
promise: The length of the diff's changed list.
@row_add n=163 row=contract/command-span/never-fielded area=standard-contracts rec=1
boundary: command span
item: never fielded
promise: No screen text, image, findings, graph, actions answer, label, id record, argument, call, outcome or refusal value is ever a field.
@row_add n=164 row=contract/tool-call-span/tool area=standard-contracts rec=1
boundary: tool_call span
item: tool
promise: Target escher_driver; the table's word for the tool, absent for an unknown tool.
@row_add n=165 row=contract/tool-call-span/ending area=standard-contracts rec=1
boundary: tool_call span
item: ending
promise: accepted, refused or error (a session or protocol error).
@row_add n=166 row=absence/settle-verb-on-socket area=standard-contracts rec=1
capability: A settle verb or a busy-source reply of its own on the session socket
