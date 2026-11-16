@task_prose part=chunk
An acting driver call (click, type, press, advance, scroll) whose step causes a handler panic, an error-level log record or a handler's own log record returns it in that call's answer, beside the diff it returns today.
Each returned item is tied to the stable element id the step acted on and, through the UI graph, to the handler (owner id and event name) that ran; a handler panic on a held session is caught on the run path and returned as the call's outcome instead of ending the host process.
The same answer field rides every ratified carrier the acting verbs ride (in process, the session socket's call, the command line, MCP, a kept check recorded and replayed), and help and the MCP tool listing describe it.
What of a log may enter an answer is the founder's boundary fork, bounded by his log rulings; escher-telemetry's target allowlist and scrub stay as they are and no span or log line carries the new answer content.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for an acting step — edits: the run path, catching a handler panic and capturing what the step caused
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the escher-driver session, Session::run and the held session's acting calls — edits: the run path and the call's outcome
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command and refusal schema and the acting verb results — edits: the answer, a new result field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description (help) shared by the command line and MCP — edits: the self-description of the new field
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's own hand-written JSON answer writer — edits: the answer's written form
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server role, its tools/list tool listing and tool results — edits: the MCP listing
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording that writes the kept check file — edits: the kept check's recorded answer
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check reader and Check::replay through the one run path — edits: the kept check's replayed answer
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: the escher-telemetry bootstrap, init and the subscriber with its target filter, where a step's records may be captured
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: the escher-telemetry line format and scrub of what the sink prints
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: the escher-telemetry chaining panic hook and its panic event
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph and its registry, the handler identity (owner and writers) an answer item is tied through
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the Dioxus DOM bridge's reached record of handlers, written before dispatch
@task_file n=14 path=examples/seven_guis
what: the seven_guis headless stand, beside which a minimal fixture app whose handler panics or logs proves the behaviour
@task_file n=15 path=tests/blitz-tests/tests/
what: the blitz-tests integration test targets, where a new target for this behaviour runs in both layout modes
@task_item n=1 part=needs
The acting verb results (click, type, press, advance, scroll) gain a field carrying the handler panic, error-level log record or handler log record the step caused, beside the existing added, changed, removed, settled and busy results and the diff.
@task_item n=2 part=needs
Each item in the new field is tied to the stable element id the step acted on and, through the UI graph's existing node identity, to the handler (owner id and event name); no NodeId crosses a session or the socket, and no process-local value enters the answer.
@task_item n=3 part=needs
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome rather than ending the hosting process; what the instance is after a caught panic (poisoned, rebooted or held) and how locks recover from poisoning is read by P3.
@task_item n=4 part=needs
A per-step capture of records emitted during the step, bounded per call and attributed to the step's target and handler, through the tracing subscriber, the reached record or the Dioxus runtime's handler result, without changing what the telemetry sink prints.
@task_item n=5 part=needs
The new field crosses every ratified carrier the acting verbs ride: the in-process answer, the session socket's call and its ok reply, the command line's answer line, the MCP tool result, and the kept check recorded and replayed.
@task_item n=6 part=needs
The kept check carries the field as its answer does, or holds it out of the digest (a design point for P4); its masking, size bounds, strict read, never-logged rule and replay through the one run path stay in force, and replay's first difference reads it as it reads other answer fields.
@task_item n=7 part=needs
The driver's self-description (help) and the MCP tool listing describe the new field; one self-description serves the command line and MCP, and the capability statement stays held to the driver schema.
@task_item n=8 part=needs
What of a log may enter an answer is the founder's boundary fork: authored text (a panic payload, a log message, a log field's value) or an open value set crossing onto a carrier widens the boundary, a new field of a closed value set inside a ratified carrier does not; the fork goes to founder-forks.md with its options whole.
@task_item n=9 part=needs
The founder's log rulings bound the field: content-free log fields, no screen or snapshot content in logs, telemetry printing closed spans, and the scrub sets of escher-telemetry.
@task_item n=10 part=needs
Scrub held: escher-telemetry's target allowlist and scrub of url, href, src, html, text, value, attrs, path, request and error on escher targets stay as they are, and its panic hook keeps chaining.
@task_item n=11 part=needs
The driver's command span and the MCP tool_call span gain no field holding what an answer now carries; the driver opens spans and emits no event, and no log line carries the new answer content or diff content.
@task_item n=12 part=needs
No new carrier, listener, socket, port, verb, exit status or session error; a new refusal cause or result field only as the schema tables admit, the refusal causes staying closed and the validated verb set unwidened.
@task_item n=13 part=needs
No panic path on a driver call's arguments; calls run only after validation, and a panic caught here is the app's handler's, not the driver's.
@task_item n=14 part=needs
The answer is written by the driver's own hand-written JSON writer with its string escaping; no serde_json type enters the driver's public API.
@task_item n=15 part=needs
The bounded answer stays inside the session wire's reply bound, the session client's call answer read bound and the MCP result bound, answer-too-large being the existing session error past them.
@task_item n=16 part=needs
The layout read, explore and Session::reach keep their contracts; explore's per-path panic ending is not widened to carry a payload, exploration staying off the carriers.
@task_item n=17 part=needs
The handler identity comes from the UI graph's owner and the reached record written before dispatch and before handlers run, as the Dioxus bridge and handler coverage reader read it today.
@task_item n=18 part=needs
The proof needs a minimal fixture app beside the seven_guis stand, since no stand task panics or logs from a handler; the stand itself boots offline and its readings stay as they are.
@task_item n=19 part=needs
A new blitz-tests target proves the behaviour on each carrier, in both layout modes.
@task_standing n=1
convention: integration tests live in blitz-tests, one test target per file, one behaviour per integration test file, shared fixtures stated once
home: conventions · design-philosophy
@task_standing n=2
convention: every scenario runs in both layout modes
home: conventions
@task_standing n=3
convention: tests are descriptively named, assert the fixture's condition first, and use numbered cases in a reduced fixture where the stand lacks a case
home: conventions
@task_standing n=4
convention: modules and public items are documented, constants carry their rationale, and rustdoc warnings are errors
home: conventions
@task_standing n=5
convention: expect and unwrap carry a justification, errors are enums and fallible APIs return typed results
home: conventions · cross-cutting-patterns
@task_standing n=6
convention: driver unit tests sit in path children of their module, at the end of the source file
home: existing-scopes · conventions
@task_standing n=7
convention: each crate the chunk changes holds its line coverage floor
home: standard-contracts
@task_item n=1 part=not_needed
Style, layout and paint engine rows (Stylo, Taffy, Parley, anyrender, damage, text decoration): the chunk changes no engine behaviour.
@task_item n=2 part=not_needed
The page JavaScript runtime (Boa, script globals, script fetcher, its uncaught errors): the errors this chunk returns are a Dioxus app handler's, not page script's.
@task_item n=3 part=not_needed
The select list, form controls, range input and their default actions and keyboard models: no form-control behaviour changes.
@task_item n=4 part=not_needed
The accessibility tree, AccessKit and its platform adapter: the chunk reads no accessibility state.
@task_item n=5 part=not_needed
The WPT runner, its suites, results, outputs and diff-to-PR tooling: the chunk touches no WPT surface.
@task_item n=6 part=not_needed
The browser app, its history, persistence, about pages, toolbar and favicon cache: the chunk does not reach the browser.
@task_item n=7 part=not_needed
Networking and fetching (blitz-net, net providers, HTTP cache, per-host caps, URL schemes): no load behaviour changes.
@task_item n=8 part=not_needed
Upstream sync, cherry-pick and dependency-bump rows, including the Taffy take past #1372: this chunk takes nothing from another tree.
@task_item n=9 part=not_needed
The cold-agent run, its verdict fields, reasons and model provider: the chunk adds no cold-agent measure.
@task_item n=10 part=not_needed
The lint verb, the screenshot verb and the snapshot text and node keys: their answers are not where the new field lands.
@task_item n=11 part=not_needed
Windowed shell, winit, redraw, IME, platform quirks, wasm and Android entry rows: the behaviour is proved headless.
