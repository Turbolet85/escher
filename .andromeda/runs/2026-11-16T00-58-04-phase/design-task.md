@task_prose part=chunk
Builds one read of an element's placement and size by its stable id: where it sits and at what size (the bounds the snapshot already reads) and why.
The why is the chain that placed it: the parent's layout mode, the inputs that mode took from the computed style, the containing block, and any scroll or transform that moves it.
The read runs no step and changes nothing, like snapshot, lint, graph and actions; it reads what the engine already holds, with no second layout and no engine change.
The answer's placement kind is an open set that can name a box placed by the app inside a canvas, an owner element by its stable id; nothing in 0.3.0 produces that case.
Proven on the stand in both layout modes with minimal fixtures; checks of a scrolled case are re-read when the later Taffy take lands.
@task_file n=1 path=escher-driver/src/schema.rs mark=edits
what: the driver's schema, the verb table and its exit statuses — edits: the new read's verb entry, if the read is a driver verb
@task_file n=2 path=escher-driver/src/schema/spec.rs mark=edits
what: the schema's spec of each verb's arguments and answer — edits: the new read's spec, if the read is a driver verb
@task_file n=3 path=escher-driver/src/execute.rs mark=edits
what: the executor that runs each driver verb against the session — edits: the new read's execution, if the read is a driver verb
@task_file n=4 path=escher-driver/src/json.rs mark=edits
what: the written forms of each verb's answer, the JSON line an agent reads — edits: the new read's answer form
@task_file n=5 path=escher-driver/src/cli.rs mark=edits
what: the command line of escher-session, its commands and arguments — edits: the new read's command
@task_file n=6 path=escher-driver/src/mcp/ mark=edits
what: the MCP role's tool listing and tool results — edits: the new read's tool and its result
@task_file n=7 path=escher-driver/src/describe.rs mark=edits
what: the driver's self-description, the help an agent reads — edits: the new read's entry
@task_file n=8 path=docs/capabilities.md mark=edits
what: the capability statement, what the driver lets an agent do — edits: the new read's line
@task_file n=9 path=.github/scripts/test_capability_statement.py mark=edits
what: the check pinning the capability statement to the verbs the driver has — edits: its pin of the new read
@task_item n=1 part=needs
A new read of an element's placement and size, addressed by the element's stable id, answering its bounds as the snapshot reads them and the chain that placed it; it runs no step and changes nothing, the same as snapshot, lint, graph and actions.
@task_item n=2 part=needs
The answer names the parent's layout mode as a closed word (block, flex, grid, inline, absolute or fixed positioning, a float) and the inputs that mode took from computed style: sizing, margins and offsets, the parent's alignment and gaps.
@task_item n=3 part=needs
The answer names the containing block the element was placed in and any scroll offset or transform that moves it from where layout put it.
@task_item n=4 part=needs
The placement kind is not a closed set: the shape reserves a kind for a box placed by the app inside a canvas, naming its owner element by stable id, so 0.4.0 adds it with no contract change; this mirrors the UI graph's reserved owner///key id form.
@task_item n=5 part=needs
The read takes Taffy's resolved layout per node and the computed Stylo style already turned into the Taffy style; it runs no second layout, and an engine change, if one appears, is the founder's fork.
@task_item n=6 part=needs
The widening rule decides the answer's fields: closed words (layout modes, placement kinds) and numbers inside an already ratified carrier are not a widening; a new verb, an open value set, or any field carrying a page's style text as the author wrote it is the founder's fork and goes to founder-forks.md while he is away.
@task_item n=7 part=needs
If the read is a driver verb it is a new verb on every ratified carrier, as lint, graph and actions were: escher-session's command line commands and arguments, its answer line, the MCP role's tool listing and results, help, and the recording and replay of a kept check.
@task_item n=8 part=needs
The new verb's refusals and exit statuses are read from the schema's exit-status table, and its session errors carry the session error object with its remedy, on stderr and as the MCP error ending.
@task_item n=9 part=needs
The command span records the verb and, when refused, the cause alone; no span field holds the read's content or its size.
@task_item n=10 part=needs
The read's output is read by an agent, not a person: one line on stdout in the command line role and one tool result in the MCP role, in the same written forms the other read verbs use.
@task_item n=11 part=needs
The capability statement gains the new read and its pinned check is updated to the verb set the driver has.
@task_item n=12 part=needs
The stable id the read takes is the same element id the snapshot, the UI graph line and the actions line use; an unknown or stale id is refused the way those verbs refuse it.
@task_item n=13 part=needs
The link to lint: where a lint finding names an element, this read tells its cause; whether lint's answer points to it is left open.
@task_item n=14 part=needs
Proven on the stand in both layout modes, with minimal fixtures for a case the stand lacks: a flex child, a grid item, a positioned box, a scrolled box, a transformed box.
@task_item n=15 part=needs
Checks of a scrolled box's placement are marked to be re-read when Taffy moves past the scrollable-overflow change, which reverses the scroll origin for every document.
@task_item n=16 part=needs
The bounds the read answers are in the logical viewport (physical size over total scale), the same coordinates the snapshot's bounds use.
@task_standing n=1
convention: every new escher-session verb has its command, arguments and answer listed in help and the self-description, its exit statuses taken from the schema table
home: surface
@task_standing n=2
convention: every new MCP tool result follows the MCP role's result and error-ending form, one JSON result on stdout
home: surface
@task_standing n=3
convention: escher-session output carries no colour and gets no terminal test; it is read by agents
home: surface · surface-cli
@task_standing n=4
convention: every new test and fixture is built with system fonts so text lays out the same on every runner
home: typography
@task_item n=1 part=not_needed
Color palette rows: engine colours, color-scheme recascade and colour tokens; the read answers no colour.
@task_item n=2 part=not_needed
Iconography rows: cursor icons, cursor resolution and favicons; the read sets no cursor and reads no icon.
@task_item n=3 part=not_needed
Depth strategy rows on paint levels, stacking contexts and hit-testing at paint clips; the read reports placement from layout, it does not paint or hit-test.
@task_item n=4 part=not_needed
Motion rows in both motion areas: smooth scroll, scrollbar fades, select list motion and the virtual clock's advance; the read moves no clock and scrolls nothing, it only reports a scroll offset already there.
@task_item n=5 part=not_needed
Spacing rows on presentational hints (body margins, table cellspacing, select list placed in the viewport): these are engine behaviours the read observes, not rules it implements.
@task_item n=6 part=not_needed
Web-spa surface: WASM pages rendering into a host canvas; this is not the app-placed canvas case the answer reserves, and the chunk builds no canvas or virtual node.
@task_item n=7 part=not_needed
Brand identity, border radius, anti-patterns, self-validation protocol and design decisions log: no rows bind a driver read of layout.
