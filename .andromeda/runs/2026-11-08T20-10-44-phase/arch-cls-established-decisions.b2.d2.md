@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md kept out of the tree, and the chunk reads or writes no AGENTS.md file
@answer n=2 entity=established-decisions#12.1 v=yes
why: a click inside a just-scrolled box is dropped under the overlay scrollbar hazard, so a replayed actions step or stand check that clicks after a scroll must advance past the fade first
@answer n=3 entity=established-decisions#13.1 v=no
why: table, replaced, float, damage and paint-tree layout rules, and the chunk changes no layout and takes no layout value
@answer n=4 entity=established-decisions#14.1 v=no
why: replaced-element natural height from aspect-ratio, a layout rule the chunk does not change or read
@answer n=5 entity=established-decisions#14.2 v=no standing=S1
why: the pseudo-element flow engine change and its test file asserting both layout modes, the convention S1 names
@answer n=6 entity=established-decisions#14.3 v=no
why: IME enabling for a text input focused at editor construction, a path the chunk does not add or touch
@answer n=7 entity=established-decisions#15.1 v=yes
why: a bridge-removed node drops its element_id_mapping and node_id_mapping entries so it stops resolving, which the coverage recording relies on for data-dioxus-id, and the disabled rule moves focusability, unkeyed_actionable and the snapshot's clickable reading that the actionable rule uses
@answer n=8 entity=established-decisions#16.1 v=yes
why: inert nodes read not focusable through Node::is_focussable and are skipped by DioxusDocument::unkeyed_actionable, which the actionable_nodes rule and the snapshot's clickable reading take
@answer n=9 entity=established-decisions#16.2 v=no
why: the mutator's inert focus fixup on flush, and the chunk's coverage read and replay move no focus
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within and redraw bookkeeping on focus moves, state the chunk neither reads nor changes
@answer n=11 entity=established-decisions#16.4 v=no
why: select choice, option rules and form submission, none of which the chunk's actionable reading or handler coverage names
@answer n=12 entity=established-decisions#16.5 v=no
why: optgroup recording and group label, a select path the chunk does not touch
@answer n=13 entity=established-decisions#16.6 v=no
why: open select list header painting, and the chunk paints nothing
@answer n=14 entity=established-decisions#16.7 v=no
why: default-sheet optgroup and multiple row rules, styling the chunk does not change and does not paint
@answer n=15 entity=established-decisions#16.8 v=no
why: the multiple select size height hint, a select layout rule the chunk leaves alone
@answer n=16 entity=established-decisions#17.1 v=no
why: hit tests skipping a node dropped since the last resolve, and the chunk names no hit test
@answer n=17 entity=established-decisions#17.2 v=no
why: the :open state on details and dialog, a path the chunk does not reach
@answer n=18 entity=established-decisions#18.1 v=no
why: HTML parse options and XHTML sniffing, and no HTML fixture or parse path changes
@answer n=19 entity=established-decisions#19.1 v=no
why: windowed shell creation and drop ordering, while the chunk runs on the headless stand and builds no windowed shell
@answer n=20 entity=established-decisions#20.1 v=no
why: windowed redraw requests through the event loop, a path the headless stand does not name
@answer n=21 entity=established-decisions#21.1 v=no
why: painting, layer limits, culling and the password mask, and the chunk paints nothing
@answer n=22 entity=established-decisions#22.1 v=no
why: render_element border box and culling on the scaled size, a painting rule the chunk does not change
@answer n=23 entity=established-decisions#23.1 v=no
why: the Boa JavaScript runtime and its scripts, and the chunk's events are DOM events on the Dioxus bridge with no script path touched
@answer n=24 entity=established-decisions#24.1 v=yes
why: DioxusDocument::new adds no stylesheet and a config's ua_stylesheets is used as given, which the headless stand's pinned font and viewport boot passes through
@answer n=25 entity=established-decisions#25.1 v=yes
why: the bridge's buffered render through MutationBuffer and the MountedData path is the one the chunk's recording on the bridge's event path runs under, so a recorded handler must not borrow the document RefCell
