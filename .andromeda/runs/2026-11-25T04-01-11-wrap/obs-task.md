@task_prose part=session
The session added virtual nodes declared: an app's canvas widget declares keyed, named, role-bearing regions inside its content box, and the dioxus bridge reads and validates them in process.
The engine's widget trait gained a defaulted method returning no declarations and a declaration type (key, AccessKit role, name, region), behind the custom-widget feature; no engine code calls it.
The bridge gained an ungated read of a canvas's declared nodes: each taken node gets the reserved id {owner}///{key}; each refused declaration reads a closed reason (empty key, key holding a slash, duplicate key with the first kept, empty name, region outside the content box) with its index alone; a stale or non-canvas node reads no-canvas, an unlaid one not-laid-out.
The read has no panic path, no log site, print, span or field, no changed-set mark and no invalidation signal; no declared node reaches the snapshot, the graph or any carrier, and a check asserts that in both layout modes.
Every answer behind it is a delegate's and lands PROVISIONAL; the names' crossing to the accessibility tree and the invalidation and changed-set mark stay deferred to the later virtual-nodes-as-elements entry.
No dependency, manifest, CI workflow, gate, telemetry sink or driver verb changed; nothing was retired.
@task_file n=1 path=packages/blitz-dom/src/node/custom_widget.rs
what: the engine's custom widget trait (the canvas widget boundary), now with the defaulted virtual-nodes declaration method and the declaration type
@task_file n=2 path=packages/blitz-dom/src/node/mod.rs
what: the engine's node module and its re-exports of the widget types
@task_file n=3 path=packages/blitz-dom/src/lib.rs
what: the engine crate root and its public re-export list
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus bridge crate root, its ungated module list and public re-export list
@task_file n=5 path=packages/dioxus-native-dom/src/element_id.rs
what: the bridge's stable element id and the reserved virtual-node id form {owner}///{key}, documented as now produced in process
@task_file n=6 path=packages/dioxus-native-dom/src/virtual_node.rs
what: the bridge's in-process declared-node read of a canvas, its validation with closed refusal reasons, and its unit tests
@task_file n=7 path=tests/blitz-tests/tests/virtual_nodes_declared.rs
what: the integration check of declared nodes in the content box, of a widget declaring nothing, and that a declaring canvas puts nothing on the snapshot or graph
@task_item n=1 part=touches
decision/debug-types-never-logged: the answer-bearing types never logged gain the declared-node types (the declaration, the taken node, the refusal, the read's answer and its unread reason).
@task_item n=2 part=touches
The canvas read and the layout read open no span: a new sibling in-process read, of a canvas's declared nodes, likewise opens no span and records no field.
@task_item n=3 part=touches
Engine log sites write fixed messages and escher-session diagnostics are fixed strings: the declared-node read adds no log site, print or diagnostic, so no fixed-message rule gains an entry or an exception.
@task_item n=4 part=touches
Session logs carry no page content and content-named span values are never stored: a declared node's name and key are app content that stays in process, written to no log, span or answer.
@task_item n=5 part=touches
The graph answer, the snapshot node and the snapshot text (content, clickable): unchanged by the read; a declaring canvas adds no node, no reserved id and no declared name to the snapshot or the graph, asserted by a check.
@task_item n=6 part=touches
Escher crates take tracing ungated and the engine's tracing stays behind its feature: the bridge's ungated module list gains the declared-node module, which takes no tracing; the engine side is behind the custom-widget feature and logs nothing.
@task_item n=7 part=touches
Driver emits no event and its command span carries no content: no verb, span field or command answer gains a declared node; the canvas field of the command span is unchanged.
@task_item n=8 part=touches
Refusal cause names: the declared-node refusals (empty key, key holding a slash, duplicate key, empty name, region outside) and unread reasons (no canvas, not laid out) are a separate in-process set, not driver refusal causes, and no carrier prints them.
@task_fact n=1 slug=6-log-coverage
fact: The declared-node read of a canvas (its validation and its answer) has no log site, print, span or field; a refused declaration is a closed reason with its index alone, and nothing of it is logged (PROVISIONAL, a delegate's answer).
@task_fact n=2 slug=4-span-trace-coverage
fact: The declared-node read opens no span, like the canvas read and the layout read.
@task_fact n=3 slug=8-pii-scrubbing-compliance
fact: A declared virtual node's name, key and reserved id stay in process: no snapshot, graph, span or log carries them until the virtual-nodes-as-elements entry decides their crossing (PROVISIONAL, deferred to the founder's fork).
@task_item n=1 part=untouched
escher's telemetry sink and its log line contracts (format, stream, scrub, RUST_LOG, service name and version, resource keys): no sink or line changed.
@task_item n=2 part=untouched
The agent-run and cold-agent harness events, verdict fields, counted verbs and their logs and transcripts: no harness changed.
@task_item n=3 part=untouched
Metric coverage and the metric and telemetry-backend absences: no metric or exporter added.
@task_item n=4 part=untouched
Error capture: panic hooks, script error surfaces, window error events and WPT reports; the new read has no panic path.
@task_item n=5 part=untouched
CI integration: workflows, artifacts, recording bounds, session state and recording directories and the build-directory sweep line; CI ran unchanged and green.
@task_item n=6 part=untouched
escher-session's commands, exit codes, mcp role, coverage role, host wire and the tool_call span and its endings.
@task_item n=7 part=untouched
Select, optgroup, drag-and-drop, mutation buffer, state registry and page script event rows.
