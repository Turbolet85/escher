@task_prose part=chunk
Builds one read of a named element's placement and size, by its stable id: where it sits and at what size (the bounds the snapshot already reads), and why — the layout mode of its parent, the computed-style inputs that mode took, the containing block, and any scroll or transform that moves it.
The read runs no step and changes nothing, like snapshot, lint, graph and actions, and is taken from what the engine already holds (Taffy's resolved layout per node and the Stylo style converted into the Taffy style) — no second layout, no engine change.
The answer's placement kind is not a closed set: its shape reserves the case «placed by the app inside canvas X», naming an owner element by its stable id, so 0.4.0 needs no contract change; nothing in 0.3.0 produces that case.
If the read is a driver verb it is added to every ratified carrier as the precedents lint, graph and actions were; whether any of its fields widen the boundary is P4's to settle, and a fork that is the founder's goes to founder-forks.md.
Proven on the stand in both layout modes with a minimal fixture where the stand lacks a case; its checks of a scrolled case are re-read when Taffy is later taken past #1372.
@task_file n=1 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command and refusal schema — the validated verb set and its argument kinds — edits: the new read verb's entry, if the read is a verb
@task_file n=2 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver verb table and each verb's specification — edits: the new verb's table entry, if the read is a verb
@task_file n=3 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's executor that runs a validated command against the session's document — edits: the new read's execution, if the read is a verb
@task_file n=4 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's own hand-written JSON writer for answers (the written forms) — edits: the written form of the placement answer
@task_file n=5 path=packages/escher-driver/src/cli.rs mark=edits
what: the escher-session command line — its verbs, flags and usage — edits: the new verb's command-line form, if the read is a verb
@task_file n=6 path=packages/escher-driver/src/mcp/listing.rs mark=edits
what: the mcp role's tool listing — edits: the new verb's tool entry, if the read is a verb
@task_file n=7 path=packages/escher-driver/src/mcp/result.rs mark=edits
what: the mcp role's tool call results — edits: the new verb's result, if the read is a verb
@task_file n=8 path=packages/escher-driver/src/describe.rs mark=edits
what: the one self-description shared by the command line and MCP — edits: the new verb's description, if the read is a verb
@task_file n=9 path=docs/capabilities.md mark=edits
what: the capability statement, held to the driver schema — edits: the new capability's line, if the read is a verb
@task_file n=10 path=.github/scripts/test_capability_statement.py mark=edits
what: the CI check that pins the capability statement to the driver schema — edits: the pin for the new verb, if the read is a verb
@task_item n=1 part=needs
A new driver read (if a verb) on the validated verb set: the verb set is widened only by ratification, a new verb on every ratified carrier is a boundary widening, and a new verb is the founder's fork while he is away — it goes to founder-forks.md and the step stops.
@task_item n=2 part=needs
The read's target is an element named by its stable element id, as the other id-taking verbs take it; no NodeId or process-local value crosses a carrier, and an unknown or stale id is refused with a cause from the closed refusal-cause set and the per-verb target check.
@task_item n=3 part=needs
The answer's placement and size: the bounds the snapshot already reads for that element (node bounds, scrolled box bounds, transformed element bounds), so the read and the snapshot agree on where and how large an element is.
@task_item n=4 part=needs
The answer's why: the parent's layout mode as closed words (block, flex, grid, inline, absolute or fixed positioning, a float), the computed-style inputs that mode took (sizing, margins and offsets, the parent's alignment and gaps), the containing block, and any scroll or transform moving the element from where layout put it.
@task_item n=5 part=needs
The answer's placement kind is an open value set: it must be able to name «placed by the app inside canvas X» with an owner element by stable id, reserved the way the UI graph and stable ids reserved the virtual node id form `{owner}///{key}`; no 0.3.0 code produces that case.
@task_item n=6 part=needs
The widening rule decides each field: a new field of a closed value set inside a ratified carrier (layout modes, placement kinds, numbers) is no widening; a field carrying a page's authored style text (a length as written, a property's value), an open value set, a new carrier or exit still is, and is the founder's fork.
@task_item n=7 part=needs
The read changes nothing and runs no step, as the snapshot, lint, graph and actions verbs' side-effect contracts state; it moves no clock, waits on no load and settles nothing.
@task_item n=8 part=needs
The read comes from the engine's existing layout output: Taffy's resolved layout per node and the Stylo computed style that the stylo_taffy conversion turned into the Taffy style (the blitz-dom layout exports and Taffy traits); no second layout, and any engine change that appears is the founder's fork under the engine-change rule.
@task_item n=9 part=needs
The engine's existing layout mappings the explanation must report truthfully: tables laid out as Taffy grid, display table as grid, overflow auto laid out as scroll, flex-start and flex-end patched to flow, normal alignment mapped to Taffy normal, gap normal as zero, sizing keywords on min and max as auto, text nodes never laid out individually, anonymous blocks only in layout children.
@task_item n=10 part=needs
Scroll and transform readings the answer must stay consistent with: absolute position subtracting containing-block (ancestor) scrolls, the client rect being the box's own and ignoring its own scroll, the visible region and client rect read through transforms, and z-axis rotate applied as planar.
@task_item n=11 part=needs
The answer is written by the driver's own hand-written JSON writer with its string escaping; no serde_json type is in the driver's public API, and the answer stays inside the MCP result bound and the session socket's answer bound.
@task_item n=12 part=needs
The read's crossing on every carrier, if it is a verb: the command line's verbs and usage, the mcp role's tool listing, tool results and self-description, the session socket's call and answer, and the kept check's recording and replay (kept verbs, answer, digest); one self-description serves the command line and MCP.
@task_item n=13 part=needs
The capability statement is held to the driver schema and gains the new verb with its pin; the root README's count of driver commands an MCP client gets as tools follows the verb table.
@task_item n=14 part=needs
The command span records verb and, refused, cause alone; no field holds the read's content or its size, and the driver logs no content and opens spans only.
@task_item n=15 part=needs
The lint link: where a lint finding names an element, this read is what tells its cause; whether the lint's answer points to it or the two stay apart is P4's to decide, and the lint's findings and reason keys are otherwise unchanged.
@task_item n=16 part=needs
Proof on the seven_guis headless stand in both layout modes, with a minimal fixture where the stand lacks a case (a flex child, a grid item, a positioned box, a scrolled or transformed box); stand readings are identical on every CI OS and the stand boots offline.
@task_item n=17 part=needs
The read's checks of a scrolled case are flagged to be re-read when Taffy is later taken past #1372, whose reversed scroll origin may change a scrolled box's reading.
@task_standing n=1
convention: every new scenario test runs in both layout modes, and both modes must give identical layouts
home: conventions
@task_standing n=2
convention: a new test asserts its fixture's precondition first, with numbered cases in a feature fixture and relative URLs between fixtures
home: conventions
@task_standing n=3
convention: new tests carry descriptive names, one behaviour per integration test file, and unit tests sit at the end of their source file or in path children of their driver module
home: conventions · existing-scopes
@task_standing n=4
convention: every new expect or unwrap carries its justification, clippy allows stay local, and rustdoc warnings are errors
home: conventions
@task_standing n=5
convention: every new module and public item is documented, with spec URLs cited where the code implements a CSS layout rule
home: conventions
@task_standing n=6
convention: a new stand check keeps its shared helpers in non-target modules and its readings identical on every CI OS
home: standard-contracts
@task_item n=1 part=not_needed
The browser app, its history, bookmarks, persistence and about pages: the read lives in the driver and touches no browser chrome.
@task_item n=2 part=not_needed
The WPT runner, its result kinds, statuses, outputs and CI artifacts: the read is proven on the stand, not by WPT.
@task_item n=3 part=not_needed
Painting and rendering — the paint scene, renderers, gradients, borders, masking, screenshots and their encoding: the read reports layout, it paints nothing.
@task_item n=4 part=not_needed
The open select list machinery, select choice paths and select keyboard and pointer default actions: the read explains placement and drives no control.
@task_item n=5 part=not_needed
Page scripting: Boa, the script globals, script fetching and JS microtask or timer rules — the read runs no script.
@task_item n=6 part=not_needed
Networking and loading: the net provider, HTTP cache, per-host caps, stylesheet and image fetching — the read issues no request.
@task_item n=7 part=not_needed
The acting verbs' input dispatch — click, type, press, scroll, key names, event dispatch order and focus rules — since the read acts on nothing.
@task_item n=8 part=not_needed
The platform accessibility adapter and AccessKit tree refresh: the read neither builds nor refreshes the accessibility tree.
@task_item n=9 part=not_needed
Canvas, virtual-node production and the drag verb: they stay in 0.4.0; only the answer's shape is kept open for the canvas case.
@task_item n=10 part=not_needed
The entries «Errors with the action» and «A component in isolation», and the take of Taffy past #1372 itself: each is its own entry.
@task_item n=11 part=not_needed
The cold-agent harness, its verdict, prompt and model-provider rows: the chunk adds no cold-agent run.
@task_item n=12 part=not_needed
Windowed shell concerns — winit, redraw requests, IME, platform quirks, Android and iOS entries, wasm builds: the read runs headless.
