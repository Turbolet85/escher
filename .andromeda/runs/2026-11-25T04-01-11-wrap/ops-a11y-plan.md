@row_set n=1 row=decision/snapshot-model-single-source cell=rule seen=2 detector=none
value: The snapshot's text form, parts, diff, MCP results, node-key help and screenshot read the snapshot model and nothing else; the canvas read, which reads one canvas element's draw commands, and the virtual-node read, which reads one canvas's declared nodes, are no readers of the snapshot model.
basis: fct-k6v6mfx2 (a11y-plan §7 change decision/snapshot-model-single-source), fct-jw55vkbf (the crossing: no carrier gains a reading)
because: The chunk adds an in-process read of a canvas's declared virtual nodes that feeds no carrier, so the row naming what is no reader of the snapshot model must name it beside the canvas read.
@row_add n=1 row=decision/canvas-declared-virtual-nodes area=screen-reader-support fact=F1 detector=none
name: Virtual nodes declared by a canvas
rule: A canvas widget may declare virtual nodes, each a key, an AccessKit role, a name and a region in its content box; the Dioxus document reads them in process, validated in declaration order, each taken node given the id {owner}///{key}.
state: provisional
basis: fct-wkaf2evl (Symbols / APIs and Behaviour), fct-ow7mqtxk (the reserved form now produced in process)
because: The chunk gives a canvas a way to declare keyed, named, role-bearing regions, a decision future screen-reader work must build on, resting on a delegate's answer.
@row_add n=2 row=absence/virtual-nodes-on-carriers area=screen-reader-support fact=F2 detector=none
capability: Declared virtual nodes on the accessibility tree, platform adapter, snapshot, diff or UI graph, or taking a click
basis: fct-jw55vkbf (the crossing: no carrier gains a reading), fct-nq55p7ac (card row 1, the names' crossing stays the founder's fork)
because: No declared virtual node reaches any carrier or takes a click, a whole capability the project does not yet have.
@row_add n=3 row=decision/virtual-node-role-is-accesskit-role area=screen-reader-support fact=F3 detector=none
name: Virtual node role is AccessKit's own role
rule: A declared virtual node's role is AccessKit's own role as the app states it, with no second role mapping.
state: provisional
basis: fct-wkaf2evl (VirtualNodeDeclaration role: accesskit::Role), fct-k6v6mfx2 (the role decision, PROVISIONAL on F4 A)
because: The chunk fixes the declared node's role as the tree's own type rather than a mapping of its own, a delegate's answer future work must respect.
@row_add n=4 row=decision/virtual-node-declaration-validation area=screen-reader-support fact=F4 detector=none
name: Virtual-node declarations validated with closed refusals
rule: A virtual-node declaration is refused with a closed reason and its index alone when its key is empty, holds a slash or repeats an earlier one, the first kept, its name is empty or its region leaves the content box; a stale or non-canvas node reads no canvas, an unlaid one not laid out, no panic.
state: provisional
basis: fct-wkaf2evl (Behaviour: the closed refusals and unread reasons), fct-nq55p7ac (duplicate first-kept, a delegate's answer)
because: The chunk adds the validation contract of a declaration, with its duplicate rule resting on a delegate's answer.
@row_add n=5 row=decision/virtual-node-declaration-marks-nothing area=a11y-strategy fact=F5 detector=none
name: Virtual-node declaration change marks nothing
rule: A change to a canvas's virtual-node declarations writes no mark in the engine's changed set and signals no reader, so no accessibility tree refresh follows from it.
why: The mark and the invalidation are left to the first reader that refreshes.
state: provisional
basis: fct-wkaf2evl (Behaviour: no changed-set mark, no invalidation signal), fct-k6v6mfx2 (PROVISIONAL on F1 B′, F6 A)
because: The chunk decides a declaration change marks and signals nothing, which bounds the tree's refresh on change, on a delegate's answer.
