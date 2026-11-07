@answer n=1 entity=established-decisions#11.16 v=no
why: the AGENTS.md exclusion concerns an upstream instruction file the session never touches
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay-scrollbar hazard lives in blitz-dom event code, which the session's files do not reach
@answer n=3 entity=established-decisions#13.1 v=no
why: the table, replaced-element and damage layout rules sit in blitz-dom layout, which the session does not reach
@answer n=4 entity=established-decisions#14.1 v=no
why: the replaced-element aspect-ratio rule lives in blitz-dom replaced layout, outside the reached files
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification lives in blitz-dom construct code, outside the reached files
@answer n=6 entity=established-decisions#14.3 v=yes mode=stale
why: its citation dioxus_document.rs:755-789 for input_focused_on_mount_enables_ime sits after the 12 added lines above 755 in the bridge file, so it moves to about 767-801
@answer n=7 entity=established-decisions#15.1 v=yes mode=stale
why: its citation dioxus_document.rs:528-562 for the bridge's removal and mapping clearing sits after the added lines in the bridge file and has shifted by 12 lines
@answer n=8 entity=established-decisions#16.1 v=no
why: inert handling lives in blitz-dom node and accessibility code that the session does not reach
@answer n=9 entity=established-decisions#16.2 v=no
why: the HTML focus fixup lives in the blitz-dom mutator, outside the reached files
@answer n=10 entity=established-decisions#16.3 v=yes mode=add-to
why: it records set_focus_to returning false for a node not in the document, and F3 extends it: a task polled during render sees the previous render, so a node created in that render is not yet there and set_focus_to returns false for it
@answer n=11 entity=established-decisions#16.4 v=no
why: select choice rules live in blitz-dom select code and the mutator, outside the reached files
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup rules live in blitz-dom select code, outside the reached files
@answer n=13 entity=established-decisions#16.6 v=no
why: the select header paint lives in blitz-paint text code, which the session does not reach
@answer n=14 entity=established-decisions#16.7 v=no
why: the default-sheet select rules live in blitz-dom assets, which the session does not reach
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple-select size hint lives in blitz-dom select_list code, outside the reached files
@answer n=16 entity=established-decisions#17.1 v=no
why: the hit-test skip of dropped nodes lives in blitz-dom, outside the reached files
@answer n=17 entity=established-decisions#17.2 v=no
why: the :open state lives in blitz-dom element code, outside the reached files
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing rules live in blitz-html and blitz-dom config, outside the reached files
@answer n=19 entity=established-decisions#19.1 v=no
why: the windowing rules live in blitz-shell, which the session does not reach
@answer n=20 entity=established-decisions#20.1 v=no
why: the windowed shell's redraw path lives in blitz-shell, and the bridge's single flush leaves redraw requests unchanged
@answer n=21 entity=established-decisions#21.1 v=no
why: the paint masking and layer rules live in blitz-paint and blitz-dom document code, neither reached by the session
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element culling lives in blitz-paint, which the session does not reach
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript runtime and browser loader lie outside the session's files
@answer n=24 entity=established-decisions#24.1 v=yes mode=stale
why: its citation dioxus_document.rs:109-113 for the base_url default sits after the 3 added lines above it in the bridge file, so it shifts to about 112-116
@answer n=25 entity=established-decisions#25.1 v=yes mode=contradict
why: the buffered render on the 0.7 bridge closes the hazard, so the record's standing hazard, its no-engine-change status, its should_panic pins and its route-entry owner are no longer true
