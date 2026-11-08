@answer n=1 entity=input-validation#1.76 v=no
why: its select_list_allowed rule lives in blitz-dom and the select_popup tests, which the session did not reach
@answer n=2 entity=input-validation#1.77 v=no
why: the inert-subtree focus rule lives in blitz-dom accessibility and mutator, which the session did not reach
@answer n=3 entity=input-validation#1.78 v=no
why: the range rules live in blitz-dom, and the actions answer carries no control value, so the range value's carriers stand
@answer n=4 entity=input-validation#1.79 v=no
why: keyboard activation lives in blitz-dom events, which the session did not reach, and the actions verb adds no key path
@answer n=5 entity=input-validation#1.80 v=no
why: the checkbox and radio change event lives in blitz-dom pointer.rs, which the session did not reach
@answer n=6 entity=input-validation#1.81 v=yes mode=stale
why: the session reached schema.rs, which this row cites at 246-249 for the type verb's help, and its verb table and FieldKind count moved
@answer n=7 entity=input-validation#1.82 v=no
why: the select list state lives in blitz-dom document.rs and resolve.rs, which the session did not reach
@answer n=8 entity=input-validation#1.83 v=no
why: the open list's keys live in blitz-dom keyboard and pointer, which the session did not reach
@answer n=9 entity=input-validation#1.84 v=no
why: the pointer in and around an open list lives in blitz-dom, which the session did not reach
@answer n=10 entity=input-validation#1.85 v=no
why: hostile select markup lives in blitz-dom and the select_popup tests, which the session did not reach
@answer n=11 entity=input-validation#1.86 v=no
why: multiple-select keys and clicks live in blitz-dom events, which the session did not reach
@answer n=12 entity=input-validation#1.87 v=no
why: the optgroup reading lives in blitz-dom select and mutator, which the session did not reach
@answer n=13 entity=input-validation#1.88 v=no
why: the multiple select's chosen set lives in dioxus_document.rs, which the session did not reach, and the actions answer carries no values
@answer n=14 entity=input-validation#1.89 v=yes mode=stale
why: the session reached execute.rs and schema.rs, which this row cites at 251-255, 644-650 and 247-249 for the type verb on a select
@answer n=15 entity=input-validation#1.90 v=yes mode=stale
why: the session reached dioxus-native-dom lib.rs, which this row cites at 46-49 for re-exports, and execute.rs, which it names for the readers' callers
@answer n=16 entity=input-validation#1.91 v=yes mode=stale
why: the session reached execute.rs, whose focus_select this row names as a caller of close_select_list, and the shared check_target now routes the type arm that calls it
@answer n=17 entity=input-validation#1.92 v=yes mode=stale
why: the session reached execute.rs, which this row cites at 440-442 for the recorded type's mask, and the actions answer carries no control value, so MASKED_VALUE occurs 0 times in it
@answer n=18 entity=input-validation#1.93 v=no
why: the aspect-ratio rules live in blitz-dom layout, which the session did not reach
@answer n=19 entity=input-validation#1.94 v=no
why: the inline-box height rules live in blitz-dom layout, which the session did not reach
@answer n=20 entity=input-validation#1.95 v=no
why: text_extent lives in blitz-dom, and the actions read goes through validate and check_target rather than Session::lint, so its one caller stands
@answer n=21 entity=input-validation#1.96 v=yes mode=stale
why: the session reached execute.rs, whose in_view and crop this row names as visible_region's callers, and the shared check_target now routes those checks
@answer n=22 entity=input-validation#1.97 v=no
why: the geometry culling rule lives in blitz-paint render.rs, which the session did not reach
@answer n=23 entity=input-validation#1.98 v=no
why: the select list painting lives in blitz-paint, which the session did not reach
@answer n=24 entity=input-validation#1.99 v=no
why: the non-invertible transform hit rule lives in blitz-dom node.rs, which the session did not reach
@answer n=25 entity=input-validation#1.100 v=no
why: the gradient clamping lives in blitz-paint gradient.rs, which the session did not reach
