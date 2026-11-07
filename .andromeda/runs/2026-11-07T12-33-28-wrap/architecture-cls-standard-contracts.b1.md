@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: the DioxusDocument impl cited at packages/dioxus-native-dom/src/dioxus_document.rs:266-341 moves under the 52 lines the buffered render adds in that bridge file
@answer n=2 entity=standard-contracts#2.1 v=no
why: the clock and its re-exports are blitz-dom code the session does not touch
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is blitz-dom document code the session does not touch
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list fields and methods are blitz-dom document code the session does not touch
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its two reads are blitz-dom code the session does not touch
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve and the removal arm are blitz-dom code the session does not touch
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: the crate-root re-export cited at packages/dioxus-native-dom/src/lib.rs:36-39 moves down one line under the new private mutation_buffer mod line at line 21
@answer n=8 entity=standard-contracts#2.7 v=no
why: the mutator's select record on a multiple write is blitz-dom code the session does not touch
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is blitz-dom net code the session does not touch
@answer n=10 entity=standard-contracts#3.1 v=yes mode=add-to
why: the wrap re-says the windowed AT witness owner res-llkgqygv as re-carried at 0.3.0 (target next), which this owner clause must now carry
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single select's aria-expanded write is blitz-dom accessibility code the session does not touch
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus rule is blitz-dom accessibility code the session does not touch
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native control toggled write is blitz-dom accessibility code the session does not touch
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height read is blitz-dom resolved-style code the session does not touch
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator code whose flush and restyle rules the session leaves unchanged
@answer n=16 entity=standard-contracts#4.1 v=no
why: the event driver and the bridge's converter are untouched, and the converter still carries the value to oninput unchanged
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions are blitz-dom event code the session does not touch
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label arm's focus on a bound select is blitz-dom pointer code the session does not touch
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable change event is blitz-dom pointer code the session does not touch
@answer n=20 entity=standard-contracts#5.1 v=no
why: scroll_into_view keeps its signature and callers, MountedData::scroll_to included, and its blitz-dom body is untouched
@answer n=21 entity=standard-contracts#6.1 v=no
why: get_client_bounding_rect and its readers, dioxus-native-dom's get_client_rect included, are unchanged by a session that only moves the bridge's render and flush path
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region is blitz-dom scrolling code the session does not touch
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is blitz-dom code the session does not touch
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk is blitz-dom node and layout code the session does not touch
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is blitz-dom node code the session does not touch
