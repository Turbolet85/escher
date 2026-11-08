@task_prose part=chunk
Builds one read-only driver read of the current screen's actionable nodes (v030-06): each node's stable id, its optional owner, its verbs read from the schema's verb table VERBS with no closed list of verb names, and per verb whether a call would be accepted now or the one refusal cause it would return.
The node model is the UI graph's GraphNode, one per snapshot node, with an optional owner element; nothing is keyed on a DOM NodeId.
Refusal causes come from the one path the executor uses (action_point: disabled, then off-screen, then covered), so the read agrees with a real call by construction.
It runs no step, dispatches nothing and changes nothing, like snapshot, lint and graph; the actionable-key check (residual L12) may fold in if it fits.
The verb-table change, the carrier (socket call, command line, MCP, kept check) and the answer's shape are founder forks; proven on the stand in both layout modes with minimal fixtures for covered, off-screen and owner-served targets.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the verb table VERBS
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: VerbSpec
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: action_point, the shared refusal path
@task_file n=4 path=packages/escher-driver/src/json.rs
@task_file n=5 path=packages/escher-driver/src/cli.rs
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs
@task_file n=8 path=packages/escher-driver/src/recording.rs
@task_file n=9 path=packages/escher-driver/src/replay.rs
@task_file n=10 path=packages/escher-driver/src/describe.rs
@task_file n=11 path=docs/capabilities.md
@task_file n=12 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the capability statement pin
@task_file n=13 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=14 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=15 path=packages/dioxus-native-dom/src/actionable.rs
@task_item n=1 part=needs
The command span of escher-driver: the new read, run through Session::run, opens one INFO span command at target escher_driver recording verb and, when refused, cause alone; no span field holds the read's content, its ids, verb lists or its size.
@task_item n=2 part=needs
The tool_call span over MCP: a new verb answered on the MCP server opens one tool_call span whose tool field is the verb table's word and whose ending is accepted, refused or error; no other field.
@task_item n=3 part=needs
The escher-driver span census: the crate keeps its 2 info_span! sites and emits no event; the new read adds no span, event, log site, subscriber or env read.
@task_item n=4 part=needs
The new answer (a new Outcome variant, beside Outcome::Graphed and Outcome::Linted) is written by Outcome::to_json alone as a command's answer, over MCP its JSON line, kept whole in a recorded check, and never logged, fielded or printed with Debug.
@task_item n=5 part=needs
The answer carries stable ids, verb names and closed cause words (disabled, covered, off-screen) and booleans only, never a NodeId, page text or control value; the PII scrub of escher's sink and the content-named span field set apply to anything that does reach a span.
@task_item n=6 part=needs
The recording module writes each answered call's arguments, accepted, the answer digest and the answer into the kept check at rest through the crate's json writer, never as a log line, field or print; replay reads with serde_json only.
@task_item n=7 part=needs
describe's self-description (help over the command line and MCP) gains the new verb as one line of JSON through the crate's json writer, never a log line, and adds no subscriber, env read or print.
@task_item n=8 part=needs
The escher-session command line writes the new read's answer or refusal as one uncoloured JSON line on stdout and the mcp role writes protocol lines only on stdout; the sink stays on stderr only.
@task_item n=9 part=needs
host_log and mcp_log, which read a sink-installing escher-session host and mcp role at trace under both builds, must still read 0 id, name and typed-text needles with the new verb called, and one tool_call line per call.
@task_item n=10 part=needs
dioxus-native-dom's graph, snapshot and actionable modules are read here and stay read-only; any reader P3 adds there holds no log site, print, env read or failure path.
@task_item n=11 part=needs
The executor modules command, refusal, schema and execute install no subscriber, read no env var and no clock, and print nothing; a shared refusal path reused by the read keeps that.
@task_item n=12 part=needs
The new stand checks and fixtures (covered, off-screen and owner-served targets, both layout modes) install no subscriber, read no env var and print nothing, their readings going into assertions only, the silence census reading 0 over them and over the shared modules session_common, common and fixtures they read.
@task_standing n=1
convention: each new stand or engine check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, and the silence census reads 0 over the file and any shared module it reads
home: log-coverage
@task_standing n=2
convention: each new escher-driver module or answer type adds no tracing site, env read or print; an Outcome is written by to_json alone and never logged, fielded or printed with Debug
home: log-coverage
@task_standing n=3
convention: escher-driver opens only the command and tool_call spans, their fields closed words, bools and counts, and emits no event; a new verb adds no span or field
home: span-trace-coverage
@task_standing n=4
convention: every field reaching escher's sink passes the allowlist scrub, content-named fields redacted at print time; host_log and mcp_log read 0 needles at trace under both builds after the change
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
The upstream browser app's logging, its FPS overlay, urlbar and document-load log lines and its fmt::init stdout subscriber.
@task_item n=2 part=not_needed
blitz-vibey-script's JS error surface, take_js_errors and take_messages drains, and the file: load refusal.
@task_item n=3 part=not_needed
The WPT runner's env_logger logging, panic backtrace hook and report message fields.
@task_item n=4 part=not_needed
Metric coverage: frame and phase timings, debug_timer, log-times features and paint layer counters.
@task_item n=5 part=not_needed
The engine's resource-load, SVG parse, font and fetch error logging in blitz-dom, blitz-net and blitz-shell, and the URL and attribute values those lines carry.
@task_item n=6 part=not_needed
The upstream publish builds' CARGO_LOG, the WPT score publishing on upstream, and the apt-install script's diagnostic.
@task_item n=7 part=not_needed
The agent-run harness log and its events.jsonl and run.log.
@task_item n=8 part=not_needed
WASM panic hooks (console_error_panic_hook) and wasm_hello's tracing_wasm subscriber.
@task_item n=9 part=not_needed
The engine tracing feature wiring across blitz crates, dioxus-native and the apps.
