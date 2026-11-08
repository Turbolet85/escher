@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md stays out of the tree and the session adds no file or build that reads it.
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar hazard is blitz-dom pointer behaviour the session leaves alone, and the Cells walk it cites is in no reached file.
@answer n=3 entity=established-decisions#13.1 v=no
why: its layout claims live in blitz-dom and blitz-html files the session does not reach.
@answer n=4 entity=established-decisions#14.1 v=no
why: the replaced-element aspect-ratio rule lives in blitz-dom replaced.rs, which the session does not reach.
@answer n=5 entity=established-decisions#14.2 v=no
why: the pseudo-element flow rule lives in blitz-dom construct.rs and its tests, none of which the session reaches.
@answer n=6 entity=established-decisions#14.3 v=yes mode=stale
why: dioxus_document.rs, which holds the cited input_focused_on_mount_enables_ime test at lines 775-809, is reached whole by the session, so those lines can have moved.
@answer n=7 entity=established-decisions#15.1 v=yes mode=add-to
why: snapshot.rs gains NodeState.clickable, which reads ElementData::is_disabled, a reader this disabled rule's list of readers does not yet carry.
@answer n=8 entity=established-decisions#16.1 v=no
why: the inert subtree leaves the accessibility tree and so the snapshot, and the session changes no inert rule.
@answer n=9 entity=established-decisions#16.2 v=no
why: the inert focus fixup lives in blitz-dom mutator.rs, which the session does not reach.
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within marking lives in blitz-dom document.rs and mutator.rs, neither reached, and the session only reads focus for the clickable token.
@answer n=11 entity=established-decisions#16.4 v=no
why: the select rules live in blitz-dom node/select.rs, form.rs and default.css, none reached, and its click stays with scrolling and input.
@answer n=12 entity=established-decisions#16.5 v=no
why: the optgroup rules live in blitz-dom select.rs and blitz-paint text.rs, neither reached.
@answer n=13 entity=established-decisions#16.6 v=no
why: the group header painting lives in blitz-paint text.rs, which the session does not reach.
@answer n=14 entity=established-decisions#16.7 v=no
why: the default sheet rules live in blitz-dom assets/default.css, which the session does not reach.
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple-select size hint lives in blitz-dom select_list.rs and stylo.rs, neither reached.
@answer n=16 entity=established-decisions#17.1 v=no
why: the dropped-node hit-test skip is blitz-dom engine code the session does not reach.
@answer n=17 entity=established-decisions#17.2 v=no
why: the open state lives in blitz-dom element.rs and mutator.rs, neither reached.
@answer n=18 entity=established-decisions#18.1 v=no
why: its HTML parsing and feature-gating rules live in blitz-html and blitz-dom, none reached, and the new graph gating names no parser rule.
@answer n=19 entity=established-decisions#19.1 v=no
why: the windowed shell in blitz-shell is not reached and the session adds no window or AccessKit change.
@answer n=20 entity=established-decisions#20.1 v=no
why: the redraw path through blitz-shell's event loop is not reached and the session changes no redraw request.
@answer n=21 entity=established-decisions#21.1 v=no
why: the painting rules live in blitz-paint and blitz-dom, none reached by the session.
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element lives in blitz-paint, which the session does not reach.
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript runtime and the browser app are untouched by the chunk.
@answer n=24 entity=established-decisions#24.1 v=yes mode=add-to
why: the bridge's event dispatch now sets the SignalRegistry dispatch around runtime.handle_event when a registry exists, which this Dioxus integration decision must carry.
@answer n=25 entity=established-decisions#25.1 v=yes mode=stale
why: its named merge surfaces dioxus_document.rs and lib.rs are reached by the session and its cited lines there can have moved, though the buffered render itself stays unchanged.
