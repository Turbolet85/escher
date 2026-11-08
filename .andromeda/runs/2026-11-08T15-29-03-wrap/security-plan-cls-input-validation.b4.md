@answer n=1 entity=input-validation#1.76 v=no
why: the select opening rule lives in blitz-dom document.rs and select_popup tests, which the session did not reach and whose rule it does not change
@answer n=2 entity=input-validation#1.77 v=no
why: the inert subtree focus rule lives in blitz-dom accessibility.rs and mutator.rs, which the session did not reach
@answer n=3 entity=input-validation#1.78 v=no
why: the range input rules live in blitz-dom and snapshot.rs files the session did not reach, and the actions answer carries no control value
@answer n=4 entity=input-validation#1.79 v=no
why: keyboard activation lives in blitz-dom events files the session did not reach and changes nothing in them
@answer n=5 entity=input-validation#1.80 v=no
why: the checkbox and radio change event lives in blitz-dom pointer.rs, which the session did not reach
@answer n=6 entity=input-validation#1.81 v=yes mode=stale
why: its cited help text at packages/escher-driver/src/schema.rs:246-249 sits in a reached file the session changed whole, so the cited lines can have moved
@answer n=7 entity=input-validation#1.82 v=no
why: the select list state field lives in blitz-dom document.rs, which the session did not reach
@answer n=8 entity=input-validation#1.83 v=no
why: the open list key handling lives in blitz-dom keyboard.rs and pointer.rs, which the session did not reach
@answer n=9 entity=input-validation#1.84 v=no
why: the pointer rules around an open list live in blitz-dom pointer.rs and document.rs, which the session did not reach
@answer n=10 entity=input-validation#1.85 v=no
why: the hostile markup rule for an open list lives in blitz-dom and its tests, which the session did not reach
@answer n=11 entity=input-validation#1.86 v=no
why: the multiple select keys and clicks live in blitz-dom keyboard.rs and pointer.rs, which the session did not reach
@answer n=12 entity=input-validation#1.87 v=no
why: the optgroup rules live in blitz-dom node/select.rs, mutator.rs and form.rs, which the session did not reach
@answer n=13 entity=input-validation#1.88 v=no
why: the multiple select set leaves through dioxus_document.rs, which the session did not reach
@answer n=14 entity=input-validation#1.89 v=yes mode=stale
why: its driver type path cited at packages/escher-driver/src/execute.rs:644-650 and 251-255 and schema.rs:247-249 sits in reached files whose type arm now runs the shared check_target
@answer n=15 entity=input-validation#1.90 v=yes mode=stale
why: its re-export cited at packages/dioxus-native-dom/src/lib.rs:46-49 sits in a reached file, and its focus_select and locate callers sit in reached execute.rs
@answer n=16 entity=input-validation#1.91 v=no
why: the select list public readers and writer live in blitz-dom files the session did not reach, and the session adds no caller of close_select_list
@answer n=17 entity=input-validation#1.92 v=yes mode=stale
why: its kept check citation at packages/escher-driver/src/execute.rs:440-442 sits in a reached file the session changed whole, so the cited lines can have moved
@answer n=18 entity=input-validation#1.93 v=no
why: aspect ratio handling lives in blitz-dom layout/replaced.rs, which the session did not reach
@answer n=19 entity=input-validation#1.94 v=no
why: inline-box height clamping lives in blitz-dom layout/inline, which the session did not reach
@answer n=20 entity=input-validation#1.95 v=no
why: the text extent reader lives in blitz-dom text_extent.rs, which the session did not reach, and the session adds no text_extent caller
@answer n=21 entity=input-validation#1.96 v=yes mode=stale
why: its named callers execute.rs in_view and crop sit in a reached file whose screenshot path now runs them through the shared check_target
@answer n=22 entity=input-validation#1.97 v=no
why: geometry culling lives in blitz-paint render.rs, which the session did not reach
@answer n=23 entity=input-validation#1.98 v=no
why: the select list paint lives in blitz-paint render files, which the session did not reach
@answer n=24 entity=input-validation#1.99 v=no
why: the non-invertible transform hit rule lives in blitz-dom node.rs, which the session did not reach
@answer n=25 entity=input-validation#1.100 v=no
why: gradient length clamping lives in blitz-paint gradient.rs, which the session did not reach
