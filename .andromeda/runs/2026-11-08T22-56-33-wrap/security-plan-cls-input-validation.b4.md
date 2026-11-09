@answer n=1 entity=input-validation#1.76 v=no
why: the session changes no blitz-dom select or accessibility file, and the select aria-expanded and open rule lives only there
@answer n=2 entity=input-validation#1.77 v=no
why: the aria state readers in blitz-dom accessibility.rs and snapshot.rs are outside the session's reach, and the coverage line carries no toggle or invalid state
@answer n=3 entity=input-validation#1.78 v=no
why: the native checkbox checked-state reader lives in blitz-dom accessibility.rs, which the session does not reach
@answer n=4 entity=input-validation#1.79 v=no
why: the inert presence check lives in blitz-dom node and mutator files the session does not reach
@answer n=5 entity=input-validation#1.80 v=no
why: select_list_allowed and the row rules live in blitz-dom document.rs and select files, none reached by the session
@answer n=6 entity=input-validation#1.81 v=no
why: the tree focus and the inert focus clear live in blitz-dom accessibility.rs and mutator.rs, which the session does not reach
@answer n=7 entity=input-validation#1.82 v=no
why: the range rules and value readers live in blitz-dom and snapshot.rs, none reached, and the coverage line carries no control value
@answer n=8 entity=input-validation#1.83 v=no
why: the Enter and Space default actions live in blitz-dom keyboard.rs and pointer.rs, which the session does not reach
@answer n=9 entity=input-validation#1.84 v=no
why: the checkbox and radio change arms live in blitz-dom pointer.rs, which the session does not reach
@answer n=10 entity=input-validation#1.85 v=yes mode=stale
why: the session's reached record is written in the reached file dioxus_document.rs handle_event beside the SignalRegistry dispatch, which can move the cited lines 421-426
@answer n=11 entity=input-validation#1.86 v=no
why: the select_list field and its readers live in blitz-dom document.rs, which the session does not reach
@answer n=12 entity=input-validation#1.87 v=no
why: the open list key match lives in blitz-dom keyboard.rs and pointer.rs, which the session does not reach
@answer n=13 entity=input-validation#1.88 v=no
why: the pointer in and around an open list lives in blitz-dom pointer.rs, document.rs and resolve.rs, none reached by the session
@answer n=14 entity=input-validation#1.89 v=no
why: the hostile-markup select rules live in blitz-dom select files the session does not reach
@answer n=15 entity=input-validation#1.90 v=no
why: the multiple select keys and clicks live in blitz-dom keyboard.rs and pointer.rs, which the session does not reach
@answer n=16 entity=input-validation#1.91 v=no
why: the optgroup reading lives in blitz-dom node/select.rs, mutator.rs and form.rs, none reached by the session
@answer n=17 entity=input-validation#1.92 v=yes mode=stale
why: the session reaches dioxus_document.rs whole, and its cited lines 34-48 for multiple_select_values can move with the new handle_event hook
@answer n=18 entity=input-validation#1.93 v=no
why: the type-on-select rule lives in escher-driver execute.rs and schema.rs, neither reached, and the coverage role adds no verb, result field or exit to type
@answer n=19 entity=input-validation#1.94 v=yes mode=stale
why: the session reaches dioxus-native-dom lib.rs whole and adds re-exports 35 to 36, so the cited lines 48-51 can move
@answer n=20 entity=input-validation#1.95 v=no
why: the open-list public readers and writer live in blitz-dom document.rs, select_list.rs and lib.rs, none reached by the session
@answer n=21 entity=input-validation#1.96 v=no
why: the password and file mask lives in snapshot, snapshot_text, snapshot_diff and blitz-paint, none reached, and the coverage line holds no value text
@answer n=22 entity=input-validation#1.97 v=no
why: the aspect-ratio clamps live in blitz-dom layout replaced.rs, which the session does not reach
@answer n=23 entity=input-validation#1.98 v=no
why: the inline-box height clamp lives in blitz-dom inline layout files the session does not reach
@answer n=24 entity=input-validation#1.99 v=no
why: text_extent lives in blitz-dom text_extent.rs, which the session does not reach
@answer n=25 entity=input-validation#1.100 v=no
why: visible_region lives in blitz-dom scrolling.rs and document.rs, which the session does not reach
