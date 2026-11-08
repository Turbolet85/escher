@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md kept out of the tree; no build or chunk code reads it
@answer n=2 entity=established-decisions#12.1 v=no
why: overlay-scrollbar click hazard on driver clicks; the graph read dispatches no click and the scope names no scrolled-box click
@answer n=3 entity=established-decisions#13.1 v=no
why: table, damage and text-input layout rules; the graph reads already-built nodes and changes no layout
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element aspect-ratio layout; the chunk changes no layout
@answer n=5 entity=established-decisions#14.2 v=no
why: pseudo-element flow classification is box-tree layout; the graph walks the vdom, not the box tree
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enabling for a focused text input on construction; no focus or IME path is changed
@answer n=7 entity=established-decisions#15.1 v=yes
why: DioxusDocument::unkeyed_actionable moves with focusability, and a removed node clears element_id_mapping, which link 1's stable-id vdom walk reads
@answer n=8 entity=established-decisions#16.1 v=yes
why: inert subtrees are skipped by unkeyed_actionable and leave the accessibility tree that the snapshot and build_accessibility_tree read
@answer n=9 entity=established-decisions#16.2 v=no
why: focus fixup on inert nodes in DocumentMutator::flush; the chunk neither moves focus nor fires events
@answer n=10 entity=established-decisions#16.3 v=no
why: :focus-within bookkeeping on focus moves; the chunk does not change focus
@answer n=11 entity=established-decisions#16.4 v=yes
why: the accessibility builder keeps select options under display none and writes their selected state, so the graph keyed on the agent-visible node reads these option nodes
@answer n=12 entity=established-decisions#16.5 v=yes
why: the builder gives an optgroup Role::Group named by its label, an agent-visible node the graph keys on
@answer n=13 entity=established-decisions#16.6 v=no
why: paint of an open select list's group header; the graph paints nothing
@answer n=14 entity=established-decisions#16.7 v=no
why: default-sheet rows for optgroup and multiple selects are style; the chunk changes no style
@answer n=15 entity=established-decisions#16.8 v=no
why: multiple-select size hint is layout
@answer n=16 entity=established-decisions#17.1 v=no
why: hit test and paint-tree skip of dropped nodes; the graph reads through the vdom walk, not the hit test
@answer n=17 entity=established-decisions#17.2 v=no
why: :open state on details and dialog is a CSS state the snapshot does not read
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parsing options and template fragments; the Dioxus bridge builds no HTML template in the document
@answer n=19 entity=established-decisions#19.1 v=no
why: windowed shell's AccessKit initialisation and windowing; the chunk's stand is headless
@answer n=20 entity=established-decisions#20.1 v=no
why: windowed shell redraw path; the graph read is headless
@answer n=21 entity=established-decisions#21.1 v=no
why: paint masking of password inputs in blitz-paint; the chunk paints nothing and the entity does not rule over the graph's own value masking
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling for transforms; paint only
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript runtime and script fetching; the stand is a Dioxus app with no script
@answer n=24 entity=established-decisions#24.1 v=no
why: document base_url and the absence of a UA stylesheet from DioxusDocument::new; the chunk changes no document config
@answer n=25 entity=established-decisions#25.1 v=yes
why: the buffered render through MutationBuffer and the single inner borrow bind the registry's signal writes and the graph read on the same bridge, and dioxus_document.rs carries element_id and unkeyed_actionable, the owned merge surface
