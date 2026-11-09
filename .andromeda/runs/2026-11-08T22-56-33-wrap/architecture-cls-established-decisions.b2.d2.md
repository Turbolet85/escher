@answer n=1 entity=established-decisions#11.16 v=no
why: the AGENTS.md placement rule concerns upstream sync files and build inputs, which the session does not reach
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar hazard lives in blitz-dom hit testing and stand_cells_served, neither reached, and the session adds no scroll or advance step
@answer n=3 entity=established-decisions#13.1 v=no
why: the table, replaced-element and float layout rules live in unreached blitz-dom layout files
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio sizing lives in unreached blitz-dom layout/replaced.rs
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification lives in unreached blitz-dom construct.rs and its own test file, and the new tests run both layout modes so the convention holds
@answer n=6 entity=established-decisions#14.3 v=yes mode=stale
why: its citation of dioxus_document.rs:786-820 for input_focused_on_mount_enables_ime sits in a file the session reaches whole, so the cited lines may move
@answer n=7 entity=established-decisions#15.1 v=yes mode=stale
why: its citation of dioxus_document.rs:559-593 for the bridge's removal mapping clearing sits in a file the session reaches whole, so the cited lines may move
@answer n=8 entity=established-decisions#16.1 v=no
why: the inert rule lives in unreached blitz-dom node and accessibility code, and the session states DioxusDocument::unkeyed_actionable unchanged
@answer n=9 entity=established-decisions#16.2 v=no
why: the focus fixup in DocumentMutator::flush lives in unreached blitz-dom mutator.rs
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within handling lives in unreached blitz-dom document.rs and mutator.rs
@answer n=11 entity=established-decisions#16.4 v=no
why: select choice and selectedness rules live in unreached blitz-dom node/select.rs, mutator.rs and form.rs
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup handling lives in unreached blitz-dom select.rs, form.rs and mutator.rs
@answer n=13 entity=established-decisions#16.6 v=no
why: the open select group header paint lives in unreached blitz-paint text.rs
@answer n=14 entity=established-decisions#16.7 v=no
why: the default sheet's optgroup and multiple rules live in unreached blitz-dom assets/default.css
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple select size hint lives in unreached blitz-dom select_list.rs and stylo.rs
@answer n=16 entity=established-decisions#17.1 v=no
why: the hit-test skip of dropped nodes lives in unreached blitz-dom code
@answer n=17 entity=established-decisions#17.2 v=no
why: the open-state rule lives in unreached blitz-dom element.rs and mutator.rs
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing settings live in unreached blitz-html and blitz-dom config.rs
@answer n=19 entity=established-decisions#19.1 v=no
why: windowing and shell rules live in unreached blitz-shell and blitz-traits files
@answer n=20 entity=established-decisions#20.1 v=no
why: the windowed shell redraw path lives in unreached blitz-shell files
@answer n=21 entity=established-decisions#21.1 v=no
why: layer, culling and border painting rules live in unreached blitz-paint files
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling live in unreached blitz-paint render.rs
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript runtime rules live in unreached blitz-vibey-script and apps/browser files
@answer n=24 entity=established-decisions#24.1 v=yes mode=stale
why: its base_url and default stylesheet citations at dioxus_document.rs:111-115 sit in a file the session reaches whole, so the cited lines may move
@answer n=25 entity=established-decisions#25.1 v=yes mode=stale
why: its buffered-render citations at dioxus_document.rs:309-311 and its named merge surfaces dioxus_document.rs and lib.rs are files the session reaches
