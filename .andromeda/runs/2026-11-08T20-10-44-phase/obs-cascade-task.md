@task_prose part=chunk
Builds one handler-coverage command (v030-07): after a set of kept checks has been replayed on the stand through Check::from_bytes and Check::replay, it lists exactly the UI graph's handlers (a graph node id and a handler event name, as DioxusDocument::graph reads them) that none of those checks exercised.
Adds a record of which (element, event) handlers a run reached that holds on every app, registry or not, since the graph's confirmed flag and SignalRegistry::dispatches read only where an app registered a state.
States the canvas limit in the acceptance: only DOM listeners on agent-visible nodes are counted, one per (node, event); handlers a canvas dispatches inside one listener are not, the shape keyed on the graph node id with its optional owner, never on a NodeId.
Writes the owed replay test (CARRY cry-sxrivdk3): a recorded session holding an actions step, kept, read back and replayed on a fresh session, the replay holding, and a differing actions answer naming the step and the differing key; v030-06 reads verified only once it passes.
The coverage read is read-only: it runs no step of its own and carries no control value, typed text, page text beyond the graph verb's ids, component and event names, or NodeId.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=2 path=packages/dioxus-native-dom/src/signal_registry.rs
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=4 path=packages/escher-driver/src/replay.rs
@task_file n=5 path=packages/escher-driver/src/recording.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/command.rs
@task_file n=8 path=packages/escher-driver/src/execute.rs
@task_file n=9 path=packages/escher-driver/src/describe.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/mcp.rs
@task_file n=12 path=packages/escher-driver/src/mcp/listing.rs
@task_file n=13 path=.github/scripts/test_capability_statement.py
@task_file n=14 path=docs/capabilities.md
@task_file n=15 path=README.md
@task_item n=1 part=needs
The command span opened by execute for every call handed to Session::run: a coverage read or a replayed actions step runs under it, its eight fields closed words, bools and counts, never the screen text, an argument or an Outcome; escher-driver keeping its 2 info_span sites and emitting no event.
@task_item n=2 part=needs
The tool_call span over MCP tools/call: if the coverage read becomes a verb, its tool word joins the verb table's closed words and its ending stays accepted, refused or error, two fields and no other.
@task_item n=3 part=needs
How a new Outcome for the coverage answer is written: by Outcome::to_json alone as a command's answer, over MCP its JSON line, kept whole in a recorded check, never logged, fielded or printed with Debug, like Outcome::Graphed's ids, component names and handler event names.
@task_item n=4 part=needs
The values the coverage answer may carry: graph node ids, the optional owner, component and handler event names only; no control value, typed text, page text or NodeId, as the graph answer already holds.
@task_item n=5 part=needs
The recording module's kept check: each answered call's arguments, accepted, its answer's FNV-1a 64 digest and the answer itself written through the crate's json writer, a file at rest; the owed test's kept actions step lives there.
@task_item n=6 part=needs
The replay module and Check::replay: no tracing site, env read or print, serde_json only to read; the replay test's failure message names the step and the differing key, never a captured answer or page text.
@task_item n=7 part=needs
The recording directory's carve-out: owner-only 0700, refused when open to group or others, holding call content at rest, never a log or event line, uploaded by no CI leg.
@task_item n=8 part=needs
dioxus-native-dom's handler-reach record (signal_registry dispatches, graph handlers, the bridge dispatch path): it adds no log site, span, field, log field or env read, the debug logs of mutation_writer untouched.
@task_item n=9 part=needs
The sink's line format and allowlist scrub: any new span field or tool word passes the escher_ target rule, a content-named field printed only as [redacted], so no handler id or event name enters a field outside the closed set.
@task_item n=10 part=needs
host_log and mcp_log: if a new verb lands, both read it at trace under both builds with 0 id, name and typed-text needles and one tool_call line per call.
@task_item n=11 part=needs
The stand's headless boot in process installs no escher sink: the replay test and the coverage check replay checks in process on seven_guis::stand boots, so no sink line is produced by them.
@task_item n=12 part=needs
The verb count pins if a fifteenth verb is added: VERBS fourteen to fifteen and the pins holding that count (test_capability_statement.py, docs/capabilities.md, README tool count, schema_tests.rs, cli_tests.rs, stand_mcp_tools), stand_mcp_tools' Recorder reading the tool_call and command spans.
@task_item n=13 part=needs
The cold-agent pipe's server.log span counts by tool word and its counted verbs list: a new verb's place in that list is the Cold-agent bar entry's call, the pipe never printing server.log.
@task_item n=14 part=needs
The canvas limit's statement: handlers a canvas dispatches internally are not counted, 0.4.0 extending the count to virtual nodes keyed on node id and owner, never redefining it.
@task_standing n=1
convention: each new test or check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, the silence census reading 0 over the file and any shared module it reads
home: log-coverage
@task_standing n=2
convention: a new reader of tests/blitz-tests/tests/session_common/mod.rs, fixtures/mod.rs or common/mod.rs is recorded in that module's reader list and count
home: log-coverage
@task_standing n=3
convention: a new escher-driver module or function holds no tracing site, env read or print; the driver keeps its 2 info_span sites and the command span its eight fields
home: span-trace-coverage
@task_standing n=4
convention: a new Outcome, result field or read type derives Debug only if nothing logs, fields or prints it; it is written once, by to_json, as a command's answer
home: pii-scrubbing-compliance
@task_standing n=5
convention: a committed kept check or fixture holds no typed text, host path or secret
home: ci-integration
@task_standing n=6
convention: a new span field or MCP tool word is a closed word, bool or count, judged pair by pair by the sink's allowlist scrub
home: § 8. PII Scrubbing & Compliance
@task_item n=1 part=not_needed
Metric coverage: in-process timings, debug_timer, the log-times features and the WPT scores; the chunk adds no timing or counter export.
@task_item n=2 part=not_needed
Error capture: browser JS error drains, blitz-vibey-script's window error surface, take_js_errors and the 256-error bound; the chunk captures no errors (v030-10's).
@task_item n=3 part=not_needed
Panic hooks: console_error_panic_hook in WASM builds, the wpt runner's hook and escher-telemetry's chained hook; the chunk adds or changes none.
@task_item n=4 part=not_needed
Values logged as-is by the engine: visited URLs, urlbar text, resource URLs in blitz-dom and blitz-net, the SVG outer HTML field, CSS property values, debug_log_node; the chunk touches no engine log site.
@task_item n=5 part=not_needed
The upstream apps' tracing_subscriber fmt::init on stdout, wasm_hello's tracing_wasm and the wpt runner's env_logger; not on the chunk's path.
@task_item n=6 part=not_needed
Feature wiring of the tracing cargo feature across browser, rdme, blitz, blitz-vibey-script and dioxus-native; the chunk changes no feature.
@task_item n=7 part=not_needed
The agent-run harness log format and its state area under target/agent-run; the chunk adds no event to it.
@task_item n=8 part=not_needed
CI publish builds at CARGO_LOG info, the WPT report published upstream only, and apt-install.sh's diagnostic line; the chunk changes no workflow.
@task_item n=9 part=not_needed
The upstream take sink readings (ebacd68bd, e42ee1f25), blitz-net's test-only loopback binds and the file: load refusal; no upstream merge in this chunk.
@task_item n=10 part=not_needed
The select model, select list, load wait, visual lint, text_extent, contrast, keyboard and screenshot silence records; the chunk reads none of those surfaces.
@task_item n=11 part=not_needed
The empty sections SLO Invariants, Obs Anti-Patterns and Obs Decisions Log, which hold no recorded intent.
@task_item n=12 part=not_needed
OTel SDK init and telemetry backends, observed absent; the chunk adds no exporter.
