@row_add n=1 row=decision/closed-driver-verb-schema area=input-validation rec=1
name: Closed driver verb schema
rule: Every driver call is checked against a closed table of verbs and bounded argument kinds and answered with one typed command or one refusal, the first failure in a fixed order; a refused call changes nothing.
why: Each widening of the verb table is a widening of a validated surface, ratified.
@row_add n=2 row=decision/one-validation-path-for-every-carrier area=input-validation rec=1,6,8
name: One validation path for every carrier
rule: The session socket, the command line and MCP each pass a call through the schema's validation before anything runs, and nothing public runs a command that did not come from that validation.
why: The crossing to external input surfaces was ratified with this check as the one they all pass through.
@row_add n=3 row=contract/driver-argument-kinds/id area=input-validation rec=1
boundary: Driver argument kinds
item: id
promise: Text of 1 to 1024 bytes.
@row_add n=4 row=contract/driver-argument-kinds/text area=input-validation rec=1
boundary: Driver argument kinds
item: text
promise: Text of 0 to 4096 bytes.
@row_add n=5 row=contract/driver-argument-kinds/key area=input-validation rec=1
boundary: Driver argument kinds
item: key
promise: One name of a closed twelve-name list.
@row_add n=6 row=contract/driver-argument-kinds/flag area=input-validation rec=1
boundary: Driver argument kinds
item: flag
promise: True or false.
@row_add n=7 row=contract/driver-argument-kinds/milliseconds area=input-validation rec=1
boundary: Driver argument kinds
item: milliseconds
promise: A whole number from 1 to 60000.
@row_add n=8 row=contract/driver-argument-kinds/name area=input-validation rec=1
boundary: Driver argument kinds
item: name
promise: Text of 1 to 32 bytes of a-z, 0-9 and -.
@row_add n=9 row=contract/driver-argument-kinds/part area=input-validation rec=1
boundary: Driver argument kinds
item: part
promise: A whole number from 1 to 1000.
@row_add n=10 row=decision/unknown-id-refused-stale-or-not-found area=input-validation rec=1
name: Unknown id refused as stale or not-found
rule: An id that names no element on the screen as it reads now is refused with nothing run: stale when the session's record of ids holds it, not-found when not.
@row_add n=11 row=decision/target-refusal-order area=input-validation rec=1
name: Fixed order of target refusals
rule: A click or type on an element that cannot take it is refused with nothing run, the first of disabled, off-screen, covered that holds, leaving the snapshot and both focus readings as they were; scroll reports in_view instead of refusing.
why: The detection of refusal causes by code was ratified.
@row_add n=12 row=decision/advance-moves-virtual-clock-exactly area=input-validation rec=1
name: Advance moves the virtual clock exactly
rule: The advance verb, accepted on every session, moves the session's virtual clock by exactly the milliseconds asked inside one settled step and reports exactly that time.
why: Ratified.
@row_add n=13 row=decision/id-record-bounded-and-session-fed area=input-validation rec=1
name: Bounded id record fed by the session
rule: The session keeps no node handle between calls, only a record of at most 4096 id texts fed from its own snapshots and never from a call's argument.
@row_add n=14 row=decision/no-panic-on-call-input area=input-validation rec=1
name: No panic on call-supplied input
rule: Neither the schema check nor the executor has unwrap, expect, panic or unreachable on anything a call supplies, and neither reads a clock itself.
@row_add n=15 row=decision/screen-pixels-in-band area=input-validation rec=1,6,8
name: Screen pixels in band
rule: A screenshot's PNG travels inside the answer itself as base64 text, on the command line and as an MCP image block.
why: Ratified as the in-band crossing.
@row_add n=16 row=decision/driver-outcomes-never-logged area=input-validation rec=1
name: Driver outcomes never logged
rule: The driver crate logs and fields no command, call, argument, outcome, refusal, screen text, image, finding, session label or id record; an outcome is written only as a command-line answer or an MCP tool result.
why: Outcomes leave the process only by the ratified crossing.
@row_add n=17 row=decision/one-command-span-per-call area=input-validation rec=1
name: One command span per call
rule: Every call handed to the session is wrapped in one INFO span named command under the escher_driver target, entered before validation so a refusal is inside it, holding fixed words, booleans and counts only.
@row_add n=18 row=contract/driver-command-span/verb area=input-validation rec=1
boundary: The driver command span
item: verb
promise: The verb table's own word, never the caller's verb text.
@row_add n=19 row=contract/driver-command-span/cause area=input-validation rec=1
boundary: The driver command span
item: cause
promise: The name of the returned refusal's cause, unrecorded until it applies.
@row_add n=20 row=contract/driver-command-span/settled area=input-validation rec=1
boundary: The driver command span
item: settled
promise: A boolean, unrecorded until it applies.
@row_add n=21 row=contract/driver-command-span/busy area=input-validation rec=1
boundary: The driver command span
item: busy
promise: The schema's class word, unrecorded until it applies.
@row_add n=22 row=contract/driver-command-span/passes area=input-validation rec=1
boundary: The driver command span
item: passes
promise: A count, unrecorded until it applies.
@row_add n=23 row=contract/driver-command-span/added area=input-validation rec=1
boundary: The driver command span
item: added
promise: The length of the returned diff's added list.
@row_add n=24 row=contract/driver-command-span/removed area=input-validation rec=1
boundary: The driver command span
item: removed
promise: The length of the returned diff's removed list.
@row_add n=25 row=contract/driver-command-span/changed area=input-validation rec=1
boundary: The driver command span
item: changed
promise: The length of the returned diff's changed list.
@row_add n=26 row=contract/mcp-tool-call-span/tool area=input-validation rec=1
boundary: The MCP tool_call span
item: tool
promise: The verb table's word for the called tool; the span parents the call's command span.
@row_add n=27 row=contract/mcp-tool-call-span/ending area=input-validation rec=1
boundary: The MCP tool_call span
item: ending
promise: One of accepted, refused or error.
@row_add n=28 row=decision/driver-installs-no-subscriber area=input-validation rec=1,6
name: Driver installs no subscriber
rule: The driver crate and its command line install no tracing subscriber and read no environment variable; clocks are read only by the host's idle expiry, the MCP server on a request, and the session's bounded load wait.
@row_add n=29 row=contract/driver-verbs/graph area=input-validation rec=2
boundary: Driver verbs
item: graph
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[…],"states":[…]} of ids, owner ids, component names, state names, handler event names, flag words and booleans, never a control value, typed text or NodeId.
@row_add n=30 row=contract/driver-verbs/actions area=input-validation rec=3
boundary: Driver verbs
item: actions
promise: Takes no argument (any argument refused malformed, nothing run), runs no step, and answers {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent, never a control value, typed text or NodeId.
@row_add n=31 row=contract/actions-answer/keyed area=input-validation rec=3
boundary: The actions answer
item: keyed
promise: A boolean on each listed node; the field is provisional.
@row_none n=1 rec=4 type=4
relied: Every id-taking verb and the actions read meet one shared target check, so what the actions read lists agrees with what a call would be refused.
@row_none n=2 rec=5 type=4
relied: The actions read runs nothing and changes no snapshot, focus reading or diff, so it can be called without moving the session.
@row_add n=32 row=contract/cli-exit-codes/0 area=input-validation rec=6
boundary: CLI exit codes
item: 0
promise: Accepted: the answer as one line of JSON on stdout.
@row_add n=33 row=contract/cli-exit-codes/1 area=input-validation rec=6
boundary: CLI exit codes
item: 1
promise: Refused: the refusal on stdout.
@row_add n=34 row=contract/cli-exit-codes/2 area=input-validation rec=6
boundary: CLI exit codes
item: 2
promise: Usage: one fixed line on stderr, stdout empty, nothing booted and no state created.
@row_add n=35 row=contract/cli-exit-codes/3 area=input-validation rec=6
boundary: CLI exit codes
item: 3
promise: Session error: the error object with kind, message and remedy on stdout and its fixed message on stderr.
@row_add n=36 row=decision/no-echo-of-caller-input area=input-validation rec=6,7
name: No echo of caller input
rule: No command-line ending writes back anything the caller typed: no path, label, id, value or file content.
@row_add n=37 row=decision/closed-pipe-drops-answer area=input-validation rec=6
name: Closed pipe drops the answer
rule: A write to a closed stdout is dropped and the command keeps its exit status, never a panic.
guard: a_closed_stdout_drops_the_answer_and_keeps_the_ending
@row_add n=38 row=contract/cli-flags/id area=input-validation rec=6
boundary: escher-session flags
item: --id
promise: Followed by one value, whatever that word is.
@row_add n=39 row=contract/cli-flags/text area=input-validation rec=6
boundary: escher-session flags
item: --text
promise: Followed by one value, whatever that word is.
@row_add n=40 row=contract/cli-flags/key area=input-validation rec=6
boundary: escher-session flags
item: --key
promise: Followed by one value, whatever that word is.
@row_add n=41 row=contract/cli-flags/ms area=input-validation rec=6
boundary: escher-session flags
item: --ms
promise: Followed by one value, whatever that word is.
@row_add n=42 row=contract/cli-flags/part area=input-validation rec=6
boundary: escher-session flags
item: --part
promise: Followed by one value, whatever that word is.
@row_add n=43 row=contract/cli-flags/shift area=input-validation rec=6
boundary: escher-session flags
item: --shift
promise: Bare, taking no value.
@row_add n=44 row=contract/cli-flags/session area=input-validation rec=6
boundary: escher-session flags
item: --session
promise: Followed by a directory, given exactly once, else usage.
@row_add n=45 row=contract/cli-flags/record area=input-validation rec=6
boundary: escher-session flags
item: --record
promise: Followed by a directory, taken at most once and only on start, the host role and the mcp role; anywhere else, twice or with no value is usage.
@row_add n=46 row=contract/cli-roles/serve area=input-validation rec=6
boundary: escher-session roles
item: serve
promise: serve <app> --session <dir> in exactly that shape, optionally with one --record <dir> after it, else usage.
@row_add n=47 row=contract/cli-roles/mcp area=input-validation rec=6
boundary: escher-session roles
item: mcp
promise: mcp alone or with one --record <dir> selects the MCP role; any other word after it is usage.
@row_add n=48 row=contract/cli-roles/coverage area=input-validation rec=7
boundary: escher-session roles
item: coverage
promise: coverage <app> <check>... with no --session or --record, else usage; each check read up to 1,048,576 bytes, and an unreadable, oversized, non-check or other-app file refused malformed, status 1, holding nothing of the path or file.
@row_add n=49 row=contract/coverage-answer/line area=input-validation rec=7
boundary: The coverage answer
item: line
promise: One line {"unexercised":[…],"checks":n,"held":m} on stdout, status 0, holding ids, owner ids, event names and the two counts only.
@row_add n=50 row=decision/mcp-over-stdio area=input-validation rec=8
name: MCP over stdio only
rule: The MCP server speaks newline-delimited JSON-RPC 2.0 on the invoking process's stdin and stdout, with no listener, port, socket, authentication or environment read.
why: The MCP carrier and its bounds were ratified as is.
@row_add n=51 row=contract/mcp-request/line-length area=input-validation rec=8
boundary: MCP requests
item: request line length
promise: At most 65,536 bytes; a longer line is read to its terminator, dropped and answered -32700.
@row_add n=52 row=contract/mcp-request/id area=input-validation rec=8
boundary: MCP requests
item: id
promise: Echoed only as an i64 integer or a string of at most 128 bytes, else -32600 with id null; the one client-sent value written back.
@row_add n=53 row=contract/mcp-request/notification area=input-validation rec=8
boundary: MCP requests
item: a request with no id
promise: Treated as a notification: no reply, and nothing runs.
@row_add n=54 row=contract/mcp-errors/32700 area=input-validation rec=8
boundary: MCP protocol errors
item: -32700
promise: parse error, with no data: a non-UTF-8, non-JSON or overlong line.
@row_add n=55 row=contract/mcp-errors/32600 area=input-validation rec=8
boundary: MCP protocol errors
item: -32600
promise: invalid request, with no data: not an object with jsonrpc "2.0" and a string method, or an id of another form.
@row_add n=56 row=contract/mcp-errors/32601 area=input-validation rec=8
boundary: MCP protocol errors
item: -32601
promise: method not found, with no data: every method but initialize, ping, tools/list and tools/call.
@row_add n=57 row=contract/mcp-errors/32602 area=input-validation rec=8
boundary: MCP protocol errors
item: -32602
promise: invalid params or unknown tool, with no data: params not an object with a string name and an optional object arguments, or a name outside the verb table.
@row_add n=58 row=contract/mcp-initialize/protocol-revision area=input-validation rec=8
boundary: MCP initialize
item: protocol revision
promise: Matched against a closed table of four supported revisions; any other revision asked for is answered with the newest of the four.
@row_add n=59 row=contract/mcp-tool-arguments/value-kinds area=input-validation rec=8
boundary: MCP tool arguments
item: value kinds
promise: A string becomes text, an i64 a number, a bool a flag; any other value is refused malformed before anything runs, and a repeated key collapses to its last value.
@row_add n=60 row=contract/mcp-tool-result/size-bound area=input-validation rec=8
boundary: MCP tool results
item: size bound
promise: A result's text over 1,048,576 bytes is replaced by the answer-too-large result; a result holding an image is bounded by its whole written form.
@row_add n=61 row=contract/mcp-tool-result/screenshot area=input-validation rec=8
boundary: MCP tool results
item: screenshot
promise: An accepted shot is an image block (base64 PNG, image/png) then a text block of the shot's line without png, isError false; a refused shot is one text block with isError true.
@row_add n=62 row=decision/mcp-listing-fixed-text area=input-validation rec=8
name: MCP listing is fixed program text
rule: The MCP tool listing's descriptions are the program's own fixed table text: each verb's help and each result field's name, kind and help.
why: Ratified as the program's own text inside the ratified carrier.
@row_none n=3 rec=9 type=4
@row_add n=63 row=decision/external-navigation-get-http-mailto area=input-validation rec=10
name: External navigation limited to safe schemes
rule: Navigation handed to an external browser is limited to GET requests with http, https or mailto schemes.
@row_none n=4 rec=11 type=4
@row_none n=5 rec=12 type=4
@row_none n=6 rec=13 type=4
@row_none n=7 rec=14 type=4
@row_none n=8 rec=15 type=4
@row_none n=9 rec=16 type=4
@row_none n=10 rec=17 type=4
@row_add n=64 row=decision/file-urls-only-from-file-documents area=input-validation rec=18
name: file URLs load only from file documents
rule: A script fetch, a module import or a script src may load a file: URL only for a file: document, in every document, remote pages included.
why: A narrowing, not a boundary widening.
guard: remote_document_cannot_load_file_urls
@row_add n=65 row=decision/default-fetcher-file-and-data-only area=input-validation rec=18
name: Default script fetcher takes file and data only
rule: The default script fetcher takes only file: and data: URLs.
@row_add n=66 row=absence/same-origin-and-cors-checks area=input-validation rec=18
capability: Same-origin and CORS checks on script fetches
@row_none n=11 rec=19 type=4
@row_add n=67 row=decision/html-parsed-without-scripting area=input-validation rec=20
name: HTML parsed with scripting disabled
rule: HTML is parsed with the parser's scripting flag disabled.
@row_add n=68 row=decision/failed-stylesheet-blocks-no-render area=input-validation rec=21
name: Failed stylesheet blocks no render
rule: In every document, remote pages included, a stylesheet that fails to load, is empty, is not CSS or is not valid UTF-8 blocks no render.
guard: failed_stylesheet_unblocks_rendering, empty_or_non_css_stylesheet_unblocks_rendering
@row_add n=69 row=decision/unresolvable-stylesheet-href-no-load area=input-validation rec=22
name: Unresolvable stylesheet href starts no load
rule: In every document, a stylesheet href that cannot be resolved starts no load and blocks no render.
why: Resolution goes through the non-panicking resolver instead of the one that panics.
guard: invalid_stylesheet_url_does_not_block_rendering
@row_none n=12 rec=23 type=4
@row_none n=13 rec=24 type=4
@row_none n=14 rec=25 type=4
@row_none n=15 rec=26 type=4
@row_none n=16 rec=27 type=4
@row_add n=70 row=decision/hintless-font-face-source-sniffed area=input-validation rec=28
name: Hint-less font-face source fetched and sniffed
rule: An @font-face source whose URL cannot be resolved is skipped; a source with no format hint and no extension is fetched and its format sniffed from its bytes.
why: Ratified as upstream's behaviour, with a check asked for.
guard: font_face_sniffed_source
@row_add n=71 row=absence/url-taking-driver-verb area=input-validation rec=28
capability: A driver verb that takes a URL
@row_none n=17 rec=29 type=4
@row_none n=18 rec=30 type=4
@row_none n=19 rec=31 type=4
@row_none n=20 rec=32 type=4
@row_none n=21 rec=33 type=4
@row_none n=22 rec=34 type=4
@row_none n=23 rec=35 type=4
@row_none n=24 rec=36 type=4
@row_none n=25 rec=37 type=4
@row_none n=26 rec=38 type=4
@row_none n=27 rec=39 type=4
@row_none n=28 rec=40 type=4
@row_none n=29 rec=41 type=4
@row_none n=30 rec=42 type=4
@row_none n=31 rec=43 type=4
@row_none n=32 rec=44 type=4
@row_add n=72 row=decision/multiple-select-size-rows area=input-validation rec=45
name: Multiple select size sets its rows
rule: A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint, and a single select's size is unread.
why: A boundary widening, ratified.
guard: size_reads_a_whole_number_above_zero_capped
@row_none n=33 rec=46 type=4
@row_none n=34 rec=47 type=4
@row_add n=73 row=decision/disabled-read-by-presence area=input-validation rec=48
name: disabled read by presence on form controls
rule: The disabled attribute is read by its presence, whatever its value, and only on a button, input, select or textarea, for focusability, pointer and keyboard activation and the snapshot's enabled and clickable readings.
why: The presence reading was ratified.
@row_none n=35 rec=49 type=4
relied: The :open pseudo-class matches a details or dialog by its open attribute's presence, kept current by attribute writes and removals.
@row_none n=36 rec=50 type=4
@row_none n=37 rec=51 type=4
@row_none n=38 rec=52 type=4
