@row_none n=1 rec=1 type=4

@row_none n=2 rec=2 type=4

@row_none n=3 rec=3 type=4

@row_none n=4 rec=4 type=4

@row_none n=5 rec=5 type=4

@row_none n=6 rec=6 type=4

@row_none n=7 rec=7 type=4

@row_none n=8 rec=8 type=4

@row_add n=1 row=decision/cells-grid-keyboard area=input-validation rec=9
name: Cells grid keyboard model
rule: The 7GUIs Cells grid takes focus with tabindex 0 and a closed key match and no role: arrows move the current cell, clamped, Enter opens the editor on it, focused, and the editor's Enter commits, stops propagation and returns focus to the grid.
why: Ratified as to the grid.

@row_none n=9 rec=10 type=4

@row_none n=10 rec=11 type=5

@row_add n=2 row=decision/stderr-only-telemetry-sink area=logging-monitoring rec=12
name: Escher's own stderr telemetry sink
rule: Both seven_guis binaries, the native app and the escher-session driver host, install escher's own telemetry sink first thing: a stderr-only subscriber whose formatter applies an allowlist of targets.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed, and the chained std panic hook still prints the raw panic message to stderr.

@row_add n=3 row=decision/engine-target-safe-fields area=logging-monitoring rec=12
name: Engine targets print only safe fields
rule: An event whose target starts with blitz, dioxus_native, stylo_taffy, accesskit_xplat, debug_timer or js_console prints only node_id, status, waiting_nodes, property, log.module_path, log.file and log.line; every other field, the message included, prints as [redacted].

@row_add n=4 row=decision/escher-target-content-scrub area=logging-monitoring rec=12
name: Escher targets scrub content-named fields
rule: An event whose target starts with escher_ prints every field except url, href, src, html, text, value, attrs, path, request, error and panic.payload, which print as [redacted].

@row_add n=5 row=decision/outside-target-dropped area=logging-monitoring rec=12
name: Outside targets dropped whole
rule: An event or bridged log record from a target outside the allowlist is not printed at all, zero bytes, at every level including WARN and ERROR, whatever RUST_LOG names.
why: The named cost: a third-party WARN or ERROR no longer prints either.

@row_add n=6 row=decision/closed-span-lines area=logging-monitoring rec=12
name: One sink line per closed span
rule: The sink prints one line per closed span whose target the allowlist admits and nothing on creation, record, enter or exit; each span field is judged by the event rule, an outside-target span writes no byte, and a content-named span field's value is never stored.
why: A boundary widening of the sink, ratified as built.

@row_add n=7 row=contract/tool-call-span/line area=logging-monitoring rec=12
boundary: The tool_call span line
item: line
promise: One closed line per MCP tools/call, the parent of that call's command span, holding tool and ending only.

@row_add n=8 row=contract/tool-call-span/tool area=logging-monitoring rec=12
boundary: The tool_call span line
item: tool
promise: The tool table's word for the call, a closed word outside the content-named fields; absent for an unknown tool.

@row_add n=9 row=contract/tool-call-span/ending area=logging-monitoring rec=12
boundary: The tool_call span line
item: ending
promise: One of the closed words accepted, refused or error.

@row_add n=10 row=contract/command-span/line area=logging-monitoring rec=12
boundary: The command span line
item: line
promise: A sink-installing host writes one closed command span line per call it runs.

@row_add n=11 row=contract/command-span/cause area=logging-monitoring rec=12,19
boundary: The command span line
item: cause
promise: Recorded when the call is refused, as a closed cause word such as fonts-unavailable.

@row_add n=12 row=decision/no-screen-content-in-logs area=logging-monitoring rec=12,18,19,20
name: Screen content stays out of logs
rule: No driver call puts screen content in a log: typed text, element ids, accessible names, a shot's image, lint findings, a part's text, a UI graph, an actions answer and a coverage check reach no log line at any level.
guard: stand_act_spans, host_log and mcp_log, read at RUST_LOG=trace under both builds

@row_add n=13 row=contract/escher-session-exit-codes/0 area=logging-monitoring rec=13
boundary: escher-session exit codes
item: 0
promise: An accepted command: one uncoloured JSON line on stdout holding the answer, or for the coverage role its one coverage line.

@row_add n=14 row=contract/escher-session-exit-codes/1 area=logging-monitoring rec=13
boundary: escher-session exit codes
item: 1
promise: A refusal: one uncoloured JSON line on stdout holding the refusal, or for the coverage role a malformed refusal.

@row_add n=15 row=contract/escher-session-exit-codes/2 area=logging-monitoring rec=13
boundary: escher-session exit codes
item: 2
promise: The usage line on stderr with stdout empty; the usage line holds the clause coverage role: coverage <app> <check>....

@row_add n=16 row=contract/escher-session-exit-codes/3 area=logging-monitoring rec=13
boundary: escher-session exit codes
item: 3
promise: The session-error object on stdout and the session error's fixed message on stderr; in the mcp role an Err ending's fixed message on stderr.

@row_add n=17 row=contract/escher-session-output/caller-text area=logging-monitoring rec=13
boundary: escher-session output
item: caller-typed text
promise: No output line holds anything the caller typed, except the admitted request id the mcp role echoes.

@row_add n=18 row=contract/escher-session-output/esc-byte area=logging-monitoring rec=13
boundary: escher-session output
item: ESC byte
promise: No output line holds an ESC byte; output is uncoloured whatever the stream is.

@row_add n=19 row=contract/escher-session-output/accepted-stderr area=logging-monitoring rec=13
boundary: escher-session output
item: stderr of an accepted command
promise: An accepted command's stderr is empty with RUST_LOG unset.

@row_add n=20 row=contract/escher-session-mcp-stdout/protocol-lines area=logging-monitoring rec=13
boundary: The escher-session mcp role's stdout
item: protocol lines
promise: The mcp role writes JSON-RPC protocol lines only on stdout, and nothing at its end.

@row_add n=21 row=contract/escher-session-coverage-line/content area=logging-monitoring rec=13
boundary: The escher-session coverage role's stdout line
item: content
promise: The coverage line holds ids, owner ids, event names and two counts, and nothing of a named path.

@row_none n=11 rec=14 type=4
relied: A failed load's error text stops in the load response and the redacted Resource load failed line, and no carrier takes it.

@row_none n=12 rec=15 type=5

@row_none n=13 rec=16 type=4
relied: A select list's option and group labels leave only as painted pixels and the snapshot's existing name, the list holding no log site, span or field.

@row_none n=14 rec=17 type=4
relied: The render-path mutation buffer logs nothing itself and its replay re-fires the mutation writer's own trace calls, so the log stream is the one the direct write gives.

@row_add n=22 row=contract/command-span/verb area=logging-monitoring rec=18,19
boundary: The command span line
item: verb
promise: Records the instance-level verb; a graph or actions call records the verb alone, plus cause when refused.

@row_add n=23 row=decision/cold-agent-logs-metadata-only area=logging-monitoring rec=21
name: Cold-agent pipe logs run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words only: no content-named field and no transcript text, tool argument, tool answer, reply or check.
guard: The scrub-set key check and four planted markers, under the shim

@row_add n=24 row=decision/cold-agent-transcript-raw area=logging-monitoring rec=21
name: Cold-agent raw transcript kept unscrubbed
rule: The cold-agent session's raw transcript stays unscrubbed by design, is never printed and is gitignored; each committed copy has its host paths, the session dir and the home prefix, masked.

@row_add n=25 row=decision/cold-agent-server-output-unprinted area=logging-monitoring rec=21
name: Cold-agent server output never printed
rule: The cold-agent pipe reads its server's stderr log only by counting span lines and writes the agent's check as parsed, printing or copying neither; the replay children's stderr goes to /dev/null.

@row_add n=26 row=decision/cold-agent-recordings-owner-only area=logging-monitoring rec=21
name: Cold-agent recordings owner-only
rule: The cold-agent live server records its kept checks into an owner-only directory, read for booleans and counts only, never printed, copied or committed, and removed at cleanup; the replay children run without recording.

@row_add n=27 row=decision/cold-agent-wrong-call-bar area=logging-monitoring rec=21
name: Cold-agent wrong-call bar
rule: The cold-agent verdict decides its wrong calls against a 5 % bar, failing with the reason wrong-calls-over-bar above it.

@row_add n=28 row=decision/ci-secrets-in-actions area=secret-management rec=22
name: CI secrets in GitHub Actions secrets and vars
rule: CI secrets live in GitHub Actions secrets and vars: signing keys, the keystore and its passwords, the Apple certificate, the cross-repo dispatch token and the workflow token are read from them.

@row_add n=29 row=decision/ci-signing-material-removed area=secret-management rec=22
name: CI signing material removed after use
rule: A signing key or keystore that CI writes to disk from a secret is removed in an always() step.

@row_add n=30 row=decision/live-runs-use-operator-login area=secret-management rec=23
name: Live runs use the operator's own login
rule: Live cold-agent and MCP trial runs reach the model provider only through the operator's own Claude Code login held outside the repository: no API key, no env read, no CI secret, no new secret source and no auth library; CI runs only the shim contract tests.
why: Ratified.

@row_none n=15 rec=24 type=7

@row_none n=16 rec=25 type=7

@row_none n=17 rec=26 type=7

@row_none n=18 rec=27 type=7

@row_none n=19 rec=28 type=7

@row_none n=20 rec=29 type=7

@row_none n=21 rec=30 type=7

@row_none n=22 rec=31 type=7

@row_none n=23 rec=32 type=7

@row_none n=24 rec=33 type=7

@row_none n=25 rec=34 type=7

@row_none n=26 rec=35 type=7

@row_none n=27 rec=36 type=7

@row_add n=31 row=decision/iframe-nesting-depth-limit area=threat-model-summary rec=37
name: Iframe nesting depth limit
rule: Iframe nesting depth is limited.
why: Guards against infinitely recursive self-embedding pages.

@row_add n=32 row=decision/import-nesting-depth-limit area=threat-model-summary rec=37
name: CSS @import nesting depth limit
rule: CSS @import nesting depth is limited.
why: Prevents unbounded recursion, such as an import URL that grows geometrically per level.

@row_add n=33 row=decision/stale-iframe-responses-discarded area=threat-model-summary rec=37
name: Stale iframe responses discarded
rule: An iframe response whose iframe was removed or re-navigated since the request is discarded by request id.

@row_add n=34 row=decision/skip-snapshot-of-unstyled-node area=threat-model-summary rec=37
name: Never-styled nodes not snapshotted
rule: Snapshotting a never-styled node is skipped.
why: Stylo invalidation would otherwise panic.

@row_add n=35 row=decision/script-fetcher-file-data-only area=threat-model-summary rec=38
name: Script fetcher takes file and data URLs only
rule: The default script fetcher supports only file: and data: URLs; any other scheme returns UnsupportedScheme.

@row_add n=36 row=decision/file-url-only-for-file-documents area=threat-model-summary rec=38
name: File URLs load only for file documents
rule: A file: URL loads only for a file: document; for any other document, remote pages included, JS fetch, module import and script src alike refuse it with a FetchError.

@row_add n=37 row=decision/browser-javascript-opt-in area=threat-model-summary rec=38
name: Browser JavaScript opt-in
rule: In the browser app JavaScript execution is opt-in, behind the non-default javascript feature.

@row_add n=38 row=decision/bootstrap-native-helper-removed area=threat-model-summary rec=38
name: Bootstrap native helper hidden from pages
rule: The URLSearchParams bootstrap's native search-params helper is deleted from globalThis before any page script runs.

@row_add n=39 row=decision/session-socket-owner-only area=threat-model-summary rec=39
name: Session socket owner-only
rule: The driver session's one Unix-domain socket sits in a caller-named state directory created 0700, a present one open to group or others refused, the socket file 0600; no other authentication, no TCP or UDP port, unix only.

@row_add n=40 row=decision/session-socket-carries-calls area=threat-model-summary rec=39
name: Session socket carries calls and answers
rule: The session socket carries the lifecycle and calls with their answers: an element id and typed text in; ids, accessible names, control values with a password's or file input's as the fixed mask, the snapshot text and the diff out; no NodeId crosses.
why: A boundary widening, ratified.

@row_add n=41 row=decision/snapshot-state-readings area=threat-model-summary rec=39
name: Three state readings per control
rule: The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, as closed words and booleans.
why: Inside the ratified crossing, ratified.

@row_add n=42 row=decision/screenshot-in-band area=threat-model-summary rec=39,43
name: Screenshot carried in band
rule: A screenshot's PNG travels in band, base64 inside the socket's JSON answer line or as an MCP image block, the whole screen or one element's visible part, a password input painted masked.
why: The in-band image was the ratified choice; the answer bound is unchanged.

@row_add n=43 row=decision/lint-findings-crossing area=threat-model-summary rec=39,43
name: Lint findings cross every ratified carrier
rule: A lint's findings cross every ratified carrier as stable ids, closed defect words and numbers, never page text, colour or value.

@row_add n=44 row=decision/open-list-shot-pixels area=threat-model-summary rec=39,43
name: Open select list shot carries labels as pixels
rule: A shot of an open select list carries the page's option labels and group labels as pixels, on the socket and MCP carriers alike, the shot's fields unmoved.
why: A boundary widening, ratified.
guard: cli_commands, pinning a hosted shot equal to an in-process one

@row_add n=45 row=decision/select-list-crossing area=threat-model-summary rec=39,43
name: Select list crossing limited to bounds and pixels
rule: The select list crosses the ratified carriers only as a row's own bounds in the snapshot and the diff and as pixels in a shot; node state holds no field of it and no expanded node key stands.

@row_add n=46 row=decision/graph-answer-crossing area=threat-model-summary rec=40,44
name: Graph answer crossing
rule: A graph answer carries the screen's UI graph on the socket, the command line, MCP and a kept check: ids, owner ids, component names, registered state names, handler event names, closed flag words and booleans, never a control value, typed text or NodeId.
why: A boundary widening, ratified.

@row_add n=47 row=decision/actions-answer-crossing area=threat-model-summary rec=41,45
name: Actions answer crossing
rule: An actions answer carries what each actionable node can take now on the socket, the command line, MCP and a kept check: ids, owner ids, keyed booleans, verb-table words and closed cause words, never a control value, typed text or NodeId.
why: A boundary widening, ratified.

@row_add n=48 row=contract/actions-answer/nodes area=threat-model-summary rec=41
boundary: The actions answer
item: nodes
promise: The answer is an object whose nodes list holds one entry per actionable node, each with id, owner, keyed and verbs.

@row_add n=49 row=contract/actions-answer/id area=threat-model-summary rec=41
boundary: The actions answer
item: id
promise: The node's id.

@row_add n=50 row=contract/actions-answer/owner area=threat-model-summary rec=41
boundary: The actions answer
item: owner
promise: The node's owner id.

@row_add n=51 row=contract/actions-answer/keyed area=threat-model-summary rec=41
boundary: The actions answer
item: keyed
promise: A boolean.

@row_add n=52 row=contract/actions-answer/verbs area=threat-model-summary rec=41
boundary: The actions answer
item: verbs
promise: A list of entries each holding verb, a word of the verb table, and cause, a closed cause word.

@row_add n=53 row=decision/coverage-check-files-untrusted area=threat-model-summary rec=42
name: Kept check files read as untrusted
rule: The coverage role reads each caller-named kept check as untrusted input through a bounded read and a strict parse; an unreadable, oversized, non-check or other-app file is refused malformed, holding nothing of the path or the file.

@row_add n=54 row=decision/coverage-answer-crossing area=threat-model-summary rec=42
name: Coverage answer crossing
rule: The coverage answer line carries ids, owner ids, event names and the checks and held counts only.
why: A boundary widening, ratified.

@row_add n=55 row=decision/mcp-stdio-no-auth area=threat-model-summary rec=43
name: MCP server over stdio, no authentication
rule: The driver's MCP server runs as a stdio child a client spawns under the invoking user, JSON-RPC 2.0 on stdin and stdout, no listener, port or socket; the spawning client is the trust boundary, with no authentication.

@row_add n=56 row=decision/mcp-result-crossing area=threat-model-summary rec=43
name: MCP result crossing
rule: An MCP result carries ids, accessible names, control values with a password's or file input's as the mask, the snapshot text and an acting call's diff to the client; what a client does onward, a model provider included, is the client's.
why: A boundary widening, ratified.

@row_add n=57 row=decision/cold-agent-screen-to-provider area=threat-model-summary rec=43
name: Cold-agent runs carry the screen to the provider
rule: A cold-agent live run carries the stand's screen, element ids, accessible names and control values in snapshot texts and diffs, through the claude client to the model provider, and the model's reply returns as an agent-written check the pipe parses and replays.
why: A boundary widening, ratified.

@row_add n=58 row=decision/actions-keyed-field area=threat-model-summary rec=45
name: Actions answer keyed field
rule: The actions answer carries a keyed boolean for each actionable node.
state: provisional

@row_add n=59 row=decision/capability-statement-mcp-crossing area=threat-model-summary rec=46
name: Committed statement of the MCP crossing
rule: A committed capability statement tells its readers what an MCP client carries onward: the spawning client as the boundary with no authentication, the carried set, and onward use as the client's; it adds no carrier.
