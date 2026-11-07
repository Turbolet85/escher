@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md staying out of the tree is a repository-layout rule the buffered render does not touch
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay scrollbar pointer hazard concerns driver clicks after a scroll, which the probe's focus pins do not make
@answer n=3 entity=established-decisions#13.1 v=yes
why: its damage-bit classification and relayout rules are what incremental_oracle's incremental and non-incremental identity re-measure reads
@answer n=4 entity=established-decisions#14.1 v=no
why: the replaced-element aspect-ratio rule is a per-feature layout value the chunk neither changes nor re-measures
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification is a per-feature layout rule outside the buffered render's reach
@answer n=6 entity=established-decisions#14.3 v=yes
why: the input_focused_on_mount_enables_ime test sits in the dioxus_document.rs mod tests the chunk edits and exercises the mount-time focus path through initial_build
@answer n=7 entity=established-decisions#15.1 v=yes
why: a node the bridge removes is dropped with its descendants and its mappings cleared, and interaction state on removed nodes is cleared, both of which the replayed buffer must keep
@answer n=8 entity=established-decisions#16.1 v=yes
why: the focus rules skipping inert nodes in the Tab walk, autofocus and unkeyed_actionable are the focus rules scope item 5 names and the mount-time autofocus runs through the replayed initial_build
@answer n=9 entity=established-decisions#16.2 v=yes
why: DocumentMutator::flush's clear_focus fixup on inert focus is the flush the replayed buffer must keep under one borrow
@answer n=10 entity=established-decisions#16.3 v=yes
why: set_focus_to and its FOCUS_WITHIN and redraw behaviour are the engine focus change the inverted pins and the ported test assert
@answer n=11 entity=established-decisions#16.4 v=yes
why: flush settles recorded selects and Dioxus re-render writes dynamic selected through the mutator the replay must keep flushing
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup membership and Role::Group are select-model rules the plan excludes, and the replay reaches the same record arm unchanged
@answer n=13 entity=established-decisions#16.6 v=no
why: the open select list header paint is paint_select_list, which the plan excludes
@answer n=14 entity=established-decisions#16.7 v=no
why: the default sheet's optgroup and multiple rows are select styling the chunk does not touch
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple select size hint is select layout the chunk does not touch
@answer n=16 entity=established-decisions#17.1 v=no
why: the hit-test and paint-walk skip of dropped nodes is a hit-test rule the chunk's code does not call, and removal itself is carried by established-decisions#15.1
@answer n=17 entity=established-decisions#17.2 v=no
why: the :open state of details and dialog is a per-element state the buffered render does not change
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing options are upstream of the Dioxus bridge and the chunk does not parse HTML
@answer n=19 entity=established-decisions#19.1 v=no
why: windowing, AccessKit and View lifecycle rules are shell behaviour the chunk does not change, and the windowed launch is read only
@answer n=20 entity=established-decisions#20.1 v=yes
why: the windowed redraw path through the event loop is the redraw behaviour the replayed mutations must keep, and it names the focus_redraw check that witnesses it
@answer n=21 entity=established-decisions#21.1 v=no
why: painting, layer caps and the password mask are paint rules outside the chunk
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element's border box and culling are paint rules the chunk does not touch
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript on Boa and its event synthesis are outside a Dioxus document, which runs no page script
@answer n=24 entity=established-decisions#24.1 v=no
why: DioxusDocument::new's base_url and default stylesheet, the hot-reload gate and in-memory history are construction and launch facts the chunk does not change
@answer n=25 entity=established-decisions#25.1 v=yes
why: this is the hazard the chunk fixes, the RefCell held across render_immediate that a polled set_focus borrows through, owned by the MountedData buffered render
