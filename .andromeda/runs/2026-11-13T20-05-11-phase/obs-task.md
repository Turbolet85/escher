@task_prose part=chunk
Restructures three functions, keeping behaviour the same, to bring each one's cognitive complexity under the ceiling of 15 or closer to it: blitz-dom's click default-action handler handle_click (97), escher-driver's command executor execute (32) and dioxus-native-dom's UI graph builder graph (50).
Extracted helpers stay in the file that holds the function today. No file is split, no module is added, no new helper reads over 15, and a function left above 15 names why.
Behaviour stays the same: dispatch order, event sequences, default actions, refusal order, outcomes, span fields, snapshot and graph content. No new verb, argument, key, field, cause, error, exit, log field or span.
Re-measures the three functions and every extracted helper before and after with rust-code-analysis-cli on the dev host, never in CI. Re-runs the existing click, act, graph and handler-coverage checks, both layout modes, the incremental oracle and the fast leg.
@task_file n=1 path=packages/blitz-dom/src/events/pointer.rs mark=edits
what: blitz-dom's pointer event handling, holding the click default actions (checkbox and radio input and change, button focus and submit, select arms, links, summary, labels) — edits: the handle_click function and the helpers extracted from it
@task_file n=2 path=packages/escher-driver/src/execute.rs mark=edits
what: escher-driver's executor: the per-verb dispatch of driver commands, the target check and the refusal order — edits: the execute function and the helpers extracted from it
@task_file n=3 path=packages/dioxus-native-dom/src/graph.rs mark=edits
what: dioxus-native-dom's UI graph builder behind the graph answer (nodes and states) — edits: the DioxusDocument graph function and the helpers extracted from it
@task_item n=1 part=needs
The driver's command span and its fields (verb, settled, passes, added, removed, changed, busy, cause) are opened around the per-verb dispatch the restructure moves. Each verb must still emit the same single span with the same fields and values, and no field may be added.
@task_item n=2 part=needs
The MCP tool_call span, its command child and its ending (accepted, refused, error) must keep the same values when a driver call goes through the restructured executor.
@task_item n=3 part=needs
The refusal causes (not-found, stale, off-screen, covered, disabled, malformed, fonts-unavailable, unknown-verb) and the order the executor checks them in must stay the same. No cause is added or renamed.
@task_item n=4 part=needs
The graph answer's shape and content (nodes and states) produced by the UI graph builder must not change.
@task_item n=5 part=needs
The driver command answers (graphed, linted, shot, actions) must keep the same shape and content after the executor is restructured.
@task_item n=6 part=needs
The page script events that click default actions fire (change on a checkbox, change on a radio, one change per interaction) must keep the same count and order.
@task_item n=7 part=needs
Select interaction default actions (chosen option, multiple select value, open list rows and labels) move with the click handler's select arms and must keep their behaviour. The chosen set and the row labels are never logged.
@task_item n=8 part=needs
The snapshot's clickable marking for nodes and text, which the click default actions sit behind, must not change.
@task_item n=9 part=needs
Indirect: the driver emits no tracing event and installs no subscriber, and its spans carry no call content. Helpers extracted from the executor must not add an event, a span or a content-bearing field.
@task_item n=10 part=needs
Indirect: the graph answer, a part's answer and answer-bearing types are never logged. Helpers extracted from the graph builder and the executor must not log or debug-print what they carry.
@task_item n=11 part=needs
Indirect: engine tracing in blitz-dom stays behind its cargo feature, and engine targets print only safe fields. A helper extracted from the click handler must not add an ungated or content-bearing trace.
@task_item n=12 part=needs
The handler coverage line (owner, shape, content), which reads the graph through the handler-coverage check, must stay the same.
@task_standing n=1
convention: a check added to cover a click, executor or graph arm before it is moved runs silently, with no sink installed in a headless stand, and unsets RUST_LOG where it asserts empty stderr
home: 3-observability-harness-contract
@task_standing n=2
convention: a check added for a moved arm that reads a span or a log line asserts only on the closed span's line and its fixed fields, never on content
home: log-coverage · 6-log-coverage
@task_item n=1 part=not_needed
The cold-agent pipe's events, verdict fields, reasons, counted verbs and kept checks. The chunk does not touch the cold-agent harness.
@task_item n=2 part=not_needed
The agent-run harness events (boot, run start and end, test, status, cleanup) and its raw log. No harness script changes.
@task_item n=3 part=not_needed
escher-telemetry's sink installation, panic hook, scrub set, resource keys (service.name, service.version) and the RUST_LOG configuration. The sink is used unchanged.
@task_item n=4 part=not_needed
Script error reporting: the window error event, window.onerror, script error source labels and the embedder's drain of script diagnostics. These are untouched.
@task_item n=5 part=not_needed
WPT reports and the publishing of WPT scores upstream only. The chunk does not touch them.
@task_item n=6 part=not_needed
Session recording files, the recording and session state directories, recording bounds, the host wire version and session lifecycle expiry. No session or recording logic changes.
@task_item n=7 part=not_needed
CI artifact uploads (per-leg log on failure, coverage on success), the package-install diagnostic line and the WPT and fork CI publishing. No CI leg is added or changed.
@task_item n=8 part=not_needed
Stylesheet loading and file: load rules, and the WASM panic hook. The chunk does not touch them.
