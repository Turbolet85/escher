@answer n=1 entity=standard-contracts#1.1 v=yes
why: DioxusDocument's Document impl, with its stable-id accessibility_tree override and the trait's handle_ui_event default beside the bridge's event path, sits in dioxus_document.rs, a reached file.
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock is outside the chunk's scope (not_needed item 10), and the chunk reads or changes no time source.
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is the snapshot's control value, and neither the coverage answer nor the actions answer carries a control value.
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open-list readers and writers are select-list state the chunk neither drives nor reads; the list's effect on hits binds through #6.4.
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and reconcile_select_list maintain select-list state the chunk does not touch.
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call orders select-list state the chunk does not change.
@answer n=7 entity=standard-contracts#2.6 v=yes
why: Session::actions calls is_select through execute.rs target_of, so the actions verb the replay test compares carries this select test.
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select mutator record is not on the coverage, actions or replay path.
@answer n=9 entity=standard-contracts#2.8 v=no
why: the failed-stylesheet error path is network behaviour, which the chunk's stand pages do not reach.
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, selector, CSSOM and accessibility-name rules are not read by the coverage list, the actions answer or the replay.
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded state and open steering are accessibility state the chunk does not read.
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus write is accessibility output the chunk does not build.
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkbox toggled write is accessibility output, and no chunk answer carries a checked state.
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved width and height read through unrounded layout are style reads that no chunk answer carries.
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is a mutator path the chunk's code and tests do not call.
@answer n=16 entity=standard-contracts#4.1 v=yes
why: EventHandler::handle_event is the signature the bridge's handle_event implements, and the non-bubbling and target-routing rules decide which (element, event) handlers a run reaches.
@answer n=17 entity=standard-contracts#4.2 v=yes
why: a select list's default actions (Enter opens, Tab chooses, a choice's input then change) are DOM events reaching listeners the coverage record counts.
@answer n=18 entity=standard-contracts#4.3 v=yes
why: a label click reaching a bound select without re-dispatch decides which select listeners a run reaches.
@answer n=19 entity=standard-contracts#4.4 v=yes
why: the engine's change event after input on a checkbox or radio click is a DOM event reaching change listeners the coverage record counts.
@answer n=20 entity=standard-contracts#5.1 v=yes
why: visible_region and the hit walk feed the cause words of the actions answer, and scroll writes dispatch a scroll event the record counts.
@answer n=21 entity=standard-contracts#6.1 v=yes
why: visible_region reads transformed_bounds, which this entity defines, and the events driver's coordinate adjustment sits on the event path.
@answer n=22 entity=standard-contracts#6.2 v=yes
why: visible_region under transforms feeds in_view and the off-screen cause the actions answer carries.
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent's one non-test caller is Session::lint, which the chunk neither changes nor calls.
@answer n=24 entity=standard-contracts#6.4 v=yes
why: the hit walk answers the open select list first and skips dropped nodes, which the covered cause in the actions answer reads.
@answer n=25 entity=standard-contracts#6.5 v=yes
why: a non-invertible transform taking no hit feeds the covered cause the actions answer carries.
