@carried n=1 label=architecture/established-decisions fact=fct-apdputpm
why: The probe's reading for the founder must name this established hazard with its measured mechanism and coordinates, re-verified at HEAD, and must keep it standing as "no engine change" until the founder rules on the fix.
quote: A measured hazard of a task spawned from a key handler, no engine change: such a task that writes a signal is first polled inside `render_immediate`, which holds the document's `RefCell`, and a `MountedData` call in it panics at `events.rs:162` (`RefCell already borrowed`), so Cells calls its handles synchronously. Any app meets it; measured headlessly through the harness's `poll(None)`, the windowed shell's same lines read and not run (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/readings.md).
@carried n=2 label=architecture fact=fct-7in62t3p
why: Every option for the MountedData fix brought to the founder must be framed under the established pattern for a panic fix in the engine: a direct engine change for every document, ratified, flagged upstreamable with nothing sent, its merge surface owned by the Upstream sync line.
quote: the key-press panic's fix, `BaseDocument::deallocate_anonymous_block` setting `layout_parent` to `None` on each child still naming the block, is one more direct engine change for every document, ratified, flagged upstreamable with nothing sent upstream, its merge surface a line owned by Upstream sync ahead of determinism and exploration.
@carried n=3 label=architecture/established-decisions fact=fct-nl4nv5ny
why: The probe's node-to-component and state-to-nodes readings must be taken through nodes the bridge still maps, since a node removed by a re-render no longer resolves.
quote: a node the Dioxus bridge removes is dropped with its descendants and their `element_id_mapping` / `node_id_mapping` entries are cleared, so a re-render-removed node no longer resolves
@carried n=4 label=architecture fact=fct-nb3g6bqn
why: The probe must leave the driver session's crossing exactly as established: no new carrier or exit, no NodeId crossing, no event, subscriber or env read added.
quote: ids, names, masked values, snapshot text, diffs, shots and lint findings cross, never a `NodeId`; the library depends on no app, emits no event, installs no subscriber and reads no env var; a hosted session expires when idle; a new carrier or exit widens the boundary.
@binding n=1
**MountedData hazard ↔ the scope's reading for the founder and its folded CARRY**: the reading names the hazard and its mechanism as this decision states it, P3 re-verifying `events.rs:162` and `render_immediate` / `poll` at HEAD after the 2026-11-01 sync; the decision's "no engine change" stays as written unless the founder's fork chooses a fix (architecture/established-decisions fct-apdputpm)
@binding n=2
**Engine-change pattern ↔ the fork brought to the founder (P4)**: each fix option must say it is a direct engine change for every document that needs his ratification, is flagged upstreamable with nothing sent, and has its merge surface owned by the Upstream sync line. Whether the chosen fix lands in this chunk or in a chunk of its own stays part of the fork (architecture fct-7in62t3p)
@binding n=3
**Bridge removal ↔ the probe's links (1) and (4)**: a verdict that a component or reader is exposed for a node must be measured on nodes still in the bridge's mappings, never on a node a re-render removed (architecture/established-decisions fct-nl4nv5ny)
@binding n=4
**Driver session ↔ the scope's "Nothing crosses" boundary and the security and observability plans**: probe code adds no carrier, exit, event, subscriber or env read, and no `NodeId` crosses. A component name, handler or signal value the probe reads is printed, logged and carried by none of them (architecture fct-nb3g6bqn)
