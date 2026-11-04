@task_prose part=chunk
Builds a probe of the four links of the UI graph (node to component, node to handlers, handler to state written, state to nodes reading it), each read through what Dioxus 0.7.3 and escher's bridge dioxus-native-dom already expose, with no Dioxus fork and no engine change for every document.
Each link gets one of three verdicts (exposed, exposed with a cost, not exposed without a fork-level change), measured by a run on the headless stand (a lean task with handler-written state, Counter or CRUD, plus one served task) in both layout modes where layout matters; the probe is test or example code that measures.
Answers whether a node's click listener can be read from the listener map today, as link 2's answer for click.
Re-verifies at HEAD the MountedData hazard: a task spawned from a key handler, first polled inside render_immediate in dioxus_document.rs poll, panics RefCell already borrowed at events.rs:162. It names the hazard and brings its fix to the founder as a fork with options, not decided here.
Writes one plain-words reading for the founder, with each verdict, its evidence and the cost class, before The UI graph is planned; it adds no verb, field, span, log field, listener, env var or crate.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=re-reads
what: the MountedData panic coordinate events.rs:162, re-verified at HEAD after the upstream sync
@task_file n=2 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=re-reads
what: poll, where render_immediate holds the document RefCell while the spawned task is first polled
@task_file n=3 path=.andromeda/cache/chunks/2026-10-31-engine-debts/evidence/readings.md mark=re-reads
what: the hazard measured beside step 15 (headless through the harness poll(None); the windowed path read, not run)
@task_item n=1 part=needs
The new probe check under tests/blitz-tests: it must install no subscriber, read no env var, print nothing and bind no listener. Its readings go into assertions only, and the silence census reads 0 over its file.
@task_item n=2 part=needs
Values masked, never logged: nothing the probe reads (a component name, a handler or event name, a signal's value, node text) is printed, logged, put in a log field or span field, or shown through a Debug print. This is the PII scrubbing rule, applied to the probe's own code.
@task_item n=3 part=needs
The headless stand seven_guis::stand (boot, and boot_cells or boot_load for the served task) installs no escher sink. A probe booting it in process has no sink, so a verdict's evidence comes from assertions and never from captured log lines.
@task_item n=4 part=needs
Engine features by invocation: a workspace build turns blitz-dom/tracing on, and cargo test -p blitz-tests leaves it off. The probe must give the same verdicts under both builds and must not depend on engine tracing events, which dioxus-native-dom's mutation_writer debug logs fill with text-node contents and attribute values.
@task_item n=5 part=needs
dioxus-native-dom as a read surface (DioxusDocument, its element-id and listener reads, poll, events.rs): the probe reads them as they are and adds no log site, tracing call, span or field there. The crate's optional tracing and log-times features stay as they are.
@task_item n=6 part=needs
escher-driver's spans stay untouched: the command span with its eight fields and the tool_call span with tool and ending. The probe adds no span or field, so escher-driver keeps its 2 info_span! sites and stand_act_spans its eight fields.
@task_item n=7 part=needs
The MountedData panic as error capture: in a binary that installs escher's sink (seven_guis_native, escher-session), the chaining panic hook logs one ERROR event at escher_telemetry::panic with panic.payload redacted, and std's default hook still prints the raw message. The reading states the hazard's mechanism without the probe adding any capture, hook or log line.
@task_item n=8 part=needs
The windowed shell runs the same poll lines, so the panic is expected there, but this was read from the code and not run. The reading must keep the measured-or-read mark for each claim: a verdict read from the code and not run says so.
@task_item n=9 part=needs
The reading for the founder is a document at rest. It names APIs, verdicts, costs and forks, and holds no probed run values (component names, handler names or signal values from a run). It is not a log line, field or print.
@task_item n=10 part=needs
host_log and mcp_log still pass under both builds with 0 needles at trace. The probe touches neither the sink nor the driver, so packages/escher-telemetry and packages/escher-driver stay unchanged.
@task_item n=11 part=needs
Any probe code kept under examples/seven_guis holds no print, tracing call, escher_telemetry use, env read, socket, listener or blitz_net. The served-stand silence census still reads 0 over stand.rs and the shared modules.
@task_standing n=1
convention: each new stand or engine check installs no subscriber, reads no env var, prints nothing and binds no listener, its readings going into assertions only, the silence census reading 0 over its file
home: log-coverage
@task_standing n=2
convention: a new reader of a stand check shared module (tests/common/mod.rs, session_common/mod.rs, fixtures/mod.rs) is counted among that module's readers, and the census over the module still reads 0
home: section 3 Observability Harness Contract (Logging stack)
@task_standing n=3
convention: a change adds no log site, span, field, log field or env read, escher-driver keeping its 2 info_span! sites and stand_act_spans its eight fields
home: span-trace-coverage
@task_standing n=4
convention: a value read from a page or app (ids, names, control values, text) or a type deriving Debug that holds them is never logged, fielded or printed
home: pii-scrubbing-compliance
@task_standing n=5
convention: a check's failure or assertion message names a kind and a count, never a needle or a captured line
home: log-coverage
@task_item n=1 part=not_needed
Metric coverage: frame and phase timings, debug_timer, blitz-paint layer counters and WPT figures. The probe adds no timing or counter.
@task_item n=2 part=not_needed
CI telemetry artifact handling: the merged log and coverage artifacts, upstream WPT score publishing, publish builds at CARGO_LOG info, and the apt-install diagnostic line.
@task_item n=3 part=not_needed
apps/browser emitters and the values they log as-is (visited URLs, urlbar text, the document loader's JS error drain), plus the in-app FPS overlay and devtools overlays.
@task_item n=4 part=not_needed
The blitz-vibey-script JS error surface: take_js_errors, take_messages, the 256-error retention, window.onerror, the error event fields and document_may_load's Forbidden refusal.
@task_item n=5 part=not_needed
The wpt runner: its env_logger logging, its panic-backtrace hook, the crashed-test message and the pump_net_provider timeout log.
@task_item n=6 part=not_needed
The agent-run harness log and the cold-agent pipe log: their JSON event shapes, verdict fields, server.log span counting, transcript and record directories.
@task_item n=7 part=not_needed
The upstream apps' fmt::init stdout subscribers, wasm_hello's tracing_wasm, and console_error_panic_hook in WASM builds. The probe runs natively and headlessly.
@task_item n=8 part=not_needed
The engine log sites that carry content: blitz-dom resource URLs, debug_log_node, pointer href logs, the SVG outer HTML field, image-fetch URLs, the invalid CSS value warning, blitz-net request URLs and fetch errors, blitz-shell's data URI errors.
@task_item n=9 part=not_needed
escher-driver's MCP and recording surfaces: serve_mcp replies, the recording and replay modules, kept checks at rest, and the self-description. The probe adds no verb, tool or answer field.
@task_item n=10 part=not_needed
The earlier chunks' silence records over their own files (visual lint, screenshot image module, select list, load wait, parts, session recording, clock, upstream takes). The probe changes none of those files.
@task_item n=11 part=not_needed
The service identity and line format of escher's sink, and the RUST_LOG EnvFilter default. The probe installs no sink and reads no env var.
