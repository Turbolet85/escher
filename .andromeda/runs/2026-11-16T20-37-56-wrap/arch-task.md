@task_prose part=session
The session put each failure's text into the acting verbs' caused answer: one item per panic, boundary error or kept log record, with its message, source (app, framework or engine), target, file, line, column, error boundary, cause chain and kept fields, scrubbed by field name and cut at measured bounds.
The capture stopped being count-only: it keeps record text, admits engine targets at warn and above for the first time, and installs once per process a chaining panic hook that keeps the payload and location on the open capture and always runs the hook it replaced.
dioxus-native-dom gained a boundary registry provided at the root scope and a public boundary_marks reader; the driver reads it instead of root_error_mark, which stays public.
The answer gained caused_past on click, type, press, advance and scroll; the scrub set CONTENT_FIELDS grew from 11 to 12 names with password; escher_telemetry::subscriber became a public uninstalled builder taking a LevelFilter.
Six engine log sites moved their interpolated values into named fields under fixed messages, and blitz-dom gained TRACING_COMPILED; level, target and per-call-site tracing gating are unchanged.
Retired: the closed-words caused shape with a count per kind and level, the capture's count-only contract, and the driver's read of root_error_mark; two new blitz-tests targets were added and an MCP comparison now decodes the result text whole after a Windows CI red.
@task_file n=1 path=tests/blitz-tests/tests/stand_act_error_text.rs
what: added, then edited after implement; the founder check of the caused text on every carrier (in process, MCP, session socket host children, kept check replay), a stand-repeat target that installs escher's own sink in process over an in-memory writer at trace
@task_file n=2 path=tests/blitz-tests/tests/capture_panic_hook.rs
what: added; a blitz-tests target checking the capture's chaining panic hook installs once and leaves a panic outside a capture reading as before
@task_file n=3 path=packages/escher-telemetry/src/capture.rs
what: edited; the scoped capture with its process-wide dispatcher pair, now keeping record text, sources, bounds and the chaining panic hook
@task_file n=4 path=packages/escher-telemetry/src/lib.rs
what: edited; the escher-telemetry bootstrap and its init path, now exporting a public subscriber builder and the capture's types
@task_file n=5 path=packages/escher-telemetry/src/format.rs
what: edited; the telemetry line format and scrub, holding the content field set CONTENT_FIELDS
@task_file n=6 path=packages/blitz-dom/src/events/pointer.rs
what: edited; the engine's pointer event handling, its link-click log site (a direct engine change)
@task_file n=7 path=packages/blitz-dom/src/form.rs
what: edited; the engine's form submission, its unimplemented-scheme log site (a direct engine change)
@task_file n=8 path=packages/blitz-dom/src/mutator.rs
what: edited; the DocumentMutator, its iframe load log site (a direct engine change)
@task_file n=9 path=packages/blitz-dom/src/layout/mod.rs
what: edited; the engine's layout construction, its text-node error log site (a direct engine change)
@task_file n=10 path=packages/blitz-dom/src/lib.rs
what: edited; the blitz-dom crate root, now exporting TRACING_COMPILED
@task_file n=11 path=packages/dioxus-native/src/assets.rs
what: edited; dioxus-native's asset loading, its two log sites
@task_file n=12 path=packages/dioxus-native-dom/src/boundary.rs
what: edited; dioxus-native-dom's error boundary readers, root_error_mark beside the new boundary registry and boundary_marks
@task_file n=13 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: edited; the Dioxus document, which now provides the boundary registry before its first build
@task_file n=14 path=packages/dioxus-native-dom/src/lib.rs
what: edited; the dioxus-native-dom public API re-exports
@task_file n=15 path=packages/escher-driver/src/caused.rs
what: edited; the driver's caused reading of the acting verb results
@task_file n=16 path=packages/escher-driver/src/execute.rs
what: edited; the driver's acting step and its outcomes (Acted, Advanced, Scrolled)
@task_file n=17 path=packages/escher-driver/src/json.rs
what: edited; the driver's own hand-written JSON answer writer
@task_file n=18 path=packages/escher-driver/src/schema.rs
what: edited; the driver's command and refusal schema, the self-description shared by the command line and MCP
@task_file n=19 path=packages/escher-driver/src/schema_tests.rs
what: edited; the schema's unit tests and field-kind table
@task_file n=20 path=packages/escher-driver/src/lib.rs
what: edited; the escher-driver public API re-exports
@task_file n=21 path=packages/escher-driver/src/fixture_tests.rs
what: edited; driver unit tests over fixtures, a companion of the acting step
@task_file n=22 path=packages/escher-driver/src/recording_tests.rs
what: edited; driver unit tests of session recording, a companion of the acting step
@task_file n=23 path=packages/escher-driver/src/replay.rs
what: edited; kept check replay, a companion of the acting step
@task_file n=24 path=tests/blitz-tests/tests/stand_act_errors.rs
what: edited, then edited after implement; the stand check of errors on their own session, root boundary sentinel inverted, its MCP comparison decoding the result text whole
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited after implement; the shared stand-check helpers module, now holding mcp_result_text
@task_item n=1 part=touches
contract/acting-verb-results/caused is stale: caused is now one item per record carrying kind, level, source, target, message with cut, file, line, column, boundary, chain with chain_past, fields with fields_past, and id, owner, event; the count per kind and level is retired; at most 16 items (CAUSED_KEPT), chain at most 8 (CHAIN_KEPT).
@task_item n=2 part=touches
The acting verb results and driver verbs click, type, press, advance and scroll gained a field caused_past, written after caused only when non-empty; rows listing those verbs' result fields or the answer JSON outcome shape are touched.
@task_item n=3 part=touches
decision/driver-library-side-effect-free is broken unless restated: the capture the driver opens now installs, once per process, a chaining panic hook (it prints, logs and fields nothing and always runs the hook it replaced).
@task_item n=4 part=touches
decision/driver-spans-only and decision/escher-driver-spans-ungated: the driver still opens spans and emits no event, but the capture it opens now carries a process-wide hook; the stated exception belongs beside them.
@task_item n=5 part=touches
contract/dioxus-native-dom-api/root-error-mark: root_error_mark and RootErrorMark stay public but the driver no longer calls them; boundary_marks, BoundaryMark, Boundary, BoundaryError and BoundaryRegistry stand beside them.
@task_item n=6 part=touches
decision/accessibility-feature-gating in dioxus-native-dom: the new boundary registry and boundary_marks are ungated by any feature.
@task_item n=7 part=touches
decision/dioxus-root-contexts: the Dioxus document now provides a boundary registry as a root context before the first build, and the error boundary factory registers every boundary in it.
@task_item n=8 part=touches
decision/direct-engine-changes-flagged-upstreamable: six engine log sites (link click, form scheme, iframe load, text-node layout error, two asset loads) are new direct engine edits flagged upstreamable, plus TRACING_COMPILED in the blitz-dom crate root.
@task_item n=9 part=touches
decision/tracing-gated-per-call-site and decision/engine-tracing-gated-per-call-site: gating is unchanged at each edited site; TRACING_COMPILED now names whether engine tracing is compiled; the interpolating warn and error sites went from 6 to 0 of 43.
@task_item n=10 part=touches
decision/telemetry-scrubbing and contract/telemetry-scrub/content-span-fields: the content field set CONTENT_FIELDS grew from 11 to 12 names with password, on the sink and the capture alike; any row naming the set's members or count is touched.
@task_item n=11 part=touches
contract/telemetry-scrub/engine-targets and contract/telemetry/engine-fields: the sink's engine rule (safe fields alone, message redacted) is unchanged on its lines, but the engine's values moved into the fields href, url, src, text and request, and the capture keeps an engine record's message with only content-named fields redacted.
@task_item n=12 part=touches
decision/capture-dispatcher-pair and decision/capture-before-own-recorder: the capture now admits app targets at every level, framework and engine targets at warn and error (engine newly), escher targets never; it keeps text bounded by TEXT_CUT 1024, FIELDS_KEPT 8 and RECORDS_KEPT 16, and it is no longer Copy.
@task_item n=13 part=touches
contract/telemetry/panic-hook and contract/telemetry-panic/panic-event: the sink's own panic hook is unchanged; a second, chaining hook from the capture now runs in every process that opens a capture.
@task_item n=14 part=touches
contract/telemetry-init/init, decision/telemetry-one-install-site and decision/escher-telemetry-sink: escher_telemetry::subscriber is now a public builder returning the sink's subscriber plus the capture layer uninstalled, with a LevelFilter; a test installs it in process.
@task_item n=15 part=touches
contract/headless-stand/no-telemetry: stand_act_error_text installs escher's own sink in process over an in-memory writer at trace, an exception to the headless stand running without a sink.
@task_item n=16 part=touches
contract/driver-session/command-schema, decision/one-self-description-for-cli-and-mcp and decision/capability-statement-matches-schema: the schema's caused help is rewritten and the field caused_past added; the capability statement names no caused today and an account of it is owed.
@task_item n=17 part=touches
decision/measured-limits-stated-in-the-tool: the caused bounds (16 items, chain 8, fields 8, text cut 1024) are stated in the schema's help where an agent reads the tool.
@task_item n=18 part=touches
decision/answers-written-by-own-json-writer and decision/hand-written-answers: the hand-written writer gained the caused item and caused_past shapes; contract/answer-json/outcome and contract/answer-json/string-escaping cover the new text.
@task_item n=19 part=touches
decision/driver-logs-no-content, decision/kept-check-never-logged and contract/command-span/never-fielded: the error text crosses in the answer alone; the command span keeps its eight fields and holds no caused; no sink line holds the text.
@task_item n=20 part=touches
contract/kept-check/answer, decision/kept-check-masking and decision/recorded-session-kept-check: a kept check's answer now holds the error text and replays it; a changed cut stops replay naming caused; a masked control's value stays masked.
@task_item n=21 part=touches
decision/fixed-program-text-on-carriers and decision/actions-answer-crossing: author-written messages, payloads and cause chains now cross every carrier in caused, beyond the program's fixed text.
@task_item n=22 part=touches
contract/mcp-server/tool-result and contract/mcp-role/tool-call-crossings: the same caused crosses serve_mcp as in process; a check comparing a tool result's text decodes the JSON string whole.
@task_item n=23 part=touches
decision/blitz-tests-one-target-per-file, decision/blitz-tests-single-test-target and contract/ci-leg-legs/stand-repeat: two new blitz-tests targets, stand_act_error_text joining stand-repeat by its prefix and capture_panic_hook.
@task_item n=24 part=touches
decision/stand-check-shared-helpers and decision/shared-test-modules: mcp_result_text is now the one shared copy in the stand-check helpers module.
@task_item n=25 part=touches
decision/per-crate-coverage-floor and decision/coverage-floors: floors unchanged and held (escher-driver 97.89, dioxus-native-dom 90.47, blitz-test-harness 96.08); touched only where a row holds a measured figure.
@task_fact n=1 slug=standard-contracts
fact: The acting verbs click, type, press, advance and scroll answer caused_past after caused when non-empty: one entry per kind and source counting the items past caused's 16, a panic or error entry writing no source.
@task_fact n=2 slug=standard-contracts
fact: caused orders panics first, then errors new since the step's boundary marks, then records in record order; a caught panic the capture's hook did not see still answers a panic item with message <payload unread>.
@task_fact n=3 slug=standard-contracts
fact: The capture keeps text from three sources (app at every level, framework and engine at warn and error, escher never), scrubs by field name in CONTENT_FIELDS for every source while keeping messages, cuts each text at 1024 bytes saying how much, and keeps at most 8 fields and 16 records per source, counting the rest.
@task_fact n=4 slug=established-decisions
fact: The capture installs once per process a chaining panic hook, the stated exception to a side-effect-free driver library: it keeps payload and location on the capture open on the panicking thread, prints and logs nothing, and always runs the hook it replaced.
@task_fact n=5 slug=standard-contracts
fact: dioxus-native-dom provides a boundary registry at the root scope before the first build; every error boundary built under it registers, boundary_marks reads them by identity, and the registry grows for the document's life because a dropped boundary cannot be told.
@task_fact n=6 slug=standard-contracts
fact: escher_telemetry::subscriber is a public builder returning the sink's subscriber with the capture layer, uninstalled, at a given LevelFilter.
@task_fact n=7 slug=standard-contracts
fact: blitz-dom exports TRACING_COMPILED, true when the engine's tracing feature is compiled in.
@task_fact n=8 slug=conventions
fact: Engine warn and error log sites carry their values in named fields under fixed messages and never interpolate a value into the message.
@task_fact n=9 slug=conventions
fact: A stand check comparing an MCP tool result's text against the in-process answer decodes the JSON string whole through the one shared helper mcp_result_text.
@task_fact n=10 slug=cross-cutting-patterns
fact: CONTENT_FIELDS holds password among its 12 content field names, on the sink and the capture alike.
@task_item n=1 part=untouched
Style resolution, cascade, Stylo, Taffy layout modes and incremental layout: no layout or style result moved.
@task_item n=2 part=untouched
Paint, renderers, screenshots and the scrollbar and select list decisions.
@task_item n=3 part=untouched
The accessibility tree contracts and AccessKit adapter decisions.
@task_item n=4 part=untouched
Snapshot, snapshot diff, stable element ids, node keys, lint and the graph and actions verbs.
@task_item n=5 part=untouched
Session socket wire version, session errors, refusal causes and faults, exit codes and command line usage: no new method, verb, refusal cause, session error or exit.
@task_item n=6 part=untouched
Occupied names and resources: no binary, crate, env var, port, socket or listener was taken or freed.
@task_item n=7 part=untouched
Stack and dependencies: none added or bumped; the toolchain, CI workflow, actions pins and Nix flake are unchanged.
@task_item n=8 part=untouched
Browser app, WPT runner, scripting with Boa and network provider decisions.
@task_item n=9 part=untouched
Upstream take and sync decisions: no upstream merge, cherry-pick or rebase in this session.
