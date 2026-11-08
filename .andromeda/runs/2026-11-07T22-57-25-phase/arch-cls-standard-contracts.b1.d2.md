@answer n=1 entity=standard-contracts#1.1 v=yes
why: the DioxusDocument accessibility_tree override with stable ids and build_accessibility_tree, both named by the scope, are what the graph is keyed on
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock is a blitz-dom guarantee the chunk does not call, read or re-measure, and the scope names no clock use
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value feeds the snapshot's existing value field, which the chunk only extends by one boolean, and a select value is not a masked control
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list state is blitz-dom internal, read through no call the graph or the stand makes
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its reconcile reads belong to the select list the chunk never opens or reads
@answer n=6 entity=standard-contracts#2.5 v=no
why: the resolve-time reconcile of the select list runs for a list the graph does not read
@answer n=7 entity=standard-contracts#2.6 v=no
why: closed_select_of and the select readers feed the snapshot's existing value field, a unit the chunk must only keep holding, and is_select is the driver's locate test the graph does not use
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select write record is a mutator rule the chunk neither writes through nor reads
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler error handling is resource loading, which the graph does not do
@answer n=10 entity=standard-contracts#3.1 v=yes
why: build_accessibility_tree and its TreeUpdate nodes, names, bounds and states are the function the scope names, and the graph is keyed on its nodes
@answer n=11 entity=standard-contracts#3.2 v=yes
why: the single select's expanded write is a state of a node that build_accessibility_tree, named by the scope, builds for the graph's key
@answer n=12 entity=standard-contracts#3.3 v=yes
why: the tree writes focus only for built nodes, the node set the graph's agent-visible keying relies on
@answer n=13 entity=standard-contracts#3.4 v=yes
why: the native checked-to-toggled write in build_accessibility_node is part of the named tree, and the snapshot's checked reads it
@answer n=14 entity=standard-contracts#3.5 v=no
why: the unrounded width and height of resolved_style_value are a style reader the graph does not read or pin
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom's own invalidation, which the chunk neither writes through nor re-measures
@answer n=16 entity=standard-contracts#4.1 v=yes
why: the activation click that reaches an app's onclick through the event driver and the Dioxus bridge's oninput value path are the DOM-to-vdom routes the scope names for the click reading and item 6
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions concern a list the chunk does not open, click or pin
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label-to-select focus arm is a click default action outside the graph read and the scope
@answer n=19 entity=standard-contracts#4.4 v=no
why: change dispatch on checkable controls is an engine event the scope does not name
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll, hit and tree readers are blitz-dom geometry the graph does not read, and the driver's covered and scroll paths the chunk does not change
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms moves the snapshot's bounds, a field the chunk extends only by one boolean and does not re-measure
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is read by the driver's in_view and crop, which the chunk does not change
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is read by Session::lint alone, which the chunk does not touch
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk over dropped nodes is read by the driver's covered and the pointer path, neither of which the graph read uses
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is a hit-testing rule the graph does not use
