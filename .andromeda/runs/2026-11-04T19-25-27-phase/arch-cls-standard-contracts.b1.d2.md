@answer n=1 entity=standard-contracts#1.1 v=yes
why: the DocumentConfig fields the stand's pinned viewport and offline boot set, the poll-time changed-set drain the shell loop runs (item 10), and DioxusDocument's accessibility_tree override with stable ids that the accessibility-gated snapshot reads (item 17)
@answer n=2 entity=standard-contracts#2.1 v=yes
why: the VirtualClock the harness shares, which only tick and advance_time move, is the clock the document holds through its config and reads for the fling and smooth frames under poll(None)
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value is the chosen-value reader of a select; the probe reads no select (not_needed item 1)
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list state and its readers and writers; the probe reads no select (not_needed item 1)
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list is the reader of the open select list's state; the probe reads no select (not_needed item 1)
@answer n=6 entity=standard-contracts#2.5 v=no
why: resolve's reconcile call and the removal arm clear select-list state only; the probe reads no select (not_needed item 1)
@answer n=7 entity=standard-contracts#2.6 v=no
why: select_values, select_of, the typeahead clear and closed_select_of concern a select's value and open list; the probe reads no select (not_needed item 1)
@answer n=8 entity=standard-contracts#2.7 v=no
why: the record of a multiple select's write settles a select's set; the probe reads no select (not_needed item 1)
@answer n=9 entity=standard-contracts#2.8 v=yes
why: a failed stylesheet stops blocking pending render, which the stand's offline boot (item 5) depends on when the first poll waits for resources
@answer n=10 entity=standard-contracts#3.1 v=yes
why: the accessibility tree build with its author_id ids and the states class is what the bridge's accessibility-gated read-only snapshot check (item 17) and the stable ids of element_id read through DioxusDocument
@answer n=11 entity=standard-contracts#3.2 v=no
why: a single select's expanded state and its open-list steering; the probe reads no select (not_needed item 1)
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus field names no link of the UI graph; the probe reads no focus in the tree
@answer n=13 entity=standard-contracts#3.4 v=no
why: the toggled state a native checkbox or radio writes into the tree; no link of the UI graph reads checkedness and the scope names no native checkable control
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height read the unrounded layout, which is geometry; no link of the UI graph reads geometry (not_needed item 14)
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on set, clear, removal and insertion is the engine's own write path; the probe writes nothing to the DOM or vdom
@answer n=16 entity=standard-contracts#4.1 v=yes
why: the event driver queues a click so the app's onclick handlers run before the default action, the dispatched event names feed the listener map, and the Dioxus oninput converter carries the value unchanged (item 12, links 2 and 3)
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list's default actions (pointer, keyboard, wheel, choice) concern an open select; the probe reads no select (not_needed item 1)
@answer n=18 entity=standard-contracts#4.3 v=no
why: a label's click focusing a bound single select opens nothing; the probe reads no select (not_needed item 1)
@answer n=19 entity=standard-contracts#4.4 v=yes
why: the engine's change event after input on a checkbox or radio click is an event name the listener map shows, and the app's click handler runs before that default action (item 12)
@answer n=20 entity=standard-contracts#5.1 v=yes
why: NodeTree get and the stale-id indexing panic bind the bridge's element_id and element_ids reads of a node the document may have dropped (item 13 and link 1); the scroll API and visible_region are geometry that no link reads
@answer n=21 entity=standard-contracts#6.1 v=no
why: get_client_bounding_rect under transforms is a geometry reader; no link of the UI graph reads geometry (not_needed item 14), and the hazard is a RefCell borrow, not a value
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is a geometry reader; no link of the UI graph reads geometry (not_needed item 14)
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is a geometry reader whose one non-test caller is escher-driver's lint; no link of the UI graph reads geometry (not_needed item 14)
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk skipping dropped nodes is hit testing; no link of the UI graph reads hit results (not_needed item 14)
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is hit testing; no link of the UI graph reads hit results (not_needed item 14)
