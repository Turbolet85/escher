@answer n=1 entity=screen-reader-support#1.1 v=no
why: TextRun labelling of a parent is accessible name computation, which the read does not list (not_needed 3)
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label naming is accessible name computation, which the read does not list (not_needed 3)
@answer n=3 entity=screen-reader-support#1.3 v=no
why: label-for and aria-labelledby association is name computation, which the read does not list (not_needed 3)
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: a closed select's options take the select's client rect as bounds and an open list's row reads its own, which the owner-served node and its causes must agree with (needs 9)
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the tree's expanded reading of a single select is not a snapshot field the read consumes, and the read carries no expanded state
@answer n=6 entity=screen-reader-support#1.6 v=yes
why: the highlighted row is in no snapshot, so an owner-served open-list row the read lists cannot carry it (needs 9)
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is unmeasured and out of the read, which runs no assistive technology
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: a multiple select's rows read their own client rects as bounds, which the owner-served node's causes and hit test must agree with (needs 9)
@answer n=9 entity=screen-reader-support#1.9 v=no
why: multiple_select_values feeds event form values, which the read neither carries nor touches, while value stays select_value
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: every element node's author_id is the stable id the read lists per node, and no TextRun, root or Window node carries one (needs 4)
@answer n=11 entity=screen-reader-support#2.2 v=yes
why: the graph agreeing with the snapshot, the reserved /// separator kept out of served ids and a graph read changing nothing bind the read's owner-served node (needs 2)
@answer n=12 entity=screen-reader-support#3.1 v=yes
why: it is the snapshot model the read consumes, its role, name, focus, enabled and clickable readings, the disabled keying and the MASKED_VALUE password reading (needs 1, 5, 6, 11)
@answer n=13 entity=screen-reader-support#3.2 v=yes
why: clickable is the snapshot reading the read takes for a node that acts on click, from current listeners and excluding disabled form controls (needs 1, 3)
@answer n=14 entity=screen-reader-support#3.3 v=yes
why: DioxusDocument::graph is the one-node-per-snapshot-node model with an optional owner that the new read is built on (needs 2)
@answer n=15 entity=screen-reader-support#3.4 v=no
why: NodeState holds no expanded field and the read adds no snapshot field, so the NODE_FIELDS count is not moved by the chunk
@answer n=16 entity=screen-reader-support#3.5 v=no
why: a label leaves an open list only as paint and the snapshot name, and the read's answer carries no label and adds no log site
@answer n=17 entity=screen-reader-support#3.6 v=no
why: a checkbox's toggled and native checked readings on the tree are not carried by the read's verbs or refusal causes
@answer n=18 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and the unkeyed_actionable check are the folded residual L12 check the read carries per node (needs 3, 4)
@answer n=19 entity=screen-reader-support#4.2 v=yes
why: the CLICKABLE table fixes which elements read clickable per stand task, which the read's actionable set must agree with in both layout modes (needs 1)
@answer n=20 entity=screen-reader-support#4.3 v=yes
why: it states the clickable reading on Counter, Cells and CRUD that the read's listed clickable nodes take from the model (needs 1, 3)
@answer n=21 entity=screen-reader-support#4.4 v=yes
why: clickable true on counter-increment and Cells' data cells, false on counter-value and cells-grid, is the value the read lists per node (needs 1)
@answer n=22 entity=screen-reader-support#4.5 v=yes
why: the listener mark over-includes a removed listener while clickable reads current listeners, a distinction the check and the read must keep (needs 3)
@answer n=23 entity=screen-reader-support#4.6 v=no
why: cells-grid's missing role and cell semantics are out of the chunk (not_needed 6), and the read adds no role mapping
@answer n=24 entity=screen-reader-support#4.7 v=yes
why: the folded check must return nothing on the load page, whose hidden load-done element is outside the tree and snapshot and so bounds what the read can list (needs 3, 8)
@answer n=25 entity=screen-reader-support#5.1 v=yes
why: the CRUD list's listbox role, its non-focusable option rows and the flight type select's options are owner-served sub-targets the read must list in agreement with a click (needs 9, 10)
