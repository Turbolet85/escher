@answer n=1 entity=standard-contracts#1.1 v=no
why: the Document trait, changed set, select and clock readers of blitz-dom are not named by the chunk's scope
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock moves no time in this chunk and no clock-driven behaviour is named in its scope
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value and the select choice writers are outside the chunk, which reads no control value
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list state and its readers are not changed or named by the chunk
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and the reconcile path concern select lists the chunk does not reach
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal clear for a select are not reached by the chunk
@answer n=7 entity=standard-contracts#2.6 v=yes
why: is_select reaches the named actions verb through target_of, which Session::actions calls, so the replayed actions step carries it
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select write record is select model behaviour the chunk does not touch
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error and failed stylesheet loads are networking, which the chunk does not touch
@answer n=10 entity=standard-contracts#3.1 v=no
why: the selector API, CSSOM and platform accessibility tree builder are not named by the chunk's scope
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded state and aria-expanded read are select and ARIA behaviour outside the chunk
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus write concerns the platform tree, which the chunk does not read
@answer n=13 entity=standard-contracts#3.4 v=no
why: a native control's toggled state in the platform tree is not read by the chunk's coverage or actions path
@answer n=14 entity=standard-contracts#3.5 v=no
why: unrounded resolved width and height are a resolved-style read the chunk does not make
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is engine behaviour the chunk neither changes nor reads
@answer n=16 entity=standard-contracts#4.1 v=yes
why: EventHandler::handle_event and the node chain of EventDriver carry the DOM events the bridge's handle_event records, and the actions verb's clicks and keys reach them through pointer, focus and Enter or Space activation
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions are select behaviour the chunk does not change
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label click focusing a bound select is select behaviour the chunk does not reach
@answer n=19 entity=standard-contracts#4.4 v=no
why: the engine's change dispatch on a checkbox or radio click is not a rule the recorder must honour, since it records whichever DOM listener is reached
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API, traversal, selection and NodeTree stale-id reads are not named by the scope, and the chunk keys its record on graph ids
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is read by the snapshot bounds and get_client_rect, which the chunk does not name or change
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms and its driver callers in_view and crop are not in the chunk's scope
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent and its one caller Session::lint are outside the chunk
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk's skipping of dropped nodes is not named by the scope, and the coverage read performs no hit test
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule concerns hit testing the chunk does not touch
