@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the probe's link (4) reads a state-reading node's name off the tree, and the TextRun labelling of its parent is what gives that name.
@answer n=2 entity=screen-reader-support#1.2 v=yes
why: aria-label is what names the CRUD list People, the listbox surface the probe reads.
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label-for association is the builder's ARIA name read the chunk does not own, and the stand's labelled inputs are not in scope.
@answer n=4 entity=screen-reader-support#1.4 v=no
why: ListBoxOption children and the select's value belong to the select list, which the probe does not read.
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single select's expanded state from engine state belongs to the select list, not in scope.
@answer n=6 entity=screen-reader-support#1.6 v=no
why: the open list's highlighted row belongs to the select list, not in scope.
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the assistive-technology reading of an open list belongs to the select list and the windowed read, not in scope.
@answer n=8 entity=screen-reader-support#1.8 v=no
why: a multiple select's row bounds are a select-list reading, not in scope.
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values of a multiple select is a select-list reading, not in scope.
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: the author_id of every element node is the element id the probe addresses each node by (need 1), the same rule as the standing S2 lines it also states.
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot reads role, name and focus off the accessibility tree (need 3), and the MASKED_VALUE password rule stays as it is (need 9).
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: the probe adds no snapshot field, and the thirteen NODE_FIELDS with no expanded key is the field set its link (2) answer rests on.
@answer n=13 entity=screen-reader-support#3.3 v=no
why: option and group labels leaving an open list only as pixels is a select-list reading, not in scope.
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checkbox and radio checked state is not read by the Counter or CRUD surfaces.
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and its check unkeyed_actionable are what link (2) runs and the click answer rests on (need 2).
@answer n=16 entity=screen-reader-support#4.2 v=no
why: Cells' cells-grid aria_label and its missing role concern Cells, which the probe does not touch.
@answer n=17 entity=screen-reader-support#4.3 v=no
why: the load page is a stand page beside the lean four, outside the Counter or CRUD surfaces the probe runs on.
@answer n=18 entity=screen-reader-support#5.1 v=yes
why: the CRUD list's listbox and option rows with aria-selected, and the list's keyboard selection gap, are the surface the probe drives (need 8).
@answer n=19 entity=screen-reader-support#6.1 v=no
why: the HTML tag name on element nodes is not a read the probe makes, since it addresses nodes by author_id.
@answer n=20 entity=screen-reader-support#6.2 v=yes
why: the tree focus and its fallback to the window node are what the probe reads after a key press (needs 3 and 6).
@answer n=21 entity=screen-reader-support#6.3 v=no
why: the role-mapping test's stated goal concerns role derivation, and the probe names no role.
@answer n=22 entity=screen-reader-support#7.1 v=no
why: initial-tree request routing in accesskit_xplat is the adapter's own path, not the poll refresh the probe reads (not_needed 9).
@answer n=23 entity=screen-reader-support#8.1 v=no
why: the AccessKit adapter's creation and event forwarding is not_needed 9, since no adapter is built.
@answer n=24 entity=screen-reader-support#8.2 v=yes
why: the Dioxus snapshot and tree sit behind dioxus-native-dom's accessibility feature, which the probe must enable to reach seven_guis (need 10).
@answer n=25 entity=screen-reader-support#9.1 v=no
why: the lang and xml:lang element language is not read by the probe.
