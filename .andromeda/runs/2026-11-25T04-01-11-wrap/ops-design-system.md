@row_add n=1 row=contract/canvas-virtual-nodes/declaration area=surface fact=F1 detector=none
boundary: A canvas widget's declared virtual nodes
item: declaration
promise: A canvas widget may declare virtual nodes inside the canvas, each with a key, a name, an AccessKit role and a region; the declarations are read in process only, by the Dioxus document's virtual-nodes read, which refuses a stale or non-canvas node as NoCanvas and an unlaid canvas as NotLaidOut.
basis: fct-wkaf2evl
because: The chunk adds the defaulted widget method and the in-process read of a canvas's declared nodes, a new contract at the app's canvas surface that no row states.
@row_add n=2 row=contract/canvas-virtual-nodes/id area=surface fact=F1 detector=none
boundary: A canvas widget's declared virtual nodes
item: id
promise: Each taken declared node gets the reserved id {owner}///{key}, its canvas's stable id then its key.
basis: fct-ow7mqtxk
because: The reserved form is now produced in process for a canvas's taken nodes, and the surface must say which id a declared node carries.
@row_add n=3 row=contract/canvas-virtual-nodes/region area=surface fact=F1 detector=none
boundary: A canvas widget's declared virtual nodes
item: region
promise: A declared node's region must lie inside the canvas's content box, the laid-out size less border and padding, at a non-negative origin with a positive width and height; a region past it is refused RegionOutside.
basis: fct-wkaf2evl
because: The read validates each region against the canvas's content box, a promise an app declaring nodes relies on.
@row_add n=4 row=contract/canvas-virtual-nodes/refusal area=surface fact=F1 detector=none
boundary: A canvas widget's declared virtual nodes
item: refusal
promise: A refused declaration gets one closed reason, EmptyKey, KeyHoldsSlash, DuplicateKey, EmptyName or RegionOutside, held with its declaration's index alone; of two declarations sharing a key the first is kept.
basis: fct-wkaf2evl
because: The chunk gives every refused declaration a closed reason with no panic path, a contract of the new read that no row states.
@row_add n=5 row=decision/virtual-nodes-reach-no-carrier area=surface fact=F2 detector=none
name: Virtual nodes reach no carrier
rule: Declared virtual nodes reach no carrier: the snapshot, the UI graph, the accessibility tree, the actions line and the driver verbs hold no declared node, no declared name and no /// id.
why: whether a declared node's name and key cross to the carriers is an open fork, deferred to the virtual-nodes-as-elements entry
guard: a_declaring_canvas_puts_nothing_on_the_snapshot_or_graph
state: provisional
basis: fct-jw55vkbf
because: The chunk lets a canvas declare nodes yet keeps every agent carrier free of them on a delegate's deferral, a decision the surface's carrier rows must be read with.
