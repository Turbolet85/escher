@task_prose part=session
The session added a caused member to every acting verb's answer (click, type, press, advance, scroll): the panic, handler error or log records the step caused, by kind, level, handler id, owner id, event name and count.
It added a log capture to escher-telemetry: a capture layer beside the sink layer, the EnvFilter now filtering the sink layer alone, and a scoped capture default that prints nothing.
It added a per-step drain of reached handlers and a root error boundary mark to the Dioxus bridge, and the driver now runs each acting step inside one capture with panics caught, the session answering after a caught panic.
An exploration path now ends Panicked when an accepted answer carries a panic item, and its kept check holds the panicked step.
Measured: the Dioxus framework crates trace on every re-render and signal write, so framework targets count only at warn and error; per-thread capture dispatchers proved unsound and were replaced by one process-wide pair.
The caused crossing (closed words, ids and counts, no message or payload) stands on the founder's own word, no longer provisional; the founder also chose a layout verb on the carriers, owed by a follow-up.
@task_file n=1 path=packages/escher-telemetry/src/capture.rs
what: new; escher-telemetry's log capture: the capture layer, the counted levels, the framework target prefix set and the process-wide scoped dispatcher pair
@task_file n=2 path=packages/escher-telemetry/src/lib.rs
what: edited; the escher-telemetry bootstrap: init and its subscriber (sink layer under the EnvFilter, capture layer beside it), the crate root re-exports and crate doc
@task_file n=3 path=packages/dioxus-native-dom/src/boundary.rs
what: new; the Dioxus document's root error boundary mark reader (root_error_mark, RootErrorMark)
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the dioxus-native-dom crate root and its public re-exports
@task_file n=5 path=packages/dioxus-native-dom/src/reached.rs
what: edited; the Dioxus bridge's reached record: reached_handlers, record_reached and the new per-step drain take_step_reached
@task_file n=6 path=packages/escher-driver/src/caused.rs
what: new; the driver's caused items: Caused, CausedKind, CausedLevel and the reading of a step's panics, errors and captured logs
@task_file n=7 path=packages/escher-driver/Cargo.toml
what: edited; escher-driver's manifest, its dependencies (now escher-telemetry)
@task_file n=8 path=packages/escher-driver/src/execute.rs
what: edited; the driver's run path: the acting step, settle and the Outcome variants
@task_file n=9 path=packages/escher-driver/src/session.rs
what: edited; the driver session: Session::start and the sessions it holds
@task_file n=10 path=packages/escher-driver/src/schema.rs
what: edited; the driver verb table and its field specs (the self-description help and MCP listing read)
@task_file n=11 path=packages/escher-driver/src/schema_tests.rs
what: edited; the verb table's unit checks: result fields per verb and the distinct field kinds table
@task_file n=12 path=packages/escher-driver/src/json.rs
what: edited; the driver's own hand-written JSON answer writer and its written-form table
@task_file n=13 path=packages/escher-driver/src/explore.rs
what: edited; escher-driver explore: path expansion and path ending
@task_file n=14 path=packages/escher-driver/src/fixture_tests.rs
what: edited; the driver's fixture unit checks
@task_file n=15 path=packages/escher-driver/src/recording_tests.rs
what: edited; the session recording unit checks
@task_file n=16 path=packages/escher-driver/src/replay.rs
what: edited; kept check replay (Check::replay)
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: edited; the escher-driver crate root and its public re-exports
@task_file n=18 path=tests/blitz-tests/tests/stand_act_errors.rs
what: new; the stand check for caused on acting answers in both layout modes, in process and over serve_mcp, kept check replay and the command span around a panicking call, its fixture page kept local
@task_file n=19 path=tests/blitz-tests/tests/stand_explore.rs
what: edited; the exploration stand check, a panicking path ending Panicked
@task_file n=20 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; the shared stand-check helpers module (non-target)
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited; the command span stand check
@task_file n=22 path=Cargo.lock
what: edited; the workspace lockfile, one dependency line for the new crate edge
@task_item n=1 part=touches
contract/telemetry/init and contract/telemetry-init/init: the subscriber init installs is now a registry with the sink layer filtered by the EnvFilter and an unfiltered capture layer beside it; the LogTracer level, panic hook and printed output unchanged.
@task_item n=2 part=touches
contract/telemetry/rust-log and contract/environment-variables/rust-log: RUST_LOG's EnvFilter now filters the sink layer alone, not the whole subscriber.
@task_item n=3 part=touches
decision/escher-driver-spans-ungated, decision/driver-library-side-effect-free and decision/driver-spans-only: the driver library still installs no global subscriber, but during an acting step it sets escher-telemetry's scoped capture default when the held default has no capture layer; it prints nothing.
@task_item n=4 part=touches
decision/telemetry-one-install-site and decision/escher-telemetry-sink: the capture's scoped dispatcher pair is process-wide, built once and never dropped, beside the one install site; a row saying no other dispatcher exists is stale.
@task_item n=5 part=touches
contract/headless-stand/no-telemetry: the headless stand still prints no telemetry, though acting steps of its sessions now run under the scoped capture default.
@task_item n=6 part=touches
contract/acting-verb-results rows (added, busy, changed, removed, settled) and contract/driver-verbs click, type, press, advance and scroll: each acting verb's result gains caused, last among its fields, written only when non-empty; result fields per verb move 5/6 to 6/7 where a row holds that count.
@task_item n=7 part=touches
contract/verb-table/order and decision/one-self-description-for-cli-and-mcp: the verb table gains the CAUSED field spec (kind Entries, not always present), carried to help and the MCP listing from the table; the distinct field kinds move 29 to 30 where a row holds that count.
@task_item n=8 part=touches
contract/answer-json/outcome, decision/hand-written-answers and decision/answers-written-by-own-json-writer: the hand-written answer now writes caused items with keys kind, level, id, owner, event, count, leaving id, owner and event out when absent.
@task_item n=9 part=touches
contract/driver-public-api rows and contract/dioxus-native-dom-api rows: public enum Outcome's Acted, Advanced and Scrolled gain a caused field (every exhaustive outside match moves); Caused, CausedKind, CausedLevel and RootErrorMark are new re-exported public types.
@task_item n=10 part=touches
contract/reached-record rows (reached-handler, reached-handlers, record-reached, debug) and contract/dioxus-bridge/reached-record: the record gains a per-step list drained by take_step_reached (each handler once, first-reached order); reached_handlers and the Debug form unchanged.
@task_item n=11 part=touches
decision/dioxus-event-path-records-opt-in: Session::start now turns the reached record on for every session the driver holds.
@task_item n=12 part=touches
decision/reached-record-before-dispatch and decision/reached-recorded-before-handlers: the per-step list is written by the same record, before handlers run.
@task_item n=13 part=touches
contract/driver-explore/path-ending and contract/driver-explore/expansion: an accepted answer carrying a panic item now ends the path Panicked (a re-reach step Diverged), still with no payload; its kept check holds the panicked step where it held none.
@task_item n=14 part=touches
contract/check-replay/first-difference, contract/kept-check/answer and decision/kept-check-replays-through-run: a kept check now holds caused in its answers and a replay whose caused differs stops at that step naming caused.
@task_item n=15 part=touches
contract/command-span rows and contract/command-span/never-fielded: the command span keeps its eight fields around a panicking call and holds nothing of caused; no log site, field or env read added.
@task_item n=16 part=touches
decision/driver-logs-no-content, decision/diff-content-never-logged and decision/no-echo-of-typed-input: caused carries closed words, ids and counts, never a panic payload, error text or log message.
@task_item n=17 part=touches
contract/telemetry-scrub/engine-targets, contract/telemetry-scrub/escher-targets, contract/telemetry-scrub/other-targets and decision/telemetry-target-prefix-drop: the capture counts by the same target prefix sets, engine and escher targets excluded, other targets at every level, with a new framework prefix set counted only at warn and error.
@task_item n=18 part=touches
decision/settle-never-waits-on-time-or-loads, contract/busy-classes/render and contract/load-wait rows: the step and its settle run under a panic catch; a panic in settle runs it once more and a second reads busy render.
@task_item n=19 part=touches
decision/owning-component-via-public-api and contract/ui-graph/owner: a caused item's owner is the handler's owner id as the UI graph reads it.
@task_item n=20 part=touches
Rows listing what an acting answer carries across the carriers (decision/actions-answer-crossing, decision/ui-graph-crossing, decision/fixed-program-text-on-carriers): caused joins the crossing, standing on the founder's own word; an Err the root boundary takes crosses only as screen content.
@task_item n=21 part=touches
decision/separately-usable-crates, decision/workspace-dependencies and name/crate/escher-driver: escher-driver gains a workspace path dependency on escher-telemetry, a new crate edge; no registry crate added or bumped.
@task_item n=22 part=touches
decision/layout-read-in-process-only: the founder chose a layout verb on every ratified carrier, overturning the provisional in-process-only reading; the verb is owed by a follow-up entry.
@task_item n=23 part=touches
decision/capability-statement-matches-schema and contract/capability-statement/sections: the schema gains caused; what the capability statement says of it is owed to its own entry.
@task_item n=24 part=touches
decision/single-reader-page-stays-local and decision/stand-check-shared-helpers: the new stand check keeps its one-reader fixture page local and shared session helpers moved with the Outcome field.
@task_fact n=1 slug=standard-contracts
fact: Each acting verb's answer carries caused when its step caused something: one item per kind (panic, error, log) and level (error, warn, info, debug, trace) with the reached handler's id, owner id, event name and count; closed words, ids and counts only, no message or payload; limited to the first handler, the app root boundary and tracing records (not the log crate).
@task_fact n=2 slug=cross-cutting-patterns
fact: The log capture counts records from targets outside the engine and escher prefix sets at every level, except the Dioxus framework targets (dioxus, generational_box, warnings), counted only at warn and error because they trace on every re-render and signal write.
@task_fact n=3 slug=cross-cutting-patterns
fact: escher-telemetry's subscriber is a registry holding the sink layer under the EnvFilter and an unfiltered capture layer; the capture layer holds counts by level only and prints nothing.
@task_fact n=4 slug=standard-contracts
fact: Where the held default has no capture layer, the capture uses a process-wide pair of scoped dispatchers built once and never dropped, one used and a twin never used, so tracing-core never falls into its one-dispatcher interest mode; per-thread dispatchers were rejected.
@task_fact n=5 slug=standard-contracts
fact: The driver runs an acting step and its settle inside one capture under a panic catch; the session keeps answering after a caught panic, a second panic in settle reading busy render.
@task_fact n=6 slug=standard-contracts
fact: A handler Err the app root error boundary takes is read by the root error mark and reads one error item in caused; the boundary's rendering of its Display reaches the screen as ordinary screen content, never caused; an Err the app's own boundary catches leaves no mark.
@task_fact n=7 slug=standard-contracts
fact: DioxusDocument::take_step_reached drains the handlers reached since the last drain, each once in first-reached order, and DioxusDocument::root_error_mark reads the root error boundary's error context, RootErrorMark comparing by identity and printing its name alone.
@task_fact n=8 slug=conventions
fact: A stand check that installs its own recorder opens an empty capture first, so the capture's dispatchers are registered before the recorder and no call site's interest is cached from a thread with no subscriber.
@task_fact n=9 slug=standard-contracts
fact: escher-driver depends on escher-telemetry, a workspace path crate edge, for the step capture.
@task_item n=1 part=untouched
Engine layout, style, paint, text and hit-testing decisions and contracts: no engine behaviour changed.
@task_item n=2 part=untouched
Select list, form control, focus, keyboard and accessibility tree rows: no control behaviour or tree state moved.
@task_item n=3 part=untouched
The session socket wire, MCP server protocol, request and answer bounds and the session host process model: no wire version, method, bound or listener changed.
@task_item n=4 part=untouched
Refusal causes, refusal faults, session errors, node keys, argument kinds, key names and exit codes: none added or changed, and no verb was added to the validated verb set.
@task_item n=5 part=untouched
CI leg contracts, workflow, coverage floors and cargo-deny audit: no leg script, floor or threshold changed.
@task_item n=6 part=untouched
Browser app, WPT runner, script runtime, net provider and example CLI rows.
@task_item n=7 part=untouched
Occupied names for binaries, servers and environment variables: no name taken or freed.
