@answer n=1 entity=standard-contracts#1.1 v=yes
why: DioxusDocument's accessibility_tree override writes the stable element ids the coverage list is keyed on (scope item 4)
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock is named not needed; the coverage read moves no time and the harness tick and settle stay as they are
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is the select model's value reading feeding the snapshot value, which the chunk's wire boundary keeps off the answer
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list field and its readers are the select model, named not needed by the scope
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and reconcile_select_list are the open-list reader and reconcile, named not needed by the scope
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal arm clearing the select list are select-model paths the scope excludes
@answer n=7 entity=standard-contracts#2.6 v=yes
why: is_select is read by execute.rs target_of, which Session::actions calls, and closed_select_of feeds the snapshot builder the clickable reading rides on (scope items 7 and 17)
@answer n=8 entity=standard-contracts#2.7 v=no
why: a multiple write recording its select is the select model, named not needed by the scope
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is the resource-loading path; the chunk makes no request and the stand's load wait is unchanged
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, selector and CSSOM surface is used by no unit the chunk edits or names, and the accessibility states it lists are not the snapshot's clickable reading
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded state and its open attribute are the select model, which the scope leaves unchanged
@answer n=12 entity=standard-contracts#3.3 v=no
why: the accessibility tree's focus write is the platform-tree builder, which the chunk does not read
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checked toggle is written into the accessibility tree, which the chunk's snapshot read does not use
@answer n=14 entity=standard-contracts#3.5 v=no
why: the unrounded width and height in resolved_style_value is a reader no unit the chunk edits or calls
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs only where a mutation is made, and the chunk makes none
@answer n=16 entity=standard-contracts#4.1 v=yes
why: the exercised record counts the DOM events the engine dispatches to listeners, so the DomEventKind name mapping and the activation click dispatch bind it (scope item 3)
@answer n=17 entity=standard-contracts#4.2 v=no
why: select list default actions are the select model, which the scope names as not changed
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label's click on a select is the select model, named not needed by the scope
@answer n=19 entity=standard-contracts#4.4 v=yes
why: a checkbox or radio click dispatches an engine change after input, which a replayed click step reaches and the record must count (scope items 5 and 6)
@answer n=20 entity=standard-contracts#5.1 v=yes
why: scroll_into_view and visible_region are the in-view reads that Session::run's off-screen refusal and the actions answer take, and scroll writes dispatch scroll events the record counts
@answer n=21 entity=standard-contracts#6.1 v=no
why: the transformed client rect feeds bounds and pointer coordinates, but the clickable reading names no bounds and no stand step targets a transformed element
@answer n=22 entity=standard-contracts#6.2 v=yes
why: visible_region under transforms is the in-view reading behind Session::run's off-screen refusal and the actions answer's booleans
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is read by Session::lint alone, and lint is not in the chunk's scope
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk's dropped-node skip is an engine rule the chunk's clicks take as given, and the owed replay removes no node under a click
@answer n=25 entity=standard-contracts#6.5 v=no
why: a non-invertible transform taking no hit is a case the chunk's stand checks and replay do not build
