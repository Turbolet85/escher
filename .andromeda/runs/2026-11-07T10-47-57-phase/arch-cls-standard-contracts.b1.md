@answer n=1 entity=standard-contracts#1.1 v=yes
why: the Document trait's inner, inner_mut and poll contract as DioxusDocument implements it, and poll is an edited render site.
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock drives overlay fade, double-click, pan and fling, none of which the buffered render or the probe reads.
@answer n=3 entity=standard-contracts#2.2 v=yes
why: select_value is the reader of the snapshot's select value, which the stand_* checks re-assert after the buffered render.
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select_list field and its public and crate-private readers and writers are not reached by the render path, only the removal arm is (2.4, 2.5).
@answer n=5 entity=standard-contracts#2.4 v=yes
why: the removal arm reads a removed select's list state, which the replayed removal must still clear.
@answer n=6 entity=standard-contracts#2.5 v=yes
why: resolve's reconcile order and clear_interaction_state_for_removed_node clearing a removed select's field are the interaction state on removed nodes the buffer must keep.
@answer n=7 entity=standard-contracts#2.6 v=yes
why: closed_select_of is the snapshot builder's read of a select's open state, and select_values and select_of are read by the snapshot's select values, all re-asserted by the stand_* checks.
@answer n=8 entity=standard-contracts#2.7 v=no
why: the mutator's recording of a multiple select is not named by the scope, the replay issues the same calls and the mutator keeps its own recording.
@answer n=9 entity=standard-contracts#2.8 v=no
why: a failed stylesheet's NetHandler error is networking, and the task puts blitz-net and resource loading out of scope.
@answer n=10 entity=standard-contracts#3.1 v=yes
why: the replay issues these doc.mutate() calls (create_element, set_attribute, insert_nodes_before, remove_and_drop_node and the rest), so their contract binds the chunk's replay.
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded write in the engine's accessibility tree is not named by the scope and no edited unit writes it.
@answer n=12 entity=standard-contracts#3.3 v=yes
why: the tree's focus is read from focus_node_id, the focus that set_focus moves and the focus rule the scope names.
@answer n=13 entity=standard-contracts#3.4 v=no
why: a native checkbox's toggled state in the engine's accessibility tree is not named by the scope and no edited unit writes it.
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height of resolved_style_value are a style reader the scope does not name.
@answer n=15 entity=standard-contracts#3.6 v=yes
why: the replay applies mutations whose restyle hints incremental_oracle, re-read as a witness, relies on for incremental and non-incremental layout identity.
@answer n=16 entity=standard-contracts#4.1 v=yes
why: focus change dispatches blur, focusout, focus and focusin, which set_focus must reach, and the driver's queued handling bears on the mounted-event order.
@answer n=17 entity=standard-contracts#4.2 v=no
why: a select list's own default actions on pointer, wheel and keys, and the pins name no select list.
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label click that focuses a bound select is a click default action, not a set_focus change.
@answer n=19 entity=standard-contracts#4.4 v=no
why: change dispatch on a checkable control's click is a click default action the chunk never drives.
@answer n=20 entity=standard-contracts#5.1 v=yes
why: scroll_into_view is the method behind MountedData::scroll_to, whose signature the chunk's callers keep, and the NodeTree stale-id panic it names is witnessed by the re-read stale_node_mapping.
@answer n=21 entity=standard-contracts#6.1 v=yes
why: dioxus-native-dom's get_client_rect reads get_client_bounding_rect through transforms, and get_client_rect is a MountedData call the scope names.
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms has callers in the driver's in_view, crop and lint, none of which the chunk reaches.
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent's one non-test caller is escher-driver's Session::lint, which the chunk does not reach.
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk's skip of dropped children is on the pointer hit path, which the render, replay and focus path does not reach, and its witness hit_test_dropped_nodes is not re-run.
@answer n=25 entity=standard-contracts#6.5 v=no
why: a non-invertible transform takes no hit, a hit-path rule the chunk does not reach.
