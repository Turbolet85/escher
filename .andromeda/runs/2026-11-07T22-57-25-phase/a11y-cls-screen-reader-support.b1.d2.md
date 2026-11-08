@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the graph keys on agent-visible accesskit nodes with an optional owner, so the owner-less text-run nodes and their labelling of the parent are read through this build
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label and aria-labelledby naming is a name rule the chunk does not change and the graph does not read
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label-to-control association is a name rule the chunk does not touch
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: the snapshot bounds of closed-select options and of an open list's painted rows, read through select_list_bounds before the client rect, are the bounds the graph node must carry
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the select's expanded reading is engine open-list state that the graph neither reads nor writes and no snapshot field carries
@answer n=6 entity=screen-reader-support#1.6 v=no
why: a stated limit that an open list's highlighted row is in no snapshot and no tree, which the chunk does not change
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the unmeasured assistive-technology reading of an open list is recorded as unrunnable-here and the chunk changes no such reading
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: a multiple select's rows and groups reading their own client rects as bounds is the snapshot bounds rule the graph node must carry
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values() form entries for a multiple select are event payload, not a graph value, a masked carrier or a logged value
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: every element node carries its stable element id as author_id and text-run nodes carry none, which the chunk's author-id and graph keying must keep
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot model, its seven NodeState fields, to_text, diff, text_parts, password masking and node-key help are the readers the new reacts-to-click boolean joins
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: NODE_FIELDS reads thirteen and the new boolean moves that count, so the pin is re-pinned
@answer n=13 entity=screen-reader-support#3.3 v=yes
why: an option or group label reaches no log site, which the allowlist and redaction for escher and blitz targets must keep for graph names
@answer n=14 entity=screen-reader-support#3.4 v=no
why: native checkbox and radio checked and toggled readings come from the tree's role and the element, and the chunk changes neither
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and DioxusDocument::unkeyed_actionable are what the reacts-to-click field fulfils under item 2
@answer n=16 entity=screen-reader-support#4.2 v=yes
why: the reacts-to-click reading measured by link 2 is the field this chunk adds, and the Cells count of 677 it states is re-pinned
@answer n=17 entity=screen-reader-support#4.3 v=yes
why: the over-inclusive listener mark stays in force beside the exact dynamic-attribute reading that link 1 takes from the vdom walk
@answer n=18 entity=screen-reader-support#4.4 v=yes
why: cells-grid states no role and is one of the 677 counted no-role reacting elements, so its roleless reading is an input of the re-pinned Cells count
@answer n=19 entity=screen-reader-support#4.5 v=yes
why: the load page's keyed reading and its empty unkeyed_actionable result are the check's outcome on a stand page the id and listener work can move
@answer n=20 entity=screen-reader-support#5.1 v=yes
why: the CRUD list's ListBox and ListBoxOption roles, its People name and the flight booker's aria-invalid dates are stated roles and names the seven_guis signal move must keep
@answer n=21 entity=screen-reader-support#6.1 v=no
why: an element node's HTML tag name is a tree property the chunk does not touch and the graph does not carry
@answer n=22 entity=screen-reader-support#6.2 v=yes
why: the window root id u64::MAX and the rule that focus names only built nodes bound the reserved virtual-node id form and the graph's focus reading
@answer n=23 entity=screen-reader-support#6.3 v=no
why: a role-mapping test's stated goal binds no code of the chunk
@answer n=24 entity=screen-reader-support#7.1 v=no
why: the platform adapter's initial-tree routing is owned by res-llkgqygv and is not on the path the graph reads
@answer n=25 entity=screen-reader-support#8.1 v=no
why: the adapter's creation from the raw window handle is platform code off the chunk's path
