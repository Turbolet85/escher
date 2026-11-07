@answer n=1 entity=standard-contracts#1.1 v=yes
why: DioxusDocument's accessibility_tree override with stable ids is the snapshot the buffered render must leave reading the same
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock times scrollbar fade, double-click, pan and fling, which the buffered render, focus and the witnesses do not touch
@answer n=3 entity=standard-contracts#2.2 v=yes
why: select_value is the value the snapshot's value field reads
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list's fields and readers belong to the popup, reached by the chunk only through 2.4, 2.5 and 2.6
@answer n=5 entity=standard-contracts#2.4 v=yes
why: the removal arm reads the list through live_select_list, so a node the bridge removes must still clear its list state after the replayed buffer
@answer n=6 entity=standard-contracts#2.5 v=yes
why: clear_interaction_state_for_removed_node clears the select on removal, and resolve reconciles the list before refresh_hover on the path replayed removals take
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of is called by the snapshot builder, so its answer for an open select must read the same after the buffered render
@answer n=8 entity=standard-contracts#2.7 v=yes
why: the replayed buffer must reach multiple writes and removals through DocumentMutator, which records the select and settles its set
@answer n=9 entity=standard-contracts#2.8 v=no
why: a failed stylesheet's release of a pending critical resource is a net path the Dioxus render does not reach
@answer n=10 entity=standard-contracts#3.1 v=yes
why: build_accessibility_tree is the tree DioxusDocument's accessibility_tree returns and the snapshot reads, so its names, states and roles must read the same after the buffered render
@answer n=11 entity=standard-contracts#3.2 v=yes
why: a single select's aria-expanded and expanded are fields of the same tree the snapshot reads
@answer n=12 entity=standard-contracts#3.3 v=yes
why: the tree's focus is written only for a built node, and set_focus must move focus to the named element the tree reports
@answer n=13 entity=standard-contracts#3.4 v=yes
why: a native control's toggled state in the tree is the checked value the snapshot reads through the element
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved_style_value is the CSSOM computed-style reader, which neither the bridge's mounted methods nor the snapshot reads
@answer n=15 entity=standard-contracts#3.6 v=yes
why: set_attribute, removal and insertion restyle through DocumentMutator, the path the replayed buffer must keep
@answer n=16 entity=standard-contracts#4.1 v=yes
why: set_focus must reach the focus change that dispatches blur, focusout, focus and focusin, under the focus rules it runs by
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's pointer, wheel and keyboard default actions are not the focus change set_focus reaches
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label's click focusing a bound select is a click dispatch the chunk does not make
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkbox and radio change event follows a click the chunk does not dispatch
@answer n=20 entity=standard-contracts#5.1 v=yes
why: scroll_to and scroll_into_view are the MountedData scroll verbs the task names, and the NodeTree stale-id panic is what stale_node_mapping re-runs
@answer n=21 entity=standard-contracts#6.1 v=yes
why: get_client_bounding_rect is the reader behind MountedData get_client_rect, which user code calls during render
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region's callers are the driver's in_view, crop and lint, none of which the Dioxus bridge reaches
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent's one caller is escher-driver's Session::lint, outside the Dioxus bridge
@answer n=24 entity=standard-contracts#6.4 v=yes
why: a node the bridge removes is dropped, and the hit walk that resolve's refresh_hover runs after a replayed removal must skip it
@answer n=25 entity=standard-contracts#6.5 v=no
why: a non-invertible transform's hit rule is not reached by the removal or focus path the chunk changes
