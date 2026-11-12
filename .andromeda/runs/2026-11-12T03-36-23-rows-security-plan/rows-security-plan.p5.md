@row_add n=1 row=absence/served-network-api area=api-security rec=1
capability: A served network API: no binary serves a TCP or UDP port
@row_add n=2 row=decision/session-socket-sole-listener area=api-security rec=1
name: Session socket as the one listener
rule: The driver session's Unix-domain socket is the project's one listener, serving the session lifecycle and driver calls and their answers.
@row_add n=3 row=decision/mcp-server-over-stdio area=api-security rec=1,41
name: MCP server over process stdio
rule: The MCP server role serves the driver's verbs as tools over its own stdin and stdout only, binding no listener, port or socket, and answers only the process that spawned it, with no handshake, token, secret or peer check.
why: The crossing was ratified as is.
@row_add n=4 row=contract/mcp-server/request-line area=api-security rec=1
boundary: MCP server
item: request line
promise: A request line is bounded at 65,536 bytes.
@row_add n=5 row=contract/mcp-server/tool-result area=api-security rec=1
boundary: MCP server
item: tool result
promise: A tool result is bounded at 1,048,576 bytes, one holding an image measured by its whole written form and a text-only one by its text.
@row_add n=6 row=contract/mcp-server/idle-expiry area=api-security rec=1
boundary: MCP server
item: idle expiry
promise: The idle expiry is checked when a request arrives.
@row_add n=7 row=contract/mcp-server/help area=api-security rec=1
boundary: MCP server
item: help
promise: The help tool is answered by the server itself from the schema's fixed tables, running nothing on a session.
@row_add n=8 row=contract/session-wire/recording-failed area=api-security rec=1
boundary: Driver session wire
item: ok v3 recording-failed
promise: Wire v3 replies include the fixed reply ok v3 recording-failed.
@row_add n=9 row=decision/outbound-per-host-limit area=api-security rec=2
name: Outbound per-host concurrency limit
rule: Concurrent outbound requests are limited to 6 per host.
@row_add n=10 row=absence/cors-same-origin-policy area=api-security rec=3
capability: A CORS or same-origin policy
@row_add n=11 row=absence/content-security-policy area=api-security rec=4
capability: A content security policy
@row_add n=12 row=absence/response-body-size-limit area=api-security rec=5
capability: A response body size limit on outbound requests
@row_add n=13 row=absence/request-timeout area=api-security rec=6
capability: A request timeout on outbound requests
@row_add n=14 row=decision/settle-never-waits-on-loads area=api-security rec=7
name: Settle never waits on a load
rule: A harness settle reports a load in flight at once as not settled with the class Loads, waits on no clock and moves none, and its pass loop is bounded at 64 passes.
why: The absent request timeout cannot hang a settle.
@row_add n=15 row=decision/load-wait-bound area=api-security rec=7
name: Acting commands bounded by LOAD_WAIT
rule: An acting driver command waits on a load in flight for at most LOAD_WAIT, 5 s of wall time, moving no time on the held instance, then returns the load still in flight as settled false with its busy class.
why: The absent request timeout cannot hang a command.
@row_add n=16 row=decision/time-moves-only-on-advance area=api-security rec=7
name: Time moves only on advance
rule: Time on a held instance moves only when a caller asks for it with advance, 1 to 60000 ms a call; no driver verb has an argument or result field of a wait, and snapshot, screenshot, lint, graph and actions wait on no load.
@row_add n=17 row=decision/scrollbar-fade-on-document-clock area=api-security rec=7
name: Scrollbar fade reads the document clock
rule: The overlay scrollbar fade reads the document's clock, so on a harness-built document two shots of one state read the same pixels until an advance moves the fade; a document built with no clock still fades by wall time.
why: Ratified.
@row_add n=18 row=decision/failed-load-ends-wait area=api-security rec=7
name: A failed load ends the wait
rule: A failed load ends a settle's wait, so a failed stylesheet never leaves a document not settled on Loads for good.
@row_add n=19 row=contract/driver-answer/settled area=api-security rec=7
boundary: Driver command answer
item: settled
promise: A step that ran and did not go quiet is a result, settled false with its busy class, not a refusal; the outcome names a class only, never a URL, request, node, id, name or value.
@row_add n=20 row=contract/driver-command-line/answer-bounds area=api-security rec=7
boundary: Driver command line
item: answer bounds
promise: A started host has 30 s to answer and each read of a call's answer is bounded at 30 s; these bound a transport and wait on no load.
@row_none n=1 rec=8 type=4
@row_none n=2 rec=9 type=4
@row_none n=3 rec=10 type=4
relied: Aborting the configured AbortSignal cancels every in-flight fetch tied to the document.
@row_none n=4 rec=11 type=4
@row_add n=21 row=decision/js-fetch-get-head-only area=api-security rec=12
name: JS-facing fetch is GET and HEAD only
rule: Script-facing fetch allows only GET and HEAD, over the embedder's ScriptFetcher.
@row_none n=5 rec=13 type=4
@row_none n=6 rec=14 type=4
relied: Sub-document navigations route back to the parent document instead of navigating the host.
@row_none n=7 rec=15 type=4
@row_none n=8 rec=16 type=4
@row_none n=9 rec=17 type=4
@row_none n=10 rec=18 type=4
@row_add n=22 row=decision/outbound-http-through-blitz-net area=api-security rec=19
name: Outbound HTTP through blitz-net
rule: The apps' outbound HTTP goes through blitz-net with http2, and in rdme through reqwest.
@row_add n=23 row=decision/ci-github-api-through-gh area=api-security rec=20
name: CI GitHub API calls through gh
rule: CI calls the GitHub API through the gh CLI, with GH_TOKEN taken from the workflow token secret.
@row_add n=24 row=decision/session-socket-owner-only area=api-security rec=21,40
name: Session socket reachable by the owning user only
rule: Access to the driver session socket is the filesystem's alone: the state directory is 0700, the socket file 0600, a directory open to its group or others is refused, and there is no handshake, token, secret or peer-credential check.
@row_add n=25 row=contract/session-socket/connections area=api-security rec=21
boundary: Driver session socket
item: connections
promise: One connection is served at a time, with a 2 s read and write bound.
@row_add n=26 row=contract/session-socket/request-line area=api-security rec=21
boundary: Driver session socket
item: request line
promise: A request line is at most 16,384 bytes.
@row_add n=27 row=contract/session-socket/lifecycle-reply area=api-security rec=21
boundary: Driver session socket
item: lifecycle reply
promise: A lifecycle reply is at most 128 bytes on its JSON line.
@row_add n=28 row=contract/session-socket/oversize area=api-security rec=21
boundary: Driver session socket
item: oversize
promise: An answer is at most 1,048,576 bytes on its JSON line; over it the reply is oversize, read by the client as AnswerTooLarge.
@row_add n=29 row=contract/session-socket/stop area=api-security rec=21
boundary: Driver session socket
item: stop
promise: stop removes the socket and the state directory and writes no record.
@row_add n=30 row=decision/host-idle-expiry area=api-security rec=21
name: Host idle expiry
rule: A host that has answered no request for IDLE_EXPIRY, 1,800 s, ends exactly as on stop; every answered request, refused or malformed included, restarts the count, and a connection broken before its request is whole does not.
why: The answer to the orphaned-host question: a host whose client died without stop kept running.
@row_add n=31 row=decision/graph-answer-unpaged area=api-security rec=22
name: The graph answer is unpaged
rule: The graph verb takes no argument and a screen's whole graph is one answer under the 1,048,576-byte answer bound.
why: A part argument would widen the crossing.
guard: stand_graph the_whole_cells_answer_is_within_the_answer_bound
@row_add n=32 row=decision/actions-answer-unpaged area=api-security rec=23
name: The actions answer is unpaged
rule: The actions verb takes no argument and a screen's whole answer is one answer under the 1,048,576-byte answer bound.
guard: stand_actions the_whole_cells_answer_is_within_the_answer_bound
@row_add n=33 row=absence/authentication-authorization area=authentication-authorization rec=24,25,26,27,28,29,30,31,32,33,34
capability: Authentication or authorization of users or callers, and credential handling
@row_none n=11 rec=35 type=4
@row_none n=12 rec=36 type=6
@row_none n=13 rec=37 type=6
@row_add n=34 row=decision/publish-secrets-upstream-main-only area=authentication-authorization rec=38
name: Publish secrets reachable only upstream
rule: The publish job runs only in DioxusLabs/blitz and uses the Signed Builds environment only on main or ci-test branches, so no fork ref reaches the environment or its secrets.
@row_add n=35 row=decision/ci-token-read-only area=authentication-authorization rec=39
name: ci.yml token is read-only
rule: ci.yml declares a workflow-level contents read permission and no job-level grant, so every ci.yml job's GITHUB_TOKEN is read-only.
guard: test_ci_workflows.py
@row_add n=36 row=decision/kept-check-owner-only area=data-protection rec=42,44
name: Kept checks are owner-only
rule: A recorded check is written 0600 into a 0700 directory the launcher named, is never logged, and is never removed by escher but for its own check when a recording ends.
why: Ratified.
except: Two scripted stand-Counter checks, one through the command line and one through the mcp role, are committed publicly, holding no typed text, host path, name or secret and no screenshot step.
guard: recording::the_committed_checks_are_what_the_binary_records
@row_add n=37 row=decision/kept-check-answer-whole area=data-protection rec=42
name: Kept check keeps each answer whole
rule: A kept check keeps each answer as the line the driver wrote, masked as on every carrier, with a shot kept as its width and height and never its pixels.
why: Ratified, a shot's pixels excepted.
@row_add n=38 row=decision/kept-check-call-content area=data-protection rec=42
name: Kept check call-content class
rule: A kept check holds call content: target ids, press keys, advance ms and text typed into unmasked controls, a masked or unresolved type's text kept as MASKED_STAND_IN.
why: Ratified.
@row_add n=39 row=decision/kept-check-graph-answer area=data-protection rec=42
name: Kept check holds the graph answer
rule: A kept graph answer holds ids, owner ids, component names, registered state names, handler event names and closed flag words and booleans, never a control value or typed text.
why: A boundary widening, ratified.
@row_add n=40 row=decision/kept-check-actions-answer area=data-protection rec=42
name: Kept check holds the actions answer
rule: A kept actions answer holds ids, owner ids, keyed booleans, the table's verb words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.
guard: stand_replay a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes
state: provisional
@row_add n=41 row=contract/kept-check-file/entry area=data-protection rec=42
boundary: Kept check file
item: entry
promise: A check is a JSON array: the app label, then per answered call its verb, its arguments, its acceptance, a FNV-1a 64 digest of its written answer and the answer itself as answer.
@row_add n=42 row=decision/cold-agent-records-private area=data-protection rec=42
name: Cold-agent recordings stay private
rule: The cold-agent pipe records into an owner-only directory, never prints, copies or commits the checks, reads them for bools and counts only and removes them with its state on cleanup.
@row_add n=43 row=contract/coverage-line/unexercised area=data-protection rec=43
boundary: Coverage answer line
item: unexercised
promise: An array of id, owner and event per unexercised handler, owner left out when absent.
@row_add n=44 row=contract/coverage-line/checks area=data-protection rec=43
boundary: Coverage answer line
item: checks
promise: The count of checks replayed.
@row_add n=45 row=contract/coverage-line/held area=data-protection rec=43
boundary: Coverage answer line
item: held
promise: The count of checks that held.
@row_add n=46 row=decision/coverage-line-no-user-data area=data-protection rec=43
name: Coverage line carries no user data
rule: The coverage answer line holds ids, owner ids, event names and counts only, never a control value, typed text, path or NodeId.
guard: stand_handler_coverage a_typed_sentinel_never_reaches_the_coverage_line
@row_add n=47 row=decision/dependency-audit-cargo-deny area=dependency-security rec=45
name: Dependency audit by cargo-deny
rule: cargo-deny checks advisories and licences over the whole workspace graph, all features, for the six platforms CI builds, on every push.
why: Ratified: audit the whole workspace and check licences.
@row_add n=48 row=decision/advisory-ignores-by-id area=dependency-security rec=45
name: Advisory ignores by ID only
rule: An advisory is ignored only by its ID, each with a written reason, with no blanket allow and unmaintained and unsound at their defaults.
@row_add n=49 row=decision/local-audit-before-push area=dependency-security rec=46
name: Run the audit leg locally before a push
rule: The local fast leg runs no audit, so the audit leg is run locally before a push.
why: A CI audit can go red on advisories published after a green audit with the lock and deny config unchanged.
@row_none n=14 rec=47 type=8
@row_none n=15 rec=48 type=8
@row_none n=16 rec=49 type=8
@row_none n=17 rec=50 type=8
@row_none n=18 rec=51 type=8
@row_add n=50 row=decision/no-git-dependency area=dependency-security rec=52,53,54
name: No git dependency
rule: The workspace takes no git dependency.
@row_add n=51 row=decision/engine-dependency-pins area=dependency-security rec=53,54
name: Engine dependency pins
rule: The pins stay: Stylo 0.22.0, Taffy the registry 0.15, Dioxus 0.7.3 and winit =0.31.0-beta.3.
@row_add n=52 row=decision/serde-json-read-only area=dependency-security rec=55
name: serde_json, read only
rule: serde_json, with no features and no preserve_order, reads the MCP server's untrusted stdin, a kept check's bytes and the driver's own answer line for replay, and writes nothing.
why: Chosen as read-only use.
