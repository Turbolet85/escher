@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: the session adds the reached record and its handle_event write to dioxus_document.rs, which holds the cited DioxusDocument Document impl at lines 280-355, so that citation may have moved
@answer n=2 entity=standard-contracts#2.1 v=no
why: the session changes no clock, config or viewport code, and reaches no blitz-dom file
@answer n=3 entity=standard-contracts#2.2 v=no
why: the session changes no select value reader or writer, and the snapshot.rs callers it names are not reached
@answer n=4 entity=standard-contracts#2.3 v=no
why: the session reaches no blitz-dom file, and the select list fields and readers are untouched
@answer n=5 entity=standard-contracts#2.4 v=no
why: the session reaches no blitz-dom file, and live_select_list and its readers are untouched
@answer n=6 entity=standard-contracts#2.5 v=no
why: the session reaches no blitz-dom file, and resolve's select reconcile and the removal arm are untouched
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: the session adds ReachedHandler to dioxus-native-dom's crate-root re-exports in lib.rs (35 to 36), which holds the cited re-export lines 48-51, so that citation may have moved
@answer n=8 entity=standard-contracts#2.7 v=no
why: the session reaches no blitz-dom file, and the mutator's multiple-select write is untouched
@answer n=9 entity=standard-contracts#2.8 v=no
why: the session reaches no blitz-dom file, and NetHandler::error is untouched
@answer n=10 entity=standard-contracts#3.1 v=yes mode=stale
why: the session adds the record_reached write to dioxus_document.rs, which holds the cited DioxusDocument accessibility_tree override at lines 280-355, so that citation may have moved
@answer n=11 entity=standard-contracts#3.2 v=no
why: the session reaches no blitz-dom file, and the single select's aria-expanded write is untouched
@answer n=12 entity=standard-contracts#3.3 v=no
why: the session reaches no blitz-dom file, and the tree's focus write is untouched
@answer n=13 entity=standard-contracts#3.4 v=no
why: the session reaches no blitz-dom file, and the native control's toggled write is untouched
@answer n=14 entity=standard-contracts#3.5 v=no
why: the session reaches no blitz-dom file, and resolved_style's width and height reads are untouched
@answer n=15 entity=standard-contracts#3.6 v=no
why: the session reaches no blitz-dom file, and the mutator's restyle paths are untouched
@answer n=16 entity=standard-contracts#4.1 v=no
why: the bridge's handle_event writes the reached record before runtime.handle_event and passes the event value on unchanged, so the bridge converter's carry holds, and the blitz-dom driver is not reached
@answer n=17 entity=standard-contracts#4.2 v=no
why: the session reaches no blitz-dom file, and the select list's default actions are untouched
@answer n=18 entity=standard-contracts#4.3 v=no
why: the session reaches no blitz-dom file, and the label arm's focus-only behaviour is untouched
@answer n=19 entity=standard-contracts#4.4 v=no
why: the session reaches no blitz-dom file, and the checkable change dispatch is untouched
@answer n=20 entity=standard-contracts#5.1 v=no
why: the session changes no scroll, box or harness-helper reading, and its common module gains only HANDLERS_AT_BOOT, UNEXERCISED_AFTER_DRIVES and grid_cells
@answer n=21 entity=standard-contracts#6.1 v=no
why: the session reaches no blitz-dom file, and the client rect under transforms and its dioxus readers are untouched
@answer n=22 entity=standard-contracts#6.2 v=no
why: the session reaches no blitz-dom file, and visible_region under transforms is untouched
@answer n=23 entity=standard-contracts#6.3 v=no
why: the session reaches no blitz-dom file, and text_extent is untouched
@answer n=24 entity=standard-contracts#6.4 v=no
why: the session reaches no blitz-dom file, and the hit walk's dropped-node skip is untouched
@answer n=25 entity=standard-contracts#6.5 v=no
why: the session reaches no blitz-dom file, and the non-invertible transform rule is untouched
