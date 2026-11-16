@task_prose part=session
The five acting verbs (click, type, press, advance, scroll) now answer a `caused` field: one item per panic, handler error or captured log level their step caused, by node id, owner id, event, closed kind and level words and a count, absent when nothing was caused; no message, payload or error text crosses, on the founder's own word (no longer PROVISIONAL).
escher-telemetry gained a scoped log capture (`capture`, `CaptureLayer`, `Captured`) counting records by level outside the engine and escher_ targets, the Dioxus framework targets (dioxus*, generational_box, warnings) at warn and error only; the sink's EnvFilter now filters the sink layer alone, the capture layer sits beside it unfiltered, and the capture's own dispatcher is one process-wide pair kept registered.
The driver catches a panic in a step and in its settle, keeps answering after it, and exploration ends a panicking path Panicked with its kept check holding the panicked step; the reached-handler record gained a per-step drain and the dioxus document a root error boundary mark.
A new stand check, stand_act_errors, proves it in both layout modes in process, over serve_mcp and through a kept check, plus the command span keeping its eight fields around a panicking call; after implement it opens an empty capture before installing its own recorder, and a crate doc link was disambiguated after CI's Documentation job read it red.
Measured: acting verbs' result fields 5/6 to 6/7, the schema's field kinds table 29 to 30 rows, the JSON written-form table 14 to 17 rows, escher-driver lib tests 120 to 121, escher-telemetry lib tests 15 to 21, reached and boundary tests 4 to 10; blitz-tests gained one stand_ target, joining stand-repeat by prefix.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: new — the telemetry crate's scoped log capture: counts records by level inside a closure, the framework target set at warn and above, the process-wide dispatcher pair with its never-used twin
@task_file n=2 path=packages/dioxus-native-dom/src/boundary.rs
what: new — the dioxus document's read of the root error boundary's mark, telling a new handler Err from none
@task_file n=3 path=packages/escher-driver/src/caused.rs
what: new — the driver's caused items: the closed kind words (panic, error, log) and level words, built from one step's reading
@task_file n=4 path=tests/blitz-tests/tests/stand_act_errors.rs
what: new, then edited after implement — the stand check of an acting call's caused answer in both layout modes, over serve_mcp and a kept check, and of the command span keeping its eight fields around a panicking call; its fixture page kept in its own file
@task_file n=5 path=packages/escher-telemetry/src/lib.rs
what: edited — the telemetry crate root: the sink's init, its subscriber with the EnvFilter now on the sink layer alone and the capture layer beside it; the crate doc's capture link
@task_file n=6 path=packages/dioxus-native-dom/src/lib.rs
what: edited — the dioxus document crate root, re-exporting the root error mark
@task_file n=7 path=packages/dioxus-native-dom/src/reached.rs
what: edited — the reached-handler record that handler coverage reads, now also a per-step list drained each step, each handler once in first-reached order
@task_file n=8 path=packages/escher-driver/Cargo.toml
what: edited — the driver's manifest, taking the telemetry crate as a new dependency edge
@task_file n=9 path=packages/escher-driver/src/execute.rs
what: edited — the driver session's acting step: runs the step and its settle under the capture and a panic catch, and builds the acting call's outcome with caused
@task_file n=10 path=packages/escher-driver/src/session.rs
what: edited — the driver Session: Session::start now turns on the reached record on every held session
@task_file n=11 path=packages/escher-driver/src/schema.rs
what: edited — the driver's verb table: the caused field spec last among the acting verbs' result fields, with its help text
@task_file n=12 path=packages/escher-driver/src/schema_tests.rs
what: edited — the table-driven tests of the verb table and its distinct field kinds, with their row counts
@task_file n=13 path=packages/escher-driver/src/json.rs
what: edited — the driver JSON answers' written form: caused written after the verb's other fields only when non-empty, and its written-form table test
@task_file n=14 path=packages/escher-driver/src/explore.rs
what: edited — the exploration: reads an accepted answer carrying a panic item as a panicked path ending
@task_file n=15 path=packages/escher-driver/src/fixture_tests.rs
what: edited — the driver's unit tests over its shared fixture, matching the acting outcomes' new field
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: edited — the driver's session recording unit tests, matching the acting outcomes' new field
@task_file n=17 path=packages/escher-driver/src/replay.rs
what: edited — check replay, comparing a recorded acting answer including caused
@task_file n=18 path=packages/escher-driver/src/lib.rs
what: edited — the driver crate root, re-exporting the caused types
@task_file n=19 path=tests/blitz-tests/tests/stand_explore.rs
what: edited — the stand exploration check: a panicking path ends Panicked with no payload and its kept check holds the panicked step
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the shared session check module, its exhaustive match on acting outcomes moved
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited — the command span check of the acting verbs, its exhaustive match on acting outcomes moved
@task_file n=22 path=Cargo.lock
what: edited — the workspace lockfile, one dependency line for the driver's telemetry edge
@task_item n=1 part=touches
contract/session-recording/exploration-checks: a panicked exploration path's kept check now holds the panicked step (it held none); the report names this row stale.
@task_item n=2 part=touches
The acting call result rows (contract/session-run/changed-node-states, contract/driver-session/act, contract/driver-session/acting-verbs, contract/driver-session/run-carriers): every acting verb's answer on every carrier gains the caused field, present only when the step caused something.
@task_item n=3 part=touches
contract/driver-verbs/verb-table and the help rows (contract/mcp-server/help, contract/mcp-help/parity, contract/cli-help/output, contract/mcp-server/tools-list): the verb table gains the CAUSED field spec, carried into help and the MCP listing from the table.
@task_item n=4 part=touches
decision/table-tests-count-their-rows: counts moved — the field kinds table 29 to 30 rows, the written-form table 14 to 17, acting verbs' result fields 5/6 to 6/7; a row holding any of these counts is touched.
@task_item n=5 part=touches
decision/telemetry-target-allowlist, decision/sink-drops-third-party-targets and decision/sink-capture-counted-by-target: the sink and its allowlist are unchanged, but a capture layer now sits beside the sink, unfiltered by its EnvFilter, counting third-party targets by level (Dioxus framework targets only at warn and error).
@task_item n=6 part=touches
decision/log-silence-read-on-a-sink-host and decision/headless or no-subscriber readings: an acting step of an in-process session now sets a scoped capture default that prints nothing; any row saying the driver library or stand installs no subscriber reads against it.
@task_item n=7 part=touches
The command span rows (contract/driver-command-span/one-line-per-call, contract/stand-log/command-span, contract/session-host-stderr/command-span): the span keeps its eight fields around a panicking call and holds nothing of caused, now checked by a new stand test.
@task_item n=8 part=touches
Screen-content and no-value rows (decision/session-writes-no-screen-content, decision/session-logs-no-screen-content, decision/driver-log-fixed-words-only, decision/actions-answer-holds-no-value): caused carries only ids, owner ids, event names, closed words and counts; an Err the root boundary takes reaches the screen as the boundary's rendering of its Display, an existing screen-content crossing.
@task_item n=9 part=touches
Handler coverage rows (decision/handler-coverage-bubbling-counts-ancestors, contract/handler-coverage-limits/bubbling-dispatch): the reached-handler record they read gains a per-step drain, a bubbling dispatch draining its target then its ancestor; caused ties an item to the first handler reached alone.
@task_item n=10 part=touches
decision/one-test-target-per-file and decision/blitz-tests-one-target-per-file: blitz-tests gains one target, stand_act_errors.
@task_item n=11 part=touches
decision/stand-repeat-three-rounds and decision/zero-failure-stand-repeat: the new stand_ target joins stand-repeat by its prefix, no leg script changed.
@task_item n=12 part=touches
decision/stand-checks-in-both-layout-modes: the new caused checks run in both layout modes.
@task_item n=13 part=touches
decision/shared-fixture-pages, decision/shared-fixtures-in-one-module and decision/shared-stand-tables-in-one-module: the caused fixture page stays in its one reader's check file, not the shared fixtures module.
@task_item n=14 part=touches
decision/mcp-tested-as-child-and-in-process: the same caused member is checked crossing serve_mcp in process.
@task_item n=15 part=touches
decision/recorded-checks-replayed-as-tests, contract/check-replay/stop and decision/checks-byte-equal-to-recording: a kept check replays its caused, a flipped count stops at its step naming caused, and an answer that caused nothing reads as before so the committed Counter checks replay unmoved.
@task_item n=16 part=touches
decision/assertions-print-no-argument-value: the new Debug forms print counts alone (Captured) or the type name alone (RootErrorMark).
@task_item n=17 part=touches
decision/hand-written-stand-ins: the command span check's hand-written recorder subscriber must now be preceded by an empty capture so two dispatchers stay registered.
@task_item n=18 part=touches
decision/local-pre-push-gate and decision/ci-legs-host-reproducible: the local gate block and the fast leg run no rustdoc, so an ambiguous intra-doc link read red only in CI's Documentation job.
@task_item n=19 part=touches
decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema and decision/capability-check-names-held-to-schema: the schema gains the caused field the capability statement is held to; what the statement says of caused is owed to its existing entry.
@task_fact n=1 slug=integration-test-strategy
fact: An acting call's answer carries caused only when its step caused something — one item per kind and level, with node id, owner id and event where known and a count — and never a message, payload or error text; its stated limits are the first handler reached, the app's root boundary alone, and records of the log crate.
@task_fact n=2 slug=integration-test-strategy
fact: After a caught panic in a step the session keeps answering and a later acting call answers with no caused; a panic in the settle runs it once more and a second reads not settled, busy render.
@task_fact n=3 slug=integration-test-strategy
fact: A handler Err the root boundary takes is checked on a session of its own and its sentinel check reads caused alone, because the root boundary replaces the app and renders the error on screen.
@task_fact n=4 slug=3-test-harness-contract
fact: The scoped log capture counts every level from targets outside the engine and escher_ prefixes, except the Dioxus framework targets (dioxus*, generational_box, warnings), counted only at warn and error since they trace on every re-render and signal write.
@task_fact n=5 slug=3-test-harness-contract
fact: The capture keeps one process-wide dispatcher pair registered for the process, and a test installing its own recorder subscriber opens an empty capture first, so tracing-core never falls into its one-dispatcher interest mode and caches a call site as never.
@task_fact n=6 slug=session-lifecycle
fact: Exploration reads an accepted answer carrying a panic item as a panicked path ending with no payload, a re-reach step as diverged, and the kept check holds the panicked step.
@task_fact n=7 slug=9-ci-integration
fact: Rustdoc with denied warnings runs in CI's Documentation job and the local doc leg, not in the fast leg or the local gate block, so a doc-link ambiguity reads red only there.
@task_item n=1 part=untouched
WPT conformance, the WPT runner, its CLI, outputs, status set and pinned commit: nothing of web-platform testing moved.
@task_item n=2 part=untouched
Pixel, screenshot, contrast and lint rows, the harness screenshot and contrast contracts and the lint defects: no rendering, contrast or lint behaviour changed.
@task_item n=3 part=untouched
Agent-run and cold-agent rows, their events, verbs, exit codes, verdicts and state: the cold-agent and agent-run pipes are unchanged.
@task_item n=4 part=untouched
Refusal causes and session errors: no refusal cause, session error, exit code or node key was added; a refused call carries no caused.
@task_item n=5 part=untouched
Session wire, socket, state directory and host lifecycle rows: no wire version, bound or lifecycle verb moved.
@task_item n=6 part=untouched
Mocking, virtual clock and advance rows: time and the advance contract are unchanged, the advance answer only gains caused like every acting verb.
@task_item n=7 part=untouched
CI script contracts (apt-install exit codes, coverage floor script exit codes, CI artifacts) and the coverage floors themselves: unchanged, the coverage leg read 3 of 3 crates at or above their floors.
@task_item n=8 part=untouched
Layout reader and layout reason rows: the layout read is unchanged by this session.
@task_item n=9 part=untouched
The windowed end-to-end absence and the mocking library absence still hold.
