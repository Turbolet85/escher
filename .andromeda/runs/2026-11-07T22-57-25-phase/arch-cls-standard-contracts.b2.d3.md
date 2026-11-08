@answer n=1 entity=standard-contracts#6.6 v=yes
why: records a direct engine change for every document in stylo.rs, the list that item 20 says must stay as it is
@answer n=2 entity=standard-contracts#7.1 v=no
why: crate-private layout and stylo_taffy API the chunk never calls, bounds come from already-built layout
@answer n=3 entity=standard-contracts#8.1 v=yes
why: records a direct engine change for every document in node.rs, the list that item 20 says must stay as it is
@answer n=4 entity=standard-contracts#9.1 v=yes
why: the graph reads each node kind, FileInput is part of the masked-control test and the range input value is a control value the graph carries
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled is the pointer gate a click must pass, so the reacts-to-a-click reading of a disabled control is a fork over it
@answer n=6 entity=standard-contracts#10.2 v=no
why: the select list's disabled-row reading for popup rows the graph does not read
@answer n=7 entity=standard-contracts#10.3 v=no
why: select steps, typeahead and choice writes are select-popup rules the graph does not read
@answer n=8 entity=standard-contracts#10.4 v=no
why: select cursor and gesture internals are crate-private and outside the graph's read
@answer n=9 entity=standard-contracts#10.5 v=no
why: select list view constants and read types are popup paint values the graph does not use
@answer n=10 entity=standard-contracts#10.6 v=no
why: the focus-restyle hold on a just-opened select list is a click-open behaviour the scope does not name
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list's row cap and placement are popup rules the graph never reads
@answer n=12 entity=standard-contracts#10.8 v=no
why: out-of-flow recording is a style flag that the graph's bounds do not read
@answer n=13 entity=standard-contracts#10.9 v=yes
why: records a direct engine change for every document in node.rs and construct.rs, the list that item 20 says must stay as it is
@answer n=14 entity=standard-contracts#10.10 v=no
why: the class attribute sync is a style-matching rule, and the chunk names ElementData by type only and builds no literal of it
@answer n=15 entity=standard-contracts#10.11 v=no
why: open-state sync for details and dialog is not part of the graph's reading of nodes
@answer n=16 entity=standard-contracts#11.1 v=yes
why: the load page's LoadNet keeps the default is_noop false so its fetch counts as a load, and the Viewport fields carry the pinned 800x600 Light stand
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net are not used by the stand, a Dioxus app, and the task excludes networking
@answer n=18 entity=standard-contracts#13.1 v=no
why: password paint masking is a paint-frame rule and the graph paints nothing
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list is a popup paint rule the graph does not touch
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text colour and synthesized bold are paint internals the graph does not touch
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation paint through blend layers the graph does not touch
@answer n=22 entity=standard-contracts#13.5 v=yes
why: records the CARRY owner on the Upstream sync line that item 20 says must stay as it is
@answer n=23 entity=standard-contracts#14.1 v=no
why: the BlitzShellProvider redraw path is a windowed shell path, and the stand is headless with no shell
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the stand boots through Harness with HarnessOptions for the pinned viewport, and settle with the load-page counter decides the graph read on the load page
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the stand's documents read the harness clock through from_vdom, and the graph read and its replay must not move that clock
