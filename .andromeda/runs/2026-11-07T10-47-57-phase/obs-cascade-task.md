@task_prose part=chunk
Ports upstream 286874cc1's buffered render onto the Dioxus 0.7 bridge in dioxus-native-dom: DioxusDocument's two render sites, initial_build (vdom.rebuild) and poll (vdom.render_immediate), run the vdom into an in-process buffer of 0.7 WriteMutations calls with the document unborrowed, then take the document's borrow once, replay the buffer through the existing MutationWriter and flush the queued mounted events as before.
A task polled during render can then call MountedData (set_focus, get_client_rect, scroll_to and the rest) with no RefCell already borrowed panic. The change reaches every DioxusDocument: the headless stand, the windowed dioxus-native launch and any app on the bridge.
Ports upstream's regression test task_polled_during_render_can_access_document as the witness, and inverts dioxus_runtime_probe's two should_panic hazard pins into tests that focus moves and nothing panics, in both layout modes.
Re-asserts that behaviour is otherwise unchanged, using the existing stand_*, stale_node_mapping, incremental_oracle and the probe's other eight tests as the witness. Dioxus stays at 0.7, and nothing crosses: no log field, span, env var or carrier.
@task_file n=1 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=edits
what: the initial_build and poll render sites, and the ported test in its mod tests
@task_file n=2 path=packages/dioxus-native-dom/src/mutation_buffer.rs
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs mark=edits
what: the module list
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_writer.rs mark=edits
what: DioxusState, if the buffer lives on it; otherwise kept byte-identical to upstream at 434e0661
@task_file n=5 path=tests/blitz-tests/tests/dioxus_runtime_probe.rs mark=edits
what: the two should_panic pins at :581 and :587 inverted, and its module text re-said
@task_item n=1 part=needs
dioxus-native-dom as an emitter: its log calls compile only behind the optional tracing feature, which dioxus-native's tracing feature turns on. Any log site the port adds or moves in the render path has to stay behind that feature, with a no-op path when it is off.
@task_item n=2 part=needs
The mutation buffer holds ids, tag names, attribute names and values, and text node contents. These are page content, and they must never be printed, logged or put in a field. The buffer gets no Debug output that reaches a log line.
@task_item n=3 part=needs
mutation_writer.rs's existing debug log sites record text node contents and attribute values as they are. The replay goes through the same MutationWriter, so those sites fire on the replay exactly as they did on the direct write. None is added, moved or widened, and the file stays byte-identical where the port allows.
@task_item n=4 part=needs
The PII rule for engine targets: dioxus-native-dom's events reach escher's sink only when the engine tracing feature is on. There they are judged by the engine target allowlist, with only the safe fields printed. The port adds no new field name that the scrub would have to judge.
@task_item n=5 part=needs
The flush of queued mounted events (flush_queued_mounted_events) keeps its order and its silence. The two callers change shape, but they add no log site, print or env read.
@task_item n=6 part=needs
The ported witness test task_polled_during_render_can_access_document, inside dioxus-native-dom's own mod tests: it installs no subscriber, reads no env var and prints nothing. Its focus and client rect readings go into assertions only.
@task_item n=7 part=needs
dioxus_runtime_probe in tests/blitz-tests: the two inverted tests and the other eight stay silent, with the silence census (print, env read, subscriber, escher_telemetry, listener) reading 0 over the file. A failure message names the layout mode and the outcome, never captured page text.
@task_item n=8 part=needs
The invariant that nothing crosses: no verb, argument, node key, result field, cause, session error, exit, carrier, listener, port, env var, workspace crate, log field or span is added. The silence and census readings the plan records over the touched files and the stand checks stay as they are.
@task_item n=9 part=needs
The sink checks host_log and mcp_log: the render path change reaches the escher-session host and its mcp role, so both still pass under both builds and read 0 id, name and typed-text needles at trace. escher-driver keeps its 2 info_span sites, and stand_act_spans keeps its eight fields.
@task_item n=10 part=needs
The engine features by runner: a workspace build turns blitz-dom tracing on, while a -p blitz-tests or -p seven_guis build leaves it off. The new and inverted tests run silent under both.
@task_item n=11 part=needs
The panic surface: the hazard that is removed was a RefCell already borrowed panic, and in seven_guis_native and escher-session a panic reaches the chaining escher-telemetry panic hook. The port adds no panic path, and no new failure path, in the render sites.
@task_standing n=1
convention: each new or rewritten check installs no subscriber, reads no env var and prints nothing, its readings going into assertions only, and the silence census reads 0 over its file
home: log-coverage
@task_standing n=2
convention: each engine-crate log site is compiled only behind the crate's tracing cargo feature, with a no-op path when the feature is off
home: telemetry-strategy
@task_standing n=3
convention: page text, attribute values and ids held in process are never logged, fielded or printed with Debug
home: pii-scrubbing-compliance
@task_item n=1 part=not_needed
The agent-run harness log and its events.jsonl and run.log: the chunk leaves scripts/agent-run.sh and its event shapes untouched.
@task_item n=2 part=not_needed
The cold-agent pipe log, its verdict fields, server.log span counting, transcript and record directory: the chunk does not touch the cold-agent pipe.
@task_item n=3 part=not_needed
CI artifact handling, the publish builds' CARGO_LOG, apt-install diagnostics and the WPT score publishing: the chunk adds no CI job and no artifact.
@task_item n=4 part=not_needed
Metric coverage: frame and phase timings, debug_timer, blitz-paint layer counters, the FPS overlay and the bench and screenshot timings. The chunk adds no timing or counter.
@task_item n=5 part=not_needed
The apps/browser emitter: document load, urlbar and history log lines, and its tracing_subscriber fmt init on stdout.
@task_item n=6 part=not_needed
The wpt runner: env_logger logging, its panic backtrace hook, report message fields and pump_net_provider's timeout log.
@task_item n=7 part=not_needed
The blitz-vibey-script JS error surface: the window error event, onerror, take_js_errors and take_messages draining, the 256-error cap and document_may_load's Forbidden refusal.
@task_item n=8 part=not_needed
blitz-net and blitz-dom error capture: fetch error logging, NetHandler::error, Resource load failed, stylesheet parsing without an error reporter and SVG parse warnings. The chunk changes no network or style path.
@task_item n=9 part=not_needed
escher-driver's recording and replay modules, the kept checks at rest and the session state directory: the chunk changes no driver module.
@task_item n=10 part=not_needed
The upstream example apps' subscriber setup (todomvc, readme, wasm_hello with tracing_wasm) and the WASM console_error_panic_hook installs.
