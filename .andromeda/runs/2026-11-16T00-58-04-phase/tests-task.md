@task_prose part=chunk
Adds one reading of an element, by its stable id: where it sits and at what size, and why — the layout mode of its parent, the computed-style inputs that mode took, its containing block, and any scroll or transform that moves it.
The reading runs no step and changes nothing, like snapshot, lint, graph and actions; it reads the engine's resolved layout and computed style, with no second layout and no engine change.
Its answer's placement kind is an open set: the shape reserves «placed by the app inside canvas X», an owner element by stable id, which nothing in 0.3.0 produces.
If it is a driver verb it lands on every ratified carrier — schema, executor, written forms, command line, MCP listing and results, recording and replay, self-description — and in the capability statement and its pin.
Proven on the stand in both layout modes, with minimal fixtures where the stand lacks a flex child, a grid item, a positioned, scrolled or transformed box.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema: the verb table every carrier reads, its argument kinds and its level check — edits: the verb table, a new reading verb
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's verb specs, each verb's arguments and answer — edits: the new verb's spec
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver executor that runs each verb against the held instance — edits: the new reading verb's execution
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's JSON answers, the written form of each verb's answer — edits: the new verb's answer form
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line, its command forms, usage line and help — edits: the new verb's command form
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the escher-session MCP role's tools/list listing — edits: the new verb's tool entry
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the escher-session MCP tool results, a verb's answer or refusal served over MCP — edits: the new verb's result
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the help each surface serves — edits: the new verb's description
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement, its claim sections and the checks each claim names, pinned to the driver schema — edits: a claim for the layout-explained reading
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the capability statement's pin: claim section names and counts held to the schema — edits: the expected verb names and counts
@task_item n=1 part=needs
The new reading verb over a held instance: given an element's stable id it answers its bounds (as the snapshot reads them) and the chain that placed it; it is a reading verb, acts on nothing and leaves the instance unchanged.
@task_item n=2 part=needs
The answer's placement kind is not a closed set: it must be able to name a box placed by the app inside a canvas, its owner element by stable id, without a contract change later — reserved as the UI graph reserved the owner-key virtual id form.
@task_item n=3 part=needs
The why chain: the parent's layout mode (block, flex, grid, inline, absolute or fixed positioning, float), the computed-style inputs it took (sizing, margins, offsets, the parent's alignment and gaps), the containing block, and any scroll or transform moving the box from where layout put it.
@task_item n=4 part=needs
Read from what the engine already holds — the resolved layout per node and the computed style turned into the layout style — with no second layout pass and no engine change; an engine change that appears is the founder's fork.
@task_item n=5 part=needs
A new verb on every ratified carrier is a boundary widening: the driver verb table and its lookup, the executor, the JSON answer forms, the command line's command form and help, the MCP tools/list order and tool results, the self-description, and the recording and replay of a kept check — each with the same answer on both surfaces, as lint, graph and actions were added.
@task_item n=6 part=needs
The capability statement: its claim section names and counts are held to the driver schema, and the new claim names the checks that prove it.
@task_item n=7 part=needs
The widening rule over the answer's fields: closed words (layout modes, placement kinds) and numbers are no widening; a field carrying the page's style text as the author wrote it, or a property's value, is the founder's fork, so the answer holds no such value unless it is ratified.
@task_item n=8 part=needs
The command span for the new verb records the verb and, refused, the cause alone; no span or driver log line holds the answer's content or its size — fixed words and counts only.
@task_item n=9 part=needs
Refusals of the new verb: a malformed, unknown, stale or not-found element id is refused by the fixed refusal causes in their order, with their remedy, changing nothing and echoing no input.
@task_item n=10 part=needs
The answer travels the session wire and the snapshot-style byte budget: it stays inside the answer bound, and an oversize answer is refused as any other.
@task_item n=11 part=needs
A scrolled or transformed box reads where it stands: the answer accounts for the scroll offset and transform; its checks of a scrolled case are re-read when Taffy moves past #1372 (reversed scroll origin).
@task_item n=12 part=needs
The lint link: a lint finding that names an element (clipped, overlap, off-screen, zero-size, truncated) is what this reading explains the cause of; the lint findings and their order stay as they are unless the chunk's P4 joins them.
@task_item n=13 part=needs
Proof on the seven_guis headless stand in both layout modes, on the bundled font with no skip, with minimal fixture pages stated in code beside it for a flex child, a grid item, a positioned box, and a scrolled or transformed one.
@task_item n=14 part=needs
The new verb's command-line and MCP checks run the package's own binary over the socket session, unix-gated, each check owning its host; a recorded check of the new verb replays through the ordinary call path.
@task_item n=15 part=needs
The new verb's unit and executor tests: table-driven tests over the layout modes count their rows, and a failing assertion prints no argument value.
@task_standing n=1
convention: each new integration test file is its own test target; shared fixtures and stand tables sit in one shared module that is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new layout assertion runs in both layout modes on the bundled font, never skipped
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=3
convention: each new check holds no sleep and no wall-clock read; time moves only by the virtual clock
home: integration-test-strategy
@task_standing n=4
convention: each new unit test sits in an inline test module of its source file and is controlled red-before-green
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=5
convention: each new fixture page or test datum is stated in code, the stand booting offline, nothing seeded
home: test-data-fixtures · 7-test-data-fixtures
@task_standing n=6
convention: each new stand stand-in is hand-written, with no mocking library
home: mocking-stubbing-discipline · 8-mocking-stubbing-discipline
@task_standing n=7
convention: each new stand check holds to the zero-failure budget across the repeated stand rounds
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
WPT conformance rows — the runner, its CLI, statuses, expectations and pinned commit: the chunk adds no conformance suite and changes no layout.
@task_item n=2 part=not_needed
Cold-agent pipe rows — its verbs, events, verdict, exit codes, isolation and wrong-call bar: the chunk adds no cold-agent task.
@task_item n=3 part=not_needed
Agent-run rows — its verbs, selections, events, exit codes and state files: the chunk adds no agent-run selection.
@task_item n=4 part=not_needed
Session lifecycle rows — start, attach, stop, idle expiry, the state directory and its path bound, the hello and version handshake: the new verb rides the existing call path without changing the lifecycle.
@task_item n=5 part=not_needed
Virtual-clock, advance and timer-tick rows, and the settle and load-wait rows of acting verbs: the new verb reads and acts on nothing.
@task_item n=6 part=not_needed
Typed-password, type-on-select and change-event rows: the reading answers placement and size, not values.
@task_item n=7 part=not_needed
Handler coverage rows — bubbling, canvas handlers, unreached screens, the coverage line: the chunk counts no listener.
@task_item n=8 part=not_needed
Screenshot, pixel-test, PNG and contrast-formula rows: the reading paints and measures no colour.
@task_item n=9 part=not_needed
Paint rows — the select list painter, scrollbar thumbs and fade, box-shadow expectations, font embolden: the chunk changes no painting.
@task_item n=10 part=not_needed
Upstream Dioxus bridge and buffered-render rows, and the blitz-tests manifest rows: the chunk takes nothing from upstream.
@task_item n=11 part=not_needed
CI package-install, action-pinning, secrets, cache and artifact rows: the chunk changes no workflow.
@task_item n=12 part=not_needed
Canvas virtual-node production and drag: those stay in 0.4.0; only the answer's shape is kept open for them.
