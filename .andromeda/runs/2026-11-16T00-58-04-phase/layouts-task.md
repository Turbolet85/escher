@task_prose part=chunk
Builds one read-only read of an element's placement and size by its stable id: where it sits and at what size, and why — its parent's layout mode, the computed-style inputs that mode took, its containing block, and any scroll or transform that moves it.
The read runs no step and changes nothing, like snapshot, lint, graph and actions; it reads Taffy's resolved layout and the computed Stylo style the engine already holds, with no second layout and no engine change.
The answer's placement kind is an open set: its shape reserves a «placed by the app inside canvas X» case naming an owner element by its stable id, though nothing in 0.3.0 produces it.
If the read is a driver verb it joins every ratified carrier — schema, executor, written forms, command line, MCP listing and results, recording and replay, self-description and the capability statement — as lint, graph and actions did.
It is proven on the stand in both layout modes, with minimal fixtures where the stand lacks a case; its scrolled-case checks are re-read when Taffy is later taken past #1372.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's request schema — the verbs the session accepts and their arguments — edits: the new layout-explained verb
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the schema's verb specification — each verb's arguments and answer shape — edits: the new verb's spec
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the executor that runs each verb against the session's document — edits: the new verb's execution
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the written (JSON) forms of each verb's answer — edits: the new answer's written form
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — its command form and verbs — edits: the new verb's command
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the MCP server's tools/list listing — edits: the new verb's tool entry
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the MCP server's tool results and their bound — edits: the new verb's result
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the session's self-description (help answer) of its verbs — edits: the new verb's description
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement — what an agent can do through the driver — edits: the layout-explained capability
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the check pinning the capability statement to the driver's verbs — edits: the pin for the new verb
@task_item n=1 part=needs
A new read-only escher-session verb whose answer gives a named element's bounds (where it sits, at what size) and the chain that placed it, by the element's stable id.
@task_item n=2 part=needs
The answer names the parent's layout mode as a closed word: block, flex, grid, inline, absolute or fixed positioning, float.
@task_item n=3 part=needs
The answer names the inputs that mode took from the computed style — sizing, margins and offsets, the parent's alignment and gaps — and the containing block the element was placed in.
@task_item n=4 part=needs
The answer names any scroll or transform that moves the element from where layout put it; holding scrollers and clipping boxes are the same chain the visible-region and into-view reads walk.
@task_item n=5 part=needs
The answer's placement kind is an open value set: it reserves «placed by the app inside canvas X» with the owner element's stable id, so 0.4.0 can add it with no contract change, as the UI graph reserved the owner/key id form.
@task_item n=6 part=needs
The read takes no step and changes no state, like snapshot, lint, graph and actions; its bounds are those the snapshot already reads.
@task_item n=7 part=needs
The read is built from what the engine already holds — Taffy's resolved layout per node and the computed Stylo style turned into the Taffy style — with no second layout and no engine change; an engine change that appears is the founder's fork.
@task_item n=8 part=needs
The new verb reaches every ratified carrier as a boundary widening: the command form, the help answer, the MCP tools/list and its results within the result bound, and the session recording as a replayable check.
@task_item n=9 part=needs
The widening rule decides the answer's fields: closed words (layout modes, placement kinds) and numbers are not a widening; a new verb, an open value set, or a field carrying a page's style text as the author wrote it is the founder's fork, written to founder-forks.md while he is away and the step stops.
@task_item n=10 part=needs
The command span records verb and, refused, cause alone; no field of it holds the read's content or its size.
@task_item n=11 part=needs
Caller input is never written back: an answer names elements by their stable ids and closed words, not echoing what the caller sent.
@task_item n=12 part=needs
A refused read is a refusal with a cause on the session's exit codes, as the other verbs refuse.
@task_item n=13 part=needs
Proof on the stand in both layout modes, under the headless stand's pinned viewport, against the stand's stable element ids (e.g. cells grid, task cards, timer progress, circle canvas).
@task_item n=14 part=needs
Minimal fixtures beside the stand where it lacks a case: a flex child, a grid item, a positioned box, a scrolled or transformed one.
@task_item n=15 part=needs
The scrolled-case checks are marked to be re-read when Taffy is taken past #1372 (scrollable overflow, reversed scroll origin), since a scrolled box may read differently after it.
@task_item n=16 part=needs
The lint's link: where a lint finding names an element, this read tells its cause; whether the lint answer points to it is left to P4.
@task_item n=17 part=needs
The capability statement names the new capability and its pin check holds it against the driver's verbs.
@task_standing n=1
convention: every new stand check addresses elements by the stand's stable ids as attributes alone, never by position or text
home: surface-desktop-native
@task_standing n=2
convention: every new stand check runs headless at the pinned viewport and boots the stand lint-clean
home: surface-desktop-native
@task_standing n=3
convention: every new session verb is listed in MCP tools/list, described in the help answer, and recordable as a replayable check file
home: surface-cli
@task_standing n=4
convention: every new log line or span field carries no caller input and no answer content
home: surface-cli
@task_item n=1 part=not_needed
The ShellProvider window chrome controls (drag, maximise, minimise, decorations, close): the read touches no window.
@task_item n=2 part=not_needed
The cold-agent run pipe and the agent-run test contract: their usage, exits, stdout and record directories are not changed by a new read.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: benchmarks the read does not touch.
@task_item n=4 part=not_needed
Stand behaviours of particular tasks — the CRUD list keyboard region, the flight type select at boot, the load page's network provider, the stand accent colour: the read observes placement, not these behaviours.
@task_item n=5 part=not_needed
The absence of arbitrary index.html templates and the fixed document skeleton: the read builds no document.
@task_item n=6 part=not_needed
The screenshot answer and its MCP screenshot result: the read adds no image.
@task_item n=7 part=not_needed
Canvas virtual-node production and drag: they stay in 0.4.0; only the answer's shape is reserved.
