@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md placement, which no build or chunk code reads.
@answer n=2 entity=established-decisions#12.1 v=no
why: overlay scrollbar pointer hazard in events/pointer.rs, and the chunk drives no click after a scroll.
@answer n=3 entity=established-decisions#13.1 v=no
why: layout and damage mechanics the buffer does not touch, and the re-verified incremental identity is witnessed by incremental_oracle, which this rule does not name.
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio height in compute_replaced_layout, a layout rule the chunk does not touch.
@answer n=5 entity=established-decisions#14.2 v=no standing=S2
why: every test of this kind runs both layout modes through a for incremental in [false, true] loop, owed by each new test.
@answer n=6 entity=established-decisions#14.3 v=yes
why: set_focus, now reachable during the vdom render, can focus a text input before its editor is constructed, the case where IME is enabled at construction.
@answer n=7 entity=established-decisions#15.1 v=yes
why: a node the bridge removes is dropped with its descendants and cleared from the id mappings, and interaction state on removed nodes is cleared, both of which the replayed buffer must keep.
@answer n=8 entity=established-decisions#16.1 v=no
why: inert skips in the Tab walk, autofocus, unkeyed_actionable and hit_inner, none named by the scope's set_focus path, and unkeyed_actionable is an unedited unit of dioxus_document.rs.
@answer n=9 entity=established-decisions#16.2 v=yes
why: DocumentMutator::flush's focus fixup on inert nodes runs on the Drop the buffer replay runs under, which the scope's flush-on-Drop names.
@answer n=10 entity=established-decisions#16.3 v=yes
why: set_focus_to, FOCUS_WITHIN, clear_focus and the in-document redraw request are the verbs and effects the ported test and the inverted pins drive.
@answer n=11 entity=established-decisions#16.4 v=yes
why: flush settles recorded selects on the same Drop the replay runs under, and Dioxus static-template clones and dynamic selected writes pass through the replay.
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup recording and label rules in select.rs, which the chunk does not touch and the flush-on-Drop scope does not name.
@answer n=13 entity=established-decisions#16.6 v=no
why: optgroup header painting in an open select list, and paint is not reached.
@answer n=14 entity=established-decisions#16.7 v=no
why: default-sheet optgroup and multiple-row display rules, and the chunk edits no stylesheet and paints nothing.
@answer n=15 entity=established-decisions#16.8 v=no
why: multiple-select size hint through size_rows, and select layout is not reached.
@answer n=16 entity=established-decisions#17.1 v=no
why: hit test and paint-tree skip of a dropped node, which the chunk does not change, with removal itself carried by #15.1.
@answer n=17 entity=established-decisions#17.2 v=no
why: :open state on details and dialog, and no open-state write or selector is in the chunk.
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing options and template fragments, and the Dioxus bridge builds nodes by mutations without parsing HTML.
@answer n=19 entity=established-decisions#19.1 v=no
why: windowing, AccessKit and surface lifecycle in blitz-shell, which the chunk does not change.
@answer n=20 entity=established-decisions#20.1 v=no
why: the shell's redraw route through its event loop, which the chunk leaves as it is while keeping the same mutator redraw calls.
@answer n=21 entity=established-decisions#21.1 v=no
why: painting layers, culling and masks, and paint is not reached.
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling on the scaled size, and paint is not reached.
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript on Boa and external script fetching, and a Dioxus document runs no page script.
@answer n=24 entity=established-decisions#24.1 v=yes
why: DioxusDocument's default base_url and single user-agent sheet, which initial_build must keep as the document is built, and the CustomWidgetAttr on an object's data, which the buffer must carry through.
@answer n=25 entity=established-decisions#25.1 v=yes
why: the RefCell hazard of poll holding the document across render_immediate while a queued task calls set_focus, which this chunk fixes and the probe pins.
