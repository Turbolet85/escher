@task_prose part=chunk
Builds one read of a named element's placement and size, by its stable id: where it sits and at what size, and why — its parent's layout mode, the computed-style inputs that mode took, its containing block, and any scroll or transform that moves it.
The read runs no step and changes nothing, like snapshot, lint, graph and actions; it reads only what the engine already holds (Taffy's resolved layout and the computed Stylo style), with no second layout and no engine change.
The answer's placement kind is an open set: its shape reserves the case placed by the app inside a canvas, naming the owner element by its stable id, so 0.4.0 needs no contract change; nothing in 0.3.0 produces that case.
If the read is a driver verb it is added on every ratified carrier, as lint, graph and actions were; whether it is a verb, and which answer fields it carries, is P4's to settle under the widening rule, a fork that is the founder's going to the founder-forks list.
Proven on the stand in both layout modes, with minimal fixtures for a flex child, a grid item, a positioned box and a scrolled or transformed one; its scrolled-case checks are re-read when Taffy moves past #1372.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the closed driver verb schema, the one validation gate every carrier's call passes — edits: the verb set, if the read is a driver verb
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver verb specs and their argument kinds — edits: the read's verb spec, if it is a driver verb
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's executor, where each verb's read or action runs against the document — edits: the read's execution
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the written forms of the driver's answers, the answer that crosses every ratified carrier — edits: the read's answer form
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line, its closed argv reader and its result line — edits: the read's verb on the command line
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the MCP server's tool listing, fixed program text — edits: the read's tool entry
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the MCP server's tool results, bounded in size — edits: the read's result
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's self-description of its verbs — edits: the read's entry
@task_file n=9 path=docs/capabilities.md mark=edits
what: the committed capability statement of what the driver offers across the MCP crossing — edits: the read's capability
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the pin that holds the committed capability statement to the driver's verbs
@task_item n=1 part=needs
The new read itself: a read-only verb-like read that runs no step, moves no session state and changes nothing, as the actions, graph and lint reads do.
@task_item n=2 part=needs
The element is named by its stable element id: the id is looked up anew on each call, an unknown, stale or detached one is refused as stale or not-found in the fixed refusal order, and no NodeId or other process-local handle crosses with the answer or is kept between calls.
@task_item n=3 part=needs
The answer's content: bounds (as the snapshot already reads them), the parent's layout mode, the placement kind, the containing block, and scroll or transform offsets, as closed words and numbers; the answer holds no page content, no attribute text and no control value, and the value mask holds on every carrier it crosses.
@task_item n=4 part=needs
The boundary widening rule over the answer: a new field of a closed value set inside an already ratified carrier is no widening, while a new verb, a new carrier, an open value set, or a field carrying a page's style text as the author wrote it is the founder's fork, recorded and not decided while he is away.
@task_item n=5 part=needs
The placement kind is not a closed set: the shape reserves placed by the app inside canvas X, naming the owner by its stable id, as the stable element id reserved its virtual-node form; that reservation is itself a value set question under the widening rule.
@task_item n=6 part=needs
If the read is a driver verb: the closed driver verb schema and its single validation gate, one validation path for every carrier, the driver argument kinds (an id), and the new verb entering the driver verbs contract beside actions and graph as a boundary widening on every ratified carrier.
@task_item n=7 part=needs
The command span for the read records its verb and, when refused, its cause alone; no span field holds the read's content or its size, one command span per call, and driver outcomes and screen content never reach a log.
@task_item n=8 part=needs
Refusals of the read come from the closed refusal cause set, hold nothing a call supplied, are logged by cause name only, and no call-supplied input panics the driver or is echoed back.
@task_item n=9 part=needs
The MCP crossing of the read: a fixed-text listing entry, a result inside the MCP result size bound, the MCP server never printing or panicking, and the committed capability statement of the MCP crossing with its pin kept true.
@task_item n=10 part=needs
The command line and session socket crossing: the read's answer on the escher-session result line inside the driver command line answer bounds, the session socket replies carrying no attribute text and no NodeId.
@task_item n=11 part=needs
The recording and replay of a kept check that holds the read: the kept check keeps each answer whole, as it holds the graph and actions answers, the recorded check file is read strictly as untrusted, and each replayed step runs through the session.
@task_item n=12 part=needs
Read only from what the engine already holds — Taffy's final layout per node and the computed Stylo style mapped into Taffy's style — with no second layout; an engine change, should one appear, is the founder's fork under the engine-change rule.
@task_item n=13 part=needs
Elements without a resolved layout or style: a never-styled or unstyled node answers none or is refused rather than read, as the snapshot and the text extent reader treat it, and a stale node id never panics.
@task_item n=14 part=needs
Scroll and transform in the answer: the visible region read through transforms, a non-invertible transform, and programmatic scroll clamping bound how a moved box's offset is read; scrolled-case checks are re-read after Taffy moves past #1372, whose scroll origin change may alter them.
@task_item n=15 part=needs
The answer's size: whether the read is paged or unpaged, as the graph and actions answers are unpaged, and the answer bounds every carrier enforces on it.
@task_item n=16 part=needs
The lint's link: if P4 has a lint finding point to this read for its cause, the lint findings still hold no page content and still cross every ratified carrier as they do now.
@task_item n=17 part=needs
New tests prove the read on the stand in both layout modes, with minimal committed fixtures where the stand lacks a case; fixture state names are checked against the closed fixture table.
@task_standing n=1
convention: each new log line goes through escher's allowlisted stderr telemetry sink under an escher_ target, content-named fields withheld
home: logging-monitoring
@task_standing n=2
convention: each new committed check or fixture of the stand holds no user data and keeps any kept check owner-only and uncommitted
home: data-protection
@task_standing n=3
convention: each new driver code path is held by its crate's line coverage floor
home: input-validation
@task_item n=1 part=not_needed
Authentication and authorization: CI token permissions, publish job guards and the absence of application authentication are not touched by a read-only layout answer.
@task_item n=2 part=not_needed
Secret management: CI secrets, signing material, the operator's login and environment variables are not reached.
@task_item n=3 part=not_needed
Outbound network and fetch rules: script fetchers, file and data URL rules, CORS and same-origin absences, per-host limits, request timeouts and response size limits are not changed by this chunk.
@task_item n=4 part=not_needed
Browser app storage: history in sqlite, cookies, the HTTP cache, clipboard, file dialogs and TLS are not reached.
@task_item n=5 part=not_needed
Select list and form control behaviour: select list rows, keys, painting and closing, range input actions, password painting and multiple select rules are not changed; the read only reports placement.
@task_item n=6 part=not_needed
The WPT runner and its CI workflows, the apt-install script and the cold-agent harness are not changed by this chunk.
@task_item n=7 part=not_needed
Canvas, virtual-node production and drag: no virtual node is produced in 0.3.0; only the answer's shape reserves the canvas case.
@task_item n=8 part=not_needed
The Taffy move past #1372 and upstream sync takes are a separate entry; this chunk changes no engine dependency pin.
@task_item n=9 part=not_needed
Errors with the action and a component in isolation are their own entries; their refusal reworks and isolation fixtures are not this chunk's.
@task_item n=10 part=not_needed
HTML parsing, CSS parsing and serialization rules (invalid declarations, selectors, presentational hints, inline SVG output) are not changed; the read consumes computed style only.
