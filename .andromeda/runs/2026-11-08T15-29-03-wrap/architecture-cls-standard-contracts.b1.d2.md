@answer n=1 entity=standard-contracts#1.1 v=no
why: the blitz-dom document core is not in any reached file, and the session left the engine unchanged
@answer n=2 entity=standard-contracts#2.1 v=no
why: the document clock lives in blitz-dom clock.rs and config.rs, neither reached by the session
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is a blitz-dom reader in document.rs, which the session did not reach
@answer n=4 entity=standard-contracts#2.3 v=yes mode=stale
why: execute.rs is reached whole and holds the driver's focus_select, a named caller of close_select_list, so that caller claim can have moved
@answer n=5 entity=standard-contracts#2.4 v=no
why: the live_select_list reader and its callers are blitz-dom internals in document.rs and pointer.rs, not reached by the session
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve.rs and the removal arm are blitz-dom engine code the session did not reach
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: dioxus-native-dom lib.rs is reached and gains the ActionableNode re-export, so the cited lib.rs:46-49 may have moved, and execute.rs locate and focus_select are reached whole
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select record sits in blitz-dom mutator.rs, which the session did not reach
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is a blitz-dom net.rs contract the session did not reach
@answer n=10 entity=standard-contracts#3.1 v=no
why: the accessibility tree and the DioxusDocument override are unchanged, and actionable_nodes adds no accessibility-tree claim that this entity states
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select expanded write is blitz-dom accessibility.rs, which the session did not reach
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree focus rule is a blitz-dom accessibility.rs reading the session did not change
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control toggled write is blitz-dom accessibility.rs, which the session did not reach
@answer n=14 entity=standard-contracts#3.5 v=no
why: the unrounded resolved width and height read is in blitz-dom resolved_style.rs, which the session did not reach
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator.rs, which the session did not reach
@answer n=16 entity=standard-contracts#4.1 v=no
why: event dispatch and the Dioxus converter are unchanged, and the mark still routes events as before
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list default actions are blitz-dom pointer and keyboard code the session did not reach
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click arm is blitz-dom pointer code and its test select_popup.rs is not reached
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event is blitz-dom pointer.rs, which the session did not reach
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API and the absolute position reader are blitz-dom code the session did not change, and the harness helpers it names are not a cited reached file
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is blitz-dom document.rs, and its dioxus readers are not cited in any reached file
@answer n=22 entity=standard-contracts#6.2 v=yes mode=stale
why: execute.rs is reached whole, and its screenshot arm now meets in_view and crop through check_target, so the callers claim can have moved
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is a blitz-dom reader whose one driver caller, Session::lint, lives outside the reached files
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk skip is blitz-dom node.rs code the session did not reach
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform rule is blitz-dom node.rs code the session did not reach
