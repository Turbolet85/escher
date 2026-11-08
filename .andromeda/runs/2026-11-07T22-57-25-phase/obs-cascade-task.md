@task_prose part=chunk
Builds the UI graph (v030-05): one read that gives, for a held instance's current screen, each node, its component, its handlers, the state they write and the nodes that read that state.
Link 1 and link 2 are read by the dioxus-core vdom walk the stable ids already run (dynamic attributes, exact, mounted included); links 3 and 4 by a new escher-side signal registry in dioxus-native-dom that the seven_guis stand's signals move onto.
Adds the reacts-to-a-click reading as a closed boolean snapshot field (absorbing res-gvelrxja), moving NODE_FIELDS, the schema help texts, describe() and every pin that counts them.
The read crosses escher-driver as a new verb, outcome, written form, MCP tool and recorded step; values stay masked and no log site, span field or print carries a node, component name, handler, state or value.
Reserves a virtual-node id form, keys the graph on the agent-visible accesskit node with an optional owner element, and lets a handler on an owner serve sub-targets; no 0.4.0 capability is added.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=2 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs mark=edits
what: the pinned starting reading of the reacts-to-a-click field (0 on the lean tasks, Circle Drawer 1, Cells 676, both layout modes)
@task_item n=1 part=needs
dioxus-native-dom's new graph reader, signal registry and snapshot field: they add no log site, span, field, print or env read, and they carry no node, component name, handler, state name or value into any log, field or print.
@task_item n=2 part=needs
dioxus-native-dom's optional tracing feature and its existing debug logs in mutation_writer, which already record text-node contents and attribute values: the graph code must add no such site.
@task_item n=3 part=needs
Masking: a state value or control value the graph carries reads masked as on every carrier, a password's or a file input's value as the fixed mask MASKED_VALUE.
@task_item n=4 part=needs
escher-driver's command span: the new verb's call through Session::run opens the one INFO span command, whose verb field is the verb table's closed word. The eight fields stay closed words, bools and counts, and no graph content (component, handler, state, node) becomes a field.
@task_item n=5 part=needs
escher-driver's tool_call span: the new MCP tool opens tool_call with only tool (its verb-table word) and ending (accepted, refused or error), and adds no other field.
@task_item n=6 part=needs
escher-driver keeps its 2 info_span! sites and 0 events, and stand_act_spans keeps its eight fields, after the new verb and outcome land.
@task_item n=7 part=needs
The new Outcome variant is written in one place, by Outcome::to_json, as a command's answer and over MCP as its JSON line. It derives Debug like the rest of Outcome, but nothing logs, fields or prints it with Debug.
@task_item n=8 part=needs
escher-driver's recording module writes each answered call's arguments, digest and answer into the kept check through the crate's json writer, so the new outcome's answer lands there as a file at rest, never a log line, field or print.
@task_item n=9 part=needs
host_log and mcp_log read a sink-installing escher-session host and its mcp role at trace into their assertions: with the new verb and tool exercised, they read 0 id, name and typed-sentinel needles under both builds, and the span counts stay one tool_call line per call.
@task_item n=10 part=needs
escher's sink scrub: an escher_ target prints every field but the content-named set, an engine target only the safe fields, and every other target is dropped. Any closed span the new verb opens is judged pair by pair by that rule.
@task_item n=11 part=needs
escher-driver's describe() self-description and the schema help texts are written as one line of JSON through the crate's json writer, never a log line. Their moved text for the new verb and field stays an answer only.
@task_item n=12 part=needs
The seven_guis stand (seven_guis::stand and its tasks) installs no subscriber, reads no env var and prints nothing. Moving the stand's signals onto the registry keeps the silence census at 0 over stand.rs and the stand tasks.
@task_item n=13 part=needs
The two seven_guis binaries, seven_guis_native and escher-session, run escher_telemetry::init. The graph read reaches stderr only through that sink, and the escher-session command line writes the read's answer as one line of JSON on stdout, never a log line.
@task_standing n=1
convention: each new or extended test check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, with the silence census reading 0 over the file and any shared module it reads
home: log-coverage
@task_standing n=2
convention: each new escher-driver or dioxus-native-dom module holds no tracing site, env read or print, and adds no log site, span, field, log field or env read
home: log-coverage
@task_standing n=3
convention: each new outcome or read type that holds ids, names or values is written by to_json alone as an answer and is never logged, fielded or printed with Debug
home: pii-scrubbing-compliance
@task_standing n=4
convention: each new verb or MCP tool adds no span and no span field beyond command's eight and tool_call's two, each a closed word, bool or count
home: span-trace-coverage
@task_item n=1 part=not_needed
apps/browser's logged events and its visited-URL and urlbar logging: the chunk does not touch the browser app.
@task_item n=2 part=not_needed
The wpt runner's env_logger logging, its panic hook and its report message fields: no wpt code is touched.
@task_item n=3 part=not_needed
blitz-vibey-script's JS error surface, the take_js_errors and take_messages drains and the document_may_load refusal: no script runtime change.
@task_item n=4 part=not_needed
Metric coverage, debug_timer and the log-frame-times and log-phase-times features: the chunk adds no timing or counter.
@task_item n=5 part=not_needed
CI artifact handling, the publish builds' CARGO_LOG and apt-install.sh diagnostics: the chunk adds no CI job or artifact.
@task_item n=6 part=not_needed
The agent-run harness log and its events.jsonl format: the chunk does not change scripts/agent-run.sh.
@task_item n=7 part=not_needed
blitz-net fetch errors, the Resource load failed line and the network stand-ins HeldNet, LateNet and LoadNet: the graph reads no network.
@task_item n=8 part=not_needed
blitz-dom's engine log sites (layout, SVG parse warnings, pointer href logging, CSS invalid-value warnings) and its println! debug dumps: no blitz-dom engine change is expected.
@task_item n=9 part=not_needed
The upstream apps' fmt::init stdout subscribers and wasm_hello's tracing_wasm install: not touched.
@task_item n=10 part=not_needed
The silence records of earlier chunks' checks (select list, load wait, visual lint, clock and upstream-take checks): they are past measurements this chunk does not re-run.
@task_item n=11 part=not_needed
escher-telemetry's panic hook and its startup event: the chunk does not change escher-telemetry.
@task_item n=12 part=not_needed
SLO invariants, obs anti-patterns and the obs decisions log: they hold no recorded intent.
