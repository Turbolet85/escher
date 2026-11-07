@answer n=1 entity=input-validation#1.76 v=no
why: the session's buffered render and its bridge do not reach handle_click's checkbox and radio change dispatch in blitz-dom's pointer.rs
@answer n=2 entity=input-validation#1.77 v=yes mode=stale
why: the session added 12 lines above dioxus_document.rs:407-412, the select choice's input and change dispatch carrying select_value, so that citation moved
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field and its readers live in blitz-dom's document.rs, which the session does not reach
@answer n=4 entity=input-validation#1.79 v=no
why: an open list's key handling lives in blitz-dom's keyboard.rs, which the session does not reach
@answer n=5 entity=input-validation#1.80 v=no
why: a pointer in or around an open list is handled in blitz-dom's pointer.rs and resolve.rs, which the session does not reach
@answer n=6 entity=input-validation#1.81 v=no
why: hostile select markup and the list's row cap sit in blitz-dom's select list code, which the session does not reach
@answer n=7 entity=input-validation#1.82 v=no
why: a multiple select's keys and clicks live in blitz-dom's keyboard.rs and pointer.rs, which the session does not reach
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup handling lives in blitz-dom's node/select.rs and mutator.rs, which the session does not reach
@answer n=9 entity=input-validation#1.84 v=yes mode=stale
why: dioxus_document.rs:32-46, where multiple_select_values fills the event values, moved by the session's added line 7 above it
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type on a select lives in escher-driver's execute.rs, which the session does not reach
@answer n=11 entity=input-validation#1.86 v=yes mode=stale
why: lib.rs is a merge surface for the session's one added mod line, so its re-export citation lib.rs:36-39 may have moved and is re-read
@answer n=12 entity=input-validation#1.87 v=no
why: the select list's public readers and writer live in blitz-dom's document.rs and lib.rs:106-109, which the session does not reach
@answer n=13 entity=input-validation#1.88 v=no
why: the password and file mask is read in dioxus-native-dom's snapshot.rs, snapshot_text.rs and snapshot_diff.rs, none reached, and the buffer replays the same DOM so the masked reads hold
@answer n=14 entity=input-validation#1.89 v=no
why: aspect ratio clamps live in blitz-dom's layout/replaced.rs, which the session does not reach
@answer n=15 entity=input-validation#1.90 v=no
why: inline-box height clamps live in blitz-dom's layout/inline/layout.rs, which the session does not reach
@answer n=16 entity=input-validation#1.91 v=no
why: text_extent lives in blitz-dom's text_extent.rs, which the session does not reach
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region lives in blitz-dom's scrolling.rs and document.rs, which the session does not reach
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling lives in blitz-paint's render.rs and blitz-dom's construct.rs, which the session does not reach
@answer n=19 entity=input-validation#1.94 v=no
why: the select list's paint lives in blitz-paint's select_list.rs, which the session does not reach
@answer n=20 entity=input-validation#1.95 v=no
why: hit testing under a non-invertible transform lives in blitz-dom's node.rs, which the session does not reach
@answer n=21 entity=input-validation#1.96 v=no
why: gradient clamps live in blitz-paint's gradient.rs, which the session does not reach
@answer n=22 entity=input-validation#1.97 v=no
why: radius scaling lives in blitz-paint's css_box.rs, which the session does not reach
@answer n=23 entity=input-validation#1.98 v=no
why: hidden-input paint skipping lives in blitz-paint's render.rs, which the session does not reach
@answer n=24 entity=input-validation#1.99 v=no
why: scroll target clamping lives in blitz-dom's scrolling.rs, which the session does not reach
@answer n=25 entity=input-validation#1.100 v=no
why: the select list's own scroll lives in blitz-dom's scrolling.rs and node/select_list.rs, which the session does not reach
