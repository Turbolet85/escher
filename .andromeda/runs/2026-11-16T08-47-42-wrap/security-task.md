@task_prose part=session
The five acting verbs (click, type, press, advance, scroll) gain a result field caused, written only when non-empty: one item per kind and level, with the closed kind words panic, error and log, the five closed level words, the reached handler's stable id, its owner id, its event name and a count. It holds no message, no payload, no target and no field value.
escher-telemetry gains a scoped log capture (CaptureLayer, capture, Captured) that counts records by level and prints nothing. The sink subscriber is now the registry, plus the sink layer filtered by the RUST_LOG EnvFilter, plus the capture layer, which is unfiltered. When the default holds no capture layer, the capture uses its own process-wide pair of dispatchers, built once and never dropped. The twin is never used and is there to keep tracing-core out of its one-dispatcher mode.
The capture counts Dioxus framework targets (dioxus, generational_box, warnings) only at warn and error, which was the operator's ruling. It skips the engine and escher_ prefixes and counts every other outside target at every level.
An acting step now runs inside one capture under catch_unwind, so a handler panic is caught, its payload is never read, and the session keeps answering. The step reads the root error boundary's mark and drains a per-step reached-handler list. Exploration marks a path as panicked when an accepted answer carries a panic item, and the kept check now holds the panicked step.
The founder's own word (F1) settled caused as built, with closed words only, so it is no longer provisional. Text for every source is left to a follow-up. A handler Err that the root boundary takes reaches the screen as the boundary's rendering of its Display, which is an existing screen-content crossing and never part of caused.
The span, the sink output, the panic hook, the verbs, the refusal causes, the session errors, the exits and the environment reads are unchanged. The only new edge is escher-driver to escher-telemetry, and no registry crate was added.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: new; escher's telemetry crate's scoped log capture, which counts records by level with no message, field or target held, has a framework-target set at warn and above, and a process-wide pair of scoped dispatchers (after implement, a comment beside the never-used twin)
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited; escher's telemetry crate root, which holds the stderr allowlist sink's subscriber install (init_with_writer), the RUST_LOG EnvFilter now on the sink layer alone with a capture layer beside it, and the re-exports of the capture (after implement, a crate doc link fixed)
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: new; the Dioxus native document's reader of the root error boundary mark (root_error_mark, RootErrorMark), which holds no error text
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the Dioxus native bridge crate root, which declares and re-exports the boundary module
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited; the reached-handler record, which gains a per-step list drained by take_step_reached, each handler once and in first-reached order
@task_file n=6 path=packages/escher-driver/src/caused.rs
what: new; the driver's caused item (Caused, CausedKind, CausedLevel), which turns a step reading into the closed words, ids and counts an acting answer carries
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: edited; the driver's command execution and settled step, which now runs each acting step inside one capture under catch_unwind and puts caused on Acted, Advanced and Scrolled
@task_file n=8 path=packages/escher-driver/src/session.rs
what: edited; the driver session, whose start now turns on the reached-handler record on every held session
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited; the closed driver verb schema and its field table, which gains the CAUSED field last on the five acting verbs, with its stated limits in its help
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: edited; the verb schema's field and field-kind tables under test
@task_file n=11 path=packages/escher-driver/src/json.rs
what: edited; the driver's written answer form, which writes caused after a verb's other fields only when non-empty and leaves out absent id, owner and event
@task_file n=12 path=packages/escher-driver/src/explore.rs
what: edited; the exploration run, which reads a panicked path from an answer's panic item and has its kept check hold the panicked step
@task_file n=13 path=packages/escher-driver/src/fixture_tests.rs
what: edited; fixture state tests matching the outcome's new field
@task_file n=14 path=packages/escher-driver/src/recording_tests.rs
what: edited; recorded check tests matching the outcome's new field
@task_file n=15 path=packages/escher-driver/src/replay.rs
what: edited; the kept check replay, which compares each step's answer and now includes caused
@task_file n=16 path=packages/escher-driver/src/lib.rs
what: edited; the driver crate root, which declares the private caused module and re-exports its types
@task_file n=17 path=packages/escher-driver/Cargo.toml
what: edited; the driver crate's manifest, which gains the workspace-inherited escher-telemetry dependency
@task_file n=18 path=Cargo.lock
what: edited; the workspace lockfile, which gains one path-crate dependency line and adds or bumps no registry crate
@task_file n=19 path=tests/blitz-tests/tests/stand_act_errors.rs
what: new; the stand check of caused on panic, Err, logging click, type and press in both layout modes, in process, over serve_mcp and through a kept check, with sentinel checks and the command span check around a panicking call (after implement, it opens the capture before installing its recorder)
@task_file n=20 path=tests/blitz-tests/tests/stand_explore.rs
what: edited; the exploration stand check, which covers a panicked path's ending and its kept check
@task_file n=21 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; shared stand-check session helpers matching the outcome's new field
@task_file n=22 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited; the command span stand check, matching the outcome's new field
@task_item n=1 part=touches
Acting answers on every ratified carrier (session socket, command line and MCP tool result) now carry caused. This touches the crossing decisions over what an answer carries, such as the MCP result crossing, the socket carrying what the screen reads, and the driver session socket reply contracts, which gain a closed-content member.
@task_item n=2 part=touches
The stable id and owner id now also leave through caused items, alongside the event name. This touches the ratified exits of the stable id, the stable element id carriers contract, and the actions answer id and owner contracts that caused reuses.
@task_item n=3 part=touches
caused's kind and level are closed-value fields in a ratified carrier. This touches the boundary widening rule and the rule that a closed-value field is no widening.
@task_item n=4 part=touches
The driver installs no subscriber, and the driver installs no subscriber and reads no env. During an acting step of a session, when the default holds no capture layer, escher-telemetry now sets its own scoped capture dispatcher as the default. That dispatcher prints nothing and no env read was added, but it is an exception these rows do not state.
@task_item n=5 part=touches
RUST_LOG, the driver reading only RUST_LOG, and the environment variables RUST_LOG contract: the EnvFilter now filters the sink layer alone and the capture layer beside it is unfiltered. No env read was added.
@task_item n=6 part=touches
The log sink allowlists targets and drops other and outside targets whole, and so does the telemetry sink target allowlist and escher's allowlist telemetry sink. The sink's output is unchanged, but outside and framework targets the sink drops are now counted by level beside it, holding nothing of the record. This is indirect, through the sink's subscriber.
@task_item n=7 part=touches
The command span holds no content, there is one command span per call, and the driver command span contract keeps its eight fields. These still hold around a panicking call and caused is not on the span, as a new stand check asserts. This is indirect, through the run path that now wraps the step.
@task_item n=8 part=touches
Driver outcomes are never logged, and session output holds no caller input. The Outcome variants Acted, Advanced and Scrolled gain caused, which still is not logged and holds no caller input. A refused call carries no caused.
@task_item n=9 part=touches
The reached-handler record without log or NodeId, and the reached record deduplicated and kept past removal, gain a per-step list drained by take_step_reached. Dispatch was recorded only where a signal registry stands, and the record is now turned on for every held session at start.
@task_item n=10 part=touches
The closed driver verb schema and the single validation gate gain the CAUSED field spec, of kind Entries and not always present. No verb, argument kind, refusal cause, session error or exit was added. The MCP listing, which is fixed program text, carries the new field's help from the table.
@task_item n=11 part=touches
The kept check keeps each answer whole, and the kept check file contents and recorded check answer contracts now hold caused. Replay compares it, and a flipped count stops at its step naming caused.
@task_item n=12 part=touches
The exploration report holds closed content only. A path that panics now ends Panicked, read from the answer's panic item with no payload, and its kept check holds the panicked step.
@task_item n=13 part=touches
No panic on call-supplied input, and stale node ids never panic. A panic in an app handler during an acting step is now caught and the session answers. A second panic in the settle reads busy render. This is indirect, through the run path.
@task_item n=14 part=touches
Screen content stays out of logs, and there is the screen-content crossing. A handler Err taken by the app's root error boundary now reaches the screen and the step diff as Dioxus's default boundary rendering of its Display, which is an existing screen-content crossing that the operator ruled stands outside caused.
@task_item n=15 part=touches
An error reporting integration remains absent. Answers now report panic, error and log counts per handler in band to the caller, but nothing is exported.
@task_item n=16 part=touches
Workspace-inherited dependency versions and locked builds: escher-driver takes escher-telemetry with workspace = true, and the lockfile gains one path-crate line. No registry crate was added or bumped.
@task_item n=17 part=touches
Contracts that hold a size bound on an answer, the MCP tool result size bound and the driver command line answer bounds, are reached because acting answers grow by the caused member. The bounds themselves did not move.
@task_fact n=1 slug=threat-model-summary
fact: The answer to every acting verb (click, type, press, advance, scroll) carries caused on every ratified carrier, and only when the step caused something. It holds one item per kind and level, with the closed kind words panic, error and log and the five closed level words, the first reached handler's stable id, its owner id and event name, and a count. It holds no message, payload, target or field value. This stands on the founder's own word (F1), a narrower crossing than his word allows, and text is left to a follow-up.
@task_fact n=2 slug=threat-model-summary
fact: A handler Err taken by the app's root error boundary crosses only as screen content, through Dioxus's default boundary rendering of its Display, and never through caused. caused reports that Err as one error item, with no byte of its text.
@task_fact n=3 slug=error-handling
fact: The limits of caused: it ties records to the first handler the step reached, an error that an app's own error boundary catches gives no item, and records sent through the log crate are not counted.
@task_fact n=4 slug=error-handling
fact: A panic in an app handler during an acting step is caught, and its payload is never read. The answer carries a panic item, the session keeps answering, and a panic in the settle runs the settle once more, after which a second panic reads busy render.
@task_fact n=5 slug=logging-monitoring
fact: escher-telemetry's scoped log capture counts records by level and prints nothing, holding no message, field or target. It counts the Dioxus framework targets (dioxus, generational_box, warnings) only at warn and error, never counts the engine and escher_ prefixes, and counts every other outside target at every level.
@task_fact n=6 slug=logging-monitoring
fact: A convention with an exception: the driver and the headless stand install no subscriber, except that during an acting step escher-telemetry sets a scoped capture dispatcher as the thread's default when the default holds no capture layer. It is one process-wide pair, built once and never dropped, with a twin that is never used, and it prints nothing.
@task_item n=1 part=untouched
Outbound networking and fetching: script and sub-resource fetches, CORS and same-origin absences, TLS, cookies, the HTTP cache, request timeouts and response size limits. The session reached no network code.
@task_item n=2 part=untouched
Authentication and authorization, CI tokens, publish and signing secrets, and the release jobs. The session reached none of them.
@task_item n=3 part=untouched
The dependency audit, the licence gate, engine dependency pins, CI actions pinned to SHAs, and automated updates. No registry crate and no CI workflow moved.
@task_item n=4 part=untouched
The session socket's bounds, owner-only permissions and single listener, the MCP stdio transport, protocol errors and idle expiry, and the recording and state directories. Their transport, framing and file handling are unchanged.
@task_item n=5 part=untouched
Form-control rows: select lists, range inputs, password and file value masking, the accessibility adapter and accessible names, and the snapshot state words. None of their readers moved.
@task_item n=6 part=untouched
HTML and CSS parsing, stylesheets, fonts, iframe and import nesting limits, serialization, and navigation schemes. These are engine parsing and loading paths the session did not reach.
@task_item n=7 part=untouched
The cold-agent and agent-run harnesses, the apt install script, the WPT runner and its workflows, and the coverage floor checker. No harness or gate script changed.
@task_item n=8 part=untouched
Data at rest: the browser history in sqlite, database encryption, the clipboard and file dialogs.
@task_item n=9 part=untouched
The escher-session command line flags, roles and exit codes. No flag, role or exit was added.
