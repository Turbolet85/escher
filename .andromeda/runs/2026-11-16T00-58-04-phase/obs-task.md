@task_prose part=chunk
Builds one read-only read, by an element's stable id, of where the element sits and at what size, and why: the layout mode of its parent, the computed-style inputs that mode took, the containing block, and any scroll or transform that moves it.
It reads Taffy's resolved layout and the computed Stylo style the engine already holds. It runs no second layout and makes no engine change.
The answer's placement kind is an open set. Its shape reserves «placed by the app inside canvas X», naming an owner element by its stable id, though nothing in 0.3.0 produces that case.
If the read is a driver verb, it is a new verb on every ratified carrier, the way lint, graph and actions were. It is proven on the stand in both layout modes, with minimal fixtures where the stand lacks a case.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command schema, the set of verbs and their arguments that every carrier shares
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's per-verb specification
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's executor, which runs each command against the session and builds its answer and refusal cause
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the written forms of the driver's command answers, the JSON that each verb's answer takes
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the driver's command line, its verbs and arguments for escher-session
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: MCP's tool listing, the tools escher-session's mcp role offers
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: MCP's tool results, how a command answer or refusal is returned through a tool_call
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, its help text that lists the verbs
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement, kept up to date by hand, that names what an agent can do through the driver
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the check that pins the capability statement against the verbs the driver actually offers
@task_item n=1 part=needs
The new placement read answers with an element's bounds (the same bounds the snapshot reads) and the chain that placed it: the parent's layout mode (block, flex, grid, inline, absolute or fixed positioning, float), the sizing, margins, offsets, alignment and gaps that mode took from the computed style, the containing block, and any scroll or transform. It changes nothing in the document, like snapshot, lint, graph and actions.
@task_item n=2 part=needs
The answer's placement kind is not a closed set. It must be able to name an element placed by the app inside a canvas, with the owner given by its stable id, so 0.4.0 can add that case without a contract change. The UI graph reserved the owner-and-key id form the same way.
@task_item n=3 part=needs
The read takes Taffy's final layout for each node and the computed Stylo style that was converted into the Taffy style. There is no second layout pass, and any engine change that turns out to be needed is the founder's fork.
@task_item n=4 part=needs
If the read is a driver verb, it joins every ratified carrier: the schema, the executor, the written answer forms, the command line, MCP's tool listing and results, escher-session's stdout command answer and mcp JSON-RPC lines, the self-description help, and the hand-kept capability statement with its pin. This is ratified as a boundary widening, as lint, graph and actions were.
@task_item n=5 part=needs
The driver's command span gets the new verb as its verb value and records cause only when the command is refused. No span field holds the read's content or its size, and the span's field count stays the same.
@task_item n=6 part=needs
If the read is offered over MCP, its tool_call span carries the new tool name and one of the closed endings (accepted, refused, error), and the command span is its child.
@task_item n=7 part=needs
Refusals of the read use the closed set of refusal cause names: not-found for an unknown stable id, malformed for bad arguments, and stale where the id no longer holds. It adds no free-text cause.
@task_item n=8 part=needs
The widening rule sorts the answer's fields. Closed words (layout modes, placement kinds) and numbers inside an already ratified carrier are no widening. A new verb, or a field carrying a page's authored style text (a length as written, a property's value), is the founder's fork, which goes to founder-forks.md while he is away. P4 settles which of these the answer needs.
@task_item n=9 part=needs
The read's answer is never logged and never echoed into telemetry, as with the graph answer and other answer-bearing types. The driver emits no event of its own and installs no subscriber, and session logs carry no page content: no geometry, no style values, no ids from the answer.
@task_item n=10 part=needs
Recording and replay of a kept check of the new read: the recording file and kept check file hold its arguments, answer and answer digest. Committed kept checks hold no private data, and the recording stays within its bounds and its private directory.
@task_item n=11 part=needs
The lint link: where a lint finding names an element, this read is what tells the finding's cause. P4 decides whether lint's answer points to it.
@task_item n=12 part=needs
Proof on the stand in both layout modes, with minimal fixtures beside it for any case the stand lacks: a flex child, a grid item, a positioned box, and a scrolled or transformed box.
@task_item n=13 part=needs
A check of a scrolled box's placement may read differently after the later Taffy take past #1372 (ScrollableOverflow, reversed scroll origin), so those checks are marked to be re-read at that take.
@task_item n=14 part=needs
For a cold agent, the new read is a verb like graph and actions, and the counted verbs and verdict fields decide whether it is counted toward the agent's calls.
@task_standing n=1
convention: every new test of the driver or session runs silent. A headless stand has no sink, an empty-stderr check unsets RUST_LOG, and a log check that fails names only a kind and a count.
home: 3-observability-harness-contract
@task_standing n=2
convention: every new fixture or committed kept check holds no secrets and no private data, and local session state is never uploaded.
home: 9-ci-integration · ci-integration
@task_standing n=3
convention: every new diagnostic or error text the chunk adds is a fixed string with no page or caller content in it.
home: 6-log-coverage · log-coverage
@task_item n=1 part=not_needed
The script error surface (the window error event, window.onerror arguments, script error source labels, the embedder drain of script diagnostics): the read touches no script errors, and «Errors with the action» is its own entry.
@task_item n=2 part=not_needed
WPT reporting and the publishing of WPT scores: the chunk adds no WPT run or report.
@task_item n=3 part=not_needed
Form controls and their privacy rows (select lists, optgroup labels, multiple-select form entries, checkbox or radio change events): the read adds no form-control behaviour.
@task_item n=4 part=not_needed
The agent-run and cold-agent harness event lines (boot, run.start, run.end, status, cleanup, test events) and transcript handling: the chunk adds no harness event.
@task_item n=5 part=not_needed
escher-telemetry's startup and panic events, the panic hook's chaining, sink installation order in each native main, and the sink's resource keys: none of these change.
@task_item n=6 part=not_needed
Engine and upstream tracing feature gates, and the WASM console panic hook: the chunk changes no engine and no WASM build.
@task_item n=7 part=not_needed
CI package-install diagnostic lines and the artifacts fork CI uploads (a per-leg log on failure, a coverage report on success): the chunk changes no workflow.
@task_item n=8 part=not_needed
Stylesheet loading and the rule against file: loads from non-file: documents: the read only reads computed style that has already been resolved.
