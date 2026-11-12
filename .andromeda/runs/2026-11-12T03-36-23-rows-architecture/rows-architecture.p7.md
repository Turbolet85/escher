@row_none n=1 rec=1 type=5
@row_add n=1 row=contract/escher-session-command-line/coverage area=existing-scopes rec=2
boundary: escher-session command line
item: coverage <app> <check>...
promise: The coverage role lists the handlers its named kept checks left unexercised, the same boot closure called once per check.
@row_none n=2 rec=3 type=5
@row_none n=3 rec=4 type=5
@row_none n=4 rec=5 type=5
@row_none n=5 rec=6 type=5
@row_add n=2 row=decision/driver-unit-tests-in-path-children area=existing-scopes rec=7
name: Driver unit tests in path children
rule: An escher-driver module holds its unit tests in a #[path] child file of its own, never a module of the crate's public tree.
@row_none n=6 rec=8 type=5
@row_none n=7 rec=9 type=4
@row_none n=8 rec=10 type=5
@row_none n=9 rec=11 type=5
@row_none n=10 rec=12 type=5
@row_add n=3 row=decision/dioxus-bridge-buffered-render area=existing-scopes rec=13,14
name: Dioxus bridge buffered render
rule: The Dioxus bridge renders buffered, not borrowed, so a task spawned by a key handler can access the document and the focus it sets acts.
guard: dioxus_runtime_probe's two mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_add n=4 row=decision/internal-crates-unpublished area=inherited-defaults rec=15
name: Internal crates unpublished
rule: The root package is virtual and unpublished, and the app, example and test crates, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script and wpt set publish = false.
except: rdme sets publish = true.
@row_add n=5 row=decision/published-crates-versioned-together area=inherited-defaults rec=15
name: Published crates versioned together
rule: The published packages/ library crates are versioned together through the workspace version and exact = pins between them.
@row_add n=6 row=absence/in-repo-release-tool area=inherited-defaults rec=15
capability: An in-repo release tool
@row_add n=7 row=decision/ci-quality-gates area=inherited-defaults rec=16
name: CI quality gates
rule: Every change passes cargo fmt, clippy with warnings denied, rustdoc with warnings denied over every workspace library crate, a cargo-deny audit of advisories and licences, per-crate coverage floors and a zero-failure stand-repeat budget.
guard: the CI legs of .github/scripts/ci-leg.sh
@row_add n=8 row=decision/engine-tracing-gated-per-call-site area=inherited-defaults rec=17
name: Engine tracing gated per call site
rule: Optional capabilities are Cargo features, and tracing in the engine crates is gated per call site.
except: escher-telemetry and escher-driver, its command and tool_call spans included, are ungated.
@row_add n=9 row=absence/served-network-port area=occupied-resources rec=18
capability: A served network port: no TCP or UDP port any binary serves
@row_add n=10 row=decision/session-socket-one-listener area=occupied-resources rec=18
name: Session socket the one listener
rule: The driver session's Unix-domain socket is the only listener served: mode 0600 inside a caller-named state directory created 0700, with no second listener and no network port.
why: A boundary widening, ratified.
@row_add n=11 row=decision/session-socket-carries-calls area=occupied-resources rec=18
name: Session socket carries calls
rule: The session socket carries calls and their answers: an element id and typed text cross in; ids, accessible names, masked password and file-input values, snapshot text and diff cross out, never a NodeId.
why: A boundary widening, ratified.
@row_add n=12 row=contract/session-socket-wire/version area=occupied-resources rec=18
boundary: the session socket's wire
item: version
promise: The wire version is v3.
@row_add n=13 row=contract/session-socket-wire/hello area=occupied-resources rec=18
boundary: the session socket's wire
item: hello
promise: The hello request is served.
@row_add n=14 row=contract/session-socket-wire/stop area=occupied-resources rec=18
boundary: the session socket's wire
item: stop
promise: The stop request is served.
@row_add n=15 row=contract/session-socket-wire/call area=occupied-resources rec=18
boundary: the session socket's wire
item: call
promise: The call request runs a driver call and returns its answer.
@row_add n=16 row=contract/session-socket-wire/recording-failed area=occupied-resources rec=18
boundary: the session socket's wire
item: recording-failed
promise: A call's replies include the fixed recording-failed reply.
@row_add n=17 row=contract/session-socket-wire/request-line-bound area=occupied-resources rec=18
boundary: the session socket's wire
item: request line bound
promise: A request line is at most 16,384 bytes.
@row_add n=18 row=contract/session-socket-wire/answer-bound area=occupied-resources rec=18
boundary: the session socket's wire
item: answer bound
promise: An answer is at most 1,048,576 bytes.
@row_add n=19 row=contract/session-socket-wire/non-unix-targets area=occupied-resources rec=18
boundary: the session socket's wire
item: non-unix targets
promise: On targets other than unix the lifecycle and a call return SessionError::Unsupported.
@row_add n=20 row=decision/mcp-role-over-stdio area=occupied-resources rec=18
name: MCP role over standard streams
rule: The escher-session mcp role speaks newline-delimited JSON-RPC 2.0 over its own stdin and stdout to the MCP client that spawned it, opening no listener, socket or port.
why: A boundary widening, ratified.
@row_add n=21 row=contract/mcp-role/tool-call-crossings area=occupied-resources rec=18
boundary: the mcp role
item: tool call crossings
promise: A tool call brings an element id and typed text in; a result takes ids, accessible names, masked password and file-input values, snapshot text and diff out.
@row_add n=22 row=contract/mcp-role/help area=occupied-resources rec=18
boundary: the mcp role
item: help
promise: The help tool is answered by the server with no session run.
@row_add n=23 row=contract/mcp-role/handshake-revision area=occupied-resources rec=18
boundary: the mcp role
item: handshake revision
promise: The mcp role speaks four handshake revisions, the fourth, of 25 November 2025, its default.
@row_add n=24 row=contract/mcp-role/self-description area=occupied-resources rec=18
boundary: the mcp role
item: self-description and remedy
promise: A result carries the tool's self-description and a session error's remedy, from the program's own fixed static table text.
@row_add n=25 row=contract/mcp-role/tool-listing area=occupied-resources rec=18
boundary: the mcp role
item: tool listing
promise: The tool listing names each verb's result fields.
@row_add n=26 row=contract/driver-diff/differs area=occupied-resources rec=18,19
boundary: the driver diff
item: differs
promise: A changed node's differs names closed node-key words, bounds among them.
@row_add n=27 row=decision/screen-pixels-in-band area=occupied-resources rec=18,19
name: Screen pixels in band
rule: A screenshot leaves in band inside the answer on both carriers, open select labels as painted pixels included, adding no listener or port and leaving the wire's bounds unchanged.
why: A boundary widening, ratified.
@row_add n=28 row=contract/session-socket-wire/screenshot-answer area=occupied-resources rec=18
boundary: the session socket's wire
item: screenshot answer
promise: The answer to a screenshot call holds its PNG as base64 inside the JSON line.
@row_add n=29 row=contract/mcp-role/screenshot-result area=occupied-resources rec=18
boundary: the mcp role
item: screenshot result
promise: A screenshot result is an image block (image/png, base64) then a text block, bounded by its whole written form.
@row_add n=30 row=contract/driver-lint/answer area=occupied-resources rec=18
boundary: the lint verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids and the closed defect words and numbers, never a NodeId, page text, colour or control value.
@row_add n=31 row=contract/driver-snapshot/row-bounds area=occupied-resources rec=19
boundary: the driver snapshot
item: row bounds
promise: An open select list's row carries its own bounds in the snapshot and the diff.
@row_add n=32 row=contract/driver-graph/answer area=occupied-resources rec=20
boundary: the graph verb
item: answer
promise: On both carriers and in a kept check, the answer holds stable ids, owner ids, component names, handler event names, registered state names, declared writes, the flags undeclared and unconfirmed and booleans; never a value, typed text or NodeId.
@row_add n=33 row=contract/driver-graph/argument area=occupied-resources rec=20
boundary: the graph verb
item: argument
promise: graph takes no argument; a call that gives one is refused malformed with nothing run.
@row_add n=34 row=contract/driver-snapshot/clickable area=occupied-resources rec=20
boundary: the driver snapshot
item: clickable
promise: A snapshot node's clickable is a closed bool, crossing in the snapshot and the diff.
