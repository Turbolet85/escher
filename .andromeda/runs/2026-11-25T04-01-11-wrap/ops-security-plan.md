@row_set n=1 row=contract/stable-element-id/virtual-node-form cell=promise seen=1 detector=none
value: The id grammar reserves `{owner}///{key}` for a virtual node, spelled for a non-empty key with no `/` and produced in process only for a canvas's taken declared nodes; no served id holds `///`, and the anchored path stays the only served id holding `//`.
basis: fct-ow7mqtxk
because: The reserved form moves from produced by nothing to produced in process by the declared-node read for a canvas's taken nodes, served by no carrier, so the contract must say where it is produced.
@row_set n=2 row=contract/stable-element-id/no-collision cell=promise seen=1 detector=none
value: An author key holds no `/`, an anchored path is the only served id holding `//`, and a component or document path has no empty segment, so no `id` attribute can make two ids equal, a key spelled like a component name included.
basis: fct-ow7mqtxk
because: A virtual node id `{owner}///{key}` holding `//` is now produced in process, so the anchored path is the only such id among served ids alone.
@row_add n=1 row=absence/virtual-nodes-on-carriers area=input-validation fact=F1 detector=D-security-input
capability: Virtual nodes on any carrier: no snapshot, graph, canvas answer, accessibility tree or MCP result holds a declared node
basis: fct-k6v6mfx2
because: A canvas's declared virtual nodes, their ids and names cross no carrier, and whether name and key ride the accessible-name and stable-id exits stays a deferred fork, so the absence must stand as a row.
@row_add n=2 row=decision/virtual-node-declarations-validated area=input-validation fact=F2 detector=D-security-input
name: Declared virtual nodes validated by a closed refusal set
rule: A canvas's declared virtual nodes are validated by their reader in declaration order against a closed refusal set: empty key, key holding a slash, later duplicate key with the first kept, empty name, region outside the canvas content box; a refusal holds the index and reason alone, no panic path.
guard: virtual_node::tests, virtual_nodes_declared
state: provisional
basis: fct-wkaf2evl
because: The chunk adds an app-authored declaration input with its validation in its reader, a new validated surface the plan must hold.
@row_add n=3 row=decision/declared-node-read-closed-unread area=error-handling fact=F3 detector=none
name: Declared-node read fails by a closed set without panic
rule: The declared-node read of a canvas answers a stale or non-canvas node as no canvas and a canvas before its first layout as not laid out, a closed set of two, with no panic path.
guard: an_unlaid_canvas_reads_not_laid_out, a_non_canvas_or_stale_node_reads_no_canvas
basis: fct-jw55vkbf
because: The new in-process read refuses an unusable canvas from a closed set and never panics, a rule later readers of declared nodes must keep.
@row_add n=4 row=decision/declared-nodes-never-logged area=logging-monitoring fact=F4 detector=none
name: Declared virtual nodes kept out of logs
rule: The declared-node read and its validation hold no log site, print, span or field, so a declared name, key or virtual node id reaches no log.
basis: fct-wkaf2evl
because: The chunk adds a read of app-declared names and keys with no log site, and the standing screen-content row would pass its bound if it took this class.
@row_add n=5 row=decision/virtual-node-role-accesskit area=input-validation fact=F5 detector=none
name: Declared virtual node role is AccessKit's own
rule: A declared virtual node's role is AccessKit's Role type itself with no second mapping, and declaration validation reads no role.
state: provisional
basis: fct-wkaf2evl
because: The declaration carries its role as AccessKit's closed type, so no role mapping or role check is an input surface of the declaration.
