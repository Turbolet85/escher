@answer n=1 entity=input-validation#1.76 v=no
why: the change reaches only the Dioxus render path and its mounted-event flush, not the engine's click-driven checkbox and radio change dispatch in blitz-dom pointer.rs
@answer n=2 entity=input-validation#1.77 v=yes mode=stale
why: the buffered render adds 12 lines above dioxus_document.rs:407-412, so the cited choice-dispatch lines now read 419-424
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field and its readers live in blitz-dom document.rs and pointer.rs, none reached by the render-path change
@answer n=4 entity=input-validation#1.79 v=no
why: open-list key handling lives in blitz-dom keyboard.rs, outside the reached dioxus-native-dom files
@answer n=5 entity=input-validation#1.80 v=no
why: the pointer, close and reconcile paths sit in blitz-dom pointer.rs, document.rs and resolve.rs, none reached by the change
@answer n=6 entity=input-validation#1.81 v=no
why: hostile-markup box bounds and MAX_ROWS are in blitz-dom and the select_popup tests, none reached by the change
@answer n=7 entity=input-validation#1.82 v=no
why: multiple-select keys and clicks are in blitz-dom keyboard.rs and pointer.rs, not reached by the change
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup handling lives in blitz-dom form.rs, node/select.rs and mutator.rs, none reached by the change
@answer n=9 entity=input-validation#1.84 v=yes mode=stale
why: the buffered render adds one line at dioxus_document.rs:7, so the cited multiple_select_values lines at dioxus_document.rs:32-46 now read 33-47
@answer n=10 entity=input-validation#1.85 v=no
why: the driver's type on a select lives in escher-driver execute.rs and schema.rs, which the change does not reach
@answer n=11 entity=input-validation#1.86 v=yes mode=stale
why: the one mod line added to the reached dioxus-native-dom lib.rs is a merge surface that can move the re-export lines cited at lib.rs:36-39, and no report shows they still hold
@answer n=12 entity=input-validation#1.87 v=no
why: the open_select and list readers sit in blitz-dom document.rs and blitz-dom lib.rs, not in a reached dioxus-native-dom file
@answer n=13 entity=input-validation#1.88 v=no
why: the password and file mask reader sits in snapshot.rs and the paint and driver paths, none reached by the render-path change
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio handling is in blitz-dom layout/replaced.rs, not reached by the change
@answer n=15 entity=input-validation#1.90 v=no
why: the inline-box height clamp is in blitz-dom layout/inline/layout.rs, not reached by the change
@answer n=16 entity=input-validation#1.91 v=no
why: the text_extent reader is in blitz-dom text_extent.rs, not reached by the change
@answer n=17 entity=input-validation#1.92 v=no
why: visible_region is in blitz-dom scrolling.rs, not reached by the change
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling is in blitz-paint render.rs and blitz-dom construct.rs, not reached by the change
@answer n=19 entity=input-validation#1.94 v=no
why: select-list painting is in blitz-paint render.rs and render/select_list.rs, not reached by the change
@answer n=20 entity=input-validation#1.95 v=no
why: the non-invertible transform hit rule is in blitz-dom node/node.rs, not reached by the change
@answer n=21 entity=input-validation#1.96 v=no
why: gradient length clamping is in blitz-paint gradient.rs, not reached by the change
@answer n=22 entity=input-validation#1.97 v=no
why: border-radius scaling is in blitz-paint kurbo_css/css_box.rs, not reached by the change
@answer n=23 entity=input-validation#1.98 v=no
why: the hidden-input paint skip is in blitz-paint render.rs, not reached by the change
@answer n=24 entity=input-validation#1.99 v=no
why: the target clamp and missing-node scroll return are in blitz-dom scrolling.rs, not reached, and the missing-node return stays unchanged
@answer n=25 entity=input-validation#1.100 v=no
why: an open select list's own scroll is in blitz-dom scrolling.rs and node/select_list.rs, not reached by the change
