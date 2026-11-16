@task_prose part=chunk
Builds one read-only read of a named element's placement and size by its stable id: where it sits and at what size (the bounds the snapshot already reads) and why.
The why is the chain that placed it: its parent's layout mode, the inputs that mode took from the computed style, the containing block, and any scroll or transform that moves it.
It is read from the layout and computed style the engine already holds, with no second layout and no engine change; it runs no step and changes nothing, like snapshot, lint, graph and actions.
The answer's placement kind is an open set that reserves an app-placed-inside-canvas case naming an owner element by stable id; nothing in 0.3.0 produces it.
If the read is a driver verb it joins every ratified carrier, as lint, graph and actions did; its checks of a scrolled case are re-read when Taffy moves past #1372.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's request and answer schema, the closed value sets on its carriers — edits: the new read's verb and its answer shape
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's verb specification, each verb and its fields — edits: the new read's verb entry
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's executor, which runs each verb against the session and answers or refuses — edits: the new read's execution
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's written JSON forms of each answer — edits: the new read's answer form
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the driver's command line, its verbs and their arguments — edits: the new read's command
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: MCP's tool listing, the tools the driver offers an agent — edits: the new read's tool entry
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: MCP tool results, each verb's answer as an MCP result — edits: the new read's result
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the verbs and carriers it states to an agent — edits: the new read's entry
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement, what escher states an agent can do and its reach, edited by hand — edits: the new read's capability
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin, the check that holds the statement to the driver's verbs — edits: the new read's pinned entry
@task_item n=1 part=needs
The new read's answer for a named element: its bounds as the snapshot already reads them, plus why — its parent's layout mode (block, flex, grid, inline, absolute or fixed positioning, a float), the inputs that mode took from computed style (sizing, margins, offsets, the parent's alignment and gaps), its containing block, and any scroll or transform moving it.
@task_item n=2 part=needs
The element is named by its stable id, the same id the snapshot, graph and actions answers carry; an unknown or unplaced id is a refusal with a closed cause, as the driver's refusal causes are.
@task_item n=3 part=needs
The answer's placement kind is not a closed set: it reserves a case placed by the app inside a canvas, naming the owner element by its stable id, so 0.4.0 adds it with no contract change; nothing in 0.3.0 produces it, as the served node id reserved the /// separator for virtual nodes.
@task_item n=4 part=needs
The read changes nothing and runs no step, like snapshot, lint, graph, actions and a screenshot: it reads the engine's resolved layout and the computed style already turned into the layout style, with no second layout and no engine change.
@task_item n=5 part=needs
Indirect, through the widening rule on the carriers: a new field of a closed value set (layout modes, placement kinds) or a number is not a widening; a new verb, an open value set, or a field carrying a page's style text (a length as authored, a property value) is the founder's fork and goes to the founder's forks list, not decided by the chunk.
@task_item n=6 part=needs
The new read on every ratified carrier as lint, graph and actions were: the schema, the executor, the JSON forms, the command line, MCP's listing and results, the recording and replay of a kept check, and the self-description.
@task_item n=7 part=needs
The capability statement and its pin name the new read; the statement's reach is edited by hand.
@task_item n=8 part=needs
Indirect, through the driver's command span: it records the verb and, refused, the cause alone; no field holds the read's content or its size, under the log allowlist and redaction rules of escher's own targets.
@task_item n=9 part=needs
The link to the lint: where a lint finding names an element and its bounds, this read is what tells its cause; whether the lint's answer points to it is P4's to settle.
@task_item n=10 part=needs
Proven on the stand in both layout modes, with a minimal fixture beside it where the stand lacks a case: a flex child, a grid item, a positioned box, a scrolled or a transformed one.
@task_item n=11 part=needs
A scrolled box's placement may read differently once Taffy moves past #1372 (reversed scroll origin); the chunk's checks of a scrolled case are the ones re-read at that take.
@task_standing n=1
convention: each new check is a Rust test over the session's own reads, not a browser or screen-reader harness
home: 3-a11y-assertion-harness-contract
@task_standing n=2
convention: each stand expectation the new tests assert is stated once in a shared table, not restated per test
home: screen-reader-support
@task_standing n=3
convention: each new stand check runs repeated rounds in the slow-tier test leg and gates CI
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
Keyboard navigation and activation: Tab order, Enter and Space, range input keys, select list keys, the seven_guis grid and list keys; the read sends no input.
@task_item n=2 part=not_needed
ARIA states and property reads and the role mapping: the read answers placement and size, not role, name or state.
@task_item n=3 part=not_needed
The accessible name's precedence and labels naming their element.
@task_item n=4 part=not_needed
Contrast thresholds, the contrast ratio formula, stand colours and scrollbar thumb strokes: the read measures no colour.
@task_item n=5 part=not_needed
The open select list's colours, row heights, motion and its open state.
@task_item n=6 part=not_needed
Password masking and value readings of form controls: the answer carries no control value.
@task_item n=7 part=not_needed
Click and focus behaviour: a click focusing a control, disabled controls stopping a click, focus fixup and focus dropping on removal.
@task_item n=8 part=not_needed
The Dioxus reached-handler record, handler coverage and UI graph state registration.
@task_item n=9 part=not_needed
Cognitive accessibility: invalid state beyond colour or class.
@task_item n=10 part=not_needed
Form event values and the input and change events a select fires.
