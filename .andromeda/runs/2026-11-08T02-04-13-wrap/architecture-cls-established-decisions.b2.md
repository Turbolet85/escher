@answer n=1 entity=established-decisions#11.16 v=no
why: the upstream AGENTS.md exclusion rests on the upstream sync and on no file the session reaches
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar hazard is an engine-level claim and the session does not change the stand_cells_served walk or its advance
@answer n=3 entity=established-decisions#13.1 v=no
why: the layout decision cites blitz-dom layout files that the session does not reach
@answer n=4 entity=established-decisions#14.1 v=no
why: the replaced-element aspect-ratio claim lives in blitz-dom replaced.rs, which the session does not reach
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification is a blitz-dom construct.rs claim, unchanged by the session
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enablement on a focused text editor is a blitz-dom construct.rs claim, unchanged by the session
@answer n=7 entity=established-decisions#15.1 v=no
why: the disabled rule lives in blitz-dom element and event files, and the session only reads ElementData::is_disabled without changing it
@answer n=8 entity=established-decisions#16.1 v=no
why: inert handling lives in blitz-dom node, hit and accessibility files, none reached by the session
@answer n=9 entity=established-decisions#16.2 v=no
why: the focus fixup on inert nodes is a blitz-dom mutator.rs claim, unchanged by the session
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within bookkeeping is a blitz-dom document.rs claim, unchanged by the session
@answer n=11 entity=established-decisions#16.4 v=no
why: select choice rules live in blitz-dom select.rs and mutator.rs, none reached by the session
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup rules live in blitz-dom select.rs and form.rs, none reached by the session
@answer n=13 entity=established-decisions#16.6 v=no
why: the optgroup header paint is a blitz-paint text.rs claim, unreached by the session
@answer n=14 entity=established-decisions#16.7 v=no
why: the default sheet rules live in blitz-dom default.css, unreached by the session
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple-select size hint lives in blitz-dom select_list.rs and stylo.rs, unreached by the session
@answer n=16 entity=established-decisions#17.1 v=no
why: the dropped-node hit test skip is a blitz-dom engine claim the session does not reach
@answer n=17 entity=established-decisions#17.2 v=no
why: the open state lives in blitz-dom element.rs and mutator.rs, none reached by the session
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing lives in blitz-html, which the session does not reach
@answer n=19 entity=established-decisions#19.1 v=no
why: the windowing and shell claims live in blitz-shell, which the session does not reach
@answer n=20 entity=established-decisions#20.1 v=no
why: redraw routing through the windowed event loop lives in blitz-shell, unchanged by the session
@answer n=21 entity=established-decisions#21.1 v=no
why: the painting limits live in blitz-paint, which the session does not reach
@answer n=22 entity=established-decisions#22.1 v=no
why: the render_element border box and culling live in blitz-paint, which the session does not reach
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript runtime and browser loader are untouched by the chunk
@answer n=24 entity=established-decisions#24.1 v=yes mode=add-to
why: the bridge event path now sets the SignalRegistry dispatch around runtime.handle_event where a registry exists, and DioxusDocument::graph joins the Dioxus integration in the reached dioxus_document.rs, a fact this decision must now carry
@answer n=25 entity=established-decisions#25.1 v=yes mode=stale
why: the event dispatch change and DioxusDocument::graph land in dioxus_document.rs and lib.rs, the merge surfaces this decision names, so its cited lines can move, though the buffered render itself is unchanged
