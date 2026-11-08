@answer n=1 entity=standard-contracts#1.1 v=yes
why: the Document trait's accessibility_tree and DioxusDocument's stable-id override are the build_accessibility_tree call the graph keys on
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock drives scroll, fling and double-click timing, none of which the graph read or the pinned stand checks name
@answer n=3 entity=standard-contracts#2.2 v=yes
why: select_value is the reader behind the snapshot value field that the graph's control values come through
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list's state fields and readers are reached only through closed_select_of, and the chunk names no select-list call nor changes one
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and the reconcile reads are internal to the select list, which the chunk does not touch
@answer n=6 entity=standard-contracts#2.5 v=no
why: the reconcile call in resolve and the removal arm clear select state that the graph read does not read
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of is the call the snapshot builder makes to read a select's value, and the snapshot is in scope
@answer n=8 entity=standard-contracts#2.7 v=no
why: the mutator's multiple-select record is a write the read-only graph does not make and no scope item names
@answer n=9 entity=standard-contracts#2.8 v=no
why: the stylesheet error response is resource loading, which the graph read does not perform
@answer n=10 entity=standard-contracts#3.1 v=yes
why: build_accessibility_tree and the TreeUpdate nodes it builds, both named by the scope, are the agent-visible nodes the graph keys on
@answer n=11 entity=standard-contracts#3.2 v=yes
why: the single select's expanded write is part of the node build_accessibility_tree hands the graph's key
@answer n=12 entity=standard-contracts#3.3 v=yes
why: the tree names only built nodes in its focus, which the graph's node key relies on
@answer n=13 entity=standard-contracts#3.4 v=yes
why: the native checkbox or radio toggled state written by build_accessibility_node is part of the node the graph keys on
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height reader serves getComputedStyle and no graph read or scope item reads it
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs on writes that the read-only chunk does not make
@answer n=16 entity=standard-contracts#4.1 v=yes
why: handler event names reach the graph through the DomEventKind mapping, and the Click dispatch to Dioxus onclick is what the reacts-to-a-click reading witnesses
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions are behaviour of a component the chunk only reads, and no scope item names them
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click focusing a bound select is a default action the chunk does not name
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event is an engine dispatch for every document that the graph read neither reads nor changes
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll, hit and selection readers are not named by the scope, and the snapshot bounds reach only get_client_bounding_rect
@answer n=21 entity=standard-contracts#6.1 v=yes
why: get_client_bounding_rect is the reader behind the snapshot bounds that the graph nodes carry, so the transform rule moves them
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is read by the driver's in_view, crop and lint, none of which the graph read names
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent has one caller, Session::lint, which the chunk does not change
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk over dropped nodes serves hit, covered and pointer handling, none of which the graph read uses
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule concerns hit testing that the graph read does not perform
