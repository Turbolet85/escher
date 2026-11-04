@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the probe's name reads of the Counter button and the CRUD rows come from text runs labelling their parents
@answer n=2 entity=screen-reader-support#1.2 v=yes
why: the CRUD list is named People by aria-label, and the aria-labelledby over aria-label order sets the name the probe reads
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label-for association names inputs only, and the probe reads no input, listbox or counter label
@answer n=4 entity=screen-reader-support#1.4 v=no
why: select option nodes and select_value concern select elements, and the probe reads no select list
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single select's expanded state concerns an open select list, which the probe does not read
@answer n=6 entity=screen-reader-support#1.6 v=no
why: an open list's highlighted row is outside the probe's reads, since the probe reads no select list
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the assistive-technology reading of an open list is unmeasured and outside the probe's surfaces
@answer n=8 entity=screen-reader-support#1.8 v=no
why: the bounds of multiple-select rows are geometry the probe does not read
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values() of a multiple select is a form path the probe does not use
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: author_id is the stable element id the probe addresses each node by, and the tree must equal the harness document
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the probe's role, name and focus reads come from accessibility_tree through the snapshot model with no second mapping
@answer n=12 entity=screen-reader-support#3.2 v=yes
why: the probe adds no snapshot field, and NODE_FIELDS holding thirteen with no expanded key bounds the click answer
@answer n=13 entity=screen-reader-support#3.3 v=no
why: option and group labels in an open list leave only as painted pixels, a select-list surface the probe does not read
@answer n=14 entity=screen-reader-support#3.4 v=no
why: the native checkbox or radio checked state is read on controls the probe does not touch
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule, its unkeyed_actionable check and the listener mark are the check the probe runs and the click answer's residual line
@answer n=16 entity=screen-reader-support#4.2 v=no
why: the cells-grid aria-label and missing role concern the Cells page, which the probe does not read
@answer n=17 entity=screen-reader-support#4.3 v=no
why: the load page's check reading is outside the probe's Counter or CRUD and served-task surfaces
@answer n=18 entity=screen-reader-support#5.1 v=yes
why: the CRUD listbox and its option rows with aria-selected are the handler-to-state surface the probe reads
@answer n=19 entity=screen-reader-support#6.1 v=no
why: the element tag name is not read or named by the probe
@answer n=20 entity=screen-reader-support#6.2 v=yes
why: the probe reads focus from the tree to reach the focused control, and the window fallback for unbuilt focus binds that read
@answer n=21 entity=screen-reader-support#6.3 v=no
why: the role-mapping test's stated goal is commentary on the mapping suite, not a rule the probe honours
@answer n=22 entity=screen-reader-support#7.1 v=yes
why: the windowed tree handover that routes the initial-tree request to a handler is part of the windowed refresh path item 5 reads, not runs
@answer n=23 entity=screen-reader-support#8.1 v=yes
why: the adapter's creation and event forwarding into the shell loop is the windowed refresh path the probe reads, not runs
@answer n=24 entity=screen-reader-support#8.2 v=yes
why: the accessibility feature gating the Dioxus exposure is the feature the scope names as reaching seven_guis
@answer n=25 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang element language is outside everything the probe reads
