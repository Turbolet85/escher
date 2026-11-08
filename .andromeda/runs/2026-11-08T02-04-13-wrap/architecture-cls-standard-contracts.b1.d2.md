@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: dioxus_document.rs, a reached file, gains DioxusDocument::graph under accessibility beside the snapshot and changes DioxusEventHandler::handle_event, so the cited lines 279-354 of that file can have moved; the accessibility_tree override itself holds.
@answer n=2 entity=standard-contracts#2.1 v=no
why: the clock is blitz-dom engine state, and no blitz-dom file moved in the session.
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is a blitz-dom read, and its snapshot value and apply_select_choice callers are unchanged, the snapshot change only adding clickable beside them.
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list state and its public readers and writers are blitz-dom select-model code, which the session does not touch.
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and its reconcile reads are private blitz-dom document.rs internals, unmoved by the session.
@answer n=6 entity=standard-contracts#2.5 v=no
why: the reconcile call in resolve and the removal arm are blitz-dom code, unmoved by the session.
@answer n=7 entity=standard-contracts#2.6 v=yes mode=add-to
why: fact F2 belongs beside this entry, since a graph option or optgroup owner is the stable id of BaseDocument::select_of, and the closed_select_of citations in snapshot.rs and lib.rs, both reached files, can also move.
@answer n=8 entity=standard-contracts#2.7 v=no
why: the mutator's multiple-select write and removal record are blitz-dom code, which the session does not reach.
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error and the stylesheet resource handling are blitz-dom net.rs, untouched by the session.
@answer n=10 entity=standard-contracts#3.1 v=no
why: the mutation, selector, CSSOM and accessibility-builder surfaces are blitz-dom or blitz-shell code the session does not touch, and its test citations are unreached files.
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select expanded write is in the blitz-dom accessibility builder, which the session leaves unchanged.
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree focus rule is in the blitz-dom accessibility builder, unchanged by the session.
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkable state write is in the blitz-dom accessibility builder, unchanged by the session.
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height read is blitz-dom resolved_style.rs, which the session does not reach.
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator.rs code, unmoved by the session.
@answer n=16 entity=standard-contracts#4.1 v=no
why: the EventHandler trait, the driver and the default actions are blitz-dom event code, and the bridge change only wraps the runtime.handle_event call, leaving the converter's value path as it was.
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions are blitz-dom event code, and the choice path through apply_select_choice is unchanged by the session.
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click arm that focuses a bound select is blitz-dom pointer code, unmoved by the session.
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkable control change dispatch is blitz-dom pointer code, unmoved by the session.
@answer n=20 entity=standard-contracts#5.1 v=no
why: the scroll API and scroll_into_view are blitz-dom scrolling code, untouched, and their signature and callers are unchanged by the session.
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is blitz-dom code, and its named callers are not edited by the session.
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is blitz-dom code, and its execute.rs callers in_view and crop are unchanged, the graph change adding only a new match arm.
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is blitz-dom code, and Session::lint is not changed in its reading of it.
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk and its dropped-node skip are blitz-dom code, which the session does not touch.
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is blitz-dom node code, unmoved by the session.
