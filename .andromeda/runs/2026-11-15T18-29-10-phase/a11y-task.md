@task_prose part=chunk
Moves the workspace's taffy layout dependency from the crates.io registry 0.15.1 to upstream's git pin a3997d55 (the rev upstream blitz names at ea07ee716), so the tree carries Taffy #1303, the caller-defined float id, and the Taffy content of upstream's left-out chain.
Takes the blitz side of that chain as upstream wrote it: the inline layout call that passes the node's taffy id as place_floated_box's new first argument, and the code edits of the nine "Bump Taffy" commits; no other upstream pick.
The lock moves with it: one taffy package from a git source in place of the registry one, every other package only as the move forces.
Before any plan, research re-measures the move in a trial worktree: whether the workspace builds; the fast, clippy, audit, msrv (rust 1.91), doc and build legs, the incremental_oracle, the stand checks and the WPT runner's build; the lock delta; and the wasm example's own taffy resolution.
If the move cannot be taken cleanly the phase stops with the measurement; a fork that widens a boundary or needs a master rule lifted beyond the Taffy-only exception goes to the forks file and stops the phase.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest, the home of the workspace's dependency set and its no-git-dependency boundary with the Taffy-only exception — edits: the taffy dependency entry
@task_file n=2 path=Cargo.lock mark=edits
what: the workspace lock every build, test and CI leg reads under --locked — edits: the taffy package entry and its source
@task_file n=3 path=packages/blitz-dom/src/layout/inline/layout.rs mark=edits
what: the engine's inline layout component, which places floated boxes through Taffy — edits: the place_floated_box call, its new first argument the node's taffy id
@task_file n=4 path=examples/wasm_hello/Cargo.toml mark=re-reads
what: the wasm leg, a standalone workspace with its own lock outside the root's --locked, whose taffy resolution is read on its own
@task_file n=5 path=deny.toml mark=re-reads
what: the audit leg's advisories and licences checks, kept as they stand and re-run over the moved lock
@task_item n=1 part=needs
A new layout engine version changes the computed layout of every document, remote pages included: every box position and size the engine reports may move, and the chunk must measure that change, not assume it away.
@task_item n=2 part=needs
Floats are now placed under a caller-defined float id (#1303): the inline layout passes the node's taffy id when it places a floated box, so floated content's placement is a behaviour the take changes.
@task_item n=3 part=needs
Bounds the snapshot and the accessibility tree report for a node, a closed select's options, an open list's rows and a multiple select's rows and groups are read from layout, so the move reaches every bound those readings carry.
@task_item n=4 part=needs
The open select list's rows taking one height, and the shot of an open list's row, rest on layout the move may shift; their checks must still hold after the move.
@task_item n=5 part=needs
Hit-testing and the driver's synthetic click at a node's centre, its covered-click refusal and a click in a shown scrollbar band read layout boxes; the move must leave those checks passing.
@task_item n=6 part=needs
Content with display none stays out of the accessibility tree; the layout engine's handling of display none comes with the new Taffy and the exclusion must still hold.
@task_item n=7 part=needs
The stand checks are re-run as part of the measurement: the stand's contrast assertions measured from computed styles, the stand reading no lint finding at boot, the shared focus-order table, all repeated three rounds.
@task_item n=8 part=needs
The lint's contrast finding carries bounds read from layout; its bounds may move with the new layout and the finding's shape must stay as it stands.
@task_item n=9 part=needs
The accessibility test leg in the slow tier and the accessibility checks that gate CI must stay green on the moved tree; a red leg with no in-scope fix is a stop for the phase.
@task_item n=10 part=needs
Accessibility is a default-on feature and the session library always enables it: the build, clippy and doc legs measured on the moved tree are the feature-on builds, so the AccessKit tree's code path is compiled and checked under the new Taffy.
@task_item n=11 part=needs
The lock may move only taffy and what the move forces: the AccessKit crates and the platform adapter the accessibility tree rests on take no version change the move does not force, and any such forced change is read in the measurement.
@task_item n=12 part=needs
A hidpi or zoom change reshapes text and the viewport carries scheme and zoom; text and box layout read under the new Taffy at every zoom the stand checks use must still meet their checks.
@task_item n=13 part=needs
Whether each layout change the chain brings is its own ratification, or the founder's Taffy-only lifting stands as the ruling for them, is a fork the measurement names and does not settle.
@task_item n=1 part=not_needed
Role mapping by HTML-AAM, ARIA role, state and property reads, and accessible name precedence: they derive from attributes and elements, never from layout.
@task_item n=2 part=not_needed
Keyboard handling rows: range input keys, select list key handling and its input and change events, Tab order and focus movement, keyboard activation; the move changes no key or focus logic.
@task_item n=3 part=not_needed
The choice of AccessKit for the tree and of accesskit_xplat as platform adapter, the stand binaries building no adapter, and the adapter's creation order: the chunk changes no adapter or tree source.
@task_item n=4 part=not_needed
Cognitive accessibility rows (invalid state not by colour or class alone): no layout or dependency reach.
@task_item n=5 part=not_needed
Driver and MCP answer shapes: the actions answer, handler coverage, MCP tool results, the snapshot text form, the snapshot diff, served node id separator, the UI graph state registration and the Dioxus reached-handler record; the move adds or changes no field.
@task_item n=6 part=not_needed
Password masking, the password caret and value reveal rules: untouched by a layout dependency move.
