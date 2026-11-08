@task_prose part=chunk
Adds one read-only driver read of the current screen's actionable nodes (v030-06), dispatching nothing, like snapshot, lint and graph.
For each node it gives the stable id, the owner element where the handler sits on another element, the verbs it takes, and per verb whether a call is accepted now or the one refusal cause (disabled, off-screen, covered) the call would return.
The node model is the UI graph's GraphNode with an optional owner, so a select's handler serves its options and groups, and nothing is keyed on a DOM NodeId; verbs come from the schema's verb table VERBS with no closed list of verb names.
Refusal reasons come from the same path the executor uses (action_point: disabled, then off-screen, then covered), so they match a real call. The actionable-key check (author key vs positional path) folds in if it fits.
It is proven on the stand in both layout modes, with a minimal fixture where the stand lacks a covered, off-screen or owner-served case.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the verb table VERBS
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: VerbSpec
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: action_point, the shared refusal path
@task_file n=4 path=packages/escher-driver/src/json.rs
@task_file n=5 path=packages/escher-driver/src/cli.rs
@task_file n=6 path=packages/escher-driver/src/mcp
@task_file n=7 path=packages/escher-driver/src/recording.rs
@task_file n=8 path=packages/escher-driver/src/replay.rs
@task_file n=9 path=packages/escher-driver/src/describe.rs
@task_file n=10 path=docs/capabilities.md
@task_file n=11 path=.github/scripts/test_capability_statement.py
@task_file n=12 path=packages/dioxus-native-dom/src/graph.rs
@task_file n=13 path=packages/dioxus-native-dom/src/snapshot.rs
@task_file n=14 path=packages/dioxus-native-dom/src/actionable.rs
@task_item n=1 part=needs
The cli surface's output rule: the driver's command line and its mcp role print only JSON on stdout, one line per command or newline-delimited JSON-RPC lines, with no colour or ESC byte and no terminal test; the exit status carries the class (0 accepted, 1 refused, 2 usage, 3 session error). The new read's answer must follow it.
@task_item n=2 part=needs
The cli surface's graph read (the cli-graph subject) and the pending unpaged-graph subject: the new read is a sibling of graph and reads the same UI graph, so how graph's answer is shaped and paged on the command line applies to it.
@task_item n=3 part=needs
The cli surface's component patterns for the driver's agent-read outputs, which cover the snapshot, lint and graph reads the new read sits beside.
@task_item n=4 part=needs
Hit-testing as the depth strategy states it: a point outside a paint-clipping box (overflow, or contain including paint, root excepted) reaches none of its children, and an open select list's row is answered before the walk. This decides the covered cause the read reports through action_point, and why an owner-served select row is not clipped.
@task_item n=5 part=needs
The open select list's rows as owner-served sub-targets: a select's handler serves its options and groups, and the rows are answered and scrolled by the select list itself (SelectListView). The read lists them under their select owner.
@task_item n=6 part=needs
Into-view scrolling as the motion section states it: the driver's scroll is instant in every box and the viewport, and into view on an open select row moves only that list's scroll. This governs the off-screen cause and keeps the read from scrolling or moving anything.
@task_item n=7 part=needs
The harness clock rule: click, type and press leave the VirtualClock where it was, and only advance moves it. The read dispatches nothing, so it must not move the clock or wait on settle.
@task_item n=8 part=needs
The disabled state of a target, the first cause action_point checks, which the read reports per verb; the select list's DISABLED_TEXT row colour is how a disabled option is painted, not how it is detected.
@task_item n=9 part=needs
The cursor rule, Pointer inside links and Text for text inputs, only as a sign of which elements a person sees as actionable; the actionable set itself comes from handlers, owner-served nodes and the unkeyed_actionable criteria.
@task_item n=10 part=needs
The screenshot fonts-unavailable refusal and the system-fonts build feature: the stand proof in both layout modes depends on text measuring non-zero, otherwise off-screen and covered checks pass vacuously.
@task_standing n=1
convention: Every output of an agent-read driver surface is JSON only on stdout, one line per command, no colour or ESC byte, with the exit status giving the class; each new command or answer the chunk adds follows it.
home: Surface: cli, Component Patterns (body and file surface)
@task_standing n=2
convention: A test that depends on text layout runs with blitz-dom's system-fonts feature, so font-dependent assertions do not pass vacuously; each new stand or fixture test follows it.
home: Typography, Loading (body)
@task_standing n=3
convention: A new example or test fixture is recorded under its section's Example fixtures list.
home: Example fixtures entries of the body sections (file depth-strategy, file motion)
@task_item n=1 part=not_needed
The colour palette, core colours, surface scale, text hierarchy, semantic colours and border progression: the read reports ids, verbs and causes, not paint.
@task_item n=2 part=not_needed
Typography's type scale and engine text mapping, beyond the system-fonts rule: the chunk sets no type.
@task_item n=3 part=not_needed
Spacing and border radius values and their absent scales: the chunk adds no layout or styling.
@task_item n=4 part=not_needed
Iconography: browser toolbar icons, IconButton, favicons and size grid; the chunk does not touch the browser app.
@task_item n=5 part=not_needed
The desktop-native surface: titlebar, custom widget, devtools overlay, window and stack; the read is a driver command, not a window feature.
@task_item n=6 part=not_needed
The mobile-native surface: urlbar, back button, capture, Android and iOS builds.
@task_item n=7 part=not_needed
The web-spa surface and its canvas: canvas virtual nodes stay in 0.4.0, and the chunk produces none.
@task_item n=8 part=not_needed
The WPT runner and the paint_bench and screenshot examples as command-line binaries using owo-colors: they are not the driver's agent-read surface.
@task_item n=9 part=not_needed
Animation runtime and reduced motion, other than the clock rule: the read runs no animation and no settle.
@task_item n=10 part=not_needed
Brand identity, anti-patterns, self-validation protocol and the design decisions log: they hold no recorded intent.
