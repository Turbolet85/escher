@task_prose part=chunk
Builds an error field on an acting driver call's answer (click, type, press, advance, scroll): the handler panic, error-level log record or handler log record its step caused, beside the snapshot diff it returns today.
Each item is tied to the stable element id the step acted on and, through the UI graph, to the handler that ran (owner id and event name) — the graph's existing node identity, never an engine node id.
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome instead of ending the host process; records emitted during a step are captured, bounded per call.
The field rides every ratified carrier of the acting verbs (in process, the session socket's call, the command line, MCP, the kept check recorded and replayed), and the driver's help and the MCP tool listing describe it.
What of a log may enter an answer is the founder's boundary fork; the telemetry scrub, the span fields and the log lines stay as they are.
@task_file n=1 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's run path for the acting verbs — edits: the step's run, where a handler panic is caught and the step's records are attached to the answer
@task_file n=2 path=packages/escher-driver/src/session.rs mark=edits
what: the held session and its run of a call — edits: the session's run path (a caught handler panic returned as the call's outcome)
@task_file n=3 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema: the acting verbs' answer, its result fields and refusal causes — edits: the answer's new field
@task_file n=4 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description (help) — edits: the description of the new answer field
@task_file n=5 path=packages/escher-driver/src/json.rs mark=edits
what: the answer's written form on the carriers — edits: the new field's written form
@task_file n=6 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP tool results and the MCP tool listing — edits: the tool listing's description and the result carrying the new field
@task_file n=7 path=packages/escher-driver/src/recording.rs mark=edits
what: the kept check's recording of a session's calls and answers — edits: the recorded answer
@task_file n=8 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check's replay and its answer comparison — edits: the replayed answer's comparison
@task_file n=9 path=packages/escher-telemetry/src/lib.rs
what: the telemetry subscriber, its target allowlist and its layers — where a step's records can be captured without changing what the sink prints
@task_file n=10 path=packages/escher-telemetry/src/format.rs
what: the log sink's line format and its scrub of escher targets' fields
@task_file n=11 path=packages/escher-telemetry/src/panic.rs
what: the chaining panic hook
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
what: the UI graph: node identity and handler identity (owner id and event name)
@task_file n=13 path=packages/dioxus-native-dom/src/reached.rs
what: the Dioxus document reached-handler record
@task_file n=14 path=examples/seven_guis
what: the stand, whose tasks no handler panics or logs from; a minimal fixture app beside it carries the case
@task_file n=15 path=tests/blitz-tests/tests
what: the integration tests over the driver and the stand — a new test target for this behaviour, in both layout modes
@task_item n=1 part=needs
The acting verbs' answer gains one field listing the handler panic, error-level log record or handler log record the step caused; the snapshot diff it returns today stays beside it unchanged.
@task_item n=2 part=needs
Each item of the new field is tied to the served node id of the element the step acted on and, through the UI graph, to the handler that ran (owner id and event name) — the UI graph's existing node identity and the reached-handler record, never an engine node id.
@task_item n=3 part=needs
A handler panic during a step on a held session is caught on the run path and returned as the call's outcome rather than ending the host process; what the instance is after a caught panic is read in P3.
@task_item n=4 part=needs
The capture of what a step caused: records emitted during the step, bounded per call, attributed to the step's target and handler.
@task_item n=5 part=needs
The same field on every ratified carrier the acting verbs ride: in process, the session socket's call, the command line, the MCP tool results and the kept check, recorded and replayed.
@task_item n=6 part=needs
The driver's self-description and the MCP tool listing describe the new field.
@task_item n=7 part=needs
Indirect, through the carriers: any authored text crossing onto a carrier — a panic payload, a log message, a log field's value — is a boundary widening the founder decides; a new field of a closed value set inside a ratified carrier is not one. The founder's log rulings bound it: content-free log fields, no screen content in logs, the closed-span line.
@task_item n=8 part=needs
Indirect, through the telemetry sink: the target allowlist and the redaction of escher targets' fields stay as they are; the driver's command span and the MCP tool_call span gain no field holding what the answer now carries; no log line carries the new answer content.
@task_item n=9 part=needs
Indirect, through the answer: a password value is never revealed and no screen content leaves through the new field — a captured record's text is subject to the same rule as any answer value.
@task_item n=10 part=needs
No new carrier, listener, socket, port, verb, exit status or session error; a new refusal cause or result field only as the driver schema's tables admit and the plan names.
@task_item n=11 part=needs
No panic path on a driver call's arguments: a panic caught here is the app handler's, never the driver's.
@task_item n=12 part=needs
The UI graph stays read-only and carries no values: the handler identity is read from it, and nothing captured is written into it.
@task_item n=13 part=needs
The layout read, explore and Session::reach keep their contracts; explore's per-path panic ending is not widened to carry a payload.
@task_item n=14 part=needs
Whether the new field joins the kept answer's comparison as any other answer field or is held out of the digest is a design point for P4; the kept check's recording and replay must stay deterministic either way.
@task_item n=15 part=needs
The proof: a minimal fixture app beside the stand whose handlers panic, return an error and log, driven through every acting verb, in a new test target run in both layout modes.
@task_standing n=1
convention: each new test target over the stand or a fixture rides the CI tier the plan assigns to it and repeats its stand checks over the rounds the plan sets
home: 9-ci-integration · ci-integration
@task_standing n=2
convention: each new example or fixture binary builds no platform accessibility adapter, and the session library it is driven through keeps accessibility on
home: a11y-scope-summary
@task_standing n=3
convention: each new expectation over a fixture's roles, names or answers is stated once, in a shared table, and every reader asserts against it
home: screen-reader-support
@task_item n=1 part=not_needed
Role mapping, accessible-name precedence and ARIA state and property reads: the chunk reads no new role, name or state off the accessibility tree.
@task_item n=2 part=not_needed
Key handling contracts of controls — range input keys, single and multiple select keys and lists, the Cells grid, the CRUD list, Tab order, keyboard activation: the acting verbs keep what they do to a control; only their answer gains a field.
@task_item n=3 part=not_needed
Contrast, colour, lint findings, select list rendering and the stand's visual design: the chunk touches no paint or style.
@task_item n=4 part=not_needed
Hidden and inert content, focusability and focus moves: no rule over which nodes enter the tree or take focus changes.
@task_item n=5 part=not_needed
Cognitive accessibility and invalid-state presentation: the chunk adds no user-facing state or text to the app.
@task_item n=6 part=not_needed
The screenshot verb, the actions verb and the snapshot's select and list row readings: none is an acting verb and none gains the field.
@task_item n=7 part=not_needed
The accessibility tree's platform adapter and AccessKit event routing: the chunk does not reach assistive-technology output.
