@row_none n=1 rec=1 type=S
@row_add n=1 row=contract/cssom-script-api/error-names area=error-handling rec=2
boundary: CSSOM script API
item: CSSOM errors
promise: CSSOM errors reach script as DOMException names.
@row_add n=2 row=contract/cssom-script-api/selectortext-assignment area=error-handling rec=2
boundary: CSSOM script API
item: selectorText assignment
promise: Assigning selectorText never throws.
@row_add n=3 row=contract/cssom-script-api/csstext-assignment area=error-handling rec=2
boundary: CSSOM script API
item: cssText assignment
promise: Assigning cssText never throws.
@row_add n=4 row=contract/js-module-loading/resolve-or-fetch-failure area=error-handling rec=3
boundary: JS module loading
item: resolve or fetch failure
promise: A module resolve or fetch failure reaches script as a JS TypeError.
@row_none n=2 rec=4 type=4
@row_add n=5 row=contract/mounted-element-operations/missing-node area=error-handling rec=5
boundary: Dioxus mounted-element operations
item: missing node
promise: An operation on a node that does not exist returns MountedError::OperationFailed wrapping NodeNotExistErr.
@row_none n=3 rec=6 type=4
@row_none n=4 rec=7 type=4
@row_add n=6 row=decision/ci-matrix-runs-every-job area=error-handling rec=8
name: CI matrix runs every job
rule: CI matrix jobs run with fail-fast off, so one failing matrix job does not cancel its siblings.
@row_add n=7 row=decision/refusal-closed-cause-set area=error-handling rec=9
name: Command refusal as a closed set of causes
rule: An escher-driver command refusal is a typed value from a closed set of eight causes, each with a fixed name, meaning and remedy holding no path, and with no free-text field.
why: So a refusal cannot hold anything a call supplied.
except: A Fault is carried only beside the malformed cause.
@row_add n=8 row=decision/session-error-fixed-table area=error-handling rec=9
name: Session errors from a fixed table
rule: An escher-driver session error is written from a fixed table, holding nothing a call supplied.
@row_add n=9 row=decision/mcp-errors-fixed-no-data area=error-handling rec=9
name: MCP protocol errors fixed, no data
rule: MCP protocol errors are fixed codes and fixed messages and carry no data member.
@row_add n=10 row=decision/refusal-logged-by-cause-name area=error-handling rec=9
name: Refusal logged by cause name only
rule: Of an escher-driver command refusal, only its cause's fixed name reaches a log.
@row_add n=11 row=decision/stale-node-ids-never-panic area=error-handling rec=11,10,17
name: Stale node ids never panic
rule: A stale node id is skipped in layout children and resolves to none on lookup rather than panicking; a stale animation entry for a removed node is skipped on the next resolve.
guard: stale_interaction_state, stale_node_mapping, animations and custom_widget_layout tests
@row_none n=5 rec=12 type=4
relied: Resolve and hit testing may run with no DOM and return early with a warning, and a UI event on a document with no target is ignored.
@row_none n=6 rec=13 type=4
@row_none n=7 rec=14 type=4
@row_none n=8 rec=15 type=4
relied: Event handling may ask for the document while it is mutably borrowed and gets None instead of a panic.
@row_none n=9 rec=16 type=4
@row_add n=12 row=contract/query-apis/missing-element area=error-handling rec=18
boundary: Query APIs
item: missing element
promise: Query APIs return Result<Option<NodeId>>, and a missing element is None.
@row_add n=13 row=contract/fragment-navigation/unknown-target area=error-handling rec=18
boundary: Fragment navigation
item: unknown fragment target
promise: An unknown fragment target returns None or false and leaves scroll at 0.
@row_add n=14 row=contract/wpt-results/crash area=error-handling rec=19
boundary: WPT runner results
item: CRASH
promise: A test that panics is caught and reported as a CRASH result.
@row_add n=15 row=contract/wpt-results/fail-uncaught-js-error area=error-handling rec=19
boundary: WPT runner results
item: FAIL with Uncaught JS error
promise: A harness test that never reports but had uncaught JS errors yields FAIL with an "Uncaught JS error" subtest.
@row_add n=16 row=contract/wpt-results/timeout area=error-handling rec=19
boundary: WPT runner results
item: TIMEOUT
promise: A harness test that never reports and had no uncaught JS errors yields TIMEOUT.
@row_none n=10 rec=20 type=S
@row_none n=11 rec=21 type=5
@row_none n=12 rec=22 type=5
@row_none n=13 rec=23 type=5
@row_none n=14 rec=24 type=5
@row_none n=15 rec=25 type=5
@row_none n=16 rec=26 type=5
@row_none n=17 rec=27 type=5
@row_none n=18 rec=28 type=5
@row_none n=19 rec=29 type=5
@row_none n=20 rec=30 type=5
@row_none n=21 rec=31 type=5
@row_add n=17 row=absence/error-reporting-integration area=error-handling rec=32
capability: An error reporting integration
@row_none n=22 rec=33 type=S
@row_none n=23 rec=34 type=S
@row_none n=24 rec=35 type=5
@row_add n=18 row=contract/network-tracing-log/url area=logging-monitoring rec=36
boundary: Network tracing log
item: url
promise: A network request log records the request URL as field url.
@row_add n=19 row=contract/network-tracing-log/status area=logging-monitoring rec=36
boundary: Network tracing log
item: status
promise: A network request log records the response status as field status.
@row_add n=20 row=contract/network-tracing-log/error area=logging-monitoring rec=36
boundary: Network tracing log
item: error
promise: A failed network request log records the failure as field error.
@row_add n=21 row=contract/network-tracing-log/path area=logging-monitoring rec=36
boundary: Network tracing log
item: path
promise: The cache directory path is logged at info level as field path.
@row_add n=22 row=contract/resource-load-failure-log/url area=logging-monitoring rec=37
boundary: Resource load failure log
item: url
promise: A resource load failure log carries structured field url.
@row_add n=23 row=contract/resource-load-failure-log/waiting-nodes area=logging-monitoring rec=37
boundary: Resource load failure log
item: waiting_nodes
promise: A resource load failure log carries structured field waiting_nodes.
@row_add n=24 row=contract/resource-load-failure-log/error area=logging-monitoring rec=37
boundary: Resource load failure log
item: error
promise: A resource load failure log carries structured field error.
@row_none n=25 rec=38 type=4
@row_add n=25 row=decision/js-console-to-debug-log area=logging-monitoring rec=39
name: JS console routed to the debug log
rule: Script console output (log, info, warn, error) goes to the log crate at debug level under target js_console, never to stdout or stderr.
why: Keeps stdout and stderr clean.
@row_none n=26 rec=40 type=4
@row_none n=27 rec=41 type=5
@row_add n=26 row=decision/release-publish-upstream-only area=logging-monitoring rec=42
name: Release publishing only from upstream
rule: The release-cli publish job is repository-guarded and runs only in the upstream DioxusLabs/blitz repository.
@row_none n=28 rec=43 type=S
@row_add n=27 row=decision/tracing-for-engine-logging area=logging-monitoring rec=44
name: tracing for engine logging
rule: Engine crates log through the tracing crate behind an optional tracing feature, and their logging macros compile only with that feature on.
@row_add n=28 row=decision/tracing-feature-propagates area=logging-monitoring rec=45
name: One tracing feature across the stack
rule: Turning on dioxus-native's tracing feature also turns on tracing in dioxus-native-dom, blitz-shell, blitz-dom, blitz-html and blitz-net.
@row_add n=29 row=decision/env-logger-for-wpt-runner area=logging-monitoring rec=46
name: env_logger for the WPT runner
rule: The WPT runner logs through env_logger.
@row_add n=30 row=decision/escher-telemetry-allowlist-sink area=logging-monitoring rec=47
name: escher's allowlist telemetry sink
rule: Both seven_guis binaries log through escher's own stderr-only allowlist subscriber.
except: The scrubbing reaches that sink only: upstream's fmt::init subscribers, env_logger and the panic hook stay unscrubbed.
@row_add n=31 row=decision/engine-target-field-allowlist area=logging-monitoring rec=47
name: Engine log targets keep only allowlisted fields
rule: In escher's telemetry sink an engine target keeps only seven allowlisted fields, and every other field is written as [redacted].
except: An allowlisted log.file field can still carry a host path.
@row_add n=32 row=decision/escher-target-content-fields-withheld area=logging-monitoring rec=47
name: escher_ targets withhold content-named fields
rule: In escher's telemetry sink an escher_ target keeps all fields but eleven content-named ones.
@row_add n=33 row=decision/other-log-targets-dropped area=logging-monitoring rec=47
name: Other log targets dropped whole
rule: In escher's telemetry sink any target other than an engine or escher_ target is dropped whole at every level, and spans follow the same rule.
@row_add n=34 row=contract/escher-session-stdout/command-line area=logging-monitoring rec=48
boundary: escher-session stdout
item: command result line
promise: escher-session writes one uncoloured JSON line per command on stdout.
@row_add n=35 row=contract/escher-session-stdout/mcp-role area=logging-monitoring rec=48
boundary: escher-session stdout
item: mcp role
promise: The mcp role writes JSON-RPC lines only on stdout.
@row_add n=36 row=contract/escher-session-stderr/usage-messages area=logging-monitoring rec=48
boundary: escher-session stderr
item: usage messages
promise: escher-session writes usage messages on stderr.
@row_add n=37 row=contract/escher-session-stderr/error-messages area=logging-monitoring rec=48
boundary: escher-session stderr
item: error messages
promise: escher-session writes error messages on stderr.
@row_add n=38 row=decision/session-output-holds-no-caller-input area=logging-monitoring rec=48
name: Session output holds no caller input
rule: No escher-session output line holds anything the caller typed.
except: The mcp role's echoed request id.
@row_none n=29 rec=49 type=7
@row_add n=39 row=decision/agent-run-events-metadata-only area=logging-monitoring rec=50
name: Agent-run events hold harness metadata only
rule: The agent-run test contract's JSON-line events carry no content-named field, only harness metadata, and never captured test output or panic text.
@row_add n=40 row=decision/agent-run-raw-log-unprinted area=logging-monitoring rec=50
name: Agent-run raw log kept unprinted
rule: The agent-run raw cargo and libtest output log is unscrubbed, never printed by logs, and gitignored.
@row_add n=41 row=decision/cold-agent-events-metadata-only area=logging-monitoring rec=51
name: Cold-agent events hold run metadata only
rule: The cold-agent pipe's events and verdict carry counts, booleans and closed words, never transcript text, a tool argument or answer, a reply or the check.
@row_add n=42 row=decision/cold-agent-record-unprinted area=logging-monitoring rec=51
name: Cold-agent record kept unprinted
rule: The cold-agent server log, check, raw transcript and the live child's kept checks are never printed; the transcript stays unscrubbed and gitignored, and each committed copy has its host paths masked.
@row_add n=43 row=absence/monitoring-and-alerts area=logging-monitoring rec=52
capability: Monitoring and alerting
@row_none n=30 rec=53 type=S
@row_none n=31 rec=54 type=S
@row_none n=32 rec=55 type=S
