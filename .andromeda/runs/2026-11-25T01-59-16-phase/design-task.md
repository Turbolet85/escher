@task_prose part=chunk
Lets an app declare virtual nodes inside a canvas widget (an element the app paints from Rust through the custom widget): each node has an author key, a role from a closed set, a non-empty name and a region inside the canvas's content box.
A declaration that breaks a rule is not taken and nothing panics; each taken node gets the reserved id form, the canvas element's stable id, the virtual key separator, then the key.
The declared nodes, validated and with their ids, can be read in process; no carrier, verb, log field or session reading is added by this chunk.
Every operator fork (the declaration's form, the names' crossing, the role set, where the declaration lives, the region's frame) is written into the founder forks evidence with its cost and most conservative option, and the phase stops there with no default taken.
The two carried rows (the security summary's drag-and-drop gaps, the test plan's one-host screenshot clause) ride this entry to its wrap and are not written by hand.
@task_file n=1 path=packages/dioxus-native-dom/src/element_id.rs
what: the stable element id and its reserved virtual node id form — the virtual key separator and the virtual node id function a taken node's id is spelled through
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget trait an app paints a canvas through, with its commented accessibility tree note allowing multiple nodes per widget — one candidate home of the declaration
@task_file n=3 path=chunks/2026-11-25-virtual-nodes-declared/evidence/founder-forks.md mark=edits
what: the founder forks evidence — each fork with what crosses, the rule it meets, each option's measured cost, its dependences, whose it is and the most conservative option that still builds the entry
@task_item n=1 part=needs
The canvas widget (custom widget) as the owner of virtual nodes: a declaration attaches nodes to one canvas element and the canvas's stable id is the owner part of every node id.
@task_item n=2 part=needs
Validation of a declaration: author key non-empty and holding no slash, role from a closed set, name non-empty, region inside the canvas's content box; a declaration breaking any of these is not taken and no node stands for it.
@task_item n=3 part=needs
No panic path on untrusted input: a malformed, empty or oversized declaration from the app is refused quietly, never panics the engine or the renderer.
@task_item n=4 part=needs
The reserved virtual node id form: owner id, the virtual key separator, then the key, spelled only through the existing virtual node id function and matching the form the security plan reserves.
@task_item n=5 part=needs
Duplicate keys in one canvas are refused, so an id names one node only.
@task_item n=6 part=needs
The region is held in the canvas's content-box frame, the same frame the widget paints in and hears pointer events in (custom widget paint size and pointer frame).
@task_item n=7 part=needs
An in-process read of the declared nodes: what a canvas declares now, validated and with its ids.
@task_item n=8 part=needs
The declaration's form is a fork: a list given at each paint, or callbacks with invalidation; its cost is measured, not defaulted.
@task_item n=9 part=needs
The names' crossing is a fork: a node's name is app-authored text, and authored text on a ratified carrier (snapshot, actions, graph, layout, MCP results) is a widening by the founder's rule; this chunk crosses nothing onto a carrier and the fork's cost is measured against those carriers.
@task_item n=10 part=needs
The chunk adds no driver verb, argument kind, cause, session error, exit status, carrier, log field or span, and no new reading on the session socket, the command line, MCP or a kept check.
@task_item n=11 part=needs
Where the declaration lives (the engine's custom widget trait or the Dioxus renderer), the role set and the region's frame rule are further forks written beside the two named ones, each with whose it is and its most conservative option.
@task_item n=12 part=needs
Premises closed at P3: nothing produces a virtual node id yet; the widget's accessibility tree is a commented note; AccessKit carries nodes with no DOM behind them, with bounds and a transform on the parent.
@task_item n=13 part=needs
Evidence hygiene: no raw log with a temp or home path enters evidence; a probe worktree lives outside the build target directory and no plan step deletes it; a guard refusing a write is a stop; a control enters the plan with its red reading.
@task_item n=14 part=needs
The two carried rows are named in the plan so the wrap carries them: the security summary's drag-and-drop input source and unrestricted dropped-file reader gaps (written by the wrap's leaf pass) and the test plan's one-host screenshot repeatability clause against the cross-OS canvas shot digests (written by the wrap's report); neither adds a code step.
@task_item n=1 part=not_needed
Colour palette rows (engine colours, select list colours, colour scheme, base colour, Circle Drawer canvas colours): declaring nodes paints nothing and sets no colour.
@task_item n=2 part=not_needed
Typography rows (font faces, system fonts, font context, root-relative units, lang locale): a node's name is held, never laid out or shaped as text.
@task_item n=3 part=not_needed
Spacing rows (body margins, table cellspacing hints): no box model or presentational hint is touched.
@task_item n=4 part=not_needed
Motion rows (virtual clock, scroll behaviour, overlay scrollbar fades, select list motion): no animation, scroll or clock is reached.
@task_item n=5 part=not_needed
Iconography rows (cursor icons, cursor resolution, favicon): no cursor or icon is set.
@task_item n=6 part=not_needed
Select list rows (its engine overlay, events, gestures, chosen and highlighted rows, its accessibility): the select element is not touched.
@task_item n=7 part=not_needed
WPT runner rows (result colours, output hyperlinks, secondary text): the WPT runner is not touched.
@task_item n=8 part=not_needed
Shell provider and window rows (decorations, maximize, minimize, window drag, alpha mode) and the JS window object's sizes and device pixel ratio: no window or script surface is touched.
@task_item n=9 part=not_needed
escher-session task stands (counter, crud, cells, timer, flight-booker, load, order, unknown task): no task stand is added or changed; the Circle Drawer's circles as nodes belong to a later entry.
@task_item n=10 part=not_needed
Mobile targets, WASM web-spa canvas sizing and the default user-agent sheet: no platform or stylesheet path is changed.
