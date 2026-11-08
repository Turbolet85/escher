@answer n=1 entity=standard-contracts#1.1 v=yes
why: DioxusDocument's accessibility override with stable ids is the stable element id scheme the read's ids rest on.
@answer n=2 entity=standard-contracts#2.1 v=yes
why: it names VirtualClock and its advance call, which the read must avoid since it moves no virtual clock time.
@answer n=3 entity=standard-contracts#2.2 v=no
why: the chosen-value state and its writers are neither read nor written by the chunk, and the snapshot value field is outside its scope.
@answer n=4 entity=standard-contracts#2.3 v=yes
why: the public open-list readers open_select, select_list_bounds and select_list_view are what the shared refusal path answers first for an open list.
@answer n=5 entity=standard-contracts#2.4 v=yes
why: live_select_list is the reader of the open list state that the open-list refusal answer rests on.
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call order is blitz-dom's own, and the read runs no resolve step.
@answer n=7 entity=standard-contracts#2.6 v=yes
why: is_select and closed_select_of are named by the scope as the select tests the driver reads.
@answer n=8 entity=standard-contracts#2.7 v=no
why: the write record for a multiple select is a mutator write that the read never makes and no scope name covers.
@answer n=9 entity=standard-contracts#2.8 v=no
why: the NetHandler error path concerns stylesheet loads, and the chunk makes no fetch.
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, selector and accessibility-state API catalogue is not named by the scope, and the read neither mutates, queries nor builds an adapter tree.
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded state in the platform tree is an accessibility write the read does not touch.
@answer n=12 entity=standard-contracts#3.3 v=no
why: focus in the built accessibility tree is an adapter output the read does not list.
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control's toggled state in the accessibility tree is an adapter output, not the snapshot's checked reading the chunk uses.
@answer n=14 entity=standard-contracts#3.5 v=no
why: unrounded resolved width and height are a computed-style read the chunk does not make.
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs only on writes, which the read never makes.
@answer n=16 entity=standard-contracts#4.1 v=no
why: event dispatch and default actions belong to the executor's own calls, and the read dispatches nothing.
@answer n=17 entity=standard-contracts#4.2 v=yes
why: a click on an option of an open list and a single select's open by click are the owner-served calls the proof on the stand drives.
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label arm's focus of a bound select lies outside the scope's owner-served and refusal cases.
@answer n=19 entity=standard-contracts#4.4 v=no
why: the change event on checkable controls is not reached by the read or the refusal path.
@answer n=20 entity=standard-contracts#5.1 v=yes
why: visible_region's clipping, the hit walk's clipping stop and scroll_into_view are the seeing and hit readings the refusal path rests on.
@answer n=21 entity=standard-contracts#6.1 v=yes
why: the snapshot's bounds and get_client_rect read get_client_bounding_rect under transforms, which the off-screen reading rests on.
@answer n=22 entity=standard-contracts#6.2 v=yes
why: visible_region under transforms is the off-screen reading that in_view calls on the shared refusal path.
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is read by Session::lint alone, which the chunk leaves unchanged.
@answer n=24 entity=standard-contracts#6.4 v=yes
why: the hit walk answering an open select list first and the covered reading through hit are the refusal path's hit.
@answer n=25 entity=standard-contracts#6.5 v=yes
why: a non-invertible transform taking no hit is what the covered reading through hit answers for a target under it.
