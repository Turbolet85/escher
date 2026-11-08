@answer n=1 entity=standard-contracts#1.1 v=no
why: no blitz-dom engine file is in the session's change, and the document-core readers it names stand on untouched engine code
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock is a blitz-dom config and no driver or test file the session reaches touches it
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value and the choice writers live in blitz-dom document.rs, which the session does not reach
@answer n=4 entity=standard-contracts#2.3 v=yes mode=stale
why: it names the driver's focus_select as a caller of close_select_list, and execute.rs, where focus_select lives, is a reached file the session edits
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and reconcile_select_list are blitz-dom internals that no reached file names
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve and clear_interaction_state_for_removed_node are blitz-dom engine code outside the session's change
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: it cites dioxus-native-dom lib.rs:46-49 for the crate-root re-exports, which the session extends with ActionableNode, and names execute.rs locate and focus_select, where locate now ends in target_of
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select write record lives in blitz-dom mutator.rs, which the session does not reach
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is an engine resource-loading contract with no driver or test file reached
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, query, CSSOM and accessibility-tree readers are blitz-dom engine surfaces the session does not change
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select aria-expanded rule is blitz-dom accessibility.rs, which the session does not reach
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree focus rule is blitz-dom accessibility.rs, which the session does not reach
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control state write is blitz-dom accessibility.rs, which the session does not reach
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height reader is blitz-dom resolved_style.rs, which the session does not reach
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator.rs, which the session does not reach
@answer n=16 entity=standard-contracts#4.1 v=no
why: the event driver contracts are blitz-dom and blitz-traits code, and the session edits no event file
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list default actions are blitz-dom events code, none of it reached by the session
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click arm is blitz-dom pointer.rs, which the session does not reach
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event is blitz-dom pointer.rs code, which the session does not reach
@answer n=20 entity=standard-contracts#5.1 v=yes mode=stale
why: it names the test harness's layout_rect, layout_rect_of, center_of and input helper, and the harness file tests/blitz-tests/tests/common/mod.rs is a reached file the session changes whole
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is blitz-dom document.rs geometry, which the session does not reach
@answer n=22 entity=standard-contracts#6.2 v=yes mode=stale
why: it names execute.rs in_view and crop as visible_region's callers, and the session routes the screenshot id path through check_target into in_view and crop in execute.rs
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is a blitz-dom reader whose one driver caller is Session::lint in lint.rs, which the session does not reach
@answer n=24 entity=standard-contracts#6.4 v=yes mode=stale
why: it names the driver's covered reading the hit walk, and the session's check_target now serves the click and type action_point path in execute.rs, a reached file
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is blitz-dom node.rs code, which the session does not reach
