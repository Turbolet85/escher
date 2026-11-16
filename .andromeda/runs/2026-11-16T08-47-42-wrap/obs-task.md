@task_prose part=session
The five acting verbs (click, type, press, advance, scroll) now answer what their step caused: a new caused field, present only when non-empty, one item per kind (panic, error, log) and level, tied to the handler's id, owner and event, with a count and no message, payload or error text.
escher-telemetry gained a scoped capture that counts tracing records by level during a closure and prints nothing; the sink's EnvFilter now filters the sink layer alone, the capture layer stands unfiltered beside it.
Where the default subscriber holds no capture layer, the capture runs on a process-wide pair of dispatchers kept for the life of the process, one never used, so tracing-core never falls into its one-dispatcher interest mode; a per-thread form was tried and retired.
Dioxus framework targets (dioxus*, generational_box, warnings) count only at warn and error, engine and escher_ targets not at all, every other outside target at every level; an Err the root default boundary takes renders its Display on screen and crosses only as screen content, never in caused.
A handler or settle panic in an acting step is caught and the session keeps answering; exploration reads an accepted answer carrying a panic item as a panicked path, its kept check now holding the panicked step.
The driver's command span keeps its eight fields around a panicking call and holds nothing of caused; escher-driver now depends on escher-telemetry; the caused crossing stands on the founder's word, no longer provisional.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: added, then edited after implement — escher-telemetry's scoped log capture: the capture layer, the captured per-level counts, the counted target sets and the process-wide capture dispatcher pair
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited — escher-telemetry's crate root and sink install: the subscriber built of the sink layer under the RUST_LOG EnvFilter beside the unfiltered capture layer, and the crate doc
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: added — the document's read of the root error boundary: the root error mark an acting step compares before and after
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited — dioxus-native-dom's module list and re-exports
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited — the reached record of handlers, now with a per-step list drained after each step
@task_file n=6 path=packages/escher-driver/Cargo.toml
what: edited — escher-driver's dependencies, now taking escher-telemetry
@task_file n=7 path=Cargo.lock
what: edited — the workspace lock file, one dependency line for escher-driver's new edge
@task_file n=8 path=packages/escher-driver/src/caused.rs
what: added — the caused item of an acting answer: its closed kind and level words and how a step's reading becomes items
@task_file n=9 path=packages/escher-driver/src/execute.rs
what: edited — the driver's command run: the settled acting step under capture and panic catch, the command span around it, and the acting outcomes carrying caused
@task_file n=10 path=packages/escher-driver/src/session.rs
what: edited — the driver session's start, which now turns the reached record on for every held session
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: edited — the verb table, which gains the caused result field on the five acting verbs with its help
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: edited — the verb table's checks: each verb's result fields and the field-kinds list
@task_file n=13 path=packages/escher-driver/src/json.rs
what: edited — the command answer's written JSON form, which writes caused last and only when non-empty
@task_file n=14 path=packages/escher-driver/src/explore.rs
what: edited — the driver's auto-exploration, which catches a panicking call and ends its path Panicked
@task_file n=15 path=packages/escher-driver/src/fixture_tests.rs
what: edited — the driver's fixture checks, matching the acting outcomes' new field
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: edited — the session recording's checks, matching the acting outcomes' new field
@task_file n=17 path=packages/escher-driver/src/replay.rs
what: edited — the kept check's replay, which compares answers including caused
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: edited — escher-driver's crate root: the caused module and its re-exports
@task_file n=19 path=tests/blitz-tests/tests/stand_act_errors.rs
what: added, then edited after implement — the stand check of caused: panic, Err, logging click, type and press in both layout modes, over serve_mcp and a kept check, and the command span's eight fields around a panicking call
@task_file n=20 path=tests/blitz-tests/tests/stand_explore.rs
what: edited — the stand check of auto-exploration, now reading a panicked path's kept check holding the panicked step
@task_file n=21 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the shared session helpers of the stand checks, matching the acting outcomes' new field
@task_file n=22 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the stand check of the command span's fields on acting calls, matching the acting outcomes' new field
@task_item n=1 part=touches
decision/driver-library-installs-no-subscriber: the driver now sets escher-telemetry's scoped capture as the default during each acting step of an in-process session (a process-wide dispatcher pair where the default holds no capture layer); it prints nothing.
@task_item n=2 part=touches
decision/headless-stand-no-sink: a headless stand still installs no sink, but each acting step now runs under the scoped capture default, which counts and prints nothing.
@task_item n=3 part=touches
decision/driver-emits-no-event: the driver still emits no event, but it now sets a scoped capture subscriber during an acting step; the never-installs-a-subscriber half reads against that.
@task_item n=4 part=touches
contract/telemetry-sink/rust-log and contract/escher-sink-env/rust-log: the RUST_LOG EnvFilter now filters the sink layer alone; a capture layer stands unfiltered beside it in the same subscriber; the sink's output is unchanged.
@task_item n=5 part=touches
contract/escher-telemetry-events/telemetry-installed and decision/sink-installed-first-in-main: the installed subscriber is now the registry with the filtered sink layer plus the capture layer; the install order, the LogTracer level and the panic hook are unchanged.
@task_item n=6 part=touches
decision/exploration-catches-panics: exploration now also reads an accepted answer carrying a panic item as panicked (path Panicked, a re-reach step Diverged), and a panicked path's kept check now holds the panicked step.
@task_item n=7 part=touches
decision/hosted-call-through-session-run: the acting step inside the session's run now runs under catch_unwind and capture; a panic in the settle runs it once more and a second reads busy render.
@task_item n=8 part=touches
contract/command-answer/graphed, contract/escher-session-stdout/command-answer and contract/mcp-role-stdout/json-rpc: an acting verb's answer gains a caused member, last and only when non-empty, each item kind, level, id, owner, event, count, with id, owner and event left out when absent.
@task_item n=9 part=touches
contract/kept-check/answer, contract/kept-check/answer-digest and contract/recording-file/answer: recorded and kept answers now carry caused when non-empty; a replay compares it and a flipped count stops naming caused.
@task_item n=10 part=touches
contract/command-span/field-count, decision/command-span-no-call-content and decision/one-span-per-command: the command span keeps its eight fields around a panicking call and holds nothing of caused; a new check asserts it.
@task_item n=11 part=touches
decision/driver-spans-carry-no-content and decision/session-logs-carry-no-page-content: caused adds no log site, field or print; it crosses only in the answer.
@task_item n=12 part=touches
decision/reached-record-ids-and-events-only: the reached record gains a per-step list drained after each step, still ids and event names only; every held session now turns the record on at start.
@task_item n=13 part=touches
contract/coverage-line/content and contract/coverage-line/owner: the handler coverage line reads the reached record, which every held session now keeps on from its start.
@task_item n=14 part=touches
decision/sink-target-allowlist and decision/engine-target-safe-fields: the capture keeps its own target sets beside the sink's — engine and escher_ targets not counted, Dioxus framework targets (dioxus*, generational_box, warnings) at warn and error only, every other target at every level.
@task_item n=15 part=touches
decision/debug-types-never-logged: the new answer-bearing types' Debug forms print counts alone (Captured) or the name alone (RootErrorMark).
@task_item n=16 part=touches
decision/scrub-at-sink-not-call-sites, contract/telemetry-scrub-set/error and decision/escher-target-content-named-set: caused carries no message, payload or Err text, so no scrub reaches it; the sink's scrub is unchanged and sentinels read 0 in caused.
@task_item n=17 part=touches
decision/answers-echo-nothing-typed and decision/part-answer-never-logged: caused holds closed words, ids and counts only, never typed text, and is never logged.
@task_item n=18 part=touches
absence/error-reporting-service: acting answers now carry an in-process count of the panics, errors and logs a step caused; no error-reporting client or service was added.
@task_item n=19 part=touches
decision/escher-crates-tracing-ungated and decision/escher-crates-always-trace: escher-driver gains a crate edge to escher-telemetry, so escher-telemetry's capture now compiles into every escher-driver build.
@task_item n=20 part=touches
contract/escher-telemetry-panic-hook/previous-hook: a handler panic in an acting step is now caught by the driver after the hook runs; the hook and its chaining are unchanged.
@task_fact n=1 slug=3-observability-harness-contract
fact: During each acting step of an in-process session the driver runs the step under escher-telemetry's scoped capture, which counts tracing records per level and prints nothing; where the default subscriber holds no capture layer it uses a process-wide pair of dispatchers built once and never dropped, one never used, so tracing-core never takes callsite interest from one thread's default alone.
@task_fact n=2 slug=7-error-capture-reporting
fact: An acting answer (click, type, press, advance, scroll) carries caused only when its step caused something: one item per kind (panic, error, log) and level, with the first reached handler's id, owner and event and a count, and never a message, payload or error text; it sees only errors the root boundary takes and only tracing records.
@task_fact n=3 slug=7-error-capture-reporting
fact: A panic in an acting step's handler or settle is caught and the session keeps answering; the panic reads as a caused panic item, a second panic in the settle reads busy render.
@task_fact n=4 slug=6-log-coverage
fact: The capture counts the Dioxus framework targets (dioxus*, generational_box, warnings) only at warn and error, engine and escher_ targets not at all, and every other outside target at every level, because the framework traces on every re-render and signal write.
@task_fact n=5 slug=pii-scrubbing-compliance
fact: An Err the root default boundary takes reaches the screen as the boundary's rendering of its Display, an existing screen-content crossing, and never reaches caused; an Err an app's own boundary catches leaves no mark.
@task_fact n=6 slug=pii-scrubbing-compliance
fact: The caused crossing as built (ids, owner ids, event names, closed kind and level words, counts) stands on the founder's own word and is no longer provisional.
@task_item n=1 part=untouched
The cold-agent and agent-run harness rows (events, verdict fields and reasons, counted verbs, transcripts, cleanup): no harness script or verdict changed.
@task_item n=2 part=untouched
CI integration rows (GitHub workflows, artifacts, the package-install diagnostic, WPT publishing): no workflow or leg script changed; the new stand check joins the stand-repeat leg by its name prefix.
@task_item n=3 part=untouched
Metric coverage and the absences of metrics exporters, metric emission, OTel export and a telemetry backend: the capture's counts cross only in answers, nothing is exported.
@task_item n=4 part=untouched
The engine's script error surface, the window error event, window.onerror and the WPT report rows: no script or engine error path changed.
@task_item n=5 part=untouched
The tool_call span and its ending, and the refusal cause names: no verb, cause, session error, node key or exit was added.
@task_item n=6 part=untouched
escher-session's exit codes, error object, commands and stderr rows, and the host wire version: none moved.
@task_item n=7 part=untouched
Recording directory and session state privacy, recording bounds and session idle expiry: unchanged.
@task_item n=8 part=untouched
Select, optgroup, snapshot, open-list shot, state registry and runtime-probe privacy rows: no page content read changed.
@task_item n=9 part=untouched
The layout read rows and the graph and actions answer rows: the layout read, the UI graph and the actions answer are unchanged.
