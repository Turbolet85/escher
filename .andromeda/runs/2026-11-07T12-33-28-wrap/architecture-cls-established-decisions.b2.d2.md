@answer n=1 entity=established-decisions#11.16 v=no
why: the record concerns an AGENTS.md file the session never touches
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar hazard lives in blitz-dom's pointer code, which the session does not reach
@answer n=3 entity=established-decisions#13.1 v=no
why: the layout rules live in blitz-dom's layout modules, none of the session's four files
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio sizing lives in blitz-dom's replaced.rs, outside the session's files
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification lives in blitz-dom's construct.rs, outside the session's files
@answer n=6 entity=established-decisions#14.3 v=yes mode=stale
why: the bridge document file, a merge surface, gains 12 lines above the test, so its cited range dioxus_document.rs:755-789 has moved
@answer n=7 entity=established-decisions#15.1 v=yes mode=stale
why: the bridge document file gains 12 lines above line 528, so its cited range dioxus_document.rs:528-562 has moved
@answer n=8 entity=established-decisions#16.1 v=no
why: inert handling lives in blitz-dom's node.rs and accessibility.rs, which the session does not reach
@answer n=9 entity=established-decisions#16.2 v=no
why: the focus fixup lives in blitz-dom's mutator.rs, which the session does not reach
@answer n=10 entity=established-decisions#16.3 v=yes mode=add-to
why: F3 adds that a task polled during render sees a node created in the same render as absent, so set_focus_to returns false for it, beside this record's not-in-the-document return
@answer n=11 entity=established-decisions#16.4 v=no
why: the select choice rules live in blitz-dom's select.rs and mutator.rs, which the session does not reach
@answer n=12 entity=established-decisions#16.5 v=no
why: the optgroup rules live in blitz-dom's select.rs and form.rs, which the session does not reach
@answer n=13 entity=established-decisions#16.6 v=no
why: the select group header paint lives in blitz-paint's text.rs, which the session does not reach
@answer n=14 entity=established-decisions#16.7 v=no
why: the optgroup and multiple row rules live in blitz-dom's default.css, which the session does not reach
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple select size hint lives in blitz-dom's select_list.rs, which the session does not reach
@answer n=16 entity=established-decisions#17.1 v=no
why: the dropped-node hit test skip lives in blitz-dom's hit-test code, which the session does not reach
@answer n=17 entity=established-decisions#17.2 v=no
why: the open state lives in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing lives in blitz-html and blitz-dom's config.rs, which the session does not reach
@answer n=19 entity=established-decisions#19.1 v=no
why: the windowing and shell rules live in blitz-shell, which the session does not reach
@answer n=20 entity=established-decisions#20.1 v=no
why: the redraw path lives in blitz-shell's event loop, which the session does not reach
@answer n=21 entity=established-decisions#21.1 v=no
why: the painting limits live in blitz-paint, which the session does not reach
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element's border box and culling live in blitz-paint's render.rs, which the session does not reach
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript runtime lives in blitz-vibey-script and the browser app, which the session does not reach
@answer n=24 entity=established-decisions#24.1 v=yes mode=stale
why: the bridge document file gains 3 lines above line 109, so its cited range dioxus_document.rs:109-113 has moved
@answer n=25 entity=established-decisions#25.1 v=yes mode=contradict
why: the buffered render on the 0.7 bridge closes the borrow hazard this record states as standing and owns to a route entry
