@task_prose part=session
The acting verbs' caused answer now carries text: one item per panic, error or log record with its message, file, line and column, source (app, framework, engine) and target, the boundary (root or own) and cause chain of an error, the kept fields of a log, and the handler id, owner and event; the old closed-words shape (a count per kind and level) is retired.
A new caused_past field follows caused on click, type, press, advance and scroll, counting by kind and source what lies past the first 16 items; texts are cut at 1024 bytes, chains at 8 links, fields at 8, records at 16.
The test-scoped capture now keeps record text, admits the engine's warn and error records (never counted before), redacts any field named in the content-field scrub set (which gains password, 11 to 12 names), and installs once per process a chaining panic hook that keeps a panic's payload and location.
The document now registers every error boundary and exposes boundary marks; the driver reads every boundary's marks, no longer the root error mark alone; six engine warn and error sites now carry their values in fields under fixed messages, and the engine exposes whether tracing was compiled in.
Two new integration targets (the error-text founder check, joining the stand-repeat leg by its prefix, and the panic-hook check) were added; the root-boundary Err check's sentinel reading was inverted; after a red Windows CI run the MCP carrier comparison now decodes the tool result text whole through one shared session-check helper. CI went green 17 of 17.
Counts moved: escher-driver unit tests 121 to 122, escher-telemetry unit tests 21 to 25, the boundary unit tests 3 to 5, the schema field-kind table 30 to 31 rows; coverage floors held, 3 of 3 crates.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: new stand check (its own blitz-tests target, in the stand-repeat leg) that each failure answers what failed, where and from which source in both layout modes and both builds, that the root-boundary Err answers its chain on a session of its own, that a flood is cut, counted and bounded, that the same caused crosses serve_mcp in process and the session socket (host children), that a kept check replays its caused, and that no sentinel reaches the sink's lines at trace
  — edited
@task_file n=2 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: new check (its own blitz-tests target) that the capture's panic hook installs once and a panic outside a capture reads as before
  — edited
@task_file n=3 path=packages/escher-telemetry/src/capture.rs
what: the scoped test capture of telemetry records (the capture the driver opens per step) — its admitted targets by source and level, its scrub, its kept texts and bounds, and its process-wide chaining panic hook
  — edited
@task_file n=4 path=packages/escher-telemetry/src/lib.rs
what: the telemetry crate root — the sink's subscriber builder (now public, with a level filter) and the capture's exports
  — edited
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: the sink's line format and the content-field scrub set (CONTENT_FIELDS), redacted on the sink and the capture alike
  — edited
@task_file n=6 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the link click's warn, now a fixed message with href and url fields
  — edited
@task_file n=7 path=packages/blitz-dom/src/form.rs
what: the engine's form submission — the unimplemented-scheme warn, now a url field and a fixed message
  — edited
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: the engine's document mutator — the iframe load's unresolvable-src warn, now a src field and a fixed message
  — edited
@task_file n=9 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout — the text-node error's field renamed from data to text
  — edited
@task_file n=10 path=packages/blitz-dom/src/lib.rs
what: the engine crate root — the new TRACING_COMPILED constant telling whether the tracing feature is built in
  — edited
@task_file n=11 path=packages/dioxus-native/src/assets.rs
what: the native asset provider — its two warns now a request field with fixed messages
  — edited
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: the dioxus document's error boundaries — the root error mark and the new boundary registry, boundary marks (root or own) and the error a mark reads with its chain; its unit tests
  — edited
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the dioxus document — provides the boundary registry before the first build
  — edited
@task_file n=14 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root — exports the boundary types
  — edited
@task_file n=15 path=packages/escher-driver/src/caused.rs
what: the driver's caused answer — one item per panic, error and log record, its order, the 16 kept and the rest counted in caused_past
  — edited
@task_file n=16 path=packages/escher-driver/src/execute.rs
what: the driver's executor of the acting verbs — the settled step reads every boundary's marks and the acting results gain caused_past
  — edited
@task_file n=17 path=packages/escher-driver/src/json.rs
what: the driver's JSON answers — the writing of caused items and caused_past; its table tests
  — edited
@task_file n=18 path=packages/escher-driver/src/schema.rs
what: the driver schema — the caused field's help and the new caused_past field on the five acting verbs
  — edited
@task_file n=19 path=packages/escher-driver/src/schema_tests.rs
what: the driver schema's unit tests — the field-kind table, 30 to 31 rows
  — edited
@task_file n=20 path=packages/escher-driver/src/lib.rs
what: the driver crate root — exports of the caused types
  — edited
@task_file n=21 path=packages/escher-driver/src/fixture_tests.rs
what: the driver's fixture unit tests — the acting results' new caused_past member
  — edited
@task_file n=22 path=packages/escher-driver/src/recording_tests.rs
what: the driver's session recording unit tests — the acting result's new caused_past member
  — edited
@task_file n=23 path=packages/escher-driver/src/replay.rs
what: check replay through the ordinary call path — the acting result's new caused_past member
  — edited
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: the stand check that a step's errors cross every carrier in caused, with the root-boundary Err on a session of its own; its MCP carrier comparison now decodes the tool result text whole
  — edited
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared session-check module (no target) — gains the one MCP tool-result text decoder used by the session checks
  — edited
@task_item n=1 part=touches
contract/driver-session/acting-verbs — the acting verbs' result shape moved: caused items now carry text, location, source, boundary, chain and fields, count per kind is gone, and caused_past follows caused.
@task_item n=2 part=touches
contract/session-recording/exploration-checks — the caused item shape that exploration checks and kept checks replay moved; a kept check replays its caused and a changed copy stops naming caused.
@task_item n=3 part=touches
decision/root-boundary-err-on-own-session — the split onto its own session kept, but the root Err's text now crosses in caused (chain, boundary root) and the check's sentinel reading is inverted.
@task_item n=4 part=touches
decision/session-writes-no-screen-content, decision/session-logs-no-screen-content, decision/host-stderr-no-screen-content — error text, a panic payload and an app's log messages now cross in the answer; the logs and stderr still hold none (checked at trace).
@task_item n=5 part=touches
decision/typed-password-never-returned and decision/typed-password-never-leaves — a field named password is now redacted on the capture and the sink; a typed password reads 0 times in the answer; a message holds what its author wrote.
@task_item n=6 part=touches
contract/driver-session/password and contract/driver-json/masked-value — a masked control's value stays the mask in the snapshot while caused carries error text beside it.
@task_item n=7 part=touches
decision/telemetry-target-allowlist and decision/sink-drops-third-party-targets — the capture now admits engine targets at warn and error (framework the same, app at every level, escher targets never); the sink's own target and engine-field rules are unchanged.
@task_item n=8 part=touches
decision/sink-capture-counted-by-target and decision/mcp-capture-counts-by-span — the capture is no longer count-only; it keeps record text and panics.
@task_item n=9 part=touches
decision/driver-log-fixed-words-only and contract/driver-command-span/one-line-per-call — the command span keeps its eight fields and holds no caused, asserted by the new check.
@task_item n=10 part=touches
decision/log-silence-read-on-a-sink-host and decision/sink-host-output-drained — the new error-text check installs escher's own sink in process over an in-memory writer at trace, an exception to the headless stand running without a sink.
@task_item n=11 part=touches
decision/stand-repeat-three-rounds, decision/joining-file-keeps-its-legs and decision/zero-failure-stand-repeat — a new stand target joins the stand-repeat leg by its name prefix; 3 of 3 rounds green.
@task_item n=12 part=touches
decision/one-test-target-per-file, decision/blitz-tests-one-target-per-file and decision/blitz-tests-single-test-target — two new blitz-tests targets added; one shared session module gains a helper and stays no target (decision/shared-test-modules-not-targets).
@task_item n=13 part=touches
decision/session-checks-share-one-module and decision/shared-stand-tables-in-one-module — the MCP tool-result text decoder moved, as the one copy, into the shared session-check module.
@task_item n=14 part=touches
decision/mcp-tested-as-child-and-in-process and contract/mcp-tool-result/* — the in-process MCP carrier check now decodes the tool result's JSON string whole before comparing caused; a Windows path's backslashes broke the old un-escape of quotes alone.
@task_item n=15 part=touches
decision/stand-checks-in-both-layout-modes and decision/layout-tests-in-both-layout-modes — the new checks run both modes; they also run both builds, tracing on and off, read through the engine's TRACING_COMPILED.
@task_item n=16 part=touches
decision/socket-session-checks-unix-gated, decision/socket-checks-unix-gated, decision/session-check-owns-its-host and decision/failed-check-host-reaped — two ignored host-child tests serve the socket carrier; the check starts and terminates them.
@task_item n=17 part=touches
decision/mutation-controlled-tests and decision/no-change-read-control-needs-pending-work — the cut control first did not hold because the check computed its expectation from the constant the control mutated; the check now pins the measured value.
@task_item n=18 part=touches
decision/table-tests-count-their-rows — the schema field-kind table count moved 30 to 31 rows.
@task_item n=19 part=touches
decision/capability-check-names-held-to-schema, decision/capability-statement-pinned and decision/capabilities-doc-pinned-to-schema — the schema gains a caused_past field on five verbs and caused's help is rewritten; the capability statement names no caused yet.
@task_item n=20 part=touches
contract/session-wire/answer-bound and decision/snapshot-text-budget — a flooded step's answer line stays under 1,048,576 bytes through the caused bounds (16 items, 1024-byte texts).
@task_item n=21 part=touches
decision/agent-run-events-content-free and decision/assertions-print-no-argument-value — panic payloads now live in the capture and answer; the capture's types print no text in Debug, the hook prints nothing.
@task_item n=22 part=touches
contract/session-run/changed-node-states, contract/session-act/settle and contract/driver-session/act — the acting call result gains caused_past and the settle step reads every boundary's marks.
@task_item n=23 part=touches
decision/per-crate-coverage-floors and decision/coverage-floor-per-crate — the coverage leg re-read green with the new crates' code (escher-driver 97.89, dioxus-native-dom 90.47, harness 96.08); no floor moved.
@task_fact n=1 slug=session-lifecycle
fact: The acting verbs' caused is bounded: 16 items kept in order (panics, errors new since the step's boundary marks, then records), each text cut at 1024 bytes with its cut stated, 8 chain links and 8 fields kept with the rest counted, and caused_past counting the rest by kind and source.
@task_fact n=2 slug=session-lifecycle
fact: A caught panic the capture's hook did not see still answers a panic item whose message is the fixed text <payload unread>, so the panic count never loses an item.
@task_fact n=3 slug=integration-test-strategy
fact: The capture redacts a field by its name in the content-field scrub set (12 names, password among them) on every source, engine included, and keeps the message and the other fields whole; checked with password, value, href, url and style value reading redacted.
@task_fact n=4 slug=integration-test-strategy
fact: A check comparing an MCP tool result against the in-process answer decodes the result's JSON string whole through the one shared helper; un-escaping quotes alone leaves a Windows path's backslashes doubled.
@task_fact n=5 slug=integration-test-strategy
fact: The error-text check installs escher's own sink in process over an in-memory writer at trace and reads 0 sentinel payloads or raw hrefs on its lines — an exception to the headless stand running without a sink.
@task_fact n=6 slug=test-strategy
fact: Engine-sourced caused items are asserted only on a build with tracing compiled in, read through the engine's TRACING_COMPILED constant; the founder check runs on both builds.
@task_fact n=7 slug=unit-test-strategy
fact: A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal, else the control reads green under the mutation.
@task_fact n=8 slug=integration-test-strategy
fact: The capture's panic hook installs once per process, keeps a panic's payload and location only on a capture open on the panicking thread, then always runs the hook it replaced; a panic outside a capture reads as before, checked by its own target.
@task_item n=1 part=untouched
WPT conformance: the runner, its CLI, statuses, expectations, pinned commit and upstream-only workflow.
@task_item n=2 part=untouched
The cold-agent pipe: its events, verdict, exit codes, isolation, prompt and replay bounds.
@task_item n=3 part=untouched
agent-run verbs, selections, state and exit codes, and the CI leg script's exit codes.
@task_item n=4 part=untouched
CI workflow plumbing: actions pinned to SHAs, no secrets, read-only permissions, caches, trigger branches, apt installs and their exit codes, artifacts.
@task_item n=5 part=untouched
Harness construction, contrast, screenshot and inspection helpers, and lint defects and findings: no harness API or lint rule moved.
@task_item n=6 part=untouched
The virtual clock and advance: advance moves and reports exactly the ms asked; stand timer ticks and stand-ins are unchanged.
@task_item n=7 part=untouched
escher-session command-line forms, exit codes and statuses, session wire versions and hello, session errors and the state directory: no verb, exit, refusal cause, session error, socket or env var was added.
@task_item n=8 part=untouched
Mocking discipline: no mocking library, hand-written stand-ins only; the new checks use real failures in a test app.
@task_item n=9 part=untouched
Mutation testing on the dev host and the cognitive-complexity ceiling: neither was run or changed.
