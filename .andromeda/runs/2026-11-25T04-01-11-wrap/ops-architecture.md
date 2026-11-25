@row_set n=1 row=contract/stable-element-id/virtual-node-id cell=promise seen=1 detector=none
value: A reserved fifth form {owner}///{key}, which no tier produces and only the in-process declared-node read gives a canvas's taken nodes, is spelled by the public VIRTUAL_KEY_SEPARATOR and virtual_node_id, which answers None for an empty or /-bearing key.
basis: fct-ow7mqtxk
because: The reserved form moves from produced by nothing to produced in process by DioxusDocument::virtual_nodes for a canvas's taken nodes, so the row must say who now gives it.
@row_set n=2 row=contract/dioxus-bridge/virtual-node-id cell=promise seen=1 detector=none
value: A reserved fifth id form {owner}///{key} is spelled by the public virtual_node_id and written by no walk; DioxusDocument::virtual_nodes gives it in process to a canvas's taken nodes, and no carrier serves it.
basis: fct-ow7mqtxk
because: virtual_node_id gains its first production caller in the declared-node read, so the form is now produced in process while still written by no walk and served by no carrier.
@row_set n=3 row=decision/accessibility-feature-gating cell=rule seen=3 detector=D-arch-resources
value: The `accessibility` feature enables dioxus-native-dom's `accesskit` and gates its accessibility-tree override and its `snapshot`, `snapshot_diff`, `snapshot_text`, `actionable` and `graph` modules; its `signal_registry`, `reached`, `seed`, `boundary` and `virtual_node` modules stay ungated.
basis: fct-udi2afgw
because: dioxus-native-dom gains the ungated module virtual_node, so the row's list of ungated modules must name it.
@row_add n=1 row=contract/nodes-widgets/virtual-nodes area=standard-contracts fact=F1 detector=D-arch-resources
boundary: blitz-dom nodes and widgets
item: Widget::virtual_nodes and VirtualNodeDeclaration
promise: Behind custom-widget, a defaulted method returning an empty list through which a widget declares regions, each a VirtualNodeDeclaration of key, AccessKit Role, name and region as a BoundingRect; the engine never calls it.
basis: fct-wkaf2evl
because: blitz-dom's public surface gains the type and the defaulted trait method, a new contract at the widget boundary not yet registered.
@row_add n=2 row=contract/dioxus-bridge/virtual-nodes area=standard-contracts fact=F2 detector=D-arch-resources
boundary: Dioxus DOM bridge
item: DioxusDocument::virtual_nodes
promise: An ungated in-process read of one canvas's declarations, answering the taken nodes, each with id {owner}///{key}, and the refused declarations by index, or NoCanvas for a stale or non-canvas node and NotLaidOut before the first layout; no panic path.
basis: fct-wkaf2evl
because: dioxus-native-dom gains the public read and its five re-exported types, a new contract at the bridge not yet registered.
@row_add n=3 row=contract/dioxus-bridge/virtual-node-refusal area=standard-contracts fact=F3 detector=D-arch-resources
boundary: Dioxus DOM bridge
item: VirtualNodeRefusal
promise: Validated in declaration order, a declaration is refused with one closed reason: EmptyKey, KeyHoldsSlash, DuplicateKey (the first of a key kept), EmptyName or RegionOutside (not inside the content box with positive size, NaN and infinities failing); a refusal holds the declaration's index alone.
basis: fct-wkaf2evl
because: The declared-node read validates each declaration against a closed refusal set, a promise of the bridge's new read that no row holds.
@row_add n=4 row=decision/virtual-node-role-is-accesskit-role area=established-decisions fact=F4 detector=none
name: Declared virtual node role is AccessKit's Role
rule: A declared virtual node's role is AccessKit's Role itself, with no second role mapping of escher's own.
state: provisional
basis: fct-wkaf2evl
because: VirtualNodeDeclaration carries accesskit::Role directly, a delegate's choice applied ahead of ratification that future work must respect.
@row_add n=5 row=decision/virtual-node-declaration-marks-nothing area=established-decisions fact=F5 detector=none
name: A declaration change marks and signals nothing
rule: A change of a widget's declared virtual nodes neither marks its canvas in the changed set nor signals any reader; invalidation and the changed-set mark are left to the first reader that refreshes.
state: provisional
basis: fct-wkaf2evl
because: The declared-node read writes no changed-set mark and no invalidation signal, a delegate's choice that leaves both to the first refreshing reader.
@row_add n=6 row=absence/virtual-nodes-on-carriers area=standard-contracts fact=F6 detector=none
capability: Virtual nodes on any carrier: no verb, snapshot, graph, diff or accessibility tree holds a declared node, its id or name
basis: fct-jw55vkbf
because: No carrier gains a reading of a declared node, asserted by a_declaring_canvas_puts_nothing_on_the_snapshot_or_graph, a whole capability the project does not yet have.
