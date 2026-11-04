@answer n=1 entity=screen-reader-support#1.1 v=yes
why: the probe reads a reading node's name, which the TextRun children of the element give it (Counter's text)
@answer n=2 entity=screen-reader-support#1.2 v=yes
why: the scope names the CRUD list as named People by aria-label, the aria-label-to-label rule that carries it
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the probe reads no label-for input; the scope names only the aria-label naming of the CRUD list
@answer n=4 entity=screen-reader-support#1.4 v=no
why: a select's option list is not a probe surface (not_needed n=2); the probe reads no select list
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the single select's expanded reading from the open list belongs to the select popup, which the probe does not read
@answer n=6 entity=screen-reader-support#1.6 v=no
why: an open list's highlighted row is a select-popup reading the probe does not read
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is a select-popup reading, unmeasured and not run here
@answer n=8 entity=screen-reader-support#1.8 v=no
why: a multiple select's row bounds are a select-popup reading the probe does not read
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values() of a multiple select is a form reading the probe does not use
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: every element node's stable element id as author_id is the id the probe addresses a node by (scope n=1), and the scope names the shared tables
@answer n=11 entity=screen-reader-support#3.1 v=yes
why: the snapshot's role, name and focused readings off accessibility_tree are what the probe reads of each node (scope n=3)
@answer n=12 entity=screen-reader-support#3.2 v=no
why: the expanded node key belongs to an open select list the probe does not read; the field count stays a re-check every change owes
@answer n=13 entity=screen-reader-support#3.3 v=no
why: an option or group label leaving an open list only as painted pixels is a select-popup reading the probe does not read
@answer n=14 entity=screen-reader-support#3.4 v=no
why: native checkbox and radio checked states are not on the Counter or CRUD surfaces the probe reads
@answer n=15 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and DioxusDocument::unkeyed_actionable, whose listener mark and residual line link (2)'s click answer reads (scope n=2)
@answer n=16 entity=screen-reader-support#4.2 v=no
why: Cells' cells-grid is not on the probe's surfaces (Counter or CRUD)
@answer n=17 entity=screen-reader-support#4.3 v=no
why: the load page is not a surface the probe reads; its check result is a re-check every change owes
@answer n=18 entity=screen-reader-support#5.1 v=yes
why: the CRUD list's listbox role, its option rows' aria-selected and the arrow keys that move its selection are the handler-to-state surface (scope n=8)
@answer n=19 entity=screen-reader-support#6.1 v=no
why: the probe's verdicts read no HTML tag name; the tag in unkeyed_actionable's output is the check's own report
@answer n=20 entity=screen-reader-support#6.2 v=yes
why: tree focus falls to the Window node when the focused node is not built, which the probe's press on the focused control reads (scope n=6, n=7)
@answer n=21 entity=screen-reader-support#6.3 v=no
why: a statement of the role-mapping test's goal, with no rule the probe must honour
@answer n=22 entity=screen-reader-support#7.1 v=yes
why: the windowed shell's routing of initial-tree and tree requests to one handler is the path the probe reads, not runs, for the tree refresh (scope n=5, n=10)
@answer n=23 entity=screen-reader-support#8.1 v=no
why: the AccessKit adapter's creation is the not_needed platform-adapter item, and no adapter is built
@answer n=24 entity=screen-reader-support#8.2 v=yes
why: the accessibility feature gating the Dioxus and blitz exposure is what the probe reaches seven_guis through escher-driver (scope n=10)
@answer n=25 entity=screen-reader-support#9.1 v=no
why: lang and xml:lang set the element language, which the probe does not read
