@answer n=1 entity=screen-reader-support#1.1 v=no
why: TextRun nodes and a parent's labelling by its text are name computation, which the task drops as not needed since the read lists ids, not names.
@answer n=2 entity=screen-reader-support#1.2 v=no
why: aria-label and aria-labelledby name computation is dropped by the task's not-needed item, the read naming nothing.
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label-to-control name association is name computation, dropped by the task's not-needed item, the read naming nothing.
@answer n=4 entity=screen-reader-support#1.4 v=yes
why: an option's bounds and its presence in the tree under display none fix the owner-served option rows and their off-screen and covered causes, which the read must agree with.
@answer n=5 entity=screen-reader-support#1.5 v=yes
why: a single select's open state, which decides whether an owner-served option's verb is accepted now, is read from the tree's expanded with focus staying on the select.
@answer n=6 entity=screen-reader-support#1.6 v=yes
why: an open list's highlighted row is in no snapshot and no tree, so the read cannot list it as an owner-served node, as the task's needs item on the open-list row states.
@answer n=7 entity=screen-reader-support#1.7 v=no
why: what an assistive technology reads of an open list is an AT measurement, and the read is no AT witness.
@answer n=8 entity=screen-reader-support#1.8 v=yes
why: a multiple select's rows read their own client rects as bounds, which the read's off-screen and covered causes for owner-served rows must agree with.
@answer n=9 entity=screen-reader-support#1.9 v=no
why: evt.values() form entries for a multiple select are event payload, and the read lists no values.
@answer n=10 entity=screen-reader-support#2.1 v=yes
why: every element node carries its stable author_id and no TextRun, document-root or Window node does, which is the id the read lists per node and the basis of the author-key versus positional-path distinction.
@answer n=11 entity=screen-reader-support#2.2 v=yes
why: the read is a read like graph, so stand_graph's contract that every graph node is a snapshot node, that a graph read changes nothing and holds no typed text, and that served ids avoid the reserved owner separator binds the new read's owner ids.
@answer n=12 entity=screen-reader-support#3.1 v=yes
why: the snapshot's role, name, focus and enabled model and the disabled refusal rule (enabled Some(false) refused with nothing dispatched) fix the read's disabled cause and its agreement with a real call.
@answer n=13 entity=screen-reader-support#3.2 v=yes
why: clickable, taken from the vdom walk's current click listeners on an element that is no disabled form control, is the reading the read's click verb takes for a listed node.
@answer n=14 entity=screen-reader-support#3.3 v=yes
why: DioxusDocument::graph, one GraphNode per snapshot node with an owner for options and optgroups, is the node the read lists, and its read-only contract is the one the read inherits.
@answer n=15 entity=screen-reader-support#3.4 v=yes
why: a list's open state reaches the tree and not the snapshot, so the read takes an owner-served option's open-list state from the tree, not from a node key.
@answer n=16 entity=screen-reader-support#3.5 v=no
why: an option label leaving an open list as painted pixels and a row's shot cropping are screenshot facts, and the read takes no screenshot and lists no names.
@answer n=17 entity=screen-reader-support#3.6 v=no
why: native checkbox and radio checked states are read by toggled and checked, which the read lists neither of, its output being verbs and refusal causes.
@answer n=18 entity=screen-reader-support#4.1 v=yes
why: the actionable-key rule and unkeyed_actionable are the check the task folds in as residual L12, and its focusability, listener mark and interactive-role inputs define the read's actionable set.
@answer n=19 entity=screen-reader-support#4.2 v=yes
why: the CLICKABLE table's per-task counts of click-reacting elements with no role are actionable nodes the read lists on the stand, so its stand proof must cover them.
@answer n=20 entity=screen-reader-support#4.3 v=yes
why: the dioxus_runtime_probe measurement of which elements react to click is the witness for the clickable reading the read's click verb relies on.
@answer n=21 entity=screen-reader-support#4.4 v=yes
why: clickable reads true on counter-increment and on Cells' data cells and false on counter-value and cells-grid, the values the read's click acceptance is proven against on the stand.
@answer n=22 entity=screen-reader-support#4.5 v=yes
why: the listener mark over-reports a removed listener while clickable reads current listeners, which decides whether the read accepts a click on a listed node.
@answer n=23 entity=screen-reader-support#4.6 v=no
why: cells-grid's aria_label and its missing role concern role and name semantics the read does not map or list, and cells-grid reads clickable false.
@answer n=24 entity=screen-reader-support#4.7 v=yes
why: the actionable-key check returns nothing on the load page, one of the six tasks escher-session serves, so the folded-in check's stand proof must hold there too.
@answer n=25 entity=screen-reader-support#5.1 v=yes
why: CRUD person rows are options of a listbox, not focusable, with the list holding focus, which fixes how the read lists those rows against what a click on a row does.
