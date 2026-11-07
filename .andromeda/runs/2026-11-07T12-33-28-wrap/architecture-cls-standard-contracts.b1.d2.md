@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: the session adds 52 lines in eight ranges to dioxus_document.rs, which moves the cited DioxusDocument impl range 266-341
@answer n=2 entity=standard-contracts#2.1 v=no
why: the clock lives in blitz-dom's config, clock and iframe files, none of which the session reaches
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value and its writers sit in blitz-dom's document.rs, which the session does not reach
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list fields and methods sit in blitz-dom's document.rs, which the session does not reach
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its reads sit in blitz-dom's document.rs and pointer.rs, neither reached by the session
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal arm sit in blitz-dom files the session does not reach
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: the session adds one mod line at line 21 of dioxus-native-dom's crate root, which moves the cited lib.rs:36-39 down one line
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select record lives in blitz-dom's mutator.rs, which the session does not reach
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error lives in blitz-dom's net.rs, which the session does not reach
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, query and CSSOM API sits in blitz-dom files the session does not reach, with unchanged signatures
@answer n=11 entity=standard-contracts#3.2 v=no
why: single-select aria-expanded is built in blitz-dom's accessibility.rs, which the session does not reach
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus rule is in blitz-dom's accessibility.rs, which the session does not reach
@answer n=13 entity=standard-contracts#3.4 v=no
why: native checkbox state is written in blitz-dom's accessibility.rs, which the session does not reach
@answer n=14 entity=standard-contracts#3.5 v=no
why: resolved width and height read blitz-dom's resolved_style.rs, which the session does not reach
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is in blitz-dom's mutator.rs, which the session does not reach and whose contracts are unchanged
@answer n=16 entity=standard-contracts#4.1 v=no
why: the event driver and default actions sit in blitz-dom's events files, none reached, and the session changes only the bridge's render and flush path, not the value the converter carries to oninput
@answer n=17 entity=standard-contracts#4.2 v=no
why: a select list's default actions sit in blitz-dom's events/pointer.rs and keyboard.rs, neither reached by the session
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label arm sits in blitz-dom's events/pointer.rs, which the session does not reach
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event is dispatched from blitz-dom's events/pointer.rs, which the session does not reach
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API is in blitz-dom's scrolling.rs, not reached, and scroll_into_view's signature and callers, MountedData::scroll_to included, are unchanged
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect is in blitz-dom's document.rs, not reached, and its readers, the dioxus-native-dom get_client_rect included, keep their reading
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region is in blitz-dom's scrolling.rs, which the session does not reach
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is in blitz-dom's text_extent.rs, which the session does not reach
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk sits in blitz-dom's node and paint-tree files, none reached by the session
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform rule sits in blitz-dom's node.rs, which the session does not reach
