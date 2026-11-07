@answer n=1 entity=standard-contracts#1.1 v=yes
why: the DioxusDocument accessibility_tree override with stable ids (dioxus_document.rs) is the accessibility snapshot the scope says must read the same after the buffered render
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock and its readers are outside the render path, the poll and the probe and stand scope
@answer n=3 entity=standard-contracts#2.2 v=yes
why: select_value is the value the Dioxus snapshot reads for a select, a value the re-measured snapshot must keep
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list fields and their writers are not reached by the render path and no scope item names them
@answer n=5 entity=standard-contracts#2.4 v=yes
why: the removal arm reads the select list through live_select_list, the interaction state on a removed node that the replayed buffer must still clear
@answer n=6 entity=standard-contracts#2.5 v=yes
why: clear_interaction_state_for_removed_node clears the removed select's list, the interaction-state-on-removal rule the replayed buffer must keep
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of is the value the Dioxus snapshot builder reads for a select, re-measured by the stand checks
@answer n=8 entity=standard-contracts#2.7 v=no
why: a multiple select's set settling on a mutator write or removal is form state, not interaction state, and no scope item names it
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler error responses are networking and resource loading, which the render path does not reach
@answer n=10 entity=standard-contracts#3.1 v=yes
why: the doc.mutate() methods the replayed buffer drives through DocumentMutator, and the accessibility tree naming the snapshot reads, bind the replay
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single select's tree expanded write is not a value the snapshot reads; the snapshot reads the closed-select answer of standard-contracts#2.6 instead
@answer n=12 entity=standard-contracts#3.3 v=yes
why: the tree's focus write is the accessibility side of the focus the inverted pins and the ported test assert moves to the named element
@answer n=13 entity=standard-contracts#3.4 v=yes
why: the snapshot's checked reads the native checked state this rule writes into the tree, a value the stand checks re-measure
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved width and height are CSS used values that neither the snapshot, the render path nor the scoped tests read
@answer n=15 entity=standard-contracts#3.6 v=yes
why: the attribute, child-list and removal writes the replayed buffer applies go through DocumentMutator's restyle and changed-set path, which must hold after the one-borrow replay
@answer n=16 entity=standard-contracts#4.1 v=yes
why: focus change dispatches blur, focusout, focus and focusin, which set_focus from render must reach, and the event driver's dispatch order the mounted events keep
@answer n=17 entity=standard-contracts#4.2 v=no
why: select list default actions on pointer, key and wheel input are not reached by the render or focus path
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label click on a select is a click default action the chunk does not touch
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkbox and radio change event is a click default action the chunk does not reach
@answer n=20 entity=standard-contracts#5.1 v=yes
why: MountedData scroll_to and scroll carry scroll_into_view and the scroll writes the scope names as calls made from render, and the NodeTree stale-id indexing panic is the check stale_node_mapping re-runs
@answer n=21 entity=standard-contracts#6.1 v=yes
why: get_client_bounding_rect is read by the snapshot bounds and by dioxus-native-dom's get_client_rect, a MountedData call the scope names from render, so the transform rule over it is re-measured
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is read by execute.rs and lint.rs, neither reached by the chunk nor read by the snapshot
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is read only by escher-driver's Session::lint, which the chunk does not reach
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk over dropped nodes serves hit, covered and pointer handling, which the render path does not reach; stale_node_mapping checks indexing, not hit testing
@answer n=25 entity=standard-contracts#6.5 v=no
why: rejecting hits under a non-invertible transform is a hit-test rule the chunk does not touch
