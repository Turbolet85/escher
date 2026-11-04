@answer n=1 entity=standard-contracts#1.1 v=no
why: its Document core claims cite blitz-dom and blitz-html sources only, no packages/ code changed and none of the three reached files is named, so the claim holds
@answer n=2 entity=standard-contracts#2.1 v=no
why: the clock contract cites blitz-dom config, clock and iframe sources, none reached by the session, so the claim holds
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value cites blitz-dom document.rs only, untouched by the session's changed files, so the claim holds
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select_list fields and their readers cite blitz-dom document.rs line ranges, no engine code changed, so the claim holds
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its two reconcile reads cite blitz-dom sources the session did not reach, so the claim holds
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal clear cite blitz-dom resolve.rs and document.rs, unmoved, so the claim holds
@answer n=7 entity=standard-contracts#2.6 v=no
why: the select value and typeahead readers cite blitz-dom and dioxus-native-dom snapshot sources the session did not change, so the claim holds
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select mutator record cites blitz-dom mutator.rs, untouched by the session, so the claim holds
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler error cites blitz-dom net.rs, unreached by the session's three files, so the claim holds
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, query, CSSOM and accessibility-tree claims cite blitz-dom and test files none of which the session changed, so the claim holds
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select expanded write cites blitz-dom accessibility.rs, no engine code changed, so the claim holds
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus write cites blitz-dom accessibility.rs, unmoved by the session, so the claim holds
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control state write cites blitz-dom accessibility.rs and a test file the session did not reach, so the claim holds
@answer n=14 entity=standard-contracts#3.5 v=no
why: the unrounded width and height read cites blitz-dom resolved_style.rs, untouched by the session, so the claim holds
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation cites blitz-dom mutator.rs line ranges the session did not change, so the claim holds
@answer n=16 entity=standard-contracts#4.1 v=no
why: the events contract cites blitz-dom and blitz-traits event sources, no engine code changed, and the new probe test reaches no default action, so the claim holds
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list default actions cite blitz-dom events sources the session did not touch, so the claim holds
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label arm cites blitz-dom pointer.rs and select_popup.rs, neither reached by the session, so the claim holds
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event cites blitz-dom pointer.rs and checkable_change_event.rs, unmoved by the session, so the claim holds
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API and scroll_into_view contract cite blitz-dom scrolling.rs and fragment tests, no engine code changed, and the MountedData hazard concerns set_focus not scroll_to, so the claim holds
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms cites blitz-dom document.rs and a test file the session did not reach, so the claim holds
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms cites blitz-dom scrolling.rs and test files the session did not change, so the claim holds
@answer n=23 entity=standard-contracts#6.3 v=no
why: the text extent reader cites blitz-dom text_extent.rs, untouched by the session, so the claim holds
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk skip cites blitz-dom node.rs and paint_tree.rs, no engine code changed, so the claim holds
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule cites blitz-dom node.rs and its test file, neither reached by the session, so the claim holds
