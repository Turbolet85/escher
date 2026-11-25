@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas custom widget, each with an author key, a role from a closed set, a non-empty name and a region inside the canvas's content box.
A declaration that breaks a rule is refused with no node and no panic; two nodes of one canvas with the same key are refused.
Each taken node gets the reserved virtual node id form, the canvas element's stable id, the virtual key separator, then the key, through the existing virtual node id function.
The declared nodes, validated and with their ids, are readable in process; nothing new crosses a carrier, and the chunk claims no capability.
Every fork (the declaration's form, the names' crossing, the role set, where the declaration lives, the region's frame) is written to the founder-forks evidence and the phase stops at P4 with no default.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id component in the Dioxus DOM bridge — the virtual node id function and the virtual key separator that spell the reserved virtual node form
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget's Widget trait in blitz-dom nodes and widgets, behind the custom-widget feature, with its commented accessibility tree hook for multiple nodes per widget
@task_item n=1 part=needs
The Widget trait of blitz-dom's custom widget: a canvas element whose content the app paints from Rust is where virtual nodes are declared; whether the declaration lives on the engine Widget trait or in the Dioxus renderer is a fork.
@task_item n=2 part=needs
The author key of a virtual node: non-empty and holding no slash, so that it reads as an author key in the stable element id tiers.
@task_item n=3 part=needs
The reserved virtual node id form: owner stable id, the virtual key separator, then the key, produced only through the virtual node id function of the stable element ids and the Dioxus bridge; nothing produces such a node before this chunk.
@task_item n=4 part=needs
A virtual node id holds no process-local value and no NodeId: its owner part is the canvas element's stable id, and tier distinctness between the virtual form and the other id tiers holds.
@task_item n=5 part=needs
The virtual node role: a closed set of roles; the set itself is a fork written with the others.
@task_item n=6 part=needs
The virtual node name: non-empty, and it is text the app author wrote; authored text on a ratified carrier is a widening of the session boundary, so whether a name or key crosses any carrier is the names' crossing fork.
@task_item n=7 part=needs
The virtual node region: inside the canvas's content box and held in the content-box frame, the frame the custom widget paints in and hears pointer events in; the region's frame and its rule are a fork.
@task_item n=8 part=needs
Validation of a declaration is on untrusted input: a declaration breaking any rule is not taken, no node stands for it, and no code path panics; any expect or unwrap on that path needs its justification.
@task_item n=9 part=needs
Duplicate keys within one canvas are refused so that one id names one node.
@task_item n=10 part=needs
An in-process reader of what a canvas declares now, validated and with ids; it adds no driver verb, argument kind, refusal cause, session error, exit status, carrier, log field or span, and no new reading on the session socket, the command line, MCP or a kept check.
@task_item n=11 part=needs
The declaration's form fork: a list given at each paint, or callbacks with invalidation; its cost measured against the changed set and the paint path the custom widget already takes.
@task_item n=12 part=needs
The custom-widget feature gating: the declaration and its reader compile only where the custom-widget feature and its hit-testing gating already allow.
@task_item n=13 part=needs
The AccessKit premise to close at P3: AccessKit carries nodes with no DOM behind them, with bounds and a transform on the parent, read as the intent states it.
@task_item n=14 part=needs
Fork additions in the engine crate live in functions of escher's own and a direct engine change is flagged upstreamable.
@task_item n=15 part=needs
The founder-forks evidence: each fork states what crosses, the rule it meets, each option's measured cost, the dependences between forks, whose it is, and the most conservative option that still builds the entry; no raw log with a temp or home path goes into it.
@task_item n=16 part=needs
Operator conditions on the plan: a probe worktree stands outside the build target directory and no step deletes it; a guard's refusal of a write is a stop; a control enters the plan with its red reading.
@task_item n=17 part=needs
The capability statement is not changed: the chunk claims no capability and v040-11 stays pooled across its five entries.
@task_item n=18 part=needs
Two carried items ride the wrap only, written by no plan step that edits code: the security summary's OS drag-and-drop input source and dropped-file reader gaps, written by the leaf pass, and the no-screenshot-step test decision's one-host clause against the cross-OS canvas shot digests, carried by the wrap's report.
@task_standing n=1
convention: each new unit test sits at the end of its source file, with a descriptive name, asserting its fixture's condition first
home: conventions
@task_standing n=2
convention: each new public item and module carries its docs, rustdoc warnings being errors
home: conventions
@task_standing n=3
convention: each new expect or unwrap carries its justification and each unsafe block its annotation
home: conventions
@task_standing n=4
convention: each new integration test file in blitz-tests holds one behaviour and is one test target, in both layout modes where layout is read
home: conventions
@task_standing n=5
convention: new code passes the fmt and clippy gates and keeps its crate's coverage floor
home: inherited-defaults
@task_item n=1 part=not_needed
The WPT runner, its result kinds, statuses, outputs and CI artifacts: no web platform test is touched.
@task_item n=2 part=not_needed
The browser app, its history store, about pages, toolbar and persistence.
@task_item n=3 part=not_needed
Networking, the net provider, script fetching and the Boa JavaScript runtime with its globals.
@task_item n=4 part=not_needed
The select list, form controls, form submission and range input rules.
@task_item n=5 part=not_needed
The UI graph, the actions verb, handler coverage, layout explained and exploration: owned by the later entry that brings virtual nodes into every read.
@task_item n=6 part=not_needed
The keyboard path into a canvas, key names and keyboard activation: owned by the canvas keyboard path entry.
@task_item n=7 part=not_needed
The Circle Drawer stand's circles and draw commands grouped by node: owned by a later entry.
@task_item n=8 part=not_needed
The cold-agent and agent-run scripts, their events, verdicts and exit codes.
@task_item n=9 part=not_needed
The lint verb's reason keys and findings, including the canvas label truncation lint.
@task_item n=10 part=not_needed
CSS layout rules (flex, grid, tables, writing modes, scrolling primitives) and text decoration.
