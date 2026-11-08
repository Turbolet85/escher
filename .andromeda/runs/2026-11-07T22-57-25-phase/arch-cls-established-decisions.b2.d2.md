@answer n=1 entity=established-decisions#11.16 v=no
why: AGENTS.md staying out of the tree is an upstream-sync fact that the graph read does not touch
@answer n=2 entity=established-decisions#12.1 v=no
why: the overlay-scrollbar click hazard binds driver clicks, which the graph read does not issue and the scope does not name
@answer n=3 entity=established-decisions#13.1 v=no
why: table and layout rules; the graph reads built nodes and changes no layout, and no scope item names a layout rule
@answer n=4 entity=established-decisions#14.1 v=yes
why: a direct engine change for every document in the item 20 list, which the chunk must leave as it is
@answer n=5 entity=established-decisions#14.2 v=yes
why: a direct engine change for every document in the item 20 list (pseudo-element flow), which the chunk must leave as it is
@answer n=6 entity=established-decisions#14.3 v=yes
why: a direct engine change for every document in the item 20 list (IME on a focused text input), which the chunk must leave as it is
@answer n=7 entity=established-decisions#15.1 v=yes
why: names unkeyed_actionable (item 2) and the element_id and node_id mapping clearing on removal that the element_id walk resolves through (item 1); disabled also gates the click reading (item 3)
@answer n=8 entity=established-decisions#16.1 v=yes
why: an inert subtree leaves the accessibility tree and so the snapshot (items 3 and 4), and unkeyed_actionable skips it (item 2)
@answer n=9 entity=established-decisions#16.2 v=no
why: HTML focus fixup on inert subtrees; focus is named by no scope item and the graph reads no focus
@answer n=10 entity=established-decisions#16.3 v=no
why: focus-within bookkeeping; no scope item names focus and the graph reads none
@answer n=11 entity=established-decisions#16.4 v=yes
why: select options and their chosen state are nodes and control values the accessibility builder writes (items 4 and 10), which the graph is keyed on
@answer n=12 entity=established-decisions#16.5 v=yes
why: optgroup's Role::Group and its label are built by build_accessibility_tree (item 4), so they are graph nodes
@answer n=13 entity=established-decisions#16.6 v=no
why: header paint of an open select list; the graph paints nothing
@answer n=14 entity=established-decisions#16.7 v=no
why: default-sheet display rules for optgroup and multiple rows; styling the graph does not read
@answer n=15 entity=established-decisions#16.8 v=no
why: a multiple select's size hint is layout only, which the chunk does not read or change
@answer n=16 entity=established-decisions#17.1 v=yes
why: a direct engine change for every document in the item 20 list (hit-test skip of dropped nodes), which the chunk must leave as it is
@answer n=17 entity=established-decisions#17.2 v=yes
why: a direct engine change for every document in the item 20 list (the OPEN element state), which the chunk must leave as it is
@answer n=18 entity=established-decisions#18.1 v=yes
why: its file-input feature gate bounds the masked-control test (item 10) and the features the chunk forwards (item 16)
@answer n=19 entity=established-decisions#19.1 v=no
why: windowing and shell rules; the chunk opens no window and forwards no shell event
@answer n=20 entity=established-decisions#20.1 v=yes
why: a direct engine change for every document in the item 20 list (redraw requests through the shell loop), which the chunk must leave as it is
@answer n=21 entity=established-decisions#21.1 v=yes
why: the password-input paint mask is a direct engine change for every document in the item 20 list with named merge surfaces, beside the masked-value rule (item 10)
@answer n=22 entity=established-decisions#22.1 v=yes
why: a direct engine change for every document in the item 20 list (render_element border box and culling), which the chunk must leave as it is
@answer n=23 entity=established-decisions#23.1 v=no
why: JavaScript runtime rules; the stand is a Dioxus app with no script (item 5 not needed)
@answer n=24 entity=established-decisions#24.1 v=no
why: DioxusDocument::new's default stylesheet and base_url; the scope names no constructor behaviour and the stand's config does not change
@answer n=25 entity=established-decisions#25.1 v=yes
why: the buffered render of the Dioxus bridge that the stand and the signal registry's writes run through, with its named merge surfaces dioxus_document.rs, mutation_buffer.rs and lib.rs
