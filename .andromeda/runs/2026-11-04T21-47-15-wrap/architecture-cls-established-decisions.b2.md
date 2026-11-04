@answer n=1 entity=established-decisions#11.16 v=no
why: the AGENTS.md note cites no file the session reached, and the session adds no agent-doc or tree-level file
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar hazard cites events/pointer.rs and document.rs under packages/, which the session left unchanged
@answer n=3 entity=established-decisions#13.1 v=no
why: the layout record cites packages/blitz-dom and layout tests the session did not reach, and no test count it carries moved
@answer n=4 entity=established-decisions#14.1 v=no
why: the replaced-element aspect-ratio rule cites replaced.rs under packages/blitz-dom, unchanged by the session
@answer n=5 entity=established-decisions#14.2 v=no
why: the pseudo-element flow rule cites construct.rs and its own test file, neither reached by the session's files
@answer n=6 entity=established-decisions#14.3 v=no
why: the IME enablement cites construct.rs and dioxus_document.rs under packages/, unchanged by the session
@answer n=7 entity=established-decisions#15.1 v=no
why: the DOM-semantics record cites packages/ sources and tests other than the new target, and the session reaches none of them
@answer n=8 entity=established-decisions#16.1 v=no
why: the inert rule cites packages/blitz-dom sources and inert_subtree.rs, none changed or reached by the session
@answer n=9 entity=established-decisions#16.2 v=no
why: the focus fixup cites mutator.rs and inert_subtree.rs, untouched by the session
@answer n=10 entity=established-decisions#16.3 v=no
why: the focus-within rule cites document.rs and mutator.rs lines the session did not change
@answer n=11 entity=established-decisions#16.4 v=no
why: the select choice rules cite node/select.rs, mutator.rs, form.rs and default.css under packages/, all unchanged by the session
@answer n=12 entity=established-decisions#16.5 v=no
why: the optgroup rules cite select.rs and form.rs under packages/, unchanged by the session
@answer n=13 entity=established-decisions#16.6 v=no
why: the open-list group header cites blitz-paint text.rs and select popup tests, none reached by the session
@answer n=14 entity=established-decisions#16.7 v=no
why: the default-sheet optgroup and multiple rules cite default.css under packages/, unchanged by the session
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple-select size hint cites select_list.rs, stylo.rs and default.css under packages/, unchanged by the session
@answer n=16 entity=established-decisions#17.1 v=no
why: the dropped-node hit-test skip cites packages/ sources the session did not change
@answer n=17 entity=established-decisions#17.2 v=no
why: the :open state rule cites element.rs, mutator.rs and stylo.rs under packages/, unchanged by the session
@answer n=18 entity=established-decisions#18.1 v=no
why: the HTML parsing rules cite packages/blitz-html and blitz-dom config.rs, none changed or reached by the session
@answer n=19 entity=established-decisions#19.1 v=no
why: the shell and windowing rules cite packages/blitz-shell and blitz-traits, none changed by the session
@answer n=20 entity=established-decisions#20.1 v=no
why: the redraw routing cites blitz-shell lib.rs, window.rs and event.rs, none changed by the session
@answer n=21 entity=established-decisions#21.1 v=no
why: the painting limits cite blitz-paint sources the session did not change
@answer n=22 entity=established-decisions#22.1 v=no
why: the render_element border box rule cites blitz-paint render.rs, unchanged by the session
@answer n=23 entity=established-decisions#23.1 v=no
why: the JavaScript rules cite blitz-vibey-script and browser sources the session did not reach
@answer n=24 entity=established-decisions#24.1 v=yes mode=add-to
why: the Dioxus integration record gains F2 (Dioxus 0.7.10 public API reads a node's owning component with no fork-level change), F5 (a GlobalSignal's readers reachable through public API, hook-local use_signal state not) and F7 (the founder's answers for The UI graph)
@answer n=25 entity=established-decisions#25.1 v=yes mode=add-to
why: the MountedData hazard record gains F1, the hazard pinned as the two should_panic tests of dioxus_runtime_probe in both layout modes, with its coordinates re-measured at HEAD and its fix routed to a port chunk
