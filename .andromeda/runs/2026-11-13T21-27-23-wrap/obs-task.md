@task_prose part=session
The session restructured three functions toward a cognitive complexity of 15 with behaviour unchanged: the engine's click default actions, the driver session's command execution, and the Dioxus document's UI graph build.
Each was split into private per-arm or per-step functions; no public symbol, signature or caller changed, and no span field, refusal cause, verb, log field, env read, listener or log site was added or removed.
The driver's command span stays opened in the session's run with its eight fields recorded where they were; the graph build still reads the state registry and the dispatch record under the accessibility gate.
It added one silent integration check of click default actions (link navigation, the label re-dispatch, focus clearing, a disabled submit, a multiple select's label, the double click), holding no tracing, print, env read or sink.
Measured: the three functions' cognitive readings fell to 2, 1 and 4; coverage floors held (escher-driver 98.01 %, dioxus-native-dom 90.09 %, blitz-test-harness 95.74 %); CI green, 17 of 17 checks.
Nothing was retired, no dependency, config or schema changed.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling — the click default actions: checkbox and radio toggling with their change events, label re-dispatch to its bound control, summary toggling, select and option choosing, link navigation, file input, focus clearing and the double click dispatch
@task_file n=2 path=packages/escher-driver/src/execute.rs
what: the driver's command execution — one arm per verb (snapshot, click, type, key, advance, scroll, screenshot, graph, lint, actions) producing the command answer, under the command span opened by the session's run
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph answer builder — nodes, their handlers, the state readings from the state registry, handler writes and reads, built from the dispatch record under the accessibility gate
@task_file n=4 path=tests/blitz-tests/tests/click_default_actions.rs
what: a new silent integration check of click default actions in both layout modes, with a recording navigation provider
@task_item n=1 part=touches
The driver's command span (contract/command-span/* and contract/escher-driver-command-span/*): the verb functions it wraps were split out of one execute; the span is still opened in the session's run with its fields recorded at the same points — a picker should confirm no field moved.
@task_item n=2 part=touches
decision/one-span-per-command and decision/hosted-call-through-session-run: command execution was restructured beneath the session's run, which stays its sole caller; the rule still holds.
@task_item n=3 part=touches
decision/driver-emits-no-event, decision/driver-library-installs-no-subscriber, decision/driver-spans-carry-no-content and decision/command-span-no-call-content: rules over the driver's command execution, which was restructured with no event, subscriber or span field added.
@task_item n=4 part=touches
The graph answer (contract/graph-answer/shape, contract/graph-answer/content, contract/command-answer/graphed, decision/graph-answer-never-logged, contract/command-span/graph): the graph builder and the driver's graph arm were split into helper functions, its shape and content unchanged.
@task_item n=5 part=touches
decision/state-registry-holds-no-values and decision/no-state-page-skips-registry: rules over the graph builder's reading of the state registry, now in its own helper.
@task_item n=6 part=touches
decision/reached-record-ids-and-events-only: the graph builder's handler and dispatch reading of the reached record moved into helpers.
@task_item n=7 part=touches
contract/page-events/checkbox-change, contract/page-events/radio-change and contract/page-events/single-change: the click handling that dispatches the change events on a checkbox and a radio moved into per-arm functions; behaviour unchanged, now also exercised by the new check.
@task_item n=8 part=touches
decision/checks-silent: a new silent check of click default actions joins the silent checks; it holds no tracing, print, env read or sink.
@task_item n=9 part=touches
contract/snapshot-node/clickable and contract/snapshot-text/clickable: what a click reaches is decided by the engine's click walk, restructured with behaviour unchanged.
@task_item n=10 part=touches
The select rows (decision/select-chosen-set-never-logged, decision/select-list-label-never-logged, contract/multiple-select-form-entries/chosen-option, contract/snapshot/multiple-select-value): the select and option click arms and the driver's type step on a select were moved into named functions with no log site added.
@task_item n=11 part=touches
decision/session-logs-carry-no-page-content and decision/shot-image-leaves-only-in-answer: the driver's typing and screenshot arms (the masked typed text, the crop and its error map) were moved into named functions with no log site added.
@task_item n=12 part=touches
decision/no-file-loads-from-non-file-documents: link navigation on click, which resolves the href against the document URL, moved into its own function unchanged.
@task_item n=1 part=untouched
escher's telemetry sink, its log line format, scrub set, target allowlist, RUST_LOG handling, resource keys and panic hook: no sink, scrub or subscriber code changed.
@task_item n=2 part=untouched
The MCP tool_call span, its ending and the refusal cause names: no cause, tool or ending was added, renamed or moved.
@task_item n=3 part=untouched
The cold-agent pipe and the agent-run harness: their events, verdict fields, counted verbs, kept checks and transcripts are not reached.
@task_item n=4 part=untouched
escher-session's commands, exit codes, stderr, coverage role line, host wire, recording directory and session lifecycle: unchanged.
@task_item n=5 part=untouched
Metric emission, metrics exporters, telemetry backends and WPT score publishing: the session adds no metric, exporter or backend.
@task_item n=6 part=untouched
Script error capture, the window error event and the WPT report: not reached.
@task_item n=7 part=untouched
CI integration rows (artifacts, workflows, cleanup, package-install diagnostics): no workflow or CI script changed; the coverage floors held.
@task_item n=8 part=untouched
Engine and escher tracing gating by cargo feature: no tracing call or feature gate was added, removed or moved.
