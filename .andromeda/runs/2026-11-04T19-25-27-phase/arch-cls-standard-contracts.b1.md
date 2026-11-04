@answer n=1 entity=standard-contracts#1.1 v=yes
why: the stand's pinned DocumentConfig values (800x600 viewport, bundled font, offline net providers) and the Document trait the harness pumps through handle_ui_event and poll(None) are what the probe passes and drives
@answer n=2 entity=standard-contracts#2.1 v=yes
why: the VirtualClock that tick and advance_time move is the DocumentConfig clock the harness shares, and its advance adds with saturation
@answer n=3 entity=standard-contracts#2.2 v=no
why: the select value read is a select reader, and the probe reads no select (not_needed n=1)
@answer n=4 entity=standard-contracts#2.3 v=no
why: the open select list state is the select model the probe does not read (not_needed n=1)
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list reads only an open select list, which the probe does not read
@answer n=6 entity=standard-contracts#2.5 v=no
why: the reconcile of the select list in resolve is a select path the probe does not reach
@answer n=7 entity=standard-contracts#2.6 v=no
why: select values, select_of and the closed-select snapshot answer for selects the probe does not read
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select write record is a select rule, and the probe reads no select
@answer n=9 entity=standard-contracts#2.8 v=yes
why: the offline stand boot named in scope item 5 depends on a failed stylesheet no longer blocking render
@answer n=10 entity=standard-contracts#3.1 v=yes
why: the probe writes nothing to the DOM, so the mutate() writers are calls it must avoid, and the query and accessibility-tree readers are the read surface a read-only check uses under the accessibility feature
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single select's expanded state and aria-expanded are select behaviour the probe does not read
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree's focus write concerns the platform accessibility tree, which the probe does not read
@answer n=13 entity=standard-contracts#3.4 v=no
why: native checkbox state in the tree concerns a checkable control the chunk does not use
@answer n=14 entity=standard-contracts#3.5 v=no
why: unrounded resolved width and height are geometry, which no link of the UI graph reads (not_needed n=14)
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation runs only for writes the probe does not make, and no scope item names restyle or snapshot_node
@answer n=16 entity=standard-contracts#4.1 v=yes
why: the event driver's dispatch and the Dioxus onclick queued before the default action are what links 2 and 3 are measured through handle_ui_event (task item 12)
@answer n=17 entity=standard-contracts#4.2 v=no
why: the select list default actions are select behaviour the probe does not drive (not_needed n=1)
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label-to-select focus arm concerns a bound select the probe does not read
@answer n=19 entity=standard-contracts#4.4 v=no
why: the checkbox and radio change event is not named in the scope, and the scope names no checkable control
@answer n=20 entity=standard-contracts#5.1 v=yes
why: the NodeTree stale-id indexing panic binds the probe's id reads, which must go through get when a handler's render drops a node, while the scroll, hit and geometry readers in this entity are not needed (not_needed n=14)
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is a geometry reader, and the scope states that no link of the UI graph reads geometry (not_needed n=14)
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is a geometry reader the scope excludes (not_needed n=14)
@answer n=23 entity=standard-contracts#6.3 v=no
why: the text extent is a geometry reader that only the driver's lint calls, and the scope excludes geometry
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk's skip of dropped nodes governs click targeting, which the scope excludes as hit testing (not_needed n=14)
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform rule is hit testing the scope excludes (not_needed n=14)
