@row_add n=1 row=decision/cold-agent-kept-checks-stay-local area=ci-integration rec=1,16
name: Cold-agent kept checks stay local
rule: The cold-agent pipe's live run keeps its checks in an owner-only (mode 700) recording directory; the kept checks are never printed, copied, uploaded by a CI leg or committed, and are removed with the rest of the state at cleanup; the replay children run without recording.

@row_add n=2 row=decision/committed-checks-hold-no-private-data area=ci-integration rec=2
name: Committed checks hold no private data
rule: A kept check committed to git is one of the repository's own scripted stand-Counter sessions and holds no typed text, host path, name, secret or screenshot step.
why: The repository is public.

@row_add n=3 row=absence/error-reporting-service area=error-capture-reporting rec=3,4,5,6,7,8
capability: An error-reporting or error-capture service (such as Sentry)

@row_none n=1 rec=9 type=4
relied: A failed, empty or non-CSS stylesheet is answered as an error, so it leaves the pending critical resources and stops blocking render.

@row_none n=2 rec=10 type=4
relied: A net handler that does not override the error method drops the error, the trait's default doing nothing.

@row_add n=4 row=contract/telemetry-scrub-set/url area=error-capture-reporting rec=11
boundary: escher-telemetry's scrub set
item: url
promise: A `url` field is in the scrub set, so the sink prints its value as `[redacted]`.

@row_add n=5 row=contract/telemetry-scrub-set/error area=error-capture-reporting rec=11
boundary: escher-telemetry's scrub set
item: error
promise: An `error` field is in the scrub set, so the sink prints its value as `[redacted]`; a failed fetch's error text reaches no carrier beyond its log line.

@row_add n=6 row=decision/closed-span-lines area=log-coverage rec=12
name: Closed spans as a second record class
rule: The sink writes one line for each closed span whose target the allowlist admits, and no line when a span is created, recorded to, entered or exited.
why: Ratified as a boundary widening of the log record classes.

@row_add n=7 row=contract/span-line/field-order area=log-coverage rec=12
boundary: The closed span's line
item: field order
promise: `{time} {LEVEL} {target} service.name=… service.version=… span={the span's name}`, then the span's own fields in the order recorded, then `message`, `time.busy` and `time.idle`.

@row_add n=8 row=contract/span-line/level-and-target area=log-coverage rec=12
boundary: The closed span's line
item: level and target
promise: The line's level and target are the span's own.

@row_add n=9 row=contract/span-line/time-busy area=log-coverage rec=12
boundary: The closed span's line
item: time.busy
promise: The busy timing is tracing-subscriber's own; no escher code reads a clock for it.

@row_add n=10 row=contract/span-line/time-idle area=log-coverage rec=12
boundary: The closed span's line
item: time.idle
promise: The idle timing is tracing-subscriber's own; no escher code reads a clock for it.

@row_add n=11 row=contract/span-line/span area=log-coverage rec=12
boundary: The closed span's line
item: span
promise: `span` is a reserved field name holding the span's name.

@row_add n=12 row=contract/span-line/scrub area=log-coverage rec=12
boundary: The closed span's line
item: scrub
promise: Every pair of the line, `span` included, passes the scrub under the span's target.

@row_add n=13 row=contract/span-line/value-quoting area=log-coverage rec=12
boundary: The closed span's line
item: value quoting
promise: A string value is Debug-quoted (`span="command"`, `message="close"`); a number or a bool is written bare.

@row_add n=14 row=contract/span-line/target-outside-allowlist area=log-coverage rec=12
boundary: The closed span's line
item: target outside the allowlist
promise: A span from a target outside the allowlist writes no byte at any point.

@row_add n=15 row=contract/event-line/inside-a-span area=log-coverage rec=12
boundary: The event line
item: event inside a span
promise: An event emitted inside a span keeps its line unchanged, gaining no span name and no span field.

@row_add n=16 row=decision/cold-agent-verdict-holds-no-content area=log-coverage rec=13
name: Cold-agent verdict holds no content
rule: Every field of the cold-agent verdict is a count, a boolean or a closed word, holding no transcript text, tool argument, tool answer, reply or check.

@row_add n=17 row=contract/cold-agent-verdict/counts-agree area=log-coverage rec=13
boundary: The cold-agent verdict
item: counts_agree
promise: Ties the server's span lines to the transcript: the `tool_call` lines equal the escher tool uses, and its refused and error lines equal the escher results marked `is_error`.

@row_add n=18 row=contract/cold-agent-verdict/wrong-calls area=log-coverage rec=13
boundary: The cold-agent verdict
item: wrong_calls
promise: The sum of refused, error and foreign tool uses and permission denials, deciding the verdict against `wrong_call_bar_pct`.

@row_add n=19 row=contract/cold-agent-verdict/wrong-call-bar-pct area=log-coverage rec=13
boundary: The cold-agent verdict
item: wrong_call_bar_pct
promise: The bar on wrong calls, 5 percent of the tool calls.

@row_add n=20 row=contract/cold-agent-verdict/reasons-order area=log-coverage rec=13
boundary: The cold-agent verdict
item: reasons order
promise: Reasons appear in the order `task-unmet`, `capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`, `no-check`, `check-failed`, `check-cannot-fail`.

@row_add n=21 row=contract/cold-agent-verdict-reasons/capability-unused area=log-coverage rec=13
boundary: The cold-agent verdict's reasons
item: capability-unused
promise: Given when a counted verb or a part read went unused.

@row_add n=22 row=contract/cold-agent-verdict-reasons/wrong-calls-over-bar area=log-coverage rec=13
boundary: The cold-agent verdict's reasons
item: wrong-calls-over-bar
promise: Given when `wrong_calls × 100 > 5 × tool_calls`.

@row_add n=23 row=decision/graph-and-actions-not-counted-verbs area=log-coverage rec=14
name: Graph and actions not counted for a cold agent
rule: The `graph` and `actions` verbs are not counted verbs and are not required of a cold agent, so neither adds a verdict field or a `capability-unused` reason.

@row_add n=24 row=contract/cold-agent-counted-verbs/lint area=log-coverage rec=15
boundary: The cold-agent pipe's counted verbs
item: lint
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.

@row_add n=25 row=contract/cold-agent-counted-verbs/screenshot area=log-coverage rec=15
boundary: The cold-agent pipe's counted verbs
item: screenshot
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.

@row_add n=26 row=contract/cold-agent-counted-verbs/advance area=log-coverage rec=15
boundary: The cold-agent pipe's counted verbs
item: advance
promise: A counted verb: its accepted `tool_call` span lines are counted from the server log.

@row_add n=27 row=decision/cold-agent-server-log-read-by-span-name area=log-coverage rec=15
name: Cold-agent server log read by span name only
rule: The cold-agent pipe reads the server log by span name only and never prints or copies it.

@row_add n=28 row=decision/decoded-screen-text-stays-in-the-match area=log-coverage rec=17
name: Decoded screen text stays in the match
rule: The cold-agent pipe's decoded paged screen text goes into the expect match only, never into an event or the verdict.

@row_none n=3 rec=18 type=5

@row_add n=29 row=contract/escher-session-exit-codes/0 area=log-coverage rec=19
boundary: escher-session exit codes
item: 0
promise: Accepted: one line of JSON on stdout, stderr empty with `RUST_LOG` unset; also `help`, the host role's quiet end and the `mcp` role's `Ok` end.

@row_add n=30 row=contract/escher-session-exit-codes/1 area=log-coverage rec=19
boundary: escher-session exit codes
item: 1
promise: Refused: one line of JSON on stdout, stderr empty with `RUST_LOG` unset.

@row_add n=31 row=contract/escher-session-exit-codes/2 area=log-coverage rec=19
boundary: escher-session exit codes
item: 2
promise: Usage: one fixed line on stderr built from `VERBS` and the six task names, stdout empty.

@row_add n=32 row=contract/escher-session-exit-codes/3 area=log-coverage rec=19
boundary: escher-session exit codes
item: 3
promise: Session error: the error object on stdout and its fixed message on stderr.

@row_add n=33 row=contract/escher-session-error-object/remedy area=log-coverage rec=19
boundary: escher-session's error object
item: remedy
promise: The error object carries a fixed `remedy` beside `kind` and `message`.

@row_add n=34 row=contract/escher-session-commands/help area=log-coverage rec=19
boundary: escher-session commands
item: help
promise: `help` (at most one `--session <dir>`, not read) writes the self-description's one line of JSON on stdout, status 0, reaching no session; `help` with an argument is refused `malformed`, status 1.

@row_add n=35 row=contract/escher-session-commands/mcp area=log-coverage rec=19
boundary: escher-session commands
item: mcp
promise: `escher-session mcp` writes JSON-RPC protocol lines only on stdout and nothing at its end: status 0 on success, status 3 with the error's fixed message on stderr on failure; any argument but one `--record <dir>` is usage, status 2.

@row_add n=36 row=contract/escher-session-commands/open-recording-directory area=log-coverage rec=19
boundary: escher-session commands
item: recording directory open to group or others
promise: `start` ends with status 3 and nothing spawned, and `mcp --record` with status 3, its fixed message on stderr and stdout empty.

@row_add n=37 row=contract/escher-session-stderr/telemetry-not-installed area=log-coverage rec=19
boundary: escher-session's stderr
item: telemetry not installed
promise: The binary's one direct stderr print is `telemetry not installed: …`.

@row_add n=38 row=decision/fixed-diagnostics area=log-coverage rec=19
name: Fixed diagnostics
rule: Every escher-session diagnostic is a fixed string holding no path, label, id or value, and no ending holds anything the caller typed.

@row_add n=39 row=contract/escher-session-coverage/command-shape area=log-coverage rec=20
boundary: escher-session's coverage role
item: command shape
promise: `escher-session coverage <app> <check>...` takes an app the binary boots and one or more paths, no `--session` or `--record` among them; any other shape is usage, status 2.

@row_add n=40 row=contract/escher-session-coverage/accepted area=log-coverage rec=20
boundary: escher-session's coverage role
item: accepted
promise: One uncoloured JSON coverage line on stdout, status 0.

@row_add n=41 row=contract/escher-session-coverage/boot-failure area=log-coverage rec=20
boundary: escher-session's coverage role
item: boot failure
promise: A boot that fails ends with the session error, status 3.

@row_add n=42 row=contract/escher-session-coverage/malformed area=log-coverage rec=20
boundary: escher-session's coverage role
item: malformed
promise: A file that cannot be read, is over 1,048,576 bytes, is no check or is a check of another app is refused `malformed`, status 1, holding nothing of the path or the file.

@row_add n=43 row=contract/command-span/line area=log-coverage rec=21
boundary: The driver's command span
item: line
promise: One closed-span line per command run, INFO at target `escher_driver`, `span="command"`, with the recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`; no line at the default `warn`.

@row_add n=44 row=contract/command-span/verb area=log-coverage rec=21
boundary: The driver's command span
item: verb
promise: One of the verb table's fourteen words; the three session-level ones and the tool-level `help` appear only beside the cause `unknown-verb`.

@row_add n=45 row=contract/command-span/cause area=log-coverage rec=21
boundary: The driver's command span
item: cause
promise: One of eight cause names, present when the command is refused.

@row_add n=46 row=contract/command-span/busy area=log-coverage rec=21
boundary: The driver's command span
item: busy
promise: One of three busy-class words.

@row_add n=47 row=contract/command-span/screenshot area=log-coverage rec=21
boundary: The driver's command span
item: screenshot
promise: A shot's line is `span="command" verb="screenshot"` (with `cause` when refused) and holds no image byte, size or dimension.

@row_add n=48 row=contract/command-span/lint area=log-coverage rec=21
boundary: The driver's command span
item: lint
promise: A lint's line is `span="command" verb="lint"` (with `cause` when refused) and holds no finding, id, bound or count.

@row_add n=49 row=contract/command-span/snapshot-part area=log-coverage rec=21
boundary: The driver's command span
item: snapshot part read
promise: A part read's line is `span="command" verb="snapshot"` (with `cause` when refused, a `part` outside 1 to 1000 refused `malformed`) and holds no part number, count, size or text.

@row_add n=50 row=contract/command-span/graph area=log-coverage rec=21
boundary: The driver's command span
item: graph
promise: A graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and holds no node, state, id or name.

@row_add n=51 row=contract/command-span/actions area=log-coverage rec=21
boundary: The driver's command span
item: actions
promise: An actions line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no node, id, owner, verb word or cause word of its answer.

@row_add n=52 row=contract/host-wire/v3 area=log-coverage rec=21
boundary: The escher-session host wire
item: v3
promise: Wire version `v3` carries `hello`, `stop` and `call`.

@row_add n=53 row=contract/host-wire/recording-failed area=log-coverage rec=21
boundary: The escher-session host wire
item: recording-failed
promise: A call after which the host's recording ended is answered by the one fixed reply `recording-failed`.

@row_add n=54 row=decision/coverage-line-never-logged area=log-coverage rec=21,22,46
name: Coverage line never logged
rule: The handler coverage line is a command line answer only, never logged or fielded; each replayed step of a kept check writes its own command span line.
guard: a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace

@row_add n=55 row=contract/tool-call-span/line area=log-coverage rec=21
boundary: The driver's tool_call span
item: line
promise: One closed-span line per MCP tool call, INFO at target `escher_driver`, `span="tool_call"`, with the recorded ones of `tool` and `ending`, then `message="close"`, `time.busy`, `time.idle`; no line at the default `warn`.

@row_add n=56 row=contract/tool-call-span/tool area=log-coverage rec=21
boundary: The driver's tool_call span
item: tool
promise: One of the tool table's fourteen words.

@row_add n=57 row=contract/tool-call-span/ending area=log-coverage rec=21
boundary: The driver's tool_call span
item: ending
promise: One of `accepted` · `refused` · `error`.

@row_add n=58 row=contract/tool-call-span/help area=log-coverage rec=21
boundary: The driver's tool_call span
item: help
promise: A `help` call's line reads `span="tool_call" tool="help" ending="accepted"` with no `command` line beside it.

@row_add n=59 row=contract/command-span/field-count area=log-coverage rec=21
boundary: The driver's command span
item: field count
promise: The `command` span keeps its eight fields; the driver's fixed help and meaning texts add no log site or span field.

@row_add n=60 row=contract/coverage-line/shape area=log-coverage rec=22
boundary: The handler coverage line
item: shape
promise: `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`.

@row_add n=61 row=contract/coverage-line/owner area=log-coverage rec=22
boundary: The handler coverage line
item: owner
promise: `owner` is left out when absent.

@row_add n=62 row=decision/capability-statement-updated-by-hand area=log-coverage rec=23
name: Capability statement updated by hand
rule: A change to the driver's span set or to the scrub's limits is made in the published capability statement by hand.
why: The statement's pin reads none of its emitter claims.

@row_none n=4 rec=24 type=5

@row_none n=5 rec=25 type=5

@row_none n=6 rec=26 type=5

@row_none n=7 rec=27 type=5

@row_none n=8 rec=28 type=5

@row_none n=9 rec=29 type=5

@row_none n=10 rec=30 type=5

@row_none n=11 rec=31 type=5

@row_none n=12 rec=32 type=5

@row_none n=13 rec=33 type=4
relied: The mutation buffer's replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.

@row_add n=63 row=contract/graph-answer/shape area=log-coverage rec=34
boundary: The graph answer
item: shape
promise: `{"nodes":[…],"states":[…]}`.

@row_add n=64 row=decision/graph-answer-never-logged area=log-coverage rec=34
name: Graph answer never logged
rule: The graph answer is written as a command answer alone, never logged or fielded.

@row_none n=14 rec=35 type=4
relied: The actions read acts on nothing and changes nothing in the document.

@row_none n=15 rec=36 type=4
relied: A page with no registered state reaches no signal registry and dispatches as before.

@row_none n=16 rec=37 type=4
relied: The reached record holds stable ids and event names only, no node number, and its debug form prints its count alone.

@row_none n=17 rec=38 type=5

@row_none n=18 rec=39 type=5

@row_none n=19 rec=40 type=6

@row_none n=20 rec=41 type=7

@row_none n=21 rec=42 type=7

@row_none n=22 rec=43 type=7

@row_none n=23 rec=44 type=7

@row_none n=24 rec=45 type=7

@row_none n=25 rec=47 type=7

@row_none n=26 rec=48 type=5

@row_none n=27 rec=49 type=7

@row_none n=28 rec=50 type=7

@row_none n=29 rec=51 type=7

@row_none n=30 rec=52 type=7

@row_none n=31 rec=53 type=7

@row_none n=32 rec=54 type=7

@row_none n=33 rec=55 type=7
