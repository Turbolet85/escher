@answer n=1 entity=input-validation#1.76 v=no
why: its checkbox and radio change rule lives in blitz-dom pointer.rs and blitz-vibey-script, which the session did not change, and the bridge dispatch edit is inert without a registry
@answer n=2 entity=input-validation#1.77 v=yes mode=stale
why: it cites schema.rs:246-249 and snapshot.rs:257-281, reached files the session moved, since snapshot.rs gained lines ahead of them and schema.rs gained the graph verb
@answer n=3 entity=input-validation#1.78 v=no
why: the select list field and its readers live in blitz-dom document.rs and resolve.rs, none of which the session reached
@answer n=4 entity=input-validation#1.79 v=no
why: open-list key matching lives in blitz-dom keyboard.rs and pointer.rs, untouched by the session
@answer n=5 entity=input-validation#1.80 v=no
why: the pointer and close rules live in blitz-dom pointer.rs, document.rs and resolve.rs, untouched by the session
@answer n=6 entity=input-validation#1.81 v=no
why: the hostile-markup rule lives in blitz-dom and its test select_popup.rs, neither reached by the session
@answer n=7 entity=input-validation#1.82 v=no
why: multiple-select keys and clicks live in blitz-dom keyboard.rs and pointer.rs, untouched by the session
@answer n=8 entity=input-validation#1.83 v=no
why: optgroup handling lives in blitz-dom node/select.rs, mutator.rs and form.rs, none reached by the session
@answer n=9 entity=input-validation#1.84 v=yes mode=stale
why: it cites dioxus_document.rs:33-47, a reached file the session edited, so those lines may have moved and the claim cannot be shown to hold
@answer n=10 entity=input-validation#1.85 v=yes mode=stale
why: it cites execute.rs:571-577 and 242-246 and schema.rs:247-249, reached files whose verb arm and table the graph verb changed
@answer n=11 entity=input-validation#1.86 v=yes mode=stale
why: it cites snapshot.rs:254-264 and dioxus-native-dom lib.rs:45-48, reached files that moved, since snapshot.rs gained lines ahead of 254 and lib.rs gained the new modules
@answer n=12 entity=input-validation#1.87 v=yes mode=stale
why: its named caller, the driver's focus_select, sits in execute.rs, a reached file, so the caller claim cannot be shown to hold after the change
@answer n=13 entity=input-validation#1.88 v=yes mode=stale
why: it cites snapshot.rs:278-305, snapshot_text.rs:53 and snapshot_diff.rs:111, whose lines moved as the file-input masking row's citations, and the graph answer is a new carrier the mask must cover
@answer n=14 entity=input-validation#1.89 v=no
why: aspect-ratio handling lives in blitz-dom layout/replaced.rs, which the session did not reach
@answer n=15 entity=input-validation#1.90 v=no
why: the inline-box height clamp lives in blitz-dom layout/inline/layout.rs, which the session did not reach
@answer n=16 entity=input-validation#1.91 v=yes mode=stale
why: its named callers, Session::lint and the lint verb's arm, are driver code in files the session's change reached, so the caller claim cannot be shown to hold
@answer n=17 entity=input-validation#1.92 v=yes mode=stale
why: its callers execute.rs in_view and crop sit in execute.rs, a reached file the session edited, so the caller claim cannot be shown to hold
@answer n=18 entity=input-validation#1.93 v=no
why: geometry culling lives in blitz-paint render.rs and blitz-dom construct.rs and document.rs, none reached by the session
@answer n=19 entity=input-validation#1.94 v=no
why: select-list painting lives in blitz-paint and blitz-dom, neither reached by the session
@answer n=20 entity=input-validation#1.95 v=no
why: the non-invertible transform hit rule lives in blitz-dom node.rs, which the session did not reach
@answer n=21 entity=input-validation#1.96 v=no
why: gradient clamping lives in blitz-paint gradient.rs, which the session did not reach
@answer n=22 entity=input-validation#1.97 v=no
why: border radius scaling lives in blitz-paint kurbo_css, which the session did not reach
@answer n=23 entity=input-validation#1.98 v=no
why: the hidden-input paint rule lives in blitz-paint render.rs, which the session did not reach
@answer n=24 entity=input-validation#1.99 v=no
why: scroll target clamping lives in blitz-dom scrolling.rs, which the session did not reach
@answer n=25 entity=input-validation#1.100 v=no
why: the select list's own scroll lives in blitz-dom scrolling.rs and node/select_list.rs, neither reached by the session
